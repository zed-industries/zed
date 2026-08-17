/*
 * G.729, G729 Annex D postfilter
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
#ifndef AVCODEC_G729POSTFILTER_H
#define AVCODEC_G729POSTFILTER_H

#include <stdint.h>
#include "acelp_pitch_delay.h"
#include "audiodsp.h"

/**
 * tilt compensation factor (G.729, k1>0)
 * 0.2 in Q15
 */
#define G729_TILT_FACTOR_PLUS       6554

/**
 * tilt compensation factor (G.729, k1<0)
 * 0.9 in Q15
 */
#define G729_TILT_FACTOR_MINUS     29491

/* 4.2.2 */
#define FORMANT_PP_FACTOR_NUM  18022             //0.55 in Q15
#define FORMANT_PP_FACTOR_DEN  22938             //0.70 in Q15

/**
 * gain adjustment factor (G.729, 4.2.4)
 * 0.9875 in Q15
 */
#define G729_AGC_FACTOR            32358
#define G729_AGC_FAC1 (32768-G729_AGC_FACTOR)

/**
 * 1.0 / (1.0 + 0.5) in Q15
 * where 0.5 is the minimum value of
 * weight factor, controlling amount of long-term postfiltering
 */
#define MIN_LT_FILT_FACTOR_A       21845

/**
 * Short interpolation filter length
 */
#define SHORT_INT_FILT_LEN         2

/**
 * Long interpolation filter length
 */
#define LONG_INT_FILT_LEN          8

/**
 * Number of analyzed fractional pitch delays in second stage of long-term
 * postfilter
 */
#define ANALYZED_FRAC_DELAYS       7

/**
 * Amount of past residual signal data stored in buffer
 */
#define RES_PREV_DATA_SIZE (PITCH_DELAY_MAX + LONG_INT_FILT_LEN + 1)

/**
 * \brief Signal postfiltering (4.2)
 * \param dsp initialized DSP context
 * \param[in,out] ht_prev_data  (Q12) pointer to variable receiving tilt
 *                     compensation filter data from previous subframe
 * \param[in,out] voicing  (Q0) pointer to variable receiving voicing decision
 * \param lp_filter_coeffs (Q12) LP filter coefficients
 * \param pitch_delay_int integer part of the pitch delay
 * \param[in,out] residual  (Q0) residual signal buffer (used in long-term postfilter)
 * \param[in,out] res_filter_data  (Q0) speech data of previous subframe
 * \param[in,out] pos_filter_data  (Q0) previous speech data for short-term postfilter
 * \param[in,out] speech  (Q0) signal buffer
 * \param subframe_size size of subframe
 *
 * Filtering has the following  stages:
 *   Long-term postfilter (4.2.1)
 *   Short-term postfilter (4.2.2).
 *   Tilt-compensation (4.2.3)
 */
void ff_g729_postfilter(AudioDSPContext *adsp, int16_t* ht_prev_data, int* voicing,
                     const int16_t *lp_filter_coeffs, int pitch_delay_int,
                     int16_t* residual, int16_t* res_filter_data,
                     int16_t* pos_filter_data, int16_t *speech,
                     int subframe_size);

/**
 * \brief Adaptive gain control (4.2.4)
 * \param gain_before (Q0) gain of speech before applying postfilters
 * \param gain_after  (Q0) gain of speech after applying postfilters
 * \param[in,out] speech  (Q0) signal buffer
 * \param subframe_size length of subframe
 * \param gain_prev (Q12) previous value of gain coefficient
 *
 * \return (Q12) last value of gain coefficient
 */
int16_t ff_g729_adaptive_gain_control(int gain_before, int gain_after, int16_t *speech,
                                   int subframe_size, int16_t gain_prev);

#endif // AVCODEC_G729POSTFILTER_H
