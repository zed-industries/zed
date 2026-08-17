/*
 * ALAC encoder and decoder common data
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

#ifndef AVCODEC_ALAC_DATA_H
#define AVCODEC_ALAC_DATA_H

#include <stdint.h>

#include "libavutil/channel_layout.h"

enum AlacRawDataBlockType {
    /* At the moment, only SCE, CPE, LFE, and END are recognized. */
    TYPE_SCE,
    TYPE_CPE,
    TYPE_CCE,
    TYPE_LFE,
    TYPE_DSE,
    TYPE_PCE,
    TYPE_FIL,
    TYPE_END
};

#define ALAC_MAX_CHANNELS 8

extern const uint8_t ff_alac_channel_layout_offsets[ALAC_MAX_CHANNELS][ALAC_MAX_CHANNELS];

extern const AVChannelLayout ff_alac_ch_layouts[ALAC_MAX_CHANNELS + 1];

extern const enum AlacRawDataBlockType ff_alac_channel_elements[ALAC_MAX_CHANNELS][5];

#endif /* AVCODEC_ALAC_DATA_H */
