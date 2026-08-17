/*
 * MXF
 * Copyright (c) 2006 SmartJog S.A., Baptiste Coudurier <baptiste dot coudurier at smartjog dot com>
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
#ifndef AVFORMAT_MXF_H
#define AVFORMAT_MXF_H

#include <stdint.h>
#include "libavutil/log.h"
#include "libavutil/pixfmt.h"
#include "libavutil/rational.h"
#include "libavutil/uuid.h"

typedef AVUUID UID;

enum MXFMetadataSetType {
    MaterialPackage = 1,
    SourcePackage,
    SourceClip,
    TimecodeComponent,
    PulldownComponent,
    Sequence,
    MultipleDescriptor,
    Descriptor,
    Track,
    CryptoContext,
    Preface,
    Identification,
    ContentStorage,
    SubDescriptor,
    IndexTableSegment,
    EssenceContainerData,
    EssenceGroup,
    TaggedValue,
    TapeDescriptor,
    AVCSubDescriptor,
    AudioChannelLabelSubDescriptor,
    SoundfieldGroupLabelSubDescriptor,
    GroupOfSoundfieldGroupsLabelSubDescriptor,
    FFV1SubDescriptor,
    JPEG2000SubDescriptor,
    MetadataSetTypeNB
};

enum MXFFrameLayout {
    FullFrame = 0,
    SeparateFields,
    OneField,
    MixedFields,
    SegmentedFrame,
};

typedef struct MXFContentPackageRate {
    int rate;
    AVRational tb;
} MXFContentPackageRate;

typedef struct KLVPacket {
    UID key;
    int64_t offset;
    uint64_t length;
    int64_t next_klv;
} KLVPacket;

typedef enum {
    NormalWrap = 0,
    D10D11Wrap,
    RawAWrap,
    RawVWrap,
    J2KWrap
} MXFWrappingIndicatorType;

typedef struct MXFLocalTagPair {
    int local_tag;
    UID uid;
} MXFLocalTagPair;

extern const uint8_t ff_mxf_random_index_pack_key[16];

#define FF_MXF_MasteringDisplay_PREFIX                  0x06,0x0e,0x2b,0x34,0x01,0x01,0x01,0x0e,0x04,0x20,0x04,0x01,0x01
#define FF_MXF_MasteringDisplayPrimaries                { FF_MXF_MasteringDisplay_PREFIX,0x01,0x00,0x00 }
#define FF_MXF_MasteringDisplayWhitePointChromaticity   { FF_MXF_MasteringDisplay_PREFIX,0x02,0x00,0x00 }
#define FF_MXF_MasteringDisplayMaximumLuminance         { FF_MXF_MasteringDisplay_PREFIX,0x03,0x00,0x00 }
#define FF_MXF_MasteringDisplayMinimumLuminance         { FF_MXF_MasteringDisplay_PREFIX,0x04,0x00,0x00 }

#define FF_MXF_MASTERING_CHROMA_DEN 50000
#define FF_MXF_MASTERING_LUMA_DEN   10000

typedef struct MXFCodecUL {
    UID uid;
    unsigned matching_len;
    int id;
    const char *desc;
    unsigned wrapping_indicator_pos;
    MXFWrappingIndicatorType wrapping_indicator_type;
} MXFCodecUL;

extern const MXFCodecUL ff_mxf_data_definition_uls[];
extern const MXFCodecUL ff_mxf_codec_uls[];
extern const MXFCodecUL ff_mxf_pixel_format_uls[];
extern const MXFCodecUL ff_mxf_codec_tag_uls[];
extern const MXFCodecUL ff_mxf_color_primaries_uls[];
extern const MXFCodecUL ff_mxf_color_trc_uls[];
extern const MXFCodecUL ff_mxf_color_space_uls[];

int ff_mxf_decode_pixel_layout(const char pixel_layout[16], enum AVPixelFormat *pix_fmt);
int ff_mxf_get_content_package_rate(AVRational time_base);


#define PRIxUID                             \
    "%02x.%02x.%02x.%02x."                  \
    "%02x.%02x.%02x.%02x."                  \
    "%02x.%02x.%02x.%02x."                  \
    "%02x.%02x.%02x.%02x"

#define UID_ARG(x) \
    (x)[0],  (x)[1],  (x)[2],  (x)[3],      \
    (x)[4],  (x)[5],  (x)[6],  (x)[7],      \
    (x)[8],  (x)[9],  (x)[10], (x)[11],     \
    (x)[12], (x)[13], (x)[14], (x)[15]      \

#ifdef DEBUG
#define PRINT_KEY(pc, s, x)                         \
    av_log(pc, AV_LOG_VERBOSE,                      \
           "%s "                                    \
           "0x%02x,0x%02x,0x%02x,0x%02x,"           \
           "0x%02x,0x%02x,0x%02x,0x%02x,"           \
           "0x%02x,0x%02x,0x%02x,0x%02x,"           \
           "0x%02x,0x%02x,0x%02x,0x%02x ",          \
            s, UID_ARG(x));                         \
    av_log(pc, AV_LOG_INFO,                         \
           "%s "                                    \
           "%02x.%02x.%02x.%02x."                   \
           "%02x.%02x.%02x.%02x."                   \
           "%02x.%02x.%02x.%02x."                   \
           "%02x.%02x.%02x.%02x\n",                 \
            s, UID_ARG(x))
#else
#define PRINT_KEY(pc, s, x) do { if(0)              \
    av_log(pc, AV_LOG_VERBOSE,                      \
           "%s "                                    \
           "0x%02x,0x%02x,0x%02x,0x%02x,"           \
           "0x%02x,0x%02x,0x%02x,0x%02x,"           \
           "0x%02x,0x%02x,0x%02x,0x%02x,"           \
           "0x%02x,0x%02x,0x%02x,0x%02x ",          \
            s, UID_ARG(x));                         \
    }while(0)
#endif

#endif /* AVFORMAT_MXF_H */
