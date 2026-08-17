/*
 * Copyright (c) 2003 Michael Niedermayer
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
 * ASUS V1/V2 encoder/decoder common data.
 */

#ifndef AVCODEC_ASV_H
#define AVCODEC_ASV_H

#include <stdint.h>

#include "avcodec.h"
#include "bswapdsp.h"

typedef struct ASVCommonContext {
    AVCodecContext *avctx;
    BswapDSPContext bbdsp;
    int mb_width;
    int mb_height;
    int mb_width2;
    int mb_height2;
} ASVCommonContext;

extern const uint8_t ff_asv_scantab[64];
extern const uint8_t ff_asv_ccp_tab[17][2];
extern const uint8_t ff_asv_level_tab[7][2];
extern const uint8_t ff_asv_dc_ccp_tab[8][2];
extern const uint8_t ff_asv_ac_ccp_tab[16][2];
extern const uint16_t ff_asv2_level_tab[63][2];

void ff_asv_common_init(AVCodecContext *avctx);

#endif /* AVCODEC_ASV_H */
