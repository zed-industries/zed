/*
 * Constants for DV codec
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

/**
 * @file
 * Constants for DV codec.
 */

#ifndef AVCODEC_DV_H
#define AVCODEC_DV_H

enum DVSectionType {
    DV_SECT_HEADER  = 0x1f,
    DV_SECT_SUBCODE = 0x3f,
    DV_SECT_VAUX    = 0x56,
    DV_SECT_AUDIO   = 0x76,
    DV_SECT_VIDEO   = 0x96,
};

enum DVPackType {
    DV_HEADER525     = 0x3f,  /* see dv_write_pack for important details on */
    DV_HEADER625     = 0xbf,  /* these two packs */
    DV_TIMECODE      = 0x13,
    DV_AUDIO_SOURCE  = 0x50,
    DV_AUDIO_CONTROL = 0x51,
    DV_AUDIO_RECDATE = 0x52,
    DV_AUDIO_RECTIME = 0x53,
    DV_VIDEO_SOURCE  = 0x60,
    DV_VIDEO_CONTROL = 0x61,
    DV_VIDEO_RECDATE = 0x62,
    DV_VIDEO_RECTIME = 0x63,
    DV_UNKNOWN_PACK  = 0xff,
};

#define DV_PROFILE_IS_HD(p) ((p)->video_stype & 0x10)
#define DV_PROFILE_IS_1080i50(p) (((p)->video_stype == 0x14) && ((p)->dsf == 1))
#define DV_PROFILE_IS_1080i60(p) (((p)->video_stype == 0x14) && ((p)->dsf == 0))
#define DV_PROFILE_IS_720p50(p)  (((p)->video_stype == 0x18) && ((p)->dsf == 1))

/**
 * largest possible DV frame, in bytes (1080i50)
 */
#define DV_MAX_FRAME_SIZE 576000

// LCM of video framerate numerators
#define DV_TIMESCALE_VIDEO 60000

// LCM of audio sample rates
#define DV_TIMESCALE_AUDIO 14112000

/**
 * maximum number of blocks per macroblock in any DV format
 */
#define DV_MAX_BPM 8

#endif /* AVCODEC_DV_H */
