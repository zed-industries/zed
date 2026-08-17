/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_AEC_STATE_H_
#define MODULES_AUDIO_PROCESSING_AEC3_AEC_STATE_H_

#include <stddef.h>

#include <array>
#include <atomic>
#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/audio/echo_canceller3_config.h"
#include "api/environment/environment.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/aec3/delay_estimate.h"
#include "modules/audio_processing/aec3/echo_audibility.h"
#include "modules/audio_processing/aec3/echo_path_variability.h"
#include "modules/audio_processing/aec3/erl_estimator.h"
#include "modules/audio_processing/aec3/erle_estimator.h"
#include "modules/audio_processing/aec3/filter_analyzer.h"
#include "modules/audio_processing/aec3/render_buffer.h"
#include "modules/audio_processing/aec3/reverb_model_estimator.h"
#include "modules/audio_processing/aec3/subtractor_output.h"
#include "modules/audio_processing/aec3/subtractor_output_analyzer.h"
#include "modules/audio_processing/aec3/transparent_mode.h"

namespace webrtc {

class ApmDataDumper;

// Handles the state and the conditions for the echo removal functionality.
class AecState {
 public:
  AecState(const Environment& env,
           const EchoCanceller3Config& config,
           size_t num_capture_channels);
  ~AecState();

  // Returns whether the echo subtractor can be used to determine the residual
  // echo.
  bool UsableLinearEstimate() const {
    return filter_quality_state_.LinearFilterUsable() &&
           config_.filter.use_linear_filter;
  }

  // Returns whether the echo subtractor output should be used as output.
  bool UseLinearFilterOutput() const {
    return filter_quality_state_.LinearFilterUsable() &&
           config_.filter.use_linear_filter;
  }

  // Returns whether the render signal is currently active.
  bool ActiveRender() const { return blocks_with_active_render_ > 200; }

  // Returns the appropriate scaling of the residual echo to match the
  // audibility.
  void GetResidualEchoScaling(ArrayView<float> residual_scaling) const;

  // Returns whether the stationary properties of the signals are used in the
  // aec.
  bool UseStationarityProperties() const {
    return config_.echo_audibility.use_stationarity_properties;
  }

  // Returns the ERLE.
  ArrayView<const std::array<float, kFftLengthBy2Plus1>> Erle(
      bool onset_compensated) const {
    return erle_estimator_.Erle(onset_compensated);
  }

  // Returns the non-capped ERLE.
  ArrayView<const std::array<float, kFftLengthBy2Plus1>> ErleUnbounded() const {
    return erle_estimator_.ErleUnbounded();
  }

  // Returns the fullband ERLE estimate in log2 units.
  float FullBandErleLog2() const { return erle_estimator_.FullbandErleLog2(); }

  // Returns the ERL.
  const std::array<float, kFftLengthBy2Plus1>& Erl() const {
    return erl_estimator_.Erl();
  }

  // Returns the time-domain ERL.
  float ErlTimeDomain() const { return erl_estimator_.ErlTimeDomain(); }

  // Returns the delay estimate based on the linear filter.
  int MinDirectPathFilterDelay() const {
    return delay_state_.MinDirectPathFilterDelay();
  }

  // Returns whether the capture signal is saturated.
  bool SaturatedCapture() const { return capture_signal_saturation_; }

  // Returns whether the echo signal is saturated.
  bool SaturatedEcho() const { return saturation_detector_.SaturatedEcho(); }

  // Updates the capture signal saturation.
  void UpdateCaptureSaturation(bool capture_signal_saturation) {
    capture_signal_saturation_ = capture_signal_saturation;
  }

  // Returns whether the transparent mode is active
  bool TransparentModeActive() const {
    return transparent_state_ && transparent_state_->Active();
  }

  // Takes appropriate action at an echo path change.
  void HandleEchoPathChange(const EchoPathVariability& echo_path_variability);

  // Returns the decay factor for the echo reverberation. The parameter `mild`
  // indicates which exponential decay to return. The default one or a milder
  // one that can be used during nearend regions.
  float ReverbDecay(bool mild) const {
    return reverb_model_estimator_.ReverbDecay(mild);
  }

  // Return the frequency response of the reverberant echo.
  ArrayView<const float> GetReverbFrequencyResponse() const {
    return reverb_model_estimator_.GetReverbFrequencyResponse();
  }

  // Returns whether the transition for going out of the initial stated has
  // been triggered.
  bool TransitionTriggered() const {
    return initial_state_.TransitionTriggered();
  }

  // Updates the aec state.
  // TODO(bugs.webrtc.org/10913): Compute multi-channel ERL.
  void Update(
      const std::optional<DelayEstimate>& external_delay,
      ArrayView<const std::vector<std::array<float, kFftLengthBy2Plus1>>>
          adaptive_filter_frequency_responses,
      ArrayView<const std::vector<float>> adaptive_filter_impulse_responses,
      const RenderBuffer& render_buffer,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> E2_refined,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> Y2,
      ArrayView<const SubtractorOutput> subtractor_output);

