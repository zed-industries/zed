/*
 * Loongson SIMD optimized h264chroma
 *
 * Copyright (c) 2018 Loongson Technology Corporation Limited
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

#ifndef AVCODEC_MIPS_CABAC_H
#define AVCODEC_MIPS_CABAC_H

#include "libavutil/attributes.h"
#include "libavcodec/cabac.h"
#include "libavutil/mips/mmiutils.h"
#include "config.h"

#if !HAVE_MIPS32R6 && !HAVE_MIPS64R6
#define get_cabac_inline get_cabac_inline_mips
static av_always_inline int get_cabac_inline_mips(CABACContext *c,
                                                  uint8_t * const state){
    mips_reg tmp0, tmp1, tmp2, bit;

    __asm__ volatile (
        "lbu          %[bit],        0(%[state])                   \n\t"
        "and          %[tmp0],       %[c_range],     0xC0          \n\t"
        PTR_SLL      "%[tmp0],       %[tmp0],        0x01          \n\t"
        PTR_ADDU     "%[tmp0],       %[tmp0],        %[tables]     \n\t"
        PTR_ADDU     "%[tmp0],       %[tmp0],        %[bit]        \n\t"
        /* tmp1: RangeLPS */
        "lbu          %[tmp1],       %[lps_off](%[tmp0])           \n\t"

        PTR_SUBU     "%[c_range],    %[c_range],     %[tmp1]       \n\t"
        PTR_SLL      "%[tmp0],       %[c_range],     0x11          \n\t"
        "slt          %[tmp2],       %[tmp0],        %[c_low]      \n\t"
        "beqz         %[tmp2],       1f                            \n\t"
        "move         %[c_range],    %[tmp1]                       \n\t"
        "not          %[bit],        %[bit]                        \n\t"
        PTR_SUBU     "%[c_low],      %[c_low],       %[tmp0]       \n\t"

        "1:                                                        \n\t"
        /* tmp1: *state */
        PTR_ADDU     "%[tmp0],       %[tables],      %[bit]        \n\t"
        "lbu          %[tmp1],       %[mlps_off](%[tmp0])          \n\t"
        /* tmp2: lps_mask */
        PTR_ADDU     "%[tmp0],       %[tables],      %[c_range]    \n\t"
        "lbu          %[tmp2],       %[norm_off](%[tmp0])          \n\t"

        "sb           %[tmp1],       0(%[state])                   \n\t"
        "and          %[bit],        %[bit],         0x01          \n\t"
        PTR_SLL      "%[c_range],    %[c_range],     %[tmp2]       \n\t"
        PTR_SLL      "%[c_low],      %[c_low],       %[tmp2]       \n\t"

        "and          %[tmp1],       %[c_low],       %[cabac_mask] \n\t"
        "bnez         %[tmp1],       1f                            \n\t"
        PTR_ADDIU    "%[tmp0],       %[c_low],       -0X01         \n\t"
        "xor          %[tmp0],       %[c_low],       %[tmp0]       \n\t"
        PTR_SRA      "%[tmp0],       %[tmp0],        0x0f          \n\t"
        PTR_ADDU     "%[tmp0],       %[tmp0],        %[tables]     \n\t"
        /* tmp2: ff_h264_norm_shift[x >> (CABAC_BITS - 1)] */
        "lbu          %[tmp2],       %[norm_off](%[tmp0])          \n\t"
#if HAVE_BIGENDIAN
        "lhu          %[tmp0],       0(%[c_bytestream])            \n\t"
#else
        "lhu          %[tmp0],       0(%[c_bytestream])            \n\t"
#if HAVE_MIPS32R2 || HAVE_MIPS64R2
        "wsbh         %[tmp0],       %[tmp0]                       \n\t"
#else
        "and          %[tmp1],      %[tmp0],         0xff00ff00    \n\t"
        "srl          %[tmp1],      %[tmp1],         8             \n\t"
        "and          %[tmp0],      %[tmp0],         0x00ff00ff    \n\t"
        "sll          %[tmp0],      %[tmp0],         8             \n\t"
        "or           %[tmp0],      %[tmp0],         %[tmp1]       \n\t"
#endif
#endif
        PTR_SLL      "%[tmp0],       %[tmp0],        0x01          \n\t"
        PTR_SUBU     "%[tmp0],       %[tmp0],        %[cabac_mask] \n\t"

        "li           %[tmp1],       0x07                          \n\t"
        PTR_SUBU     "%[tmp1],       %[tmp1],        %[tmp2]       \n\t"
        PTR_SLL      "%[tmp0],       %[tmp0],        %[tmp1]       \n\t"
        PTR_ADDU     "%[c_low],      %[c_low],       %[tmp0]       \n\t"

#if UNCHECKED_BITSTREAM_READER
        PTR_ADDIU    "%[c_bytestream], %[c_bytestream],     0x02                 \n\t"
#else
        "slt          %[tmp0],         %[c_bytestream],     %[c_bytestream_end]  \n\t"
        PTR_ADDIU    "%[tmp2],         %[c_bytestream],     0x02                 \n\t"
        "movn         %[c_bytestream], %[tmp2],             %[tmp0]              \n\t"
#endif
        "1:                                                        \n\t"
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

