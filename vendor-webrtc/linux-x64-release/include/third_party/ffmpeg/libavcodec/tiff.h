/*
 * Copyright (c) 2006 Konstantin Shishkov
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
 * TIFF constants & data structures
 *
 * For more information about the TIFF format, check the official docs at:
 * http://partners.adobe.com/public/developer/tiff/index.html
 * @author Konstantin Shishkov
 */

#ifndef AVCODEC_TIFF_H
#define AVCODEC_TIFF_H

/** TIFF types in ascenting priority (last in the list is highest) */
enum TiffType {
    /** TIFF image based on the TIFF 6.0 or TIFF/EP (ISO 12234-2) specifications */
    TIFF_TYPE_TIFF,
    /** Digital Negative (DNG) image */
    TIFF_TYPE_DNG,
    /** Digital Negative (DNG) image part of an CinemaDNG image sequence */
    TIFF_TYPE_CINEMADNG,
};

/** abridged list of TIFF and TIFF/EP tags */
enum TiffTags {
    TIFF_SUBFILE            = 0xfe,
    TIFF_WIDTH              = 0x100,
    TIFF_HEIGHT,
    TIFF_BPP,
    TIFF_COMPR,
    TIFF_PHOTOMETRIC        = 0x106,
    TIFF_FILL_ORDER         = 0x10A,
    TIFF_DOCUMENT_NAME      = 0x10D,
    TIFF_IMAGE_DESCRIPTION  = 0x10E,
    TIFF_MAKE               = 0x10F,
    TIFF_MODEL              = 0x110,
    TIFF_STRIP_OFFS         = 0x111,
    TIFF_SAMPLES_PER_PIXEL  = 0x115,
    TIFF_ROWSPERSTRIP       = 0x116,
    TIFF_STRIP_SIZE,
    TIFF_XRES               = 0x11A,
    TIFF_YRES               = 0x11B,
    TIFF_PLANAR             = 0x11C,
    TIFF_PAGE_NAME          = 0x11D,
    TIFF_XPOS               = 0x11E,
    TIFF_YPOS               = 0x11F,
    TIFF_GRAY_RESPONSE_CURVE= 0x123,
    TIFF_T4OPTIONS          = 0x124,
    TIFF_T6OPTIONS,
    TIFF_RES_UNIT           = 0x128,
    TIFF_PAGE_NUMBER        = 0x129,
    TIFF_SOFTWARE_NAME      = 0x131,
    TIFF_DATE               = 0x132,
    TIFF_ARTIST             = 0x13B,
    TIFF_HOST_COMPUTER      = 0x13C,
    TIFF_PREDICTOR          = 0x13D,
    TIFF_PAL                = 0x140,
    TIFF_TILE_WIDTH         = 0x142,
    TIFF_TILE_LENGTH        = 0x143,
    TIFF_TILE_OFFSETS       = 0x144,
    TIFF_TILE_BYTE_COUNTS   = 0x145,
    TIFF_SUB_IFDS           = 0x14A,
    TIFF_EXTRASAMPLES       = 0x152,
    TIFF_YCBCR_COEFFICIENTS = 0x211,
    TIFF_YCBCR_SUBSAMPLING  = 0x212,
    TIFF_YCBCR_POSITIONING  = 0x213,
    TIFF_REFERENCE_BW       = 0x214,
    TIFF_CFA_PATTERN_DIM    = 0x828D,
    TIFF_CFA_PATTERN        = 0x828E,
    TIFF_COPYRIGHT          = 0x8298,
    TIFF_MODEL_TIEPOINT     = 0x8482,
    TIFF_MODEL_PIXEL_SCALE  = 0x830E,
    TIFF_MODEL_TRANSFORMATION= 0x8480,
    TIFF_ICC_PROFILE        = 0x8773,
    TIFF_GEO_KEY_DIRECTORY  = 0x87AF,
    TIFF_GEO_DOUBLE_PARAMS  = 0x87B0,
    TIFF_GEO_ASCII_PARAMS   = 0x87B1,
};

