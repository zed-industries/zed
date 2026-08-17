/*
 * arbitrary precision integers
 * Copyright (c) 2004 Michael Niedermayer <michaelni@gmx.at>
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
 * arbitrary precision integers
 * @author Michael Niedermayer <michaelni@gmx.at>
 */

#ifndef AVUTIL_INTEGER_H
#define AVUTIL_INTEGER_H

#include <stdint.h>
#include "attributes.h"

#define AV_INTEGER_SIZE 8

typedef struct AVInteger{
    uint16_t v[AV_INTEGER_SIZE];
} AVInteger;

AVInteger av_add_i(AVInteger a, AVInteger b) av_const;
AVInteger av_sub_i(AVInteger a, AVInteger b) av_const;

/**
 * Return the rounded-down value of the base 2 logarithm of the given
 * AVInteger. This is simply the index of the most significant bit
 * which is 1, or 0 if all bits are 0.
 */
int av_log2_i(AVInteger a) av_const;
AVInteger av_mul_i(AVInteger a, AVInteger b) av_const;

/**
 * Return 0 if a==b, 1 if a>b and -1 if a<b.
 */
int av_cmp_i(AVInteger a, AVInteger b) av_const;

/**
 * bitwise shift
 * @param s the number of bits by which the value should be shifted right,
            may be negative for shifting left
 */
AVInteger av_shr_i(AVInteger a, int s) av_const;

/**
 * Return a % b.
 * @param quot a/b will be stored here.
 */
AVInteger av_mod_i(AVInteger *quot, AVInteger a, AVInteger b);

/**
 * Return a/b.
 */
AVInteger av_div_i(AVInteger a, AVInteger b) av_const;

/**
 * Convert the given int64_t to an AVInteger.
 */
AVInteger av_int2i(int64_t a) av_const;

/**
 * Convert the given AVInteger to an int64_t.
 * If the AVInteger is too large to fit into an int64_t,
 * then only the least significant 64 bits will be used.
 */
int64_t av_i2int(AVInteger a) av_const;

#endif /* AVUTIL_INTEGER_H */
