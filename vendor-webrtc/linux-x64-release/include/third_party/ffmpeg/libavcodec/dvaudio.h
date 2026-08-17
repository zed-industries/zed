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

#ifndef AVCODEC_DVAUDIO_H
#define AVCODEC_DVAUDIO_H

#include <stdint.h>

static inline int dv_get_audio_sample_count(const uint8_t *buffer, int dsf)
{
    int samples = buffer[0] & 0x3f; /* samples in this frame - min samples */

    switch ((buffer[3] >> 3) & 0x07) {
    case 0:
        return samples + (dsf ? 1896 : 1580);
    case 1:
        return samples + (dsf ? 1742 : 1452);
    case 2:
    default:
        return samples + (dsf ? 1264 : 1053);
    }
}

#endif /* AVCODEC_DVAUDIO_H */
