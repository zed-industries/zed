/*
 * Various fixed-point math operations
 *
 * Copyright (c) 2008 Vladimir Voroshilov
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

#ifndef AVCODEC_CELP_MATH_H
#define AVCODEC_CELP_MATH_H

#include <stdint.h>

typedef struct CELPMContext {
    /**
     * Return the dot product.
     * @param a input data array
     * @param b input data array
     * @param length number of elements
     *
     * @return dot product = sum of elementwise products
     */
    float (*dot_productf)(const float* a, const float* b, int length);

}CELPMContext;

/**
 * Initialize CELPMContext.
 */
void ff_celp_math_init(CELPMContext *c);
void ff_celp_math_init_mips(CELPMContext *c);

/**
 * fixed-point implementation of exp2(x) in [0; 1] domain.
 * @param power argument to exp2, 0 <= power <= 0x7fff
 *
 * @return value of (1<<20) * exp2(power / (1<<15))
 *         0x8000c <= result <= 0xfffea
 */
int ff_exp2(uint16_t power);

/**
 * Calculate log2(x).
 * @param value function argument, 0 < value <= 7fff ffff
 *
 * @return value of (1<<15) * log2(value)
 */
int ff_log2_q15(uint32_t value);

/**
 * Calculate the dot product of 2 int16_t vectors.
 * @param a input data array
 * @param b input data array
 * @param length number of elements
 *
 * @return dot product = sum of elementwise products
 */
int64_t ff_dot_product(const int16_t *a, const int16_t *b, int length);

/**
 * Shift value left or right depending on sign of offset parameter.
 * @param value value to shift
 * @param offset shift offset
 *
 * @return value << offset, if offset>=0; value >> -offset - otherwise
 */
static inline unsigned bidir_sal(unsigned value, int offset)
{
    if(offset < 0) return value >> -offset;
    else           return value <<  offset;
}

/**
 * Return the dot product.
 * @param a input data array
 * @param b input data array
 * @param length number of elements
 *
 * @return dot product = sum of elementwise products
 */
float ff_dot_productf(const float* a, const float* b, int length);

#endif /* AVCODEC_CELP_MATH_H */
