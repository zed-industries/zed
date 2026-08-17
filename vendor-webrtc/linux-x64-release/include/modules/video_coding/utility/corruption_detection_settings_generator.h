/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_UTILITY_CORRUPTION_DETECTION_SETTINGS_GENERATOR_H_
#define MODULES_VIDEO_CODING_UTILITY_CORRUPTION_DETECTION_SETTINGS_GENERATOR_H_

#include <optional>
#include <variant>

#include "api/video/corruption_detection_filter_settings.h"

namespace webrtc {

class CorruptionDetectionSettingsGenerator {
 public:
  // A struct with the parameters for a ration function used to determine the
  // standard deviation as function of the qp. It has the form f(qp) =
  // (-numerator_factor * qp) / (denumerator_term + qp) + offset.
  struct RationalFunctionParameters {
    double numerator_factor = 0.0;
    double denumerator_term = 0.0;
    double offset = 0.0;
  };

  // A struct with the parameters for an exponential function used to determine
  // the standard deviation as a function of the qp. It has the form f(qp) =
  // scale * std::exp(exponent_factor * qp - exponent_offset).
  struct ExponentialFunctionParameters {
    double scale = 0.0;
    double exponent_factor = 0.0;
    double exponent_offset = 0.0;
  };

  // Allowed error thresholds for luma (Y) and chroma (UV) channels.
  struct ErrorThresholds {
    int luma = 0;
    int chroma = 0;
  };

  // Settings relating to transient events like key-frames.
  struct TransientParameters {
    // The max qp for the codec in use (e.g. 255 for AV1).
    int max_qp = 0;

    // Temporary increase to error thresholds on keyframes.
    int keyframe_threshold_offset = 0;
    // Temporary increase to std dev on keyframes.
    double keyframe_stddev_offset = 0.0;
    // Fade-out time (in frames) for temporary keyframe offsets.
    int keyframe_offset_duration_frames = 0;

    // How many QP points count as a "large change", or 0 to disable.
    // A large change will trigger the same compensation as a keyframe.
    int large_qp_change_threshold = 0;

    // Don't use a filter kernel smaller than this.
    double std_dev_lower_bound = 0.0;
  };

  CorruptionDetectionSettingsGenerator(
      const RationalFunctionParameters& function_params,
      const ErrorThresholds& default_error_thresholds,
      const TransientParameters& transient_params);
  CorruptionDetectionSettingsGenerator(
      const ExponentialFunctionParameters& function_params,
      const ErrorThresholds& default_error_thresholds,
      const TransientParameters& transient_params);

  CorruptionDetectionFilterSettings OnFrame(bool is_keyframe, int qp);

 private:
  double CalculateStdDev(int qp) const;

  const std::variant<RationalFunctionParameters, ExponentialFunctionParameters>
      function_params_;
  const ErrorThresholds error_thresholds_;
  const TransientParameters transient_params_;

  int frames_since_keyframe_;
  std::optional<int> previous_qp_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_UTILITY_CORRUPTION_DETECTION_SETTINGS_GENERATOR_H_
