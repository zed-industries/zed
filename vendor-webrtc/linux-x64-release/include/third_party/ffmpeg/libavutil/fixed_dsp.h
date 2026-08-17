/*
 * Copyright (c) 2012
 *      MIPS Technologies, Inc., California.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the MIPS Technologies, Inc., nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE MIPS TECHNOLOGIES, INC. ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE MIPS TECHNOLOGIES, INC. BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * Author:  Nedeljko Babic (nbabic@mips.com)
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

#ifndef AVUTIL_FIXED_DSP_H
#define AVUTIL_FIXED_DSP_H

#include <stdint.h>
#include "attributes.h"
#include "libavcodec/mathops.h"

typedef struct AVFixedDSPContext {
    /* Assume len is a multiple of 16, and arrays are 32-byte aligned */
    /* Results of multiplications are scaled down by 31 bit (and rounded) if not
     * stated otherwise */

    /**
     * Overlap/add with window function.
     * Result is scaled down by "bits" bits.
     * Used primarily by MDCT-based audio codecs.
     * Source and destination vectors must overlap exactly or not at all.
     *
     * @param dst  result vector
     *             constraints: 16-byte aligned
     * @param src0 first source vector
     *             constraints: 16-byte aligned
     * @param src1 second source vector
     *             constraints: 16-byte aligned
     * @param win  half-window vector
     *             constraints: 16-byte aligned
     * @param len  length of vector
     *             constraints: multiple of 4
     * @param bits scaling parameter
     *
     */
    void (*vector_fmul_window_scaled)(int16_t *dst, const int32_t *src0, const int32_t *src1, const int32_t *win, int len, uint8_t bits);

    /**
     * Overlap/add with window function.
     * Used primarily by MDCT-based audio codecs.
     * Source and destination vectors must overlap exactly or not at all.
     *
     * @param dst  result vector
     *             constraints: 32-byte aligned
     * @param src0 first source vector
     *             constraints: 16-byte aligned
     * @param src1 second source vector
     *             constraints: 16-byte aligned
     * @param win  half-window vector
     *             constraints: 16-byte aligned
     * @param len  length of vector
     *             constraints: multiple of 4
     */
    void (*vector_fmul_window)(int32_t *dst, const int32_t *src0, const int32_t *src1, const int32_t *win, int len);

    /**
     * Fixed-point multiplication that calculates the entry wise product of two
     * vectors of integers and stores the result in a vector of integers.
     *
     * @param dst  output vector
     *             constraints: 32-byte aligned
     * @param src0 first input vector
     *             constraints: 32-byte aligned
     * @param src1 second input vector
     *             constraints: 32-byte aligned
     * @param len  number of elements in the input
     *             constraints: multiple of 16
     */
    void (*vector_fmul)(int *dst, const int *src0, const int *src1,
                        int len);

    void (*vector_fmul_reverse)(int *dst, const int *src0, const int *src1, int len);
    /**
     * Calculate the entry wise product of two vectors of integers, add a third vector of
     * integers and store the result in a vector of integers.
     *
     * @param dst  output vector
     *             constraints: 32-byte aligned
     * @param src0 first input vector
     *             constraints: 32-byte aligned
     * @param src1 second input vector
     *             constraints: 32-byte aligned
     * @param src2 third input vector
     *             constraints: 32-byte aligned
     * @param len  number of elements in the input
     *             constraints: multiple of 16
     */
    void (*vector_fmul_add)(int *dst, const int *src0, const int *src1,
                            const int *src2, int len);

    /**
     * Calculate the scalar product of two vectors of integers.
     *
     * @param v1  first vector, 16-byte aligned
     * @param v2  second vector, 16-byte aligned
     * @param len length of vectors, multiple of 4
     *
     * @return sum of elementwise products
     */
    int (*scalarproduct_fixed)(const int *v1, const int *v2, int len);

    /**
     * Calculate the sum and difference of two vectors of integers.
     *
     * @param v1  first input vector, sum output, 16-byte aligned
     * @param v2  second input vector, difference output, 16-byte aligned
     * @param len length of vectors, multiple of 4
     */
    void (*butterflies_fixed)(int *restrict v1, int *restrict v2, int len);
} AVFixedDSPContext;

/**
 * Allocate and initialize a fixed DSP context.
 * note: should be freed with a av_free call when no longer needed.
 *
 * @param strict  setting to non-zero avoids using functions which may not be IEEE-754 compliant
 */
AVFixedDSPContext * avpriv_alloc_fixed_dsp(int strict);

void ff_fixed_dsp_init_riscv(AVFixedDSPContext *fdsp);
void ff_fixed_dsp_init_x86(AVFixedDSPContext *fdsp);

/**
 * Calculate the square root
 *
 * @param x    input fixed point number
 *
 * @param bits format of fixed point number (32 - bits).bits
 *
 * note: input is normalized to (0, 1) fixed point value
 */

static av_always_inline int fixed_sqrt(int x, int bits)
{
    int retval, bit_mask, guess, square, i;
    int64_t accu;
    int shift1 = 30 - bits;
    int shift2 = bits - 15;

    if (shift1 > 0) retval = ff_sqrt(x << shift1);
    else retval = ff_sqrt(x >> -shift1);

    if (shift2 > 0) {
        retval = retval << shift2;
        bit_mask = (1 << (shift2 - 1));

        for (i=0; i<shift2; i++){
            guess = retval + bit_mask;
            accu = (int64_t)guess * guess;
            square = (int)((accu + bit_mask) >> bits);
            if (x >= square)
                retval += bit_mask;
            bit_mask >>= 1;
        }

    }
    else retval >>= (-shift2);

    return retval;
}

#endif /* AVUTIL_FIXED_DSP_H */
