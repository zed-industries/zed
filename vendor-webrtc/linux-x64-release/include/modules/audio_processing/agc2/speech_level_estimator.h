/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_SPEECH_LEVEL_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_AGC2_SPEECH_LEVEL_ESTIMATOR_H_

#include <stddef.h>

#include <type_traits>

#include "api/audio/audio_processing.h"
#include "modules/audio_processing/agc2/agc2_common.h"

namespace webrtc {
class ApmDataDumper;

// Active speech level estimator based on the analysis of the following
// framewise properties: RMS level (dBFS), peak level (dBFS), speech
// probability.
class SpeechLevelEstimator {
 public:
  SpeechLevelEstimator(
      ApmDataDumper* apm_data_dumper,
      const AudioProcessing::Config::GainController2::AdaptiveDigital& config,
      int adjacent_speech_frames_threshold);
  SpeechLevelEstimator(const SpeechLevelEstimator&) = delete;
  SpeechLevelEstimator& operator=(const SpeechLevelEstimator&) = delete;

  // Updates the level estimation.
  void Update(float rms_dbfs, float peak_dbfs, float speech_probability);
  // Returns the estimated speech plus noise level.
  float level_dbfs() const { return level_dbfs_; }
  // Returns true if the estimator is confident on its current estimate.
  bool is_confident() const { return is_confident_; }

  void Reset();

 private:
  // Part of the level estimator state used for check-pointing and restore ops.
  struct LevelEstimatorState {
    bool operator==(const LevelEstimatorState& s) const;
    inline bool operator!=(const LevelEstimatorState& s) const {
      return !(*this == s);
    }
    // TODO(bugs.webrtc.org/7494): Remove `time_to_confidence_ms` if redundant.
    int time_to_confidence_ms;
    struct Ratio {
      float numerator;
      float denominator;
      float GetRatio() const;
    } level_dbfs;
  };
  static_assert(std::is_trivially_copyable<LevelEstimatorState>::value, "");

  void UpdateIsConfident();

  void ResetLevelEstimatorState(LevelEstimatorState& state) const;

  void DumpDebugData() const;

  ApmDataDumper* const apm_data_dumper_;

  const float initial_speech_level_dbfs_;
  const int adjacent_speech_frames_threshold_;
  LevelEstimatorState preliminary_state_;
  LevelEstimatorState reliable_state_;
  float level_dbfs_;
  bool is_confident_;
  int num_adjacent_speech_frames_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_SPEECH_LEVEL_ESTIMATOR_H_
