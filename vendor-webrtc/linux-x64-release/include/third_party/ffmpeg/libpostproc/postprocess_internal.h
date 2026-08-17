/*
 * Copyright (C) 2001-2002 Michael Niedermayer (michaelni@gmx.at)
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

/**
 * @file
 * internal API header.
 */

#ifndef POSTPROC_POSTPROCESS_INTERNAL_H
#define POSTPROC_POSTPROCESS_INTERNAL_H

#include <string.h>
#include "libavutil/avutil.h"
#include "libavutil/intmath.h"
#include "libavutil/log.h"
#include "libavutil/mem_internal.h"
#include "postprocess.h"

#define V_DEBLOCK       0x01
#define H_DEBLOCK       0x02
#define DERING          0x04
#define LEVEL_FIX       0x08 ///< Brightness & Contrast

#define LUM_V_DEBLOCK   V_DEBLOCK               //   1
#define LUM_H_DEBLOCK   H_DEBLOCK               //   2
#define CHROM_V_DEBLOCK (V_DEBLOCK<<4)          //  16
#define CHROM_H_DEBLOCK (H_DEBLOCK<<4)          //  32
#define LUM_DERING      DERING                  //   4
#define CHROM_DERING    (DERING<<4)             //  64
#define LUM_LEVEL_FIX   LEVEL_FIX               //   8
#define CHROM_LEVEL_FIX (LEVEL_FIX<<4)          // 128 (not implemented yet)

// Experimental vertical filters
#define V_X1_FILTER     0x0200                  // 512
#define V_A_DEBLOCK     0x0400

// Experimental horizontal filters
#define H_X1_FILTER     0x2000                  // 8192
#define H_A_DEBLOCK     0x4000

/// select between full y range (255-0) or standard one (234-16)
#define FULL_Y_RANGE    0x8000                  // 32768

//Deinterlacing Filters
#define LINEAR_IPOL_DEINT_FILTER        0x10000 // 65536
#define LINEAR_BLEND_DEINT_FILTER       0x20000 // 131072
#define CUBIC_BLEND_DEINT_FILTER        0x8000  // (not implemented yet)
#define CUBIC_IPOL_DEINT_FILTER         0x40000 // 262144
#define MEDIAN_DEINT_FILTER             0x80000 // 524288
#define FFMPEG_DEINT_FILTER             0x400000
#define LOWPASS5_DEINT_FILTER           0x800000

#define TEMP_NOISE_FILTER               0x100000
#define FORCE_QUANT                     0x200000
#define BITEXACT                        0x1000000
#define VISUALIZE                       0x2000000

//use if you want a faster postprocessing code
//cannot differentiate between chroma & luma filters (both on or both off)
//obviously the -pp option on the command line has no effect except turning the here selected
//filters on
//#define COMPILE_TIME_MODE 0x77

/**
 * Postprocessing filter.
 */
struct PPFilter{
    const char *shortName;
    const char *longName;
    int chromDefault;       ///< is chrominance filtering on by default if this filter is manually activated
    int minLumQuality;      ///< minimum quality to turn luminance filtering on
    int minChromQuality;    ///< minimum quality to turn chrominance filtering on
    int mask;               ///< Bitmask to turn this filter on
};

/**
 * Postprocessing mode.
 */
typedef struct PPMode{
    int lumMode;                    ///< activates filters for luminance
    int chromMode;                  ///< activates filters for chrominance
    int error;                      ///< non zero on error

    int minAllowedY;                ///< for brightness correction
    int maxAllowedY;                ///< for brightness correction
    AVRational maxClippedThreshold; ///< amount of "black" you are willing to lose to get a brightness-corrected picture

    int maxTmpNoise[3];             ///< for Temporal Noise Reducing filter (Maximal sum of abs differences)

    int baseDcDiff;
    int flatnessThreshold;

    int forcedQuant;                ///< quantizer if FORCE_QUANT is used
} PPMode;

/**
 * postprocess context.
 */
typedef struct PPContext{
    /**
     * info on struct for av_log
     */
    const AVClass *av_class;

    uint8_t *tempBlocks; ///<used for the horizontal code

    /**
     * luma histogram.
     * we need 64bit here otherwise we'll going to have a problem
     * after watching a black picture for 5 hours
     */
    uint64_t *yHistogram;

    DECLARE_ALIGNED(8, uint64_t, packedYOffset);
    DECLARE_ALIGNED(8, uint64_t, packedYScale);

    /** Temporal noise reducing buffers */
    uint8_t *tempBlurred[3];
    int32_t *tempBlurredPast[3];

    /** Temporary buffers for handling the last row(s) */
    uint8_t *tempDst;
    uint8_t *tempSrc;

    uint8_t *deintTemp;

    DECLARE_ALIGNED(8, uint64_t, pQPb);
    DECLARE_ALIGNED(8, uint64_t, pQPb2);

    DECLARE_ALIGNED(32, uint64_t, pQPb_block)[4];
    DECLARE_ALIGNED(32, uint64_t, pQPb2_block)[4];

    DECLARE_ALIGNED(32, uint64_t, mmxDcOffset)[64];
    DECLARE_ALIGNED(32, uint64_t, mmxDcThreshold)[64];

    int8_t *stdQPTable;       ///< used to fix MPEG2 style qscale
    int8_t *nonBQPTable;
    int8_t *forcedQPTable;

    int QP;
    int nonBQP;

    DECLARE_ALIGNED(32, int, QP_block)[4];
    DECLARE_ALIGNED(32, int, nonBQP_block)[4];

    int frameNum;

    int cpuCaps;

    int qpStride; ///<size of qp buffers (needed to realloc them if needed)
    int stride;   ///<size of some buffers (needed to realloc them if needed)

    int hChromaSubSample;
    int vChromaSubSample;

    PPMode ppMode;
} PPContext;


static inline void linecpy(void *dest, const void *src, int lines, int stride) {
    if (stride > 0) {
        memcpy(dest, src, lines*stride);
    } else {
        memcpy((uint8_t*)dest+(lines-1)*stride, (const uint8_t*)src+(lines-1)*stride, -lines*stride);
    }
}

#endif /* POSTPROC_POSTPROCESS_INTERNAL_H */
