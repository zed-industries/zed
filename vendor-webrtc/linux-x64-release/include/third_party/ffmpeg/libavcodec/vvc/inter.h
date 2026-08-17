/*
 * VVC inter prediction
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

#ifndef AVCODEC_VVC_INTER_H
#define AVCODEC_VVC_INTER_H

#include "ctu.h"

/**
 * Loop entire CTU to predict all inter coding blocks
 * @param lc local context for CTU
 * @param rs raster order for the CTU
 * @return AVERROR
 */
int ff_vvc_predict_inter(VVCLocalContext *lc, int rs);

/**
 * CIIP(Combined Inter-Intra Prediction) for a coding block
 * @param lc local context for CTU
 */
void ff_vvc_predict_ciip(VVCLocalContext *lc);

#endif // AVCODEC_VVC_INTER_H
