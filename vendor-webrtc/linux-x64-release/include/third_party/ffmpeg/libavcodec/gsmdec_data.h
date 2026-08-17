/*
 * gsm 06.10 decoder data
 * Copyright (c) 2010 Reimar Döffinger <Reimar.Doeffinger@gmx.de>
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

#ifndef AVCODEC_GSMDEC_DATA_H
#define AVCODEC_GSMDEC_DATA_H

#include <stdint.h>

typedef struct GSMContext {
    // Contains first 120 elements from the previous frame
    // (used by long_term_synth according to the "lag"),
    // then in the following 160 elements the current
    // frame is constructed.
    int16_t ref_buf[280];
    int v[9];
    int lar[2][8];
    int lar_idx;
    int msr;
} GSMContext;

extern const uint16_t ff_gsm_long_term_gain_tab[4];
extern const uint8_t ff_gsm_requant_tab[4][8];
extern const int16_t ff_gsm_dequant_tab[64][8];

extern const int* const ff_gsm_apcm_bits[][4];

#endif /* AVCODEC_GSMDEC_DATA_H */
