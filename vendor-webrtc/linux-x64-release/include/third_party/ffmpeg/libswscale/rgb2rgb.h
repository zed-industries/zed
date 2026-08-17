/*
 *  software RGB to RGB converter
 *  pluralize by Software PAL8 to RGB converter
 *               Software YUV to YUV converter
 *               Software YUV to RGB converter
 *  Written by Nick Kurshev.
 *  YUV & runtime CPU stuff by Michael (michaelni@gmx.at)
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

#ifndef SWSCALE_RGB2RGB_H
#define SWSCALE_RGB2RGB_H

#include <stdint.h>

/* A full collection of RGB to RGB(BGR) converters */
extern void (*rgb24tobgr32)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*rgb24tobgr16)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*rgb24tobgr15)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*rgb32tobgr24)(const uint8_t *src, uint8_t *dst, int src_size);
extern void    (*rgb32to16)(const uint8_t *src, uint8_t *dst, int src_size);
extern void    (*rgb32to15)(const uint8_t *src, uint8_t *dst, int src_size);
extern void    (*rgb15to16)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*rgb15tobgr24)(const uint8_t *src, uint8_t *dst, int src_size);
extern void    (*rgb15to32)(const uint8_t *src, uint8_t *dst, int src_size);
extern void    (*rgb16to15)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*rgb16tobgr24)(const uint8_t *src, uint8_t *dst, int src_size);
extern void    (*rgb16to32)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*rgb24tobgr24)(const uint8_t *src, uint8_t *dst, int src_size);
extern void    (*rgb24to16)(const uint8_t *src, uint8_t *dst, int src_size);
extern void    (*rgb24to15)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*rgb32tobgr16)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*rgb32tobgr15)(const uint8_t *src, uint8_t *dst, int src_size);

extern void (*shuffle_bytes_0321)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*shuffle_bytes_2103)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*shuffle_bytes_1230)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*shuffle_bytes_3012)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*shuffle_bytes_3210)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*shuffle_bytes_3102)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*shuffle_bytes_2013)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*shuffle_bytes_2130)(const uint8_t *src, uint8_t *dst, int src_size);
extern void (*shuffle_bytes_1203)(const uint8_t *src, uint8_t *dst, int src_size);

void rgb64tobgr48_nobswap(const uint8_t *src, uint8_t *dst, int src_size);
void   rgb64tobgr48_bswap(const uint8_t *src, uint8_t *dst, int src_size);
void rgb48tobgr48_nobswap(const uint8_t *src, uint8_t *dst, int src_size);
void   rgb48tobgr48_bswap(const uint8_t *src, uint8_t *dst, int src_size);
void    rgb64to48_nobswap(const uint8_t *src, uint8_t *dst, int src_size);
void      rgb64to48_bswap(const uint8_t *src, uint8_t *dst, int src_size);
void rgb48tobgr64_nobswap(const uint8_t *src, uint8_t *dst, int src_size);
void   rgb48tobgr64_bswap(const uint8_t *src, uint8_t *dst, int src_size);
void    rgb48to64_nobswap(const uint8_t *src, uint8_t *dst, int src_size);
void      rgb48to64_bswap(const uint8_t *src, uint8_t *dst, int src_size);
void    rgb24to32(const uint8_t *src, uint8_t *dst, int src_size);
void    rgb32to24(const uint8_t *src, uint8_t *dst, int src_size);
void rgb16tobgr32(const uint8_t *src, uint8_t *dst, int src_size);
void    rgb16to24(const uint8_t *src, uint8_t *dst, int src_size);
void rgb16tobgr16(const uint8_t *src, uint8_t *dst, int src_size);
void rgb16tobgr15(const uint8_t *src, uint8_t *dst, int src_size);
void rgb15tobgr32(const uint8_t *src, uint8_t *dst, int src_size);
void    rgb15to24(const uint8_t *src, uint8_t *dst, int src_size);
void rgb15tobgr16(const uint8_t *src, uint8_t *dst, int src_size);
void rgb15tobgr15(const uint8_t *src, uint8_t *dst, int src_size);
void rgb12tobgr12(const uint8_t *src, uint8_t *dst, int src_size);
void    rgb12to15(const uint8_t *src, uint8_t *dst, int src_size);

void    x2rgb10to48_nobswap(const uint8_t *src, uint8_t *dst, int src_size);
void      x2rgb10to48_bswap(const uint8_t *src, uint8_t *dst, int src_size);
void x2rgb10tobgr48_nobswap(const uint8_t *src, uint8_t *dst, int src_size);
void   x2rgb10tobgr48_bswap(const uint8_t *src, uint8_t *dst, int src_size);
void    x2rgb10to64_nobswap(const uint8_t *src, uint8_t *dst, int src_size);
void      x2rgb10to64_bswap(const uint8_t *src, uint8_t *dst, int src_size);
void x2rgb10tobgr64_nobswap(const uint8_t *src, uint8_t *dst, int src_size);
void   x2rgb10tobgr64_bswap(const uint8_t *src, uint8_t *dst, int src_size);

