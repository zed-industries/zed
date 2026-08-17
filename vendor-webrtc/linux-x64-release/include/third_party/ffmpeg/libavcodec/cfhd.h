/*
 * Copyright (c) 2015 Kieran Kunhya
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

#ifndef AVCODEC_CFHD_H
#define AVCODEC_CFHD_H

#include <stdint.h>

#include "avcodec.h"
#include "bytestream.h"
#include "cfhddsp.h"

enum CFHDParam {
    SampleType       =   1,
    SampleIndexTable =   2,
    BitstreamMarker  =   4,
    VersionMajor     =   5,
    VersionMinor     =   6,
    VersionRevision  =   7,
    VersionEdit      =   8,
    TransformType    =  10,
    NumFrames        =  11,
    ChannelCount     =  12,
    WaveletCount     =  13,
    SubbandCount     =  14,
    NumSpatial       =  15,
    FirstWavelet     =  16,
    GroupTrailer     =  18,
    FrameType        =  19,
    ImageWidth       =  20,
    ImageHeight      =  21,
    FrameIndex       =  23,
    LowpassSubband   =  25,
    NumLevels        =  26,
    LowpassWidth     =  27,
    LowpassHeight    =  28,
    PixelOffset      =  33,
    LowpassQuantization=34,
    LowpassPrecision =  35,
    WaveletType      =  37,
    WaveletNumber    =  38,
    WaveletLevel     =  39,
    NumBands         =  40,
    HighpassWidth    =  41,
    HighpassHeight   =  42,
    LowpassBorder    =  43,
    HighpassBorder   =  44,
    LowpassScale     =  45,
    LowpassDivisor   =  46,
    SubbandNumber    =  48,
    BandWidth        =  49,
    BandHeight       =  50,
    SubbandBand      =  51,
    BandEncoding     =  52,
    Quantization     =  53,
    BandScale        =  54,
    BandHeader       =  55,
    BandTrailer      =  56,
    ChannelNumber    =  62,
    SampleFlags      =  68,
    FrameNumber      =  69,
    Precision        =  70,
    InputFormat      =  71,
    BandCodingFlags  =  72,
    PeakLevel        =  74,
    PeakOffsetLow    =  75,
    PeakOffsetHigh   =  76,
    Version          =  79,
    BandSecondPass   =  82,
    PrescaleTable    =  83,
    EncodedFormat    =  84,
    DisplayHeight    =  85,
    ChannelWidth     = 104,
    ChannelHeight    = 105,
};

#define VLC_BITS       9
#define SUBBAND_COUNT 10
#define SUBBAND_COUNT_3D 17

typedef struct CFHD_RL_VLC_ELEM {
    int16_t level;
    int8_t len8;
    uint16_t run;
} CFHD_RL_VLC_ELEM;

#define DWT_LEVELS 3
#define DWT_LEVELS_3D 6

typedef struct SubBand {
    ptrdiff_t stride;
    int a_width;
    int width;
    int a_height;
    int height;
    int8_t read_ok;
} SubBand;

typedef struct Plane {
    int width;
    int height;
    ptrdiff_t stride;

    int16_t *idwt_buf;
    int16_t *idwt_tmp;
    int      idwt_size;

    /* TODO: merge this into SubBand structure */
    int16_t *subband[SUBBAND_COUNT_3D];
    int16_t *l_h[10];

    SubBand band[DWT_LEVELS_3D][4];
} Plane;

typedef struct Peak {
    int level;
    int offset;
    GetByteContext base;
} Peak;

typedef struct CFHDContext {
    AVCodecContext *avctx;

    CFHD_RL_VLC_ELEM table_9_rl_vlc[2088];
    CFHD_RL_VLC_ELEM table_18_rl_vlc[4572];

    int lut[2][256];

    int planes;
    int frame_type;
    int frame_index;
    int sample_type;
    int transform_type;
    int coded_width;
    int coded_height;
    int cropped_height;
    enum AVPixelFormat coded_format;
    int progressive;

    int a_width;
    int a_height;
    int a_format;
    int a_transform_type;

    int bpc; // bits per channel/component
    int channel_cnt;
    int subband_cnt;
    int band_encoding;
    int channel_num;
    uint8_t lowpass_precision;
    uint16_t quantisation;

    int codebook;
    int difference_coding;
    int subband_num;
    int level;
    int subband_num_actual;

    uint8_t prescale_table[8];
    Plane plane[4];
    Peak peak;

    CFHDDSPContext dsp;
} CFHDContext;

int ff_cfhd_init_vlcs(CFHDContext *s);

#endif /* AVCODEC_CFHD_H */
