/*
 *  AVS3 related definitions
 *
 * Copyright (C) 2020 Huiwen Ren, <hwrenx@gmail.com>
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

#ifndef AVCODEC_AVS3_H
#define AVCODEC_AVS3_H

#define AVS3_NAL_START_CODE          0x010000
#define AVS3_SEQ_START_CODE          0xB0
#define AVS3_SEQ_END_CODE            0xB1
#define AVS3_USER_DATA_START_CODE    0xB2
#define AVS3_INTRA_PIC_START_CODE    0xB3
#define AVS3_UNDEF_START_CODE        0xB4
#define AVS3_EXTENSION_START_CODE    0xB5
#define AVS3_INTER_PIC_START_CODE    0xB6
#define AVS3_VIDEO_EDIT_CODE         0xB7
#define AVS3_FIRST_SLICE_START_CODE  0x00
#define AVS3_PROFILE_BASELINE_MAIN   0x20
#define AVS3_PROFILE_BASELINE_MAIN10 0x22

#define AVS3_ISPIC(x) ((x) == AVS3_INTRA_PIC_START_CODE || (x) == AVS3_INTER_PIC_START_CODE)
#define AVS3_ISUNIT(x) ((x) == AVS3_SEQ_START_CODE || AVS3_ISPIC(x))

#include "libavutil/avutil.h"
#include "libavutil/pixfmt.h"
#include "libavutil/rational.h"

static const AVRational ff_avs3_frame_rate_tab[16] = {
    { 0    , 0   }, // forbid
    { 24000, 1001},
    { 24   , 1   },
    { 25   , 1   },
    { 30000, 1001},
    { 30   , 1   },
    { 50   , 1   },
    { 60000, 1001},
    { 60   , 1   },
    { 100  , 1   },
    { 120  , 1   },
    { 200  , 1   },
    { 240  , 1   },
    { 300  , 1   },
    { 0    , 0   }, // reserved
    { 0    , 0   }  // reserved
};

static const int ff_avs3_color_primaries_tab[10] = {
    AVCOL_PRI_RESERVED0   ,    // 0
    AVCOL_PRI_BT709       ,    // 1
    AVCOL_PRI_UNSPECIFIED ,    // 2
    AVCOL_PRI_RESERVED    ,    // 3
    AVCOL_PRI_BT470M      ,    // 4
    AVCOL_PRI_BT470BG     ,    // 5
    AVCOL_PRI_SMPTE170M   ,    // 6
    AVCOL_PRI_SMPTE240M   ,    // 7
    AVCOL_PRI_FILM        ,    // 8
    AVCOL_PRI_BT2020           // 9
};

static const int ff_avs3_color_transfer_tab[15] = {
    AVCOL_TRC_RESERVED0    , // 0
    AVCOL_TRC_BT709        , // 1
    AVCOL_TRC_UNSPECIFIED  , // 2
    AVCOL_TRC_RESERVED     , // 3
    AVCOL_TRC_GAMMA22      , // 4
    AVCOL_TRC_GAMMA28      , // 5
    AVCOL_TRC_SMPTE170M    , // 6
    AVCOL_TRC_SMPTE240M    , // 7
    AVCOL_TRC_LINEAR       , // 8
    AVCOL_TRC_LOG          , // 9
    AVCOL_TRC_LOG_SQRT     , // 10
    AVCOL_TRC_BT2020_12    , // 11
    AVCOL_TRC_SMPTE2084    , // 12
    AVCOL_TRC_UNSPECIFIED  , // 13
    AVCOL_TRC_ARIB_STD_B67   // 14
};

static const int ff_avs3_color_matrix_tab[12] = {
    AVCOL_SPC_RESERVED     , // 0
    AVCOL_SPC_BT709        , // 1
    AVCOL_SPC_UNSPECIFIED  , // 2
    AVCOL_SPC_RESERVED     , // 3
    AVCOL_SPC_FCC          , // 4
    AVCOL_SPC_BT470BG      , // 5
    AVCOL_SPC_SMPTE170M    , // 6
    AVCOL_SPC_SMPTE240M    , // 7
    AVCOL_SPC_BT2020_NCL   , // 8
    AVCOL_SPC_BT2020_CL    , // 9
    AVCOL_SPC_UNSPECIFIED  , // 10
    AVCOL_SPC_UNSPECIFIED    // 11
};

static const enum AVPictureType ff_avs3_image_type[4] = {
    AV_PICTURE_TYPE_NONE,
    AV_PICTURE_TYPE_I,
    AV_PICTURE_TYPE_P,
    AV_PICTURE_TYPE_B
};

#endif /* AVCODEC_AVS3_H */
