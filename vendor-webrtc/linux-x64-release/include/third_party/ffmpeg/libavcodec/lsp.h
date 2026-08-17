/*
 * LSP computing for ACELP-based codecs
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

#ifndef AVCODEC_LSP_H
#define AVCODEC_LSP_H

#include <stdint.h>

/**
  (I.F) means fixed-point value with F fractional and I integer bits
*/

/**
 * @brief ensure a minimum distance between LSFs
 * @param[in,out] lsfq LSF to check and adjust
 * @param lsfq_min_distance minimum distance between LSFs
 * @param lsfq_min minimum allowed LSF value
 * @param lsfq_max maximum allowed LSF value
 * @param lp_order LP filter order
 */
void ff_acelp_reorder_lsf(int16_t* lsfq, int lsfq_min_distance, int lsfq_min, int lsfq_max, int lp_order);

/**
 * Adjust the quantized LSFs so they are increasing and not too close.
 *
 * This step is not mentioned in the AMR spec but is in the reference C decoder.
 * Omitting this step creates audible distortion on the sinusoidal sweep
 * test vectors in 3GPP TS 26.074.
 *
 * @param[in,out] lsf    LSFs in Hertz
 * @param min_spacing    minimum distance between two consecutive lsf values
 * @param size           size of the lsf vector
 */
void ff_set_min_dist_lsf(float *lsf, double min_spacing, int size);

/**
 * @brief Convert LSF to LSP
 * @param[out] lsp LSP coefficients (-0x8000 <= (0.15) < 0x8000)
 * @param lsf normalized LSF coefficients (0 <= (2.13) < 0x2000 * PI)
 * @param lp_order LP filter order
 *
 * @remark It is safe to pass the same array into the lsf and lsp parameters.
 */
void ff_acelp_lsf2lsp(int16_t *lsp, const int16_t *lsf, int lp_order);

/**
 * Floating point version of ff_acelp_lsf2lsp()
 */
void ff_acelp_lsf2lspd(double *lsp, const float *lsf, int lp_order);

/**
 * @brief LSP to LP conversion (3.2.6 of G.729)
 * @param[out] lp decoded LP coefficients (-0x8000 <= (3.12) < 0x8000)
 * @param lsp LSP coefficients (-0x8000 <= (0.15) < 0x8000)
 * @param lp_half_order LP filter order, divided by 2
 */
void ff_acelp_lsp2lpc(int16_t* lp, const int16_t* lsp, int lp_half_order);

/**
 * LSP to LP conversion (5.2.4 of AMR-WB)
 */
void ff_amrwb_lsp2lpc(const double *lsp, float *lp, int lp_order);

/**
 * @brief Interpolate LSP for the first subframe and convert LSP -> LP for both subframes (3.2.5 and 3.2.6 of G.729)
 * @param[out] lp_1st decoded LP coefficients for first subframe  (-0x8000 <= (3.12) < 0x8000)
 * @param[out] lp_2nd decoded LP coefficients for second subframe (-0x8000 <= (3.12) < 0x8000)
 * @param lsp_2nd LSP coefficients of the second subframe (-0x8000 <= (0.15) < 0x8000)
 * @param lsp_prev LSP coefficients from the second subframe of the previous frame (-0x8000 <= (0.15) < 0x8000)
 * @param lp_order LP filter order
 */
void ff_acelp_lp_decode(int16_t* lp_1st, int16_t* lp_2nd, const int16_t* lsp_2nd, const int16_t* lsp_prev, int lp_order);


#define MAX_LP_HALF_ORDER 10
#define MAX_LP_ORDER      (2*MAX_LP_HALF_ORDER)

/**
 * Reconstruct LPC coefficients from the line spectral pair frequencies.
 *
 * @param lsp line spectral pairs in cosine domain
 * @param lpc linear predictive coding coefficients
 * @param lp_half_order half the number of the amount of LPCs to be
 *        reconstructed, need to be smaller or equal to MAX_LP_HALF_ORDER
 *
 * @note buffers should have a minimum size of 2*lp_half_order elements.
 *
 * TIA/EIA/IS-733 2.4.3.3.5
 */
void ff_acelp_lspd2lpc(const double *lsp, float *lpc, int lp_half_order);

/**
 * Sort values in ascending order.
 *
 * @note O(n) if data already sorted, O(n^2) - otherwise
 */
void ff_sort_nearly_sorted_floats(float *vals, int len);

#endif /* AVCODEC_LSP_H */
