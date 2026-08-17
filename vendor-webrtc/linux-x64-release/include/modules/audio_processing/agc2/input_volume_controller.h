/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_INPUT_VOLUME_CONTROLLER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_INPUT_VOLUME_CONTROLLER_H_

#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/audio/audio_processing.h"
#include "modules/audio_processing/agc2/clipping_predictor.h"
#include "modules/audio_processing/audio_buffer.h"
#include "rtc_base/gtest_prod_util.h"

namespace webrtc {

class MonoInputVolumeController;

// The input volume controller recommends what volume to use, handles volume
// changes and clipping detection and prediction. In particular, it handles
// changes triggered by the user (e.g., volume set to zero by a HW mute button).
// This class is not thread-safe.
// TODO(bugs.webrtc.org/7494): Use applied/recommended input volume naming
// convention.
class InputVolumeController final {
 public:
  // Config for the constructor.
  struct Config {
    // Minimum input volume that can be recommended. Not enforced when the
    // applied input volume is zero outside startup.
    int min_input_volume = 20;
    // Lowest input volume level that will be applied in response to clipping.
    int clipped_level_min = 70;
    // Amount input volume level is lowered with every clipping event. Limited
    // to (0, 255].
    int clipped_level_step = 15;
    // Proportion of clipped samples required to declare a clipping event.
    // Limited to (0.0f, 1.0f).
    float clipped_ratio_threshold = 0.1f;
    // Time in frames to wait after a clipping event before checking again.
    // Limited to values higher than 0.
    int clipped_wait_frames = 300;
    // Enables clipping prediction functionality.
    bool enable_clipping_predictor = true;
    // Speech level target range (dBFS). If the speech level is in the range
    // [`target_range_min_dbfs`, `target_range_max_dbfs`], no input volume
    // adjustments are done based on the speech level. For speech levels below
    // and above the range, the targets `target_range_min_dbfs` and
    // `target_range_max_dbfs` are used, respectively.
    int target_range_max_dbfs = -30;
    int target_range_min_dbfs = -50;
    // Number of wait frames between the recommended input volume updates.
    int update_input_volume_wait_frames = 100;
    // Speech probability threshold: speech probabilities below the threshold
    // are considered silence. Limited to [0.0f, 1.0f].
    float speech_probability_threshold = 0.7f;
    // Minimum speech frame ratio for volume updates to be allowed. Limited to
    // [0.0f, 1.0f].
    float speech_ratio_threshold = 0.6f;
  };

  // Ctor. `num_capture_channels` specifies the number of channels for the audio
  // passed to `AnalyzePreProcess()` and `Process()`. Clamps
  // `config.startup_min_level` in the [12, 255] range.
  InputVolumeController(int num_capture_channels, const Config& config);

  ~InputVolumeController();
  InputVolumeController(const InputVolumeController&) = delete;
  InputVolumeController& operator=(const InputVolumeController&) = delete;

  // TODO(webrtc:7494): Integrate initialization into ctor and remove.
  void Initialize();

  // Analyzes `audio_buffer` before `RecommendInputVolume()` is called so tha
  // the analysis can be performed before digital processing operations take
  // place (e.g., echo cancellation). The analysis consists of input clipping
  // detection and prediction (if enabled).
  void AnalyzeInputAudio(int applied_input_volume,
                         const AudioBuffer& audio_buffer);

  // Adjusts the recommended input volume upwards/downwards based on the result
  // of `AnalyzeInputAudio()` and on `speech_level_dbfs` (if specified). Must
  // be called after `AnalyzeInputAudio()`.  The value of `speech_probability`
  // is expected to be in the range [0, 1] and `speech_level_dbfs` in the range
  // [-90, 30] and both should be estimated after echo cancellation and noise
  // suppression are applied. Returns a non-empty input volume recommendation if
  // available. If `capture_output_used_` is true, returns the applied input
  // volume.
  std::optional<int> RecommendInputVolume(
      float speech_probability,
      std::optional<float> speech_level_dbfs);

  // Stores whether the capture output will be used or not. Call when the
  // capture stream output has been flagged to be used/not-used. If unused, the
  // controller disregards all incoming audio.
  void HandleCaptureOutputUsedChange(bool capture_output_used);

  // Returns true if clipping prediction is enabled.
  // TODO(bugs.webrtc.org/7494): Deprecate this method.
  bool clipping_predictor_enabled() const { return !!clipping_predictor_; }

  // Returns true if clipping prediction is used to adjust the input volume.
  // TODO(bugs.webrtc.org/7494): Deprecate this method.
  bool use_clipping_predictor_step() const {
    return use_clipping_predictor_step_;
  }

  // Only use for testing: Use `RecommendInputVolume()` elsewhere.
  // Returns the value of a member variable, needed for testing
  // `AnalyzeInputAudio()`.
  int recommended_input_volume() const { return recommended_input_volume_; }

  // Only use for testing.
  bool capture_output_used() const { return capture_output_used_; }

 private:
  friend class InputVolumeControllerTestHelper;

  FRIEND_TEST_ALL_PREFIXES(InputVolumeControllerTest, MinInputVolumeDefault);
  FRIEND_TEST_ALL_PREFIXES(InputVolumeControllerTest, MinInputVolumeDisabled);
  FRIEND_TEST_ALL_PREFIXES(InputVolumeControllerTest,
                           MinInputVolumeOutOfRangeAbove);
  FRIEND_TEST_ALL_PREFIXES(InputVolumeControllerTest,
                           MinInputVolumeOutOfRangeBelow);
  FRIEND_TEST_ALL_PREFIXES(InputVolumeControllerTest, MinInputVolumeEnabled50);
  FRIEND_TEST_ALL_PREFIXES(InputVolumeControllerParametrizedTest,
                           ClippingParametersVerified);

