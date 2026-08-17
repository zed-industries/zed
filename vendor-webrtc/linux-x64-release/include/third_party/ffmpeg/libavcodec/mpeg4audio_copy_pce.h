/*
 * MPEG-4 Audio PCE copying function
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

#ifndef AVCODEC_MPEG4AUDIO_COPY_PCE_H
#define AVCODEC_MPEG4AUDIO_COPY_PCE_H

#include "libavutil/attributes.h"

#include "get_bits.h"
#include "put_bits.h"

static av_always_inline unsigned int ff_pce_copy_bits(PutBitContext *pb,
                                                      GetBitContext *gb,
                                                      int bits)
{
    unsigned int el = get_bits(gb, bits);
    put_bits(pb, bits, el);
    return el;
}

static inline int ff_copy_pce_data(PutBitContext *pb, GetBitContext *gb)
{
    int five_bit_ch, four_bit_ch, comment_size, bits;
    int offset = put_bits_count(pb);

    ff_pce_copy_bits(pb, gb, 10);               // Tag, Object Type, Frequency
    five_bit_ch  = ff_pce_copy_bits(pb, gb, 4); // Front
    five_bit_ch += ff_pce_copy_bits(pb, gb, 4); // Side
    five_bit_ch += ff_pce_copy_bits(pb, gb, 4); // Back
    four_bit_ch  = ff_pce_copy_bits(pb, gb, 2); // LFE
    four_bit_ch += ff_pce_copy_bits(pb, gb, 3); // Data
    five_bit_ch += ff_pce_copy_bits(pb, gb, 4); // Coupling
    if (ff_pce_copy_bits(pb, gb, 1))            // Mono Mixdown
        ff_pce_copy_bits(pb, gb, 4);
    if (ff_pce_copy_bits(pb, gb, 1))            // Stereo Mixdown
        ff_pce_copy_bits(pb, gb, 4);
    if (ff_pce_copy_bits(pb, gb, 1))            // Matrix Mixdown
        ff_pce_copy_bits(pb, gb, 3);
    for (bits = five_bit_ch*5+four_bit_ch*4; bits > 16; bits -= 16)
        ff_pce_copy_bits(pb, gb, 16);
    if (bits)
        ff_pce_copy_bits(pb, gb, bits);
    align_put_bits(pb);
    align_get_bits(gb);
    comment_size = ff_pce_copy_bits(pb, gb, 8);
    for (; comment_size > 0; comment_size--)
        ff_pce_copy_bits(pb, gb, 8);

    return put_bits_count(pb) - offset;
}

#endif /* AVCODEC_MPEG4AUDIO_COPY_PCE_H */
