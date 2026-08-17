/*
 * ScreenPressor version 3 decoder
 *
 * Copyright (c) 2017 Paul B Mahol
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

#ifndef AVCODEC_SCPR3_H
#define AVCODEC_SCPR3_H

#include <stdint.h>

typedef struct PixelModel3 {
    uint8_t    type;
    uint8_t    length;
    uint8_t    maxpos;
    uint8_t    fshift;
    uint16_t   size;
    uint32_t   cntsum;
    uint8_t    symbols[256];
    uint16_t   freqs[256];
    uint16_t   freqs1[256];
    uint16_t   cnts[256];
    uint8_t    dectab[32];
} PixelModel3;

typedef struct FillModel3 {
    uint32_t   cntsum;
    uint16_t   freqs[2][5];
    uint16_t   cnts[5];
    uint8_t    dectab[32];
} FillModel3;

typedef struct OpModel3 {
    uint32_t   cntsum;
    uint16_t   freqs[2][6];
    uint16_t   cnts[6];
    uint8_t    dectab[32];
} OpModel3;

typedef struct RunModel3 {
    uint32_t   cntsum;
    uint16_t   freqs[2][256];
    uint16_t   cnts[256];
    uint8_t    dectab[32];
} RunModel3;

typedef struct SxyModel3 {
    uint32_t   cntsum;
    uint16_t   freqs[2][16];
    uint16_t   cnts[16];
    uint8_t    dectab[32];
} SxyModel3;

typedef struct MVModel3 {
    uint32_t   cntsum;
    uint16_t   freqs[2][512];
    uint16_t   cnts[512];
    uint8_t    dectab[32];
} MVModel3;

#endif /* AVCODEC_SCPR3_H */
