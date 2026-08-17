/*
 * MPEG-2 transport stream defines
 * Copyright (c) 2003 Fabrice Bellard
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

#ifndef AVFORMAT_MPEGTS_H
#define AVFORMAT_MPEGTS_H

#include "avformat.h"

#define TS_FEC_PACKET_SIZE 204
#define TS_DVHS_PACKET_SIZE 192
#define TS_PACKET_SIZE 188
#define TS_MAX_PACKET_SIZE 204

#define NB_PID_MAX 8192
#define USUAL_SECTION_SIZE 1024 /* except EIT which is limited to 4096 */
#define MAX_SECTION_SIZE 4096

#define SYNC_BYTE 0x47
#define STUFFING_BYTE 0xFF
#define SYSTEM_CLOCK_FREQUENCY_DIVISOR 300 /* convert 27 MHz to 90 kHz */

/* pids */
#define PAT_PID         0x0000 /* Program Association Table */
#define CAT_PID         0x0001 /* Conditional Access Table */
#define TSDT_PID        0x0002 /* Transport Stream Description Table */
#define IPMP_PID        0x0003
/* PID from 0x0004 to 0x000F are reserved */
#define NIT_PID         0x0010 /* Network Information Table */
#define SDT_PID         0x0011 /* Service Description Table */
#define BAT_PID         0x0011 /* Bouquet Association Table */
#define EIT_PID         0x0012 /* Event Information Table */
#define RST_PID         0x0013 /* Running Status Table */
#define TDT_PID         0x0014 /* Time and Date Table */
#define TOT_PID         0x0014
#define NET_SYNC_PID    0x0015
#define RNT_PID         0x0016 /* RAR Notification Table */
/* PID from 0x0017 to 0x001B are reserved for future use */
/* PID value 0x001C allocated to link-local inband signalling shall not be
 * used on any broadcast signals. It shall only be used between devices in a
 * controlled environment. */
#define LINK_LOCAL_PID  0x001C
#define MEASUREMENT_PID 0x001D
#define DIT_PID         0x001E /* Discontinuity Information Table */
#define SIT_PID         0x001F /* Selection Information Table */
/* PID from 0x0020 to 0x1FFA may be assigned as needed to PMT, elementary
 * streams and other data tables */
#define FIRST_OTHER_PID 0x0020
#define  LAST_OTHER_PID 0x1FFA
/* PID 0x1FFB is used by DigiCipher 2/ATSC MGT metadata */
/* PID from 0x1FFC to 0x1FFE may be assigned as needed to PMT, elementary
 * streams and other data tables */
#define NULL_PID        0x1FFF /* Null packet (used for fixed bandwidth padding) */

/* m2ts pids */
#define M2TS_PMT_PID                      0x0100
#define M2TS_PCR_PID                      0x1001
#define M2TS_VIDEO_PID                    0x1011
#define M2TS_AUDIO_START_PID              0x1100
#define M2TS_PGSSUB_START_PID             0x1200
#define M2TS_TEXTSUB_PID                  0x1800
#define M2TS_SECONDARY_AUDIO_START_PID    0x1A00
#define M2TS_SECONDARY_VIDEO_START_PID    0x1B00

/* table ids */
#define PAT_TID         0x00 /* Program Association section */
#define CAT_TID         0x01 /* Conditional Access section */
#define PMT_TID         0x02 /* Program Map section */
#define TSDT_TID        0x03 /* Transport Stream Description section */
/* TID from 0x04 to 0x3F are reserved */
#define M4OD_TID        0x05
#define NIT_TID         0x40 /* Network Information section - actual network */
#define ONIT_TID        0x41 /* Network Information section - other network */
#define SDT_TID         0x42 /* Service Description section - actual TS */
/* TID from 0x43 to 0x45 are reserved for future use */
#define OSDT_TID        0x46 /* Service Descrition section - other TS */
/* TID from 0x47 to 0x49 are reserved for future use */
#define BAT_TID         0x4A /* Bouquet Association section */
#define UNT_TID         0x4B /* Update Notification Table section */
#define DFI_TID         0x4C /* Downloadable Font Info section */
/* TID 0x4D is reserved for future use */
#define EIT_TID         0x4E /* Event Information section - actual TS */
#define OEIT_TID        0x4F /* Event Information section - other TS */
#define EITS_START_TID  0x50 /* Event Information section schedule - actual TS */
#define EITS_END_TID    0x5F /* Event Information section schedule - actual TS */
#define OEITS_START_TID 0x60 /* Event Information section schedule - other TS */
#define OEITS_END_TID   0x6F /* Event Information section schedule - other TS */
#define TDT_TID         0x70 /* Time Date section */
#define RST_TID         0x71 /* Running Status section */
#define ST_TID          0x72 /* Stuffing section */
#define TOT_TID         0x73 /* Time Offset section */
#define AIT_TID         0x74 /* Application Inforamtion section */
#define CT_TID          0x75 /* Container section */
#define RCT_TID         0x76 /* Related Content section */
#define CIT_TID         0x77 /* Content Identifier section */
#define MPE_FEC_TID     0x78 /* MPE-FEC section */
#define RPNT_TID        0x79 /* Resolution Provider Notification section */
#define MPE_IFEC_TID    0x7A /* MPE-IFEC section */
#define PROTMT_TID      0x7B /* Protection Message section */
/* TID from 0x7C to 0x7D are reserved for future use */
#define DIT_TID         0x7E /* Discontinuity Information section */
#define SIT_TID         0x7F /* Selection Information section */
/* TID from 0x80 to 0xFE are user defined */
/* TID 0xFF is reserved */

