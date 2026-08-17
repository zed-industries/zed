/*
 * adaptive and fixed codebook vector operations for ACELP-based codecs
 *
 * Copyright (c) 2008 Vladimir Voroshilov
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

#ifndef AVCODEC_ACELP_VECTORS_H
#define AVCODEC_ACELP_VECTORS_H

#include <stdint.h>

typedef struct ACELPVContext {
    /**
     * float implementation of weighted sum of two vectors.
     * @param[out] out result of addition
     * @param in_a first vector
     * @param in_b second vector
     * @param weight_coeff_a first vector weight coefficient
     * @param weight_coeff_a second vector weight coefficient
     * @param length vectors length (should be a multiple of two)
     *
     * @note It is safe to pass the same buffer for out and in_a or in_b.
     */
    void (*weighted_vector_sumf)(float *out, const float *in_a, const float *in_b,
                                 float weight_coeff_a, float weight_coeff_b,
                                 int length);

}ACELPVContext;

/**
 * Initialize ACELPVContext.
 */
void ff_acelp_vectors_init(ACELPVContext *c);
void ff_acelp_vectors_init_mips(ACELPVContext *c);

/** Sparse representation for the algebraic codebook (fixed) vector */
typedef struct AMRFixed {
    int      n;
    int      x[10];
    float    y[10];
    int      no_repeat_mask;
    int      pitch_lag;
    float    pitch_fac;
} AMRFixed;

/**
 * Track|Pulse|        Positions
 * -------------------------------------------------------------------------
 *  1   | 0   | 0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75
 * -------------------------------------------------------------------------
 *  2   | 1   | 1, 6, 11, 16, 21, 26, 31, 36, 41, 46, 51, 56, 61, 66, 71, 76
 * -------------------------------------------------------------------------
 *  3   | 2   | 2, 7, 12, 17, 22, 27, 32, 37, 42, 47, 52, 57, 62, 67, 72, 77
 * -------------------------------------------------------------------------
 *
 * Table contains only first the pulse indexes.
 *
 * Used in G.729 @@8k, G.729 @@4.4k, AMR @@7.95k, AMR @@7.40k
 */
extern const uint8_t ff_fc_4pulses_8bits_tracks_13[16];

/**
 * Track|Pulse|        Positions
 * -------------------------------------------------------------------------
 *  4   | 3   | 3, 8, 13, 18, 23, 28, 33, 38, 43, 48, 53, 58, 63, 68, 73, 78
 *      |     | 4, 9, 14, 19, 24, 29, 34, 39, 44, 49, 54, 59, 64, 69, 74, 79
 * -------------------------------------------------------------------------
 *
 * @remark Track in the table should be read top-to-bottom, left-to-right.
 *
 * Used in G.729 @@8k, G.729 @@4.4k, AMR @@7.95k, AMR @@7.40k
 */
extern const uint8_t ff_fc_4pulses_8bits_track_4[32];

/**
 * Track|Pulse|        Positions
 * -----------------------------------------
 *  1   | 0   | 1, 6, 11, 16, 21, 26, 31, 36
 *      |     | 3, 8, 13, 18, 23, 28, 33, 38
 * -----------------------------------------
 *
 * @remark Track in the table should be read top-to-bottom, left-to-right.
 *
 * @note (EE) Reference G.729D code also uses gray decoding for each
 *            pulse index before looking up the value in the table.
 *
 * Used in G.729 @@6.4k (with gray coding), AMR @@5.9k (without gray coding)
 */
extern const uint8_t ff_fc_2pulses_9bits_track1_gray[16];

/**
 * Track|Pulse|        Positions
 * -----------------------------------------
 *  2   | 1   | 0, 7, 14, 20, 27, 34,  1, 21
 *      |     | 2, 9, 15, 22, 29, 35,  6, 26
 *      |     | 4,10, 17, 24, 30, 37, 11, 31
 *      |     | 5,12, 19, 25, 32, 39, 16, 36
 * -----------------------------------------
 *
 * @remark Track in the table should be read top-to-bottom, left-to-right.
 *
 * @note (EE.1) This table (from the reference code) does not comply with
 *              the specification.
 *              The specification contains the following table:
 *
 * Track|Pulse|        Positions
 * -----------------------------------------
 *  2   | 1   | 0, 5, 10, 15, 20, 25, 30, 35
 *      |     | 1, 6, 11, 16, 21, 26, 31, 36
 *      |     | 2, 7, 12, 17, 22, 27, 32, 37
 *      |     | 4, 9, 14, 19, 24, 29, 34, 39
 *
 * -----------------------------------------
 *
 * @note (EE.2) Reference G.729D code also uses gray decoding for each
 *              pulse index before looking up the value in the table.
 *
 * Used in G.729 @@6.4k (with gray coding)
 */
extern const uint8_t ff_fc_2pulses_9bits_track2_gray[32];

/**
 * b60 hamming windowed sinc function coefficients
 */
extern const float ff_b60_sinc[61];

/**
 * Table of pow(0.7,n)
 */
extern const float ff_pow_0_7[10];

