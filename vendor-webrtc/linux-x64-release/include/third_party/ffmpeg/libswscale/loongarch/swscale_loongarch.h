/*
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 * Contributed by Hao Chen(chenhao@loongson.cn)
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

#ifndef SWSCALE_LOONGARCH_SWSCALE_LOONGARCH_H
#define SWSCALE_LOONGARCH_SWSCALE_LOONGARCH_H

#include "libswscale/swscale.h"
#include "libswscale/swscale_internal.h"
#include "config.h"

void ff_hscale_8_to_15_lsx(SwsInternal *c, int16_t *dst, int dstW,
                           const uint8_t *src, const int16_t *filter,
                           const int32_t *filterPos, int filterSize);

void ff_hscale_8_to_19_lsx(SwsInternal *c, int16_t *_dst, int dstW,
                           const uint8_t *src, const int16_t *filter,
                           const int32_t *filterPos, int filterSize);

void ff_hscale_16_to_15_lsx(SwsInternal *c, int16_t *_dst, int dstW,
                            const uint8_t *_src, const int16_t *filter,
                            const int32_t *filterPos, int filterSize);

void ff_hscale_16_to_15_sub_lsx(SwsInternal *c, int16_t *_dst, int dstW,
                                const uint8_t *_src, const int16_t *filter,
                                const int32_t *filterPos, int filterSize, int sh);

void ff_hscale_16_to_19_lsx(SwsInternal *c, int16_t *_dst, int dstW,
                            const uint8_t *_src, const int16_t *filter,
                            const int32_t *filterPos, int filterSize);

void ff_hscale_16_to_19_sub_lsx(SwsInternal *c, int16_t *_dst, int dstW,
                                const uint8_t *_src, const int16_t *filter,
                                const int32_t *filterPos, int filterSize, int sh);

void lumRangeFromJpeg_lsx(int16_t *dst, int width);
void chrRangeFromJpeg_lsx(int16_t *dstU, int16_t *dstV, int width);
void lumRangeToJpeg_lsx(int16_t *dst, int width);
void chrRangeToJpeg_lsx(int16_t *dstU, int16_t *dstV, int width);

void planar_rgb_to_uv_lsx(uint8_t *_dstU, uint8_t *_dstV, const uint8_t *src[4],
                          int width, int32_t *rgb2yuv, void *opq);

void planar_rgb_to_y_lsx(uint8_t *_dst, const uint8_t *src[4], int width,
                         int32_t *rgb2yuv, void *opq);

void yuv2planeX_8_lsx(const int16_t *filter, int filterSize,
                         const int16_t **src, uint8_t *dest, int dstW,
                         const uint8_t *dither, int offset);

void yuv2plane1_8_lsx(const int16_t *src, uint8_t *dest, int dstW,
                      const uint8_t *dither, int offset);

void yuy2ToUV_lsx(uint8_t *dstU, uint8_t *dstV, const uint8_t *unused0, const uint8_t *src1,
                  const uint8_t *src2, int width, uint32_t *unused, void *opq);

void yvy2ToUV_lsx(uint8_t *dstU, uint8_t *dstV, const uint8_t *unused0, const uint8_t *src1,
                  const uint8_t *src2, int width, uint32_t *unused, void *opq);

void uyvyToUV_lsx(uint8_t *dstU, uint8_t *dstV, const uint8_t *unused0, const uint8_t *src1,
                  const uint8_t *src2, int width, uint32_t *unused, void *opq);

void nv12ToUV_lsx(uint8_t *dstU, uint8_t *dstV, const uint8_t *unused0, const uint8_t *src1,
                  const uint8_t *src2, int width, uint32_t *unused, void *opq);

void nv21ToUV_lsx(uint8_t *dstU, uint8_t *dstV, const uint8_t *unused0, const uint8_t *src1,
                  const uint8_t *src2, int width, uint32_t *unused, void *opq);

void abgrToA_lsx(uint8_t *_dst, const uint8_t *src, const uint8_t *unused1,
                 const uint8_t *unused2, int width, uint32_t *unused, void *opq);

void rgbaToA_lsx(uint8_t *_dst, const uint8_t *src, const uint8_t *unused1,
                 const uint8_t *unused2, int width, uint32_t *unused, void *opq);

av_cold void ff_sws_init_input_lsx(SwsInternal *c);

av_cold void ff_sws_init_output_lsx(SwsInternal *c,
                                    yuv2planar1_fn *yuv2plane1,
                                    yuv2planarX_fn *yuv2planeX,
                                    yuv2interleavedX_fn *yuv2nv12cX,
                                    yuv2packed1_fn *yuv2packed1,
                                    yuv2packed2_fn *yuv2packed2,
                                    yuv2packedX_fn *yuv2packedX,
                                    yuv2anyX_fn *yuv2anyX);

int yuv420_rgb24_lsx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                     int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

int yuv420_bgr24_lsx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                     int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

int yuv420_rgba32_lsx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                      int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

int yuv420_bgra32_lsx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                      int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

int yuv420_argb32_lsx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                      int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

int yuv420_abgr32_lsx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                      int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

#if HAVE_LASX
void ff_hscale_8_to_15_lasx(SwsInternal *c, int16_t *dst, int dstW,
                            const uint8_t *src, const int16_t *filter,
                            const int32_t *filterPos, int filterSize);

void ff_hscale_8_to_19_lasx(SwsInternal *c, int16_t *_dst, int dstW,
                            const uint8_t *src, const int16_t *filter,
                            const int32_t *filterPos, int filterSize);

void ff_hscale_16_to_19_lasx(SwsInternal *c, int16_t *_dst, int dstW,
                             const uint8_t *_src, const int16_t *filter,
                             const int32_t *filterPos, int filterSize);

void ff_hscale_16_to_15_lasx(SwsInternal *c, int16_t *dst, int dstW,
                             const uint8_t *_src, const int16_t *filter,
                             const int32_t *filterPos, int filterSize);

void lumRangeFromJpeg_lasx(int16_t *dst, int width);
void chrRangeFromJpeg_lasx(int16_t *dstU, int16_t *dstV, int width);
void lumRangeToJpeg_lasx(int16_t *dst, int width);
void chrRangeToJpeg_lasx(int16_t *dstU, int16_t *dstV, int width);

void planar_rgb_to_uv_lasx(uint8_t *_dstU, uint8_t *_dstV, const uint8_t *src[4],
                           int width, int32_t *rgb2yuv, void *opq);

void planar_rgb_to_y_lasx(uint8_t *_dst, const uint8_t *src[4], int width,
                          int32_t *rgb2yuv, void *opq);

int yuv420_rgb24_lasx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                      int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

int yuv420_bgr24_lasx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                      int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

int yuv420_rgba32_lasx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                       int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

int yuv420_bgra32_lasx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                       int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

int yuv420_argb32_lasx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                       int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

int yuv420_abgr32_lasx(SwsInternal *c, const uint8_t *const src[], const int srcStride[],
                       int srcSliceY, int srcSliceH, uint8_t *const dst[], const int dstStride[]);

void ff_interleave_bytes_lasx(const uint8_t *src1, const uint8_t *src2,
                              uint8_t *dest, int width, int height,
                              int src1Stride, int src2Stride, int dstStride);

void yuv2planeX_8_lasx(const int16_t *filter, int filterSize,
                          const int16_t **src, uint8_t *dest, int dstW,
                          const uint8_t *dither, int offset);

void yuv2plane1_8_lasx(const int16_t *src, uint8_t *dest, int dstW,
                      const uint8_t *dither, int offset);

void yuy2ToUV_lasx(uint8_t *dstU, uint8_t *dstV, const uint8_t *unused0, const uint8_t *src1,
                   const uint8_t *src2, int width, uint32_t *unused, void *opq);

void yvy2ToUV_lasx(uint8_t *dstU, uint8_t *dstV, const uint8_t *unused0, const uint8_t *src1,
                   const uint8_t *src2, int width, uint32_t *unused, void *opq);

void uyvyToUV_lasx(uint8_t *dstU, uint8_t *dstV, const uint8_t *unused0, const uint8_t *src1,
                   const uint8_t *src2, int width, uint32_t *unused, void *opq);

void nv12ToUV_lasx(uint8_t *dstU, uint8_t *dstV, const uint8_t *unused0, const uint8_t *src1,
                   const uint8_t *src2, int width, uint32_t *unused, void *opq);

void nv21ToUV_lasx(uint8_t *dstU, uint8_t *dstV, const uint8_t *unused0, const uint8_t *src1,
                   const uint8_t *src2, int width, uint32_t *unused, void *opq);

void abgrToA_lasx(uint8_t *_dst, const uint8_t *src, const uint8_t *unused1,
                  const uint8_t *unused2, int width, uint32_t *unused, void *opq);

void rgbaToA_lasx(uint8_t *_dst, const uint8_t *src, const uint8_t *unused1,
                  const uint8_t *unused2, int width, uint32_t *unused, void *opq);

av_cold void ff_sws_init_input_lasx(SwsInternal *c);

av_cold void ff_sws_init_output_lasx(SwsInternal *c,
                                     yuv2planar1_fn *yuv2plane1,
                                     yuv2planarX_fn *yuv2planeX,
                                     yuv2interleavedX_fn *yuv2nv12cX,
                                     yuv2packed1_fn *yuv2packed1,
                                     yuv2packed2_fn *yuv2packed2,
                                     yuv2packedX_fn *yuv2packedX,
                                     yuv2anyX_fn *yuv2anyX);
#endif // #if HAVE_LASX

#endif /* SWSCALE_LOONGARCH_SWSCALE_LOONGARCH_H */
