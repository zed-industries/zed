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

// Parameters for a single stage of the filter.

#ifndef AUDIO_LINEAR_FILTERS_EQUALIZER_FILTER_PARAMS_H_
#define AUDIO_LINEAR_FILTERS_EQUALIZER_FILTER_PARAMS_H_

#include <sstream>
#include <string>

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {

struct EqualizerFilterParams {
  enum Type {
    kLowpass,
    kLowShelf,
    kPeak,
    kHighShelf,
    kHighpass,
  } type;

  constexpr explicit EqualizerFilterParams(Type type)
    : type(type),
      frequency_hz(0.0f),
      quality_factor(0.707f),
      gain_db(0.0f) {}

  constexpr EqualizerFilterParams(Type type, float freq, float Q, float gain)
    : type(type),
      frequency_hz(freq),
      quality_factor(Q),
      gain_db(gain) {}

  std::string ToString() const {
    std::stringstream info;
    switch (type) {
      case kLowpass:
        info << "lowpass: {";
        break;
      case kLowShelf:
        info << "low_shelf: {";
        break;
      case EqualizerFilterParams::kPeak:
        info << "peak: {";
        break;
      case EqualizerFilterParams::kHighShelf:
        info << "high_shelf: {";
        break;
      case EqualizerFilterParams::kHighpass:
        info << "high_pass: {";
        break;
    }
    info << "frequency_hz: " << frequency_hz
         << " quality_factor: " << quality_factor << " gain_db: " << gain_db
         << "} ";
    return info.str();
  }

  // For a highpass/lowpass/shelf filters, this is the transition frequency. For
  // the parametric peak filters, this is the frequency of the peak (or valley).
  float frequency_hz;

  // Controls the width of the peak, or more generally, the amount of
  // resonance around the transitions.
  float quality_factor;

  // The amount of boost or attenuation in decibels. This is unused for
  // lowpass and highpass filters.
  float gain_db;
};

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_EQUALIZER_FILTER_PARAMS_H_
