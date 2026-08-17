/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
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

#ifndef DAV1D_COMMON_DUMP_H
#define DAV1D_COMMON_DUMP_H

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#include "common/bitdepth.h"

static inline void append_plane_to_file(const pixel *buf, ptrdiff_t stride,
                                        int w, int h, const char *const file)
{
    FILE *const f = fopen(file, "ab");
    while (h--) {
        fwrite(buf, w * sizeof(pixel), 1, f);
        buf += PXSTRIDE(stride);
    }
    fclose(f);
}

static inline void hex_fdump(FILE *out, const pixel *buf, ptrdiff_t stride,
                             int w, int h, const char *what)
{
    fprintf(out, "%s\n", what);
    while (h--) {
        int x;
        for (x = 0; x < w; x++)
            fprintf(out, " " PIX_HEX_FMT, buf[x]);
        buf += PXSTRIDE(stride);
        fprintf(out, "\n");
    }
}

static inline void hex_dump(const pixel *buf, ptrdiff_t stride,
                            int w, int h, const char *what)
{
    hex_fdump(stdout, buf, stride, w, h, what);
}

static inline void coef_dump(const coef *buf, const int w, const int h,
                             const int len, const char *what)
{
    int y;
    printf("%s\n", what);
    for (y = 0; y < h; y++) {
        int x;
        for (x = 0; x < w; x++)
            printf(" %*d", len, buf[x]);
        buf += w;
        printf("\n");
    }
}

static inline void ac_dump(const int16_t *buf, int w, int h, const char *what)
{
    printf("%s\n", what);
    while (h--) {
        for (int x = 0; x < w; x++)
            printf(" %03d", buf[x]);
        buf += w;
        printf("\n");
    }
}

#endif /* DAV1D_COMMON_DUMP_H */
