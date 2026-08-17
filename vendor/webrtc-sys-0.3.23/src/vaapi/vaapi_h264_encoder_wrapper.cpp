#include "vaapi_h264_encoder_wrapper.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <map>

#include "rtc_base/logging.h"

#define NAL_REF_IDC_NONE 0
#define NAL_REF_IDC_LOW 1
#define NAL_REF_IDC_MEDIUM 2
#define NAL_REF_IDC_HIGH 3

#define NAL_NON_IDR 1
#define NAL_IDR 5
#define NAL_SPS 7
#define NAL_PPS 8
#define NAL_SEI 6

#define SLICE_TYPE_P 0
#define SLICE_TYPE_B 1
#define SLICE_TYPE_I 2
#define IS_P_SLICE(type) (SLICE_TYPE_P == (type))
#define IS_B_SLICE(type) (SLICE_TYPE_B == (type))
#define IS_I_SLICE(type) (SLICE_TYPE_I == (type))

#define ENTROPY_MODE_CAVLC 0
#define ENTROPY_MODE_CABAC 1

#define PROFILE_IDC_BASELINE 66
#define PROFILE_IDC_MAIN 77
#define PROFILE_IDC_HIGH 100

#define BITSTREAM_ALLOCATE_STEPPING 4096

static const uint32_t MaxFrameNum = (2 << 16);
static const uint32_t MaxPicOrderCntLsb = (2 << 8);
static const uint32_t Log2MaxFrameNum = 16;
static const uint32_t Log2MaxPicOrderCntLsb = 8;
static const uint32_t num_ref_frames = 2;
static const int srcyuv_fourcc = VA_FOURCC_NV12;
static const uint32_t frame_slices = 1;

static const int rc_default_modes[] = {
    VA_RC_VBR, VA_RC_CQP, VA_RC_VBR_CONSTRAINED,
    VA_RC_CBR, VA_RC_VCM, VA_RC_NONE,
};

VAImageFormat kImageFormatI420 = {
    .fourcc = VA_FOURCC_I420,
    .byte_order = VA_LSB_FIRST,
    .bits_per_pixel = 12,
};

static int upload_surface_yuv(VADisplay va_dpy,
                              VASurfaceID surface_id,
                              int src_fourcc,
                              int src_width,
                              int src_height,
                              uint8_t* src_Y,
                              uint8_t* src_U,
                              uint8_t* src_V) {
  VAImage surface_image;
  uint8_t *surface_p = NULL, *Y_start = NULL, *U_start = NULL;
  int Y_pitch = 0, U_pitch = 0, row;
  VAStatus va_status;

  va_status = vaDeriveImage(va_dpy, surface_id, &surface_image);
  if (va_status != VA_STATUS_SUCCESS) {
    // If the driver does not support vaDeriveImage, create a new image.
    va_status = vaCreateImage(va_dpy, &kImageFormatI420, src_width, src_height,
                              &surface_image);
    if (va_status != VA_STATUS_SUCCESS) {
      RTC_LOG(LS_ERROR) << "vaCreateImage failed with status " << va_status;
      return -1;
    }
  }

  vaMapBuffer(va_dpy, surface_image.buf, (void**)&surface_p);
  assert(VA_STATUS_SUCCESS == va_status);

  Y_start = surface_p;
  Y_pitch = surface_image.pitches[0];
  switch (surface_image.format.fourcc) {
    case VA_FOURCC_NV12:
      U_start = (unsigned char*)surface_p + surface_image.offsets[1];
      U_pitch = surface_image.pitches[1];
      break;
    case VA_FOURCC_I420:
      U_start = (unsigned char*)surface_p + surface_image.offsets[1];
      U_pitch = surface_image.pitches[1];
      break;
    case VA_FOURCC_YV12:
      U_start = (unsigned char*)surface_p + surface_image.offsets[2];
      U_pitch = surface_image.pitches[2];
      break;
    case VA_FOURCC_YUY2:
      U_start = surface_p + 1;
      U_pitch = surface_image.pitches[0];
      break;
    default:
      assert(0);
  }

  /* copy Y plane */
  for (row = 0; row < src_height; row++) {
    uint8_t* Y_row = Y_start + row * Y_pitch;
    memcpy(Y_row, src_Y + row * src_width, src_width);
  }
  for (row = 0; row < src_height / 2; row++) {
    uint8_t* U_row = U_start + row * U_pitch;
    uint8_t *u_ptr = NULL, *v_ptr = NULL;
    int j;
    if (src_fourcc == VA_FOURCC_NV12) {
      memcpy(U_row, src_U + row * src_width, src_width);
      break;
    } else if (src_fourcc == VA_FOURCC_I420) {
      u_ptr = src_U + row * (src_width / 2);
      v_ptr = src_V + row * (src_width / 2);
    } else if (src_fourcc == VA_FOURCC_YV12) {
      v_ptr = src_U + row * (src_width / 2);
      u_ptr = src_V + row * (src_width / 2);
    }
    if ((src_fourcc == VA_FOURCC_I420) || (src_fourcc == VA_FOURCC_YV12)) {
      for (j = 0; j < src_width / 2; j++) {
        U_row[2 * j] = u_ptr[j];
        U_row[2 * j + 1] = v_ptr[j];
      }
    }
  }
  vaUnmapBuffer(va_dpy, surface_image.buf);
  vaDestroyImage(va_dpy, surface_image.image_id);

  return 0;
}

#define MIN(a, b) ((a) > (b) ? (b) : (a))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

struct __bitstream {
  uint32_t* buffer;
  int bit_offset;
  int max_size_in_dword;
};
typedef struct __bitstream bitstream;

static uint32_t va_swap32(uint32_t val) {
  unsigned char* pval = (unsigned char*)&val;

  return ((pval[0] << 24) | (pval[1] << 16) | (pval[2] << 8) | (pval[3] << 0));
}

static void bitstream_start(bitstream* bs) {
  bs->max_size_in_dword = BITSTREAM_ALLOCATE_STEPPING;
  bs->buffer = (uint32_t*)calloc(bs->max_size_in_dword * sizeof(int), 1);
  assert(bs->buffer);
  bs->bit_offset = 0;
}

static void bitstream_end(bitstream* bs) {
  int pos = (bs->bit_offset >> 5);
  int bit_offset = (bs->bit_offset & 0x1f);
  int bit_left = 32 - bit_offset;

  if (bit_offset) {
    bs->buffer[pos] = va_swap32((bs->buffer[pos] << bit_left));
  }
}

static void bitstream_put_ui(bitstream* bs, uint32_t val, int size_in_bits) {
  int pos = (bs->bit_offset >> 5);
  int bit_offset = (bs->bit_offset & 0x1f);
  int bit_left = 32 - bit_offset;

  if (!size_in_bits)
    return;

  bs->bit_offset += size_in_bits;

  if (bit_left > size_in_bits) {
    bs->buffer[pos] = (bs->buffer[pos] << size_in_bits | val);
  } else {
    size_in_bits -= bit_left;
    bs->buffer[pos] = (bs->buffer[pos] << bit_left) | (val >> size_in_bits);
    bs->buffer[pos] = va_swap32(bs->buffer[pos]);

    if (pos + 1 == bs->max_size_in_dword) {
      bs->max_size_in_dword += BITSTREAM_ALLOCATE_STEPPING;
      bs->buffer = bs->buffer = (uint32_t*)realloc(
          bs->buffer, bs->max_size_in_dword * sizeof(uint32_t));
      assert(bs->buffer);
    }

    bs->buffer[pos + 1] = val;
  }
}

static void bitstream_put_ue(bitstream* bs, uint32_t val) {
  int size_in_bits = 0;
  int tmp_val = ++val;

  while (tmp_val) {
    tmp_val >>= 1;
    size_in_bits++;
  }

  bitstream_put_ui(bs, 0, size_in_bits - 1);  // leading zero
  bitstream_put_ui(bs, val, size_in_bits);
}

static void bitstream_put_se(bitstream* bs, int val) {
  uint32_t new_val;

  if (val <= 0)
    new_val = -2 * val;
  else
    new_val = 2 * val - 1;

  bitstream_put_ue(bs, new_val);
}

static void bitstream_byte_aligning(bitstream* bs, int bit) {
  int bit_offset = (bs->bit_offset & 0x7);
  int bit_left = 8 - bit_offset;
  int new_val;

  if (!bit_offset)
    return;

  assert(bit == 0 || bit == 1);

  if (bit)
    new_val = (1 << bit_left) - 1;
  else
    new_val = 0;

  bitstream_put_ui(bs, new_val, bit_left);
}

static void rbsp_trailing_bits(bitstream* bs) {
  bitstream_put_ui(bs, 1, 1);
  bitstream_byte_aligning(bs, 0);
}

static void nal_start_code_prefix(bitstream* bs) {
  bitstream_put_ui(bs, 0x00000001, 32);
}

static void nal_header(bitstream* bs, int nal_ref_idc, int nal_unit_type) {
  bitstream_put_ui(bs, 0, 1); /* forbidden_zero_bit: 0 */
  bitstream_put_ui(bs, nal_ref_idc, 2);
  bitstream_put_ui(bs, nal_unit_type, 5);
}

