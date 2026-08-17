/*
 * Copyright 2024 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_CORRUPTION_DETECTION_CORRUPTION_CLASSIFIER_H_
#define VIDEO_CORRUPTION_DETECTION_CORRUPTION_CLASSIFIER_H_

#include <variant>

#include "api/array_view.h"
#include "video/corruption_detection/halton_frame_sampler.h"

namespace webrtc {

// Based on the given filtered samples to `CalculateCorruptionProbability` this
// class calculates a probability to indicate whether the frame is corrupted.
// The classification is done either by scaling the loss to the interval of [0,
// 1] using a simple `scale_factor` or by applying a logistic function to the
// loss. The logistic function is constructed based on `growth_rate` and
// `midpoint`, to the score between the original and the compressed frames'
// samples. This score is calculated using `GetScore`.
//
// TODO: bugs.webrtc.org/358039777 - Remove one of the constructors based on
// which mapping function works best in practice.
class CorruptionClassifier {
 public:
  // Calculates the corruption probability using a simple scale factor.
  explicit CorruptionClassifier(float scale_factor);
  // Calculates the corruption probability using a logistic function.
  CorruptionClassifier(float growth_rate, float midpoint);
  ~CorruptionClassifier() = default;

  // This function calculates and returns the probability (in the interval [0,
  // 1] that a frame is corrupted. The probability is determined either by
  // scaling the loss to the interval of [0, 1] using a simple `scale_factor`
  // or by applying a logistic function to the loss. The method is chosen
  // depending on the used constructor.
  double CalculateCorruptionProbability(
      ArrayView<const FilteredSample> filtered_original_samples,
      ArrayView<const FilteredSample> filtered_compressed_samples,
      int luma_threshold,
      int chroma_threshold) const;

 private:
  struct ScalarConfig {
    float scale_factor;
  };

  // Logistic function parameters. See
  // https://en.wikipedia.org/wiki/Logistic_function.
  struct LogisticFunctionConfig {
    float growth_rate;
    float midpoint;
  };

  // Returns the non-normalized score between the original and the compressed
  // frames' samples.
  double GetScore(ArrayView<const FilteredSample> filtered_original_samples,
                  ArrayView<const FilteredSample> filtered_compressed_samples,
                  int luma_threshold,
                  int chroma_threshold) const;

  const std::variant<ScalarConfig, LogisticFunctionConfig> config_;
};

}  // namespace webrtc

#endif  // VIDEO_CORRUPTION_DETECTION_CORRUPTION_CLASSIFIER_H_
