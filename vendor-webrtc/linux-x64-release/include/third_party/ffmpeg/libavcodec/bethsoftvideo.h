/*
 * Bethesda VID video decoder
 * Copyright (C) 2007 Nicholas Tung
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

#ifndef AVCODEC_BETHSOFTVIDEO_H
#define AVCODEC_BETHSOFTVIDEO_H

enum BethsoftVidBlockType
{
    PALETTE_BLOCK       = 0x02,
    FIRST_AUDIO_BLOCK   = 0x7c,
    AUDIO_BLOCK         = 0x7d,
    VIDEO_I_FRAME       = 0x03,
    VIDEO_P_FRAME       = 0x01,
    VIDEO_YOFF_P_FRAME  = 0x04,
    EOF_BLOCK           = 0x14,
};

#endif /* AVCODEC_BETHSOFTVIDEO_H */