static void sps_rbsp(VA264Context* context, bitstream* bs) {
  int profile_idc = PROFILE_IDC_BASELINE;

  if (context->config.h264_profile == VAProfileH264High)
    profile_idc = PROFILE_IDC_HIGH;
  else if (context->config.h264_profile == VAProfileH264Main)
    profile_idc = PROFILE_IDC_MAIN;

  bitstream_put_ui(bs, profile_idc, 8); /* profile_idc */
  bitstream_put_ui(bs, !!(context->constraint_set_flag & 1),
                   1); /* constraint_set0_flag */
  bitstream_put_ui(bs, !!(context->constraint_set_flag & 2),
                   1); /* constraint_set1_flag */
  bitstream_put_ui(bs, !!(context->constraint_set_flag & 4),
                   1); /* constraint_set2_flag */
  bitstream_put_ui(bs, !!(context->constraint_set_flag & 8),
                   1);        /* constraint_set3_flag */
  bitstream_put_ui(bs, 0, 4); /* reserved_zero_4bits */
  bitstream_put_ui(bs, context->seq_param.level_idc, 8); /* level_idc */
  bitstream_put_ue(
      bs, context->seq_param.seq_parameter_set_id); /* seq_parameter_set_id */

  if (profile_idc == PROFILE_IDC_HIGH) {
    bitstream_put_ue(bs, 1);    /* chroma_format_idc = 1, 4:2:0 */
    bitstream_put_ue(bs, 0);    /* bit_depth_luma_minus8 */
    bitstream_put_ue(bs, 0);    /* bit_depth_chroma_minus8 */
    bitstream_put_ui(bs, 0, 1); /* qpprime_y_zero_transform_bypass_flag */
    bitstream_put_ui(bs, 0, 1); /* seq_scaling_matrix_present_flag */
  }

  bitstream_put_ue(
      bs, context->seq_param.seq_fields.bits
              .log2_max_frame_num_minus4); /* log2_max_frame_num_minus4 */
  bitstream_put_ue(bs, context->seq_param.seq_fields.bits
                           .pic_order_cnt_type); /* pic_order_cnt_type */

  if (context->seq_param.seq_fields.bits.pic_order_cnt_type == 0)
    bitstream_put_ue(
        bs,
        context->seq_param.seq_fields.bits
            .log2_max_pic_order_cnt_lsb_minus4); /* log2_max_pic_order_cnt_lsb_minus4
                                                  */
  else {
    assert(0);
  }

  bitstream_put_ue(bs,
                   context->seq_param.max_num_ref_frames); /* num_ref_frames */
  bitstream_put_ui(bs, 0, 1); /* gaps_in_frame_num_value_allowed_flag */

  bitstream_put_ue(bs, context->seq_param.picture_width_in_mbs -
                           1); /* pic_width_in_mbs_minus1 */
  bitstream_put_ue(bs, context->seq_param.picture_height_in_mbs -
                           1); /* pic_height_in_map_units_minus1 */
  bitstream_put_ui(bs, context->seq_param.seq_fields.bits.frame_mbs_only_flag,
                   1); /* frame_mbs_only_flag */

  if (!context->seq_param.seq_fields.bits.frame_mbs_only_flag) {
    assert(0);
  }

  bitstream_put_ui(bs,
                   context->seq_param.seq_fields.bits.direct_8x8_inference_flag,
                   1); /* direct_8x8_inference_flag */
  bitstream_put_ui(bs, context->seq_param.frame_cropping_flag,
                   1); /* frame_cropping_flag */

  if (context->seq_param.frame_cropping_flag) {
    bitstream_put_ue(
        bs,
        context->seq_param.frame_crop_left_offset); /* frame_crop_left_offset */
    bitstream_put_ue(
        bs, context->seq_param
                .frame_crop_right_offset); /* frame_crop_right_offset */
    bitstream_put_ue(
        bs,
        context->seq_param.frame_crop_top_offset); /* frame_crop_top_offset */
    bitstream_put_ue(
        bs, context->seq_param
                .frame_crop_bottom_offset); /* frame_crop_bottom_offset */
  }

  // if ( frame_bit_rate < 0 ) { //TODO EW: the vui header isn't correct
  if (1) {
    bitstream_put_ui(bs, 0, 1); /* vui_parameters_present_flag */
  } else {
    bitstream_put_ui(bs, 1, 1); /* vui_parameters_present_flag */
    bitstream_put_ui(bs, 0, 1); /* aspect_ratio_info_present_flag */
    bitstream_put_ui(bs, 0, 1); /* overscan_info_present_flag */
    bitstream_put_ui(bs, 0, 1); /* video_signal_type_present_flag */
    bitstream_put_ui(bs, 0, 1); /* chroma_loc_info_present_flag */
    bitstream_put_ui(bs, 1, 1); /* timing_info_present_flag */
    {
      bitstream_put_ui(bs, 15, 32);
      bitstream_put_ui(bs, 900, 32);
      bitstream_put_ui(bs, 1, 1);
    }
    bitstream_put_ui(bs, 1, 1); /* nal_hrd_parameters_present_flag */
    {
      // hrd_parameters
      bitstream_put_ue(bs, 0);    /* cpb_cnt_minus1 */
      bitstream_put_ui(bs, 4, 4); /* bit_rate_scale */
      bitstream_put_ui(bs, 6, 4); /* cpb_size_scale */

      bitstream_put_ue(
          bs, context->config.bitrate - 1); /* bit_rate_value_minus1[0] */
      bitstream_put_ue(
          bs, context->config.bitrate * 8 - 1); /* cpb_size_value_minus1[0] */
      bitstream_put_ui(bs, 1, 1);               /* cbr_flag[0] */

      bitstream_put_ui(bs, 23, 5); /* initial_cpb_removal_delay_length_minus1 */
      bitstream_put_ui(bs, 23, 5); /* cpb_removal_delay_length_minus1 */
      bitstream_put_ui(bs, 23, 5); /* dpb_output_delay_length_minus1 */
      bitstream_put_ui(bs, 23, 5); /* time_offset_length  */
    }
    bitstream_put_ui(bs, 0, 1); /* vcl_hrd_parameters_present_flag */
    bitstream_put_ui(bs, 0, 1); /* low_delay_hrd_flag */

    bitstream_put_ui(bs, 0, 1); /* pic_struct_present_flag */
    bitstream_put_ui(bs, 0, 1); /* bitstream_restriction_flag */
  }

  rbsp_trailing_bits(bs); /* rbsp_trailing_bits */
}

static void pps_rbsp(VA264Context* context, bitstream* bs) {
  bitstream_put_ue(
      bs, context->pic_param.pic_parameter_set_id); /* pic_parameter_set_id */
  bitstream_put_ue(
      bs, context->pic_param.seq_parameter_set_id); /* seq_parameter_set_id */

  bitstream_put_ui(bs,
                   context->pic_param.pic_fields.bits.entropy_coding_mode_flag,
                   1); /* entropy_coding_mode_flag */

  bitstream_put_ui(bs, 0, 1); /* pic_order_present_flag: 0 */

  bitstream_put_ue(bs, 0); /* num_slice_groups_minus1 */

  bitstream_put_ue(
      bs, context->pic_param
              .num_ref_idx_l0_active_minus1); /* num_ref_idx_l0_active_minus1 */
  bitstream_put_ue(
      bs, context->pic_param
              .num_ref_idx_l1_active_minus1); /* num_ref_idx_l1_active_minus1
                                                 1 */

  bitstream_put_ui(bs, context->pic_param.pic_fields.bits.weighted_pred_flag,
                   1); /* weighted_pred_flag: 0 */
  bitstream_put_ui(bs, context->pic_param.pic_fields.bits.weighted_bipred_idc,
                   2); /* weighted_bipred_idc: 0 */

  bitstream_put_se(
      bs, context->pic_param.pic_init_qp - 26); /* pic_init_qp_minus26 */
  bitstream_put_se(bs, 0);                      /* pic_init_qs_minus26 */
  bitstream_put_se(bs, 0);                      /* chroma_qp_index_offset */

  bitstream_put_ui(
      bs,
      context->pic_param.pic_fields.bits.deblocking_filter_control_present_flag,
      1);                     /* deblocking_filter_control_present_flag */
  bitstream_put_ui(bs, 0, 1); /* constrained_intra_pred_flag */
  bitstream_put_ui(bs, 0, 1); /* redundant_pic_cnt_present_flag */

  /* more_rbsp_data */
  bitstream_put_ui(bs,
                   context->pic_param.pic_fields.bits.transform_8x8_mode_flag,
                   1);        /*transform_8x8_mode_flag */
  bitstream_put_ui(bs, 0, 1); /* pic_scaling_matrix_present_flag */
  bitstream_put_se(
      bs, context->pic_param
              .second_chroma_qp_index_offset); /*second_chroma_qp_index_offset
                                                */

  rbsp_trailing_bits(bs);
}

