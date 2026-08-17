/*
 * Header file for hardcoded PCM tables
 *
 * Copyright (c) 2010 Reimar Döffinger <Reimar.Doeffinger@gmx.de>
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

#ifndef AVCODEC_PCM_TABLEGEN_H
#define AVCODEC_PCM_TABLEGEN_H

#include <stdint.h>
#include "libavutil/attributes.h"

/* from g711.c by SUN microsystems (unrestricted use) */

#define         SIGN_BIT        (0x80)      /* Sign bit for a A-law byte. */
#define         QUANT_MASK      (0xf)       /* Quantization field mask. */
#define         NSEGS           (8)         /* Number of A-law segments. */
#define         SEG_SHIFT       (4)         /* Left shift for segment number. */
#define         SEG_MASK        (0x70)      /* Segment field mask. */

#define         BIAS            (0x84)      /* Bias for linear code. */

#define         VIDC_SIGN_BIT    (1)
#define         VIDC_QUANT_MASK  (0x1E)
#define         VIDC_QUANT_SHIFT (1)
#define         VIDC_SEG_SHIFT   (5)
#define         VIDC_SEG_MASK    (0xE0)

/* alaw2linear() - Convert an A-law value to 16-bit linear PCM */
static av_cold int alaw2linear(unsigned char a_val)
{
    int t;
    int seg;

    a_val ^= 0x55;

    t = a_val & QUANT_MASK;
    seg = ((unsigned)a_val & SEG_MASK) >> SEG_SHIFT;
    if(seg) t= (t + t + 1 + 32) << (seg + 2);
    else    t= (t + t + 1     ) << 3;

    return (a_val & SIGN_BIT) ? t : -t;
}

static av_cold int ulaw2linear(unsigned char u_val)
{
    int t;

    /* Complement to obtain normal u-law value. */
    u_val = ~u_val;

    /*
     * Extract and bias the quantization bits. Then
     * shift up by the segment number and subtract out the bias.
     */
    t = ((u_val & QUANT_MASK) << 3) + BIAS;
    t <<= ((unsigned)u_val & SEG_MASK) >> SEG_SHIFT;

    return (u_val & SIGN_BIT) ? (BIAS - t) : (t - BIAS);
}

static av_cold int vidc2linear(unsigned char u_val)
{
    int t;

    /*
     * Extract and bias the quantization bits. Then
     * shift up by the segment number and subtract out the bias.
     */
    t = (((u_val & VIDC_QUANT_MASK) >> VIDC_QUANT_SHIFT) << 3) + BIAS;
    t <<= ((unsigned)u_val & VIDC_SEG_MASK) >> VIDC_SEG_SHIFT;

    return (u_val & VIDC_SIGN_BIT) ? (BIAS - t) : (t - BIAS);
}

#if CONFIG_HARDCODED_TABLES
#define pcm_alaw_tableinit()
#define pcm_ulaw_tableinit()
#define pcm_vidc_tableinit()
#include "libavcodec/pcm_tables.h"
#else
/* 16384 entries per table */
static uint8_t linear_to_alaw[16384];
static uint8_t linear_to_ulaw[16384];
static uint8_t linear_to_vidc[16384];

static av_cold void build_xlaw_table(uint8_t *linear_to_xlaw,
                             int (*xlaw2linear)(unsigned char),
                             int mask)
{
    int i, j, v, v1, v2;

    j = 1;
    linear_to_xlaw[8192] = mask;
    for(i=0;i<127;i++) {
        v1 = xlaw2linear(i ^ mask);
        v2 = xlaw2linear((i + 1) ^ mask);
        v = (v1 + v2 + 4) >> 3;
        for(;j<v;j+=1) {
            linear_to_xlaw[8192 - j] = (i ^ (mask ^ 0x80));
            linear_to_xlaw[8192 + j] = (i ^ mask);
        }
    }
    for(;j<8192;j++) {
        linear_to_xlaw[8192 - j] = (127 ^ (mask ^ 0x80));
        linear_to_xlaw[8192 + j] = (127 ^ mask);
    }
    linear_to_xlaw[0] = linear_to_xlaw[1];
}

static void pcm_alaw_tableinit(void)
{
    build_xlaw_table(linear_to_alaw, alaw2linear, 0xd5);
}

static void pcm_ulaw_tableinit(void)
{
    build_xlaw_table(linear_to_ulaw, ulaw2linear, 0xff);
}

static void pcm_vidc_tableinit(void)
{
    build_xlaw_table(linear_to_vidc, vidc2linear, 0xff);
}
#endif /* CONFIG_HARDCODED_TABLES */

#endif /* AVCODEC_PCM_TABLEGEN_H */
