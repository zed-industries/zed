/*
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVFILTER_PULLUP_H
#define AVFILTER_PULLUP_H

#include "avfilter.h"

typedef struct PullupBuffer {
    int lock[2];
    uint8_t *planes[4];
} PullupBuffer;

typedef struct PullupField {
    int parity;
    PullupBuffer *buffer;
    unsigned flags;
    int breaks;
    int affinity;
    int *diffs;
    int *combs;
    int *vars;
    struct PullupField *prev, *next;
} PullupField;

typedef struct PullupFrame {
    int lock;
    int length;
    int parity;
    PullupBuffer *ifields[4], *ofields[2];
    PullupBuffer *buffer;
} PullupFrame;

typedef struct PullupContext {
    const AVClass *class;
    int junk_left, junk_right, junk_top, junk_bottom;
    int metric_plane;
    int strict_breaks;
    int strict_pairs;
    int metric_w, metric_h, metric_length;
    int metric_offset;
    int nb_planes;
    int planewidth[4];
    int planeheight[4];
    PullupField *first, *last, *head;
    PullupBuffer buffers[10];
    PullupFrame frame;

    int (*diff)(const uint8_t *a, const uint8_t *b, ptrdiff_t s);
    int (*comb)(const uint8_t *a, const uint8_t *b, ptrdiff_t s);
    int (*var )(const uint8_t *a, const uint8_t *b, ptrdiff_t s);
} PullupContext;

void ff_pullup_init_x86(PullupContext *s);

#endif /* AVFILTER_PULLUP_H */