static void slice_header(VA264Context* context, bitstream* bs) {
  int first_mb_in_slice = context->slice_param.macroblock_address;

  bitstream_put_ue(bs, first_mb_in_slice); /* first_mb_in_slice: 0 */
  bitstream_put_ue(bs, context->slice_param.slice_type); /* slice_type */
  bitstream_put_ue(
      bs,
      context->slice_param.pic_parameter_set_id); /* pic_parameter_set_id: 0 */
  bitstream_put_ui(
      bs, context->pic_param.frame_num,
      context->seq_param.seq_fields.bits.log2_max_frame_num_minus4 +
          4); /* frame_num */

  if (context->pic_param.pic_fields.bits.idr_pic_flag)
    bitstream_put_ue(bs, context->slice_param.idr_pic_id); /* idr_pic_id: 0 */

  if (context->seq_param.seq_fields.bits.pic_order_cnt_type == 0) {
    bitstream_put_ui(
        bs, context->pic_param.CurrPic.TopFieldOrderCnt,
        context->seq_param.seq_fields.bits.log2_max_pic_order_cnt_lsb_minus4 +
            4);
    /* pic_order_present_flag == 0 */
  }

  /* redundant_pic_cnt_present_flag == 0 */
  /* slice type */
  if (IS_P_SLICE(context->slice_param.slice_type)) {
    bitstream_put_ui(bs, context->slice_param.num_ref_idx_active_override_flag,
                     1); /* num_ref_idx_active_override_flag: */

    if (context->slice_param.num_ref_idx_active_override_flag)
      bitstream_put_ue(bs, context->slice_param.num_ref_idx_l0_active_minus1);

    /* ref_pic_list_reordering */
    bitstream_put_ui(bs, 0, 1); /* ref_pic_list_reordering_flag_l0: 0 */
  } else if (IS_B_SLICE(context->slice_param.slice_type)) {
    bitstream_put_ui(bs, context->slice_param.direct_spatial_mv_pred_flag,
                     1); /* direct_spatial_mv_pred: 1 */

    bitstream_put_ui(bs, context->slice_param.num_ref_idx_active_override_flag,
                     1); /* num_ref_idx_active_override_flag: */

    if (context->slice_param.num_ref_idx_active_override_flag) {
      bitstream_put_ue(bs, context->slice_param.num_ref_idx_l0_active_minus1);
      bitstream_put_ue(bs, context->slice_param.num_ref_idx_l1_active_minus1);
    }

    /* ref_pic_list_reordering */
    bitstream_put_ui(bs, 0, 1); /* ref_pic_list_reordering_flag_l0: 0 */
    bitstream_put_ui(bs, 0, 1); /* ref_pic_list_reordering_flag_l1: 0 */
  }

  if ((context->pic_param.pic_fields.bits.weighted_pred_flag &&
       IS_P_SLICE(context->slice_param.slice_type)) ||
      ((context->pic_param.pic_fields.bits.weighted_bipred_idc == 1) &&
       IS_B_SLICE(context->slice_param.slice_type))) {
  }

  /* dec_ref_pic_marking */
  if (context->pic_param.pic_fields.bits
          .reference_pic_flag) { /* nal_ref_idc != 0 */
    unsigned char no_output_of_prior_pics_flag = 0;
    unsigned char long_term_reference_flag = 0;
    unsigned char adaptive_ref_pic_marking_mode_flag = 0;

    if (context->pic_param.pic_fields.bits.idr_pic_flag) {
      bitstream_put_ui(bs, no_output_of_prior_pics_flag,
                       1); /* no_output_of_prior_pics_flag: 0 */
      bitstream_put_ui(bs, long_term_reference_flag,
                       1); /* long_term_reference_flag: 0 */
    } else {
      bitstream_put_ui(bs, adaptive_ref_pic_marking_mode_flag,
                       1); /* adaptive_ref_pic_marking_mode_flag: 0 */
    }
  }

  if (context->pic_param.pic_fields.bits.entropy_coding_mode_flag &&
      !IS_I_SLICE(context->slice_param.slice_type))
    bitstream_put_ue(
        bs, context->slice_param.cabac_init_idc); /* cabac_init_idc: 0 */

  bitstream_put_se(bs,
                   context->slice_param.slice_qp_delta); /* slice_qp_delta: 0 */

  /* ignore for SP/SI */

  if (context->pic_param.pic_fields.bits
          .deblocking_filter_control_present_flag) {
    bitstream_put_ue(
        bs,
        context->slice_param
            .disable_deblocking_filter_idc); /* disable_deblocking_filter_idc:
                                                0 */

    if (context->slice_param.disable_deblocking_filter_idc != 1) {
      bitstream_put_se(
          bs,
          context->slice_param
              .slice_alpha_c0_offset_div2); /* slice_alpha_c0_offset_div2: 2 */
      bitstream_put_se(
          bs, context->slice_param
                  .slice_beta_offset_div2); /* slice_beta_offset_div2: 2 */
    }
  }

  if (context->pic_param.pic_fields.bits.entropy_coding_mode_flag) {
    bitstream_byte_aligning(bs, 1);
  }
}

static int build_packed_pic_buffer(VA264Context* context,
                                   unsigned char** header_buffer) {
  bitstream bs;

  bitstream_start(&bs);
  nal_start_code_prefix(&bs);
  nal_header(&bs, NAL_REF_IDC_HIGH, NAL_PPS);
  pps_rbsp(context, &bs);
  bitstream_end(&bs);

  *header_buffer = (unsigned char*)bs.buffer;
  return bs.bit_offset;
}

static int build_packed_seq_buffer(VA264Context* context,
                                   unsigned char** header_buffer) {
  bitstream bs;

  bitstream_start(&bs);
  nal_start_code_prefix(&bs);
  nal_header(&bs, NAL_REF_IDC_HIGH, NAL_SPS);
  sps_rbsp(context, &bs);
  bitstream_end(&bs);

  *header_buffer = (unsigned char*)bs.buffer;
  return bs.bit_offset;
}

static int build_packed_slice_buffer(VA264Context* context,
                                     unsigned char** header_buffer) {
  bitstream bs;
  int is_idr = !!context->pic_param.pic_fields.bits.idr_pic_flag;
  int is_ref = !!context->pic_param.pic_fields.bits.reference_pic_flag;

  bitstream_start(&bs);
  nal_start_code_prefix(&bs);

  if (IS_I_SLICE(context->slice_param.slice_type)) {
    nal_header(&bs, NAL_REF_IDC_HIGH, is_idr ? NAL_IDR : NAL_NON_IDR);
  } else if (IS_P_SLICE(context->slice_param.slice_type)) {
    nal_header(&bs, NAL_REF_IDC_MEDIUM, NAL_NON_IDR);
  } else {
    assert(IS_B_SLICE(context->slice_param.slice_type));
    nal_header(&bs, is_ref ? NAL_REF_IDC_LOW : NAL_REF_IDC_NONE, NAL_NON_IDR);
  }

  slice_header(context, &bs);
  bitstream_end(&bs);

  *header_buffer = (unsigned char*)bs.buffer;
  return bs.bit_offset;
}

/*
  Assume frame sequence is: Frame#0,#1,#2,...,#M,...,#X,... (encoding order)
  1) period between Frame #X and Frame #N = #X - #N
  2) 0 means infinite for intra_period/intra_idr_period, and 0 is invalid for
  ip_period 3) intra_idr_period % intra_period (intra_period > 0) and
  intra_period % ip_period must be 0 4) intra_period and intra_idr_period take
  precedence over ip_period 5) if ip_period > 1, intra_period and
  intra_idr_period are not  the strict periods of I/IDR frames, see bellow
  examples
  -------------------------------------------------------------------
  intra_period intra_idr_period ip_period frame sequence
  (intra_period/intra_idr_period/ip_period) 0            ignored          1
  IDRPPPPPPP ...     (No IDR/I any more) 0            ignored        >=2
  IDR(PBB)(PBB)...   (No IDR/I any more) 1            0                ignored
  IDRIIIIIII...      (No IDR any more) 1            1                ignored IDR
  IDR IDR IDR... 1            >=2              ignored    IDRII IDRII IDR...
  (1/3/ignore)
  >=2          0                1          IDRPPP IPPP I...   (3/0/1)
  >=2          0              >=2          IDR(PBB)(PBB)(IBB) (6/0/3)
                                              (PBB)(IBB)(PBB)(IBB)...
  >=2          >=2              1          IDRPPPPP IPPPPP IPPPPP (6/18/1)
                                           IDRPPPPP IPPPPP IPPPPP...
  >=2          >=2              >=2        {IDR(PBB)(PBB)(IBB)(PBB)(IBB)(PBB)}
  (6/18/3) {IDR(PBB)(PBB)(IBB)(PBB)(IBB)(PBB)}... {IDR(PBB)(PBB)(IBB)(PBB)}
  (6/12/3) {IDR(PBB)(PBB)(IBB)(PBB)}... {IDR(PBB)(PBB)} (6/6/3) {IDR(PBB)(PBB)}.
*/

/*
 * Return displaying order with specified periods and encoding order
 * displaying_order: displaying order
 * frame_type: frame type
 */
#define FRAME_P 0
#define FRAME_B 1
#define FRAME_I 2
#define FRAME_IDR 7
void encoding2display_order(uint64_t encoding_order,
                            int intra_period,
                            int intra_idr_period,
                            int ip_period,
                            uint64_t* displaying_order,
                            int* frame_type) {
  int encoding_order_gop = 0;

  if (intra_period == 1) { /* all are I/IDR frames */
    *displaying_order = encoding_order;
    if (intra_idr_period == 0)
      *frame_type = (encoding_order == 0) ? FRAME_IDR : FRAME_I;
    else
      *frame_type =
          (encoding_order % intra_idr_period == 0) ? FRAME_IDR : FRAME_I;
    return;
  }

  if (intra_period == 0)
    intra_idr_period = 0;

  /* new sequence like
   * IDR PPPPP IPPPPP
   * IDR (PBB)(PBB)(IBB)(PBB)
   */
  encoding_order_gop =
      (intra_idr_period == 0)
          ? encoding_order
          : (encoding_order % (intra_idr_period + ((ip_period == 1) ? 0 : 1)));

  if (encoding_order_gop == 0) { /* the first frame */
    *frame_type = FRAME_IDR;
    *displaying_order = encoding_order;
  } else if (((encoding_order_gop - 1) % ip_period) != 0) { /* B frames */
    *frame_type = FRAME_B;
    *displaying_order = encoding_order - 1;
  } else if ((intra_period != 0) && /* have I frames */
             (encoding_order_gop >= 2) &&
             ((ip_period == 1 && encoding_order_gop % intra_period ==
                                     0) || /* for IDR PPPPP IPPPP */
              /* for IDR (PBB)(PBB)(IBB) */
              (ip_period >= 2 && ((encoding_order_gop - 1) / ip_period %
                                  (intra_period / ip_period)) == 0))) {
    *frame_type = FRAME_I;
    *displaying_order = encoding_order + ip_period - 1;
  } else {
    *frame_type = FRAME_P;
    *displaying_order = encoding_order + ip_period - 1;
  }
}

