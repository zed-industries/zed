/*
 * ELS (Entropy Logarithmic-Scale) decoder
 *
 * Copyright (c) 2013 Maxim Poliakovski
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
 * Entropy Logarithmic-Scale binary arithmetic coder
 */

#ifndef AVCODEC_ELSDEC_H
#define AVCODEC_ELSDEC_H

#include <stdint.h>
#include <sys/types.h>

#define ELS_EXPGOLOMB_LEN   10

typedef struct ElsDecCtx {
    const uint8_t *in_buf;
    unsigned x;
    size_t data_size;
    int j, t, diff, err;
} ElsDecCtx;

typedef struct ElsRungNode {
    uint8_t  rung;
    uint16_t next_index;
} ElsRungNode;

typedef struct ElsUnsignedRung {
    uint8_t      prefix_rung[ELS_EXPGOLOMB_LEN + 1];
    ElsRungNode  *rem_rung_list;
    size_t       rung_list_size;
    uint16_t     avail_index;
} ElsUnsignedRung;

void ff_els_decoder_init(ElsDecCtx *ctx, const uint8_t *in, size_t data_size);
void ff_els_decoder_uninit(ElsUnsignedRung *rung);
int  ff_els_decode_bit(ElsDecCtx *ctx, unsigned char *rung);
unsigned ff_els_decode_unsigned(ElsDecCtx *ctx, ElsUnsignedRung *ur);

#endif /* AVCODEC_ELSDEC_H */
