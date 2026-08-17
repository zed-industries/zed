/*
 * Copyright (c) 2024 Christian R. Helmrich
 * Copyright (c) 2024 Christian Lehmann
 * Copyright (c) 2024 Christian Stoffers
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

/**
 * @file
 * Public declaration of DSP context structure of XPSNR measurement filter for FFmpeg.
 *
 * Authors: Christian Helmrich, Lehmann, and Stoffers, Fraunhofer HHI, Berlin, Germany
 */

#ifndef AVFILTER_XPSNR_H
#define AVFILTER_XPSNR_H

#include <stddef.h>
#include <stdint.h>
#include "libavutil/x86/cpu.h"

/* public XPSNR DSP structure definition */

typedef struct XPSNRDSPContext {
    uint64_t (*highds_func) (const int x_act, const int y_act, const int w_act, const int h_act, const int16_t *o_m0, const int o);
    uint64_t (*diff1st_func)(const uint32_t w_act, const uint32_t h_act, const int16_t *o_m0, int16_t *o_m1, const int o);
    uint64_t (*diff2nd_func)(const uint32_t w_act, const uint32_t h_act, const int16_t *o_m0, int16_t *o_m1, int16_t *o_m2, const int o);
} XPSNRDSPContext;

#endif /* AVFILTER_XPSNR_H */
