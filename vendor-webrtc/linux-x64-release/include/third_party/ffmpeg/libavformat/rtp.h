/*
 * RTP definitions
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
#ifndef AVFORMAT_RTP_H
#define AVFORMAT_RTP_H

#include <stdint.h>
#include "libavutil/avutil.h"
#include "libavcodec/codec_id.h"
#include "libavcodec/codec_par.h"
#include "libavformat/avformat.h"

/**
 * Return the payload type for a given stream used in the given format context.
 * Static payload types are derived from the codec.
 * Dynamic payload type are derived from the id field in AVStream.
 * The format context private option payload_type overrides both.
 *
 * @param fmt   The context of the format
 * @param par   The codec parameters
 * @param idx   The stream index
 * @return The payload type (the 'PT' field in the RTP header).
 */
int ff_rtp_get_payload_type(const AVFormatContext *fmt,
                            const AVCodecParameters *par, int idx);

/**
 * Initialize a codec context based on the payload type.
 *
 * Fill the codec_type and codec_id fields of a codec context with
 * information depending on the payload type; for audio codecs, the
 * channels and sample_rate fields are also filled.
 *
 * @param par The codec parameters
 * @param payload_type The payload type (the 'PT' field in the RTP header)
 * @return In case of unknown payload type or dynamic payload type, a
 * negative value is returned; otherwise, 0 is returned
 */
int ff_rtp_get_codec_info(AVCodecParameters *par, int payload_type);

/**
 * Return the encoding name (as defined in
 * http://www.iana.org/assignments/rtp-parameters) for a given payload type.
 *
 * @param payload_type The payload type (the 'PT' field in the RTP header)
 * @return In case of unknown payload type or dynamic payload type, a pointer
 * to an empty string is returned; otherwise, a pointer to a string containing
 * the encoding name is returned
 */
const char *ff_rtp_enc_name(int payload_type);

/**
 * Return the codec id for the given encoding name and codec type.
 *
 * @param buf A pointer to the string containing the encoding name
 * @param codec_type The codec type
 * @return In case of unknown encoding name, AV_CODEC_ID_NONE is returned;
 * otherwise, the codec id is returned
 */
enum AVCodecID ff_rtp_codec_id(const char *buf, enum AVMediaType codec_type);

#define RTP_PT_PRIVATE 96
#define RTP_VERSION 2
#define RTP_MAX_SDES 256   /**< maximum text length for SDES */

/* RTCP packets use 0.5% of the bandwidth */
#define RTCP_TX_RATIO_NUM 5
#define RTCP_TX_RATIO_DEN 1000

/* An arbitrary id value for RTP Xiph streams - only relevant to indicate
 * that the configuration has changed within a stream (by changing the
 * ident value sent).
 */
#define RTP_XIPH_IDENT 0xfecdba

/* RTCP packet types */
enum RTCPType {
    RTCP_FIR    = 192,
    RTCP_NACK, // 193
    RTCP_SMPTETC,// 194
    RTCP_IJ,   // 195
    RTCP_SR     = 200,
    RTCP_RR,   // 201
    RTCP_SDES, // 202
    RTCP_BYE,  // 203
    RTCP_APP,  // 204
    RTCP_RTPFB,// 205
    RTCP_PSFB, // 206
    RTCP_XR,   // 207
    RTCP_AVB,  // 208
    RTCP_RSI,  // 209
    RTCP_TOKEN,// 210
};

#define RTP_PT_IS_RTCP(x) (((x) >= RTCP_FIR && (x) <= RTCP_IJ) || \
                           ((x) >= RTCP_SR  && (x) <= RTCP_TOKEN))

#define NTP_TO_RTP_FORMAT(x) av_rescale((x), INT64_C(1) << 32, 1000000)

#endif /* AVFORMAT_RTP_H */
