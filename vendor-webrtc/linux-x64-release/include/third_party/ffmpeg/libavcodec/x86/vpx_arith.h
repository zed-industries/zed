/**
 * VP5 and VP6 compatible video decoder (arith decoder)
 *
 * Copyright (C) 2006  Aurelien Jacobs <aurel@gnuage.org>
 * Copyright (C) 2010  Eli Friedman
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

#ifndef AVCODEC_X86_VPX_ARITH_H
#define AVCODEC_X86_VPX_ARITH_H

#include "libavutil/x86/asm.h"

#if HAVE_INLINE_ASM && HAVE_FAST_CMOV && HAVE_6REGS
#include "libavutil/attributes.h"

#define vpx_rac_get_prob vpx_rac_get_prob
static av_always_inline int vpx_rac_get_prob(VPXRangeCoder *c, uint8_t prob)
{
    unsigned int code_word = vpx_rac_renorm(c);
    unsigned int low = 1 + (((c->high - 1) * prob) >> 8);
    unsigned int low_shift = low << 16;
    int bit = 0;
    c->code_word = code_word;

    __asm__(
        "subl  %4, %1      \n\t"
        "subl  %3, %2      \n\t"
        "setae %b0         \n\t"
        "cmovb %4, %1      \n\t"
        "cmovb %5, %2      \n\t"
        : "+q"(bit), "+&r"(c->high), "+&r"(c->code_word)
        : "r"(low_shift), "r"(low), "r"(code_word)
    );

    return bit;
}
#endif

#endif /* AVCODEC_X86_VPX_ARITH_H */
