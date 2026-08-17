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

#ifndef AVCODEC_ARM_CABAC_H
#define AVCODEC_ARM_CABAC_H

#include "config.h"
#if HAVE_ARMV6T2_INLINE

#include "libavutil/attributes.h"
#include "libavutil/internal.h"
#include "libavcodec/cabac.h"

#define get_cabac_inline get_cabac_inline_arm
static av_always_inline int get_cabac_inline_arm(CABACContext *c,
                                                 uint8_t *const state)
{
    int bit;
    void *reg_b, *reg_c, *tmp;

    __asm__ volatile(
        "ldrb       %[bit]        , [%[state]]                  \n\t"
        "add        %[r_b]        , %[tables]   , %[lps_off]    \n\t"
        "mov        %[tmp]        , %[range]                    \n\t"
        "and        %[range]      , %[range]    , #0xC0         \n\t"
        "add        %[r_b]        , %[r_b]      , %[bit]        \n\t"
        "ldrb       %[range]      , [%[r_b], %[range], lsl #1]  \n\t"
        "add        %[r_b]        , %[tables]   , %[norm_off]   \n\t"
        "sub        %[r_c]        , %[tmp]      , %[range]      \n\t"
        "lsl        %[tmp]        , %[r_c]      , #17           \n\t"
        "cmp        %[tmp]        , %[low]                      \n\t"
        "it         gt                                          \n\t"
        "movgt      %[range]      , %[r_c]                      \n\t"
        "itt        cc                                          \n\t"
        "mvncc      %[bit]        , %[bit]                      \n\t"
        "subcc      %[low]        , %[low]      , %[tmp]        \n\t"
        "add        %[r_c]        , %[tables]   , %[mlps_off]   \n\t"
        "ldrb       %[tmp]        , [%[r_b], %[range]]          \n\t"
        "ldrb       %[r_b]        , [%[r_c], %[bit]]            \n\t"
        "lsl        %[low]        , %[low]      , %[tmp]        \n\t"
        "lsl        %[range]      , %[range]    , %[tmp]        \n\t"
        "uxth       %[r_c]        , %[low]                      \n\t"
        "strb       %[r_b]        , [%[state]]                  \n\t"
        "tst        %[r_c]        , %[r_c]                      \n\t"
        "bne        2f                                          \n\t"
        "ldr        %[r_c]        , [%[c], %[byte]]             \n\t"
#if UNCHECKED_BITSTREAM_READER
        "ldrh       %[tmp]        , [%[r_c]]                    \n\t"
        "add        %[r_c]        , %[r_c]      , #2            \n\t"
        "str        %[r_c]        , [%[c], %[byte]]             \n\t"
#else
        "ldr        %[r_b]        , [%[c], %[end]]              \n\t"
        "ldrh       %[tmp]        , [%[r_c]]                    \n\t"
        "cmp        %[r_c]        , %[r_b]                      \n\t"
        "itt        lt                                          \n\t"
        "addlt      %[r_c]        , %[r_c]      , #2            \n\t"
        "strlt      %[r_c]        , [%[c], %[byte]]             \n\t"
#endif
        "sub        %[r_c]        , %[low]      , #1            \n\t"
        "add        %[r_b]        , %[tables]   , %[norm_off]   \n\t"
        "eor        %[r_c]        , %[low]      , %[r_c]        \n\t"
        "rev        %[tmp]        , %[tmp]                      \n\t"
        "lsr        %[r_c]        , %[r_c]      , #15           \n\t"
        "lsr        %[tmp]        , %[tmp]      , #15           \n\t"
        "ldrb       %[r_c]        , [%[r_b], %[r_c]]            \n\t"
        "movw       %[r_b]        , #0xFFFF                     \n\t"
        "sub        %[tmp]        , %[tmp]      , %[r_b]        \n\t"
        "rsb        %[r_c]        , %[r_c]      , #7            \n\t"
        "lsl        %[tmp]        , %[tmp]      , %[r_c]        \n\t"
        "add        %[low]        , %[low]      , %[tmp]        \n\t"
        "2:                                                     \n\t"
        :    [bit]"=&r"(bit),
             [low]"+&r"(c->low),
           [range]"+&r"(c->range),
             [r_b]"=&r"(reg_b),
             [r_c]"=&r"(reg_c),
             [tmp]"=&r"(tmp)
        :        [c]"r"(c),
             [state]"r"(state),
            [tables]"r"(ff_h264_cabac_tables),
              [byte]"M"(offsetof(CABACContext, bytestream)),
               [end]"M"(offsetof(CABACContext, bytestream_end)),
          [norm_off]"I"(H264_NORM_SHIFT_OFFSET),
           [lps_off]"I"(H264_LPS_RANGE_OFFSET),
          [mlps_off]"I"(H264_MLPS_STATE_OFFSET + 128)
        : "memory", "cc"
        );

    return bit & 1;
}
#endif /* HAVE_ARMV6T2_INLINE */

#endif /* AVCODEC_ARM_CABAC_H */
