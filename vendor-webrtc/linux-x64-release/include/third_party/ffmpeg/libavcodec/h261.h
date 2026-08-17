/*
 * H.261 codec
 * Copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>
 * Copyright (c) 2004 Maarten Daniels
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
 * H.261 codec.
 */

#ifndef AVCODEC_H261_H
#define AVCODEC_H261_H

#include "mpegutils.h"
#include "rl.h"

/**
 * H261Context
 */
typedef struct H261Context {
    int mtype;
} H261Context;

#define MB_TYPE_H261_FIL MB_TYPE_CODEC_SPECIFIC

extern const uint8_t ff_h261_mba_code[35];
extern const uint8_t ff_h261_mba_bits[35];
extern const uint8_t ff_h261_mtype_code[10];
extern const uint8_t ff_h261_mtype_bits[10];
extern const uint16_t ff_h261_mtype_map[10];
extern const uint8_t ff_h261_mv_tab[17][2];
extern const uint8_t ff_h261_cbp_tab[63][2];
extern RLTable ff_h261_rl_tcoeff;

extern const uint16_t ff_h261_tcoeff_vlc[65][2];
extern const int8_t ff_h261_tcoeff_level[64];
extern const int8_t ff_h261_tcoeff_run[64];

struct MpegEncContext;
void ff_h261_loop_filter(struct MpegEncContext *s);

#endif /* AVCODEC_H261_H */
