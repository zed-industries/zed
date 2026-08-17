/*
 * Loongson  optimized cabac
 *
 * Copyright (c) 2020 Loongson Technology Corporation Limited
 * Contributed by Shiyou Yin <yinshiyou-hf@loongson.cn>
 *                Gu Xiwei(guxiwei-hf@loongson.cn)
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

#ifndef AVCODEC_LOONGARCH_CABAC_H
#define AVCODEC_LOONGARCH_CABAC_H

#include "libavutil/attributes.h"
#include "libavcodec/cabac.h"
#include "config.h"

#define GET_CABAC_LOONGARCH_UNCBSR                                      \
    "ld.bu        %[bit],        %[state],       0x0           \n\t"    \
    "andi         %[tmp0],       %[c_range],     0xC0          \n\t"    \
    "slli.d       %[tmp0],       %[tmp0],        0x01          \n\t"    \
    "add.d        %[tmp0],       %[tmp0],        %[tables]     \n\t"    \
    "add.d        %[tmp0],       %[tmp0],        %[bit]        \n\t"    \
    /* tmp1: RangeLPS */                                                \
    "ld.bu        %[tmp1],       %[tmp0],        %[lps_off]    \n\t"    \
                                                                        \
    "sub.d        %[c_range],    %[c_range],     %[tmp1]       \n\t"    \
    "slli.d       %[tmp0],       %[c_range],     0x11          \n\t"    \
    "bge          %[tmp0],       %[c_low],       1f            \n\t"    \
    "move         %[c_range],    %[tmp1]                       \n\t"    \
    "nor          %[bit],        %[bit],         %[bit]        \n\t"    \
    "sub.d        %[c_low],      %[c_low],       %[tmp0]       \n\t"    \
                                                                        \
    "1:                                                        \n\t"    \
    /* tmp1: *state */                                                  \
    "add.d        %[tmp0],       %[tables],      %[bit]        \n\t"    \
    "ld.bu        %[tmp1],       %[tmp0],        %[mlps_off]   \n\t"    \
    /* tmp2: lps_mask */                                                \
    "add.d        %[tmp0],       %[tables],      %[c_range]    \n\t"    \
    "ld.bu        %[tmp2],       %[tmp0],        %[norm_off]   \n\t"    \
                                                                        \
    "andi         %[bit],        %[bit],         0x01          \n\t"    \
    "st.b         %[tmp1],       %[state],       0x0           \n\t"    \
    "sll.d        %[c_range],    %[c_range],     %[tmp2]       \n\t"    \
    "sll.d        %[c_low],      %[c_low],       %[tmp2]       \n\t"    \
                                                                        \
    "and          %[tmp1],       %[c_low],       %[cabac_mask] \n\t"    \
    "bnez         %[tmp1],       1f                            \n\t"    \
    "ld.hu        %[tmp1],       %[c_bytestream], 0x0          \n\t"    \
    "ctz.d        %[tmp0],       %[c_low]                      \n\t"    \
    "addi.d       %[tmp2],       %[tmp0],        -16           \n\t"    \
    "revb.2h      %[tmp0],       %[tmp1]                       \n\t"    \
    "slli.d       %[tmp0],       %[tmp0],        0x01          \n\t"    \
    "sub.d        %[tmp0],       %[tmp0],        %[cabac_mask] \n\t"    \
    "sll.d        %[tmp0],       %[tmp0],        %[tmp2]       \n\t"    \
    "add.d        %[c_low],      %[c_low],       %[tmp0]       \n\t"    \
    "addi.d       %[c_bytestream], %[c_bytestream],     0x02   \n\t"    \
    "1:                                                        \n\t"    \

