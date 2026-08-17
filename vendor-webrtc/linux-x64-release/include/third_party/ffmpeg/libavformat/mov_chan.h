/*
 * Copyright (c) 2011 Justin Ruggles
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

/**
 * mov 'chan' tag reading/writing.
 * @author Justin Ruggles
 */

#ifndef AVFORMAT_MOV_CHAN_H
#define AVFORMAT_MOV_CHAN_H

#include <stdint.h>

#include "libavutil/channel_layout.h"
#include "libavcodec/codec_id.h"
#include "libavcodec/codec_par.h"
#include "avformat.h"

/**
 * Channel Layout Tag
 * This tells which channels are present in the audio stream and the order in
 * which they appear.
 *
 * @note We're using the channel layout tag to indicate channel order
 *       when the value is greater than 0x10000. The Apple documentation has
 *       some contradictions as to how this is actually supposed to be handled.
 *
 *       Core Audio File Format Spec:
 *           "The high 16 bits indicates a specific ordering of the channels."
 *       Core Audio Data Types Reference:
 *           "These identifiers specify the channels included in a layout but
 *            do not specify a particular ordering of those channels."
 */
enum MovChannelLayoutTag {
#define MOV_CH_LAYOUT_UNKNOWN             0xFFFF0000
    MOV_CH_LAYOUT_USE_DESCRIPTIONS      = (  0 << 16) | 0,
    MOV_CH_LAYOUT_USE_BITMAP            = (  1 << 16) | 0,
    MOV_CH_LAYOUT_DISCRETEINORDER       = (147 << 16) | 0,
    MOV_CH_LAYOUT_MONO                  = (100 << 16) | 1,
    MOV_CH_LAYOUT_STEREO                = (101 << 16) | 2,
    MOV_CH_LAYOUT_STEREOHEADPHONES      = (102 << 16) | 2,
    MOV_CH_LAYOUT_MATRIXSTEREO          = (103 << 16) | 2,
    MOV_CH_LAYOUT_MIDSIDE               = (104 << 16) | 2,
    MOV_CH_LAYOUT_XY                    = (105 << 16) | 2,
    MOV_CH_LAYOUT_BINAURAL              = (106 << 16) | 2,
    MOV_CH_LAYOUT_AMBISONIC_B_FORMAT    = (107 << 16) | 4,
    MOV_CH_LAYOUT_QUADRAPHONIC          = (108 << 16) | 4,
    MOV_CH_LAYOUT_PENTAGONAL            = (109 << 16) | 5,
    MOV_CH_LAYOUT_HEXAGONAL             = (110 << 16) | 6,
    MOV_CH_LAYOUT_OCTAGONAL             = (111 << 16) | 8,
    MOV_CH_LAYOUT_CUBE                  = (112 << 16) | 8,
    MOV_CH_LAYOUT_MPEG_3_0_A            = (113 << 16) | 3,
    MOV_CH_LAYOUT_MPEG_3_0_B            = (114 << 16) | 3,
    MOV_CH_LAYOUT_MPEG_4_0_A            = (115 << 16) | 4,
    MOV_CH_LAYOUT_MPEG_4_0_B            = (116 << 16) | 4,
    MOV_CH_LAYOUT_MPEG_5_0_A            = (117 << 16) | 5,
    MOV_CH_LAYOUT_MPEG_5_0_B            = (118 << 16) | 5,
    MOV_CH_LAYOUT_MPEG_5_0_C            = (119 << 16) | 5,
    MOV_CH_LAYOUT_MPEG_5_0_D            = (120 << 16) | 5,
    MOV_CH_LAYOUT_MPEG_5_1_A            = (121 << 16) | 6,
    MOV_CH_LAYOUT_MPEG_5_1_B            = (122 << 16) | 6,
    MOV_CH_LAYOUT_MPEG_5_1_C            = (123 << 16) | 6,
    MOV_CH_LAYOUT_MPEG_5_1_D            = (124 << 16) | 6,
    MOV_CH_LAYOUT_MPEG_6_1_A            = (125 << 16) | 7,
    MOV_CH_LAYOUT_MPEG_7_1_A            = (126 << 16) | 8,
    MOV_CH_LAYOUT_MPEG_7_1_B            = (127 << 16) | 8,
    MOV_CH_LAYOUT_MPEG_7_1_C            = (128 << 16) | 8,
    MOV_CH_LAYOUT_EMAGIC_DEFAULT_7_1    = (129 << 16) | 8,
    MOV_CH_LAYOUT_SMPTE_DTV             = (130 << 16) | 8,
    MOV_CH_LAYOUT_ITU_2_1               = (131 << 16) | 3,
    MOV_CH_LAYOUT_ITU_2_2               = (132 << 16) | 4,
    MOV_CH_LAYOUT_DVD_4                 = (133 << 16) | 3,
    MOV_CH_LAYOUT_DVD_5                 = (134 << 16) | 4,
    MOV_CH_LAYOUT_DVD_6                 = (135 << 16) | 5,
    MOV_CH_LAYOUT_DVD_10                = (136 << 16) | 4,
    MOV_CH_LAYOUT_DVD_11                = (137 << 16) | 5,
    MOV_CH_LAYOUT_DVD_18                = (138 << 16) | 5,
    MOV_CH_LAYOUT_AUDIOUNIT_6_0         = (139 << 16) | 6,
    MOV_CH_LAYOUT_AUDIOUNIT_7_0         = (140 << 16) | 7,
    MOV_CH_LAYOUT_AUDIOUNIT_7_0_FRONT   = (148 << 16) | 7,
    MOV_CH_LAYOUT_AAC_6_0               = (141 << 16) | 6,
    MOV_CH_LAYOUT_AAC_6_1               = (142 << 16) | 7,
    MOV_CH_LAYOUT_AAC_7_0               = (143 << 16) | 7,
    MOV_CH_LAYOUT_AAC_OCTAGONAL         = (144 << 16) | 8,
    MOV_CH_LAYOUT_TMH_10_2_STD          = (145 << 16) | 16,
    MOV_CH_LAYOUT_TMH_10_2_FULL         = (146 << 16) | 21,
    MOV_CH_LAYOUT_AC3_1_0_1             = (149 << 16) | 2,
    MOV_CH_LAYOUT_AC3_3_0               = (150 << 16) | 3,
    MOV_CH_LAYOUT_AC3_3_1               = (151 << 16) | 4,
    MOV_CH_LAYOUT_AC3_3_0_1             = (152 << 16) | 4,
    MOV_CH_LAYOUT_AC3_2_1_1             = (153 << 16) | 4,
    MOV_CH_LAYOUT_AC3_3_1_1             = (154 << 16) | 5,
    MOV_CH_LAYOUT_EAC3_6_0_A            = (155 << 16) | 6,
    MOV_CH_LAYOUT_EAC3_7_0_A            = (156 << 16) | 7,
    MOV_CH_LAYOUT_EAC3_6_1_A            = (157 << 16) | 7,
    MOV_CH_LAYOUT_EAC3_6_1_B            = (158 << 16) | 7,
    MOV_CH_LAYOUT_EAC3_6_1_C            = (159 << 16) | 7,
    MOV_CH_LAYOUT_EAC3_7_1_A            = (160 << 16) | 8,
    MOV_CH_LAYOUT_EAC3_7_1_B            = (161 << 16) | 8,
    MOV_CH_LAYOUT_EAC3_7_1_C            = (162 << 16) | 8,
    MOV_CH_LAYOUT_EAC3_7_1_D            = (163 << 16) | 8,
    MOV_CH_LAYOUT_EAC3_7_1_E            = (164 << 16) | 8,
    MOV_CH_LAYOUT_EAC3_7_1_F            = (165 << 16) | 8,
    MOV_CH_LAYOUT_EAC3_7_1_G            = (166 << 16) | 8,
    MOV_CH_LAYOUT_EAC3_7_1_H            = (167 << 16) | 8,
    MOV_CH_LAYOUT_DTS_3_1               = (168 << 16) | 4,
    MOV_CH_LAYOUT_DTS_4_1               = (169 << 16) | 5,
    MOV_CH_LAYOUT_DTS_6_0_A             = (170 << 16) | 6,
    MOV_CH_LAYOUT_DTS_6_0_B             = (171 << 16) | 6,
    MOV_CH_LAYOUT_DTS_6_0_C             = (172 << 16) | 6,
    MOV_CH_LAYOUT_DTS_6_1_A             = (173 << 16) | 7,
    MOV_CH_LAYOUT_DTS_6_1_B             = (174 << 16) | 7,
    MOV_CH_LAYOUT_DTS_6_1_C             = (175 << 16) | 7,
    MOV_CH_LAYOUT_DTS_6_1_D             = (182 << 16) | 7,
    MOV_CH_LAYOUT_DTS_7_0               = (176 << 16) | 7,
    MOV_CH_LAYOUT_DTS_7_1               = (177 << 16) | 8,
    MOV_CH_LAYOUT_DTS_8_0_A             = (178 << 16) | 8,
    MOV_CH_LAYOUT_DTS_8_0_B             = (179 << 16) | 8,
    MOV_CH_LAYOUT_DTS_8_1_A             = (180 << 16) | 9,
    MOV_CH_LAYOUT_DTS_8_1_B             = (181 << 16) | 9,
};