std::map<int, std::string> fourcc_map = {{VA_FOURCC_NV12, "NV12"},
                                         {VA_FOURCC_I420, "I420"},
                                         {VA_FOURCC_YV12, "YV12"},
                                         {VA_FOURCC_UYVY, "UYVY"}};

static std::string fourcc_to_string(int fourcc) {
  auto it = fourcc_map.find(fourcc);
  if (it != fourcc_map.end()) {
    return it->second;
  }
  RTC_LOG(LS_ERROR) << "Unknow FOURCC";
  return "Unknown";
}

std::map<int, std::string> rc_mode_map = {
    {VA_RC_NONE, "NONE"}, {VA_RC_CBR, "CBR"},
    {VA_RC_VBR, "VBR"},   {VA_RC_VCM, "VCM"},
    {VA_RC_CQP, "CQP"},   {VA_RC_VBR_CONSTRAINED, "VBR_CONSTRAINED"}};

static std::string rc_to_string(int rcmode) {
  auto it = rc_mode_map.find(rcmode);
  if (it != rc_mode_map.end()) {
    return it->second;
  }
  return "Unknown";
}

static int init_va(VA264Context* context, VADisplay va_dpy) {
  VAProfile profile_list[] = {VAProfileH264High, VAProfileH264Main,
                              VAProfileH264ConstrainedBaseline};
  VAEntrypoint* entrypoints;
  int num_entrypoints, slice_entrypoint;
  int support_encode = 0;
  int major_ver, minor_ver;
  VAStatus va_status;
  uint32_t i;

  context->va_dpy = va_dpy;
  if (!context->va_dpy) {
    return VA_STATUS_ERROR_INVALID_DISPLAY;
  }

  va_status = vaInitialize(context->va_dpy, &major_ver, &minor_ver);

  if (major_ver < 0 || minor_ver < 0 || va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaInitialize failed";
    return VA_STATUS_ERROR_INVALID_DISPLAY;
  }

  num_entrypoints = vaMaxNumEntrypoints(context->va_dpy);
  entrypoints = new VAEntrypoint[num_entrypoints * sizeof(*entrypoints)];
  if (!entrypoints) {
    RTC_LOG(LS_ERROR) << "failed to allocate VA entrypoints";
    return VA_STATUS_ERROR_INVALID_DISPLAY;
  }

  /* use the highest profile */
  for (i = 0; i < sizeof(profile_list) / sizeof(profile_list[0]); i++) {
    if ((context->config.h264_profile != ~0) &&
        context->config.h264_profile != profile_list[i])
      continue;

    context->config.h264_profile = profile_list[i];
    vaQueryConfigEntrypoints(context->va_dpy, context->config.h264_profile,
                             entrypoints, &num_entrypoints);
    for (slice_entrypoint = 0; slice_entrypoint < num_entrypoints;
         slice_entrypoint++) {
      if (context->requested_entrypoint == -1) {
        // Select the entry point based on what is avaiable
        if ((entrypoints[slice_entrypoint] == VAEntrypointEncSlice) ||
            (entrypoints[slice_entrypoint] == VAEntrypointEncSliceLP)) {
          support_encode = 1;
          context->selected_entrypoint = entrypoints[slice_entrypoint];
          break;
        }
      } else if ((entrypoints[slice_entrypoint] ==
                  context->requested_entrypoint)) {
        // Select the entry point based on what was requested in cmd line option
        support_encode = 1;
        context->selected_entrypoint = entrypoints[slice_entrypoint];
        break;
      }
    }
    if (support_encode == 1) {
      RTC_LOG(LS_INFO) << "Using EntryPoint - " << context->selected_entrypoint;
      break;
    }
  }

  if (support_encode == 0) {
    RTC_LOG(LS_ERROR)
        << "Can't find VAEntrypointEncSlice or VAEntrypointEncSliceLP for "
           "H264 profiles";
    return VA_STATUS_ERROR_UNSUPPORTED_ENTRYPOINT;
  } else {
    switch (context->config.h264_profile) {
      case VAProfileH264ConstrainedBaseline:
        RTC_LOG(LS_INFO) << "Use profile VAProfileH264ConstrainedBaseline";
        context->constraint_set_flag |= (1 << 0 | 1 << 1); /* Annex A.2.2 */
        context->config.ip_period = 1;
        break;

      case VAProfileH264Main:
        RTC_LOG(LS_INFO) << "Use profile VAProfileH264Main";
        context->constraint_set_flag |= (1 << 1); /* Annex A.2.2 */
        break;

      case VAProfileH264High:
        context->constraint_set_flag |= (1 << 3); /* Annex A.2.4 */
        RTC_LOG(LS_INFO) << "Use profile VAProfileH264High";
        break;
      default:
        RTC_LOG(LS_INFO) << "unknow profile. Set to Constrained Baseline";
        context->config.h264_profile = VAProfileH264ConstrainedBaseline;
        context->constraint_set_flag |=
            (1 << 0 | 1 << 1); /* Annex A.2.1 & A.2.2 */
        context->config.ip_period = 1;
        break;
    }
  }

  /* find out the format for the render target, and rate control mode */
  for (i = 0; i < VAConfigAttribTypeMax; i++)
    context->attrib[i].type = (VAConfigAttribType)i;

  va_status =
      vaGetConfigAttributes(context->va_dpy, context->config.h264_profile,
                            (VAEntrypoint)context->selected_entrypoint,
                            &context->attrib[0], VAConfigAttribTypeMax);
  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaGetConfigAttributes failed";
    delete[] entrypoints;
    return va_status;
  }

  /* check the interested configattrib */
  if ((context->attrib[VAConfigAttribRTFormat].value & VA_RT_FORMAT_YUV420) ==
      0) {
    RTC_LOG(LS_ERROR) << "Not find desired YUV420 RT format";
    return VA_STATUS_ERROR_INVALID_CONFIG;
  } else {
    context->config_attrib[context->config_attrib_num].type =
        VAConfigAttribRTFormat;
    context->config_attrib[context->config_attrib_num].value =
        VA_RT_FORMAT_YUV420;
    context->config_attrib_num++;
  }

  if (context->attrib[VAConfigAttribRateControl].value !=
      VA_ATTRIB_NOT_SUPPORTED) {
    int tmp = context->attrib[VAConfigAttribRateControl].value;

    // context->attrib[VAConfigAttribRateControl].value =
    // context->config.rc_mode;

    std::string rc_modes;
    if (tmp & VA_RC_NONE)
      rc_modes += "NONE ";
    if (tmp & VA_RC_VBR)
      rc_modes += "VBR ";
    if (tmp & VA_RC_CBR)
      rc_modes += "CBR ";
    if (tmp & VA_RC_VCM)
      rc_modes += "VCM ";
    if (tmp & VA_RC_CQP)
      rc_modes += "CQP ";
    if (tmp & VA_RC_VBR_CONSTRAINED)
      rc_modes += "VBR_CONSTRAINED ";

    RTC_LOG(LS_INFO) << "Support rate control mode: " << rc_modes;

    if (context->config.rc_mode == -1 || !(context->config.rc_mode & tmp)) {
      if (context->config.rc_mode != -1) {
        RTC_LOG(LS_WARNING)
            << "Warning: Don't support the specified RateControl mode: "
            << rc_to_string(context->config.rc_mode) << "!!!, switch to ";
      }

      for (i = 0; i < sizeof(rc_default_modes) / sizeof(rc_default_modes[0]);
           i++) {
        if (rc_default_modes[i] & tmp) {
          context->config.rc_mode = rc_default_modes[i];
          break;
        }
      }

      RTC_LOG(LS_INFO) << "RateControl mode: "
                       << rc_to_string(context->config.rc_mode);
    }

    context->config_attrib[context->config_attrib_num].type =
        VAConfigAttribRateControl;
    context->config_attrib[context->config_attrib_num].value =
        context->config.rc_mode;
    context->config_attrib_num++;
  }

  if (context->attrib[VAConfigAttribEncPackedHeaders].value !=
      VA_ATTRIB_NOT_SUPPORTED) {
    int tmp = context->attrib[VAConfigAttribEncPackedHeaders].value;

    RTC_LOG(LS_INFO) << "Support VAConfigAttribEncPackedHeaders: ";

    context->h264_packedheader = 1;
    context->config_attrib[context->config_attrib_num].type =
        VAConfigAttribEncPackedHeaders;
    context->config_attrib[context->config_attrib_num].value =
        VA_ENC_PACKED_HEADER_NONE;

    if (tmp & VA_ENC_PACKED_HEADER_SEQUENCE) {
      RTC_LOG(LS_INFO) << "Support packed sequence headers";
      context->config_attrib[context->config_attrib_num].value |=
          VA_ENC_PACKED_HEADER_SEQUENCE;
    }

    if (tmp & VA_ENC_PACKED_HEADER_PICTURE) {
      RTC_LOG(LS_INFO) << "Support packed picture headers";
      context->config_attrib[context->config_attrib_num].value |=
          VA_ENC_PACKED_HEADER_PICTURE;
    }

    if (tmp & VA_ENC_PACKED_HEADER_SLICE) {
      RTC_LOG(LS_INFO) << "Support packed slice headers";
      context->config_attrib[context->config_attrib_num].value |=
          VA_ENC_PACKED_HEADER_SLICE;
    }

    if (tmp & VA_ENC_PACKED_HEADER_MISC) {
      RTC_LOG(LS_INFO) << "Support packed misc headers";
      context->config_attrib[context->config_attrib_num].value |=
          VA_ENC_PACKED_HEADER_MISC;
    }

    context->enc_packed_header_idx = context->config_attrib_num;
    context->config_attrib_num++;
  }

  if (context->attrib[VAConfigAttribEncInterlaced].value !=
      VA_ATTRIB_NOT_SUPPORTED) {
    int tmp = context->attrib[VAConfigAttribEncInterlaced].value;

    RTC_LOG(LS_INFO) << "Support VAConfigAttribEncInterlaced: ";

    if (tmp & VA_ENC_INTERLACED_FRAME)
      RTC_LOG(LS_INFO) << "Support VA_ENC_INTERLACED_FRAME";
    if (tmp & VA_ENC_INTERLACED_FIELD)
      RTC_LOG(LS_INFO) << "Support VA_ENC_INTERLACED_FIELD";
    if (tmp & VA_ENC_INTERLACED_MBAFF)
      RTC_LOG(LS_INFO) << "Support VA_ENC_INTERLACED_MBAFF";
    if (tmp & VA_ENC_INTERLACED_PAFF)
      RTC_LOG(LS_INFO) << "Support VA_ENC_INTERLACED_PAFF";

    context->config_attrib[context->config_attrib_num].type =
        VAConfigAttribEncInterlaced;
    context->config_attrib[context->config_attrib_num].value =
        VA_ENC_PACKED_HEADER_NONE;
    context->config_attrib_num++;
  }

  if (context->attrib[VAConfigAttribEncMaxRefFrames].value !=
      VA_ATTRIB_NOT_SUPPORTED) {
    context->h264_maxref = context->attrib[VAConfigAttribEncMaxRefFrames].value;

    RTC_LOG(LS_INFO) << "Support " << (context->h264_maxref & 0xffff)
                     << " RefPicList0 and "
                     << ((context->h264_maxref >> 16) & 0xffff)
                     << " RefPicList1";
  }

  if (context->attrib[VAConfigAttribEncMaxSlices].value !=
      VA_ATTRIB_NOT_SUPPORTED)

    RTC_LOG(LS_INFO) << "Support "
                     << context->attrib[VAConfigAttribEncMaxSlices].value
                     << " slices";

  if (context->attrib[VAConfigAttribEncSliceStructure].value !=
      VA_ATTRIB_NOT_SUPPORTED) {
    int tmp = context->attrib[VAConfigAttribEncSliceStructure].value;

    RTC_LOG(LS_INFO) << "Support VAConfigAttribEncSliceStructure: ";

    RTC_LOG(LS_INFO) << "Support VAConfigAttribEncSliceStructure";

    if (tmp & VA_ENC_SLICE_STRUCTURE_ARBITRARY_ROWS)
      RTC_LOG(LS_INFO) << "Support VA_ENC_SLICE_STRUCTURE_ARBITRARY_ROWS";
    if (tmp & VA_ENC_SLICE_STRUCTURE_POWER_OF_TWO_ROWS)
      RTC_LOG(LS_INFO) << "Support VA_ENC_SLICE_STRUCTURE_POWER_OF_TWO_ROWS";
    if (tmp & VA_ENC_SLICE_STRUCTURE_ARBITRARY_MACROBLOCKS)
      RTC_LOG(LS_INFO)
          << "Support VA_ENC_SLICE_STRUCTURE_ARBITRARY_MACROBLOCKS";
  }
  if (context->attrib[VAConfigAttribEncMacroblockInfo].value !=
      VA_ATTRIB_NOT_SUPPORTED) {
    RTC_LOG(LS_INFO) << "Support VAConfigAttribEncMacroblockInfo";
  }

  delete[] entrypoints;

  return 0;
}

