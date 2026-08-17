/*
 * AAC definitions and structures
 * Copyright (c) 2024 Lynne
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

#ifndef AVCODEC_AAC_AACDEC_AC_H
#define AVCODEC_AAC_AACDEC_AC_H

#include "libavcodec/get_bits.h"

typedef struct AACArithState {
    uint8_t last[512 /* 2048 / 4 */ + 1];
    int last_len;
    uint8_t cur[4];
    uint16_t state_pre;
} AACArithState;

typedef struct AACArith {
    uint16_t low;
    uint16_t high;
    uint16_t val;
} AACArith;

#define FF_AAC_AC_ESCAPE 16

uint32_t ff_aac_ac_map_process(AACArithState *state, int reset, int len);
uint32_t ff_aac_ac_get_context(AACArithState *state, uint32_t old_c, int idx, int len);
uint32_t ff_aac_ac_get_pk(uint32_t c);

void ff_aac_ac_update_context(AACArithState *state, int idx, uint16_t a, uint16_t b);
void ff_aac_ac_init(AACArith *ac, GetBitContext *gb);

uint16_t ff_aac_ac_decode(AACArith *ac, GetBitContext *gb,
                          const uint16_t *cdf, uint16_t cdf_len);

void ff_aac_ac_finish(AACArithState *state, int offset, int nb);

#endif /* AVCODEC_AACDEC_AC_H */
