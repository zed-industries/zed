/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_THIRDPASS_H_
#define AOM_AV1_ENCODER_THIRDPASS_H_

#include "av1/common/enums.h"
#ifdef __cplusplus
extern "C" {
#endif

#include "av1/encoder/firstpass.h"
#include "av1/encoder/ratectrl.h"
#include "av1/encoder/tpl_model.h"

struct AV1_COMP;

// TODO(bohanli): optimize this number
#define MAX_THIRD_PASS_BUF \
  (AOMMAX((2 * MAX_GF_INTERVAL + 1), MAX_STATIC_GF_GROUP_LENGTH))

// Struct to store useful information related to a GOP, in addition to what is
// available in the bitstream
typedef struct {
  int gf_length;
  int num_frames;
  int use_arf;
} THIRD_PASS_GOP_INFO;

#if CONFIG_BITRATE_ACCURACY
typedef struct TPL_INFO {
  int gf_length;
  int tpl_ready;
  TplTxfmStats txfm_stats_list[MAX_LENGTH_TPL_FRAME_STATS];
  double qstep_ratio_ls[MAX_LENGTH_TPL_FRAME_STATS];
  FRAME_UPDATE_TYPE update_type_list[MAX_LENGTH_TPL_FRAME_STATS];
} TPL_INFO;
#endif  // CONFIG_BITRATE_ACCURACY

typedef struct {
  BLOCK_SIZE bsize;
  PARTITION_TYPE partition;
  int mi_row_start;
  int mi_col_start;
  int_mv mv[2];
  MV_REFERENCE_FRAME ref_frame[2];
  PREDICTION_MODE pred_mode;
} THIRD_PASS_MI_INFO;

// Struct to store useful information about a frame for the third pass.
// The members are extracted from the decoder by function get_frame_info.
typedef struct {
  int width;
  int height;
  int mi_stride;
  int mi_rows;
  int mi_cols;
  int base_q_idx;
  int is_show_existing_frame;
  int is_show_frame;
  int bits_allocated;
  int actual_bits;
  uint64_t sse;
  double bpm_factor;
  FRAME_TYPE frame_type;
  unsigned int order_hint;
  THIRD_PASS_MI_INFO *mi_info;
} THIRD_PASS_FRAME_INFO;

typedef struct {
  /* --- Input and decoding related members --- */
  // the input file
  const char *input_file_name;
#if CONFIG_THREE_PASS
  // input context
  struct AvxInputContext *input_ctx;
#endif
  // decoder codec context
  aom_codec_ctx_t decoder;
  // start of the frame in buf
  const unsigned char *frame;
  // end of the frame(s) in buf
  const unsigned char *end_frame;
  // whether we still have following frames in buf
  int have_frame;
  // pointer to buffer for the read frames
  uint8_t *buf;
  // size of data in buffer
  size_t bytes_in_buffer;
  // current buffer size
  size_t buffer_size;
  // error info pointer
  struct aom_internal_error_info *err_info;

  int this_frame_bits;

  /* --- Members for third pass encoding --- */
  // Array to store info about each frame.
  // frame_info[0] should point to the current frame.
  THIRD_PASS_FRAME_INFO frame_info[MAX_THIRD_PASS_BUF];
  // number of frames available in frame_info
  int frame_info_count;
  // the end of the previous GOP (order hint)
  int prev_gop_end;
  THIRD_PASS_GOP_INFO gop_info;
} THIRD_PASS_DEC_CTX;

void av1_init_thirdpass_ctx(AV1_COMMON *cm, THIRD_PASS_DEC_CTX **ctx,
                            const char *file);
void av1_free_thirdpass_ctx(THIRD_PASS_DEC_CTX *ctx);

// Set the GOP structure from the twopass bitstream.
// TODO(bohanli): this is currently a skeleton and we only return the gop
// length. This function also saves all frame information in the array
// ctx->frame_info for this GOP.
void av1_set_gop_third_pass(THIRD_PASS_DEC_CTX *ctx);

// Pop one frame out of the array ctx->frame_info. This function is used to make
// sure that frame_info[0] always corresponds to the current frame.
void av1_pop_third_pass_info(THIRD_PASS_DEC_CTX *ctx);

void av1_open_second_pass_log(struct AV1_COMP *cpi, int is_read);
void av1_close_second_pass_log(struct AV1_COMP *cpi);

// Write the current GOP information into the second pass log file.
void av1_write_second_pass_gop_info(struct AV1_COMP *cpi);
// Write the information of the frames in this GOP into the second pass log
// file.
void av1_write_second_pass_per_frame_info(struct AV1_COMP *cpi, int gf_index);

// Read the next GOP information from the second pass log file.
void av1_read_second_pass_gop_info(FILE *second_pass_log_stream,
                                   THIRD_PASS_GOP_INFO *gop_info,
                                   struct aom_internal_error_info *error);
// read the information of the frames in next GOP from the second pass log file.
void av1_read_second_pass_per_frame_info(FILE *second_pass_log_stream,
                                         THIRD_PASS_FRAME_INFO *frame_info_arr,
                                         int frame_info_count,
                                         struct aom_internal_error_info *error);

int av1_check_use_arf(THIRD_PASS_DEC_CTX *ctx);

// Calculate the ratio of third pass frame dimensions over second pass frame
// dimensions. Return them in ratio_h and ratio_w.
void av1_get_third_pass_ratio(THIRD_PASS_DEC_CTX *ctx, int fidx, int fheight,
                              int fwidth, double *ratio_h, double *ratio_w);

// Get the pointer to a second pass mi info, where mi_row and mi_col are the mi
// location in the thirdpass frame.
THIRD_PASS_MI_INFO *av1_get_third_pass_mi(THIRD_PASS_DEC_CTX *ctx, int fidx,
                                          int mi_row, int mi_col,
                                          double ratio_h, double ratio_w);

// Get the adjusted MVs of this_mi, associated with the reference frame. If no
// MV is found with the reference frame, INVALID_MV is returned.
int_mv av1_get_third_pass_adjusted_mv(THIRD_PASS_MI_INFO *this_mi,
                                      double ratio_h, double ratio_w,
                                      MV_REFERENCE_FRAME frame);

// Get the adjusted block size of this_mi.
BLOCK_SIZE av1_get_third_pass_adjusted_blk_size(THIRD_PASS_MI_INFO *this_mi,
                                                double ratio_h, double ratio_w);

// Get the adjusted mi position in the third pass frame, of a given
// third_pass_mi. Location is returned in mi_row and mi_col.
void av1_third_pass_get_adjusted_mi(THIRD_PASS_MI_INFO *third_pass_mi,
                                    double ratio_h, double ratio_w, int *mi_row,
                                    int *mi_col);

PARTITION_TYPE av1_third_pass_get_sb_part_type(THIRD_PASS_DEC_CTX *ctx,
                                               THIRD_PASS_MI_INFO *this_mi);

#if CONFIG_BITRATE_ACCURACY

void av1_pack_tpl_info(TPL_INFO *tpl_info, const GF_GROUP *gf_group,
                       const TplParams *tpl_data);

void av1_write_tpl_info(const TPL_INFO *tpl_info, FILE *log_stream,
                        struct aom_internal_error_info *error);

void av1_read_tpl_info(TPL_INFO *tpl_info, FILE *log_stream,
                       struct aom_internal_error_info *error);

#endif  // CONFIG_BITRATE_ACCURACY
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_THIRDPASS_H_
