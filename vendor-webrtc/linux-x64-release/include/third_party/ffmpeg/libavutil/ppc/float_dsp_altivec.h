/*
 * Copyright (c) 2006 Luca Barbato <lu_zero@gentoo.org>
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

#ifndef AVUTIL_PPC_FLOAT_DSP_ALTIVEC_H
#define AVUTIL_PPC_FLOAT_DSP_ALTIVEC_H

void ff_vector_fmul_altivec(float *dst, const float *src0,
                            const float *src1, int len);

void ff_vector_fmul_window_altivec(float *dst, const float *src0,
                                   const float *src1, const float *win,
                                   int len);

void ff_vector_fmul_add_altivec(float *dst, const float *src0,
                                const float *src1, const float *src2,
                                int len);

void ff_vector_fmul_reverse_altivec(float *dst, const float *src0,
                                    const float *src1, int len);

#endif /* AVUTIL_PPC_FLOAT_DSP_ALTIVEC_H */
