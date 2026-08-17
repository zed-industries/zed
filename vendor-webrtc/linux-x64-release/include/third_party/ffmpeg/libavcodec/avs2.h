/*
 * AVS2 related definitions
 *
 * Copyright (C) 2022 Zhao Zhili, <zhilizhao@tencent.com>
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

#ifndef AVCODEC_AVS2_H
#define AVCODEC_AVS2_H

#include "libavutil/rational.h"

#define AVS2_SLICE_MAX_START_CODE    0x000001AF

enum {
    AVS2_SEQ_START_CODE         = 0xB0,
    AVS2_SEQ_END_CODE           = 0xB1,
    AVS2_USER_DATA_START_CODE   = 0xB2,
    AVS2_INTRA_PIC_START_CODE   = 0xB3,
    // reserved                 = 0xB4,
    AVS2_EXTENSION_START_CODE   = 0xB5,
    AVS2_INTER_PIC_START_CODE   = 0xB6,
};

#define AVS2_ISPIC(x)  ((x) == AVS2_INTRA_PIC_START_CODE || (x) == AVS2_INTER_PIC_START_CODE)
#define AVS2_ISUNIT(x) ((x) == AVS2_SEQ_START_CODE || AVS2_ISPIC(x))

enum AVS2Profile {
    AVS2_PROFILE_MAIN_PIC   = 0x12,
    AVS2_PROFILE_MAIN       = 0x20,
    AVS2_PROFILE_MAIN10     = 0x22,
};

extern const AVRational ff_avs2_frame_rate_tab[16];

#endif
