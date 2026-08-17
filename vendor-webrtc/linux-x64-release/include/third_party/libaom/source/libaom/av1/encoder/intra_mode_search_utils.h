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

/*!\file
 * \brief Defines utility functions used in intra mode search.
 *
 * This includes rdcost estimations, histogram based pruning, etc.
 */
#ifndef AOM_AV1_ENCODER_INTRA_MODE_SEARCH_UTILS_H_
#define AOM_AV1_ENCODER_INTRA_MODE_SEARCH_UTILS_H_

#include "av1/common/enums.h"
#include "av1/common/pred_common.h"
#include "av1/common/reconintra.h"

#include "av1/encoder/encoder.h"
#include "av1/encoder/encodeframe.h"
#include "av1/encoder/model_rd.h"
#include "av1/encoder/palette.h"
#include "av1/encoder/hybrid_fwd_txfm.h"

#ifdef __cplusplus
extern "C" {
#endif

/*!\cond */
// Macro for computing the speed-preset dependent threshold which is used for
// deciding whether to enable/disable variance calculations in
// intra_rd_variance_factor().
#define INTRA_RD_VAR_THRESH(X) (1.0 - (0.25 * (X)))

#define BINS 32
static const float av1_intra_hog_model_bias[DIRECTIONAL_MODES] = {
  0.450578f,  0.695518f,  -0.717944f, -0.639894f,
  -0.602019f, -0.453454f, 0.055857f,  -0.465480f,
};

static const float av1_intra_hog_model_weights[BINS * DIRECTIONAL_MODES] = {
  -3.076402f, -3.757063f, -3.275266f, -3.180665f, -3.452105f, -3.216593f,
  -2.871212f, -3.134296f, -1.822324f, -2.401411f, -1.541016f, -1.195322f,
  -0.434156f, 0.322868f,  2.260546f,  3.368715f,  3.989290f,  3.308487f,
  2.277893f,  0.923793f,  0.026412f,  -0.385174f, -0.718622f, -1.408867f,
  -1.050558f, -2.323941f, -2.225827f, -2.585453f, -3.054283f, -2.875087f,
  -2.985709f, -3.447155f, 3.758139f,  3.204353f,  2.170998f,  0.826587f,
  -0.269665f, -0.702068f, -1.085776f, -2.175249f, -1.623180f, -2.975142f,
  -2.779629f, -3.190799f, -3.521900f, -3.375480f, -3.319355f, -3.897389f,
  -3.172334f, -3.594528f, -2.879132f, -2.547777f, -2.921023f, -2.281844f,
  -1.818988f, -2.041771f, -0.618268f, -1.396458f, -0.567153f, -0.285868f,
  -0.088058f, 0.753494f,  2.092413f,  3.215266f,  -3.300277f, -2.748658f,
  -2.315784f, -2.423671f, -2.257283f, -2.269583f, -2.196660f, -2.301076f,
  -2.646516f, -2.271319f, -2.254366f, -2.300102f, -2.217960f, -2.473300f,
  -2.116866f, -2.528246f, -3.314712f, -1.701010f, -0.589040f, -0.088077f,
  0.813112f,  1.702213f,  2.653045f,  3.351749f,  3.243554f,  3.199409f,
  2.437856f,  1.468854f,  0.533039f,  -0.099065f, -0.622643f, -2.200732f,
  -4.228861f, -2.875263f, -1.273956f, -0.433280f, 0.803771f,  1.975043f,
  3.179528f,  3.939064f,  3.454379f,  3.689386f,  3.116411f,  1.970991f,
  0.798406f,  -0.628514f, -1.252546f, -2.825176f, -4.090178f, -3.777448f,
  -3.227314f, -3.479403f, -3.320569f, -3.159372f, -2.729202f, -2.722341f,
  -3.054913f, -2.742923f, -2.612703f, -2.662632f, -2.907314f, -3.117794f,
  -3.102660f, -3.970972f, -4.891357f, -3.935582f, -3.347758f, -2.721924f,
  -2.219011f, -1.702391f, -0.866529f, -0.153743f, 0.107733f,  1.416882f,
  2.572884f,  3.607755f,  3.974820f,  3.997783f,  2.970459f,  0.791687f,
  -1.478921f, -1.228154f, -1.216955f, -1.765932f, -1.951003f, -1.985301f,
  -1.975881f, -1.985593f, -2.422371f, -2.419978f, -2.531288f, -2.951853f,
  -3.071380f, -3.277027f, -3.373539f, -4.462010f, -0.967888f, 0.805524f,
  2.794130f,  3.685984f,  3.745195f,  3.252444f,  2.316108f,  1.399146f,
  -0.136519f, -0.162811f, -1.004357f, -1.667911f, -1.964662f, -2.937579f,
  -3.019533f, -3.942766f, -5.102767f, -3.882073f, -3.532027f, -3.451956f,
  -2.944015f, -2.643064f, -2.529872f, -2.077290f, -2.809965f, -1.803734f,
  -1.783593f, -1.662585f, -1.415484f, -1.392673f, -0.788794f, -1.204819f,
  -1.998864f, -1.182102f, -0.892110f, -1.317415f, -1.359112f, -1.522867f,
  -1.468552f, -1.779072f, -2.332959f, -2.160346f, -2.329387f, -2.631259f,
  -2.744936f, -3.052494f, -2.787363f, -3.442548f, -4.245075f, -3.032172f,
  -2.061609f, -1.768116f, -1.286072f, -0.706587f, -0.192413f, 0.386938f,
  0.716997f,  1.481393f,  2.216702f,  2.737986f,  3.109809f,  3.226084f,
  2.490098f,  -0.095827f, -3.864816f, -3.507248f, -3.128925f, -2.908251f,
  -2.883836f, -2.881411f, -2.524377f, -2.624478f, -2.399573f, -2.367718f,
  -1.918255f, -1.926277f, -1.694584f, -1.723790f, -0.966491f, -1.183115f,
  -1.430687f, 0.872896f,  2.766550f,  3.610080f,  3.578041f,  3.334928f,
  2.586680f,  1.895721f,  1.122195f,  0.488519f,  -0.140689f, -0.799076f,
  -1.222860f, -1.502437f, -1.900969f, -3.206816f,
};

static const NN_CONFIG av1_intra_hog_model_nnconfig = {
  BINS,               // num_inputs
  DIRECTIONAL_MODES,  // num_outputs
  0,                  // num_hidden_layers
  { 0 },
  {
      av1_intra_hog_model_weights,
  },
  {
      av1_intra_hog_model_bias,
  },
};

#define FIX_PREC_BITS (16)
static inline int get_hist_bin_idx(int dx, int dy) {
  const int32_t ratio = (dy * (1 << FIX_PREC_BITS)) / dx;

  // Find index by bisection
  static const int thresholds[BINS] = {
    -1334015, -441798, -261605, -183158, -138560, -109331, -88359, -72303,
    -59392,   -48579,  -39272,  -30982,  -23445,  -16400,  -9715,  -3194,
    3227,     9748,    16433,   23478,   31015,   39305,   48611,  59425,
    72336,    88392,   109364,  138593,  183191,  261638,  441831, INT32_MAX
  };

  int lo_idx = 0, hi_idx = BINS - 1;
  // Divide into segments of size 8 gives better performance than binary search
  // here.
  if (ratio <= thresholds[7]) {
    lo_idx = 0;
    hi_idx = 7;
  } else if (ratio <= thresholds[15]) {
    lo_idx = 8;
    hi_idx = 15;
  } else if (ratio <= thresholds[23]) {
    lo_idx = 16;
    hi_idx = 23;
  } else {
    lo_idx = 24;
    hi_idx = 31;
  }

  for (int idx = lo_idx; idx <= hi_idx; idx++) {
    if (ratio <= thresholds[idx]) {
      return idx;
    }
  }
  assert(0 && "No valid histogram bin found!");
  return BINS - 1;
}
#undef FIX_PREC_BITS

// Normalizes the hog data.
static inline void normalize_hog(float total, float *hist) {
  for (int i = 0; i < BINS; ++i) hist[i] /= total;
}

static inline void lowbd_generate_hog(const uint8_t *src, int stride, int rows,
                                      int cols, float *hist) {
  float total = 0.1f;
  src += stride;
  for (int r = 1; r < rows - 1; ++r) {
    for (int c = 1; c < cols - 1; ++c) {
      const uint8_t *above = &src[c - stride];
      const uint8_t *below = &src[c + stride];
      const uint8_t *left = &src[c - 1];
      const uint8_t *right = &src[c + 1];
      // Calculate gradient using Sobel filters.
      const int dx = (right[-stride] + 2 * right[0] + right[stride]) -
                     (left[-stride] + 2 * left[0] + left[stride]);
      const int dy = (below[-1] + 2 * below[0] + below[1]) -
                     (above[-1] + 2 * above[0] + above[1]);
      if (dx == 0 && dy == 0) continue;
      const int temp = abs(dx) + abs(dy);
      if (!temp) continue;
      total += temp;
      if (dx == 0) {
        hist[0] += temp / 2;
        hist[BINS - 1] += temp / 2;
      } else {
        const int idx = get_hist_bin_idx(dx, dy);
        assert(idx >= 0 && idx < BINS);
        hist[idx] += temp;
      }
    }
    src += stride;
  }

  normalize_hog(total, hist);
}

// Computes and stores pixel level gradient information of a given superblock
// for LBD encode.
static inline void lowbd_compute_gradient_info_sb(MACROBLOCK *const x,
                                                  BLOCK_SIZE sb_size,
                                                  PLANE_TYPE plane) {
  PixelLevelGradientInfo *const grad_info_sb =
      x->pixel_gradient_info + plane * MAX_SB_SQUARE;
  const uint8_t *src = x->plane[plane].src.buf;
  const int stride = x->plane[plane].src.stride;
  const int ss_x = x->e_mbd.plane[plane].subsampling_x;
  const int ss_y = x->e_mbd.plane[plane].subsampling_y;
  const int sb_height = block_size_high[sb_size] >> ss_y;
  const int sb_width = block_size_wide[sb_size] >> ss_x;
  src += stride;
  for (int r = 1; r < sb_height - 1; ++r) {
    for (int c = 1; c < sb_width - 1; ++c) {
      const uint8_t *above = &src[c - stride];
      const uint8_t *below = &src[c + stride];
      const uint8_t *left = &src[c - 1];
      const uint8_t *right = &src[c + 1];
      // Calculate gradient using Sobel filters.
      const int dx = (right[-stride] + 2 * right[0] + right[stride]) -
                     (left[-stride] + 2 * left[0] + left[stride]);
      const int dy = (below[-1] + 2 * below[0] + below[1]) -
                     (above[-1] + 2 * above[0] + above[1]);
      grad_info_sb[r * sb_width + c].is_dx_zero = (dx == 0);
      grad_info_sb[r * sb_width + c].abs_dx_abs_dy_sum =
          (uint16_t)(abs(dx) + abs(dy));
      grad_info_sb[r * sb_width + c].hist_bin_idx =
          (dx != 0) ? get_hist_bin_idx(dx, dy) : -1;
    }
    src += stride;
  }
}

#if CONFIG_AV1_HIGHBITDEPTH
static inline void highbd_generate_hog(const uint8_t *src8, int stride,
                                       int rows, int cols, float *hist) {
  float total = 0.1f;
  const uint16_t *src = CONVERT_TO_SHORTPTR(src8);
  src += stride;
  for (int r = 1; r < rows - 1; ++r) {
    for (int c = 1; c < cols - 1; ++c) {
      const uint16_t *above = &src[c - stride];
      const uint16_t *below = &src[c + stride];
      const uint16_t *left = &src[c - 1];
      const uint16_t *right = &src[c + 1];
      // Calculate gradient using Sobel filters.
      const int dx = (right[-stride] + 2 * right[0] + right[stride]) -
                     (left[-stride] + 2 * left[0] + left[stride]);
      const int dy = (below[-1] + 2 * below[0] + below[1]) -
                     (above[-1] + 2 * above[0] + above[1]);
      if (dx == 0 && dy == 0) continue;
      const int temp = abs(dx) + abs(dy);
      if (!temp) continue;
      total += temp;
      if (dx == 0) {
        hist[0] += temp / 2;
        hist[BINS - 1] += temp / 2;
      } else {
        const int idx = get_hist_bin_idx(dx, dy);
        assert(idx >= 0 && idx < BINS);
        hist[idx] += temp;
      }
    }
    src += stride;
  }

  normalize_hog(total, hist);
}

// Computes and stores pixel level gradient information of a given superblock
// for HBD encode.
static inline void highbd_compute_gradient_info_sb(MACROBLOCK *const x,
                                                   BLOCK_SIZE sb_size,
                                                   PLANE_TYPE plane) {
  PixelLevelGradientInfo *const grad_info_sb =
      x->pixel_gradient_info + plane * MAX_SB_SQUARE;
  const uint16_t *src = CONVERT_TO_SHORTPTR(x->plane[plane].src.buf);
  const int stride = x->plane[plane].src.stride;
  const int ss_x = x->e_mbd.plane[plane].subsampling_x;
  const int ss_y = x->e_mbd.plane[plane].subsampling_y;
  const int sb_height = block_size_high[sb_size] >> ss_y;
  const int sb_width = block_size_wide[sb_size] >> ss_x;
  src += stride;
  for (int r = 1; r < sb_height - 1; ++r) {
    for (int c = 1; c < sb_width - 1; ++c) {
      const uint16_t *above = &src[c - stride];
      const uint16_t *below = &src[c + stride];
      const uint16_t *left = &src[c - 1];
      const uint16_t *right = &src[c + 1];
      // Calculate gradient using Sobel filters.
      const int dx = (right[-stride] + 2 * right[0] + right[stride]) -
                     (left[-stride] + 2 * left[0] + left[stride]);
      const int dy = (below[-1] + 2 * below[0] + below[1]) -
                     (above[-1] + 2 * above[0] + above[1]);
      grad_info_sb[r * sb_width + c].is_dx_zero = (dx == 0);
      grad_info_sb[r * sb_width + c].abs_dx_abs_dy_sum =
          (uint16_t)(abs(dx) + abs(dy));
      grad_info_sb[r * sb_width + c].hist_bin_idx =
          (dx != 0) ? get_hist_bin_idx(dx, dy) : -1;
    }
    src += stride;
  }
}
#endif  // CONFIG_AV1_HIGHBITDEPTH

static inline void generate_hog(const uint8_t *src8, int stride, int rows,
                                int cols, float *hist, int highbd) {
#if CONFIG_AV1_HIGHBITDEPTH
  if (highbd) {
    highbd_generate_hog(src8, stride, rows, cols, hist);
    return;
  }
#else
  (void)highbd;
#endif  // CONFIG_AV1_HIGHBITDEPTH
  lowbd_generate_hog(src8, stride, rows, cols, hist);
}

static inline void compute_gradient_info_sb(MACROBLOCK *const x,
                                            BLOCK_SIZE sb_size,
                                            PLANE_TYPE plane) {
#if CONFIG_AV1_HIGHBITDEPTH
  if (is_cur_buf_hbd(&x->e_mbd)) {
    highbd_compute_gradient_info_sb(x, sb_size, plane);
    return;
  }
#endif  // CONFIG_AV1_HIGHBITDEPTH
  lowbd_compute_gradient_info_sb(x, sb_size, plane);
}

// Gradient caching at superblock level is allowed only if all of the following
// conditions are satisfied:
// (1) The current frame is an intra only frame
// (2) Non-RD mode decisions are not enabled
// (3) The sf partition_search_type is set to SEARCH_PARTITION
// (4) Either intra_pruning_with_hog or chroma_intra_pruning_with_hog is enabled
//
// SB level caching of gradient data may not help in speedup for the following
// cases:
// (1) Inter frames (due to early intra gating)
// (2) When partition_search_type is not SEARCH_PARTITION
// Hence, gradient data is computed at block level in such cases.
static inline bool is_gradient_caching_for_hog_enabled(
    const AV1_COMP *const cpi) {
  const SPEED_FEATURES *const sf = &cpi->sf;
  return frame_is_intra_only(&cpi->common) && !sf->rt_sf.use_nonrd_pick_mode &&
         (sf->part_sf.partition_search_type == SEARCH_PARTITION) &&
         (sf->intra_sf.intra_pruning_with_hog ||
          sf->intra_sf.chroma_intra_pruning_with_hog);
}

// Function to generate pixel level gradient information for a given superblock.
// Sets the flags 'is_sb_gradient_cached' for the specific plane-type if
// gradient info is generated for the same.
static inline void produce_gradients_for_sb(AV1_COMP *cpi, MACROBLOCK *x,
                                            BLOCK_SIZE sb_size, int mi_row,
                                            int mi_col) {
  // Initialise flags related to hog data caching.
  x->is_sb_gradient_cached[PLANE_TYPE_Y] = false;
  x->is_sb_gradient_cached[PLANE_TYPE_UV] = false;
  if (!is_gradient_caching_for_hog_enabled(cpi)) return;

  const SPEED_FEATURES *sf = &cpi->sf;
  const int num_planes = av1_num_planes(&cpi->common);

  av1_setup_src_planes(x, cpi->source, mi_row, mi_col, num_planes, sb_size);

  if (sf->intra_sf.intra_pruning_with_hog) {
    compute_gradient_info_sb(x, sb_size, PLANE_TYPE_Y);
    x->is_sb_gradient_cached[PLANE_TYPE_Y] = true;
  }
  if (sf->intra_sf.chroma_intra_pruning_with_hog && num_planes > 1) {
    compute_gradient_info_sb(x, sb_size, PLANE_TYPE_UV);
    x->is_sb_gradient_cached[PLANE_TYPE_UV] = true;
  }
}

// Reuses the pixel level gradient data generated at superblock level for block
// level histogram computation.
static inline void generate_hog_using_gradient_cache(const MACROBLOCK *x,
                                                     int rows, int cols,
                                                     BLOCK_SIZE sb_size,
                                                     PLANE_TYPE plane,
                                                     float *hist) {
  float total = 0.1f;
  const int ss_x = x->e_mbd.plane[plane].subsampling_x;
  const int ss_y = x->e_mbd.plane[plane].subsampling_y;
  const int sb_width = block_size_wide[sb_size] >> ss_x;

  // Derive the offset from the starting of the superblock in order to locate
  // the block level gradient data in the cache.
  const int mi_row_in_sb = x->e_mbd.mi_row & (mi_size_high[sb_size] - 1);
  const int mi_col_in_sb = x->e_mbd.mi_col & (mi_size_wide[sb_size] - 1);
  const int block_offset_in_grad_cache =
      sb_width * (mi_row_in_sb << (MI_SIZE_LOG2 - ss_y)) +
      (mi_col_in_sb << (MI_SIZE_LOG2 - ss_x));
  const PixelLevelGradientInfo *grad_info_blk = x->pixel_gradient_info +
                                                plane * MAX_SB_SQUARE +
                                                block_offset_in_grad_cache;

  // Retrieve the cached gradient information and generate the histogram.
  for (int r = 1; r < rows - 1; ++r) {
    for (int c = 1; c < cols - 1; ++c) {
      const uint16_t abs_dx_abs_dy_sum =
          grad_info_blk[r * sb_width + c].abs_dx_abs_dy_sum;
      if (!abs_dx_abs_dy_sum) continue;
      total += abs_dx_abs_dy_sum;
      const bool is_dx_zero = grad_info_blk[r * sb_width + c].is_dx_zero;
      if (is_dx_zero) {
        hist[0] += abs_dx_abs_dy_sum >> 1;
        hist[BINS - 1] += abs_dx_abs_dy_sum >> 1;
      } else {
        const int8_t idx = grad_info_blk[r * sb_width + c].hist_bin_idx;
        assert(idx >= 0 && idx < BINS);
        hist[idx] += abs_dx_abs_dy_sum;
      }
    }
  }
  normalize_hog(total, hist);
}

static inline void collect_hog_data(const MACROBLOCK *x, BLOCK_SIZE bsize,
                                    BLOCK_SIZE sb_size, int plane, float *hog) {
  const MACROBLOCKD *xd = &x->e_mbd;
  const struct macroblockd_plane *const pd = &xd->plane[plane];
  const int ss_x = pd->subsampling_x;
  const int ss_y = pd->subsampling_y;
  const int bh = block_size_high[bsize];
  const int bw = block_size_wide[bsize];
  const int rows =
      ((xd->mb_to_bottom_edge >= 0) ? bh : (xd->mb_to_bottom_edge >> 3) + bh) >>
      ss_y;
  const int cols =
      ((xd->mb_to_right_edge >= 0) ? bw : (xd->mb_to_right_edge >> 3) + bw) >>
      ss_x;

  // If gradient data is already generated at SB level, reuse the cached data.
  // Otherwise, compute the data.
  if (x->is_sb_gradient_cached[plane]) {
    generate_hog_using_gradient_cache(x, rows, cols, sb_size, plane, hog);
  } else {
    const uint8_t *src = x->plane[plane].src.buf;
    const int src_stride = x->plane[plane].src.stride;
    generate_hog(src, src_stride, rows, cols, hog, is_cur_buf_hbd(xd));
  }

  // Scale the hog so the luma and chroma are on the same scale
  for (int b = 0; b < BINS; ++b) {
    hog[b] *= (1 + ss_x) * (1 + ss_y);
  }
}

static inline void prune_intra_mode_with_hog(
    const MACROBLOCK *x, BLOCK_SIZE bsize, BLOCK_SIZE sb_size, float th,
    uint8_t *directional_mode_skip_mask, int is_chroma) {
  const int plane = is_chroma ? AOM_PLANE_U : AOM_PLANE_Y;
  float hist[BINS] = { 0.0f };
  collect_hog_data(x, bsize, sb_size, plane, hist);

  // Make prediction for each of the mode
  float scores[DIRECTIONAL_MODES] = { 0.0f };
  av1_nn_predict(hist, &av1_intra_hog_model_nnconfig, 1, scores);
  for (UV_PREDICTION_MODE uv_mode = UV_V_PRED; uv_mode <= UV_D67_PRED;
       uv_mode++) {
    if (scores[uv_mode - UV_V_PRED] <= th) {
      directional_mode_skip_mask[uv_mode] = 1;
    }
  }
}
#undef BINS

int av1_calc_normalized_variance(aom_variance_fn_t vf, const uint8_t *const buf,
                                 const int stride, const int is_hbd);

// Returns whether caching of source variance for 4x4 sub-blocks is allowed.
static inline bool is_src_var_for_4x4_sub_blocks_caching_enabled(
    const AV1_COMP *const cpi) {
  const SPEED_FEATURES *const sf = &cpi->sf;
  if (cpi->oxcf.mode != ALLINTRA) return false;

  if (sf->part_sf.partition_search_type == SEARCH_PARTITION) return true;

  if (INTRA_RD_VAR_THRESH(cpi->oxcf.speed) <= 0 ||
      (sf->rt_sf.use_nonrd_pick_mode && !sf->rt_sf.hybrid_intra_pickmode))
    return false;

  return true;
}

// Initialize the members of Block4x4VarInfo structure to -1 at the start
// of every superblock.
static inline void init_src_var_info_of_4x4_sub_blocks(
    const AV1_COMP *const cpi, Block4x4VarInfo *src_var_info_of_4x4_sub_blocks,
    const BLOCK_SIZE sb_size) {
  if (!is_src_var_for_4x4_sub_blocks_caching_enabled(cpi)) return;

  const int mi_count_in_sb = mi_size_wide[sb_size] * mi_size_high[sb_size];
  for (int i = 0; i < mi_count_in_sb; i++) {
    src_var_info_of_4x4_sub_blocks[i].var = -1;
    src_var_info_of_4x4_sub_blocks[i].log_var = -1.0;
  }
}

// Returns the cost needed to send a uniformly distributed r.v.
static inline int write_uniform_cost(int n, int v) {
  const int l = get_unsigned_bits(n);
  const int m = (1 << l) - n;
  if (l == 0) return 0;
  if (v < m)
    return av1_cost_literal(l - 1);
  else
    return av1_cost_literal(l);
}
/*!\endcond */

/*!\brief Returns the rate cost for luma prediction mode info of intra blocks.
 *
 * \callergraph
 */
static inline int intra_mode_info_cost_y(const AV1_COMP *cpi,
                                         const MACROBLOCK *x,
                                         const MB_MODE_INFO *mbmi,
                                         BLOCK_SIZE bsize, int mode_cost,
                                         int discount_color_cost) {
  int total_rate = mode_cost;
  const ModeCosts *mode_costs = &x->mode_costs;
  const int use_palette = mbmi->palette_mode_info.palette_size[0] > 0;
  const int use_filter_intra = mbmi->filter_intra_mode_info.use_filter_intra;
  const int use_intrabc = mbmi->use_intrabc;
  // Can only activate one mode.
  assert(((mbmi->mode != DC_PRED) + use_palette + use_intrabc +
          use_filter_intra) <= 1);
  const int try_palette = av1_allow_palette(
      cpi->common.features.allow_screen_content_tools, mbmi->bsize);
  if (try_palette && mbmi->mode == DC_PRED) {
    const MACROBLOCKD *xd = &x->e_mbd;
    const int bsize_ctx = av1_get_palette_bsize_ctx(bsize);
    const int mode_ctx = av1_get_palette_mode_ctx(xd);
    total_rate +=
        mode_costs->palette_y_mode_cost[bsize_ctx][mode_ctx][use_palette];
    if (use_palette) {
      const uint8_t *const color_map = xd->plane[0].color_index_map;
      int block_width, block_height, rows, cols;
      av1_get_block_dimensions(bsize, 0, xd, &block_width, &block_height, &rows,
                               &cols);
      const int plt_size = mbmi->palette_mode_info.palette_size[0];
      int palette_mode_cost =
          mode_costs
              ->palette_y_size_cost[bsize_ctx][plt_size - PALETTE_MIN_SIZE] +
          write_uniform_cost(plt_size, color_map[0]);
      uint16_t color_cache[2 * PALETTE_MAX_SIZE];
      const int n_cache = av1_get_palette_cache(xd, 0, color_cache);
      palette_mode_cost +=
          av1_palette_color_cost_y(&mbmi->palette_mode_info, color_cache,
                                   n_cache, cpi->common.seq_params->bit_depth);
      if (!discount_color_cost)
        palette_mode_cost +=
            av1_cost_color_map(x, 0, bsize, mbmi->tx_size, PALETTE_MAP);

      total_rate += palette_mode_cost;
    }
  }
  if (av1_filter_intra_allowed(&cpi->common, mbmi)) {
    total_rate += mode_costs->filter_intra_cost[mbmi->bsize][use_filter_intra];
    if (use_filter_intra) {
      total_rate +=
          mode_costs->filter_intra_mode_cost[mbmi->filter_intra_mode_info
                                                 .filter_intra_mode];
    }
  }
  if (av1_is_directional_mode(mbmi->mode)) {
    if (av1_use_angle_delta(bsize)) {
      total_rate +=
          mode_costs->angle_delta_cost[mbmi->mode - V_PRED]
                                      [MAX_ANGLE_DELTA +
                                       mbmi->angle_delta[PLANE_TYPE_Y]];
    }
  }
  if (av1_allow_intrabc(&cpi->common))
    total_rate += mode_costs->intrabc_cost[use_intrabc];
  return total_rate;
}

/*!\brief Return the rate cost for chroma prediction mode info of intra blocks.
 *
 * \callergraph
 */
static inline int intra_mode_info_cost_uv(const AV1_COMP *cpi,
                                          const MACROBLOCK *x,
                                          const MB_MODE_INFO *mbmi,
                                          BLOCK_SIZE bsize, int mode_cost) {
  int total_rate = mode_cost;
  const ModeCosts *mode_costs = &x->mode_costs;
  const int use_palette = mbmi->palette_mode_info.palette_size[1] > 0;
  const UV_PREDICTION_MODE uv_mode = mbmi->uv_mode;
  // Can only activate one mode.
  assert(((uv_mode != UV_DC_PRED) + use_palette + mbmi->use_intrabc) <= 1);

  const int try_palette = av1_allow_palette(
      cpi->common.features.allow_screen_content_tools, mbmi->bsize);
  if (try_palette && uv_mode == UV_DC_PRED) {
    const PALETTE_MODE_INFO *pmi = &mbmi->palette_mode_info;
    total_rate +=
        mode_costs->palette_uv_mode_cost[pmi->palette_size[0] > 0][use_palette];
    if (use_palette) {
      const int bsize_ctx = av1_get_palette_bsize_ctx(bsize);
      const int plt_size = pmi->palette_size[1];
      const MACROBLOCKD *xd = &x->e_mbd;
      const uint8_t *const color_map = xd->plane[1].color_index_map;
      int palette_mode_cost =
          mode_costs
              ->palette_uv_size_cost[bsize_ctx][plt_size - PALETTE_MIN_SIZE] +
          write_uniform_cost(plt_size, color_map[0]);
      uint16_t color_cache[2 * PALETTE_MAX_SIZE];
      const int n_cache = av1_get_palette_cache(xd, 1, color_cache);
      palette_mode_cost += av1_palette_color_cost_uv(
          pmi, color_cache, n_cache, cpi->common.seq_params->bit_depth);
      palette_mode_cost +=
          av1_cost_color_map(x, 1, bsize, mbmi->tx_size, PALETTE_MAP);
      total_rate += palette_mode_cost;
    }
  }
  const PREDICTION_MODE intra_mode = get_uv_mode(uv_mode);
  if (av1_is_directional_mode(intra_mode)) {
    if (av1_use_angle_delta(bsize)) {
      total_rate +=
          mode_costs->angle_delta_cost[intra_mode - V_PRED]
                                      [mbmi->angle_delta[PLANE_TYPE_UV] +
                                       MAX_ANGLE_DELTA];
    }
  }
  return total_rate;
}

/*!\cond */
// Makes a quick intra prediction and estimate the rdcost with a model without
// going through the whole txfm/quantize/itxfm process.
static int64_t intra_model_rd(const AV1_COMMON *cm, MACROBLOCK *const x,
                              int plane, BLOCK_SIZE plane_bsize,
                              TX_SIZE tx_size, int use_hadamard) {
  MACROBLOCKD *const xd = &x->e_mbd;
  const BitDepthInfo bd_info = get_bit_depth_info(xd);
  int row, col;
  assert(!is_inter_block(xd->mi[0]));
  const int stepr = tx_size_high_unit[tx_size];
  const int stepc = tx_size_wide_unit[tx_size];
  const int txbw = tx_size_wide[tx_size];
  const int txbh = tx_size_high[tx_size];
  const int max_blocks_wide = max_block_wide(xd, plane_bsize, plane);
  const int max_blocks_high = max_block_high(xd, plane_bsize, plane);
  int64_t satd_cost = 0;
  struct macroblock_plane *p = &x->plane[plane];
  struct macroblockd_plane *pd = &xd->plane[plane];
  // Prediction.
  for (row = 0; row < max_blocks_high; row += stepr) {
    for (col = 0; col < max_blocks_wide; col += stepc) {
      av1_predict_intra_block_facade(cm, xd, plane, col, row, tx_size);
      // Here we use p->src_diff and p->coeff as temporary buffers for
      // prediction residue and transform coefficients. The buffers are only
      // used in this for loop, therefore we don't need to properly add offset
      // to the buffers.
      av1_subtract_block(
          bd_info, txbh, txbw, p->src_diff, block_size_wide[plane_bsize],
          p->src.buf + (((row * p->src.stride) + col) << 2), p->src.stride,
          pd->dst.buf + (((row * pd->dst.stride) + col) << 2), pd->dst.stride);
      av1_quick_txfm(use_hadamard, tx_size, bd_info, p->src_diff,
                     block_size_wide[plane_bsize], p->coeff);
      satd_cost += aom_satd(p->coeff, tx_size_2d[tx_size]);
    }
  }
  return satd_cost;
}
/*!\endcond */

/*!\brief Estimate the luma rdcost of a given intra mode and try to prune it.
 *
 * \ingroup intra_mode_search
 * \callergraph
 * This function first makes a quick luma prediction and estimates the rdcost
 * with a model without going through the txfm, then try to prune the current
 * mode if the new estimate y_rd > 1.25 * best_model_rd.
 *
 * \return Returns 1 if the given mode is prune; 0 otherwise.
 */
static inline int model_intra_yrd_and_prune(const AV1_COMP *const cpi,
                                            MACROBLOCK *x, BLOCK_SIZE bsize,
                                            int64_t *best_model_rd) {
  const TX_SIZE tx_size = AOMMIN(TX_32X32, max_txsize_lookup[bsize]);
  const int plane = 0;
  const AV1_COMMON *cm = &cpi->common;
  const int64_t this_model_rd =
      intra_model_rd(cm, x, plane, bsize, tx_size, /*use_hadamard=*/1);
  if (*best_model_rd != INT64_MAX &&
      this_model_rd > *best_model_rd + (*best_model_rd >> 2)) {
    return 1;
  } else if (this_model_rd < *best_model_rd) {
    *best_model_rd = this_model_rd;
  }
  return 0;
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_INTRA_MODE_SEARCH_UTILS_H_
