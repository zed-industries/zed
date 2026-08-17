/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_COARSE_FILTER_UPDATE_GAIN_H_
#define MODULES_AUDIO_PROCESSING_AEC3_COARSE_FILTER_UPDATE_GAIN_H_

#include <stddef.h>

#include <array>

#include "api/audio/echo_canceller3_config.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/aec3/fft_data.h"
#include "modules/audio_processing/aec3/render_signal_analyzer.h"

namespace webrtc {

// Provides functionality for computing the fixed gain for the coarse filter.
class CoarseFilterUpdateGain {
 public:
  explicit CoarseFilterUpdateGain(
      const EchoCanceller3Config::Filter::CoarseConfiguration& config,
      size_t config_change_duration_blocks);

  // Takes action in the case of a known echo path change.
  void HandleEchoPathChange();

  // Computes the gain.
  void Compute(const std::array<float, kFftLengthBy2Plus1>& render_power,
               const RenderSignalAnalyzer& render_signal_analyzer,
               const FftData& E_coarse,
               size_t size_partitions,
               bool saturated_capture_signal,
               FftData* G);

  // Sets a new config.
  void SetConfig(
      const EchoCanceller3Config::Filter::CoarseConfiguration& config,
      bool immediate_effect) {
    if (immediate_effect) {
      old_target_config_ = current_config_ = target_config_ = config;
      config_change_counter_ = 0;
    } else {
      old_target_config_ = current_config_;
      target_config_ = config;
      config_change_counter_ = config_change_duration_blocks_;
    }
  }

 private:
  EchoCanceller3Config::Filter::CoarseConfiguration current_config_;
  EchoCanceller3Config::Filter::CoarseConfiguration target_config_;
  EchoCanceller3Config::Filter::CoarseConfiguration old_target_config_;
  const int config_change_duration_blocks_;
  float one_by_config_change_duration_blocks_;
  // TODO(peah): Check whether this counter should instead be initialized to a
  // large value.
  size_t poor_signal_excitation_counter_ = 0;
  size_t call_counter_ = 0;
  int config_change_counter_ = 0;

  void UpdateCurrentConfig();
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_COARSE_FILTER_UPDATE_GAIN_H_