#define get_cabac_bypass get_cabac_bypass_mips
static av_always_inline int get_cabac_bypass_mips(CABACContext *c)
{
    mips_reg tmp0, tmp1;
    int res = 0;
    __asm__ volatile(
        PTR_SLL    "%[c_low],        %[c_low],        0x01                \n\t"
        "and        %[tmp0],         %[c_low],        %[cabac_mask]       \n\t"
        "bnez       %[tmp0],         1f                                   \n\t"
#if HAVE_BIGENDIAN
        "lhu        %[tmp1],         0(%[c_bytestream])                   \n\t"
#else
        "lhu        %[tmp1],         0(%[c_bytestream])                   \n\t"
#if HAVE_MIPS32R2 || HAVE_MIPS64R2
        "wsbh       %[tmp1],         %[tmp1]                              \n\t"
#else
        "and        %[tmp0],         %[tmp1],         0xff00ff00          \n\t"
        "srl        %[tmp0],         %[tmp0],         8                   \n\t"
        "and        %[tmp1],         %[tmp1],         0x00ff00ff          \n\t"
        "sll        %[tmp1],         %[tmp1],         8                   \n\t"
        "or         %[tmp1],         %[tmp1],         %[tmp0]             \n\t"
#endif
#endif
        PTR_SLL    "%[tmp1],         %[tmp1],         0x01                \n\t"
        PTR_SUBU   "%[tmp1],         %[tmp1],         %[cabac_mask]       \n\t"
        PTR_ADDU   "%[c_low],        %[c_low],        %[tmp1]             \n\t"
#if UNCHECKED_BITSTREAM_READER
        PTR_ADDIU  "%[c_bytestream], %[c_bytestream], 0x02                \n\t"
#else
        "slt        %[tmp0],         %[c_bytestream], %[c_bytestream_end] \n\t"
        PTR_ADDIU  "%[tmp1],         %[c_bytestream], 0x02                \n\t"
        "movn       %[c_bytestream], %[tmp1],         %[tmp0]             \n\t"
#endif
        "1:                                                               \n\t"
        PTR_SLL    "%[tmp1],         %[c_range],      0x11                \n\t"
        "slt        %[tmp0],         %[c_low],        %[tmp1]             \n\t"
        PTR_SUBU   "%[tmp1],         %[c_low],        %[tmp1]             \n\t"
        "movz       %[res],          %[one],          %[tmp0]             \n\t"
        "movz       %[c_low],        %[tmp1],         %[tmp0]             \n\t"
        : [tmp0]"=&r"(tmp0), [tmp1]"=&r"(tmp1), [res]"+&r"(res),
          [c_range]"+&r"(c->range), [c_low]"+&r"(c->low),
          [c_bytestream]"+&r"(c->bytestream)
        : [cabac_mask]"r"(CABAC_MASK),
#if !UNCHECKED_BITSTREAM_READER
          [c_bytestream_end]"r"(c->bytestream_end),
#endif
          [one]"r"(0x01)
        : "memory"
    );
    return res;
}

#define get_cabac_bypass_sign get_cabac_bypass_sign_mips
static av_always_inline int get_cabac_bypass_sign_mips(CABACContext *c, int val)
{
    mips_reg tmp0, tmp1;
    int res = val;
    __asm__ volatile(
        PTR_SLL    "%[c_low],        %[c_low],        0x01                \n\t"
        "and        %[tmp0],         %[c_low],        %[cabac_mask]       \n\t"
        "bnez       %[tmp0],         1f                                   \n\t"
#if HAVE_BIGENDIAN
        "lhu        %[tmp1],         0(%[c_bytestream])                   \n\t"
#else
        "lhu        %[tmp1],         0(%[c_bytestream])                   \n\t"
#if HAVE_MIPS32R2 || HAVE_MIPS64R2
        "wsbh       %[tmp1],         %[tmp1]                              \n\t"
#else
        "and        %[tmp0],         %[tmp1],         0xff00ff00          \n\t"
        "srl        %[tmp0],         %[tmp0],         8                   \n\t"
        "and        %[tmp1],         %[tmp1],         0x00ff00ff          \n\t"
        "sll        %[tmp1],         %[tmp1],         8                   \n\t"
        "or         %[tmp1],         %[tmp1],         %[tmp0]             \n\t"
#endif
#endif
        PTR_SLL    "%[tmp1],         %[tmp1],         0x01                \n\t"
        PTR_SUBU   "%[tmp1],         %[tmp1],         %[cabac_mask]       \n\t"
        PTR_ADDU   "%[c_low],        %[c_low],        %[tmp1]             \n\t"
#if UNCHECKED_BITSTREAM_READER
        PTR_ADDIU  "%[c_bytestream], %[c_bytestream], 0x02                \n\t"
#else
        "slt        %[tmp0],         %[c_bytestream], %[c_bytestream_end] \n\t"
        PTR_ADDIU  "%[tmp1],         %[c_bytestream], 0x02                \n\t"
        "movn       %[c_bytestream], %[tmp1],         %[tmp0]             \n\t"
#endif
        "1:                                                               \n\t"
        PTR_SLL    "%[tmp1],         %[c_range],      0x11                \n\t"
        "slt        %[tmp0],         %[c_low],        %[tmp1]             \n\t"
        PTR_SUBU   "%[tmp1],         %[c_low],        %[tmp1]             \n\t"
        "movz       %[c_low],        %[tmp1],         %[tmp0]             \n\t"
        PTR_SUBU   "%[tmp1],         %[zero],         %[res]              \n\t"
        "movn       %[res],          %[tmp1],         %[tmp0]             \n\t"
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
#endif /* !HAVE_MIPS32R6 && !HAVE_MIPS64R6 */
#endif /* AVCODEC_MIPS_CABAC_H */
