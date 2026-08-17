/*
 * Realmedia RTSP (RDT) definitions
 * Copyright (c) 2007 Ronald S. Bultje <rbultje@ronald.bitfreak.net>
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

#ifndef AVFORMAT_RDT_H
#define AVFORMAT_RDT_H

#include <stdint.h>
#include "avformat.h"
#include "rtpdec.h"

typedef struct RDTDemuxContext RDTDemuxContext;

/**
 * Allocate and init the RDT parsing context.
 * @param ic the containing RTSP demuxer context
 * @param first_stream_of_set_idx index to the first AVStream in the RTSP
 *              demuxer context's ic->streams array that is part of this
 *              particular stream's set of streams (with identical content)
 * @param priv_data private data of the payload data handler context
 * @param handler pointer to the parse_packet() payload parsing function
 * @return a newly allocated RDTDemuxContext. Free with ff_rdt_parse_close().
 */
RDTDemuxContext *ff_rdt_parse_open(AVFormatContext *ic,
                                   int first_stream_of_set_idx,
                                   void *priv_data,
                                   const RTPDynamicProtocolHandler *handler);
void ff_rdt_parse_close(RDTDemuxContext *s);

/**
 * Calculate the response (RealChallenge2 in the RTSP header) to the
 * challenge (RealChallenge1 in the RTSP header from the Real/Helix
 * server), which is used as some sort of client validation.
 *
 * @param response pointer to response buffer, it should be at least 41 bytes
 *                 (40 data + 1 zero) bytes long.
 * @param chksum pointer to buffer containing a checksum of the response,
 *               it should be at least 9 (8 data + 1 zero) bytes long.
 * @param challenge pointer to the RealChallenge1 value provided by the
 *                  server.
 */
void ff_rdt_calc_response_and_checksum(char response[41], char chksum[9],
                                       const char *challenge);

/**
 * Add subscription information to Subscribe parameter string.
 *
 * @param cmd string to write the subscription information into.
 * @param size size of cmd.
 * @param stream_nr stream number.
 * @param rule_nr rule number to conform to.
 */
void ff_rdt_subscribe_rule(char *cmd, int size,
                           int stream_nr, int rule_nr);

/**
 * Parse RDT-style packet header.
 *
 * @param buf input buffer
 * @param len length of input buffer
 * @param pset_id will be set to the set ID this packet belongs to
 * @param pseq_no will be set to the sequence number of the packet
 * @param pstream_id will be set to the stream ID this packet belongs to
 * @param pis_keyframe will be whether this packet belongs to a keyframe
 * @param ptimestamp will be set to the timestamp of the packet
 * @return the amount of bytes consumed, or negative on error
 */
int ff_rdt_parse_header(const uint8_t *buf, int len,
                        int *pset_id, int *pseq_no, int *pstream_id,
                        int *pis_keyframe, uint32_t *ptimestamp);

/**
 * Parse RDT-style packet data (header + media data).
 * Usage similar to rtp_parse_packet().
 */
int ff_rdt_parse_packet(RDTDemuxContext *s, AVPacket *pkt,
                        uint8_t **buf, int len);

/**
 * Parse a server-related SDP line.
 *
 * @param s the RTSP AVFormatContext
 * @param stream_index the index of the first stream in the set represented
 *               by the SDP m= line (in s->streams)
 * @param buf the SDP line
 */
void ff_real_parse_sdp_a_line(AVFormatContext *s, int stream_index,
                              const char *buf);

#endif /* AVFORMAT_RDT_H */