static int setup_encode(VA264Context* context) {
  VAStatus va_status;
  VASurfaceID* tmp_surfaceid;
  int codedbuf_size, i;

  va_status = vaCreateConfig(context->va_dpy, context->config.h264_profile,
                             (VAEntrypoint)context->selected_entrypoint,
                             &context->config_attrib[0],
                             context->config_attrib_num, &context->config_id);

  if (context->config_id == VA_INVALID_ID) {
    RTC_LOG(LS_ERROR) << "vaCreateConfig failed va_status = " << va_status;
    return -1;
  }

  /* create source surfaces */
  va_status = vaCreateSurfaces(context->va_dpy, VA_RT_FORMAT_YUV420,
                               context->frame_width_mbaligned,
                               context->frame_height_mbaligned,
                               &context->src_surface[0], SURFACE_NUM, NULL, 0);
  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateSurfaces failed va_status = " << va_status;
    return -1;
  }

  /* create reference surfaces */
  va_status = vaCreateSurfaces(context->va_dpy, VA_RT_FORMAT_YUV420,
                               context->frame_width_mbaligned,
                               context->frame_height_mbaligned,
                               &context->ref_surface[0], SURFACE_NUM, NULL, 0);
  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateSurfaces failed va_status = " << va_status;
    return -1;
  }

  tmp_surfaceid = new VASurfaceID[2 * SURFACE_NUM];
  assert(tmp_surfaceid);
  memcpy(tmp_surfaceid, context->src_surface,
         SURFACE_NUM * sizeof(VASurfaceID));
  memcpy(tmp_surfaceid + SURFACE_NUM, context->ref_surface,
         SURFACE_NUM * sizeof(VASurfaceID));

  /* Create a context for this encode pipe */
  va_status = vaCreateContext(
      context->va_dpy, context->config_id, context->frame_width_mbaligned,
      context->frame_height_mbaligned, VA_PROGRESSIVE, tmp_surfaceid,
      2 * SURFACE_NUM, &context->context_id);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateContext failed va_status = " << va_status;
    delete[] tmp_surfaceid;
    return -1;
  }

  delete[] tmp_surfaceid;

  codedbuf_size =
      (context->frame_width_mbaligned * context->frame_height_mbaligned * 400) /
      (16 * 16);

  for (i = 0; i < SURFACE_NUM; i++) {
    /* create coded buffer once for all
     * other VA buffers which won't be used again after vaRenderPicture.
     * so APP can always vaCreateBuffer for every frame
     * but coded buffer need to be mapped and accessed after
     * vaRenderPicture/vaEndPicture so VA won't maintain the coded buffer
     */
    va_status = vaCreateBuffer(context->va_dpy, context->context_id,
                               VAEncCodedBufferType, codedbuf_size, 1, NULL,
                               &context->coded_buf[i]);
    if (va_status != VA_STATUS_SUCCESS) {
      RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
      return -1;
    }
  }

  return 0;
}

#define partition(ref, field, key, ascending) \
  while (i <= j) {                            \
    if (ascending) {                          \
      while (ref[i].field < key)              \
        i++;                                  \
      while (ref[j].field > key)              \
        j--;                                  \
    } else {                                  \
      while (ref[i].field > key)              \
        i++;                                  \
      while (ref[j].field < key)              \
        j--;                                  \
    }                                         \
    if (i <= j) {                             \
      tmp = ref[i];                           \
      ref[i] = ref[j];                        \
      ref[j] = tmp;                           \
      i++;                                    \
      j--;                                    \
    }                                         \
  }

static void sort_one(VAPictureH264 ref[],
                     int left,
                     int right,
                     int ascending,
                     int frame_idx) {
  int i = left, j = right;
  uint32_t key;
  VAPictureH264 tmp;

  if (frame_idx) {
    key = ref[(left + right) / 2].frame_idx;
    partition(ref, frame_idx, key, ascending);
  } else {
    key = ref[(left + right) / 2].TopFieldOrderCnt;
    partition(ref, TopFieldOrderCnt, (signed int)key, ascending);
  }

  /* recursion */
  if (left < j)
    sort_one(ref, left, j, ascending, frame_idx);

  if (i < right)
    sort_one(ref, i, right, ascending, frame_idx);
}

static void sort_two(VAPictureH264 ref[],
                     int left,
                     int right,
                     uint32_t key,
                     uint32_t frame_idx,
                     int partition_ascending,
                     int list0_ascending,
                     int list1_ascending) {
  int i = left, j = right;
  VAPictureH264 tmp;

  if (frame_idx) {
    partition(ref, frame_idx, key, partition_ascending);
  } else {
    partition(ref, TopFieldOrderCnt, (signed int)key, partition_ascending);
  }

  sort_one(ref, left, i - 1, list0_ascending, frame_idx);
  sort_one(ref, j + 1, right, list1_ascending, frame_idx);
}

static int update_ReferenceFrames(VA264Context* context) {
  int i;

  if (context->current_frame_type == FRAME_B)
    return 0;

  context->current_curr_pic.flags = VA_PICTURE_H264_SHORT_TERM_REFERENCE;
  context->num_short_term++;
  if (context->num_short_term > num_ref_frames)
    context->num_short_term = num_ref_frames;
  for (i = context->num_short_term - 1; i > 0; i--)
    context->reference_frames[i] = context->reference_frames[i - 1];
  context->reference_frames[0] = context->current_curr_pic;

  if (context->current_frame_type != FRAME_B)
    context->current_frame_num++;
  if (context->current_frame_num > MaxFrameNum)
    context->current_frame_num = 0;

  return 0;
}

