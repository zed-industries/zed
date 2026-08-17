/*
 * VVC intra prediction
 *
 * Copyright (C) 2021 Nuo Mi
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
#ifndef AVCODEC_VVC_INTRA_H
#define AVCODEC_VVC_INTRA_H

#include "ctu.h"

/**
 * reconstruct a CTU
 * @param lc local context for CTU
 * @param rs raster order for the CTU.
 * @param rx raster order x for the CTU.
 * @param ry raster order y for the CTU.
 * @return AVERROR
 */
int ff_vvc_reconstruct(VVCLocalContext *lc, const int rs, const int rx, const int ry);

//utils for vvc_intra_template
int ff_vvc_get_top_available(const VVCLocalContext *lc, int x0, int y0, int target_size, int c_idx);
int ff_vvc_get_left_available(const VVCLocalContext *lc, int x0, int y0, int target_size, int c_idx);
int ff_vvc_get_mip_size_id(int w, int h);
int ff_vvc_need_pdpc(int w, int h, uint8_t bdpcm_flag, int mode, int ref_idx);
int ff_vvc_nscale_derive(int w, int h, int mode);
int ff_vvc_ref_filter_flag_derive(int mode);
int ff_vvc_intra_pred_angle_derive(int pred_mode);
int ff_vvc_intra_inv_angle_derive(int pred_mode);
int ff_vvc_wide_angle_mode_mapping(const CodingUnit *cu,
    int tb_width, int tb_height, int c_idx, int pred_mode_intra);

#endif // AVCODEC_VVC_INTRA_H
