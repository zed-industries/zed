/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/* Design a 2nd-order Butterworth bandpass filter.
 *
 * NOTE: For a general Butterworth filter design library, see
 * audio/linear_filters/biquad_filter_design.h
 */

#ifndef AUDIO_DSP_PORTABLE_BUTTERWORTH_H_
#define AUDIO_DSP_PORTABLE_BUTTERWORTH_H_

#include "audio/dsp/portable/biquad_filter.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Designs a 2nd-order Butterworth lowpass filter with passband
 * [0, corner_frequency_hz]. The result is a BiquadFilterCoeff, a second-order
 * filter, written to *coeffs. Returns 1 on success, 0 on failure.
 */
int DesignButterworthOrder2Lowpass(double corner_frequency_hz,
                                   double sample_rate_hz,
                                   BiquadFilterCoeffs* coeffs);

/* Designs a 2nd-order Butterworth highpass filter with stopband
 * [0, corner_frequency_hz]. The result is a BiquadFilterCoeff, a second-order
 * filter, written to *coeffs. Returns 1 on success, 0 on failure.
 */
int DesignButterworthOrder2Highpass(double corner_frequency_hz,
                                    double sample_rate_hz,
                                    BiquadFilterCoeffs* coeffs);

/* Designs a 2nd-order Butterworth bandpass filter with passband
 * [low_edge_hz, high_edge_hz]. The result is two BiquadFilterCoeffs, a fourth-
 * order filter, written to coeffs[0] and coeffs[1]. Returns 1 on success, 0 on
 * failure.
 */
int DesignButterworthOrder2Bandpass(double low_edge_hz,
                                    double high_edge_hz,
                                    double sample_rate_hz,
                                    BiquadFilterCoeffs* coeffs);

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif  /* AUDIO_DSP_PORTABLE_BUTTERWORTH_H_ */
