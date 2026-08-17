/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_COMMON_CFL_H_
#define AOM_AV1_COMMON_CFL_H_

#include "av1/common/av1_common_int.h"
#include "av1/common/blockd.h"

// Can we use CfL for the current block?
static inline CFL_ALLOWED_TYPE is_cfl_allowed(const MACROBLOCKD *xd) {
  const MB_MODE_INFO *mbmi = xd->mi[0];
  const BLOCK_SIZE bsize = mbmi->bsize;
  assert(bsize < BLOCK_SIZES_ALL);
  if (xd->lossless[mbmi->segment_id]) {
    // In lossless, CfL is available when the partition size is equal to the
    // transform size.
    const int ssx = xd->plane[AOM_PLANE_U].subsampling_x;
    const int ssy = xd->plane[AOM_PLANE_U].subsampling_y;
    const int plane_bsize = get_plane_block_size(bsize, ssx, ssy);
    return (CFL_ALLOWED_TYPE)(plane_bsize == BLOCK_4X4);
  }
  // Spec: CfL is available to luma partitions lesser than or equal to 32x32
  return (CFL_ALLOWED_TYPE)(block_size_wide[bsize] <= 32 &&
                            block_size_high[bsize] <= 32);
}

// Do we need to save the luma pixels from the current block,
// for a possible future CfL prediction?
static inline CFL_ALLOWED_TYPE store_cfl_required(const AV1_COMMON *cm,
                                                  const MACROBLOCKD *xd) {
  const MB_MODE_INFO *mbmi = xd->mi[0];

  if (cm->seq_params->monochrome) return CFL_DISALLOWED;

  if (!xd->is_chroma_ref) {
    // For non-chroma-reference blocks, we should always store the luma pixels,
    // in case the corresponding chroma-reference block uses CfL.
    // Note that this can only happen for block sizes which are <8 on
    // their shortest side, as otherwise they would be chroma reference
    // blocks.
    return CFL_ALLOWED;
  }

  // If this block has chroma information, we know whether we're
  // actually going to perform a CfL prediction
  return (CFL_ALLOWED_TYPE)(!is_inter_block(mbmi) &&
                            mbmi->uv_mode == UV_CFL_PRED);
}

static inline int get_scaled_luma_q0(int alpha_q3, int16_t pred_buf_q3) {
  int scaled_luma_q6 = alpha_q3 * pred_buf_q3;
  return ROUND_POWER_OF_TWO_SIGNED(scaled_luma_q6, 6);
}

static inline CFL_PRED_TYPE get_cfl_pred_type(int plane) {
  assert(plane > 0);
  return (CFL_PRED_TYPE)(plane - 1);
}

static inline void clear_cfl_dc_pred_cache_flags(CFL_CTX *cfl) {
  cfl->use_dc_pred_cache = false;
  cfl->dc_pred_is_cached[CFL_PRED_U] = false;
  cfl->dc_pred_is_cached[CFL_PRED_V] = false;
}

void av1_cfl_predict_block(MACROBLOCKD *const xd, uint8_t *dst, int dst_stride,
                           TX_SIZE tx_size, int plane);

void cfl_store_block(MACROBLOCKD *const xd, BLOCK_SIZE bsize, TX_SIZE tx_size);

void cfl_store_tx(MACROBLOCKD *const xd, int row, int col, TX_SIZE tx_size,
                  BLOCK_SIZE bsize);

void cfl_store_dc_pred(MACROBLOCKD *const xd, const uint8_t *input,
                       CFL_PRED_TYPE pred_plane, int width);

void cfl_load_dc_pred(MACROBLOCKD *const xd, uint8_t *dst, int dst_stride,
                      TX_SIZE tx_size, CFL_PRED_TYPE pred_plane);

// Allows the CFL_SUBSAMPLE function to switch types depending on the bitdepth.
#define CFL_lbd_TYPE uint8_t *cfl_type
#define CFL_hbd_TYPE uint16_t *cfl_type

// Declare a size-specific wrapper for the size-generic function. The compiler
// will inline the size generic function in here, the advantage is that the size
// will be constant allowing for loop unrolling and other constant propagated
// goodness.
#define CFL_SUBSAMPLE(arch, sub, bd, width, height)                       \
  void cfl_subsample_##bd##_##sub##_##width##x##height##_##arch(          \
      const CFL_##bd##_TYPE, int input_stride, uint16_t *output_q3);      \
  void cfl_subsample_##bd##_##sub##_##width##x##height##_##arch(          \
      const CFL_##bd##_TYPE, int input_stride, uint16_t *output_q3) {     \
    cfl_luma_subsampling_##sub##_##bd##_##arch(cfl_type, input_stride,    \
                                               output_q3, width, height); \
  }

