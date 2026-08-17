/*
 * Format Conversion Utils
 * Copyright (c) 2000, 2001 Fabrice Bellard
 * Copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_FMTCONVERT_H
#define AVCODEC_FMTCONVERT_H

#include <stdint.h>

typedef struct FmtConvertContext {
    /**
     * Convert an array of int32_t to float and multiply by a float value.
     * @param dst destination array of float.
     *            constraints: 16-byte aligned
     * @param src source array of int32_t.
     *            constraints: 16-byte aligned
     * @param len number of elements to convert.
     *            constraints: multiple of 8
     */
    void (*int32_to_float_fmul_scalar)(float *dst, const int32_t *src,
                                       float mul, int len);

    /**
     * Convert an array of int32_t to float and multiply by a float value from another array,
     * stepping along the float array once for each 8 integers.
     * @param c   pointer to FmtConvertContext.
     * @param dst destination array of float.
     *            constraints: 16-byte aligned
     * @param src source array of int32_t.
     *            constraints: 16-byte aligned
     * @param mul source array of float multipliers.
     * @param len number of elements to convert.
     *            constraints: multiple of 8
     */
    void (*int32_to_float_fmul_array8)(struct FmtConvertContext *c,
                                       float *dst, const int32_t *src,
                                       const float *mul, int len);

} FmtConvertContext;

void ff_fmt_convert_init(FmtConvertContext *c);

void ff_fmt_convert_init_aarch64(FmtConvertContext *c);
void ff_fmt_convert_init_arm(FmtConvertContext *c);
void ff_fmt_convert_init_ppc(FmtConvertContext *c);
void ff_fmt_convert_init_riscv(FmtConvertContext *c);
void ff_fmt_convert_init_x86(FmtConvertContext *c);
void ff_fmt_convert_init_mips(FmtConvertContext *c);

#endif /* AVCODEC_FMTCONVERT_H */
