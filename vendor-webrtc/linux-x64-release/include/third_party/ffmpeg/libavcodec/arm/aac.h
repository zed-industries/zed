/*
 * Copyright (c) 2010 Mans Rullgard <mans@mansr.com>
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

#ifndef AVCODEC_ARM_AAC_H
#define AVCODEC_ARM_AAC_H

#include "config.h"

#if HAVE_NEON_INLINE

#define VMUL2 VMUL2
static inline float *VMUL2(float *dst, const float *v, unsigned idx,
                           const float *scale)
{
    unsigned v0, v1;
    __asm__ ("ubfx     %0,  %6,  #0, #4      \n\t"
             "ubfx     %1,  %6,  #4, #4      \n\t"
             "ldr      %0,  [%5, %0, lsl #2] \n\t"
             "ldr      %1,  [%5, %1, lsl #2] \n\t"
             "vld1.32  {d1[]},   [%7,:32]    \n\t"
             "vmov     d0,  %0,  %1          \n\t"
             "vmul.f32 d0,  d0,  d1          \n\t"
             "vst1.32  {d0},     [%2,:64]!   \n\t"
             : "=&r"(v0), "=&r"(v1), "+r"(dst), "=m"(dst[0]), "=m"(dst[1])
             : "r"(v), "r"(idx), "r"(scale)
             : "d0", "d1");
    return dst;
}

#define VMUL4 VMUL4
static inline float *VMUL4(float *dst, const float *v, unsigned idx,
                           const float *scale)
{
    unsigned v0, v1, v2, v3;
    __asm__ ("ubfx     %0,  %10, #0, #2      \n\t"
             "ubfx     %1,  %10, #2, #2      \n\t"
             "ldr      %0,  [%9, %0, lsl #2] \n\t"
             "ubfx     %2,  %10, #4, #2      \n\t"
             "ldr      %1,  [%9, %1, lsl #2] \n\t"
             "ubfx     %3,  %10, #6, #2      \n\t"
             "ldr      %2,  [%9, %2, lsl #2] \n\t"
             "vmov     d0,  %0,  %1          \n\t"
             "ldr      %3,  [%9, %3, lsl #2] \n\t"
             "vld1.32  {d2[],d3[]},[%11,:32] \n\t"
             "vmov     d1,  %2,  %3          \n\t"
             "vmul.f32 q0,  q0,  q1          \n\t"
             "vst1.32  {q0},     [%4,:128]!  \n\t"
             : "=&r"(v0), "=&r"(v1), "=&r"(v2), "=&r"(v3), "+r"(dst),
               "=m"(dst[0]), "=m"(dst[1]), "=m"(dst[2]), "=m"(dst[3])
             : "r"(v), "r"(idx), "r"(scale)
             : "d0", "d1", "d2", "d3");
    return dst;
}

#define VMUL2S VMUL2S
static inline float *VMUL2S(float *dst, const float *v, unsigned idx,
                            unsigned sign, const float *scale)
{
    unsigned v0, v1, v2, v3;
    __asm__ ("ubfx     %0,  %8,  #0, #4      \n\t"
             "ubfx     %1,  %8,  #4, #4      \n\t"
             "ldr      %0,  [%7, %0, lsl #2] \n\t"
             "lsl      %2,  %10, #30         \n\t"
             "ldr      %1,  [%7, %1, lsl #2] \n\t"
             "lsl      %3,  %10, #31         \n\t"
             "vmov     d0,  %0,  %1          \n\t"
             "bic      %2,  %2,  #1<<30      \n\t"
             "vld1.32  {d1[]},   [%9,:32]    \n\t"
             "vmov     d2,  %2,  %3          \n\t"
             "veor     d0,  d0,  d2          \n\t"
             "vmul.f32 d0,  d0,  d1          \n\t"
             "vst1.32  {d0},     [%4,:64]!   \n\t"
             : "=&r"(v0), "=&r"(v1), "=&r"(v2), "=&r"(v3), "+r"(dst),
               "=m"(dst[0]), "=m"(dst[1])
             : "r"(v), "r"(idx), "r"(scale), "r"(sign)
             : "d0", "d1", "d2");
    return dst;
}

#define VMUL4S VMUL4S
static inline float *VMUL4S(float *dst, const float *v, unsigned idx,
                            unsigned sign, const float *scale)
{
    unsigned v0, v1, v2, v3, nz;
    __asm__ ("vld1.32  {d2[],d3[]},[%13,:32] \n\t"
             "ubfx     %0,  %12, #0, #2      \n\t"
             "ubfx     %1,  %12, #2, #2      \n\t"
             "ldr      %0,  [%11,%0, lsl #2] \n\t"
             "ubfx     %2,  %12, #4, #2      \n\t"
             "ldr      %1,  [%11,%1, lsl #2] \n\t"
             "ubfx     %3,  %12, #6, #2      \n\t"
             "ldr      %2,  [%11,%2, lsl #2] \n\t"
             "vmov     d0,  %0,  %1          \n\t"
             "ldr      %3,  [%11,%3, lsl #2] \n\t"
             "lsr      %6,  %12, #12         \n\t"
             "rbit     %6,  %6               \n\t"
             "vmov     d1,  %2,  %3          \n\t"
             "lsls     %6,  %6,  #1          \n\t"
             "and      %0,  %5,  #1<<31      \n\t"
             "it       cs                    \n\t"
             "lslcs    %5,  %5,  #1          \n\t"
             "lsls     %6,  %6,  #1          \n\t"
             "and      %1,  %5,  #1<<31      \n\t"
             "it       cs                    \n\t"
             "lslcs    %5,  %5,  #1          \n\t"
             "lsls     %6,  %6,  #1          \n\t"
             "and      %2,  %5,  #1<<31      \n\t"
             "it       cs                    \n\t"
             "lslcs    %5,  %5,  #1          \n\t"
             "vmov     d4,  %0,  %1          \n\t"
             "and      %3,  %5,  #1<<31      \n\t"
             "vmov     d5,  %2,  %3          \n\t"
             "veor     q0,  q0,  q2          \n\t"
             "vmul.f32 q0,  q0,  q1          \n\t"
             "vst1.32  {q0},     [%4,:128]!  \n\t"
             : "=&r"(v0), "=&r"(v1), "=&r"(v2), "=&r"(v3), "+r"(dst),
               "+r"(sign), "=r"(nz),
               "=m"(dst[0]), "=m"(dst[1]), "=m"(dst[2]), "=m"(dst[3])
             : "r"(v), "r"(idx), "r"(scale)
             : "cc", "d0", "d1", "d2", "d3", "d4", "d5");
    return dst;
}

#endif /* HAVE_NEON_INLINE */

#endif /* AVCODEC_ARM_AAC_H */
