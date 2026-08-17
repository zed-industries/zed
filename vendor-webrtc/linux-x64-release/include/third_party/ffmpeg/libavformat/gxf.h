/*
 * GXF demuxer
 * copyright (c) 2006 Reimar Doeffinger
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

#ifndef AVFORMAT_GXF_H
#define AVFORMAT_GXF_H

typedef enum {
    PKT_MAP         = 0xbc,
    PKT_MEDIA       = 0xbf,
    PKT_EOS         = 0xfb,
    PKT_FLT         = 0xfc,
    PKT_UMF         = 0xfd,
} GXFPktType;

typedef enum {
    MAT_NAME        = 0x40,
    MAT_FIRST_FIELD = 0x41,
    MAT_LAST_FIELD  = 0x42,
    MAT_MARK_IN     = 0x43,
    MAT_MARK_OUT    = 0x44,
    MAT_SIZE        = 0x45,
} GXFMatTag;

typedef enum {
    TRACK_NAME      = 0x4c,
    TRACK_AUX       = 0x4d,
    TRACK_VER       = 0x4e,
    TRACK_MPG_AUX   = 0x4f,
    TRACK_FPS       = 0x50,
    TRACK_LINES     = 0x51,
    TRACK_FPF       = 0x52,
} GXFTrackTag;

#endif /* AVFORMAT_GXF_H */
