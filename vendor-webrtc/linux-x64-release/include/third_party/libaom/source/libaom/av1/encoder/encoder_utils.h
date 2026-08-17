/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_ENCODER_UTILS_H_
#define AOM_AV1_ENCODER_ENCODER_UTILS_H_

#include "config/aom_dsp_rtcd.h"
#include "config/aom_scale_rtcd.h"

#include "av1/encoder/encoder.h"
#include "av1/encoder/encodetxb.h"

#ifdef __cplusplus
extern "C" {
#endif

#define AM_SEGMENT_ID_INACTIVE 7
#define AM_SEGMENT_ID_ACTIVE 0
#define DUMP_RECON_FRAMES 0

extern const int default_tx_type_probs[FRAME_UPDATE_TYPES][TX_SIZES_ALL]
                                      [TX_TYPES];

extern const int default_obmc_probs[FRAME_UPDATE_TYPES][BLOCK_SIZES_ALL];

extern const int default_warped_probs[FRAME_UPDATE_TYPES];

extern const int default_switchable_interp_probs[FRAME_UPDATE_TYPES]
                                                [SWITCHABLE_FILTER_CONTEXTS]
                                                [SWITCHABLE_FILTERS];

// Mark all inactive blocks as active. Other segmentation features may be set
// so memset cannot be used, instead only inactive blocks should be reset.
static inline void suppress_active_map(AV1_COMP *cpi) {
  unsigned char *const seg_map = cpi->enc_seg.map;
  int i;
  const int num_mis =
      cpi->common.mi_params.mi_rows * cpi->common.mi_params.mi_cols;
  if (cpi->active_map.enabled || cpi->active_map.update)
    for (i = 0; i < num_mis; ++i)
      if (seg_map[i] == AM_SEGMENT_ID_INACTIVE)
        seg_map[i] = AM_SEGMENT_ID_ACTIVE;
}

// Returns 'size' in the number of Mode Info (MI) units. 'size' is either the
// width or height.
static inline int size_in_mi(int size) {
  // Ensure that the decoded width and height are both multiples of
  // 8 luma pixels (note: this may only be a multiple of 4 chroma pixels if
  // subsampling is used).
  // This simplifies the implementation of various experiments,
  // eg. cdef, which operates on units of 8x8 luma pixels.
  const int aligned_size = ALIGN_POWER_OF_TWO(size, 3);
  return aligned_size >> MI_SIZE_LOG2;
}

static inline void set_mb_mi(CommonModeInfoParams *mi_params, int width,
                             int height) {
  mi_params->mi_cols = size_in_mi(width);
  mi_params->mi_rows = size_in_mi(height);
  mi_params->mi_stride = calc_mi_size(mi_params->mi_cols);

  mi_params->mb_cols = ROUND_POWER_OF_TWO(mi_params->mi_cols, 2);
  mi_params->mb_rows = ROUND_POWER_OF_TWO(mi_params->mi_rows, 2);
  mi_params->MBs = mi_params->mb_rows * mi_params->mb_cols;

  const int mi_alloc_size_1d = mi_size_wide[mi_params->mi_alloc_bsize];
  mi_params->mi_alloc_stride =
      (mi_params->mi_stride + mi_alloc_size_1d - 1) / mi_alloc_size_1d;

  assert(mi_size_wide[mi_params->mi_alloc_bsize] ==
         mi_size_high[mi_params->mi_alloc_bsize]);
}

static inline void enc_free_mi(CommonModeInfoParams *mi_params) {
  aom_free(mi_params->mi_alloc);
  mi_params->mi_alloc = NULL;
  mi_params->mi_alloc_size = 0;
  aom_free(mi_params->mi_grid_base);
  mi_params->mi_grid_base = NULL;
  mi_params->mi_grid_size = 0;
  aom_free(mi_params->tx_type_map);
  mi_params->tx_type_map = NULL;
}

static inline void enc_set_mb_mi(CommonModeInfoParams *mi_params, int width,
                                 int height, BLOCK_SIZE min_partition_size) {
  mi_params->mi_alloc_bsize = min_partition_size;

  set_mb_mi(mi_params, width, height);
}

static inline void stat_stage_set_mb_mi(CommonModeInfoParams *mi_params,
                                        int width, int height,
                                        BLOCK_SIZE min_partition_size) {
  (void)min_partition_size;
  mi_params->mi_alloc_bsize = BLOCK_16X16;

  set_mb_mi(mi_params, width, height);
}

static inline void enc_setup_mi(CommonModeInfoParams *mi_params) {
  const int mi_grid_size =
      mi_params->mi_stride * calc_mi_size(mi_params->mi_rows);
  memset(mi_params->mi_alloc, 0,
         mi_params->mi_alloc_size * sizeof(*mi_params->mi_alloc));
  memset(mi_params->mi_grid_base, 0,
         mi_grid_size * sizeof(*mi_params->mi_grid_base));
  memset(mi_params->tx_type_map, 0,
         mi_grid_size * sizeof(*mi_params->tx_type_map));
}

static inline void init_buffer_indices(
    ForceIntegerMVInfo *const force_intpel_info, int *const remapped_ref_idx) {
  int fb_idx;
  for (fb_idx = 0; fb_idx < REF_FRAMES; ++fb_idx)
    remapped_ref_idx[fb_idx] = fb_idx;
  force_intpel_info->rate_index = 0;
  force_intpel_info->rate_size = 0;
}

#define HIGHBD_BFP(BT, SDF, SDAF, VF, SVF, SVAF, SDX4DF, SDX3DF) \
  ppi->fn_ptr[BT].sdf = SDF;                                     \
  ppi->fn_ptr[BT].sdaf = SDAF;                                   \
  ppi->fn_ptr[BT].vf = VF;                                       \
  ppi->fn_ptr[BT].svf = SVF;                                     \
  ppi->fn_ptr[BT].svaf = SVAF;                                   \
  ppi->fn_ptr[BT].sdx4df = SDX4DF;                               \
  ppi->fn_ptr[BT].sdx3df = SDX3DF;

#define HIGHBD_BFP_WRAPPER(WIDTH, HEIGHT, BD)                            \
  HIGHBD_BFP(BLOCK_##WIDTH##X##HEIGHT,                                   \
             aom_highbd_sad##WIDTH##x##HEIGHT##_bits##BD,                \
             aom_highbd_sad##WIDTH##x##HEIGHT##_avg_bits##BD,            \
             aom_highbd_##BD##_variance##WIDTH##x##HEIGHT,               \
             aom_highbd_##BD##_sub_pixel_variance##WIDTH##x##HEIGHT,     \
             aom_highbd_##BD##_sub_pixel_avg_variance##WIDTH##x##HEIGHT, \
             aom_highbd_sad##WIDTH##x##HEIGHT##x4d_bits##BD,             \
             aom_highbd_sad##WIDTH##x##HEIGHT##x3d_bits##BD)

#define HIGHBD_BFP_WRAPPER_NO_SAD_AVG(WIDTH, HEIGHT, BD)                 \
  HIGHBD_BFP(BLOCK_##WIDTH##X##HEIGHT,                                   \
             aom_highbd_sad##WIDTH##x##HEIGHT##_bits##BD, /*SDAF=*/NULL, \
             aom_highbd_##BD##_variance##WIDTH##x##HEIGHT,               \
             aom_highbd_##BD##_sub_pixel_variance##WIDTH##x##HEIGHT,     \
             aom_highbd_##BD##_sub_pixel_avg_variance##WIDTH##x##HEIGHT, \
             aom_highbd_sad##WIDTH##x##HEIGHT##x4d_bits##BD,             \
             aom_highbd_sad##WIDTH##x##HEIGHT##x3d_bits##BD)

#define MAKE_BFP_SAD_WRAPPER(fnname)                                           \
  static unsigned int fnname##_bits8(const uint8_t *src_ptr,                   \
                                     int source_stride,                        \
                                     const uint8_t *ref_ptr, int ref_stride) { \
    return fnname(src_ptr, source_stride, ref_ptr, ref_stride);                \
  }                                                                            \
  static unsigned int fnname##_bits10(                                         \
      const uint8_t *src_ptr, int source_stride, const uint8_t *ref_ptr,       \
      int ref_stride) {                                                        \
    return fnname(src_ptr, source_stride, ref_ptr, ref_stride) >> 2;           \
  }                                                                            \
  static unsigned int fnname##_bits12(                                         \
      const uint8_t *src_ptr, int source_stride, const uint8_t *ref_ptr,       \
      int ref_stride) {                                                        \
    return fnname(src_ptr, source_stride, ref_ptr, ref_stride) >> 4;           \
  }

