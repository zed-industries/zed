/*
 * copyright (c) 2006 Oded Shimon <ods15@ods15.dyndns.org>
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

#ifndef AVCODEC_VORBIS_H
#define AVCODEC_VORBIS_H

#include <stdint.h>

typedef struct vorbis_floor1_entry {
    uint16_t x;
    uint16_t sort;
    uint16_t low;
    uint16_t high;
} vorbis_floor1_entry;

int ff_vorbis_ready_floor1_list(void *logctx,
                                vorbis_floor1_entry *list, int values);
unsigned int ff_vorbis_nth_root(unsigned int x, unsigned int n); // x^(1/n)
int ff_vorbis_len2vlc(uint8_t *bits, uint32_t *codes, unsigned num);
void ff_vorbis_floor1_render_list(vorbis_floor1_entry * list, int values,
                                  uint16_t *y_list, int *flag,
                                  int multiplier, float * out, int samples);

#define ilog(i) av_log2(2*(i))

#endif /* AVCODEC_VORBIS_H */