// Declare size-specific wrappers for all valid CfL sizes.
#define CFL_SUBSAMPLE_FUNCTIONS(arch, sub, bd)                            \
  CFL_SUBSAMPLE(arch, sub, bd, 4, 4)                                      \
  CFL_SUBSAMPLE(arch, sub, bd, 8, 8)                                      \
  CFL_SUBSAMPLE(arch, sub, bd, 16, 16)                                    \
  CFL_SUBSAMPLE(arch, sub, bd, 32, 32)                                    \
  CFL_SUBSAMPLE(arch, sub, bd, 4, 8)                                      \
  CFL_SUBSAMPLE(arch, sub, bd, 8, 4)                                      \
  CFL_SUBSAMPLE(arch, sub, bd, 8, 16)                                     \
  CFL_SUBSAMPLE(arch, sub, bd, 16, 8)                                     \
  CFL_SUBSAMPLE(arch, sub, bd, 16, 32)                                    \
  CFL_SUBSAMPLE(arch, sub, bd, 32, 16)                                    \
  CFL_SUBSAMPLE(arch, sub, bd, 4, 16)                                     \
  CFL_SUBSAMPLE(arch, sub, bd, 16, 4)                                     \
  CFL_SUBSAMPLE(arch, sub, bd, 8, 32)                                     \
  CFL_SUBSAMPLE(arch, sub, bd, 32, 8)                                     \
  cfl_subsample_##bd##_fn cfl_get_luma_subsampling_##sub##_##bd##_##arch( \
      TX_SIZE tx_size) {                                                  \
    CFL_SUBSAMPLE_FUNCTION_ARRAY(arch, sub, bd)                           \
    return subfn_##sub[tx_size];                                          \
  }

// Declare an architecture-specific array of function pointers for size-specific
// wrappers.
#define CFL_SUBSAMPLE_FUNCTION_ARRAY(arch, sub, bd)                           \
  static const cfl_subsample_##bd##_fn subfn_##sub[TX_SIZES_ALL] = {          \
    cfl_subsample_##bd##_##sub##_4x4_##arch,   /* 4x4 */                      \
    cfl_subsample_##bd##_##sub##_8x8_##arch,   /* 8x8 */                      \
    cfl_subsample_##bd##_##sub##_16x16_##arch, /* 16x16 */                    \
    cfl_subsample_##bd##_##sub##_32x32_##arch, /* 32x32 */                    \
    NULL,                                      /* 64x64 (invalid CFL size) */ \
    cfl_subsample_##bd##_##sub##_4x8_##arch,   /* 4x8 */                      \
    cfl_subsample_##bd##_##sub##_8x4_##arch,   /* 8x4 */                      \
    cfl_subsample_##bd##_##sub##_8x16_##arch,  /* 8x16 */                     \
    cfl_subsample_##bd##_##sub##_16x8_##arch,  /* 16x8 */                     \
    cfl_subsample_##bd##_##sub##_16x32_##arch, /* 16x32 */                    \
    cfl_subsample_##bd##_##sub##_32x16_##arch, /* 32x16 */                    \
    NULL,                                      /* 32x64 (invalid CFL size) */ \
    NULL,                                      /* 64x32 (invalid CFL size) */ \
    cfl_subsample_##bd##_##sub##_4x16_##arch,  /* 4x16  */                    \
    cfl_subsample_##bd##_##sub##_16x4_##arch,  /* 16x4  */                    \
    cfl_subsample_##bd##_##sub##_8x32_##arch,  /* 8x32  */                    \
    cfl_subsample_##bd##_##sub##_32x8_##arch,  /* 32x8  */                    \
    NULL,                                      /* 16x64 (invalid CFL size) */ \
    NULL,                                      /* 64x16 (invalid CFL size) */ \
  };

// The RTCD script does not support passing in an array, so we wrap it in this
// function.
#if CONFIG_AV1_HIGHBITDEPTH
#define CFL_GET_SUBSAMPLE_FUNCTION(arch)  \
  CFL_SUBSAMPLE_FUNCTIONS(arch, 420, lbd) \
  CFL_SUBSAMPLE_FUNCTIONS(arch, 422, lbd) \
  CFL_SUBSAMPLE_FUNCTIONS(arch, 444, lbd) \
  CFL_SUBSAMPLE_FUNCTIONS(arch, 420, hbd) \
  CFL_SUBSAMPLE_FUNCTIONS(arch, 422, hbd) \
  CFL_SUBSAMPLE_FUNCTIONS(arch, 444, hbd)
