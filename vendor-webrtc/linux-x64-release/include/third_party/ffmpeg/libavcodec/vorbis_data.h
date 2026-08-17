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

#ifndef AVCODEC_VORBIS_DATA_H
#define AVCODEC_VORBIS_DATA_H

#include <stdint.h>

#include "libavutil/attributes_internal.h"
#include "libavutil/channel_layout.h"

FF_VISIBILITY_PUSH_HIDDEN
extern const float ff_vorbis_floor1_inverse_db_table[256];
extern const float * const ff_vorbis_vwin[8];
extern const uint8_t ff_vorbis_channel_layout_offsets[8][8];
extern const AVChannelLayout ff_vorbis_ch_layouts[9];
FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_VORBIS_DATA_H */
