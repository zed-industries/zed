/*
 * TAK decoder/demuxer common code
 * Copyright (c) 2012 Paul B Mahol
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
 * @file
 * TAK (Tom's lossless Audio Kompressor) decoder/demuxer common functions
 */

#ifndef AVCODEC_TAK_H
#define AVCODEC_TAK_H

#include <stdint.h>

#include "get_bits.h"

#define TAK_FORMAT_DATA_TYPE_BITS               3
#define TAK_FORMAT_SAMPLE_RATE_BITS            18
#define TAK_FORMAT_BPS_BITS                     5
#define TAK_FORMAT_CHANNEL_BITS                 4
#define TAK_FORMAT_VALID_BITS                   5
#define TAK_FORMAT_CH_LAYOUT_BITS               6
#define TAK_SIZE_FRAME_DURATION_BITS            4
#define TAK_SIZE_SAMPLES_NUM_BITS              35
#define TAK_LAST_FRAME_POS_BITS                40
#define TAK_LAST_FRAME_SIZE_BITS               24
#define TAK_ENCODER_CODEC_BITS                  6
#define TAK_ENCODER_PROFILE_BITS                4
#define TAK_SAMPLE_RATE_MIN                  6000
#define TAK_CHANNELS_MIN                        1
#define TAK_BPS_MIN                             8
#define TAK_FRAME_HEADER_FLAGS_BITS             3
#define TAK_FRAME_HEADER_SYNC_ID           0xA0FF
#define TAK_FRAME_HEADER_SYNC_ID_BITS          16
#define TAK_FRAME_HEADER_SAMPLE_COUNT_BITS     14
#define TAK_FRAME_HEADER_NO_BITS               21
#define TAK_FRAME_DURATION_QUANT_SHIFT          5
#define TAK_CRC24_BITS                         24


#define TAK_FRAME_FLAG_IS_LAST                0x1
#define TAK_FRAME_FLAG_HAS_INFO               0x2
#define TAK_FRAME_FLAG_HAS_METADATA           0x4

#define TAK_MAX_CHANNELS               (1 << TAK_FORMAT_CHANNEL_BITS)

#define TAK_MIN_FRAME_HEADER_BITS      (TAK_FRAME_HEADER_SYNC_ID_BITS + \
                                        TAK_FRAME_HEADER_FLAGS_BITS   + \
                                        TAK_FRAME_HEADER_NO_BITS      + \
                                        TAK_CRC24_BITS)

#define TAK_MIN_FRAME_HEADER_LAST_BITS (TAK_MIN_FRAME_HEADER_BITS + 2 + \
                                        TAK_FRAME_HEADER_SAMPLE_COUNT_BITS)

#define TAK_ENCODER_BITS               (TAK_ENCODER_CODEC_BITS + \
                                        TAK_ENCODER_PROFILE_BITS)

#define TAK_SIZE_BITS                  (TAK_SIZE_SAMPLES_NUM_BITS + \
                                        TAK_SIZE_FRAME_DURATION_BITS)

#define TAK_FORMAT_BITS                (TAK_FORMAT_DATA_TYPE_BITS   + \
                                        TAK_FORMAT_SAMPLE_RATE_BITS + \
                                        TAK_FORMAT_BPS_BITS         + \
                                        TAK_FORMAT_CHANNEL_BITS + 1 + \
                                        TAK_FORMAT_VALID_BITS   + 1 + \
                                        TAK_FORMAT_CH_LAYOUT_BITS   * \
                                        TAK_MAX_CHANNELS)

#define TAK_STREAMINFO_BITS            (TAK_ENCODER_BITS + \
                                        TAK_SIZE_BITS    + \
                                        TAK_FORMAT_BITS)

#define TAK_MAX_FRAME_HEADER_BITS      (TAK_MIN_FRAME_HEADER_LAST_BITS + \
                                        TAK_STREAMINFO_BITS + 31)

#define TAK_STREAMINFO_BYTES           ((TAK_STREAMINFO_BITS       + 7) / 8)
#define TAK_MAX_FRAME_HEADER_BYTES     ((TAK_MAX_FRAME_HEADER_BITS + 7) / 8)
#define TAK_MIN_FRAME_HEADER_BYTES     ((TAK_MIN_FRAME_HEADER_BITS + 7) / 8)

enum TAKCodecType {
    TAK_CODEC_MONO_STEREO  = 2,
    TAK_CODEC_MULTICHANNEL = 4,
};

enum TAKMetaDataType {
    TAK_METADATA_END = 0,
    TAK_METADATA_STREAMINFO,
    TAK_METADATA_SEEKTABLE,
    TAK_METADATA_SIMPLE_WAVE_DATA,
    TAK_METADATA_ENCODER,
    TAK_METADATA_PADDING,
    TAK_METADATA_MD5,
    TAK_METADATA_LAST_FRAME,
};

enum TAKFrameSizeType {
    TAK_FST_94ms = 0,
    TAK_FST_125ms,
    TAK_FST_188ms,
    TAK_FST_250ms,
    TAK_FST_4096,
    TAK_FST_8192,
    TAK_FST_16384,
    TAK_FST_512,
    TAK_FST_1024,
    TAK_FST_2048,
};

typedef struct TAKStreamInfo {
    int               flags;
    enum TAKCodecType codec;
    int               data_type;
    int               sample_rate;
    int               channels;
    int               bps;
    int               frame_num;
    int               frame_samples;
    int               last_frame_samples;
    uint64_t          ch_layout;
    int64_t           samples;
} TAKStreamInfo;

int ff_tak_check_crc(const uint8_t *buf, unsigned int buf_size);

/**
 * Parse the Streaminfo metadata block.
 * @param[out] s  storage for parsed information
 * @param[in]  buf   input buffer
 * @param[in]  size  size of input buffer in bytes
 * @return non-zero on error, 0 if OK
 */
int avpriv_tak_parse_streaminfo(TAKStreamInfo *s, const uint8_t *buf, int size);

/**
 * Validate and decode a frame header.
 * @param      logctx            for use as av_log() context
 * @param[in]  gb                GetBitContext from which to read frame header
 * @param[out] s                 frame information
 * @param      log_level_offset  log level offset, can be used to silence
 *                               error messages.
 * @return non-zero on error, 0 if OK
 */
int ff_tak_decode_frame_header(void *logctx, GetBitContext *gb,
                               TAKStreamInfo *s, int log_level_offset);
#endif /* AVCODEC_TAK_H */
