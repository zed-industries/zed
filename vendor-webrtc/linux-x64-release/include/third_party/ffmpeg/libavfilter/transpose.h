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
#ifndef AVFILTER_TRANSPOSE_H
#define AVFILTER_TRANSPOSE_H

#include <stddef.h>
#include <stdint.h>

enum PassthroughType {
    TRANSPOSE_PT_TYPE_NONE,
    TRANSPOSE_PT_TYPE_LANDSCAPE,
    TRANSPOSE_PT_TYPE_PORTRAIT,
};

enum TransposeDir {
    TRANSPOSE_CCLOCK_FLIP,
    TRANSPOSE_CLOCK,
    TRANSPOSE_CCLOCK,
    TRANSPOSE_CLOCK_FLIP,
    TRANSPOSE_REVERSAL,    // rotate by half-turn
    TRANSPOSE_HFLIP,
    TRANSPOSE_VFLIP,
};

typedef struct TransVtable {
    void (*transpose_8x8)(uint8_t *src, ptrdiff_t src_linesize,
                          uint8_t *dst, ptrdiff_t dst_linesize);
    void (*transpose_block)(uint8_t *src, ptrdiff_t src_linesize,
                            uint8_t *dst, ptrdiff_t dst_linesize,
                            int w, int h);
} TransVtable;

void ff_transpose_init_x86(TransVtable *v, int pixstep);

#endif