static int update_RefPicList(VA264Context* context) {
  uint32_t current_poc = context->current_curr_pic.TopFieldOrderCnt;

  if (context->current_frame_type == FRAME_IDR) {
    // per issue 1189 in Intel Media Driver:
    // https://github.com/intel/media-driver/issues/1189
    // For the start of each IDR, reset ALL the reference pic lists to invalid
    uint32_t flags = VA_PICTURE_H264_INVALID;
    for (int i = 0; i < SURFACE_NUM * 2; i++) {
      context->slice_param.RefPicList0[i].flags = flags;
      context->slice_param.RefPicList1[i].flags = flags;
      context->ref_pic_list0_p[i].flags = flags;
      context->ref_pic_list0_b[i].flags = flags;
      context->ref_pic_list1_b[i].flags = flags;
      context->slice_param.RefPicList1[i].picture_id = VA_INVALID_SURFACE;
      context->slice_param.RefPicList0[i].picture_id = VA_INVALID_SURFACE;
      context->ref_pic_list0_p[i].picture_id = VA_INVALID_SURFACE;
      context->ref_pic_list0_b[i].picture_id = VA_INVALID_SURFACE;
      context->ref_pic_list1_b[i].picture_id = VA_INVALID_SURFACE;
    }

    for (int i = 0; i < SURFACE_NUM; i++) {
      context->reference_frames[i].picture_id = VA_INVALID_SURFACE;
      context->reference_frames[i].flags = flags;
    }
  }

  if (context->current_frame_type == FRAME_P) {
    memcpy(context->ref_pic_list0_p, context->reference_frames,
           context->num_short_term * sizeof(VAPictureH264));
    sort_one(context->ref_pic_list0_p, 0, context->num_short_term - 1, 0, 1);
  }

  if (context->current_frame_type == FRAME_B) {
    memcpy(context->ref_pic_list0_b, context->reference_frames,
           context->num_short_term * sizeof(VAPictureH264));
    sort_two(context->ref_pic_list0_b, 0, context->num_short_term - 1,
             current_poc, 0, 1, 0, 1);

    memcpy(context->ref_pic_list1_b, context->reference_frames,
           context->num_short_term * sizeof(VAPictureH264));
    sort_two(context->ref_pic_list1_b, 0, context->num_short_term - 1,
             current_poc, 0, 0, 1, 0);
  }

  return 0;
}

template <typename VAEncMiscParam>
VAEncMiscParam& AllocateMiscParameterBuffer(
    std::vector<uint8_t>& misc_buffer,
    VAEncMiscParameterType misc_param_type) {
  constexpr size_t buffer_size =
      sizeof(VAEncMiscParameterBuffer) + sizeof(VAEncMiscParam);
  misc_buffer.resize(buffer_size);
  auto* va_buffer =
      reinterpret_cast<VAEncMiscParameterBuffer*>(misc_buffer.data());
  va_buffer->type = misc_param_type;
  return *reinterpret_cast<VAEncMiscParam*>(va_buffer->data);
}

void CreateVAEncRateControlParams(uint32_t bps,
                                  uint32_t target_percentage,
                                  uint32_t window_size,
                                  uint32_t initial_qp,
                                  uint32_t min_qp,
                                  uint32_t max_qp,
                                  uint32_t framerate,
                                  uint32_t buffer_size,
                                  std::vector<uint8_t> misc_buffers[3]) {
  auto& rate_control_param =
      AllocateMiscParameterBuffer<VAEncMiscParameterRateControl>(
          misc_buffers[0], VAEncMiscParameterTypeRateControl);
  rate_control_param.bits_per_second = bps;
  rate_control_param.target_percentage = target_percentage;
  rate_control_param.window_size = window_size;
  rate_control_param.initial_qp = initial_qp;
  rate_control_param.min_qp = min_qp;
  rate_control_param.max_qp = max_qp;
  rate_control_param.rc_flags.bits.disable_frame_skip = true;

  auto& framerate_param =
      AllocateMiscParameterBuffer<VAEncMiscParameterFrameRate>(
          misc_buffers[1], VAEncMiscParameterTypeFrameRate);
  framerate_param.framerate = framerate;

  auto& hrd_param = AllocateMiscParameterBuffer<VAEncMiscParameterHRD>(
      misc_buffers[2], VAEncMiscParameterTypeHRD);
  hrd_param.buffer_size = buffer_size;
  hrd_param.initial_buffer_fullness = buffer_size / 2;
}

static int render_sequence(VA264Context* context) {
  VABufferID seq_param_buf, rc_param_buf, misc_param_tmpbuf, render_id[2];
  VAStatus va_status;
  VAEncMiscParameterBuffer *misc_param, *misc_param_tmp;
  VAEncMiscParameterRateControl* misc_rate_ctrl;

  context->seq_param.level_idc = 41 /*SH_LEVEL_3*/;
  context->seq_param.picture_width_in_mbs = context->frame_width_mbaligned / 16;
  context->seq_param.picture_height_in_mbs =
      context->frame_height_mbaligned / 16;
  context->seq_param.bits_per_second = context->config.bitrate;

  context->seq_param.intra_period = context->config.intra_period;
  context->seq_param.intra_idr_period = context->config.intra_idr_period;
  context->seq_param.ip_period = context->config.ip_period;

  context->seq_param.max_num_ref_frames = num_ref_frames;
  context->seq_param.seq_fields.bits.frame_mbs_only_flag = 1;
  context->seq_param.time_scale = 900;
  context->seq_param.num_units_in_tick =
      15; /* Tc = num_units_in_tick / time_sacle */
  context->seq_param.seq_fields.bits.log2_max_pic_order_cnt_lsb_minus4 =
      Log2MaxPicOrderCntLsb - 4;
  context->seq_param.seq_fields.bits.log2_max_frame_num_minus4 =
      Log2MaxFrameNum - 4;
  context->seq_param.seq_fields.bits.frame_mbs_only_flag = 1;
  context->seq_param.seq_fields.bits.chroma_format_idc = 1;
  context->seq_param.seq_fields.bits.direct_8x8_inference_flag = 1;

  if (context->config.frame_width != context->frame_width_mbaligned ||
      context->config.frame_height != context->frame_height_mbaligned) {
    context->seq_param.frame_cropping_flag = 1;
    context->seq_param.frame_crop_left_offset = 0;
    context->seq_param.frame_crop_right_offset =
        (context->frame_width_mbaligned - context->config.frame_width) / 2;
    context->seq_param.frame_crop_top_offset = 0;
    context->seq_param.frame_crop_bottom_offset =
        (context->frame_height_mbaligned - context->config.frame_height) / 2;
  }

  va_status = vaCreateBuffer(
      context->va_dpy, context->context_id, VAEncSequenceParameterBufferType,
      sizeof(context->seq_param), 1, &context->seq_param, &seq_param_buf);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
    return -1;
  }

  va_status = vaCreateBuffer(
      context->va_dpy, context->context_id, VAEncMiscParameterBufferType,
      sizeof(VAEncMiscParameterBuffer) + sizeof(VAEncMiscParameterRateControl),
      1, NULL, &rc_param_buf);
  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
    return -1;
  }

  vaMapBuffer(context->va_dpy, rc_param_buf, (void**)&misc_param);
  misc_param->type = VAEncMiscParameterTypeRateControl;
  misc_rate_ctrl = (VAEncMiscParameterRateControl*)misc_param->data;
  memset(misc_rate_ctrl, 0, sizeof(*misc_rate_ctrl));
  misc_rate_ctrl->bits_per_second = context->config.bitrate;
  misc_rate_ctrl->target_percentage = 66;
  misc_rate_ctrl->window_size = 1000;
  misc_rate_ctrl->initial_qp = context->config.initial_qp;
  misc_rate_ctrl->min_qp = context->config.minimal_qp;
  misc_rate_ctrl->basic_unit_size = 0;
  vaUnmapBuffer(context->va_dpy, rc_param_buf);

  render_id[0] = seq_param_buf;
  render_id[1] = rc_param_buf;

  va_status =
      vaRenderPicture(context->va_dpy, context->context_id, &render_id[0], 2);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaRenderPicture failed va_status = " << va_status;
    return -1;
  }

  return 0;
}

std::map<int, std::string> frame_type_map = {{FRAME_P, "P"},
                                             {FRAME_B, "B"},
                                             {FRAME_I, "I"},
                                             {FRAME_IDR, "IDR"}};

static std::string frametype_to_string(int ftype) {
  auto it = frame_type_map.find(ftype);
  if (it != frame_type_map.end()) {
    return it->second;
  }
  return "Unknown";
}

