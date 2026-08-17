/*
 * LPC utility code
 * Copyright (c) 2006  Justin Ruggles <justin.ruggles@gmail.com>
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

#ifndef AVCODEC_LPC_H
#define AVCODEC_LPC_H

#include <stdint.h>
#include <stddef.h>
#include "libavutil/lls.h"

#define ORDER_METHOD_EST     0
#define ORDER_METHOD_2LEVEL  1
#define ORDER_METHOD_4LEVEL  2
#define ORDER_METHOD_8LEVEL  3
#define ORDER_METHOD_SEARCH  4
#define ORDER_METHOD_LOG     5

#define MIN_LPC_ORDER        1
#define MAX_LPC_ORDER       32

/**
 * LPC analysis type
 */
enum FFLPCType {
    FF_LPC_TYPE_DEFAULT     = -1, ///< use the codec default LPC type
    FF_LPC_TYPE_NONE        =  0, ///< do not use LPC prediction or use all zero coefficients
    FF_LPC_TYPE_FIXED       =  1, ///< fixed LPC coefficients
    FF_LPC_TYPE_LEVINSON    =  2, ///< Levinson-Durbin recursion
    FF_LPC_TYPE_CHOLESKY    =  3, ///< Cholesky factorization
    FF_LPC_TYPE_NB              , ///< Not part of ABI
};

typedef struct LPCContext {
    int blocksize;
    int max_order;
    enum FFLPCType lpc_type;
    double *windowed_buffer;
    double *windowed_samples;

    /**
     * Apply a Welch window to an array of input samples.
     * The output samples have the same scale as the input, but are in double
     * sample format.
     * @param data    input samples
     * @param len     number of input samples
     * @param w_data  output samples
     */
    void (*lpc_apply_welch_window)(const int32_t *data, ptrdiff_t len,
                                   double *w_data);
    /**
     * Perform autocorrelation on input samples with delay of 0 to lag.
     * @param data  input samples.
     *              constraints: no alignment needed, but must have at
     *              least lag*sizeof(double) valid bytes preceding it, and
     *              size must be at least (len+1)*sizeof(double) if data is
     *              16-byte aligned or (len+2)*sizeof(double) if data is
     *              unaligned.
     * @param len   number of input samples to process
     * @param lag   maximum delay to calculate
     * @param autoc output autocorrelation coefficients.
     *              constraints: array size must be at least lag+1.
     */
    void (*lpc_compute_autocorr)(const double *data, ptrdiff_t len, int lag,
                                 double *autoc);

    // TODO: these should be allocated to reduce ABI compatibility issues
    LLSModel lls_models[2];
} LPCContext;


/**
 * Calculate LPC coefficients for multiple orders
 */
int ff_lpc_calc_coefs(LPCContext *s,
                      const int32_t *samples, int blocksize, int min_order,
                      int max_order, int precision,
                      int32_t coefs[][MAX_LPC_ORDER], int *shift,
                      enum FFLPCType lpc_type, int lpc_passes,
                      int omethod, int min_shift, int max_shift, int zero_shift);

int ff_lpc_calc_ref_coefs(LPCContext *s,
                          const int32_t *samples, int order, double *ref);

double ff_lpc_calc_ref_coefs_f(LPCContext *s, const float *samples, int len,
                               int order, double *ref);

/**
 * Initialize LPCContext.
 */
int ff_lpc_init(LPCContext *s, int blocksize, int max_order,
                enum FFLPCType lpc_type);
void ff_lpc_init_riscv(LPCContext *s);
void ff_lpc_init_x86(LPCContext *s);

/**
 * Uninitialize LPCContext.
 */
void ff_lpc_end(LPCContext *s);

#endif /* AVCODEC_LPC_H */