void ff_rgb24toyv12_c(const uint8_t *src, uint8_t *ydst, uint8_t *udst,
                      uint8_t *vdst, int width, int height, int lumStride,
                      int chromStride, int srcStride, const int32_t *rgb2yuv);

/**
 * Height should be a multiple of 2 and width should be a multiple of 16.
 * (If this is a problem for anyone then tell me, and I will fix it.)
 */
extern void (*yv12toyuy2)(const uint8_t *ysrc, const uint8_t *usrc, const uint8_t *vsrc, uint8_t *dst,
                          int width, int height,
                          int lumStride, int chromStride, int dstStride);

/**
 * Width should be a multiple of 16.
 */
extern void (*yuv422ptoyuy2)(const uint8_t *ysrc, const uint8_t *usrc, const uint8_t *vsrc, uint8_t *dst,
                             int width, int height,
                             int lumStride, int chromStride, int dstStride);

/**
 * Height should be a multiple of 2 and width should be a multiple of 16.
 * (If this is a problem for anyone then tell me, and I will fix it.)
 */
extern void (*yuy2toyv12)(const uint8_t *src, uint8_t *ydst, uint8_t *udst, uint8_t *vdst,
                          int width, int height,
                          int lumStride, int chromStride, int srcStride);

/**
 * Height should be a multiple of 2 and width should be a multiple of 16.
 * (If this is a problem for anyone then tell me, and I will fix it.)
 */
extern void (*yv12touyvy)(const uint8_t *ysrc, const uint8_t *usrc, const uint8_t *vsrc, uint8_t *dst,
                          int width, int height,
                          int lumStride, int chromStride, int dstStride);

/**
 * Width should be a multiple of 16.
 */
extern void (*yuv422ptouyvy)(const uint8_t *ysrc, const uint8_t *usrc, const uint8_t *vsrc, uint8_t *dst,
                             int width, int height,
                             int lumStride, int chromStride, int dstStride);

/**
 * Height should be a multiple of 2 and width should be a multiple of 2.
 * (If this is a problem for anyone then tell me, and I will fix it.)
 */
extern void (*ff_rgb24toyv12)(const uint8_t *src, uint8_t *ydst, uint8_t *udst, uint8_t *vdst,
                              int width, int height,
                              int lumStride, int chromStride, int srcStride,
                              const int32_t *rgb2yuv);
extern void (*planar2x)(const uint8_t *src, uint8_t *dst, int width, int height,
                        int srcStride, int dstStride);

extern void (*interleaveBytes)(const uint8_t *src1, const uint8_t *src2, uint8_t *dst,
                               int width, int height, int src1Stride,
                               int src2Stride, int dstStride);

extern void (*deinterleaveBytes)(const uint8_t *src, uint8_t *dst1, uint8_t *dst2,
                                 int width, int height, int srcStride,
                                 int dst1Stride, int dst2Stride);

extern void (*vu9_to_vu12)(const uint8_t *src1, const uint8_t *src2,
                           uint8_t *dst1, uint8_t *dst2,
                           int width, int height,
                           int srcStride1, int srcStride2,
                           int dstStride1, int dstStride2);

extern void (*yvu9_to_yuy2)(const uint8_t *src1, const uint8_t *src2, const uint8_t *src3,
                            uint8_t *dst,
                            int width, int height,
                            int srcStride1, int srcStride2,
                            int srcStride3, int dstStride);

extern void (*uyvytoyuv420)(uint8_t *ydst, uint8_t *udst, uint8_t *vdst, const uint8_t *src,
                            int width, int height,
                            int lumStride, int chromStride, int srcStride);
extern void (*uyvytoyuv422)(uint8_t *ydst, uint8_t *udst, uint8_t *vdst, const uint8_t *src,
                            int width, int height,
                            int lumStride, int chromStride, int srcStride);
extern void (*yuyvtoyuv420)(uint8_t *ydst, uint8_t *udst, uint8_t *vdst, const uint8_t *src,
                            int width, int height,
                            int lumStride, int chromStride, int srcStride);
extern void (*yuyvtoyuv422)(uint8_t *ydst, uint8_t *udst, uint8_t *vdst, const uint8_t *src,
                            int width, int height,
                            int lumStride, int chromStride, int srcStride);

void ff_sws_rgb2rgb_init(void);

void rgb2rgb_init_aarch64(void);
void rgb2rgb_init_riscv(void);
void rgb2rgb_init_x86(void);
void rgb2rgb_init_loongarch(void);

#endif /* SWSCALE_RGB2RGB_H */