static int calc_poc(VA264Context* context, int pic_order_cnt_lsb) {
  static int PicOrderCntMsb_ref = 0, pic_order_cnt_lsb_ref = 0;
  int prevPicOrderCntMsb, prevPicOrderCntLsb;
  int PicOrderCntMsb, TopFieldOrderCnt;

  if (context->current_frame_type == FRAME_IDR)
    prevPicOrderCntMsb = prevPicOrderCntLsb = 0;
  else {
    prevPicOrderCntMsb = PicOrderCntMsb_ref;
    prevPicOrderCntLsb = pic_order_cnt_lsb_ref;
  }

  if ((pic_order_cnt_lsb < prevPicOrderCntLsb) &&
      ((prevPicOrderCntLsb - pic_order_cnt_lsb) >=
       (int)(MaxPicOrderCntLsb / 2)))
    PicOrderCntMsb = prevPicOrderCntMsb + MaxPicOrderCntLsb;
  else if ((pic_order_cnt_lsb > prevPicOrderCntLsb) &&
           ((pic_order_cnt_lsb - prevPicOrderCntLsb) >
            (int)(MaxPicOrderCntLsb / 2)))
    PicOrderCntMsb = prevPicOrderCntMsb - MaxPicOrderCntLsb;
  else
    PicOrderCntMsb = prevPicOrderCntMsb;

  TopFieldOrderCnt = PicOrderCntMsb + pic_order_cnt_lsb;

  if (context->current_frame_type != FRAME_B) {
    PicOrderCntMsb_ref = PicOrderCntMsb;
    pic_order_cnt_lsb_ref = pic_order_cnt_lsb;
  }

  return TopFieldOrderCnt;
}

static int render_picture(VA264Context* context) {
  VABufferID pic_param_buf;
  VAStatus va_status;
  int i = 0;

  context->pic_param.CurrPic.picture_id =
      context->ref_surface[(context->current_frame_display % SURFACE_NUM)];
  context->pic_param.CurrPic.frame_idx = context->current_frame_num;
  context->pic_param.CurrPic.flags = 0;
  context->pic_param.CurrPic.TopFieldOrderCnt = calc_poc(
      context, (context->current_frame_display - context->current_idr_display) %
                   MaxPicOrderCntLsb);
  context->pic_param.CurrPic.BottomFieldOrderCnt =
      context->pic_param.CurrPic.TopFieldOrderCnt;
  context->current_curr_pic = context->pic_param.CurrPic;

  memcpy(context->pic_param.ReferenceFrames, context->reference_frames,
         context->num_short_term * sizeof(VAPictureH264));
  for (i = context->num_short_term; i < SURFACE_NUM; i++) {
    context->pic_param.ReferenceFrames[i].picture_id = VA_INVALID_SURFACE;
    context->pic_param.ReferenceFrames[i].flags = VA_PICTURE_H264_INVALID;
  }

  context->pic_param.pic_fields.bits.idr_pic_flag =
      (context->current_frame_type == FRAME_IDR);
  context->pic_param.pic_fields.bits.reference_pic_flag =
      (context->current_frame_type != FRAME_B);
  context->pic_param.pic_fields.bits.entropy_coding_mode_flag =
      context->config.h264_entropy_mode;
  context->pic_param.pic_fields.bits.deblocking_filter_control_present_flag = 1;
  context->pic_param.frame_num = context->current_frame_num;
  context->pic_param.coded_buf =
      context->coded_buf[(context->current_frame_display % SURFACE_NUM)];
  context->pic_param.last_picture =
      0;  // (context->current_frame_encoding == frame_count);
  context->pic_param.pic_init_qp = context->config.initial_qp;

  va_status = vaCreateBuffer(
      context->va_dpy, context->context_id, VAEncPictureParameterBufferType,
      sizeof(context->pic_param), 1, &context->pic_param, &pic_param_buf);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
    return -1;
  }

  va_status =
      vaRenderPicture(context->va_dpy, context->context_id, &pic_param_buf, 1);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaRenderPicture failed va_status = " << va_status;
    return -1;
  }

  return 0;
}

static int render_packedsequence(VA264Context* context) {
  VAEncPackedHeaderParameterBuffer packedheader_param_buffer;
  VABufferID packedseq_para_bufid, packedseq_data_bufid, render_id[2];
  uint32_t length_in_bits;
  unsigned char* packedseq_buffer = NULL;
  VAStatus va_status;

  length_in_bits = build_packed_seq_buffer(context, &packedseq_buffer);

  packedheader_param_buffer.type = VAEncPackedHeaderSequence;

  packedheader_param_buffer.bit_length = length_in_bits; /*length_in_bits*/
  packedheader_param_buffer.has_emulation_bytes = 0;
  va_status = vaCreateBuffer(context->va_dpy, context->context_id,
                             VAEncPackedHeaderParameterBufferType,
                             sizeof(packedheader_param_buffer), 1,
                             &packedheader_param_buffer, &packedseq_para_bufid);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
    return -1;
  }

  va_status = vaCreateBuffer(
      context->va_dpy, context->context_id, VAEncPackedHeaderDataBufferType,
      (length_in_bits + 7) / 8, 1, packedseq_buffer, &packedseq_data_bufid);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
    return -1;
  }

  render_id[0] = packedseq_para_bufid;
  render_id[1] = packedseq_data_bufid;
  va_status =
      vaRenderPicture(context->va_dpy, context->context_id, render_id, 2);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaRenderPicture failed va_status = " << va_status;
    return -1;
  }

  free(packedseq_buffer);

  return 0;
}

static int render_packedpicture(VA264Context* context) {
  VAEncPackedHeaderParameterBuffer packedheader_param_buffer;
  VABufferID packedpic_para_bufid, packedpic_data_bufid, render_id[2];
  uint32_t length_in_bits;
  unsigned char* packedpic_buffer = NULL;
  VAStatus va_status;

  length_in_bits = build_packed_pic_buffer(context, &packedpic_buffer);
  packedheader_param_buffer.type = VAEncPackedHeaderPicture;
  packedheader_param_buffer.bit_length = length_in_bits;
  packedheader_param_buffer.has_emulation_bytes = 0;

  va_status = vaCreateBuffer(context->va_dpy, context->context_id,
                             VAEncPackedHeaderParameterBufferType,
                             sizeof(packedheader_param_buffer), 1,
                             &packedheader_param_buffer, &packedpic_para_bufid);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
    return -1;
  }

  va_status = vaCreateBuffer(
      context->va_dpy, context->context_id, VAEncPackedHeaderDataBufferType,
      (length_in_bits + 7) / 8, 1, packedpic_buffer, &packedpic_data_bufid);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
    return -1;
  }
  render_id[0] = packedpic_para_bufid;
  render_id[1] = packedpic_data_bufid;
  va_status =
      vaRenderPicture(context->va_dpy, context->context_id, render_id, 2);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaRenderPicture failed va_status = " << va_status;
    return -1;
  }
  free(packedpic_buffer);

  return 0;
}

static void render_packedslice(VA264Context* context) {
  VAEncPackedHeaderParameterBuffer packedheader_param_buffer;
  VABufferID packedslice_para_bufid, packedslice_data_bufid, render_id[2];
  uint32_t length_in_bits;
  unsigned char* packedslice_buffer = NULL;
  VAStatus va_status;

  length_in_bits = build_packed_slice_buffer(context, &packedslice_buffer);
  packedheader_param_buffer.type = VAEncPackedHeaderSlice;
  packedheader_param_buffer.bit_length = length_in_bits;
  packedheader_param_buffer.has_emulation_bytes = 0;

  va_status = vaCreateBuffer(
      context->va_dpy, context->context_id,
      VAEncPackedHeaderParameterBufferType, sizeof(packedheader_param_buffer),
      1, &packedheader_param_buffer, &packedslice_para_bufid);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
    return;
  }

  va_status = vaCreateBuffer(
      context->va_dpy, context->context_id, VAEncPackedHeaderDataBufferType,
      (length_in_bits + 7) / 8, 1, packedslice_buffer, &packedslice_data_bufid);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
    return;
  }
  render_id[0] = packedslice_para_bufid;
  render_id[1] = packedslice_data_bufid;
  va_status =
      vaRenderPicture(context->va_dpy, context->context_id, render_id, 2);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaRenderPicture failed va_status = " << va_status;
    return;
  }
  free(packedslice_buffer);
}

static int render_slice(VA264Context* context) {
  VABufferID slice_param_buf;
  VAStatus va_status;
  int i;

  update_RefPicList(context);

  /* one frame, one slice */
  context->slice_param.macroblock_address = 0;
  context->slice_param.num_macroblocks = context->frame_width_mbaligned *
                                         context->frame_height_mbaligned /
                                         (16 * 16); /* Measured by MB */
  context->slice_param.slice_type = (context->current_frame_type == FRAME_IDR)
                                        ? 2
                                        : context->current_frame_type;
  if (context->current_frame_type == FRAME_IDR) {
    if (context->current_frame_encoding != 0)
      ++context->slice_param.idr_pic_id;
  } else if (context->current_frame_type == FRAME_P) {
    int refpiclist0_max = context->h264_maxref & 0xffff;
    memcpy(context->slice_param.RefPicList0, context->ref_pic_list0_p,
           ((refpiclist0_max > 32) ? 32 : refpiclist0_max) *
               sizeof(VAPictureH264));

    for (i = refpiclist0_max; i < 32; i++) {
      context->slice_param.RefPicList0[i].picture_id = VA_INVALID_SURFACE;
      context->slice_param.RefPicList0[i].flags = VA_PICTURE_H264_INVALID;
    }
  } else if (context->current_frame_type == FRAME_B) {
    int refpiclist0_max = context->h264_maxref & 0xffff;
    int refpiclist1_max = (context->h264_maxref >> 16) & 0xffff;

    memcpy(context->slice_param.RefPicList0, context->ref_pic_list0_b,
           ((refpiclist0_max > 32) ? 32 : refpiclist0_max) *
               sizeof(VAPictureH264));
    for (i = refpiclist0_max; i < 32; i++) {
      context->slice_param.RefPicList0[i].picture_id = VA_INVALID_SURFACE;
      context->slice_param.RefPicList0[i].flags = VA_PICTURE_H264_INVALID;
    }

    memcpy(context->slice_param.RefPicList1, context->ref_pic_list1_b,
           ((refpiclist1_max > 32) ? 32 : refpiclist1_max) *
               sizeof(VAPictureH264));
    for (i = refpiclist1_max; i < 32; i++) {
      context->slice_param.RefPicList1[i].picture_id = VA_INVALID_SURFACE;
      context->slice_param.RefPicList1[i].flags = VA_PICTURE_H264_INVALID;
    }
  }

  context->slice_param.slice_alpha_c0_offset_div2 = 0;
  context->slice_param.slice_beta_offset_div2 = 0;
  context->slice_param.direct_spatial_mv_pred_flag = 1;
  context->slice_param.pic_order_cnt_lsb =
      (context->current_frame_display - context->current_idr_display) %
      MaxPicOrderCntLsb;

  if (context->h264_packedheader &&
      context->config_attrib[context->enc_packed_header_idx].value &
          VA_ENC_PACKED_HEADER_SLICE)
    render_packedslice(context);

  va_status = vaCreateBuffer(
      context->va_dpy, context->context_id, VAEncSliceParameterBufferType,
      sizeof(context->slice_param), 1, &context->slice_param, &slice_param_buf);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaCreateBuffer failed va_status = " << va_status;
    return -1;
  }

  va_status = vaRenderPicture(context->va_dpy, context->context_id,
                              &slice_param_buf, 1);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaRenderPicture failed va_status = " << va_status;
    return -1;
  }

  return 0;
}

