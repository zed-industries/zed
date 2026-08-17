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

// Compute coefficients for two-way digital crossovers.

#ifndef AUDIO_LINEAR_FILTERS_CROSSOVER_H_
#define AUDIO_LINEAR_FILTERS_CROSSOVER_H_

#include "audio/linear_filters/biquad_filter_coefficients.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {

// Variants are discussed here:
// https://en.wikipedia.org/wiki/Audio_crossover
enum CrossoverType {
  // Butterworth crossovers have a 3dB gain bump at the crossover frequency.
  kButterworth,
  // kLinkwitzRiley crossovers are flat across the entire spectrum. Lots of
  // great details about Linkwitz-Riley crossovers can be found here:
  // http://www.rane.com/note160.html.
  kLinkwitzRiley,
};

// An order N crossover provides roughly -6 * N dB/oct rolloff above and below
// the crossover frequency (lowpass and highpass taps, respectively).  It is
// typical to use orders of 1, 2, or 4, but higher order filters are supported.
// Going above 8th order is unnecessary. For every four orders, the high and
// low signals go out of phase an additional 360 degrees. There is internal
// compensation for the phase for filters with 4k + 2 stages so that the
// outputs appear in phase.
//
// NOTE: if type = kLinkwitzRiley, order must be even.
class CrossoverFilterDesign {
 public:
  CrossoverFilterDesign(CrossoverType type, int order,
                        float crossover_frequency_hz, float sample_rate_hz);

  const BiquadFilterCascadeCoefficients& GetLowpassCoefficients() const {
    return lowpass_;
  }

  const BiquadFilterCascadeCoefficients& GetHighpassCoefficients() const {
    return highpass_;
  }

 private:
  BiquadFilterCascadeCoefficients lowpass_;
  BiquadFilterCascadeCoefficients highpass_;
};

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_CROSSOVER_H_