#define MAKE_BFP_SADAVG_WRAPPER(fnname)                                        \
  static unsigned int fnname##_bits8(                                          \
      const uint8_t *src_ptr, int source_stride, const uint8_t *ref_ptr,       \
      int ref_stride, const uint8_t *second_pred) {                            \
    return fnname(src_ptr, source_stride, ref_ptr, ref_stride, second_pred);   \
  }                                                                            \
  static unsigned int fnname##_bits10(                                         \
      const uint8_t *src_ptr, int source_stride, const uint8_t *ref_ptr,       \
      int ref_stride, const uint8_t *second_pred) {                            \
    return fnname(src_ptr, source_stride, ref_ptr, ref_stride, second_pred) >> \
           2;                                                                  \
  }                                                                            \
  static unsigned int fnname##_bits12(                                         \
      const uint8_t *src_ptr, int source_stride, const uint8_t *ref_ptr,       \
      int ref_stride, const uint8_t *second_pred) {                            \
    return fnname(src_ptr, source_stride, ref_ptr, ref_stride, second_pred) >> \
           4;                                                                  \
  }

#define MAKE_BFP_SAD4D_WRAPPER(fnname)                                        \
  static void fnname##_bits8(const uint8_t *src_ptr, int source_stride,       \
                             const uint8_t *const ref_ptr[], int ref_stride,  \
                             unsigned int *sad_array) {                       \
    fnname(src_ptr, source_stride, ref_ptr, ref_stride, sad_array);           \
  }                                                                           \
  static void fnname##_bits10(const uint8_t *src_ptr, int source_stride,      \
                              const uint8_t *const ref_ptr[], int ref_stride, \
                              unsigned int *sad_array) {                      \
    int i;                                                                    \
    fnname(src_ptr, source_stride, ref_ptr, ref_stride, sad_array);           \
    for (i = 0; i < 4; i++) sad_array[i] >>= 2;                               \
  }                                                                           \
  static void fnname##_bits12(const uint8_t *src_ptr, int source_stride,      \
                              const uint8_t *const ref_ptr[], int ref_stride, \
                              unsigned int *sad_array) {                      \
    int i;                                                                    \
    fnname(src_ptr, source_stride, ref_ptr, ref_stride, sad_array);           \
    for (i = 0; i < 4; i++) sad_array[i] >>= 4;                               \
  }

#if CONFIG_AV1_HIGHBITDEPTH
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad128x128)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad128x128_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad128x128x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad128x128x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad128x64)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad128x64_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad128x64x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad128x64x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad64x128)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad64x128_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad64x128x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad64x128x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad32x16)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad32x16_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad32x16x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad32x16x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad16x32)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad16x32_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad16x32x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad16x32x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad64x32)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad64x32_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad64x32x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad64x32x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad32x64)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad32x64_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad32x64x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad32x64x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad32x32)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad32x32_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad32x32x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad32x32x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad64x64)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad64x64_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad64x64x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad64x64x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad16x16)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad16x16_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad16x16x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad16x16x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad16x8)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad16x8_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad16x8x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad16x8x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad8x16)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad8x16_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad8x16x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad8x16x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad8x8)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad8x8_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad8x8x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad8x8x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad8x4)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad8x4x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad8x4x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad4x8)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad4x8x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad4x8x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad4x4)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad4x4x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad4x4x3d)

#if !CONFIG_REALTIME_ONLY
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad4x16)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad4x16x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad4x16x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad16x4)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad16x4x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad16x4x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad8x32)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad8x32_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad8x32x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad8x32x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad32x8)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad32x8_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad32x8x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad32x8x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad16x64)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad16x64_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad16x64x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad16x64x3d)
MAKE_BFP_SAD_WRAPPER(aom_highbd_sad64x16)
MAKE_BFP_SADAVG_WRAPPER(aom_highbd_sad64x16_avg)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad64x16x4d)
MAKE_BFP_SAD4D_WRAPPER(aom_highbd_sad64x16x3d)
#endif
#endif  // CONFIG_AV1_HIGHBITDEPTH

#define HIGHBD_MBFP(BT, MCSDF, MCSVF) \
  ppi->fn_ptr[BT].msdf = MCSDF;       \
  ppi->fn_ptr[BT].msvf = MCSVF;

