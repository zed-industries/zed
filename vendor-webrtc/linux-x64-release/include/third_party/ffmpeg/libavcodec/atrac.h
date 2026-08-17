/*
 * common functions for the ATRAC family of decoders
 *
 * Copyright (c) 2009-2013 Maxim Poliakovski
 * Copyright (c) 2009 Benjamin Larsson
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
 * ATRAC common header
 */

#ifndef AVCODEC_ATRAC_H
#define AVCODEC_ATRAC_H

/**
 *  Gain control parameters for one subband.
 */
typedef struct AtracGainInfo {
    int   num_points;   ///< number of gain control points
    int   lev_code[7];  ///< level at corresponding control point
    int   loc_code[7];  ///< location of gain control points
} AtracGainInfo;

/**
 *  Gain compensation context structure.
 */
typedef struct AtracGCContext {
    float   gain_tab1[16];  ///< gain compensation level table
    float   gain_tab2[31];  ///< gain compensation interpolation table
    int     id2exp_offset;  ///< offset for converting level index into level exponent
    int     loc_scale;      ///< scale of location code = 2^loc_scale samples
    int     loc_size;       ///< size of location code in samples
} AtracGCContext;

extern float ff_atrac_sf_table[64];

/**
 * Generate common tables.
 */
void ff_atrac_generate_tables(void);

/**
 *  Initialize gain compensation context.
 *
 * @param gctx            pointer to gain compensation context to initialize
 * @param id2exp_offset   offset for converting level index into level exponent
 * @param loc_scale       location size factor
 */
void ff_atrac_init_gain_compensation(AtracGCContext *gctx, int id2exp_offset,
                                     int loc_scale);

/**
 * Apply gain compensation and perform the MDCT overlapping part.
 *
 * @param gctx         pointer to gain compensation context
 * @param in           input buffer
 * @param prev         previous buffer to perform overlap against
 * @param gc_now       gain control information for current frame
 * @param gc_next      gain control information for next frame
 * @param num_samples  number of samples to process
 * @param out          output data goes here
 */
void ff_atrac_gain_compensation(AtracGCContext *gctx, float *in, float *prev,
                                AtracGainInfo *gc_now, AtracGainInfo *gc_next,
                                int num_samples, float *out);

/**
 * Quadrature mirror synthesis filter.
 *
 * @param inlo      lower part of spectrum
 * @param inhi      higher part of spectrum
 * @param nIn       size of spectrum buffer
 * @param pOut      out buffer
 * @param delayBuf  delayBuf buffer
 * @param temp      temp buffer
 */
void ff_atrac_iqmf(float *inlo, float *inhi, unsigned int nIn, float *pOut,
                   float *delayBuf, float *temp);

#endif /* AVCODEC_ATRAC_H */