#define GET_CABAC_LOONGARCH                                             \
    "ld.bu        %[bit],        %[state],       0x0           \n\t"    \
    "andi         %[tmp0],       %[c_range],     0xC0          \n\t"    \
    "slli.d       %[tmp0],       %[tmp0],        0x01          \n\t"    \
    "add.d        %[tmp0],       %[tmp0],        %[tables]     \n\t"    \
    "add.d        %[tmp0],       %[tmp0],        %[bit]        \n\t"    \
    /* tmp1: RangeLPS */                                                \
    "ld.bu        %[tmp1],       %[tmp0],        %[lps_off]    \n\t"    \
                                                                        \
    "sub.d        %[c_range],    %[c_range],     %[tmp1]       \n\t"    \
    "slli.d       %[tmp0],       %[c_range],     0x11          \n\t"    \
    "bge          %[tmp0],       %[c_low],       1f            \n\t"    \
    "move         %[c_range],    %[tmp1]                       \n\t"    \
    "nor          %[bit],        %[bit],         %[bit]        \n\t"    \
    "sub.d        %[c_low],      %[c_low],       %[tmp0]       \n\t"    \
                                                                        \
    "1:                                                        \n\t"    \
    /* tmp1: *state */                                                  \
    "add.d        %[tmp0],       %[tables],      %[bit]        \n\t"    \
    "ld.bu        %[tmp1],       %[tmp0],        %[mlps_off]   \n\t"    \
    /* tmp2: lps_mask */                                                \
    "add.d        %[tmp0],       %[tables],      %[c_range]    \n\t"    \
    "ld.bu        %[tmp2],       %[tmp0],        %[norm_off]   \n\t"    \
                                                                        \
    "andi         %[bit],        %[bit],         0x01          \n\t"    \
    "st.b         %[tmp1],       %[state],       0x0           \n\t"    \
    "sll.d        %[c_range],    %[c_range],     %[tmp2]       \n\t"    \
    "sll.d        %[c_low],      %[c_low],       %[tmp2]       \n\t"    \
                                                                        \
    "and          %[tmp1],       %[c_low],       %[cabac_mask] \n\t"    \
    "bnez         %[tmp1],       1f                            \n\t"    \
    "ld.hu        %[tmp1],       %[c_bytestream], 0x0          \n\t"    \
    "ctz.d        %[tmp0],       %[c_low]                      \n\t"    \
    "addi.d       %[tmp2],       %[tmp0],        -16           \n\t"    \
    "revb.2h      %[tmp0],       %[tmp1]                       \n\t"    \
    "slli.d       %[tmp0],       %[tmp0],        0x01          \n\t"    \
    "sub.d        %[tmp0],       %[tmp0],        %[cabac_mask] \n\t"    \
    "sll.d        %[tmp0],       %[tmp0],        %[tmp2]       \n\t"    \
                                                                        \
    "add.d        %[c_low],      %[c_low],       %[tmp0]       \n\t"    \
                                                                        \
    "slt      %[tmp0],  %[c_bytestream],  %[c_bytestream_end]  \n\t"    \
    "add.d    %[c_bytestream], %[c_bytestream],     %[tmp0]    \n\t"    \
    "add.d    %[c_bytestream], %[c_bytestream],     %[tmp0]    \n\t"    \
    "1:                                                        \n\t"    \

#define get_cabac_inline get_cabac_inline_loongarch
static av_always_inline
int get_cabac_inline_loongarch(CABACContext *c, uint8_t * const state)
{
    int64_t tmp0, tmp1, tmp2, bit;

    __asm__ volatile (
#if UNCHECKED_BITSTREAM_READER
        GET_CABAC_LOONGARCH_UNCBSR
#else
        GET_CABAC_LOONGARCH
#endif
    : [bit]"=&r"(bit), [tmp0]"=&r"(tmp0), [tmp1]"=&r"(tmp1), [tmp2]"=&r"(tmp2),
      [c_range]"+&r"(c->range), [c_low]"+&r"(c->low),
      [c_bytestream]"+&r"(c->bytestream)
    : [state]"r"(state), [tables]"r"(ff_h264_cabac_tables),
#if !UNCHECKED_BITSTREAM_READER
      [c_bytestream_end]"r"(c->bytestream_end),
#endif
      [lps_off]"i"(H264_LPS_RANGE_OFFSET),
      [mlps_off]"i"(H264_MLPS_STATE_OFFSET + 128),
      [norm_off]"i"(H264_NORM_SHIFT_OFFSET),
      [cabac_mask]"r"(CABAC_MASK)
    : "memory"
    );

    return bit;
}

