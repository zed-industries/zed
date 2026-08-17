/*
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

#ifndef AVCODEC_AARCH64_CABAC_H
#define AVCODEC_AARCH64_CABAC_H

#include "config.h"
#if HAVE_INLINE_ASM

#include "libavutil/attributes.h"
#include "libavutil/internal.h"
#include "libavcodec/cabac.h"

#define get_cabac_inline get_cabac_inline_aarch64
static av_always_inline int get_cabac_inline_aarch64(CABACContext *c,
                                                     uint8_t *const state)
{
    int bit;
    void *reg_a, *reg_b, *reg_c, *tmp;

    __asm__ volatile(
        "ldrb       %w[bit]       , [%[state]]                  \n\t"
        "add        %[r_b]        , %[tables]   , %[lps_off]    \n\t"
        "mov        %w[tmp]       , %w[range]                   \n\t"
        "and        %w[range]     , %w[range]   , #0xC0         \n\t"
        "lsl        %w[r_c]       , %w[range]   , #1            \n\t"
        "add        %[r_b]        , %[r_b]      , %w[bit], UXTW \n\t"
        "ldrb       %w[range]     , [%[r_b], %w[r_c], SXTW]     \n\t"
        "sub        %w[r_c]       , %w[tmp]     , %w[range]     \n\t"
        "lsl        %w[tmp]       , %w[r_c]     , #17           \n\t"
        "cmp        %w[tmp]       , %w[low]                     \n\t"
        "csel       %w[tmp]       , %w[tmp]     , wzr      , cc \n\t"
        "csel       %w[range]     , %w[r_c]     , %w[range], gt \n\t"
        "cinv       %w[bit]       , %w[bit]     , cc            \n\t"
        "sub        %w[low]       , %w[low]     , %w[tmp]       \n\t"
        "add        %[r_b]        , %[tables]   , %[norm_off]   \n\t"
        "add        %[r_a]        , %[tables]   , %[mlps_off]   \n\t"
        "ldrb       %w[tmp]       , [%[r_b], %w[range], SXTW]   \n\t"
        "ldrb       %w[r_a]       , [%[r_a], %w[bit], SXTW]     \n\t"
        "lsl        %w[low]       , %w[low]     , %w[tmp]       \n\t"
        "lsl        %w[range]     , %w[range]   , %w[tmp]       \n\t"
        "uxth       %w[r_c]       , %w[low]                     \n\t"
        "strb       %w[r_a]       , [%[state]]                  \n\t"
        "cbnz       %w[r_c]       , 2f                          \n\t"
        "ldr        %[r_c]        , [%[c], %[byte]]             \n\t"
        "ldr        %[r_a]        , [%[c], %[end]]              \n\t"
        "ldrh       %w[tmp]       , [%[r_c]]                    \n\t"
        "cmp        %[r_c]        , %[r_a]                      \n\t"
        "b.ge       1f                                          \n\t"
        "add        %[r_a]        , %[r_c]      , #2            \n\t"
        "str        %[r_a]        , [%[c], %[byte]]             \n\t"
        "1:                                                     \n\t"
        "sub        %w[r_c]       , %w[low]     , #1            \n\t"
        "eor        %w[r_c]       , %w[r_c]     , %w[low]       \n\t"
        "rev        %w[tmp]       , %w[tmp]                     \n\t"
        "lsr        %w[r_c]       , %w[r_c]     , #15           \n\t"
        "lsr        %w[tmp]       , %w[tmp]     , #15           \n\t"
        "ldrb       %w[r_c]       , [%[r_b], %w[r_c], SXTW]     \n\t"
        "mov        %w[r_b]       , #0xFFFF                     \n\t"
        "mov        %w[r_a]       , #7                          \n\t"
        "sub        %w[tmp]       , %w[tmp]     , %w[r_b]       \n\t"
        "sub        %w[r_c]       , %w[r_a]     , %w[r_c]       \n\t"
        "lsl        %w[tmp]       , %w[tmp]     , %w[r_c]       \n\t"
        "add        %w[low]       , %w[low]     , %w[tmp]       \n\t"
        "2:                                                     \n\t"
        :    [bit]"=&r"(bit),
             [low]"+&r"(c->low),
           [range]"+&r"(c->range),
             [r_a]"=&r"(reg_a),
             [r_b]"=&r"(reg_b),
             [r_c]"=&r"(reg_c),
             [tmp]"=&r"(tmp)
        :        [c]"r"(c),
             [state]"r"(state),
            [tables]"r"(ff_h264_cabac_tables),
              [byte]"i"(offsetof(CABACContext, bytestream)),
               [end]"i"(offsetof(CABACContext, bytestream_end)),
          [norm_off]"I"(H264_NORM_SHIFT_OFFSET),
           [lps_off]"I"(H264_LPS_RANGE_OFFSET),
          [mlps_off]"I"(H264_MLPS_STATE_OFFSET + 128)
        : "memory", "cc"
        );

    return bit & 1;
}

#endif /* HAVE_INLINE_ASM */

#endif /* AVCODEC_AARCH64_CABAC_H */