#define HIGHBD_MBFP_WRAPPER(WIDTH, HEIGHT, BD)                    \
  HIGHBD_MBFP(BLOCK_##WIDTH##X##HEIGHT,                           \
              aom_highbd_masked_sad##WIDTH##x##HEIGHT##_bits##BD, \
              aom_highbd_##BD##_masked_sub_pixel_variance##WIDTH##x##HEIGHT)

#define MAKE_MBFP_COMPOUND_SAD_WRAPPER(fnname)                           \
  static unsigned int fnname##_bits8(                                    \
      const uint8_t *src_ptr, int source_stride, const uint8_t *ref_ptr, \
      int ref_stride, const uint8_t *second_pred_ptr, const uint8_t *m,  \
      int m_stride, int invert_mask) {                                   \
    return fnname(src_ptr, source_stride, ref_ptr, ref_stride,           \
                  second_pred_ptr, m, m_stride, invert_mask);            \
  }                                                                      \
  static unsigned int fnname##_bits10(                                   \
      const uint8_t *src_ptr, int source_stride, const uint8_t *ref_ptr, \
      int ref_stride, const uint8_t *second_pred_ptr, const uint8_t *m,  \
      int m_stride, int invert_mask) {                                   \
    return fnname(src_ptr, source_stride, ref_ptr, ref_stride,           \
                  second_pred_ptr, m, m_stride, invert_mask) >>          \
           2;                                                            \
  }                                                                      \
  static unsigned int fnname##_bits12(                                   \
      const uint8_t *src_ptr, int source_stride, const uint8_t *ref_ptr, \
      int ref_stride, const uint8_t *second_pred_ptr, const uint8_t *m,  \
      int m_stride, int invert_mask) {                                   \
    return fnname(src_ptr, source_stride, ref_ptr, ref_stride,           \
                  second_pred_ptr, m, m_stride, invert_mask) >>          \
           4;                                                            \
  }

#if CONFIG_AV1_HIGHBITDEPTH
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad128x128)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad128x64)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad64x128)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad64x64)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad64x32)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad32x64)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad32x32)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad32x16)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad16x32)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad16x16)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad16x8)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad8x16)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad8x8)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad8x4)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad4x8)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad4x4)
#if !CONFIG_REALTIME_ONLY
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad4x16)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad16x4)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad8x32)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad32x8)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad16x64)
MAKE_MBFP_COMPOUND_SAD_WRAPPER(aom_highbd_masked_sad64x16)
#endif
#endif

#define HIGHBD_SDSFP(BT, SDSF, SDSX4DF) \
  ppi->fn_ptr[BT].sdsf = SDSF;          \
  ppi->fn_ptr[BT].sdsx4df = SDSX4DF;

#define HIGHBD_SDSFP_WRAPPER(WIDTH, HEIGHT, BD)                   \
  HIGHBD_SDSFP(BLOCK_##WIDTH##X##HEIGHT,                          \
               aom_highbd_sad_skip_##WIDTH##x##HEIGHT##_bits##BD, \
               aom_highbd_sad_skip_##WIDTH##x##HEIGHT##x4d##_bits##BD)

#define MAKE_SDSF_SKIP_SAD_WRAPPER(fnname)                                  \
  static unsigned int fnname##_bits8(const uint8_t *src, int src_stride,    \
                                     const uint8_t *ref, int ref_stride) {  \
    return fnname(src, src_stride, ref, ref_stride);                        \
  }                                                                         \
  static unsigned int fnname##_bits10(const uint8_t *src, int src_stride,   \
                                      const uint8_t *ref, int ref_stride) { \
    return fnname(src, src_stride, ref, ref_stride) >> 2;                   \
  }                                                                         \
  static unsigned int fnname##_bits12(const uint8_t *src, int src_stride,   \
                                      const uint8_t *ref, int ref_stride) { \
    return fnname(src, src_stride, ref, ref_stride) >> 4;                   \
  }

#define MAKE_SDSF_SKIP_SAD_4D_WRAPPER(fnname)                                 \
  static void fnname##_bits8(const uint8_t *src_ptr, int source_stride,       \
                             const uint8_t *const ref_ptr[], int ref_stride,  \
                             unsigned int *sad_array) {                       \
    fnname(src_ptr, source_stride, ref_ptr, ref_stride, sad_array);           \
  }                                                                           \
  static void fnname##_bits10(const uint8_t *src_ptr, int source_stride,      \
                              const uint8_t *const ref_ptr[], int ref_stride, \
                              unsigned int *sad_array) {                      \
    int i;                                                                    \
    fnname(src_ptr, source_stride, ref_ptr, ref_stride, sad_array);           \
    for (i = 0; i < 4; i++) sad_array[i] >>= 2;                               \
  }                                                                           \
  static void fnname##_bits12(const uint8_t *src_ptr, int source_stride,      \
                              const uint8_t *const ref_ptr[], int ref_stride, \
                              unsigned int *sad_array) {                      \
    int i;                                                                    \
    fnname(src_ptr, source_stride, ref_ptr, ref_stride, sad_array);           \
    for (i = 0; i < 4; i++) sad_array[i] >>= 4;                               \
  }

#if CONFIG_AV1_HIGHBITDEPTH
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_128x128)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_128x64)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_64x128)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_64x64)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_64x32)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_32x64)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_32x32)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_32x16)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_16x32)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_16x16)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_8x16)

#if !CONFIG_REALTIME_ONLY
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_64x16)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_16x64)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_4x16)
MAKE_SDSF_SKIP_SAD_WRAPPER(aom_highbd_sad_skip_8x32)
#endif

MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_128x128x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_128x64x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_64x128x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_64x64x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_64x32x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_32x64x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_32x32x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_32x16x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_16x32x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_16x16x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_8x16x4d)

#if !CONFIG_REALTIME_ONLY
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_64x16x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_16x64x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_4x16x4d)
MAKE_SDSF_SKIP_SAD_4D_WRAPPER(aom_highbd_sad_skip_8x32x4d)
#endif
#endif

#if !CONFIG_REALTIME_ONLY

