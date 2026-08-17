/*
 * Copyright (c) 2007 Ian Caulfield
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

#ifndef AVCODEC_MLP_PARSE_H
#define AVCODEC_MLP_PARSE_H

#include <stdint.h>

#include "libavutil/channel_layout.h"

#include "get_bits.h"

typedef struct MLPHeaderInfo
{
    int stream_type;                        ///< 0xBB for MLP, 0xBA for TrueHD
    int header_size;                        ///< Size of the major sync header, in bytes

    int group1_bits;                        ///< The bit depth of the first substream
    int group2_bits;                        ///< Bit depth of the second substream (MLP only)

    int group1_samplerate;                  ///< Sample rate of first substream
    int group2_samplerate;                  ///< Sample rate of second substream (MLP only)

    int channel_arrangement;

    int channel_modifier_thd_stream0;       ///< Channel modifier for substream 0 of TrueHD streams ("2-channel presentation")
    int channel_modifier_thd_stream1;       ///< Channel modifier for substream 1 of TrueHD streams ("6-channel presentation")
    int channel_modifier_thd_stream2;       ///< Channel modifier for substream 2 of TrueHD streams ("8-channel presentation")

    int channels_mlp;                       ///< Channel count for MLP streams
    int channels_thd_stream1;               ///< Channel count for substream 1 of TrueHD streams ("6-channel presentation")
    int channels_thd_stream2;               ///< Channel count for substream 2 of TrueHD streams ("8-channel presentation")
    uint64_t channel_layout_mlp;            ///< Channel layout for MLP streams
    uint64_t channel_layout_thd_stream1;    ///< Channel layout for substream 1 of TrueHD streams ("6-channel presentation")
    uint64_t channel_layout_thd_stream2;    ///< Channel layout for substream 2 of TrueHD streams ("8-channel presentation")

    int access_unit_size;                   ///< Number of samples per coded frame
    int access_unit_size_pow2;              ///< Next power of two above number of samples per frame

    int is_vbr;                             ///< Stream is VBR instead of CBR
    int peak_bitrate;                       ///< Peak bitrate for VBR, actual bitrate (==peak) for CBR

    int num_substreams;                     ///< Number of substreams within stream

    int extended_substream_info;            ///< Which substream of substreams carry 16-channel presentation
    int substream_info;                     ///< Which substream of substreams carry 2/6/8-channel presentation
} MLPHeaderInfo;

static const uint8_t thd_chancount[13] = {
//  LR    C   LFE  LRs LRvh  LRc LRrs  Cs   Ts  LRsd  LRw  Cvh  LFE2
     2,   1,   1,   2,   2,   2,   2,   1,   1,   2,   2,   1,   1
};

static const uint64_t thd_layout[13] = {
    AV_CH_FRONT_LEFT|AV_CH_FRONT_RIGHT,                     // LR
    AV_CH_FRONT_CENTER,                                     // C
    AV_CH_LOW_FREQUENCY,                                    // LFE
    AV_CH_SIDE_LEFT|AV_CH_SIDE_RIGHT,                       // LRs
    AV_CH_TOP_FRONT_LEFT|AV_CH_TOP_FRONT_RIGHT,             // LRvh
    AV_CH_FRONT_LEFT_OF_CENTER|AV_CH_FRONT_RIGHT_OF_CENTER, // LRc
    AV_CH_BACK_LEFT|AV_CH_BACK_RIGHT,                       // LRrs
    AV_CH_BACK_CENTER,                                      // Cs
    AV_CH_TOP_CENTER,                                       // Ts
    AV_CH_SURROUND_DIRECT_LEFT|AV_CH_SURROUND_DIRECT_RIGHT, // LRsd
    AV_CH_WIDE_LEFT|AV_CH_WIDE_RIGHT,                       // LRw
    AV_CH_TOP_FRONT_CENTER,                                 // Cvh
    AV_CH_LOW_FREQUENCY_2,                                  // LFE2
};

static inline int mlp_samplerate(int in)
{
    if (in == 0xF)
        return 0;

    return (in & 8 ? 44100 : 48000) << (in & 7) ;
}

static inline int truehd_channels(int chanmap)
{
    int channels = 0, i;

    for (i = 0; i < 13; i++)
        channels += thd_chancount[i] * ((chanmap >> i) & 1);

    return channels;
}

static inline uint64_t truehd_layout(int chanmap)
{
    int i;
    uint64_t layout = 0;

    for (i = 0; i < 13; i++)
        layout |= thd_layout[i] * ((chanmap >> i) & 1);

    return layout;
}

static inline int layout_truehd(uint64_t layout)
{
    int chanmap = 0;

    for (int i = 0; i < 13; i++) {
        if ((layout & thd_layout[i]) == thd_layout[i])
            chanmap |= 1 << i;
    }

    return chanmap;
}

int ff_mlp_read_major_sync(void *log, MLPHeaderInfo *mh, GetBitContext *gb);

#endif /* AVCODEC_MLP_PARSE_H */
