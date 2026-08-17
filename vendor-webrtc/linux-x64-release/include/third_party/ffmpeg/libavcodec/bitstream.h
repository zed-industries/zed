/*
 * Copyright (c) 2016 Alexandra Hájková
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

/**
 * @file
 * bitstream reader API header.
 */

/*
 * Bit order (endianness) is controlled by #defining BITSTREAM_BE and/or
 * BITSTREAM_LE before #including this header. The corresponding bitreading
 * functions are provided as bits_*_be()/bits_*_le() respectively.
 *
 * If neither or only BITSTREAM_BE is defined, then the default (unsuffixed)
 * bits_*() will resolve to the big-endian implementation. If only BITSTREAM_LE
 * is defined, little-endian will be the default.
 *
 * If both are defined, then the default can be controlled by defining at most
 * one of BITSTREAM_DEFAULT_LE/BE. When BITSTREAM_DEFAULT_* is not defined, no
 * default is provided and you must always explicitly use the _be() or _le()
 * variants.
 */

#ifndef AVCODEC_BITSTREAM_H
#define AVCODEC_BITSTREAM_H

#include <stdint.h>

#include "config.h"

#include "libavutil/avassert.h"
#include "libavutil/common.h"
#include "libavutil/intreadwrite.h"
#include "libavutil/log.h"

#include "mathops.h"
#include "vlc.h"

#ifndef UNCHECKED_BITSTREAM_READER
#define UNCHECKED_BITSTREAM_READER !CONFIG_SAFE_BITSTREAM_READER
#endif

// select the default endianness, if any
#if defined(BITSTREAM_LE) && defined(BITSTREAM_BE)

# if defined(BITSTREAM_DEFAULT_BE) && defined(BITSTREAM_DEFAULT_LE)
#  error "At most one of BITSTREAM_DEFAULT_BE/LE must be defined"
# elif   defined(BITSTREAM_DEFAULT_BE)
#  define BITS_DEFAULT_BE
# elif   defined(BITSTREAM_DEFAULT_LE)
#  define BITS_DEFAULT_LE
# endif

#elif defined(BITSTREAM_LE)
# define BITS_DEFAULT_LE
#else // select BE if nothing is requested explicitly
# define BITS_DEFAULT_BE
# define BITSTREAM_WANT_BE
#endif

#if defined(BITS_DEFAULT_LE)

# define BitstreamContext   BitstreamContextLE
# define bits_init          bits_init_le
# define bits_init8         bits_init8_le
# define bits_tell          bits_tell_le
# define bits_size          bits_size_le
# define bits_left          bits_left_le
# define bits_read_bit      bits_read_bit_le
# define bits_read_nz       bits_read_nz_le
# define bits_read          bits_read_le
# define bits_read_63       bits_read_63_le
# define bits_read_64       bits_read_64_le
# define bits_read_signed   bits_read_signed_le
# define bits_read_signed_nz bits_read_signed_nz_le
# define bits_peek_nz       bits_peek_nz_le
# define bits_peek          bits_peek_le
# define bits_peek_signed   bits_peek_signed_le
# define bits_peek_signed_nz bits_peek_signed_nz_le
# define bits_skip          bits_skip_le
# define bits_seek          bits_seek_le
# define bits_align         bits_align_le
# define bits_read_xbits    bits_read_xbits_le
# define bits_decode012     bits_decode012_le
# define bits_decode210     bits_decode210_le
# define bits_apply_sign    bits_apply_sign_le
# define bits_read_vlc      bits_read_vlc_le
# define bits_read_vlc_multi bits_read_vlc_multi_le

#elif defined(BITS_DEFAULT_BE)

# define BitstreamContext   BitstreamContextBE
# define bits_init          bits_init_be
# define bits_init8         bits_init8_be
# define bits_tell          bits_tell_be
# define bits_size          bits_size_be
# define bits_left          bits_left_be
# define bits_read_bit      bits_read_bit_be
# define bits_read_nz       bits_read_nz_be
# define bits_read          bits_read_be
# define bits_read_63       bits_read_63_be
# define bits_read_64       bits_read_64_be
# define bits_read_signed   bits_read_signed_be
# define bits_read_signed_nz bits_read_signed_nz_be
# define bits_peek_nz       bits_peek_nz_be
# define bits_peek          bits_peek_be
# define bits_peek_signed   bits_peek_signed_be
# define bits_peek_signed_nz bits_peek_signed_nz_be
# define bits_skip          bits_skip_be
# define bits_seek          bits_seek_be
# define bits_align         bits_align_be
# define bits_read_xbits    bits_read_xbits_be
# define bits_decode012     bits_decode012_be
# define bits_decode210     bits_decode210_be
# define bits_apply_sign    bits_apply_sign_be
# define bits_read_vlc      bits_read_vlc_be
# define bits_read_vlc_multi bits_read_vlc_multi_be

#endif

#undef BITS_DEFAULT_LE
#undef BITS_DEFAULT_BE

#define BITS_RL_VLC(level, run, bc, table, bits, max_depth) \
    do {                                                    \
        int n, nb_bits;                                     \
        unsigned int index = bits_peek(bc, bits);           \
        level = table[index].level;                         \
        n     = table[index].len;                           \
                                                            \
        if (max_depth > 1 && n < 0) {                       \
            bits_skip(bc, bits);                            \
                                                            \
            nb_bits = -n;                                   \
                                                            \
            index = bits_peek(bc, nb_bits) + level;         \
            level = table[index].level;                     \
            n     = table[index].len;                       \
            if (max_depth > 2 && n < 0) {                   \
                bits_skip(bc, nb_bits);                     \
                nb_bits = -n;                               \
                                                            \
                index = bits_peek(bc, nb_bits) + level;     \
                level = table[index].level;                 \
                n     = table[index].len;                   \
            }                                               \
        }                                                   \
        run = table[index].run;                             \
        bits_skip(bc, n);                                   \
    } while (0)

#endif /* AVCODEC_BITSTREAM_H */

// the following is deliberately outside of the standard #include guards

#if  defined(BITSTREAM_LE) && !defined(BITSTREAM_WANT_LE)
# define BITSTREAM_WANT_LE
#endif

#if defined(BITSTREAM_BE) && !defined(BITSTREAM_WANT_BE)
# define BITSTREAM_WANT_BE
#endif

#if defined(BITSTREAM_WANT_LE) && !defined(AVCODEC_BITSTREAM_LE)
#define AVCODEC_BITSTREAM_LE

#define BITSTREAM_TEMPLATE_LE
#include "bitstream_template.h"
#undef BITSTREAM_TEMPLATE_LE

#endif

#if defined(BITSTREAM_WANT_BE) && !defined(AVCODEC_BITSTREAM_BE)
#define AVCODEC_BITSTREAM_BE

#include "bitstream_template.h"

#endif

#undef BITSTREAM_WANT_LE
#undef BITSTREAM_WANT_BE