/* ISO/IEC 13818-1 Table 2-34 - Stream type assignments */
#define STREAM_TYPE_VIDEO_MPEG1     0x01
#define STREAM_TYPE_VIDEO_MPEG2     0x02
#define STREAM_TYPE_AUDIO_MPEG1     0x03
#define STREAM_TYPE_AUDIO_MPEG2     0x04
#define STREAM_TYPE_PRIVATE_SECTION 0x05
#define STREAM_TYPE_PRIVATE_DATA    0x06
#define STREAM_TYPE_AUDIO_AAC       0x0f
#define STREAM_TYPE_AUDIO_AAC_LATM  0x11
#define STREAM_TYPE_VIDEO_MPEG4     0x10
/** ISO/IEC 14496-1 (MPEG-4 Systems) SL-packetized stream or FlexMux stream
    carried in PES packets */
#define STREAM_TYPE_ISO_IEC_14496_PES     0x12
/** ISO/IEC 14496-1 (MPEG-4 Systems) SL-packetized stream or FlexMux stream
    carried in ISO_IEC_14496_section()s */
#define STREAM_TYPE_ISO_IEC_14496_SECTION 0x13
#define STREAM_TYPE_METADATA        0x15
#define STREAM_TYPE_VIDEO_H264      0x1b
/** ISO/IEC 14496-3 Audio, without using any additional transport syntax,
    such as DST, ALS and SLS */
#define STREAM_TYPE_AUDIO_MPEG4     0x1c
#define STREAM_TYPE_VIDEO_MVC       0x20
#define STREAM_TYPE_VIDEO_JPEG2000  0x21
#define STREAM_TYPE_VIDEO_HEVC      0x24
#define STREAM_TYPE_VIDEO_VVC       0x33
#define STREAM_TYPE_VIDEO_CAVS      0x42
#define STREAM_TYPE_VIDEO_AVS2      0xd2
#define STREAM_TYPE_VIDEO_AVS3      0xd4
#define STREAM_TYPE_VIDEO_VC1       0xea
#define STREAM_TYPE_VIDEO_DIRAC     0xd1

/* stream_type values [0x80, 0xff] are User Private */
#define STREAM_TYPE_BLURAY_AUDIO_PCM_BLURAY             0x80
#define STREAM_TYPE_BLURAY_AUDIO_AC3                    0x81
#define STREAM_TYPE_BLURAY_AUDIO_DTS                    0x82
#define STREAM_TYPE_BLURAY_AUDIO_TRUEHD                 0x83
#define STREAM_TYPE_BLURAY_AUDIO_EAC3                   0x84
#define STREAM_TYPE_BLURAY_AUDIO_DTS_HD                 0x85
#define STREAM_TYPE_BLURAY_AUDIO_DTS_HD_MASTER          0x86
#define STREAM_TYPE_BLURAY_AUDIO_EAC3_SECONDARY         0xa1
#define STREAM_TYPE_BLURAY_AUDIO_DTS_EXPRESS_SECONDARY  0xa2
#define STREAM_TYPE_BLURAY_SUBTITLE_PGS                 0x90
#define STREAM_TYPE_BLURAY_SUBTITLE_TEXT                0x92

#define STREAM_TYPE_SCTE_DATA_SCTE_35 0x86 /* ANSI/SCTE 35 */

#define STREAM_TYPE_ATSC_AUDIO_AC3  0x81 /* ATSC A/52 */
#define STREAM_TYPE_ATSC_AUDIO_EAC3 0x87 /* ATSC A/52 */

/* HTTP Live Streaming (HLS) Sample Encryption
   see "MPEG-2 Stream Encryption Format for HTTP Live Streaming",
https://developer.apple.com/library/archive/documentation/AudioVideo/Conceptual/HLS_Sample_Encryption/ */
#define STREAM_TYPE_HLS_SE_VIDEO_H264 0xdb
#define STREAM_TYPE_HLS_SE_AUDIO_AAC  0xcf
#define STREAM_TYPE_HLS_SE_AUDIO_AC3  0xc1
#define STREAM_TYPE_HLS_SE_AUDIO_EAC3 0xc2


