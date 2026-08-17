/*
 * VVC motion vector decoder
 *
 * Copyright (C) 2023 Nuo Mi
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_VVC_MVS_H
#define AVCODEC_VVC_MVS_H

#include "ctu.h"

void ff_vvc_round_mv(Mv *mv, int lshift, int rshift);
void ff_vvc_clip_mv(Mv *mv);
void ff_vvc_mv_scale(Mv *dst, const Mv *src, int td, int tb);
void ff_vvc_luma_mv_merge_mode(VVCLocalContext *lc, int merge_idx, int ciip_flag, MvField *mv);
void ff_vvc_luma_mv_merge_gpm(VVCLocalContext *lc, const int merge_gpm_idx[2], MvField *mv);
int ff_vvc_luma_mv_merge_ibc(VVCLocalContext *lc, int merge_idx, Mv *mv);
void ff_vvc_mvp(VVCLocalContext *lc, const int *mvp_lx_flag, const int amvr_shift,  MotionInfo *mi);
int ff_vvc_mvp_ibc(VVCLocalContext *lc, int mvp_l0_flag, int amvr_shift, Mv *mv);
void ff_vvc_sb_mv_merge_mode(VVCLocalContext *lc, int merge_subblock_idx, PredictionUnit *pu);
void ff_vvc_affine_mvp(VVCLocalContext *lc, const int *mvp_lx_flag, const int amvr_shift, MotionInfo* mi);
void ff_vvc_store_sb_mvs(const VVCLocalContext *lc, PredictionUnit *pu);
void ff_vvc_store_mv(const VVCLocalContext *lc, const MotionInfo *mi);
void ff_vvc_store_mvf(const VVCLocalContext *lc, const MvField *mvf);
void ff_vvc_store_gpm_mvf(const VVCLocalContext *lc, const PredictionUnit* pu);
void ff_vvc_update_hmvp(VVCLocalContext *lc, const MotionInfo *mi);
int ff_vvc_no_backward_pred_flag(const VVCLocalContext *lc);
MvField* ff_vvc_get_mvf(const VVCFrameContext *fc, const int x0, const int y0);
void ff_vvc_set_mvf(const VVCLocalContext *lc, const int x0, const int y0, const int w, const int h, const MvField *mvf);
void ff_vvc_set_intra_mvf(const VVCLocalContext *lc, int dmvr);

#endif //AVCODEC_VVC_MVS_H