#if CONFIG_AV1_HIGHBITDEPTH
#define HIGHBD_OBFP_WRAPPER_8(WIDTH, HEIGHT)                 \
  HIGHBD_OBFP(BLOCK_##WIDTH##X##HEIGHT,                      \
              aom_highbd_obmc_sad##WIDTH##x##HEIGHT##_bits8, \
              aom_highbd_8_obmc_variance##WIDTH##x##HEIGHT,  \
              aom_highbd_8_obmc_sub_pixel_variance##WIDTH##x##HEIGHT)

#define HIGHBD_OBFP(BT, OSDF, OVF, OSVF) \
  ppi->fn_ptr[BT].osdf = OSDF;           \
  ppi->fn_ptr[BT].ovf = OVF;             \
  ppi->fn_ptr[BT].osvf = OSVF;

#define HIGHBD_OBFP_WRAPPER(WIDTH, HEIGHT, BD)                   \
  HIGHBD_OBFP(BLOCK_##WIDTH##X##HEIGHT,                          \
              aom_highbd_obmc_sad##WIDTH##x##HEIGHT##_bits##BD,  \
              aom_highbd_##BD##_obmc_variance##WIDTH##x##HEIGHT, \
              aom_highbd_##BD##_obmc_sub_pixel_variance##WIDTH##x##HEIGHT)

#define MAKE_OBFP_SAD_WRAPPER(fnname)                                     \
  static unsigned int fnname##_bits8(const uint8_t *ref, int ref_stride,  \
                                     const int32_t *wsrc,                 \
                                     const int32_t *msk) {                \
    return fnname(ref, ref_stride, wsrc, msk);                            \
  }                                                                       \
  static unsigned int fnname##_bits10(const uint8_t *ref, int ref_stride, \
                                      const int32_t *wsrc,                \
                                      const int32_t *msk) {               \
    return fnname(ref, ref_stride, wsrc, msk) >> 2;                       \
  }                                                                       \
  static unsigned int fnname##_bits12(const uint8_t *ref, int ref_stride, \
                                      const int32_t *wsrc,                \
                                      const int32_t *msk) {               \
    return fnname(ref, ref_stride, wsrc, msk) >> 4;                       \
  }
#endif  // CONFIG_AV1_HIGHBITDEPTH
#endif  // !CONFIG_REALTIME_ONLY

#if CONFIG_AV1_HIGHBITDEPTH
#if !CONFIG_REALTIME_ONLY
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad128x128)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad128x64)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad64x128)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad64x64)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad64x32)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad32x64)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad32x32)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad32x16)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad16x32)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad16x16)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad16x8)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad8x16)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad8x8)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad8x4)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad4x8)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad4x4)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad4x16)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad16x4)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad8x32)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad32x8)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad16x64)
MAKE_OBFP_SAD_WRAPPER(aom_highbd_obmc_sad64x16)
#endif

