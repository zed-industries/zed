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

#ifndef AOM_AV1_COMMON_RECONINTER_H_
#define AOM_AV1_COMMON_RECONINTER_H_

#include "av1/common/av1_common_int.h"
#include "av1/common/convolve.h"
#include "av1/common/filter.h"
#include "av1/common/warped_motion.h"
#include "aom/aom_integer.h"

// Work out how many pixels off the edge of a reference frame we're allowed
// to go when forming an inter prediction.
// The outermost row/col of each referernce frame is extended by
// (AOM_BORDER_IN_PIXELS >> subsampling) pixels, but we need to keep
// at least AOM_INTERP_EXTEND pixels within that to account for filtering.
//
// We have to break this up into two macros to keep both clang-format and
// tools/lint-hunks.py happy.
#define AOM_LEFT_TOP_MARGIN_PX(subsampling) \
  ((AOM_BORDER_IN_PIXELS >> subsampling) - AOM_INTERP_EXTEND)
#define AOM_LEFT_TOP_MARGIN_SCALED(subsampling) \
  (AOM_LEFT_TOP_MARGIN_PX(subsampling) << SCALE_SUBPEL_BITS)

#ifdef __cplusplus
extern "C" {
#endif

#define MAX_WEDGE_TYPES 16

#define MAX_WEDGE_SIZE_LOG2 5  // 32x32
#define MAX_WEDGE_SIZE (1 << MAX_WEDGE_SIZE_LOG2)
#define MAX_WEDGE_SQUARE (MAX_WEDGE_SIZE * MAX_WEDGE_SIZE)

#define WEDGE_WEIGHT_BITS 6

#define WEDGE_NONE -1

// Angles are with respect to horizontal anti-clockwise
enum {
  WEDGE_HORIZONTAL = 0,
  WEDGE_VERTICAL = 1,
  WEDGE_OBLIQUE27 = 2,
  WEDGE_OBLIQUE63 = 3,
  WEDGE_OBLIQUE117 = 4,
  WEDGE_OBLIQUE153 = 5,
  WEDGE_DIRECTIONS
} UENUM1BYTE(WedgeDirectionType);

// 3-tuple: {direction, x_offset, y_offset}
typedef struct {
  WedgeDirectionType direction;
  int x_offset;
  int y_offset;
} wedge_code_type;

typedef uint8_t *wedge_masks_type[MAX_WEDGE_TYPES];

typedef struct {
  int wedge_types;
  const wedge_code_type *codebook;
  uint8_t *signflip;
  wedge_masks_type *masks;
} wedge_params_type;

extern const wedge_params_type av1_wedge_params_lookup[BLOCK_SIZES_ALL];

typedef struct SubpelParams {
  int xs;
  int ys;
  int subpel_x;
  int subpel_y;
  int pos_x;
  int pos_y;
} SubpelParams;

struct build_prediction_ctxt {
  const AV1_COMMON *cm;
  uint8_t **tmp_buf;
  int *tmp_width;
  int *tmp_height;
  int *tmp_stride;
  int mb_to_far_edge;
  void *dcb;  // Decoder-only coding block.
};

typedef enum InterPredMode {
  TRANSLATION_PRED,
  WARP_PRED,
} InterPredMode;

typedef enum InterCompMode {
  UNIFORM_SINGLE,
  UNIFORM_COMP,
  MASK_COMP,
} InterCompMode;

typedef struct InterPredParams {
  InterPredMode mode;
  InterCompMode comp_mode;
  WarpedMotionParams warp_params;
  ConvolveParams conv_params;
  const InterpFilterParams *interp_filter_params[2];
  int block_width;
  int block_height;
  int pix_row;
  int pix_col;
  struct buf_2d ref_frame_buf;
  int subsampling_x;
  int subsampling_y;
  const struct scale_factors *scale_factors;
  int bit_depth;
  int use_hbd_buf;
  INTERINTER_COMPOUND_DATA mask_comp;
  BLOCK_SIZE sb_type;
  int is_intrabc;
  int top;
  int left;
} InterPredParams;

// Initialize sub-pel params required for inter prediction.
static inline void init_subpel_params(const MV *const src_mv,
                                      InterPredParams *const inter_pred_params,
                                      SubpelParams *subpel_params, int width,
                                      int height) {
  const struct scale_factors *sf = inter_pred_params->scale_factors;
  int ssx = inter_pred_params->subsampling_x;
  int ssy = inter_pred_params->subsampling_y;
  int orig_pos_y = inter_pred_params->pix_row << SUBPEL_BITS;
  orig_pos_y += src_mv->row * (1 << (1 - ssy));
  int orig_pos_x = inter_pred_params->pix_col << SUBPEL_BITS;
  orig_pos_x += src_mv->col * (1 << (1 - ssx));
  const int is_scaled = av1_is_scaled(sf);
  int pos_x, pos_y;
  if (LIKELY(!is_scaled)) {
    pos_y = av1_unscaled_value(orig_pos_y, sf);
    pos_x = av1_unscaled_value(orig_pos_x, sf);
  } else {
    pos_y = av1_scaled_y(orig_pos_y, sf);
    pos_x = av1_scaled_x(orig_pos_x, sf);
  }

  pos_x += SCALE_EXTRA_OFF;
  pos_y += SCALE_EXTRA_OFF;

  const int bottom = (height + AOM_INTERP_EXTEND) << SCALE_SUBPEL_BITS;
  const int right = (width + AOM_INTERP_EXTEND) << SCALE_SUBPEL_BITS;
  pos_y = clamp(pos_y, inter_pred_params->top, bottom);
  pos_x = clamp(pos_x, inter_pred_params->left, right);

  subpel_params->pos_x = pos_x;
  subpel_params->pos_y = pos_y;
  subpel_params->subpel_x = pos_x & SCALE_SUBPEL_MASK;
  subpel_params->subpel_y = pos_y & SCALE_SUBPEL_MASK;
  subpel_params->xs = sf->x_step_q4;
  subpel_params->ys = sf->y_step_q4;
}

// Initialize interp filter required for inter prediction.
static inline void init_interp_filter_params(
    const InterpFilterParams *interp_filter_params[2],
    const InterpFilters *filter, int block_width, int block_height,
    int is_intrabc) {
  if (UNLIKELY(is_intrabc)) {
    interp_filter_params[0] = &av1_intrabc_filter_params;
    interp_filter_params[1] = &av1_intrabc_filter_params;
  } else {
    interp_filter_params[0] = av1_get_interp_filter_params_with_block_size(
        (InterpFilter)filter->x_filter, block_width);
    interp_filter_params[1] = av1_get_interp_filter_params_with_block_size(
        (InterpFilter)filter->y_filter, block_height);
  }
}

// Initialize parameters required for inter prediction at mode level.
static inline void init_inter_mode_params(
    const MV *const src_mv, InterPredParams *const inter_pred_params,
    SubpelParams *subpel_params, const struct scale_factors *sf, int width,
    int height) {
  inter_pred_params->scale_factors = sf;
  init_subpel_params(src_mv, inter_pred_params, subpel_params, width, height);
}

// Initialize parameters required for inter prediction at block level.
static inline void init_inter_block_params(InterPredParams *inter_pred_params,
                                           int block_width, int block_height,
                                           int pix_row, int pix_col,
                                           int subsampling_x, int subsampling_y,
                                           int bit_depth, int use_hbd_buf,
                                           int is_intrabc) {
  inter_pred_params->block_width = block_width;
  inter_pred_params->block_height = block_height;
  inter_pred_params->pix_row = pix_row;
  inter_pred_params->pix_col = pix_col;
  inter_pred_params->subsampling_x = subsampling_x;
  inter_pred_params->subsampling_y = subsampling_y;
  inter_pred_params->bit_depth = bit_depth;
  inter_pred_params->use_hbd_buf = use_hbd_buf;
  inter_pred_params->is_intrabc = is_intrabc;
  inter_pred_params->mode = TRANSLATION_PRED;
  inter_pred_params->comp_mode = UNIFORM_SINGLE;
  inter_pred_params->top = -AOM_LEFT_TOP_MARGIN_SCALED(subsampling_y);
  inter_pred_params->left = -AOM_LEFT_TOP_MARGIN_SCALED(subsampling_x);
}

// Initialize params required for inter prediction.
static inline void av1_init_inter_params(
    InterPredParams *inter_pred_params, int block_width, int block_height,
    int pix_row, int pix_col, int subsampling_x, int subsampling_y,
    int bit_depth, int use_hbd_buf, int is_intrabc,
    const struct scale_factors *sf, const struct buf_2d *ref_buf,
    int_interpfilters interp_filters) {
  init_inter_block_params(inter_pred_params, block_width, block_height, pix_row,
                          pix_col, subsampling_x, subsampling_y, bit_depth,
                          use_hbd_buf, is_intrabc);
  init_interp_filter_params(inter_pred_params->interp_filter_params,
                            &interp_filters.as_filters, block_width,
                            block_height, is_intrabc);
  inter_pred_params->scale_factors = sf;
  inter_pred_params->ref_frame_buf = *ref_buf;
}

static inline void av1_init_comp_mode(InterPredParams *inter_pred_params) {
  inter_pred_params->comp_mode = UNIFORM_COMP;
}

void av1_init_warp_params(InterPredParams *inter_pred_params,
                          const WarpTypesAllowed *warp_types, int ref,
                          const MACROBLOCKD *xd, const MB_MODE_INFO *mi);

static inline int has_scale(int xs, int ys) {
  return xs != SCALE_SUBPEL_SHIFTS || ys != SCALE_SUBPEL_SHIFTS;
}

static inline void revert_scale_extra_bits(SubpelParams *sp) {
  sp->subpel_x >>= SCALE_EXTRA_BITS;
  sp->subpel_y >>= SCALE_EXTRA_BITS;
  sp->xs >>= SCALE_EXTRA_BITS;
  sp->ys >>= SCALE_EXTRA_BITS;
  assert(sp->subpel_x < SUBPEL_SHIFTS);
  assert(sp->subpel_y < SUBPEL_SHIFTS);
  assert(sp->xs <= SUBPEL_SHIFTS);
  assert(sp->ys <= SUBPEL_SHIFTS);
}

static inline void inter_predictor(
    const uint8_t *src, int src_stride, uint8_t *dst, int dst_stride,
    const SubpelParams *subpel_params, int w, int h,
    ConvolveParams *conv_params, const InterpFilterParams *interp_filters[2]) {
  assert(conv_params->do_average == 0 || conv_params->do_average == 1);
  const int is_scaled = has_scale(subpel_params->xs, subpel_params->ys);
  if (is_scaled) {
    av1_convolve_2d_facade(src, src_stride, dst, dst_stride, w, h,
                           interp_filters, subpel_params->subpel_x,
                           subpel_params->xs, subpel_params->subpel_y,
                           subpel_params->ys, 1, conv_params);
  } else {
    SubpelParams sp = *subpel_params;
    revert_scale_extra_bits(&sp);
    av1_convolve_2d_facade(src, src_stride, dst, dst_stride, w, h,
                           interp_filters, sp.subpel_x, sp.xs, sp.subpel_y,
                           sp.ys, 0, conv_params);
  }
}

static inline void highbd_inter_predictor(
    const uint8_t *src, int src_stride, uint8_t *dst, int dst_stride,
    const SubpelParams *subpel_params, int w, int h,
    ConvolveParams *conv_params, const InterpFilterParams *interp_filters[2],
    int bd) {
  assert(conv_params->do_average == 0 || conv_params->do_average == 1);
  const int is_scaled = has_scale(subpel_params->xs, subpel_params->ys);
  if (is_scaled) {
    av1_highbd_convolve_2d_facade(src, src_stride, dst, dst_stride, w, h,
                                  interp_filters, subpel_params->subpel_x,
                                  subpel_params->xs, subpel_params->subpel_y,
                                  subpel_params->ys, 1, conv_params, bd);
  } else {
    SubpelParams sp = *subpel_params;
    revert_scale_extra_bits(&sp);
    av1_highbd_convolve_2d_facade(src, src_stride, dst, dst_stride, w, h,
                                  interp_filters, sp.subpel_x, sp.xs,
                                  sp.subpel_y, sp.ys, 0, conv_params, bd);
  }
}

int av1_skip_u4x4_pred_in_obmc(BLOCK_SIZE bsize,
                               const struct macroblockd_plane *pd, int dir);

static inline int is_interinter_compound_used(COMPOUND_TYPE type,
                                              BLOCK_SIZE sb_type) {
  const int comp_allowed = is_comp_ref_allowed(sb_type);
  switch (type) {
    case COMPOUND_AVERAGE:
    case COMPOUND_DISTWTD:
    case COMPOUND_DIFFWTD: return comp_allowed;
    case COMPOUND_WEDGE:
      return comp_allowed && av1_wedge_params_lookup[sb_type].wedge_types > 0;
    default: assert(0); return 0;
  }
}

static inline int is_any_masked_compound_used(BLOCK_SIZE sb_type) {
  COMPOUND_TYPE comp_type;
  int i;
  if (!is_comp_ref_allowed(sb_type)) return 0;
  for (i = 0; i < COMPOUND_TYPES; i++) {
    comp_type = (COMPOUND_TYPE)i;
    if (is_masked_compound_type(comp_type) &&
        is_interinter_compound_used(comp_type, sb_type))
      return 1;
  }
  return 0;
}

static inline int get_wedge_types_lookup(BLOCK_SIZE sb_type) {
  return av1_wedge_params_lookup[sb_type].wedge_types;
}

static inline int av1_is_wedge_used(BLOCK_SIZE sb_type) {
  return av1_wedge_params_lookup[sb_type].wedge_types > 0;
}

void av1_make_inter_predictor(const uint8_t *src, int src_stride, uint8_t *dst,
                              int dst_stride,
                              InterPredParams *inter_pred_params,
                              const SubpelParams *subpel_params);
void av1_make_masked_inter_predictor(const uint8_t *pre, int pre_stride,
                                     uint8_t *dst, int dst_stride,
                                     InterPredParams *inter_pred_params,
                                     const SubpelParams *subpel_params);

// TODO(jkoleszar): yet another mv clamping function :-(
static inline MV clamp_mv_to_umv_border_sb(const MACROBLOCKD *xd,
                                           const MV *src_mv, int bw, int bh,
                                           int ss_x, int ss_y) {
  // If the MV points so far into the UMV border that no visible pixels
  // are used for reconstruction, the subpel part of the MV can be
  // discarded and the MV limited to 16 pixels with equivalent results.
  const int spel_left = (AOM_INTERP_EXTEND + bw) << SUBPEL_BITS;
  const int spel_right = spel_left - SUBPEL_SHIFTS;
  const int spel_top = (AOM_INTERP_EXTEND + bh) << SUBPEL_BITS;
  const int spel_bottom = spel_top - SUBPEL_SHIFTS;
  MV clamped_mv = { (int16_t)(src_mv->row * (1 << (1 - ss_y))),
                    (int16_t)(src_mv->col * (1 << (1 - ss_x))) };
  assert(ss_x <= 1);
  assert(ss_y <= 1);
  const SubpelMvLimits mv_limits = {
    xd->mb_to_left_edge * (1 << (1 - ss_x)) - spel_left,
    xd->mb_to_right_edge * (1 << (1 - ss_x)) + spel_right,
    xd->mb_to_top_edge * (1 << (1 - ss_y)) - spel_top,
    xd->mb_to_bottom_edge * (1 << (1 - ss_y)) + spel_bottom
  };

  clamp_mv(&clamped_mv, &mv_limits);

  return clamped_mv;
}

static inline int64_t scaled_buffer_offset(int x_offset, int y_offset,
                                           int stride,
                                           const struct scale_factors *sf) {
  int x, y;
  if (!sf) {
    x = x_offset;
    y = y_offset;
  } else if (av1_is_scaled(sf)) {
    x = av1_scaled_x(x_offset, sf) >> SCALE_EXTRA_BITS;
    y = av1_scaled_y(y_offset, sf) >> SCALE_EXTRA_BITS;
  } else {
    x = av1_unscaled_value(x_offset, sf) >> SCALE_EXTRA_BITS;
    y = av1_unscaled_value(y_offset, sf) >> SCALE_EXTRA_BITS;
  }
  return (int64_t)y * stride + x;
}

static inline void setup_pred_plane(struct buf_2d *dst, BLOCK_SIZE bsize,
                                    uint8_t *src, int width, int height,
                                    int stride, int mi_row, int mi_col,
                                    const struct scale_factors *scale,
                                    int subsampling_x, int subsampling_y) {
  // Offset the buffer pointer
  if (subsampling_y && (mi_row & 0x01) && (mi_size_high[bsize] == 1))
    mi_row -= 1;
  if (subsampling_x && (mi_col & 0x01) && (mi_size_wide[bsize] == 1))
    mi_col -= 1;

  const int x = (MI_SIZE * mi_col) >> subsampling_x;
  const int y = (MI_SIZE * mi_row) >> subsampling_y;
  dst->buf = src + scaled_buffer_offset(x, y, stride, scale);
  dst->buf0 = src;
  dst->width = width;
  dst->height = height;
  dst->stride = stride;
}

void av1_setup_dst_planes(struct macroblockd_plane *planes, BLOCK_SIZE bsize,
                          const YV12_BUFFER_CONFIG *src, int mi_row, int mi_col,
                          const int plane_start, const int plane_end);

void av1_setup_pre_planes(MACROBLOCKD *xd, int idx,
                          const YV12_BUFFER_CONFIG *src, int mi_row, int mi_col,
                          const struct scale_factors *sf, const int num_planes);

static inline void set_default_interp_filters(
    MB_MODE_INFO *const mbmi, InterpFilter frame_interp_filter) {
  mbmi->interp_filters =
      av1_broadcast_interp_filter(av1_unswitchable_filter(frame_interp_filter));
}

static inline int av1_is_interp_needed(const MACROBLOCKD *const xd) {
  const MB_MODE_INFO *const mbmi = xd->mi[0];
  if (mbmi->skip_mode) return 0;
  if (mbmi->motion_mode == WARPED_CAUSAL) return 0;
  if (is_nontrans_global_motion(xd, xd->mi[0])) return 0;
  return 1;
}

// Sets up buffers 'dst_buf1' and 'dst_buf2' from relevant buffers in 'xd' for
// subsequent use in OBMC prediction.
void av1_setup_obmc_dst_bufs(MACROBLOCKD *xd, uint8_t **dst_buf1,
                             uint8_t **dst_buf2);

void av1_setup_build_prediction_by_above_pred(
    MACROBLOCKD *xd, int rel_mi_col, uint8_t above_mi_width,
    MB_MODE_INFO *above_mbmi, struct build_prediction_ctxt *ctxt,
    const int num_planes);
void av1_setup_build_prediction_by_left_pred(MACROBLOCKD *xd, int rel_mi_row,
                                             uint8_t left_mi_height,
                                             MB_MODE_INFO *left_mbmi,
                                             struct build_prediction_ctxt *ctxt,
                                             const int num_planes);
void av1_build_obmc_inter_prediction(const AV1_COMMON *cm, MACROBLOCKD *xd,
                                     uint8_t *above[MAX_MB_PLANE],
                                     int above_stride[MAX_MB_PLANE],
                                     uint8_t *left[MAX_MB_PLANE],
                                     int left_stride[MAX_MB_PLANE]);

const uint8_t *av1_get_obmc_mask(int length);
void av1_count_overlappable_neighbors(const AV1_COMMON *cm, MACROBLOCKD *xd);

#define MASK_MASTER_SIZE ((MAX_WEDGE_SIZE) << 1)
#define MASK_MASTER_STRIDE (MASK_MASTER_SIZE)

void av1_init_wedge_masks(void);

static inline const uint8_t *av1_get_contiguous_soft_mask(int8_t wedge_index,
                                                          int8_t wedge_sign,
                                                          BLOCK_SIZE sb_type) {
  return av1_wedge_params_lookup[sb_type].masks[wedge_sign][wedge_index];
}

void av1_dist_wtd_comp_weight_assign(const AV1_COMMON *cm,
                                     const MB_MODE_INFO *mbmi, int *fwd_offset,
                                     int *bck_offset,
                                     int *use_dist_wtd_comp_avg,
                                     int is_compound);

const uint8_t *av1_get_compound_type_mask(
    const INTERINTER_COMPOUND_DATA *const comp_data, BLOCK_SIZE sb_type);

// build interintra_predictors for one plane
void av1_build_interintra_predictor(const AV1_COMMON *cm, MACROBLOCKD *xd,
                                    uint8_t *pred, int stride,
                                    const BUFFER_SET *ctx, int plane,
                                    BLOCK_SIZE bsize);

void av1_build_intra_predictors_for_interintra(const AV1_COMMON *cm,
                                               MACROBLOCKD *xd,
                                               BLOCK_SIZE bsize, int plane,
                                               const BUFFER_SET *ctx,
                                               uint8_t *dst, int dst_stride);

void av1_combine_interintra(MACROBLOCKD *xd, BLOCK_SIZE bsize, int plane,
                            const uint8_t *inter_pred, int inter_stride,
                            const uint8_t *intra_pred, int intra_stride);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_RECONINTER_H_
