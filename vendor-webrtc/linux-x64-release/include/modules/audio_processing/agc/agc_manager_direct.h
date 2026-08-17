/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC_AGC_MANAGER_DIRECT_H_
#define MODULES_AUDIO_PROCESSING_AGC_AGC_MANAGER_DIRECT_H_

#include <atomic>
#include <memory>
#include <optional>

#include "api/array_view.h"
#include "api/audio/audio_processing.h"
#include "api/environment/environment.h"
#include "modules/audio_processing/agc/agc.h"
#include "modules/audio_processing/agc2/clipping_predictor.h"
#include "modules/audio_processing/audio_buffer.h"
#include "modules/audio_processing/logging/apm_data_dumper.h"
#include "rtc_base/gtest_prod_util.h"

namespace webrtc {

class MonoAgc;
class GainControl;

// Adaptive Gain Controller (AGC) that controls the input volume and a digital
// gain. The input volume controller recommends what volume to use, handles
// volume changes and clipping. In particular, it handles changes triggered by
// the user (e.g., volume set to zero by a HW mute button). The digital
// controller chooses and applies the digital compression gain.
// This class is not thread-safe.
// TODO(bugs.webrtc.org/7494): Use applied/recommended input volume naming
// convention.
class AgcManagerDirect final {
 public:
  // Ctor. `num_capture_channels` specifies the number of channels for the audio
  // passed to `AnalyzePreProcess()` and `Process()`. Clamps
  // `analog_config.startup_min_level` in the [12, 255] range.
  AgcManagerDirect(
      const Environment& env,
      int num_capture_channels,
      const AudioProcessing::Config::GainController1::AnalogGainController&
          analog_config);

  ~AgcManagerDirect();
  AgcManagerDirect(const AgcManagerDirect&) = delete;
  AgcManagerDirect& operator=(const AgcManagerDirect&) = delete;

  void Initialize();

  // Configures `gain_control` to work as a fixed digital controller so that the
  // adaptive part is only handled by this gain controller. Must be called if
  // `gain_control` is also used to avoid the side-effects of running two AGCs.
  void SetupDigitalGainControl(GainControl& gain_control) const;

  // Sets the applied input volume.
  void set_stream_analog_level(int level);

  // TODO(bugs.webrtc.org/7494): Add argument for the applied input volume and
  // remove `set_stream_analog_level()`.
  // Analyzes `audio` before `Process()` is called so that the analysis can be
  // performed before external digital processing operations take place (e.g.,
  // echo cancellation). The analysis consists of input clipping detection and
  // prediction (if enabled). Must be called after `set_stream_analog_level()`.
  void AnalyzePreProcess(const AudioBuffer& audio_buffer);

  // Processes `audio_buffer`. Chooses a digital compression gain and the new
  // input volume to recommend. Must be called after `AnalyzePreProcess()`. If
  // `speech_probability` (range [0.0f, 1.0f]) and `speech_level_dbfs` (range
  // [-90.f, 30.0f]) are given, uses them to override the estimated RMS error.
  // TODO(webrtc:7494): This signature is needed for testing purposes, unify
  // the signatures when the clean-up is done.
  void Process(const AudioBuffer& audio_buffer,
               std::optional<float> speech_probability,
               std::optional<float> speech_level_dbfs);

  // Processes `audio_buffer`. Chooses a digital compression gain and the new
  // input volume to recommend. Must be called after `AnalyzePreProcess()`.
  void Process(const AudioBuffer& audio_buffer);

  // TODO(bugs.webrtc.org/7494): Return recommended input volume and remove
  // `recommended_analog_level()`.
  // Returns the recommended input volume. If the input volume contoller is
  // disabled, returns the input volume set via the latest
  // `set_stream_analog_level()` call. Must be called after
  // `AnalyzePreProcess()` and `Process()`.
  int recommended_analog_level() const { return recommended_input_volume_; }

  // Call when the capture stream output has been flagged to be used/not-used.
  // If unused, the manager  disregards all incoming audio.
  void HandleCaptureOutputUsedChange(bool capture_output_used);

  float voice_probability() const;

  int num_channels() const { return num_capture_channels_; }

  // If available, returns the latest digital compression gain that has been
  // chosen.
  std::optional<int> GetDigitalComressionGain();

  // Returns true if clipping prediction is enabled.
  bool clipping_predictor_enabled() const { return !!clipping_predictor_; }

  // Returns true if clipping prediction is used to adjust the input volume.
  bool use_clipping_predictor_step() const {
    return use_clipping_predictor_step_;
  }