/* ISO/IEC 13818-1 Table 2-22 */
#define STREAM_ID_PROGRAM_STREAM_MAP        0xbc
#define STREAM_ID_PRIVATE_STREAM_1          0xbd
#define STREAM_ID_PADDING_STREAM            0xbe
#define STREAM_ID_PRIVATE_STREAM_2          0xbf
#define STREAM_ID_AUDIO_STREAM_0            0xc0
#define STREAM_ID_VIDEO_STREAM_0            0xe0
#define STREAM_ID_ECM_STREAM                0xf0
#define STREAM_ID_EMM_STREAM                0xf1
#define STREAM_ID_DSMCC_STREAM              0xf2
#define STREAM_ID_TYPE_E_STREAM             0xf8
#define STREAM_ID_METADATA_STREAM           0xfc
#define STREAM_ID_EXTENDED_STREAM_ID        0xfd
#define STREAM_ID_PROGRAM_STREAM_DIRECTORY  0xff

/* ISO/IEC 13818-1 Table 2-45 */
#define VIDEO_STREAM_DESCRIPTOR      0x02
#define REGISTRATION_DESCRIPTOR      0x05
#define ISO_639_LANGUAGE_DESCRIPTOR  0x0a
#define IOD_DESCRIPTOR               0x1d
#define SL_DESCRIPTOR                0x1e
#define FMC_DESCRIPTOR               0x1f
#define METADATA_DESCRIPTOR          0x26
#define METADATA_STD_DESCRIPTOR      0x27
/* descriptor_tag values [0x40, 0xff] are User Private */

/* DVB descriptor tag values [0x40, 0x7F] from
   ETSI EN 300 468 Table 12: Possible locations of descriptors */
#define NETWORK_NAME_DESCRIPTOR      0x40
#define SERVICE_LIST_DESCRIPTOR      0x41
#define SERVICE_DESCRIPTOR           0x48
#define STREAM_IDENTIFIER_DESCRIPTOR 0x52
#define TELETEXT_DESCRIPTOR          0x56
#define SUBTITLING_DESCRIPTOR        0x59
#define AC3_DESCRIPTOR               0x6a /* AC-3_descriptor */
#define ENHANCED_AC3_DESCRIPTOR      0x7a /* enhanced_AC-3_descriptor */
#define DTS_DESCRIPTOR               0x7b
#define EXTENSION_DESCRIPTOR         0x7f

/* DVB descriptor_tag_extension values from
   ETSI EN 300 468 Table 109: Possible locations of extended descriptors */
#define SUPPLEMENTARY_AUDIO_DESCRIPTOR 0x06

/** see "Dolby Vision Streams Within the MPEG-2 Transport Stream Format"
https://professional.dolby.com/siteassets/content-creation/dolby-vision-for-content-creators/dolby-vision-bitstreams-in-mpeg-2-transport-stream-multiplex-v1.2.pdf */
#define DOVI_VIDEO_STREAM_DESCRIPTOR 0xb0

#define DATA_COMPONENT_DESCRIPTOR 0xfd /* ARIB STD-B10 */

typedef struct MpegTSContext MpegTSContext;

MpegTSContext *avpriv_mpegts_parse_open(AVFormatContext *s);
int avpriv_mpegts_parse_packet(MpegTSContext *ts, AVPacket *pkt,
                               const uint8_t *buf, int len);
void avpriv_mpegts_parse_close(MpegTSContext *ts);

typedef struct SLConfigDescr {
    int use_au_start;
    int use_au_end;
    int use_rand_acc_pt;
    int use_padding;
    int use_timestamps;
    int use_idle;
    int timestamp_res;
    int timestamp_len;
    int ocr_len;
    int au_len;
    int inst_bitrate_len;
    int degr_prior_len;
    int au_seq_num_len;
    int packet_seq_num_len;
} SLConfigDescr;

typedef struct Mp4Descr {
    int es_id;
    int dec_config_descr_len;
    uint8_t *dec_config_descr;
    SLConfigDescr sl;
} Mp4Descr;

/*
 * ETSI 300 468 descriptor 0x6A(AC-3)
 * Refer to: ETSI EN 300 468 V1.11.1 (2010-04) (SI in DVB systems)
 */
typedef struct DVBAC3Descriptor {
    uint8_t  component_type_flag;
    uint8_t  bsid_flag;
    uint8_t  mainid_flag;
    uint8_t  asvc_flag;
    uint8_t  reserved_flags;
    uint8_t  component_type;
    uint8_t  bsid;
    uint8_t  mainid;
    uint8_t  asvc;
} DVBAC3Descriptor;

/**
 * Parse an MPEG-2 descriptor
 * @param[in] fc                    Format context (used for logging only)
 * @param st                        Stream
 * @param stream_type               STREAM_TYPE_xxx
 * @param pp                        Descriptor buffer pointer
 * @param desc_list_end             End of buffer
 * @return <0 to stop processing
 */
int ff_parse_mpeg2_descriptor(AVFormatContext *fc, AVStream *st, int stream_type,
                              const uint8_t **pp, const uint8_t *desc_list_end,
                              Mp4Descr *mp4_descr, int mp4_descr_count, int pid,
                              MpegTSContext *ts);

/**
 * Check presence of H264 startcode
 * @return <0 to stop processing
 */
int ff_check_h264_startcode(AVFormatContext *s, const AVStream *st, const AVPacket *pkt);

#endif /* AVFORMAT_MPEGTS_H */
