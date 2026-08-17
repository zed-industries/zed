/*
 * H.26L/H.264/AVC/JVT/14496-10/... encoder/decoder
 * Copyright (c) 2003-2010 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_H264QPEL_H
#define AVCODEC_H264QPEL_H

#include "qpeldsp.h"

typedef struct H264QpelContext {
    qpel_mc_func put_h264_qpel_pixels_tab[4][16];
    qpel_mc_func avg_h264_qpel_pixels_tab[4][16];
} H264QpelContext;

void ff_h264qpel_init(H264QpelContext *c, int bit_depth);

void ff_h264qpel_init_aarch64(H264QpelContext *c, int bit_depth);
void ff_h264qpel_init_arm(H264QpelContext *c, int bit_depth);
void ff_h264qpel_init_ppc(H264QpelContext *c, int bit_depth);
void ff_h264qpel_init_riscv(H264QpelContext *c, int bit_depth);
void ff_h264qpel_init_x86(H264QpelContext *c, int bit_depth);
void ff_h264qpel_init_mips(H264QpelContext *c, int bit_depth);
void ff_h264qpel_init_loongarch(H264QpelContext *c, int bit_depth);

#endif /* AVCODEC_H264QPEL_H */