/** abridged list of DNG tags */
enum DngTags {
    DNG_VERSION             = 0xC612,
    DNG_BACKWARD_VERSION    = 0xC613,
    DNG_LINEARIZATION_TABLE = 0xC618,
    DNG_BLACK_LEVEL         = 0xC61A,
    DNG_WHITE_LEVEL         = 0xC61D,
    DNG_COLOR_MATRIX1       = 0xC621,
    DNG_COLOR_MATRIX2       = 0xC622,
    DNG_CAMERA_CALIBRATION1 = 0xC623,
    DNG_CAMERA_CALIBRATION2 = 0xC624,
    DNG_ANALOG_BALANCE      = 0xC627,
    DNG_AS_SHOT_NEUTRAL     = 0xC628,
    DNG_AS_SHOT_WHITE_XY    = 0xC629,
};

/** list of CinemaDNG tags */
enum CinemaDngTags {
    CINEMADNG_TIME_CODES    = 0xC763,
    CINEMADNG_FRAME_RATE    = 0xC764,
    CINEMADNG_T_STOP        = 0xC772,
    CINEMADNG_REEL_NAME     = 0xC789,
    CINEMADNG_CAMERA_LABEL  = 0xC7A1,
};

/** list of TIFF, TIFF/EP and DNG compression types */
enum TiffCompr {
    TIFF_RAW = 1,
    TIFF_CCITT_RLE,
    TIFF_G3,
    TIFF_G4,
    TIFF_LZW,
    TIFF_JPEG,
    TIFF_NEWJPEG,
    TIFF_ADOBE_DEFLATE,
    TIFF_PACKBITS = 0x8005,
    TIFF_DEFLATE  = 0x80B2,
    TIFF_LZMA     = 0x886D,
};

enum TiffGeoTagKey {
    TIFF_GT_MODEL_TYPE_GEOKEY                = 1024,
    TIFF_GT_RASTER_TYPE_GEOKEY               = 1025,
    TIFF_GT_CITATION_GEOKEY                  = 1026,
    TIFF_GEOGRAPHIC_TYPE_GEOKEY              = 2048,
    TIFF_GEOG_CITATION_GEOKEY                = 2049,
    TIFF_GEOG_GEODETIC_DATUM_GEOKEY          = 2050,
    TIFF_GEOG_PRIME_MERIDIAN_GEOKEY          = 2051,
    TIFF_GEOG_LINEAR_UNITS_GEOKEY            = 2052,
    TIFF_GEOG_LINEAR_UNIT_SIZE_GEOKEY        = 2053,
    TIFF_GEOG_ANGULAR_UNITS_GEOKEY           = 2054,
    TIFF_GEOG_ANGULAR_UNIT_SIZE_GEOKEY       = 2055,
    TIFF_GEOG_ELLIPSOID_GEOKEY               = 2056,
    TIFF_GEOG_SEMI_MAJOR_AXIS_GEOKEY         = 2057,
    TIFF_GEOG_SEMI_MINOR_AXIS_GEOKEY         = 2058,
    TIFF_GEOG_INV_FLATTENING_GEOKEY          = 2059,
    TIFF_GEOG_AZIMUTH_UNITS_GEOKEY           = 2060,
    TIFF_GEOG_PRIME_MERIDIAN_LONG_GEOKEY     = 2061,
    TIFF_PROJECTED_CS_TYPE_GEOKEY            = 3072,
    TIFF_PCS_CITATION_GEOKEY                 = 3073,
    TIFF_PROJECTION_GEOKEY                   = 3074,
    TIFF_PROJ_COORD_TRANS_GEOKEY             = 3075,
    TIFF_PROJ_LINEAR_UNITS_GEOKEY            = 3076,
    TIFF_PROJ_LINEAR_UNIT_SIZE_GEOKEY        = 3077,
    TIFF_PROJ_STD_PARALLEL1_GEOKEY           = 3078,
    TIFF_PROJ_STD_PARALLEL2_GEOKEY           = 3079,
    TIFF_PROJ_NAT_ORIGIN_LONG_GEOKEY         = 3080,
    TIFF_PROJ_NAT_ORIGIN_LAT_GEOKEY          = 3081,
    TIFF_PROJ_FALSE_EASTING_GEOKEY           = 3082,
    TIFF_PROJ_FALSE_NORTHING_GEOKEY          = 3083,
    TIFF_PROJ_FALSE_ORIGIN_LONG_GEOKEY       = 3084,
    TIFF_PROJ_FALSE_ORIGIN_LAT_GEOKEY        = 3085,
    TIFF_PROJ_FALSE_ORIGIN_EASTING_GEOKEY    = 3086,
    TIFF_PROJ_FALSE_ORIGIN_NORTHING_GEOKEY   = 3087,
    TIFF_PROJ_CENTER_LONG_GEOKEY             = 3088,
    TIFF_PROJ_CENTER_LAT_GEOKEY              = 3089,
    TIFF_PROJ_CENTER_EASTING_GEOKEY          = 3090,
    TIFF_PROJ_CENTER_NORTHING_GEOKEY         = 3091,
    TIFF_PROJ_SCALE_AT_NAT_ORIGIN_GEOKEY     = 3092,
    TIFF_PROJ_SCALE_AT_CENTER_GEOKEY         = 3093,
    TIFF_PROJ_AZIMUTH_ANGLE_GEOKEY           = 3094,
    TIFF_PROJ_STRAIGHT_VERT_POLE_LONG_GEOKEY = 3095,
    TIFF_VERTICAL_CS_TYPE_GEOKEY             = 4096,
    TIFF_VERTICAL_CITATION_GEOKEY            = 4097,
    TIFF_VERTICAL_DATUM_GEOKEY               = 4098,
    TIFF_VERTICAL_UNITS_GEOKEY               = 4099
};