static inline void highbd_set_var_fns(AV1_PRIMARY *const ppi) {
  SequenceHeader *const seq_params = &ppi->seq_params;
  if (seq_params->use_highbitdepth) {
    switch (seq_params->bit_depth) {
      case AOM_BITS_8:
#if !CONFIG_REALTIME_ONLY
        HIGHBD_BFP_WRAPPER(64, 16, 8)
        HIGHBD_BFP_WRAPPER(16, 64, 8)
        HIGHBD_BFP_WRAPPER(32, 8, 8)
        HIGHBD_BFP_WRAPPER(8, 32, 8)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(16, 4, 8)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(4, 16, 8)
#endif
        HIGHBD_BFP_WRAPPER(32, 16, 8)
        HIGHBD_BFP_WRAPPER(16, 32, 8)
        HIGHBD_BFP_WRAPPER(64, 32, 8)
        HIGHBD_BFP_WRAPPER(32, 64, 8)
        HIGHBD_BFP_WRAPPER(32, 32, 8)
        HIGHBD_BFP_WRAPPER(64, 64, 8)
        HIGHBD_BFP_WRAPPER(16, 16, 8)
        HIGHBD_BFP_WRAPPER(16, 8, 8)
        HIGHBD_BFP_WRAPPER(8, 16, 8)
        HIGHBD_BFP_WRAPPER(8, 8, 8)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(8, 4, 8)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(4, 8, 8)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(4, 4, 8)
        HIGHBD_BFP_WRAPPER(128, 128, 8)
        HIGHBD_BFP_WRAPPER(128, 64, 8)
        HIGHBD_BFP_WRAPPER(64, 128, 8)

        HIGHBD_MBFP_WRAPPER(128, 128, 8)
        HIGHBD_MBFP_WRAPPER(128, 64, 8)
        HIGHBD_MBFP_WRAPPER(64, 128, 8)
        HIGHBD_MBFP_WRAPPER(64, 64, 8)
        HIGHBD_MBFP_WRAPPER(64, 32, 8)
        HIGHBD_MBFP_WRAPPER(32, 64, 8)
        HIGHBD_MBFP_WRAPPER(32, 32, 8)
        HIGHBD_MBFP_WRAPPER(32, 16, 8)
        HIGHBD_MBFP_WRAPPER(16, 32, 8)
        HIGHBD_MBFP_WRAPPER(16, 16, 8)
        HIGHBD_MBFP_WRAPPER(8, 16, 8)
        HIGHBD_MBFP_WRAPPER(16, 8, 8)
        HIGHBD_MBFP_WRAPPER(8, 8, 8)
        HIGHBD_MBFP_WRAPPER(4, 8, 8)
        HIGHBD_MBFP_WRAPPER(8, 4, 8)
        HIGHBD_MBFP_WRAPPER(4, 4, 8)
#if !CONFIG_REALTIME_ONLY
        HIGHBD_MBFP_WRAPPER(64, 16, 8)
        HIGHBD_MBFP_WRAPPER(16, 64, 8)
        HIGHBD_MBFP_WRAPPER(32, 8, 8)
        HIGHBD_MBFP_WRAPPER(8, 32, 8)
        HIGHBD_MBFP_WRAPPER(16, 4, 8)
        HIGHBD_MBFP_WRAPPER(4, 16, 8)
#endif

// OBMC excluded from realtime only build.
#if !CONFIG_REALTIME_ONLY
        HIGHBD_OBFP_WRAPPER_8(128, 128)
        HIGHBD_OBFP_WRAPPER_8(128, 64)
        HIGHBD_OBFP_WRAPPER_8(64, 128)
        HIGHBD_OBFP_WRAPPER_8(64, 64)
        HIGHBD_OBFP_WRAPPER_8(64, 32)
        HIGHBD_OBFP_WRAPPER_8(32, 64)
        HIGHBD_OBFP_WRAPPER_8(32, 32)
        HIGHBD_OBFP_WRAPPER_8(32, 16)
        HIGHBD_OBFP_WRAPPER_8(16, 32)
        HIGHBD_OBFP_WRAPPER_8(16, 16)
        HIGHBD_OBFP_WRAPPER_8(8, 16)
        HIGHBD_OBFP_WRAPPER_8(16, 8)
        HIGHBD_OBFP_WRAPPER_8(8, 8)
        HIGHBD_OBFP_WRAPPER_8(4, 8)
        HIGHBD_OBFP_WRAPPER_8(8, 4)
        HIGHBD_OBFP_WRAPPER_8(4, 4)
        HIGHBD_OBFP_WRAPPER_8(64, 16)
        HIGHBD_OBFP_WRAPPER_8(16, 64)
        HIGHBD_OBFP_WRAPPER_8(32, 8)
        HIGHBD_OBFP_WRAPPER_8(8, 32)
        HIGHBD_OBFP_WRAPPER_8(16, 4)
        HIGHBD_OBFP_WRAPPER_8(4, 16)
#endif

        HIGHBD_SDSFP_WRAPPER(128, 128, 8)
        HIGHBD_SDSFP_WRAPPER(128, 64, 8)
        HIGHBD_SDSFP_WRAPPER(64, 128, 8)
        HIGHBD_SDSFP_WRAPPER(64, 64, 8)
        HIGHBD_SDSFP_WRAPPER(64, 32, 8)
        HIGHBD_SDSFP_WRAPPER(32, 64, 8)
        HIGHBD_SDSFP_WRAPPER(32, 32, 8)
        HIGHBD_SDSFP_WRAPPER(32, 16, 8)
        HIGHBD_SDSFP_WRAPPER(16, 32, 8)
        HIGHBD_SDSFP_WRAPPER(16, 16, 8)
        HIGHBD_SDSFP_WRAPPER(8, 16, 8)
#if !CONFIG_REALTIME_ONLY
        HIGHBD_SDSFP_WRAPPER(64, 16, 8)
        HIGHBD_SDSFP_WRAPPER(16, 64, 8)
        HIGHBD_SDSFP_WRAPPER(8, 32, 8)
        HIGHBD_SDSFP_WRAPPER(4, 16, 8)
#endif
        break;

      case AOM_BITS_10:
#if !CONFIG_REALTIME_ONLY
        HIGHBD_BFP_WRAPPER(64, 16, 10)
        HIGHBD_BFP_WRAPPER(16, 64, 10)
        HIGHBD_BFP_WRAPPER(32, 8, 10)
        HIGHBD_BFP_WRAPPER(8, 32, 10)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(16, 4, 10)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(4, 16, 10)
#endif
        HIGHBD_BFP_WRAPPER(32, 16, 10)
        HIGHBD_BFP_WRAPPER(16, 32, 10)
        HIGHBD_BFP_WRAPPER(64, 32, 10)
        HIGHBD_BFP_WRAPPER(32, 64, 10)
        HIGHBD_BFP_WRAPPER(32, 32, 10)
        HIGHBD_BFP_WRAPPER(64, 64, 10)
        HIGHBD_BFP_WRAPPER(16, 16, 10)
        HIGHBD_BFP_WRAPPER(16, 8, 10)
        HIGHBD_BFP_WRAPPER(8, 16, 10)
        HIGHBD_BFP_WRAPPER(8, 8, 10)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(8, 4, 10)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(4, 8, 10)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(4, 4, 10)
        HIGHBD_BFP_WRAPPER(128, 128, 10)
        HIGHBD_BFP_WRAPPER(128, 64, 10)
        HIGHBD_BFP_WRAPPER(64, 128, 10)

        HIGHBD_MBFP_WRAPPER(128, 128, 10)
        HIGHBD_MBFP_WRAPPER(128, 64, 10)
        HIGHBD_MBFP_WRAPPER(64, 128, 10)
        HIGHBD_MBFP_WRAPPER(64, 64, 10)
        HIGHBD_MBFP_WRAPPER(64, 32, 10)
        HIGHBD_MBFP_WRAPPER(32, 64, 10)
        HIGHBD_MBFP_WRAPPER(32, 32, 10)
        HIGHBD_MBFP_WRAPPER(32, 16, 10)
        HIGHBD_MBFP_WRAPPER(16, 32, 10)
        HIGHBD_MBFP_WRAPPER(16, 16, 10)
        HIGHBD_MBFP_WRAPPER(8, 16, 10)
        HIGHBD_MBFP_WRAPPER(16, 8, 10)
        HIGHBD_MBFP_WRAPPER(8, 8, 10)
        HIGHBD_MBFP_WRAPPER(4, 8, 10)
        HIGHBD_MBFP_WRAPPER(8, 4, 10)
        HIGHBD_MBFP_WRAPPER(4, 4, 10)
#if !CONFIG_REALTIME_ONLY
        HIGHBD_MBFP_WRAPPER(64, 16, 10)
        HIGHBD_MBFP_WRAPPER(16, 64, 10)
        HIGHBD_MBFP_WRAPPER(32, 8, 10)
        HIGHBD_MBFP_WRAPPER(8, 32, 10)
        HIGHBD_MBFP_WRAPPER(16, 4, 10)
        HIGHBD_MBFP_WRAPPER(4, 16, 10)
#endif

// OBMC excluded from realtime only build.
#if !CONFIG_REALTIME_ONLY
        HIGHBD_OBFP_WRAPPER(128, 128, 10)
        HIGHBD_OBFP_WRAPPER(128, 64, 10)
        HIGHBD_OBFP_WRAPPER(64, 128, 10)
        HIGHBD_OBFP_WRAPPER(64, 64, 10)
        HIGHBD_OBFP_WRAPPER(64, 32, 10)
        HIGHBD_OBFP_WRAPPER(32, 64, 10)
        HIGHBD_OBFP_WRAPPER(32, 32, 10)
        HIGHBD_OBFP_WRAPPER(32, 16, 10)
        HIGHBD_OBFP_WRAPPER(16, 32, 10)
        HIGHBD_OBFP_WRAPPER(16, 16, 10)
        HIGHBD_OBFP_WRAPPER(8, 16, 10)
        HIGHBD_OBFP_WRAPPER(16, 8, 10)
        HIGHBD_OBFP_WRAPPER(8, 8, 10)
        HIGHBD_OBFP_WRAPPER(4, 8, 10)
        HIGHBD_OBFP_WRAPPER(8, 4, 10)
        HIGHBD_OBFP_WRAPPER(4, 4, 10)
        HIGHBD_OBFP_WRAPPER(64, 16, 10)
        HIGHBD_OBFP_WRAPPER(16, 64, 10)
        HIGHBD_OBFP_WRAPPER(32, 8, 10)
        HIGHBD_OBFP_WRAPPER(8, 32, 10)
        HIGHBD_OBFP_WRAPPER(16, 4, 10)
        HIGHBD_OBFP_WRAPPER(4, 16, 10)
#endif

        HIGHBD_SDSFP_WRAPPER(128, 128, 10)
        HIGHBD_SDSFP_WRAPPER(128, 64, 10)
        HIGHBD_SDSFP_WRAPPER(64, 128, 10)
        HIGHBD_SDSFP_WRAPPER(64, 64, 10)
        HIGHBD_SDSFP_WRAPPER(64, 32, 10)
        HIGHBD_SDSFP_WRAPPER(32, 64, 10)
        HIGHBD_SDSFP_WRAPPER(32, 32, 10)
        HIGHBD_SDSFP_WRAPPER(32, 16, 10)
        HIGHBD_SDSFP_WRAPPER(16, 32, 10)
        HIGHBD_SDSFP_WRAPPER(16, 16, 10)
        HIGHBD_SDSFP_WRAPPER(8, 16, 10)

#if !CONFIG_REALTIME_ONLY
        HIGHBD_SDSFP_WRAPPER(64, 16, 10)
        HIGHBD_SDSFP_WRAPPER(16, 64, 10)
        HIGHBD_SDSFP_WRAPPER(8, 32, 10)
        HIGHBD_SDSFP_WRAPPER(4, 16, 10)
#endif
        break;

      case AOM_BITS_12:
#if !CONFIG_REALTIME_ONLY
        HIGHBD_BFP_WRAPPER(64, 16, 12)
        HIGHBD_BFP_WRAPPER(16, 64, 12)
        HIGHBD_BFP_WRAPPER(32, 8, 12)
        HIGHBD_BFP_WRAPPER(8, 32, 12)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(16, 4, 12)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(4, 16, 12)
#endif
        HIGHBD_BFP_WRAPPER(32, 16, 12)
        HIGHBD_BFP_WRAPPER(16, 32, 12)
        HIGHBD_BFP_WRAPPER(64, 32, 12)
        HIGHBD_BFP_WRAPPER(32, 64, 12)
        HIGHBD_BFP_WRAPPER(32, 32, 12)
        HIGHBD_BFP_WRAPPER(64, 64, 12)
        HIGHBD_BFP_WRAPPER(16, 16, 12)
        HIGHBD_BFP_WRAPPER(16, 8, 12)
        HIGHBD_BFP_WRAPPER(8, 16, 12)
        HIGHBD_BFP_WRAPPER(8, 8, 12)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(8, 4, 12)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(4, 8, 12)
        HIGHBD_BFP_WRAPPER_NO_SAD_AVG(4, 4, 12)
        HIGHBD_BFP_WRAPPER(128, 128, 12)
        HIGHBD_BFP_WRAPPER(128, 64, 12)
        HIGHBD_BFP_WRAPPER(64, 128, 12)

        HIGHBD_MBFP_WRAPPER(128, 128, 12)
        HIGHBD_MBFP_WRAPPER(128, 64, 12)
        HIGHBD_MBFP_WRAPPER(64, 128, 12)
        HIGHBD_MBFP_WRAPPER(64, 64, 12)
        HIGHBD_MBFP_WRAPPER(64, 32, 12)
        HIGHBD_MBFP_WRAPPER(32, 64, 12)
        HIGHBD_MBFP_WRAPPER(32, 32, 12)
        HIGHBD_MBFP_WRAPPER(32, 16, 12)
        HIGHBD_MBFP_WRAPPER(16, 32, 12)
        HIGHBD_MBFP_WRAPPER(16, 16, 12)
        HIGHBD_MBFP_WRAPPER(8, 16, 12)
        HIGHBD_MBFP_WRAPPER(16, 8, 12)
        HIGHBD_MBFP_WRAPPER(8, 8, 12)
        HIGHBD_MBFP_WRAPPER(4, 8, 12)
        HIGHBD_MBFP_WRAPPER(8, 4, 12)
        HIGHBD_MBFP_WRAPPER(4, 4, 12)
#if !CONFIG_REALTIME_ONLY
        HIGHBD_MBFP_WRAPPER(64, 16, 12)
        HIGHBD_MBFP_WRAPPER(16, 64, 12)
        HIGHBD_MBFP_WRAPPER(32, 8, 12)
        HIGHBD_MBFP_WRAPPER(8, 32, 12)
        HIGHBD_MBFP_WRAPPER(16, 4, 12)
        HIGHBD_MBFP_WRAPPER(4, 16, 12)
#endif

// OBMC excluded from realtime only build.
#if !CONFIG_REALTIME_ONLY
        HIGHBD_OBFP_WRAPPER(128, 128, 12)
        HIGHBD_OBFP_WRAPPER(128, 64, 12)
        HIGHBD_OBFP_WRAPPER(64, 128, 12)
        HIGHBD_OBFP_WRAPPER(64, 64, 12)
        HIGHBD_OBFP_WRAPPER(64, 32, 12)
        HIGHBD_OBFP_WRAPPER(32, 64, 12)
        HIGHBD_OBFP_WRAPPER(32, 32, 12)
        HIGHBD_OBFP_WRAPPER(32, 16, 12)
        HIGHBD_OBFP_WRAPPER(16, 32, 12)
        HIGHBD_OBFP_WRAPPER(16, 16, 12)
        HIGHBD_OBFP_WRAPPER(8, 16, 12)
        HIGHBD_OBFP_WRAPPER(16, 8, 12)
        HIGHBD_OBFP_WRAPPER(8, 8, 12)
        HIGHBD_OBFP_WRAPPER(4, 8, 12)
        HIGHBD_OBFP_WRAPPER(8, 4, 12)
        HIGHBD_OBFP_WRAPPER(4, 4, 12)
        HIGHBD_OBFP_WRAPPER(64, 16, 12)
        HIGHBD_OBFP_WRAPPER(16, 64, 12)
        HIGHBD_OBFP_WRAPPER(32, 8, 12)
        HIGHBD_OBFP_WRAPPER(8, 32, 12)
        HIGHBD_OBFP_WRAPPER(16, 4, 12)
        HIGHBD_OBFP_WRAPPER(4, 16, 12)
#endif

        HIGHBD_SDSFP_WRAPPER(128, 128, 12)
        HIGHBD_SDSFP_WRAPPER(128, 64, 12)
        HIGHBD_SDSFP_WRAPPER(64, 128, 12)
        HIGHBD_SDSFP_WRAPPER(64, 64, 12)
        HIGHBD_SDSFP_WRAPPER(64, 32, 12)
        HIGHBD_SDSFP_WRAPPER(32, 64, 12)
        HIGHBD_SDSFP_WRAPPER(32, 32, 12)
        HIGHBD_SDSFP_WRAPPER(32, 16, 12)
        HIGHBD_SDSFP_WRAPPER(16, 32, 12)
        HIGHBD_SDSFP_WRAPPER(16, 16, 12)
        HIGHBD_SDSFP_WRAPPER(8, 16, 12)

#if !CONFIG_REALTIME_ONLY
        HIGHBD_SDSFP_WRAPPER(64, 16, 12)
        HIGHBD_SDSFP_WRAPPER(16, 64, 12)
        HIGHBD_SDSFP_WRAPPER(8, 32, 12)
        HIGHBD_SDSFP_WRAPPER(4, 16, 12)
#endif
        break;

      default:
        assert(0 &&
               "cm->seq_params->bit_depth should be AOM_BITS_8, "
               "AOM_BITS_10 or AOM_BITS_12");
    }
  }
}
#endif  // CONFIG_AV1_HIGHBITDEPTH

