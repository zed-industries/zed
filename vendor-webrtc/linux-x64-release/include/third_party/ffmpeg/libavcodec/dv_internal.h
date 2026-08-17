/*
 * DV encoder/decoder shared code
 * Copyright (c) 2002 Fabrice Bellard
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

#ifndef AVCODEC_DV_INTERNAL_H
#define AVCODEC_DV_INTERNAL_H

#include <stdint.h>

#include "dv.h"
#include "dv_profile.h"

typedef struct DVwork_chunk {
    uint16_t buf_offset;
    uint16_t mb_coordinates[5];
} DVwork_chunk;

void ff_dv_init_dynamic_tables(DVwork_chunk *work_chunks, const AVDVProfile *d);

static inline int dv_work_pool_size(const AVDVProfile *d)
{
    int size = d->n_difchan * d->difseg_size * 27;
    if (DV_PROFILE_IS_1080i50(d))
        size -= 3 * 27;
    if (DV_PROFILE_IS_720p50(d))
        size -= 4 * 27;
    return size;
}

static inline void dv_calculate_mb_xy(const AVDVProfile *sys,
                                      const uint8_t *buf,
                                      const DVwork_chunk *work_chunk,
                                      int m, int *mb_x, int *mb_y)
{
    *mb_x = work_chunk->mb_coordinates[m] & 0xff;
    *mb_y = work_chunk->mb_coordinates[m] >> 8;

    /* We work with 720p frames split in half.
     * The odd half-frame (chan == 2,3) is displaced :-( */
    if (sys->height == 720 && !(buf[1] & 0x0C))
        /* shifting the Y coordinate down by 72/2 macro blocks */
        *mb_y -= (*mb_y > 17) ? 18 : -72;
}

#endif // AVCODEC_DV_INTERNAL_H