/**
 * Table of pow(0.75,n)
 */
extern const float ff_pow_0_75[10];

/**
 * Table of pow(0.55,n)
 */
extern const float ff_pow_0_55[10];

/**
 * Decode fixed-codebook vector (3.8 and D.5.8 of G.729, 5.7.1 of AMR).
 * @param[out] fc_v decoded fixed codebook vector (2.13)
 * @param tab1 table used for first pulse_count pulses
 * @param tab2 table used for last pulse
 * @param pulse_indexes fixed codebook indexes
 * @param pulse_signs signs of the excitation pulses (0 bit value
 *                     means negative sign)
 * @param bits number of bits per one pulse index
 * @param pulse_count number of pulses decoded using first table
 * @param bits length of one pulse index in bits
 *
 * Used in G.729 @@8k, G.729 @@4.4k, G.729 @@6.4k, AMR @@7.95k, AMR @@7.40k
 */
void ff_acelp_fc_pulse_per_track(int16_t* fc_v,
                                 const uint8_t *tab1,
                                 const uint8_t *tab2,
                                 int pulse_indexes,
                                 int pulse_signs,
                                 int pulse_count,
                                 int bits);

/**
 * Decode the algebraic codebook index to pulse positions and signs and
 * construct the algebraic codebook vector for MODE_12k2.
 *
 * @note: The positions and signs are explicitly coded in MODE_12k2.
 *
 * @param fixed_index          positions of the ten pulses
 * @param fixed_sparse         pointer to the algebraic codebook vector
 * @param gray_decode          gray decoding table
 * @param half_pulse_count     number of couples of pulses
 * @param bits                 length of one pulse index in bits
 */
void ff_decode_10_pulses_35bits(const int16_t *fixed_index,
                                AMRFixed *fixed_sparse,
                                const uint8_t *gray_decode,
                                int half_pulse_count, int bits);


/**
 * weighted sum of two vectors with rounding.
 * @param[out] out result of addition
 * @param in_a first vector
 * @param in_b second vector
 * @param weight_coeff_a first vector weight coefficient
 * @param weight_coeff_a second vector weight coefficient
 * @param rounder this value will be added to the sum of the two vectors
 * @param shift result will be shifted to right by this value
 * @param length vectors length
 *
 * @note It is safe to pass the same buffer for out and in_a or in_b.
 *
 *  out[i] = (in_a[i]*weight_a + in_b[i]*weight_b + rounder) >> shift
 */
void ff_acelp_weighted_vector_sum(int16_t* out,
                                  const int16_t *in_a,
                                  const int16_t *in_b,
                                  int16_t weight_coeff_a,
                                  int16_t weight_coeff_b,
                                  int16_t rounder,
                                  int shift,
                                  int length);

/**
 * float implementation of weighted sum of two vectors.
 * @param[out] out result of addition
 * @param in_a first vector
 * @param in_b second vector
 * @param weight_coeff_a first vector weight coefficient
 * @param weight_coeff_a second vector weight coefficient
 * @param length vectors length
 *
 * @note It is safe to pass the same buffer for out and in_a or in_b.
 */
void ff_weighted_vector_sumf(float *out, const float *in_a, const float *in_b,
                             float weight_coeff_a, float weight_coeff_b,
                             int length);

/**
 * Adaptive gain control (as used in AMR postfiltering)
 *
 * @param out output buffer for filtered speech data
 * @param in the input speech buffer (may be the same as out)
 * @param speech_energ input energy
 * @param size the input buffer size
 * @param alpha exponential filter factor
 * @param gain_mem a pointer to the filter memory (single float of size)
 */
void ff_adaptive_gain_control(float *out, const float *in, float speech_energ,
                              int size, float alpha, float *gain_mem);

/**
 * Set the sum of squares of a signal by scaling
 *
 * @param out output samples
 * @param in input samples
 * @param sum_of_squares new sum of squares
 * @param n number of samples
 *
 * @note If the input is zero (or its energy underflows), the output is zero.
 *       This is the behavior of AGC in the AMR reference decoder. The QCELP
 *       reference decoder seems to have undefined behavior.
 *
 * TIA/EIA/IS-733 2.4.8.3-2/3/4/5, 2.4.8.6
 * 3GPP TS 26.090 6.1 (6)
 */
void ff_scale_vector_to_given_sum_of_squares(float *out, const float *in,
                                             float sum_of_squares, const int n);

/**
 * Add fixed vector to an array from a sparse representation
 *
 * @param out fixed vector with pitch sharpening
 * @param in sparse fixed vector
 * @param scale number to multiply the fixed vector by
 * @param size the output vector size
 */
void ff_set_fixed_vector(float *out, const AMRFixed *in, float scale, int size);

/**
 * Clear array values set by set_fixed_vector
 *
 * @param out fixed vector to be cleared
 * @param in sparse fixed vector
 * @param size the output vector size
 */
void ff_clear_fixed_vector(float *out, const AMRFixed *in, int size);

#endif /* AVCODEC_ACELP_VECTORS_H */