/**
 * Get the channel layout tag for the specified codec id and channel layout.
 * If the layout tag was not found, use a channel bitmap if possible.
 *
 * @param[in]  codec_id        codec id
 * @param[in]  channel_layout  channel layout
 * @param[out] bitmap          channel bitmap
 * @return                     channel layout tag
 */
int ff_mov_get_channel_layout_tag(const AVCodecParameters *par,
                                       uint32_t *layout,
                                       uint32_t *bitmap,
                                       uint32_t **pchannel_desc);

/**
 * Read 'chan' tag from the input stream.
 *
 * @param s     AVFormatContext
 * @param pb    AVIOContext
 * @param st    The stream to set codec values for
 * @param size  Remaining size in the 'chan' tag
 * @return      0 if ok, or negative AVERROR code on failure
 */
int ff_mov_read_chan(AVFormatContext *s, AVIOContext *pb, AVStream *st,
                     int64_t size);

/**
 * Get ISO/IEC 23001-8 ChannelConfiguration from AVChannelLayout.
 *
 */
int ff_mov_get_channel_config_from_layout(const AVChannelLayout *layout, int *config);

/**
 * Get AVChannelLayout from ISO/IEC 23001-8 ChannelConfiguration.
 *
 * @return 1  if the config was unknown, layout is untouched in this case
 *         0  if the config was found
 *         <0 on error
 */
int ff_mov_get_channel_layout_from_config(int config, AVChannelLayout *layout, uint64_t omitted_channel_map);

/**
 * Get ISO/IEC 23001-8 OutputChannelPosition from AVChannelLayout.
 */
int ff_mov_get_channel_positions_from_layout(const AVChannelLayout *layout,
                                             uint8_t *position, int position_num);

/**
 * Read 'chnl' tag from the input stream.
 *
 * @param s     AVFormatContext
 * @param pb    AVIOContext
 * @param st    The stream to set codec values for
 * @return      0 if ok, or negative AVERROR code on failure
 */
int ff_mov_read_chnl(AVFormatContext *s, AVIOContext *pb, AVStream *st);

#endif /* AVFORMAT_MOV_CHAN_H */