#else
#define CFL_GET_SUBSAMPLE_FUNCTION(arch)  \
  CFL_SUBSAMPLE_FUNCTIONS(arch, 420, lbd) \
  CFL_SUBSAMPLE_FUNCTIONS(arch, 422, lbd) \
  CFL_SUBSAMPLE_FUNCTIONS(arch, 444, lbd)
#endif

// Declare a size-specific wrapper for the size-generic function. The compiler
// will inline the size generic function in here, the advantage is that the size
// will be constant allowing for loop unrolling and other constant propagated
// goodness.
#define CFL_SUB_AVG_X(arch, width, height, round_offset, num_pel_log2)       \
  void cfl_subtract_average_##width##x##height##_##arch(const uint16_t *src, \
                                                        int16_t *dst);       \
  void cfl_subtract_average_##width##x##height##_##arch(const uint16_t *src, \
                                                        int16_t *dst) {      \
    subtract_average_##arch(src, dst, width, height, round_offset,           \
                            num_pel_log2);                                   \
  }

// Declare size-specific wrappers for all valid CfL sizes.
#define CFL_SUB_AVG_FN(arch)                                              \
  CFL_SUB_AVG_X(arch, 4, 4, 8, 4)                                         \
  CFL_SUB_AVG_X(arch, 4, 8, 16, 5)                                        \
  CFL_SUB_AVG_X(arch, 4, 16, 32, 6)                                       \
  CFL_SUB_AVG_X(arch, 8, 4, 16, 5)                                        \
  CFL_SUB_AVG_X(arch, 8, 8, 32, 6)                                        \
  CFL_SUB_AVG_X(arch, 8, 16, 64, 7)                                       \
  CFL_SUB_AVG_X(arch, 8, 32, 128, 8)                                      \
  CFL_SUB_AVG_X(arch, 16, 4, 32, 6)                                       \
  CFL_SUB_AVG_X(arch, 16, 8, 64, 7)                                       \
  CFL_SUB_AVG_X(arch, 16, 16, 128, 8)                                     \
  CFL_SUB_AVG_X(arch, 16, 32, 256, 9)                                     \
  CFL_SUB_AVG_X(arch, 32, 8, 128, 8)                                      \
  CFL_SUB_AVG_X(arch, 32, 16, 256, 9)                                     \
  CFL_SUB_AVG_X(arch, 32, 32, 512, 10)                                    \
  cfl_subtract_average_fn cfl_get_subtract_average_fn_##arch(             \
      TX_SIZE tx_size) {                                                  \
    static const cfl_subtract_average_fn sub_avg[TX_SIZES_ALL] = {        \
      cfl_subtract_average_4x4_##arch,   /* 4x4 */                        \
      cfl_subtract_average_8x8_##arch,   /* 8x8 */                        \
      cfl_subtract_average_16x16_##arch, /* 16x16 */                      \
      cfl_subtract_average_32x32_##arch, /* 32x32 */                      \
      NULL,                              /* 64x64 (invalid CFL size) */   \
      cfl_subtract_average_4x8_##arch,   /* 4x8 */                        \
      cfl_subtract_average_8x4_##arch,   /* 8x4 */                        \
      cfl_subtract_average_8x16_##arch,  /* 8x16 */                       \
      cfl_subtract_average_16x8_##arch,  /* 16x8 */                       \
      cfl_subtract_average_16x32_##arch, /* 16x32 */                      \
      cfl_subtract_average_32x16_##arch, /* 32x16 */                      \
      NULL,                              /* 32x64 (invalid CFL size) */   \
      NULL,                              /* 64x32 (invalid CFL size) */   \
      cfl_subtract_average_4x16_##arch,  /* 4x16 (invalid CFL size) */    \
      cfl_subtract_average_16x4_##arch,  /* 16x4 (invalid CFL size) */    \
      cfl_subtract_average_8x32_##arch,  /* 8x32 (invalid CFL size) */    \
      cfl_subtract_average_32x8_##arch,  /* 32x8 (invalid CFL size) */    \
      NULL,                              /* 16x64 (invalid CFL size) */   \
      NULL,                              /* 64x16 (invalid CFL size) */   \
    };                                                                    \
    /* Modulo TX_SIZES_ALL to ensure that an attacker won't be able to */ \
    /* index the function pointer array out of bounds. */                 \
    return sub_avg[tx_size % TX_SIZES_ALL];                               \
  }

