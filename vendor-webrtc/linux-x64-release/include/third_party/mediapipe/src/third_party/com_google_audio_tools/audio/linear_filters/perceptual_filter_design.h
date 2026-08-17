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

// Filters that roughly model the frequency sensitivity of the auditory system.
//
// Weightings A, B, C, and D:
// These filters have gain of 0dB at 1kHz (the exception is the K-weighting,
// whose gain is consistent with the ITU standard. At normal audio sample rates,
// the response matches that shown in the picture at the Wikipedia page. For
// low sample rates (8k, for example), there is no high frequency rolloff
// because those frequencies are still audible.
// https://en.wikipedia.org/wiki/A-weighting
//
// RLB Weighting:
// Gilbert A. Soulodre. "Evaluation of Objective Loudness Meters".
// 116th AES Convention, 2004.

//
// K Weighting:
// https://www.itu.int/dms_pubrec/itu-r/rec/bs/R-REC-BS.1770-2-201103-S!!PDF-E.pdf

#ifndef AUDIO_LINEAR_FILTERS_PERCEPTUAL_FILTER_DESIGN_H_
#define AUDIO_LINEAR_FILTERS_PERCEPTUAL_FILTER_DESIGN_H_

#include "audio/linear_filters/biquad_filter_coefficients.h"
#include "audio/linear_filters/discretization.h"
#include "audio/linear_filters/filter_poles_and_zeros.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {

enum PerceptualWeightingType {
  kAWeighting,
  kBWeighting,
  kCWeighting,
  kDWeighting,
  kRlbWeighting,
  kKWeighting,
};

// Get the continuous-time poles and zeros for a perceptual loudness weighting
// filter.
FilterPolesAndZeros PerceptualLoudnessFilterPolesAndZeros(
    PerceptualWeightingType weighting, float sample_rate_hz);

// Get coefficients for a BiquadFilterCascade that implements a perceptual
// loudness weighting filter.
BiquadFilterCascadeCoefficients PerceptualLoudnessFilterCoefficients(
    PerceptualWeightingType weighting, float sample_rate_hz);

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_PERCEPTUAL_FILTER_DESIGN_H_
