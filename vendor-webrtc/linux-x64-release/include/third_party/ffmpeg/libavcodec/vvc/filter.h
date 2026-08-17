/*
 * VVC filters
 *
 * Copyright (C) 2022 Nuo Mi
 *
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
#ifndef AVCODEC_VVC_FILTER_H
#define AVCODEC_VVC_FILTER_H

#include "dec.h"

/**
 * lmcs filter for the CTU
 * @param lc local context for CTU
 * @param x0 x position for the CTU
 * @param y0 y position for the CTU
 */
void ff_vvc_lmcs_filter(const VVCLocalContext *lc, const int x0, const int y0);

/**
 * derive boundary strength for the CTU
 * @param lc local context for CTU
 * @param rx raster x position for the CTU
 * @param ry raster y position for the CTU
 * @param rs raster position for the CTU
 */
void ff_vvc_deblock_bs(VVCLocalContext *lc, const int rx, const int ry, const int rs);

/**
 * vertical deblock filter for the CTU
 * @param lc local context for CTU
 * @param x0 x position for the CTU
 * @param y0 y position for the CTU
 * @param rs raster position for the CTU
 */
void ff_vvc_deblock_vertical(const VVCLocalContext *lc, int x0, int y0, int rs);

/**
 * horizontal deblock filter for the CTU
 * @param lc local context for CTU
 * @param x0 x position for the CTU
 * @param y0 y position for the CTU
 * @param rs raster position for the CTU
 */
void ff_vvc_deblock_horizontal(const VVCLocalContext *lc, int x0, int y0, int rs);

/**
 * sao filter for the CTU
 * @param lc local context for CTU
 * @param x0 x position for the CTU
 * @param y0 y position for the CTU
 */
void ff_vvc_sao_filter(VVCLocalContext *lc, const int x0, const int y0);

void ff_vvc_sao_copy_ctb_to_hv(VVCLocalContext* lc, int rx, int ry, int last_row);
void ff_vvc_alf_copy_ctu_to_hv(VVCLocalContext* lc, int x0, int y0);

/**
 * alf filter for the CTU
 * @param lc local context for CTU
 * @param x0 x position for the CTU
 * @param y0 y position for the CTU
 */
void ff_vvc_alf_filter(VVCLocalContext *lc, const int x0, const int y0);

#endif // AVCODEC_VVC_CTU_H
