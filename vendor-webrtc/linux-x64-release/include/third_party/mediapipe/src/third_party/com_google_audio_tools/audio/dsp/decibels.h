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

// Conversions between linear and decibel scale.

#ifndef AUDIO_DSP_DECIBELS_H_
#define AUDIO_DSP_DECIBELS_H_

#include <cmath>

#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

template <typename T>
using Promoted = decltype(std::log(T()));

template <typename T>
inline Promoted<T> AmplitudeRatioToDecibels(T linear) {
  return 20 * std::log10(linear);
}

template <typename InputEigenType, typename OutputEigenType>
void AmplitudeRatioToDecibels(const InputEigenType& linear,
                              OutputEigenType* db) {
  constexpr Promoted<typename InputEigenType::Scalar> k20_log10 = 20 / M_LN10;
  *db = k20_log10 * linear.array().log();
}

template <typename T>
Promoted<T> DecibelsToAmplitudeRatio(T decibels) {
  constexpr Promoted<T> klog10_20 = M_LN10 / 20;
  return std::exp(klog10_20 * decibels);
}

template <typename InputEigenType, typename OutputEigenType>
void DecibelsToAmplitudeRatio(const InputEigenType& db,
                              OutputEigenType* linear) {
  constexpr Promoted<typename InputEigenType::Scalar> klog10_20 = M_LN10 / 20;
  *linear = (klog10_20 * db.array()).exp();
}

template <typename T>
inline Promoted<T> PowerRatioToDecibels(T linear) {
  return 10 * std::log10(linear);
}

template <typename InputEigenType, typename OutputEigenType>
void PowerRatioToDecibels(const InputEigenType& linear,
                          OutputEigenType* db) {
  constexpr Promoted<typename InputEigenType::Scalar> k10_log10 = 10 / M_LN10;
  *db = k10_log10 * linear.array().log();
}

template <typename T>
inline Promoted<T> DecibelsToPowerRatio(T decibels) {
  constexpr Promoted<T> klog10_10 = M_LN10 / 10;
  return std::exp(klog10_10 * decibels);
}

template <typename InputEigenType, typename OutputEigenType>
void DecibelsToPowerRatio(const InputEigenType& db,
                          OutputEigenType* linear) {
  constexpr Promoted<typename InputEigenType::Scalar> klog10_10 = M_LN10 / 10;
  *linear = (klog10_10 * db.array()).exp();
}

}  // namespace audio_dsp

#endif  // AUDIO_DSP_DECIBELS_H_
