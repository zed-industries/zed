/*
 * copyright (c) 2004 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_UNARY_H
#define AVCODEC_UNARY_H

#include "get_bits.h"

/**
 * Get unary code of limited length
 * @param gb GetBitContext
 * @param[in] stop The bitstop value (unary code of 1's or 0's)
 * @param[in] len Maximum length
 * @return unary 0 based code index. This is also the length in bits of the
 * code excluding the stop bit.
 * (in case len=1)
 * 1            0
 * 0            1
 * (in case len=2)
 * 1            0
 * 01           1
 * 00           2
 * (in case len=3)
 * 1            0
 * 01           1
 * 001          2
 * 000          3
 */
static inline int get_unary(GetBitContext *gb, int stop, int len)
{
    int i;

    for(i = 0; i < len && get_bits1(gb) != stop; i++);
    return i;
}

/**
 * Get unary code terminated by a 0 with a maximum length of 33
 * @param gb GetBitContext
 * @return Unary length/index
 */
static inline int get_unary_0_33(GetBitContext *gb)
{
    return get_unary(gb, 0, 33);
}

static inline int get_unary_0_9(GetBitContext *gb)
{
    return get_unary(gb, 0, 9);
}

#endif /* AVCODEC_UNARY_H */