namespace livekit_ffi {

VaapiH264EncoderWrapper::VaapiH264EncoderWrapper()
    : va_display_(std::make_unique<VaapiDisplay>()) {
  context_ = std::make_unique<VA264Context>();
  memset((void*)context_.get(), 0, sizeof(VA264Context));
}

VaapiH264EncoderWrapper::~VaapiH264EncoderWrapper() {}

void VaapiH264EncoderWrapper::Destroy() {
  if (context_->va_dpy) {
    vaDestroySurfaces(context_->va_dpy, &context_->src_surface[0], SURFACE_NUM);
    vaDestroySurfaces(context_->va_dpy, &context_->ref_surface[0], SURFACE_NUM);
  }

  if (context_->encoded_buffer) {
    free(context_->encoded_buffer);
    context_->encoded_buffer = nullptr;
  }

  for (int i = 0; i < SURFACE_NUM; i++) {
    vaDestroyBuffer(context_->va_dpy, context_->coded_buf[i]);
  }

  vaDestroyContext(context_->va_dpy, context_->context_id);
  vaDestroyConfig(context_->va_dpy, context_->config_id);

  if (va_display_->isOpen()) {
    vaTerminate(va_display_->display());
    va_display_->Close();
  }

  context_->va_dpy = nullptr;
  context_->context_id = VA_INVALID_ID;
  memset((void*)context_.get(), 0, sizeof(VA264Context));
  initialized_ = false;
}

bool VaapiH264EncoderWrapper::Initialize(int width,
                                         int height,
                                         int bitrate,
                                         int intra_period,
                                         int idr_period,
                                         int ip_period,
                                         int frame_rate,
                                         VAProfile profile,
                                         int rc_mode) {
  context_->config.h264_entropy_mode = 1;  // cabac
  context_->config.frame_width = width;
  context_->config.frame_height = height;
  context_->config.frame_rate = frame_rate;
  context_->config.bitrate = bitrate;
  context_->config.initial_qp = 26;
  context_->config.minimal_qp = 0;
  context_->config.intra_period = intra_period;
  context_->config.intra_idr_period = idr_period;
  context_->config.ip_period = ip_period;
  context_->config.rc_mode = rc_mode;
  context_->h264_maxref = (1 << 16 | 1);
  context_->requested_entrypoint = context_->selected_entrypoint = -1;

  if (context_->config.ip_period < 1) {
    RTC_LOG(LS_WARNING) << "ip_period must be greater than 0";
    return false;
  }
  if (context_->config.intra_period != 1 &&
      context_->config.intra_period % context_->config.ip_period != 0) {
    RTC_LOG(LS_WARNING) << "intra_period must be a multiplier of ip_period";
    return false;
  }
  if (context_->config.intra_period != 0 &&
      context_->config.intra_idr_period % context_->config.intra_period != 0) {
    RTC_LOG(LS_WARNING)
        << "intra_idr_period must be a multiplier of intra_period";
    return false;
  }

  if (context_->config.bitrate == 0) {
    context_->config.bitrate = context_->config.frame_width *
                               context_->config.frame_height * 12 *
                               context_->config.frame_rate / 50;
  }

  context_->config.h264_profile = profile;

  context_->frame_width_mbaligned = (context_->config.frame_width + 15) & (~15);
  context_->frame_height_mbaligned =
      (context_->config.frame_height + 15) & (~15);
  if (context_->config.frame_width != context_->frame_width_mbaligned ||
      context_->config.frame_height != context_->frame_height_mbaligned) {
    RTC_LOG(LS_INFO) << "Source frame is " << context_->config.frame_width
                     << "x" << context_->config.frame_height
                     << " and will code clip to "
                     << context_->frame_width_mbaligned << "x"
                     << context_->frame_height_mbaligned << " with crop";
  }

  // the buffer to receive the encoded frames from encodeImage
  context_->encoded_buffer = (uint8_t*)malloc(
      context_->frame_width_mbaligned * context_->frame_height_mbaligned * 3);

  if (!va_display_->isOpen()) {
    if (!va_display_->Open()) {
      free(context_->encoded_buffer);
      context_->encoded_buffer = nullptr;
      return false;
    }
  }

  if (init_va(context_.get(), va_display_->display()) != VA_STATUS_SUCCESS) {
    free(context_->encoded_buffer);
    context_->encoded_buffer = nullptr;
    return false;
  }

  if (setup_encode(context_.get()) != VA_STATUS_SUCCESS) {
    free(context_->encoded_buffer);
    context_->encoded_buffer = nullptr;
    return false;
  }

  // reset sps/pps/slice params
  memset(&context_->seq_param, 0, sizeof(context_->seq_param));
  memset(&context_->pic_param, 0, sizeof(context_->pic_param));
  memset(&context_->slice_param, 0, sizeof(context_->slice_param));

  initialized_ = true;
  return true;
}

bool VaapiH264EncoderWrapper::Encode(int fourcc,
                                     const uint8_t* y,
                                     const uint8_t* u,
                                     const uint8_t* v,
                                     bool forceIDR,
                                     std::vector<uint8_t>& encoded) {
  if (forceIDR) {
    // reset the sequence to start with a new IDR regardless of layout
    context_->current_frame_num = context_->current_frame_display =
        context_->current_frame_encoding = 0;
  }

  uint8_t* output = context_->encoded_buffer;
  VASurfaceID surface =
      context_->src_surface[context_->current_frame_encoding % SURFACE_NUM];
  int retv = upload_surface_yuv(
      context_->va_dpy, surface, fourcc, context_->config.frame_width,
      context_->config.frame_height, (uint8_t*)y, (uint8_t*)u, (uint8_t*)v);

  if (retv != 0) {
    RTC_LOG(LS_ERROR) << "Failed to upload surface";
    return false;
  }

  encoding2display_order(
      context_->current_frame_encoding, context_->config.intra_period,
      context_->config.intra_idr_period, context_->config.ip_period,
      &context_->current_frame_display, &context_->current_frame_type);

  if (context_->current_frame_type == FRAME_IDR) {
    context_->num_short_term = 0;
    context_->current_frame_num = 0;
    context_->current_idr_display = context_->current_frame_display;
  }

  VAStatus va_status = vaBeginPicture(
      context_->va_dpy, context_->context_id,
      context_->src_surface[(context_->current_frame_display % SURFACE_NUM)]);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaBeginPicture failed va_status = " << va_status;
    return false;
  }

  // render sequence and picture parameters
  if (context_->current_frame_type == FRAME_IDR) {
    render_sequence(context_.get());
    render_picture(context_.get());
    if (context_->h264_packedheader) {
      render_packedsequence(context_.get());
      render_packedpicture(context_.get());
    }
  } else {
    render_picture(context_.get());
  }
  render_slice(context_.get());

  va_status = vaEndPicture(context_->va_dpy, context_->context_id);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaEndPicture failed va_status = " << va_status;
    return false;
  }
  va_status = vaSyncSurface(
      context_->va_dpy,
      context_->src_surface[context_->current_frame_display % SURFACE_NUM]);

  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaSyncSurface failed va_status = " << va_status;
    return false;
  }
  VACodedBufferSegment* buf_list = NULL;
  uint32_t coded_size = 0;

  va_status = vaMapBuffer(
      context_->va_dpy,
      context_->coded_buf[context_->current_frame_display % SURFACE_NUM],
      (void**)(&buf_list));
  if (va_status != VA_STATUS_SUCCESS) {
    RTC_LOG(LS_ERROR) << "vaMapBuffer failed va_status = " << va_status;
    return false;
  }
  while (buf_list != NULL) {
    memcpy(&output[coded_size], buf_list->buf, buf_list->size);
    coded_size += buf_list->size;
    buf_list = (VACodedBufferSegment*)buf_list->next;
  }

  vaUnmapBuffer(
      context_->va_dpy,
      context_->coded_buf[context_->current_frame_display % SURFACE_NUM]);

  update_ReferenceFrames(context_.get());

  context_->current_frame_encoding++;

  encoded = std::vector<uint8_t>(output, output + coded_size);
  return true;
}

}  // namespace livekit_ffi