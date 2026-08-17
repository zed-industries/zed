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

// This file contains methods of converting parameters of continuous time to
// discrete time representations.
// The most common discretization methods, and the ones featured here are
// the bilinear transform and the matched Z-transform method.
// https://en.wikipedia.org/wiki/Matched_Z-transform_method
// https://en.wikipedia.org/wiki/Bilinear_transform.
//
// There are several reasons to choose one discretization method over the other:
// Matched Z-transform method:
//   A simple mapping that does not experience frequency warping near the
//   Nyquist frequency. However, for mapping high frequencies, this transform
//   aliases. For mapping frequencies near DC, this will work well.
//
// Bilinear Transform:
//   A mapping from the continuous (infinite) frequency (f_c) axis into the
//   periodic discrete time frequency (f_d) axis. This mapping causes the high
//   frequencies to compress as they approach the Nyqist frequency. The
//   benefit of this mapping is that because f_c = infinity is mapped to
//   f_d = Nyquist frequency, there is no aliasing. Frequency pre-warping can
//   be done to ensure that a single frequency, typically DC, is mapped exactly
//   (f_c = f_d). See BilinearPrewarp.

#ifndef AUDIO_LINEAR_FILTERS_DISCRETIZATION_H_
#define AUDIO_LINEAR_FILTERS_DISCRETIZATION_H_

#include <cmath>
#include <vector>

#include "audio/linear_filters/biquad_filter_coefficients.h"
#include "audio/linear_filters/filter_poles_and_zeros.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {
// Compute the coefficient, a, for a recursive filter of the form
// y[n] = a * x[n] + (1 - a) y[n - 1] with time constant time_constant_seconds
// using the pole-matching method. This corresponds to the commonly used update
// rule, y_state += a * (x[n] - y_state), which reduces the number of
// multiplies, and doesn't require storing the output samples.
// https://en.wikipedia.org/wiki/Exponential_smoothing#Time_Constant
inline double FirstOrderCoefficientFromTimeConstant(
    double time_constant_seconds, double sample_rate_hz) {
  return 1 - std::exp(-1.0 / (time_constant_seconds * sample_rate_hz));
}

// Matched Z transform.
inline double MapFrequencyToZPlane(double corner_hz, double sample_rate_hz) {
  // Map a continuous time root to the Z-domain.
  return std::exp(-2.0 * M_PI * corner_hz / sample_rate_hz);
}

// To account for the frequency warping of the bilinear transform, prewarp
// the input frequency. The result has units of rad/s.
// https://en.wikipedia.org/wiki/Bilinear_transform#Frequency_warping
inline double BilinearPrewarp(double frequency_hz, double sample_rate_hz) {
  return 2 * sample_rate_hz * std::tan(M_PI * frequency_hz / sample_rate_hz);
}

// Create coefficients from a continuous-time filter's quadratic numerator
// and denominator s-domain polynomials, using the bilinear transform
// pre-warped such that the continuous and discrete-time transfer functions
// are matched around match_frequency_hz.
// s_numerator and s_denominator must have exactly three elements. (i.e. they
// must correspond to a second order filter).
BiquadFilterCoefficients BilinearTransform(
    const std::vector<double>& s_numerator,
    const std::vector<double>& s_denominator,
    double sample_rate_hz,
    double match_frequency_hz);

// It is common to have match_frequency_hz be DC.
BiquadFilterCoefficients BilinearTransform(
    const std::vector<double>& s_numerator,
    const std::vector<double>& s_denominator,
    double sample_rate_hz);

// Discretize an analog filter represented as poles and zeros using the
// bilinear transform.
FilterPolesAndZeros BilinearTransform(const FilterPolesAndZeros& analog_filter,
                                      double sample_rate_hz,
                                      double match_frequency_hz);

// It is common to have match_frequency_hz be DC.
FilterPolesAndZeros BilinearTransform(const FilterPolesAndZeros& analog_filter,
                                      double sample_rate_hz);
}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_DISCRETIZATION_H_
