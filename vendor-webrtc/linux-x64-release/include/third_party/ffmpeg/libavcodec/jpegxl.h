/*
 * JPEG XL Common Header Definitions
 * Copyright (c) 2023 Leo Izen <leo.izen@gmail.com>
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

#ifndef AVCODEC_JPEGXL_H
#define AVCODEC_JPEGXL_H

#define FF_JPEGXL_CODESTREAM_SIGNATURE_LE 0x0aff
#define FF_JPEGXL_CONTAINER_SIGNATURE_LE 0x204c584a0c000000
#define FF_JPEGXL_CODESTREAM_SIGNATURE_BE 0xff0a
#define FF_JPEGXL_CONTAINER_SIGNATURE_BE 0x0000000c4a584c20

typedef enum FFJXLFrameEncoding {
    JPEGXL_ENC_VARDCT,
    JPEGXL_ENC_MODULAR
} FFJXLFrameEncoding;

typedef enum FFJXLFrameType {
    JPEGXL_FRAME_REGULAR,
    JPEGXL_FRAME_LF,
    JPEGXL_FRAME_REFERENCE_ONLY,
    JPEGXL_FRAME_SKIP_PROGRESSIVE
} FFJXLFrameType;

typedef enum FFJXLBlendMode {
    JPEGXL_BM_REPLACE,
    JPEGXL_BM_ADD,
    JPEGXL_BM_BLEND,
    JPEGXL_BM_MULADD,
    JPEGXL_BM_MUL
} FFJXLBlendMode;

typedef enum FFJXLExtraChannelType {
    JPEGXL_CT_ALPHA = 0,
    JPEGXL_CT_DEPTH,
    JPEGXL_CT_SPOT_COLOR,
    JPEGXL_CT_SELECTION_MASK,
    JPEGXL_CT_BLACK,
    JPEGXL_CT_CFA,
    JPEGXL_CT_THERMAL,
    JPEGXL_CT_NON_OPTIONAL = 15,
    JPEGXL_CT_OPTIONAL
} FFJXLExtraChannelType;

typedef enum FFJXLColorSpace {
    JPEGXL_CS_RGB = 0,
    JPEGXL_CS_GRAY,
    JPEGXL_CS_XYB,
    JPEGXL_CS_UNKNOWN
} FFJXLColorSpace;

typedef enum FFJXLWhitePoint {
    JPEGXL_WP_D65 = 1,
    JPEGXL_WP_CUSTOM,
    JPEGXL_WP_E = 10,
    JPEGXL_WP_DCI = 11
} FFJXLWhitePoint;

typedef enum FFJXLPrimaries {
    JPEGXL_PR_SRGB = 1,
    JPEGXL_PR_CUSTOM,
    JPEGXL_PR_2100 = 9,
    JPEGXL_PR_P3 = 11,
} FFJXLPrimaries;

typedef enum FFJXLTransferCharacteristic {
    JPEGXL_TR_BT709 = 1,
    JPEGXL_TR_UNKNOWN,
    JPEGXL_TR_LINEAR = 8,
    JPEGXL_TR_SRGB = 13,
    JPEGXL_TR_PQ = 16,
    JPEGXL_TR_DCI,
    JPEGXL_TR_HLG,
    JPEGXL_TR_GAMMA = 1 << 24,
} FFJXLTransferCharacteristic;

#endif /* AVCODEC_JPEGXL_H */