  // Returns filter length in blocks.
  int FilterLengthBlocks() const {
    // All filters have the same length, so arbitrarily return channel 0 length.
    return filter_analyzer_.FilterLengthBlocks();
  }

 private:
  static std::atomic<int> instance_count_;
  std::unique_ptr<ApmDataDumper> data_dumper_;
  const EchoCanceller3Config config_;
  const size_t num_capture_channels_;
  const bool deactivate_initial_state_reset_at_echo_path_change_;
  const bool full_reset_at_echo_path_change_;
  const bool subtractor_analyzer_reset_at_echo_path_change_;

  // Class for controlling the transition from the intial state, which in turn
  // controls when the filter parameters for the initial state should be used.
  class InitialState {
   public:
    explicit InitialState(const EchoCanceller3Config& config);
    // Resets the state to again begin in the initial state.
    void Reset();

    // Updates the state based on new data.
    void Update(bool active_render, bool saturated_capture);

    // Returns whether the initial state is active or not.
    bool InitialStateActive() const { return initial_state_; }

    // Returns that the transition from the initial state has was started.
    bool TransitionTriggered() const { return transition_triggered_; }

   private:
    const bool conservative_initial_phase_;
    const float initial_state_seconds_;
    bool transition_triggered_ = false;
    bool initial_state_ = true;
    size_t strong_not_saturated_render_blocks_ = 0;
  } initial_state_;

  // Class for choosing the direct-path delay relative to the beginning of the
  // filter, as well as any other data related to the delay used within
  // AecState.
  class FilterDelay {
   public:
    FilterDelay(const EchoCanceller3Config& config,
                size_t num_capture_channels);

    // Returns whether an external delay has been reported to the AecState (from
    // the delay estimator).
    bool ExternalDelayReported() const { return external_delay_reported_; }

    // Returns the delay in blocks relative to the beginning of the filter that
    // corresponds to the direct path of the echo.
    ArrayView<const int> DirectPathFilterDelays() const {
      return filter_delays_blocks_;
    }

    // Returns the minimum delay among the direct path delays relative to the
    // beginning of the filter
    int MinDirectPathFilterDelay() const { return min_filter_delay_; }

    // Updates the delay estimates based on new data.
    void Update(ArrayView<const int> analyzer_filter_delay_estimates_blocks,
                const std::optional<DelayEstimate>& external_delay,
                size_t blocks_with_proper_filter_adaptation);

   private:
    const int delay_headroom_blocks_;
    bool external_delay_reported_ = false;
    std::vector<int> filter_delays_blocks_;
    int min_filter_delay_;
    std::optional<DelayEstimate> external_delay_;
  } delay_state_;

  // Classifier for toggling transparent mode when there is no echo.
  std::unique_ptr<TransparentMode> transparent_state_;

  // Class for analyzing how well the linear filter is, and can be expected to,
  // perform on the current signals. The purpose of this is for using to
  // select the echo suppression functionality as well as the input to the echo
  // suppressor.
  class FilteringQualityAnalyzer {
   public:
    FilteringQualityAnalyzer(const EchoCanceller3Config& config,
                             size_t num_capture_channels);

    // Returns whether the linear filter can be used for the echo
    // canceller output.
    bool LinearFilterUsable() const { return overall_usable_linear_estimates_; }

    // Returns whether an individual filter output can be used for the echo
    // canceller output.
    const std::vector<bool>& UsableLinearFilterOutputs() const {
      return usable_linear_filter_estimates_;
    }

    // Resets the state of the analyzer.
    void Reset();

    // Updates the analysis based on new data.
    void Update(bool active_render,
                bool transparent_mode,
                bool saturated_capture,
                const std::optional<DelayEstimate>& external_delay,
                bool any_filter_converged);

   private:
    const bool use_linear_filter_;
    bool overall_usable_linear_estimates_ = false;
    size_t filter_update_blocks_since_reset_ = 0;
    size_t filter_update_blocks_since_start_ = 0;
    bool convergence_seen_ = false;
    std::vector<bool> usable_linear_filter_estimates_;
  } filter_quality_state_;

  // Class for detecting whether the echo is to be considered to be
  // saturated.
  class SaturationDetector {
   public:
    // Returns whether the echo is to be considered saturated.
    bool SaturatedEcho() const { return saturated_echo_; }

    // Updates the detection decision based on new data.
    void Update(const Block& x,
                bool saturated_capture,
                bool usable_linear_estimate,
                ArrayView<const SubtractorOutput> subtractor_output,
                float echo_path_gain);

   private:
    bool saturated_echo_ = false;
  } saturation_detector_;

  ErlEstimator erl_estimator_;
  ErleEstimator erle_estimator_;
  size_t strong_not_saturated_render_blocks_ = 0;
  size_t blocks_with_active_render_ = 0;
  bool capture_signal_saturation_ = false;
  FilterAnalyzer filter_analyzer_;
  EchoAudibility echo_audibility_;
  ReverbModelEstimator reverb_model_estimator_;
  ReverbModel avg_render_reverb_;
  SubtractorOutputAnalyzer subtractor_output_analyzer_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_AEC_STATE_H_