#define CFL_PREDICT_lbd(arch, width, height)                                   \
  void cfl_predict_lbd_##width##x##height##_##arch(                            \
      const int16_t *pred_buf_q3, uint8_t *dst, int dst_stride, int alpha_q3); \
  void cfl_predict_lbd_##width##x##height##_##arch(                            \
      const int16_t *pred_buf_q3, uint8_t *dst, int dst_stride,                \
      int alpha_q3) {                                                          \
    cfl_predict_lbd_##arch(pred_buf_q3, dst, dst_stride, alpha_q3, width,      \
                           height);                                            \
  }

#if CONFIG_AV1_HIGHBITDEPTH
#define CFL_PREDICT_hbd(arch, width, height)                                   \
  void cfl_predict_hbd_##width##x##height##_##arch(                            \
      const int16_t *pred_buf_q3, uint16_t *dst, int dst_stride, int alpha_q3, \
      int bd);                                                                 \
  void cfl_predict_hbd_##width##x##height##_##arch(                            \
      const int16_t *pred_buf_q3, uint16_t *dst, int dst_stride, int alpha_q3, \
      int bd) {                                                                \
    cfl_predict_hbd_##arch(pred_buf_q3, dst, dst_stride, alpha_q3, bd, width,  \
                           height);                                            \
  }
#endif

// This wrapper exists because clang format does not like calling macros with
// lowercase letters.
#define CFL_PREDICT_X(arch, width, height, bd) \
  CFL_PREDICT_##bd(arch, width, height)

#define CFL_PREDICT_FN(arch, bd)                                            \
  CFL_PREDICT_X(arch, 4, 4, bd)                                             \
  CFL_PREDICT_X(arch, 4, 8, bd)                                             \
  CFL_PREDICT_X(arch, 4, 16, bd)                                            \
  CFL_PREDICT_X(arch, 8, 4, bd)                                             \
  CFL_PREDICT_X(arch, 8, 8, bd)                                             \
  CFL_PREDICT_X(arch, 8, 16, bd)                                            \
  CFL_PREDICT_X(arch, 8, 32, bd)                                            \
  CFL_PREDICT_X(arch, 16, 4, bd)                                            \
  CFL_PREDICT_X(arch, 16, 8, bd)                                            \
  CFL_PREDICT_X(arch, 16, 16, bd)                                           \
  CFL_PREDICT_X(arch, 16, 32, bd)                                           \
  CFL_PREDICT_X(arch, 32, 8, bd)                                            \
  CFL_PREDICT_X(arch, 32, 16, bd)                                           \
  CFL_PREDICT_X(arch, 32, 32, bd)                                           \
  cfl_predict_##bd##_fn cfl_get_predict_##bd##_fn_##arch(TX_SIZE tx_size) { \
    static const cfl_predict_##bd##_fn pred[TX_SIZES_ALL] = {               \
      cfl_predict_##bd##_4x4_##arch,   /* 4x4 */                            \
      cfl_predict_##bd##_8x8_##arch,   /* 8x8 */                            \
      cfl_predict_##bd##_16x16_##arch, /* 16x16 */                          \
      cfl_predict_##bd##_32x32_##arch, /* 32x32 */                          \
      NULL,                            /* 64x64 (invalid CFL size) */       \
      cfl_predict_##bd##_4x8_##arch,   /* 4x8 */                            \
      cfl_predict_##bd##_8x4_##arch,   /* 8x4 */                            \
      cfl_predict_##bd##_8x16_##arch,  /* 8x16 */                           \
      cfl_predict_##bd##_16x8_##arch,  /* 16x8 */                           \
      cfl_predict_##bd##_16x32_##arch, /* 16x32 */                          \
      cfl_predict_##bd##_32x16_##arch, /* 32x16 */                          \
      NULL,                            /* 32x64 (invalid CFL size) */       \
      NULL,                            /* 64x32 (invalid CFL size) */       \
      cfl_predict_##bd##_4x16_##arch,  /* 4x16  */                          \
      cfl_predict_##bd##_16x4_##arch,  /* 16x4  */                          \
      cfl_predict_##bd##_8x32_##arch,  /* 8x32  */                          \
      cfl_predict_##bd##_32x8_##arch,  /* 32x8  */                          \
      NULL,                            /* 16x64 (invalid CFL size) */       \
      NULL,                            /* 64x16 (invalid CFL size) */       \
    };                                                                      \
    /* Modulo TX_SIZES_ALL to ensure that an attacker won't be able to */   \
    /* index the function pointer array out of bounds. */                   \
    return pred[tx_size % TX_SIZES_ALL];                                    \
  }

#endif  // AOM_AV1_COMMON_CFL_H_