#define get_cabac_bypass get_cabac_bypass_loongarch
static av_always_inline int get_cabac_bypass_loongarch(CABACContext *c)
{
    int64_t tmp0, tmp1, tmp2;
    int res = 0;
    __asm__ volatile(
        "slli.d     %[c_low],        %[c_low],        0x01                \n\t"
        "and        %[tmp0],         %[c_low],        %[cabac_mask]       \n\t"
        "bnez       %[tmp0],         1f                                   \n\t"
        "ld.hu      %[tmp1],         %[c_bytestream], 0x0                 \n\t"
#if UNCHECKED_BITSTREAM_READER
        "addi.d     %[c_bytestream], %[c_bytestream], 0x02                \n\t"
#else
        "slt        %[tmp0],         %[c_bytestream], %[c_bytestream_end] \n\t"
        "add.d      %[c_bytestream], %[c_bytestream], %[tmp0]             \n\t"
        "add.d      %[c_bytestream], %[c_bytestream], %[tmp0]             \n\t"
#endif
        "revb.2h    %[tmp1],         %[tmp1]                              \n\t"
        "slli.d     %[tmp1],         %[tmp1],         0x01                \n\t"
        "sub.d      %[tmp1],         %[tmp1],         %[cabac_mask]       \n\t"
        "add.d      %[c_low],        %[c_low],        %[tmp1]             \n\t"
        "1:                                                               \n\t"
        "slli.d     %[tmp1],         %[c_range],      0x11                \n\t"
        "slt        %[tmp0],         %[c_low],        %[tmp1]             \n\t"
        "sub.d      %[tmp1],         %[c_low],        %[tmp1]             \n\t"
        "masknez    %[tmp2],         %[one],          %[tmp0]             \n\t"
        "maskeqz    %[res],          %[res],          %[tmp0]             \n\t"
        "or         %[res],          %[res],          %[tmp2]             \n\t"
        "masknez    %[tmp2],         %[tmp1],         %[tmp0]             \n\t"
        "maskeqz    %[c_low],        %[c_low],        %[tmp0]             \n\t"
        "or         %[c_low],        %[c_low],        %[tmp2]             \n\t"
        : [tmp0]"=&r"(tmp0), [tmp1]"=&r"(tmp1), [tmp2]"=&r"(tmp2),
          [c_range]"+&r"(c->range), [c_low]"+&r"(c->low),
          [c_bytestream]"+&r"(c->bytestream), [res]"+&r"(res)
        : [cabac_mask]"r"(CABAC_MASK),
#if !UNCHECKED_BITSTREAM_READER
          [c_bytestream_end]"r"(c->bytestream_end),
#endif
          [one]"r"(0x01)
        : "memory"
    );
    return res;
}

#define get_cabac_bypass_sign get_cabac_bypass_sign_loongarch
static av_always_inline
int get_cabac_bypass_sign_loongarch(CABACContext *c, int val)
{
    int64_t tmp0, tmp1;
    int res = val;
    __asm__ volatile(
        "slli.d     %[c_low],        %[c_low],        0x01                \n\t"
        "and        %[tmp0],         %[c_low],        %[cabac_mask]       \n\t"
        "bnez       %[tmp0],         1f                                   \n\t"
        "ld.hu      %[tmp1],         %[c_bytestream], 0x0                 \n\t"
#if UNCHECKED_BITSTREAM_READER
        "addi.d     %[c_bytestream], %[c_bytestream], 0x02                \n\t"
#else
        "slt        %[tmp0],         %[c_bytestream], %[c_bytestream_end] \n\t"
        "add.d      %[c_bytestream], %[c_bytestream], %[tmp0]             \n\t"
        "add.d      %[c_bytestream], %[c_bytestream], %[tmp0]             \n\t"
#endif
        "revb.2h    %[tmp1],         %[tmp1]                              \n\t"
        "slli.d     %[tmp1],         %[tmp1],         0x01                \n\t"
        "sub.d      %[tmp1],         %[tmp1],         %[cabac_mask]       \n\t"
        "add.d      %[c_low],        %[c_low],        %[tmp1]             \n\t"
        "1:                                                               \n\t"
        "slli.d     %[tmp1],         %[c_range],      0x11                \n\t"
        "slt        %[tmp0],         %[c_low],        %[tmp1]             \n\t"
        "sub.d      %[tmp1],         %[c_low],        %[tmp1]             \n\t"
        "masknez    %[tmp1],         %[tmp1],         %[tmp0]             \n\t"
        "maskeqz    %[c_low],        %[c_low],        %[tmp0]             \n\t"
        "or         %[c_low],        %[c_low],        %[tmp1]             \n\t"
        "sub.d      %[tmp1],         %[zero],         %[res]              \n\t"
        "maskeqz    %[tmp1],         %[tmp1],         %[tmp0]             \n\t"
        "masknez    %[res],          %[res],          %[tmp0]             \n\t"
        "or         %[res],          %[res],          %[tmp1]             \n\t"
        : [tmp0]"=&r"(tmp0), [tmp1]"=&r"(tmp1), [res]"+&r"(res),
          [c_range]"+&r"(c->range), [c_low]"+&r"(c->low),
          [c_bytestream]"+&r"(c->bytestream)
        : [cabac_mask]"r"(CABAC_MASK),
#if !UNCHECKED_BITSTREAM_READER
          [c_bytestream_end]"r"(c->bytestream_end),
#endif
          [zero]"r"(0x0)
        : "memory"
    );

    return res;
}
#endif /* AVCODEC_LOONGARCH_CABAC_H */
