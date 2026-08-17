/*
 * Discrete wavelet transform
 * Copyright (c) 2007 Kamil Nowosad
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

#ifndef AVCODEC_JPEG2000DWT_H
#define AVCODEC_JPEG2000DWT_H

/**
 * @file
 * Discrete wavelet transform
 */

#include <stdint.h>

#define FF_DWT_MAX_DECLVLS 32 ///< max number of decomposition levels
#define F_LFTG_K      1.230174104914001f
#define F_LFTG_X      0.812893066115961f
#define I_PRESHIFT 8

enum DWTType {
    FF_DWT97,
    FF_DWT53,
    FF_DWT97_INT,
    FF_DWT_NB
};

typedef struct DWTContext {
    /// line lengths { horizontal, vertical } in consecutive decomposition levels
    int linelen[FF_DWT_MAX_DECLVLS][2];
    uint8_t mod[FF_DWT_MAX_DECLVLS][2];  ///< coordinates (x0, y0) of decomp. levels mod 2
    uint8_t ndeclevels;                  ///< number of decomposition levels
    uint8_t type;                        ///< 0 for 9/7; 1 for 5/3
    int32_t *i_linebuf;                  ///< int buffer used by transform
    float   *f_linebuf;                  ///< float buffer used by transform
} DWTContext;

/**
 * Initialize DWT.
 * @param s                 DWT context
 * @param border            coordinates of transformed region {{x0, x1}, {y0, y1}}
 * @param decomp_levels     number of decomposition levels
 * @param type              0 for DWT 9/7; 1 for DWT 5/3
 */
int ff_jpeg2000_dwt_init(DWTContext *s, int border[2][2],
                         int decomp_levels, int type);

int ff_dwt_encode(DWTContext *s, void *t);
int ff_dwt_decode(DWTContext *s, void *t);

void ff_dwt_destroy(DWTContext *s);

#endif /* AVCODEC_JPEG2000DWT_H */
