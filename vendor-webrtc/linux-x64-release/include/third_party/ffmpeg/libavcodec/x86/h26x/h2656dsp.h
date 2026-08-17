/*
 * DSP for HEVC/VVC
 *
 * Copyright (C) 2022-2024 Nuo Mi
 * Copyright (c) 2023-2024 Wu Jianhua
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

#ifndef AVCODEC_X86_H26X_H2656DSP_H
#define AVCODEC_X86_H26X_H2656DSP_H

#include "config.h"
#include "libavutil/x86/asm.h"
#include "libavutil/x86/cpu.h"
#include <stdlib.h>

#define H2656_PEL_PROTOTYPE(name, D, opt) \
void ff_h2656_put_ ## name ## _ ## D ## _##opt(int16_t *dst, ptrdiff_t dststride, const uint8_t *_src, ptrdiff_t _srcstride, int height, const int8_t *hf, const int8_t *vf, int width);          \
void ff_h2656_put_uni_ ## name ## _ ## D ## _##opt(uint8_t *_dst, ptrdiff_t _dststride, const uint8_t *_src, ptrdiff_t _srcstride, int height, const int8_t *hf, const int8_t *vf, int width)     \

#define H2656_MC_8TAP_PROTOTYPES(fname, bitd, opt)    \
    H2656_PEL_PROTOTYPE(fname##4,  bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##6,  bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##8,  bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##12,  bitd, opt);       \
    H2656_PEL_PROTOTYPE(fname##16, bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##32, bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##64, bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##128, bitd, opt)

H2656_MC_8TAP_PROTOTYPES(pixels  ,  8, sse4);
H2656_MC_8TAP_PROTOTYPES(pixels  , 10, sse4);
H2656_MC_8TAP_PROTOTYPES(pixels  , 12, sse4);
H2656_MC_8TAP_PROTOTYPES(8tap_h  ,  8, sse4);
H2656_MC_8TAP_PROTOTYPES(8tap_h  , 10, sse4);
H2656_MC_8TAP_PROTOTYPES(8tap_h  , 12, sse4);
H2656_MC_8TAP_PROTOTYPES(8tap_v  ,  8, sse4);
H2656_MC_8TAP_PROTOTYPES(8tap_v  , 10, sse4);
H2656_MC_8TAP_PROTOTYPES(8tap_v  , 12, sse4);
H2656_MC_8TAP_PROTOTYPES(8tap_hv ,  8, sse4);
H2656_MC_8TAP_PROTOTYPES(8tap_hv , 10, sse4);
H2656_MC_8TAP_PROTOTYPES(8tap_hv , 12, sse4);

#define H2656_MC_4TAP_PROTOTYPES(fname, bitd, opt)    \
    H2656_PEL_PROTOTYPE(fname##2,  bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##4,  bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##6,  bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##8,  bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##12, bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##16, bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##32, bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##64, bitd, opt);        \
    H2656_PEL_PROTOTYPE(fname##128, bitd, opt)

#define H2656_MC_4TAP_PROTOTYPES_SSE4(bitd)           \
    H2656_PEL_PROTOTYPE(pixels2, bitd, sse4);         \
    H2656_MC_4TAP_PROTOTYPES(4tap_h, bitd, sse4);     \
    H2656_MC_4TAP_PROTOTYPES(4tap_v, bitd, sse4);     \
    H2656_MC_4TAP_PROTOTYPES(4tap_hv, bitd, sse4);    \

H2656_MC_4TAP_PROTOTYPES_SSE4(8)
H2656_MC_4TAP_PROTOTYPES_SSE4(10)
H2656_MC_4TAP_PROTOTYPES_SSE4(12)

#define H2656_MC_8TAP_PROTOTYPES_AVX2(fname)              \
    H2656_PEL_PROTOTYPE(fname##32 , 8, avx2);             \
    H2656_PEL_PROTOTYPE(fname##64 , 8, avx2);             \
    H2656_PEL_PROTOTYPE(fname##128, 8, avx2);             \
    H2656_PEL_PROTOTYPE(fname##16 ,10, avx2);             \
    H2656_PEL_PROTOTYPE(fname##32 ,10, avx2);             \
    H2656_PEL_PROTOTYPE(fname##64 ,10, avx2);             \
    H2656_PEL_PROTOTYPE(fname##128,10, avx2);             \
    H2656_PEL_PROTOTYPE(fname##16 ,12, avx2);             \
    H2656_PEL_PROTOTYPE(fname##32 ,12, avx2);             \
    H2656_PEL_PROTOTYPE(fname##64 ,12, avx2);             \
    H2656_PEL_PROTOTYPE(fname##128,12, avx2)              \

H2656_MC_8TAP_PROTOTYPES_AVX2(pixels);
H2656_MC_8TAP_PROTOTYPES_AVX2(8tap_h);
H2656_MC_8TAP_PROTOTYPES_AVX2(8tap_v);
H2656_MC_8TAP_PROTOTYPES_AVX2(8tap_hv);
H2656_PEL_PROTOTYPE(8tap_hv16, 8, avx2);

H2656_MC_8TAP_PROTOTYPES_AVX2(4tap_h);
H2656_MC_8TAP_PROTOTYPES_AVX2(4tap_v);
H2656_MC_8TAP_PROTOTYPES_AVX2(4tap_hv);

#endif
