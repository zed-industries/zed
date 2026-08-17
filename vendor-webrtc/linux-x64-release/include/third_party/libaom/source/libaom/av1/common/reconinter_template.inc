/*
 * Copyright (c) 2022, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef IS_DEC
#error "IS_DEC must be defined for reconinter_template.inc."
#endif

#if IS_DEC
static inline void build_one_inter_predictor(uint8_t *dst, int dst_stride,
                                             const MV *src_mv,
                                             InterPredParams *inter_pred_params,
                                             MACROBLOCKD *xd, int mi_x,
                                             int mi_y, int ref,
                                             uint8_t **mc_buf) {
#else
static inline void build_one_inter_predictor(
    uint8_t *dst, int dst_stride, const MV *src_mv,
    InterPredParams *inter_pred_params) {
#endif  // IS_DEC
  SubpelParams subpel_params;
  uint8_t *src;
  int src_stride;
#if IS_DEC
  dec_calc_subpel_params_and_extend(src_mv, inter_pred_params, xd, mi_x, mi_y,
                                    ref, mc_buf, &src, &subpel_params,
                                    &src_stride);
#else
  enc_calc_subpel_params(src_mv, inter_pred_params, &src, &subpel_params,
                         &src_stride);
#endif  // IS_DEC
  if (inter_pred_params->comp_mode == UNIFORM_SINGLE ||
      inter_pred_params->comp_mode == UNIFORM_COMP) {
    av1_make_inter_predictor(src, src_stride, dst, dst_stride,
                             inter_pred_params, &subpel_params);
  } else {
    av1_make_masked_inter_predictor(src, src_stride, dst, dst_stride,
                                    inter_pred_params, &subpel_params);
  }
}

// True if the following hold:
//  1. Not intrabc and not build_for_obmc
//  2. At least one dimension is size 4 with subsampling
//  3. If sub-sampled, none of the previous blocks around the sub-sample
//     are intrabc or inter-blocks
static bool is_sub8x8_inter(const MACROBLOCKD *xd, int plane, BLOCK_SIZE bsize,
                            int is_intrabc, int build_for_obmc) {
  if (is_intrabc || build_for_obmc) {
    return false;
  }

  const struct macroblockd_plane *const pd = &xd->plane[plane];
  const int ss_x = pd->subsampling_x;
  const int ss_y = pd->subsampling_y;
  const int is_sub4_x = (block_size_wide[bsize] == 4) && ss_x;
  const int is_sub4_y = (block_size_high[bsize] == 4) && ss_y;
  if (!is_sub4_x && !is_sub4_y) {
    return false;
  }

  // For sub8x8 chroma blocks, we may be covering more than one luma block's
  // worth of pixels. Thus (mi_x, mi_y) may not be the correct coordinates for
  // the top-left corner of the prediction source - the correct top-left corner
  // is at (pre_x, pre_y).
  const int row_start = is_sub4_y ? -1 : 0;
  const int col_start = is_sub4_x ? -1 : 0;

  for (int row = row_start; row <= 0; ++row) {
    for (int col = col_start; col <= 0; ++col) {
      const MB_MODE_INFO *this_mbmi = xd->mi[row * xd->mi_stride + col];
      if (!is_inter_block(this_mbmi)) return false;
      if (is_intrabc_block(this_mbmi)) return false;
    }
  }
  return true;
}

#if IS_DEC
static inline void build_inter_predictors_sub8x8(const AV1_COMMON *cm,
                                                 MACROBLOCKD *xd, int plane,
                                                 const MB_MODE_INFO *mi,
                                                 int mi_x, int mi_y,
                                                 uint8_t **mc_buf) {
#else
static inline void build_inter_predictors_sub8x8(const AV1_COMMON *cm,
                                                 MACROBLOCKD *xd, int plane,
                                                 const MB_MODE_INFO *mi,
                                                 int mi_x, int mi_y) {
#endif  // IS_DEC
  const BLOCK_SIZE bsize = mi->bsize;
  struct macroblockd_plane *const pd = &xd->plane[plane];
  const bool ss_x = pd->subsampling_x;
  const bool ss_y = pd->subsampling_y;
  const int b4_w = block_size_wide[bsize] >> ss_x;
  const int b4_h = block_size_high[bsize] >> ss_y;
  const BLOCK_SIZE plane_bsize = get_plane_block_size(bsize, ss_x, ss_y);
  const int b8_w = block_size_wide[plane_bsize];
  const int b8_h = block_size_high[plane_bsize];
  const int is_compound = has_second_ref(mi);
  assert(!is_compound);
  assert(!is_intrabc_block(mi));

  // For sub8x8 chroma blocks, we may be covering more than one luma block's
  // worth of pixels. Thus (mi_x, mi_y) may not be the correct coordinates for
  // the top-left corner of the prediction source - the correct top-left corner
  // is at (pre_x, pre_y).
  const int row_start = (block_size_high[bsize] == 4) && ss_y ? -1 : 0;
  const int col_start = (block_size_wide[bsize] == 4) && ss_x ? -1 : 0;
  const int pre_x = (mi_x + MI_SIZE * col_start) >> ss_x;
  const int pre_y = (mi_y + MI_SIZE * row_start) >> ss_y;

  int row = row_start;
  for (int y = 0; y < b8_h; y += b4_h) {
    int col = col_start;
    for (int x = 0; x < b8_w; x += b4_w) {
      MB_MODE_INFO *this_mbmi = xd->mi[row * xd->mi_stride + col];
      struct buf_2d *const dst_buf = &pd->dst;
      uint8_t *dst = dst_buf->buf + dst_buf->stride * y + x;
      int ref = 0;
      const RefCntBuffer *ref_buf =
          get_ref_frame_buf(cm, this_mbmi->ref_frame[ref]);
      const struct scale_factors *ref_scale_factors =
          get_ref_scale_factors_const(cm, this_mbmi->ref_frame[ref]);
      const struct scale_factors *const sf = ref_scale_factors;
      const struct buf_2d pre_buf = {
        NULL,
        (plane == 1) ? ref_buf->buf.u_buffer : ref_buf->buf.v_buffer,
        ref_buf->buf.uv_crop_width,
        ref_buf->buf.uv_crop_height,
        ref_buf->buf.uv_stride,
      };

      const MV mv = this_mbmi->mv[ref].as_mv;

      InterPredParams inter_pred_params;
      av1_init_inter_params(&inter_pred_params, b4_w, b4_h, pre_y + y,
                            pre_x + x, pd->subsampling_x, pd->subsampling_y,
                            xd->bd, is_cur_buf_hbd(xd), mi->use_intrabc, sf,
                            &pre_buf, this_mbmi->interp_filters);
      inter_pred_params.conv_params =
          get_conv_params_no_round(ref, plane, NULL, 0, is_compound, xd->bd);

#if IS_DEC
      build_one_inter_predictor(dst, dst_buf->stride, &mv, &inter_pred_params,
                                xd, mi_x + x, mi_y + y, ref, mc_buf);
#else
      build_one_inter_predictor(dst, dst_buf->stride, &mv, &inter_pred_params);
#endif  // IS_DEC

      ++col;
    }
    ++row;
  }
}

#if IS_DEC
static inline void build_inter_predictors_8x8_and_bigger(
    const AV1_COMMON *cm, MACROBLOCKD *xd, int plane, const MB_MODE_INFO *mi,
    int build_for_obmc, int bw, int bh, int mi_x, int mi_y, uint8_t **mc_buf) {
#else
static inline void build_inter_predictors_8x8_and_bigger(
    const AV1_COMMON *cm, MACROBLOCKD *xd, int plane, const MB_MODE_INFO *mi,
    int build_for_obmc, int bw, int bh, int mi_x, int mi_y) {
#endif  // IS_DEC
  const int is_compound = has_second_ref(mi);
  const int is_intrabc = is_intrabc_block(mi);
  assert(IMPLIES(is_intrabc, !is_compound));
  struct macroblockd_plane *const pd = &xd->plane[plane];
  struct buf_2d *const dst_buf = &pd->dst;
  uint8_t *const dst = dst_buf->buf;

  int is_global[2] = { 0, 0 };
  for (int ref = 0; ref < 1 + is_compound; ++ref) {
    const WarpedMotionParams *const wm = &xd->global_motion[mi->ref_frame[ref]];
    is_global[ref] = is_global_mv_block(mi, wm->wmtype);
  }

  const BLOCK_SIZE bsize = mi->bsize;
  const int ss_x = pd->subsampling_x;
  const int ss_y = pd->subsampling_y;
  const int row_start =
      (block_size_high[bsize] == 4) && ss_y && !build_for_obmc ? -1 : 0;
  const int col_start =
      (block_size_wide[bsize] == 4) && ss_x && !build_for_obmc ? -1 : 0;
  const int pre_x = (mi_x + MI_SIZE * col_start) >> ss_x;
  const int pre_y = (mi_y + MI_SIZE * row_start) >> ss_y;

  for (int ref = 0; ref < 1 + is_compound; ++ref) {
    const struct scale_factors *const sf =
        is_intrabc ? &cm->sf_identity : xd->block_ref_scale_factors[ref];
    struct buf_2d *const pre_buf = is_intrabc ? dst_buf : &pd->pre[ref];
    const MV mv = mi->mv[ref].as_mv;
    const WarpTypesAllowed warp_types = { is_global[ref],
                                          mi->motion_mode == WARPED_CAUSAL };

    InterPredParams inter_pred_params;
    av1_init_inter_params(&inter_pred_params, bw, bh, pre_y, pre_x,
                          pd->subsampling_x, pd->subsampling_y, xd->bd,
                          is_cur_buf_hbd(xd), mi->use_intrabc, sf, pre_buf,
                          mi->interp_filters);
    if (is_compound) av1_init_comp_mode(&inter_pred_params);
    inter_pred_params.conv_params = get_conv_params_no_round(
        ref, plane, xd->tmp_conv_dst, MAX_SB_SIZE, is_compound, xd->bd);

    av1_dist_wtd_comp_weight_assign(
        cm, mi, &inter_pred_params.conv_params.fwd_offset,
        &inter_pred_params.conv_params.bck_offset,
        &inter_pred_params.conv_params.use_dist_wtd_comp_avg, is_compound);

    if (!build_for_obmc)
      av1_init_warp_params(&inter_pred_params, &warp_types, ref, xd, mi);

    if (is_masked_compound_type(mi->interinter_comp.type)) {
      inter_pred_params.sb_type = mi->bsize;
      inter_pred_params.mask_comp = mi->interinter_comp;
      if (ref == 1) {
        inter_pred_params.conv_params.do_average = 0;
        inter_pred_params.comp_mode = MASK_COMP;
      }
      // Assign physical buffer.
      inter_pred_params.mask_comp.seg_mask = xd->seg_mask;
    }

#if IS_DEC
    build_one_inter_predictor(dst, dst_buf->stride, &mv, &inter_pred_params, xd,
                              mi_x, mi_y, ref, mc_buf);
#else
    build_one_inter_predictor(dst, dst_buf->stride, &mv, &inter_pred_params);
#endif  // IS_DEC
  }
}

#if IS_DEC
static inline void build_inter_predictors(const AV1_COMMON *cm, MACROBLOCKD *xd,
                                          int plane, const MB_MODE_INFO *mi,
                                          int build_for_obmc, int bw, int bh,
                                          int mi_x, int mi_y,
                                          uint8_t **mc_buf) {
  if (is_sub8x8_inter(xd, plane, mi->bsize, is_intrabc_block(mi),
                      build_for_obmc)) {
    assert(bw < 8 || bh < 8);
    build_inter_predictors_sub8x8(cm, xd, plane, mi, mi_x, mi_y, mc_buf);
  } else {
    build_inter_predictors_8x8_and_bigger(cm, xd, plane, mi, build_for_obmc, bw,
                                          bh, mi_x, mi_y, mc_buf);
  }
}
#else
static inline void build_inter_predictors(const AV1_COMMON *cm, MACROBLOCKD *xd,
                                          int plane, const MB_MODE_INFO *mi,
                                          int build_for_obmc, int bw, int bh,
                                          int mi_x, int mi_y) {
  if (is_sub8x8_inter(xd, plane, mi->bsize, is_intrabc_block(mi),
                      build_for_obmc)) {
    assert(bw < 8 || bh < 8);
    build_inter_predictors_sub8x8(cm, xd, plane, mi, mi_x, mi_y);
  } else {
    build_inter_predictors_8x8_and_bigger(cm, xd, plane, mi, build_for_obmc, bw,
                                          bh, mi_x, mi_y);
  }
}
#endif  // IS_DEC
