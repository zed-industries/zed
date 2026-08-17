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

#ifndef AVCODEC_CBS_H2645_H
#define AVCODEC_CBS_H2645_H

#include "h2645_parse.h"


typedef struct CodedBitstreamH2645Context {
    // If set, the stream being read is in MP4 (AVCC/HVCC) format.  If not
    // set, the stream is assumed to be in annex B format.
    int mp4;
    // Size in bytes of the NAL length field for MP4 format.
    int nal_length_size;
    // Packet reader.
    H2645Packet read_packet;
} CodedBitstreamH2645Context;


#endif /* AVCODEC_CBS_H2645_H */