static inline void copy_frame_prob_info(AV1_COMP *cpi) {
  FrameProbInfo *const frame_probs = &cpi->ppi->frame_probs;
  if (cpi->sf.tx_sf.tx_type_search.prune_tx_type_using_stats) {
    av1_copy(frame_probs->tx_type_probs, default_tx_type_probs);
  }
  if (cpi->sf.inter_sf.prune_obmc_prob_thresh > 0 &&
      cpi->sf.inter_sf.prune_obmc_prob_thresh < INT_MAX) {
    av1_copy(frame_probs->obmc_probs, default_obmc_probs);
  }
  if (cpi->sf.inter_sf.prune_warped_prob_thresh > 0) {
    av1_copy(frame_probs->warped_probs, default_warped_probs);
  }
  if (cpi->sf.interp_sf.adaptive_interp_filter_search == 2) {
    av1_copy(frame_probs->switchable_interp_probs,
             default_switchable_interp_probs);
  }

#if CONFIG_FPMT_TEST
  if (cpi->ppi->fpmt_unit_test_cfg == PARALLEL_SIMULATION_ENCODE) {
    FrameProbInfo *const temp_frame_probs = &cpi->ppi->temp_frame_probs;
    if (cpi->sf.tx_sf.tx_type_search.prune_tx_type_using_stats) {
      av1_copy(temp_frame_probs->tx_type_probs, default_tx_type_probs);
    }
    if (cpi->sf.inter_sf.prune_obmc_prob_thresh > 0 &&
        cpi->sf.inter_sf.prune_obmc_prob_thresh < INT_MAX) {
      av1_copy(temp_frame_probs->obmc_probs, default_obmc_probs);
    }
    if (cpi->sf.inter_sf.prune_warped_prob_thresh > 0) {
      av1_copy(temp_frame_probs->warped_probs, default_warped_probs);
    }
    if (cpi->sf.interp_sf.adaptive_interp_filter_search == 2) {
      av1_copy(temp_frame_probs->switchable_interp_probs,
               default_switchable_interp_probs);
    }

    FrameProbInfo *const temp_frame_probs_simulation =
        &cpi->ppi->temp_frame_probs_simulation;
    if (cpi->sf.tx_sf.tx_type_search.prune_tx_type_using_stats) {
      av1_copy(temp_frame_probs_simulation->tx_type_probs,
               default_tx_type_probs);
    }
    if (cpi->sf.inter_sf.prune_obmc_prob_thresh > 0 &&
        cpi->sf.inter_sf.prune_obmc_prob_thresh < INT_MAX) {
      av1_copy(temp_frame_probs_simulation->obmc_probs, default_obmc_probs);
    }
    if (cpi->sf.inter_sf.prune_warped_prob_thresh > 0) {
      av1_copy(temp_frame_probs_simulation->warped_probs, default_warped_probs);
    }
    if (cpi->sf.interp_sf.adaptive_interp_filter_search == 2) {
      av1_copy(temp_frame_probs_simulation->switchable_interp_probs,
               default_switchable_interp_probs);
    }
  }
#endif
}

