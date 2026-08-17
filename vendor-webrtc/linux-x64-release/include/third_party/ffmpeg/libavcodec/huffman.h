/*
 * Copyright (C) 2007  Aurelien Jacobs <aurel@gnuage.org>
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
 * huffman tree builder and VLC generator
 */

#ifndef AVCODEC_HUFFMAN_H
#define AVCODEC_HUFFMAN_H

#include <stdint.h>

#include "vlc.h"

typedef struct Node {
    int16_t  sym;
    int16_t  n0;
    uint32_t count;
} Node;

#define FF_HUFFMAN_FLAG_HNODE_FIRST 0x01
#define FF_HUFFMAN_FLAG_ZERO_COUNT  0x02
#define FF_HUFFMAN_BITS 10

typedef int (*HuffCmp)(const void *va, const void *vb);
int ff_huff_build_tree(void *logctx, VLC *vlc, int nb_codes, int nb_bits,
                       Node *nodes, HuffCmp cmp, int flags);

int ff_huff_gen_len_table(uint8_t *dst, const uint64_t *stats, int n, int skip0);

#endif /* AVCODEC_HUFFMAN_H */
