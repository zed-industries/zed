/*
 * Copyright (c) 2021 Loongson Technology Corporation Limited
 * Contributed by Hao Chen <chenhao@loongson.cn>
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

#ifndef AVCODEC_LOONGARCH_VC1DSP_LOONGARCH_H
#define AVCODEC_LOONGARCH_VC1DSP_LOONGARCH_H

#include "libavcodec/vc1dsp.h"
#include "libavutil/avassert.h"

void ff_vc1_inv_trans_8x8_lasx(int16_t block[64]);
void ff_vc1_inv_trans_8x8_dc_lasx(uint8_t *dest, ptrdiff_t stride, int16_t *block);
void ff_vc1_inv_trans_8x4_lasx(uint8_t *dest, ptrdiff_t stride, int16_t *block);
void ff_vc1_inv_trans_8x4_dc_lasx(uint8_t *dest, ptrdiff_t stride, int16_t *block);
void ff_vc1_inv_trans_4x8_dc_lasx(uint8_t *dest, ptrdiff_t stride, int16_t *block);
void ff_vc1_inv_trans_4x8_lasx(uint8_t *dest, ptrdiff_t stride, int16_t *blokc);
void ff_vc1_inv_trans_4x4_dc_lasx(uint8_t *dest, ptrdiff_t stride, int16_t *block);
void ff_vc1_inv_trans_4x4_lasx(uint8_t *dest, ptrdiff_t stride, int16_t *block);

#define FF_PUT_VC1_MSPEL_MC_LASX(hmode, vmode)                                \
void ff_put_vc1_mspel_mc ## hmode ## vmode ## _lasx(uint8_t *dst,             \
                                                  const uint8_t *src,         \
                                                  ptrdiff_t stride, int rnd); \
void ff_put_vc1_mspel_mc ## hmode ## vmode ## _16_lasx(uint8_t *dst,          \
                                                  const uint8_t *src,         \
                                                  ptrdiff_t stride, int rnd);

FF_PUT_VC1_MSPEL_MC_LASX(1, 1);
FF_PUT_VC1_MSPEL_MC_LASX(1, 2);
FF_PUT_VC1_MSPEL_MC_LASX(1, 3);

FF_PUT_VC1_MSPEL_MC_LASX(2, 1);
FF_PUT_VC1_MSPEL_MC_LASX(2, 2);
FF_PUT_VC1_MSPEL_MC_LASX(2, 3);

FF_PUT_VC1_MSPEL_MC_LASX(3, 1);
FF_PUT_VC1_MSPEL_MC_LASX(3, 2);
FF_PUT_VC1_MSPEL_MC_LASX(3, 3);

#define FF_PUT_VC1_MSPEL_MC_V_LASX(vmode)                                 \
void ff_put_vc1_mspel_mc0 ## vmode ## _16_lasx(uint8_t *dst,              \
                                               const uint8_t *src,        \
                                               ptrdiff_t stride, int rnd);

FF_PUT_VC1_MSPEL_MC_V_LASX(1);
FF_PUT_VC1_MSPEL_MC_V_LASX(2);
FF_PUT_VC1_MSPEL_MC_V_LASX(3);

#define FF_PUT_VC1_MSPEL_MC_H_LASX(hmode)                                 \
void ff_put_vc1_mspel_mc ## hmode ## 0_16_lasx(uint8_t *dst,              \
                                               const uint8_t *src,        \
                                               ptrdiff_t stride, int rnd);

FF_PUT_VC1_MSPEL_MC_H_LASX(1);
FF_PUT_VC1_MSPEL_MC_H_LASX(2);
FF_PUT_VC1_MSPEL_MC_H_LASX(3);

void ff_put_no_rnd_vc1_chroma_mc8_lasx(uint8_t *dst /* align 8 */,
                                       const uint8_t *src /* align 1 */,
                                       ptrdiff_t stride, int h, int x, int y);

#endif /* AVCODEC_LOONGARCH_VC1DSP_LOONGARCH_H */
