/*
 * WavPack decoder/encoder common code
 * Copyright (c) 2006,2011 Konstantin Shishkov
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

#ifndef AVCODEC_WAVPACK_H
#define AVCODEC_WAVPACK_H

#include <limits.h>
#include <stdint.h>
#include "libavutil/attributes.h"
#include "libavutil/intmath.h"

#define MAX_TERMS      16
#define MAX_TERM        8

#define WV_HEADER_SIZE    32

#define WV_MONO           0x00000004
#define WV_JOINT_STEREO   0x00000010
#define WV_CROSS_DECORR   0x00000020
#define WV_FLOAT_DATA     0x00000080
#define WV_INT32_DATA     0x00000100
#define WV_FALSE_STEREO   0x40000000
#define WV_DSD_DATA       0x80000000

#define WV_HYBRID_MODE    0x00000008
#define WV_HYBRID_SHAPE   0x00000008
#define WV_HYBRID_BITRATE 0x00000200
#define WV_HYBRID_BALANCE 0x00000400
#define WV_INITIAL_BLOCK  0x00000800
#define WV_FINAL_BLOCK    0x00001000

#define WV_MONO_DATA    (WV_MONO | WV_FALSE_STEREO)

#define WV_SINGLE_BLOCK (WV_INITIAL_BLOCK | WV_FINAL_BLOCK)

#define WV_FLT_SHIFT_ONES 0x01
#define WV_FLT_SHIFT_SAME 0x02
#define WV_FLT_SHIFT_SENT 0x04
#define WV_FLT_ZERO_SENT  0x08
#define WV_FLT_ZERO_SIGN  0x10

#define WV_MAX_CHANNELS   (1 << 12)
#define WV_MAX_SAMPLES    150000

enum WP_ID_Flags {
    WP_IDF_MASK   = 0x3F,
    WP_IDF_IGNORE = 0x20,
    WP_IDF_ODD    = 0x40,
    WP_IDF_LONG   = 0x80
};

enum WP_ID {
    WP_ID_DUMMY = 0,
    WP_ID_ENCINFO,
    WP_ID_DECTERMS,
    WP_ID_DECWEIGHTS,
    WP_ID_DECSAMPLES,
    WP_ID_ENTROPY,
    WP_ID_HYBRID,
    WP_ID_SHAPING,
    WP_ID_FLOATINFO,
    WP_ID_INT32INFO,
    WP_ID_DATA,
    WP_ID_CORR,
    WP_ID_EXTRABITS,
    WP_ID_CHANINFO,
    WP_ID_DSD_DATA,
    WP_ID_SAMPLE_RATE = 0x27,
};

typedef struct Decorr {
    int delta;
    int value;
    int weightA;
    int weightB;
    int samplesA[MAX_TERM];
    int samplesB[MAX_TERM];
    int sumA;
    int sumB;
} Decorr;

typedef struct WvChannel {
    int median[3];
    int slow_level, error_limit;
    unsigned bitrate_acc, bitrate_delta;
} WvChannel;

// macros for manipulating median values
#define GET_MED(n) ((c->median[n] >> 4) + 1)
#define DEC_MED(n) c->median[n] -= ((int)(c->median[n] + (128U >> (n)) - 2) / (128 >> (n))) * 2U
#define INC_MED(n) c->median[n] += ((int)(c->median[n] + (128U >> (n))    ) / (128 >> (n))) * 5U

// macros for applying weight
#define UPDATE_WEIGHT_CLIP(weight, delta, samples, in) \
    if ((samples) && (in)) { \
        if (((samples) ^ (in)) < 0) { \
            (weight) -= (delta); \
            if ((weight) < -1024) \
                (weight) = -1024; \
        } else { \
            (weight) += (delta); \
            if ((weight) > 1024) \
                (weight) = 1024; \
        } \
    }

static const int wv_rates[16] = {
     6000,  8000,  9600, 11025, 12000, 16000,  22050, 24000,
    32000, 44100, 48000, 64000, 88200, 96000, 192000,     0
};

// exponent table copied from WavPack source
extern const uint8_t ff_wp_exp2_table[256];
extern const uint8_t ff_wp_log2_table[256];

static av_always_inline int wp_exp2(int16_t val)
{
    int res, neg = 0;

    if (val < 0) {
        val = -val;
        neg = 1;
    }

    res   = ff_wp_exp2_table[val & 0xFF] | 0x100;
    val >>= 8;
    if (val > 31U)
        return INT_MIN;
    res   = (val > 9) ? (res << (val - 9)) : (res >> (9 - val));
    return neg ? -res : res;
}

static av_always_inline int wp_log2(uint32_t val)
{
    int bits;

    if (!val)
        return 0;
    if (val == 1)
        return 256;
    val += val >> 9;
    bits = av_log2(val) + 1;
    if (bits < 9)
        return (bits << 8) + ff_wp_log2_table[(val << (9 - bits)) & 0xFF];
    else
        return (bits << 8) + ff_wp_log2_table[(val >> (bits - 9)) & 0xFF];
}

#endif /* AVCODEC_WAVPACK_H */
