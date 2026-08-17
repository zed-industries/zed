/*
 * copyright (c) 2001 Fabrice Bellard
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

#ifndef AVFORMAT_RAWUTILS_H
#define AVFORMAT_RAWUTILS_H

#include <stdint.h>
#include "libavcodec/codec_par.h"
#include "libavcodec/packet.h"
#include "avformat.h"

#define CONTAINS_PAL 2
/**
 * Reshuffles the lines to use the user specified stride.
 *
 * @param ppkt input and output packet
 * @return negative error code or
 *         0 if no new packet was allocated
 *         non-zero if a new packet was allocated and ppkt has to be freed
 *         CONTAINS_PAL if in addition to a new packet the old contained a palette
 */
int ff_reshuffle_raw_rgb(AVFormatContext *s, AVPacket **ppkt, AVCodecParameters *par, int expected_stride);

/**
 * Retrieves the palette from a packet, either from side data, or
 * appended to the video data in the packet itself (raw video only).
 * It is commonly used after a call to ff_reshuffle_raw_rgb().
 *
 * Use 0 for the ret parameter to check for side data only.
 *
 * @param pkt pointer to packet before calling ff_reshuffle_raw_rgb()
 * @param ret return value from ff_reshuffle_raw_rgb(), or 0
 * @param palette pointer to palette buffer
 * @return negative error code or
 *         1 if the packet has a palette, else 0
 */
int ff_get_packet_palette(AVFormatContext *s, AVPacket *pkt, int ret, uint32_t *palette);

#endif /* AVFORMAT_RAWUTILS_H */