  // Sets the applied input volume and resets the recommended input volume.
  void SetAppliedInputVolume(int level);

  void AggregateChannelLevels();

  const int num_capture_channels_;

  // Minimum input volume that can be recommended.
  const int min_input_volume_;

  // TODO(bugs.webrtc.org/7494): Once
  // `AudioProcessingImpl::recommended_stream_analog_level()` becomes a trivial
  // getter, leave uninitialized.
  // Recommended input volume. After `SetAppliedInputVolume()` is called it
  // holds holds the observed input volume. Possibly updated by
  // `AnalyzePreProcess()` and `Process()`; after these calls, holds the
  // recommended input volume.
  int recommended_input_volume_ = 0;
  // Applied input volume. After `SetAppliedInputVolume()` is called it holds
  // the current applied volume.
  std::optional<int> applied_input_volume_;

  bool capture_output_used_;

  // Clipping detection and prediction.
  const int clipped_level_step_;
  const float clipped_ratio_threshold_;
  const int clipped_wait_frames_;
  const std::unique_ptr<ClippingPredictor> clipping_predictor_;
  const bool use_clipping_predictor_step_;
  int frames_since_clipped_;
  int clipping_rate_log_counter_;
  float clipping_rate_log_;

  // Target range minimum and maximum. If the seech level is in the range
  // [`target_range_min_dbfs`, `target_range_max_dbfs`], no volume adjustments
  // take place. Instead, the digital gain controller is assumed to adapt to
  // compensate for the speech level RMS error.
  const int target_range_max_dbfs_;
  const int target_range_min_dbfs_;

  // Channel controllers updating the gain upwards/downwards.
  std::vector<std::unique_ptr<MonoInputVolumeController>> channel_controllers_;
  int channel_controlling_gain_ = 0;
};

// TODO(bugs.webrtc.org/7494): Use applied/recommended input volume naming
// convention.
class MonoInputVolumeController {
 public:
  MonoInputVolumeController(int min_input_volume_after_clipping,
                            int min_input_volume,
                            int update_input_volume_wait_frames,
                            float speech_probability_threshold,
                            float speech_ratio_threshold);
  ~MonoInputVolumeController();
  MonoInputVolumeController(const MonoInputVolumeController&) = delete;
  MonoInputVolumeController& operator=(const MonoInputVolumeController&) =
      delete;

  void Initialize();
  void HandleCaptureOutputUsedChange(bool capture_output_used);

  // Sets the current input volume.
  void set_stream_analog_level(int input_volume) {
    recommended_input_volume_ = input_volume;
  }

  // Lowers the recommended input volume in response to clipping based on the
  // suggested reduction `clipped_level_step`. Must be called after
  // `set_stream_analog_level()`.
  void HandleClipping(int clipped_level_step);

  // TODO(bugs.webrtc.org/7494): Rename, audio not passed to the method anymore.
  // Adjusts the recommended input volume upwards/downwards depending on the
  // result of `HandleClipping()` and on `rms_error_dbfs`. Updates are only
  // allowed for active speech segments and when `rms_error_dbfs` is not empty.
  // Must be called after `HandleClipping()`.
  void Process(std::optional<int> rms_error_dbfs, float speech_probability);

  // Returns the recommended input volume. Must be called after `Process()`.
  int recommended_analog_level() const { return recommended_input_volume_; }

  void ActivateLogging() { log_to_histograms_ = true; }

  int min_input_volume_after_clipping() const {
    return min_input_volume_after_clipping_;
  }

  // Only used for testing.
  int min_input_volume() const { return min_input_volume_; }

 private:
  // Sets a new input volume, after first checking that it hasn't been updated
  // by the user, in which case no action is taken.
  void SetInputVolume(int new_volume);

  // Sets the maximum input volume that the input volume controller is allowed
  // to apply. The volume must be at least `kClippedLevelMin`.
  void SetMaxLevel(int level);

  int CheckVolumeAndReset();

  // Updates the recommended input volume. If the volume slider needs to be
  // moved, we check first if the user has adjusted it, in which case we take no
  // action and cache the updated level.
  void UpdateInputVolume(int rms_error_dbfs);

  const int min_input_volume_;
  const int min_input_volume_after_clipping_;
  int max_input_volume_;

  int last_recommended_input_volume_ = 0;

  bool capture_output_used_ = true;
  bool check_volume_on_next_process_ = true;
  bool startup_ = true;

  // TODO(bugs.webrtc.org/7494): Create a separate member for the applied
  // input volume.
  // Recommended input volume. After `set_stream_analog_level()` is
  // called, it holds the observed applied input volume. Possibly updated by
  // `HandleClipping()` and `Process()`; after these calls, holds the
  // recommended input volume.
  int recommended_input_volume_ = 0;

  bool log_to_histograms_ = false;

  // Counters for frames and speech frames since the last update in the
  // recommended input volume.
  const int update_input_volume_wait_frames_;
  int frames_since_update_input_volume_ = 0;
  int speech_frames_since_update_input_volume_ = 0;
  bool is_first_frame_ = true;

  // Speech probability threshold for a frame to be considered speech (instead
  // of silence). Limited to [0.0f, 1.0f].
  const float speech_probability_threshold_;
  // Minimum ratio of speech frames. Limited to [0.0f, 1.0f].
  const float speech_ratio_threshold_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_INPUT_VOLUME_CONTROLLER_H_
