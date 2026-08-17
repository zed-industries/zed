/*
 * various filters for ACELP-based codecs
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

#ifndef AVCODEC_ACELP_FILTERS_H
#define AVCODEC_ACELP_FILTERS_H

#include <stdint.h>

typedef struct ACELPFContext {
    /**
    * Floating point version of ff_acelp_interpolate()
    */
    void (*acelp_interpolatef)(float *out, const float *in,
                            const float *filter_coeffs, int precision,
                            int frac_pos, int filter_length, int length);

    /**
     * Apply an order 2 rational transfer function in-place.
     *
     * @param out output buffer for filtered speech samples
     * @param in input buffer containing speech data (may be the same as out)
     * @param zero_coeffs z^-1 and z^-2 coefficients of the numerator
     * @param pole_coeffs z^-1 and z^-2 coefficients of the denominator
     * @param gain scale factor for final output
     * @param mem intermediate values used by filter (should be 0 initially)
     * @param n number of samples (should be a multiple of eight)
     */
    void (*acelp_apply_order_2_transfer_function)(float *out, const float *in,
                                                  const float zero_coeffs[2],
                                                  const float pole_coeffs[2],
                                                  float gain,
                                                  float mem[2], int n);

}ACELPFContext;

/**
 * Initialize ACELPFContext.
 */
void ff_acelp_filter_init(ACELPFContext *c);
void ff_acelp_filter_init_mips(ACELPFContext *c);

/**
 * low-pass Finite Impulse Response filter coefficients.
 *
 * Hamming windowed sinc filter with cutoff freq 3/40 of the sampling freq,
 * the coefficients are scaled by 2^15.
 * This array only contains the right half of the filter.
 * This filter is likely identical to the one used in G.729, though this
 * could not be determined from the original comments with certainty.
 */
extern const int16_t ff_acelp_interp_filter[61];

/**
 * Generic FIR interpolation routine.
 * @param[out] out buffer for interpolated data
 * @param in input data
 * @param filter_coeffs interpolation filter coefficients (0.15)
 * @param precision sub sample factor, that is the precision of the position
 * @param frac_pos fractional part of position [0..precision-1]
 * @param filter_length filter length
 * @param length length of output
 *
 * filter_coeffs contains coefficients of the right half of the symmetric
 * interpolation filter. filter_coeffs[0] should the central (unpaired) coefficient.
 * See ff_acelp_interp_filter for an example.
 */
void ff_acelp_interpolate(int16_t* out, const int16_t* in,
                          const int16_t* filter_coeffs, int precision,
                          int frac_pos, int filter_length, int length);

/**
 * Floating point version of ff_acelp_interpolate()
 */
void ff_acelp_interpolatef(float *out, const float *in,
                           const float *filter_coeffs, int precision,
                           int frac_pos, int filter_length, int length);


/**
 * high-pass filtering and upscaling (4.2.5 of G.729).
 * @param[out]     out   output buffer for filtered speech data
 * @param[in,out]  hpf_f past filtered data from previous (2 items long)
 *                       frames (-0x20000000 <= (14.13) < 0x20000000)
 * @param in speech data to process
 * @param length input data size
 *
 * out[i] = 0.93980581 * in[i] - 1.8795834 * in[i-1] + 0.93980581 * in[i-2] +
 *          1.9330735 * out[i-1] - 0.93589199 * out[i-2]
 *
 * The filter has a cut-off frequency of 1/80 of the sampling freq
 *
 * @note Two items before the top of the in buffer must contain two items from the
 *       tail of the previous subframe.
 *
 * @remark It is safe to pass the same array in in and out parameters.
 *
 * @remark AMR uses mostly the same filter (cut-off frequency 60Hz, same formula,
 *         but constants differs in 5th sign after comma). Fortunately in
 *         fixed-point all coefficients are the same as in G.729. Thus this
 *         routine can be used for the fixed-point AMR decoder, too.
 */
void ff_acelp_high_pass_filter(int16_t* out, int hpf_f[2],
                               const int16_t* in, int length);

/**
 * Apply an order 2 rational transfer function in-place.
 *
 * @param out output buffer for filtered speech samples
 * @param in input buffer containing speech data (may be the same as out)
 * @param zero_coeffs z^-1 and z^-2 coefficients of the numerator
 * @param pole_coeffs z^-1 and z^-2 coefficients of the denominator
 * @param gain scale factor for final output
 * @param mem intermediate values used by filter (should be 0 initially)
 * @param n number of samples
 */
void ff_acelp_apply_order_2_transfer_function(float *out, const float *in,
                                              const float zero_coeffs[2],
                                              const float pole_coeffs[2],
                                              float gain,
                                              float mem[2], int n);

/**
 * Apply tilt compensation filter, 1 - tilt * z-1.
 *
 * @param mem pointer to the filter's state (one single float)
 * @param tilt tilt factor
 * @param samples array where the filter is applied
 * @param size the size of the samples array
 */
void ff_tilt_compensation(float *mem, float tilt, float *samples, int size);


#endif /* AVCODEC_ACELP_FILTERS_H */
