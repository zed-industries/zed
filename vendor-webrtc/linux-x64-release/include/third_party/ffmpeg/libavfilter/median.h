/*
 * Copyright (c) 2019 Paul B Mahol
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
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 */

#ifndef AVFILTER_MEDIAN_H
#define AVFILTER_MEDIAN_H

#include "avfilter.h"

typedef struct MedianContext {
    const AVClass *class;

    int planes;
    int radius;
    int radiusV;
    float percentile;

    int planewidth[4];
    int planeheight[4];
    int depth;
    int nb_planes;
    int nb_threads;

    uint16_t **coarse, **fine;
    int coarse_size, fine_size;
    int bins;
    int t;

    void (*hadd)(uint16_t *dst, const uint16_t *src, int bins);
    void (*hsub)(uint16_t *dst, const uint16_t *src, int bins);
    void (*hmuladd)(uint16_t *dst, const uint16_t *src, int f, int bins);

    void (*filter_plane)(AVFilterContext *ctx, const uint8_t *ssrc, int src_linesize,
                         uint8_t *ddst, int dst_linesize, int width, int height,
                         int slice_h_start, int slice_h_end, int jobnr);
} MedianContext;

#endif