static inline void restore_cdef_coding_context(CdefInfo *const dst,
                                               const CdefInfo *const src) {
  dst->cdef_bits = src->cdef_bits;
  dst->cdef_damping = src->cdef_damping;
  av1_copy(dst->cdef_strengths, src->cdef_strengths);
  av1_copy(dst->cdef_uv_strengths, src->cdef_uv_strengths);
  dst->nb_cdef_strengths = src->nb_cdef_strengths;
}

// Coding context that only needs to be restored when recode loop includes
// filtering (deblocking, CDEF, superres post-encode upscale and/or loop
// restoraton).
static inline void restore_extra_coding_context(AV1_COMP *cpi) {
  CODING_CONTEXT *const cc = &cpi->coding_context;
  AV1_COMMON *cm = &cpi->common;
  cm->lf = cc->lf;
  restore_cdef_coding_context(&cm->cdef_info, &cc->cdef_info);
  cpi->rc = cc->rc;
  cpi->ppi->mv_stats = cc->mv_stats;
}

static inline int equal_dimensions_and_border(const YV12_BUFFER_CONFIG *a,
                                              const YV12_BUFFER_CONFIG *b) {
  return a->y_height == b->y_height && a->y_width == b->y_width &&
         a->uv_height == b->uv_height && a->uv_width == b->uv_width &&
         a->y_stride == b->y_stride && a->uv_stride == b->uv_stride &&
         a->border == b->border &&
         (a->flags & YV12_FLAG_HIGHBITDEPTH) ==
             (b->flags & YV12_FLAG_HIGHBITDEPTH);
}

static inline int update_entropy(bool *ext_refresh_frame_context,
                                 bool *ext_refresh_frame_context_pending,
                                 bool update) {
  *ext_refresh_frame_context = update;
  *ext_refresh_frame_context_pending = 1;
  return 0;
}

#if !CONFIG_REALTIME_ONLY
static inline int combine_prior_with_tpl_boost(double min_factor,
                                               double max_factor,
                                               int prior_boost, int tpl_boost,
                                               int frames_to_key) {
  double factor = sqrt((double)frames_to_key);
  double range = max_factor - min_factor;
  factor = AOMMIN(factor, max_factor);
  factor = AOMMAX(factor, min_factor);
  factor -= min_factor;
  int boost =
      (int)((factor * prior_boost + (range - factor) * tpl_boost) / range);
  return boost;
}
#endif

