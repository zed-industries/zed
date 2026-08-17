/*
 * FLAC (Free Lossless Audio Codec) decoder/parser common functions
 * Copyright (c) 2008 Justin Ruggles
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
 * FLAC (Free Lossless Audio Codec) decoder/parser common functions
 */

#ifndef AVCODEC_FLAC_PARSE_H
#define AVCODEC_FLAC_PARSE_H

#include "avcodec.h"
#include "get_bits.h"

typedef struct FLACStreaminfo {
    int samplerate;         /**< sample rate                             */
    int channels;           /**< number of channels                      */
    int bps;                /**< bits-per-sample                         */
    int max_blocksize;      /**< maximum block size, in samples          */
    int max_framesize;      /**< maximum frame size, in bytes            */
    int64_t samples;        /**< total number of samples                 */
} FLACStreaminfo;

typedef struct FLACFrameInfo {
    int samplerate;         /**< sample rate                             */
    int channels;           /**< number of channels                      */
    int bps;                /**< bits-per-sample                         */
    int blocksize;          /**< block size of the frame                 */
    int ch_mode;            /**< channel decorrelation mode              */
    int64_t frame_or_sample_num;    /**< frame number or sample number   */
    int is_var_size;                /**< specifies if the stream uses variable
                                         block sizes or a fixed block size;
                                         also determines the meaning of
                                         frame_or_sample_num             */
} FLACFrameInfo;

/**
 * Parse the Streaminfo metadata block
 * @param[out] avctx   codec context to set basic stream parameters
 * @param[out] s       where parsed information is stored
 * @param[in]  buffer  pointer to start of 34-byte streaminfo data
 *
 * @return negative error code on faiure or >= 0 on success
 */
int ff_flac_parse_streaminfo(AVCodecContext *avctx, struct FLACStreaminfo *s,
                              const uint8_t *buffer);

/**
 * Validate the FLAC extradata.
 * @param[in]  avctx codec context containing the extradata.
 * @param[out] format extradata format.
 * @param[out] streaminfo_start pointer to start of 34-byte STREAMINFO data.
 * @return 1 if valid, 0 if not valid.
 */
int ff_flac_is_extradata_valid(AVCodecContext *avctx,
                               uint8_t **streaminfo_start);

/**
 * Validate and decode a frame header.
 * @param      logctx context for logging
 * @param      gb    GetBitContext from which to read frame header
 * @param[out] fi    frame information
 * @param      log_level_offset  log level offset. can be used to silence error messages.
 * @return non-zero on error, 0 if ok
 */
int ff_flac_decode_frame_header(void *logctx, GetBitContext *gb,
                                FLACFrameInfo *fi, int log_level_offset);

void ff_flac_set_channel_layout(AVCodecContext *avctx, int channels);

#endif /* AVCODEC_FLAC_PARSE_H */