/** list of TIFF, TIFF/AP and DNG PhotometricInterpretation (TIFF_PHOTOMETRIC) values */
enum TiffPhotometric {
    TIFF_PHOTOMETRIC_NONE       = -1,
    TIFF_PHOTOMETRIC_WHITE_IS_ZERO,      /* mono or grayscale, 0 is white */
    TIFF_PHOTOMETRIC_BLACK_IS_ZERO,      /* mono or grayscale, 0 is black */
    TIFF_PHOTOMETRIC_RGB,                /* RGB or RGBA*/
    TIFF_PHOTOMETRIC_PALETTE,            /* Uses a palette */
    TIFF_PHOTOMETRIC_ALPHA_MASK,         /* Transparency mask */
    TIFF_PHOTOMETRIC_SEPARATED,          /* CMYK or some other ink set */
    TIFF_PHOTOMETRIC_YCBCR,              /* YCbCr */
    TIFF_PHOTOMETRIC_CIE_LAB    = 8,     /* 1976 CIE L*a*b* */
    TIFF_PHOTOMETRIC_ICC_LAB,            /* ICC L*a*b* */
    TIFF_PHOTOMETRIC_ITU_LAB,            /* ITU L*a*b* */
    TIFF_PHOTOMETRIC_CFA        = 32803, /* Color Filter Array (TIFF/AP and DNG) */
    TIFF_PHOTOMETRIC_LOG_L      = 32844, /* CIE Log2(L) */
    TIFF_PHOTOMETRIC_LOG_LUV,            /* CIE Log L*u*v* */
    TIFF_PHOTOMETRIC_LINEAR_RAW = 34892, /* Linear Raw (DNG) */
};

enum TiffGeoTagType {
    GEOTIFF_SHORT  = 0,
    GEOTIFF_DOUBLE = 34736,
    GEOTIFF_STRING = 34737
};

typedef struct TiffGeoTag {
    enum TiffGeoTagKey key;
    enum TiffTags type;
    int count;
    int offset;
    char *val;
} TiffGeoTag;

typedef struct TiffGeoTagKeyName {
    const enum TiffGeoTagKey key;
    const char *const name;
} TiffGeoTagKeyName;

#endif /* AVCODEC_TIFF_H */