static inline void set_size_independent_vars(AV1_COMP *cpi) {
  int i;
  AV1_COMMON *const cm = &cpi->common;
  FeatureFlags *const features = &cm->features;
  for (i = LAST_FRAME; i <= ALTREF_FRAME; ++i) {
    cm->global_motion[i] = default_warp_params;
  }
  cpi->gm_info.search_done = 0;

  av1_set_speed_features_framesize_independent(cpi, cpi->speed);
  av1_set_rd_speed_thresholds(cpi);
  features->interp_filter = SWITCHABLE;
  features->switchable_motion_mode = is_switchable_motion_mode_allowed(
      features->allow_warped_motion, cpi->oxcf.motion_mode_cfg.enable_obmc);
}

static inline void release_scaled_references(AV1_COMP *cpi) {
  // Scaled references should only need to be released under certain conditions:
  // if the reference will be updated, or if the scaled reference has same
  // resolution. For now only apply this to Golden for non-svc RTC mode.
  AV1_COMMON *const cm = &cpi->common;
  const bool refresh_golden = (cpi->refresh_frame.golden_frame) ? 1 : 0;
  bool release_golden = true;
  for (int i = 0; i < INTER_REFS_PER_FRAME; ++i) {
    RefCntBuffer *const buf = cpi->scaled_ref_buf[i];
    const int golden_ref = (i == GOLDEN_FRAME - 1);
    if (golden_ref && is_one_pass_rt_params(cpi) && !cpi->ppi->use_svc &&
        buf != NULL) {
      const RefCntBuffer *const ref = get_ref_frame_buf(cm, GOLDEN_FRAME);
      const bool same_resoln = buf->buf.y_crop_width == ref->buf.y_crop_width &&
                               buf->buf.y_crop_height == ref->buf.y_crop_height;
      release_golden = refresh_golden || same_resoln;
    }
    if (buf != NULL && (!golden_ref || (golden_ref && release_golden))) {
      --buf->ref_count;
      cpi->scaled_ref_buf[i] = NULL;
    }
  }
}

static inline void restore_all_coding_context(AV1_COMP *cpi) {
  restore_extra_coding_context(cpi);
  if (!frame_is_intra_only(&cpi->common)) release_scaled_references(cpi);
}

static inline int reduce_num_ref_buffers(const AV1_COMP *cpi) {
  const SequenceHeader *const seq_params = cpi->common.seq_params;
  return is_one_pass_rt_params(cpi) &&
         use_rtc_reference_structure_one_layer(cpi) &&
         (seq_params->order_hint_info.enable_order_hint == 0) &&
         cpi->rt_reduce_num_ref_buffers;
}

// Refresh reference frame buffers according to refresh_frame_flags.
static inline void refresh_reference_frames(AV1_COMP *cpi) {
  AV1_COMMON *const cm = &cpi->common;
  // All buffers are refreshed for shown keyframes and S-frames.
  // In case of RT, golden frame refreshes the 6th slot and other reference
  // frames refresh slots 0 to 5. Slot 7 is not refreshed by any reference
  // frame. Thus, only 7 buffers are refreshed for keyframes and S-frames
  // instead of 8.
  int num_ref_buffers = REF_FRAMES;
  if (reduce_num_ref_buffers(cpi)) {
    const int refresh_all_bufs =
        (cpi->ppi->gf_group.refbuf_state[cpi->gf_frame_index] == REFBUF_RESET ||
         frame_is_sframe(cm));
    assert(IMPLIES(((cm->current_frame.refresh_frame_flags >> 7) & 1) == 1,
                   refresh_all_bufs));
    (void)refresh_all_bufs;
    num_ref_buffers--;
  }

  for (int ref_frame = 0; ref_frame < num_ref_buffers; ref_frame++) {
    if (((cm->current_frame.refresh_frame_flags >> ref_frame) & 1) == 1) {
      assign_frame_buffer_p(&cm->ref_frame_map[ref_frame], cm->cur_frame);
    }
  }
}

#if !CONFIG_REALTIME_ONLY
void av1_update_film_grain_parameters_seq(struct AV1_PRIMARY *ppi,
                                          const AV1EncoderConfig *oxcf);
void av1_update_film_grain_parameters(struct AV1_COMP *cpi,
                                      const AV1EncoderConfig *oxcf);
#endif

void av1_scale_references(AV1_COMP *cpi, const InterpFilter filter,
                          const int phase, const int use_optimized_scaler);

void av1_setup_frame(AV1_COMP *cpi);

BLOCK_SIZE av1_select_sb_size(const AV1EncoderConfig *const oxcf, int width,
                              int height, int number_spatial_layers);

void av1_apply_active_map(AV1_COMP *cpi);

#if !CONFIG_REALTIME_ONLY
uint16_t av1_setup_interp_filter_search_mask(AV1_COMP *cpi);

void av1_determine_sc_tools_with_encoding(AV1_COMP *cpi, const int q_orig);
#endif

void av1_set_size_dependent_vars(AV1_COMP *cpi, int *q, int *bottom_index,
                                 int *top_index);

void av1_finalize_encoded_frame(AV1_COMP *const cpi);

int av1_is_integer_mv(const YV12_BUFFER_CONFIG *cur_picture,
                      const YV12_BUFFER_CONFIG *last_picture,
                      ForceIntegerMVInfo *const force_intpel_info);

void av1_set_mb_ssim_rdmult_scaling(AV1_COMP *cpi);

void av1_save_all_coding_context(AV1_COMP *cpi);

#if DUMP_RECON_FRAMES == 1
void av1_dump_filtered_recon_frames(AV1_COMP *cpi);
#endif

static inline int av1_get_enc_border_size(bool resize, bool all_intra,
                                          BLOCK_SIZE sb_size) {
  // For allintra encoding mode, inter-frame motion search is not applicable and
  // the intraBC motion vectors are restricted within the tile boundaries. Hence
  // a smaller frame border size (AOM_ENC_ALLINTRA_BORDER) is used in this case.
  if (resize) {
    return AOM_BORDER_IN_PIXELS;
  }
  if (all_intra) {
    return AOM_ENC_ALLINTRA_BORDER;
  }
  return block_size_wide[sb_size] + 32;
}

static inline bool av1_is_resize_needed(const AV1EncoderConfig *oxcf) {
  const ResizeCfg *resize_cfg = &oxcf->resize_cfg;
  const SuperResCfg *superres_cfg = &oxcf->superres_cfg;
  return resize_cfg->resize_mode || superres_cfg->superres_mode;
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_ENCODER_UTILS_H_
