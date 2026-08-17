/*
 * various filters for CELP-based codecs
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

#ifndef AVCODEC_CELP_FILTERS_H
#define AVCODEC_CELP_FILTERS_H

#include <stdint.h>

typedef struct CELPFContext {
    /**
     * LP synthesis filter.
     * @param[out] out pointer to output buffer
     *        - the array out[-filter_length, -1] must
     *        contain the previous result of this filter
     * @param filter_coeffs filter coefficients.
     * @param in input signal
     * @param buffer_length amount of data to process
     * @param filter_length filter length (10 for 10th order LP filter). Must be
     *                      greater than 4 and even.
     *
     * @note Output buffer must contain filter_length samples of past
     *       speech data before pointer.
     *
     * Routine applies 1/A(z) filter to given speech data.
     */
    void (*celp_lp_synthesis_filterf)(float *out, const float *filter_coeffs,
                                      const float *in, int buffer_length,
                                      int filter_length);

    /**
     * LP zero synthesis filter.
     * @param[out] out pointer to output buffer
     * @param filter_coeffs filter coefficients.
     * @param in input signal
     *        - the array in[-filter_length, -1] must
     *        contain the previous input of this filter
     * @param buffer_length amount of data to process (should be a multiple of eight)
     * @param filter_length filter length (10 for 10th order LP filter;
     *                                      should be a multiple of two)
     *
     * @note Output buffer must contain filter_length samples of past
     *       speech data before pointer.
     *
     * Routine applies A(z) filter to given speech data.
     */
    void (*celp_lp_zero_synthesis_filterf)(float *out, const float *filter_coeffs,
                                           const float *in, int buffer_length,
                                           int filter_length);

}CELPFContext;

/**
 * Initialize CELPFContext.
 */
void ff_celp_filter_init(CELPFContext *c);
void ff_celp_filter_init_mips(CELPFContext *c);

/**
 * Circularly convolve fixed vector with a phase dispersion impulse
 *        response filter (D.6.2 of G.729 and 6.1.5 of AMR).
 * @param fc_out vector with filter applied
 * @param fc_in source vector
 * @param filter phase filter coefficients
 *
 *  fc_out[n] = sum(i,0,len-1){ fc_in[i] * filter[(len + n - i)%len] }
 *
 * @note fc_in and fc_out should not overlap!
 */
void ff_celp_convolve_circ(int16_t *fc_out, const int16_t *fc_in,
                           const int16_t *filter, int len);

/**
 * Add an array to a rotated array.
 *
 * out[k] = in[k] + fac * lagged[k-lag] with wrap-around
 *
 * @param out result vector
 * @param in samples to be added unfiltered
 * @param lagged samples to be rotated, multiplied and added
 * @param lag lagged vector delay in the range [0, n]
 * @param fac scalefactor for lagged samples
 * @param n number of samples
 */
void ff_celp_circ_addf(float *out, const float *in,
                       const float *lagged, int lag, float fac, int n);

/**
 * LP synthesis filter.
 * @param[out] out pointer to output buffer
 * @param filter_coeffs filter coefficients (-0x8000 <= (3.12) < 0x8000)
 * @param in input signal
 * @param buffer_length amount of data to process
 * @param filter_length filter length (10 for 10th order LP filter)
 * @param stop_on_overflow   1 - return immediately if overflow occurs
 *                           0 - ignore overflows
 * @param shift the result is shifted right by this value
 * @param rounder the amount to add for rounding (usually 0x800 or 0xfff)
 *
 * @return 1 if overflow occurred, 0 - otherwise
 *
 * @note Output buffer must contain filter_length samples of past
 *       speech data before pointer.
 *
 * Routine applies 1/A(z) filter to given speech data.
 */
int ff_celp_lp_synthesis_filter(int16_t *out, const int16_t *filter_coeffs,
                                const int16_t *in, int buffer_length,
                                int filter_length, int stop_on_overflow,
                                int shift, int rounder);

/**
 * LP synthesis filter.
 * @param[out] out pointer to output buffer
 *        - the array out[-filter_length, -1] must
 *        contain the previous result of this filter
 * @param filter_coeffs filter coefficients.
 * @param in input signal
 * @param buffer_length amount of data to process
 * @param filter_length filter length (10 for 10th order LP filter). Must be
 *                      greater than 4 and even.
 *
 * @note Output buffer must contain filter_length samples of past
 *       speech data before pointer.
 *
 * Routine applies 1/A(z) filter to given speech data.
 */
void ff_celp_lp_synthesis_filterf(float *out, const float *filter_coeffs,
                                  const float *in, int buffer_length,
                                  int filter_length);

/**
 * LP zero synthesis filter.
 * @param[out] out pointer to output buffer
 * @param filter_coeffs filter coefficients.
 * @param in input signal
 *        - the array in[-filter_length, -1] must
 *        contain the previous input of this filter
 * @param buffer_length amount of data to process
 * @param filter_length filter length (10 for 10th order LP filter)
 *
 * @note Output buffer must contain filter_length samples of past
 *       speech data before pointer.
 *
 * Routine applies A(z) filter to given speech data.
 */
void ff_celp_lp_zero_synthesis_filterf(float *out, const float *filter_coeffs,
                                       const float *in, int buffer_length,
                                       int filter_length);

#endif /* AVCODEC_CELP_FILTERS_H */