 private:
  friend class AgcManagerDirectTestHelper;

  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectTest, DisableDigitalDisablesDigital);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectTest,
                           AgcMinMicLevelExperimentDefault);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectTest,
                           AgcMinMicLevelExperimentDisabled);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectTest,
                           AgcMinMicLevelExperimentOutOfRangeAbove);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectTest,
                           AgcMinMicLevelExperimentOutOfRangeBelow);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectTest,
                           AgcMinMicLevelExperimentEnabled50);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectTest,
                           AgcMinMicLevelExperimentEnabledAboveStartupLevel);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectParametrizedTest,
                           ClippingParametersVerified);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectParametrizedTest,
                           DisableClippingPredictorDoesNotLowerVolume);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectParametrizedTest,
                           UsedClippingPredictionsProduceLowerAnalogLevels);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectParametrizedTest,
                           UnusedClippingPredictionsProduceEqualAnalogLevels);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectParametrizedTest,
                           EmptyRmsErrorOverrideHasNoEffect);
  FRIEND_TEST_ALL_PREFIXES(AgcManagerDirectParametrizedTest,
                           NonEmptyRmsErrorOverrideHasEffect);

  // Ctor that creates a single channel AGC and by injecting `agc`.
  // `agc` will be owned by this class; hence, do not delete it.
  AgcManagerDirect(
      const Environment& env,
      const AudioProcessing::Config::GainController1::AnalogGainController&
          analog_config,
      Agc* agc);

  void AggregateChannelLevels();

  const bool analog_controller_enabled_;

  const std::optional<int> min_mic_level_override_;
  std::unique_ptr<ApmDataDumper> data_dumper_;
  static std::atomic<int> instance_counter_;
  const int num_capture_channels_;
  const bool disable_digital_adaptive_;

  int frames_since_clipped_;

  // TODO(bugs.webrtc.org/7494): Create a separate member for the applied input
  // volume.
  // TODO(bugs.webrtc.org/7494): Once
  // `AudioProcessingImpl::recommended_stream_analog_level()` becomes a trivial
  // getter, leave uninitialized.
  // Recommended input volume. After `set_stream_analog_level()` is called it
  // holds the observed input volume. Possibly updated by `AnalyzePreProcess()`
  // and `Process()`; after these calls, holds the recommended input volume.
  int recommended_input_volume_ = 0;

  bool capture_output_used_;
  int channel_controlling_gain_ = 0;

  const int clipped_level_step_;
  const float clipped_ratio_threshold_;
  const int clipped_wait_frames_;

  std::vector<std::unique_ptr<MonoAgc>> channel_agcs_;
  std::vector<std::optional<int>> new_compressions_to_set_;

  const std::unique_ptr<ClippingPredictor> clipping_predictor_;
  const bool use_clipping_predictor_step_;
  float clipping_rate_log_;
  int clipping_rate_log_counter_;
};

// TODO(bugs.webrtc.org/7494): Use applied/recommended input volume naming
// convention.
class MonoAgc {
 public:
  MonoAgc(ApmDataDumper* data_dumper,
          int clipped_level_min,
          bool disable_digital_adaptive,
          int min_mic_level);
  ~MonoAgc();
  MonoAgc(const MonoAgc&) = delete;
  MonoAgc& operator=(const MonoAgc&) = delete;

  void Initialize();
  void HandleCaptureOutputUsedChange(bool capture_output_used);

  // Sets the current input volume.
  void set_stream_analog_level(int level) { recommended_input_volume_ = level; }

  // Lowers the recommended input volume in response to clipping based on the
  // suggested reduction `clipped_level_step`. Must be called after
  // `set_stream_analog_level()`.
  void HandleClipping(int clipped_level_step);

  // Analyzes `audio`, requests the RMS error from AGC, updates the recommended
  // input volume based on the estimated speech level and, if enabled, updates
  // the (digital) compression gain to be applied by `agc_`. Must be called
  // after `HandleClipping()`. If `rms_error_override` has a value, RMS error
  // from AGC is overridden by it.
  void Process(ArrayView<const int16_t> audio,
               std::optional<int> rms_error_override);

  // Returns the recommended input volume. Must be called after `Process()`.
  int recommended_analog_level() const { return recommended_input_volume_; }

  float voice_probability() const { return agc_->voice_probability(); }
  void ActivateLogging() { log_to_histograms_ = true; }
  std::optional<int> new_compression() const { return new_compression_to_set_; }

  // Only used for testing.
  void set_agc(Agc* agc) { agc_.reset(agc); }
  int min_mic_level() const { return min_mic_level_; }

 private:
  // Sets a new input volume, after first checking that it hasn't been updated
  // by the user, in which case no action is taken.
  void SetLevel(int new_level);

  // Set the maximum input volume the AGC is allowed to apply. Also updates the
  // maximum compression gain to compensate. The volume must be at least
  // `kClippedLevelMin`.
  void SetMaxLevel(int level);

  int CheckVolumeAndReset();
  void UpdateGain(int rms_error_db);
  void UpdateCompressor();

  const int min_mic_level_;
  const bool disable_digital_adaptive_;
  std::unique_ptr<Agc> agc_;
  int level_ = 0;
  int max_level_;
  int max_compression_gain_;
  int target_compression_;
  int compression_;
  float compression_accumulator_;
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

  std::optional<int> new_compression_to_set_;
  bool log_to_histograms_ = false;
  const int clipped_level_min_;

  // Frames since the last `UpdateGain()` call.
  int frames_since_update_gain_ = 0;
  // Set to true for the first frame after startup and reset, otherwise false.
  bool is_first_frame_ = true;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC_AGC_MANAGER_DIRECT_H_
