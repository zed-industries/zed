/*
 * MJPEG encoder
 * Copyright (c) 2016 William Ma, Ted Ying, Jerry Jiang
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
 * Huffman table generation for MJPEG encoder.
 */

#ifndef AVCODEC_MJPEGENC_HUFFMAN_H
#define AVCODEC_MJPEGENC_HUFFMAN_H

#include <stdint.h>

typedef struct MJpegEncHuffmanContext {
    int val_count[256];
} MJpegEncHuffmanContext;

// Uses the package merge algorithm to compute the Huffman table.
void ff_mjpeg_encode_huffman_init(MJpegEncHuffmanContext *s);
static inline void ff_mjpeg_encode_huffman_increment(MJpegEncHuffmanContext *s,
                                                     uint8_t val)
{
    s->val_count[val]++;
}
void ff_mjpeg_encode_huffman_close(MJpegEncHuffmanContext *s,
                                   uint8_t bits[17], uint8_t val[],
                                   int max_nval);


/**
 * Used to assign a occurrence count or "probability" to an input value
 */
typedef struct PTable {
    int value;  ///< input value
    int prob;   ///< number of occurences of this value in input
} PTable;

/**
 * Used to store intermediate lists in the package merge algorithm
 */
typedef struct PackageMergerList {
    int nitems;             ///< number of items in the list and probability      ex. 4
    int item_idx[515];      ///< index range for each item in items                   0, 2, 5, 9, 13
    int probability[514];   ///< probability of each item                             3, 8, 18, 46
    int items[257 * 16];    ///< chain of all individual values that make up items    A, B, A, B, C, A, B, C, D, C, D, D, E
} PackageMergerList;

/**
 * Used to store optimal huffman encoding results
 */
typedef struct HuffTable {
    int code;       ///< code is the input value
    int length;     ///< length of the encoding
} HuffTable;

void ff_mjpegenc_huffman_compute_bits(PTable *prob_table, HuffTable *distincts,
                                      int size, int max_length);
#endif /* AVCODEC_MJPEGENC_HUFFMAN_H */
