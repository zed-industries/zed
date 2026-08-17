/*
 * RTP depacketizer declarations
 * Copyright (c) 2010 Martin Storsjo
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

#ifndef AVFORMAT_RTPDEC_FORMATS_H
#define AVFORMAT_RTPDEC_FORMATS_H

#include "rtpdec.h"

/**
 * Parse a Windows Media Server-specific SDP line
 *
 * @param s RTSP demux context
 */
int ff_wms_parse_sdp_a_line(AVFormatContext *s, const char *p);

int ff_h263_handle_packet(AVFormatContext *ctx, PayloadContext *data,
                          AVStream *st, AVPacket *pkt, uint32_t *timestamp,
                          const uint8_t *buf, int len, uint16_t seq, int flags);

int ff_h264_parse_sprop_parameter_sets(AVFormatContext *s,
                                       uint8_t **data_ptr, int *size_ptr,
                                       const char *value);
int ff_h264_handle_aggregated_packet(AVFormatContext *ctx, PayloadContext *data, AVPacket *pkt,
                                     const uint8_t *buf, int len,
                                     int start_skip, int *nal_counters,
                                     int nal_mask);
int ff_h264_handle_frag_packet(AVPacket *pkt, const uint8_t *buf, int len,
                               int start_bit, const uint8_t *nal_header,
                               int nal_header_len);
void ff_h264_parse_framesize(AVCodecParameters *par, const char *p);

extern const RTPDynamicProtocolHandler ff_ac3_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_amr_nb_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_amr_wb_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_av1_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_dv_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_g726_16_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_g726_24_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_g726_32_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_g726_40_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_g726le_16_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_g726le_24_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_g726le_32_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_g726le_40_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_h261_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_h263_1998_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_h263_2000_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_h263_rfc2190_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_h264_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_hevc_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_ilbc_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_jpeg_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_mp4a_latm_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_mp4v_es_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_mpeg_audio_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_mpeg_audio_robust_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_mpeg_video_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_mpeg4_generic_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_mpegts_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_ms_rtp_asf_pfa_handler;
extern const RTPDynamicProtocolHandler ff_ms_rtp_asf_pfv_handler;
extern const RTPDynamicProtocolHandler ff_qcelp_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_qdm2_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_qt_rtp_aud_handler;
extern const RTPDynamicProtocolHandler ff_qt_rtp_vid_handler;
extern const RTPDynamicProtocolHandler ff_quicktime_rtp_aud_handler;
extern const RTPDynamicProtocolHandler ff_quicktime_rtp_vid_handler;
extern const RTPDynamicProtocolHandler ff_rfc4175_rtp_handler;
extern const RTPDynamicProtocolHandler ff_svq3_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_theora_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_vc2hq_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_vorbis_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_vp8_dynamic_handler;
extern const RTPDynamicProtocolHandler ff_vp9_dynamic_handler;

#endif /* AVFORMAT_RTPDEC_FORMATS_H */
