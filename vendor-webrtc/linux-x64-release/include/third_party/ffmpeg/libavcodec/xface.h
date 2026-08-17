/*
 * Copyright (c) 1990 James Ashton - Sydney University
 * Copyright (c) 2012 Stefano Sabatini
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
 * X-Face common definitions.
 */

#ifndef AVCODEC_XFACE_H
#define AVCODEC_XFACE_H

#include <stdint.h>

/* define the face size - 48x48x1 */
#define XFACE_WIDTH  48
#define XFACE_HEIGHT 48
#define XFACE_PIXELS (XFACE_WIDTH * XFACE_HEIGHT)

/* compressed output uses the full range of printable characters.
 * In ASCII these are in a contiguous block so we just need to know
 * the first and last. The total number of printables is needed too. */
#define XFACE_FIRST_PRINT '!'
#define XFACE_LAST_PRINT '~'
#define XFACE_PRINTS (XFACE_LAST_PRINT - XFACE_FIRST_PRINT + 1)

/*
 * Image is encoded as a big integer, using characters from '~' to
 * '!', for a total of 94 symbols. In order to express
 * 48x48 pixels with the worst case encoding 666 symbols should
 * be sufficient.
 */
#define XFACE_MAX_DIGITS 666

#define XFACE_BITSPERWORD 8
#define XFACE_WORDCARRY (1 << XFACE_BITSPERWORD)
#define XFACE_WORDMASK (XFACE_WORDCARRY - 1)

// This must be larger or equal to log256(94^XFACE_MAX_DIGITS)
#define XFACE_MAX_WORDS 546

/* Portable, very large unsigned integer arithmetic is needed.
 * Implementation uses arrays of WORDs. */
typedef struct {
    int nb_words;
    uint8_t words[XFACE_MAX_WORDS];
} BigInt;

/**
 * Add a to b storing the result in b.
 */
void ff_big_add(BigInt *b, uint8_t a);

/**
 * Divide b by a storing the result in b and the remainder in the word
 * pointed to by r.
 */
void ff_big_div(BigInt *b, uint8_t a, uint8_t *r);

/**
 * Multiply a by b storing the result in b.
 */
void ff_big_mul(BigInt *b, uint8_t a);

/* Each face is encoded using 9 octrees of 16x16 each. Each level of the
 * trees has varying probabilities of being white, grey or black.
 * The table below is based on sampling many faces */
enum XFaceColor { XFACE_COLOR_BLACK = 0, XFACE_COLOR_GREY, XFACE_COLOR_WHITE };

/* Data of varying probabilities are encoded by a value in the range 0 - 255.
 * The probability of the data determines the range of possible encodings.
 * Offset gives the first possible encoding of the range. */
typedef struct {
    uint8_t range;
    uint8_t offset;
} ProbRange;

extern const ProbRange ff_xface_probranges_per_level[4][3];

extern const ProbRange ff_xface_probranges_2x2[16];

void ff_xface_generate_face(uint8_t *dst, uint8_t * const src);

#endif /* AVCODEC_XFACE_H */
