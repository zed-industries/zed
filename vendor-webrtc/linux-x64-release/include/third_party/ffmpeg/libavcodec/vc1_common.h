/*
 * VC-1 and WMV3 decoder
 * Copyright (c) 2006-2007 Konstantin Shishkov
 * Partly based on vc9.c (c) 2005 Anonymous, Alex Beregszaszi, Michael Niedermayer
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

#ifndef AVCODEC_VC1_COMMON_H
#define AVCODEC_VC1_COMMON_H

#include <stdint.h>

#include "libavutil/attributes.h"
#include "startcode.h"

/** Markers used in VC-1 AP frame data */
//@{
enum VC1Code {
    VC1_CODE_RES0       = 0x00000100,
    VC1_CODE_ENDOFSEQ   = 0x0000010A,
    VC1_CODE_SLICE,
    VC1_CODE_FIELD,
    VC1_CODE_FRAME,
    VC1_CODE_ENTRYPOINT,
    VC1_CODE_SEQHDR,
};
//@}

#define IS_MARKER(x) (((x) & ~0xFF) == VC1_CODE_RES0)

/** Available Profiles */
//@{
enum Profile {
    PROFILE_SIMPLE,
    PROFILE_MAIN,
    PROFILE_COMPLEX, ///< TODO: WMV9 specific
    PROFILE_ADVANCED
};
//@}

/** Find VC-1 marker in buffer
 * @return position where next marker starts or end of buffer if no marker found
 */
static av_always_inline const uint8_t* find_next_marker(const uint8_t *src, const uint8_t *end)
{
    if (end - src >= 4) {
        uint32_t mrk = 0xFFFFFFFF;
        src = avpriv_find_start_code(src, end, &mrk);
        if (IS_MARKER(mrk))
            return src - 4;
    }
    return end;
}

static av_always_inline int vc1_unescape_buffer(const uint8_t *src, int size, uint8_t *dst)
{
    int dsize = 0, i;

    if (size < 4) {
        for (dsize = 0; dsize < size; dsize++)
            *dst++ = *src++;
        return size;
    }
    for (i = 0; i < size; i++, src++) {
        if (src[0] == 3 && i >= 2 && !src[-1] && !src[-2] && i < size-1 && src[1] < 4) {
            dst[dsize++] = src[1];
            src++;
            i++;
        } else
            dst[dsize++] = *src;
    }
    return dsize;
}

#endif /* AVCODEC_VC1_COMMON_H */
