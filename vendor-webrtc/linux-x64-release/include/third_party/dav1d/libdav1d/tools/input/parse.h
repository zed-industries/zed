/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * Copyright © 2019, James Almer <jamrial@gmail.com>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_INPUT_PARSE_H
#define DAV1D_INPUT_PARSE_H

#include <limits.h>

#include "dav1d/headers.h"

static int leb128(FILE *const f, size_t *const len) {
    uint64_t val = 0;
    unsigned i = 0, more;
    do {
        uint8_t v;
        if (fread(&v, 1, 1, f) < 1)
            return -1;
        more = v & 0x80;
        val |= ((uint64_t) (v & 0x7F)) << (i * 7);
        i++;
    } while (more && i < 8);
    if (val > UINT_MAX || more)
        return -1;
    *len = (size_t) val;
    return i;
}

// these functions are based on an implementation from FFmpeg, and relicensed
// with author's permission

static int leb(const uint8_t *ptr, int sz, size_t *const len) {
    uint64_t val = 0;
    unsigned i = 0, more;
    do {
        if (!sz--) return -1;
        const int v = *ptr++;
        more = v & 0x80;
        val |= ((uint64_t) (v & 0x7F)) << (i * 7);
        i++;
    } while (more && i < 8);
    if (val > UINT_MAX || more)
        return -1;
    *len = (size_t) val;
    return i;
}

static inline int parse_obu_header(const uint8_t *buf, int buf_size,
                                   size_t *const obu_size,
                                   enum Dav1dObuType *const type,
                                   const int allow_implicit_size)
{
    int ret, extension_flag, has_size_flag;

    if (!buf_size)
        return -1;
    if (*buf & 0x80) // obu_forbidden_bit
        return -1;

    *type = (*buf & 0x78) >> 3;
    extension_flag = (*buf & 0x4) >> 2;
    has_size_flag  = (*buf & 0x2) >> 1;
    // ignore obu_reserved_1bit
    buf++;
    buf_size--;

    if (extension_flag) {
        if (!buf_size)
            return -1;
        buf++;
        buf_size--;
        // ignore fields
    }

    if (has_size_flag) {
        ret = leb(buf, buf_size, obu_size);
        if (ret < 0)
            return -1;
        return (int) *obu_size + ret + 1 + extension_flag;
    } else if (!allow_implicit_size)
        return -1;

    *obu_size = buf_size;
    return buf_size + 1 + extension_flag;
}

#endif /* DAV1D_INPUT_PARSE_H */
