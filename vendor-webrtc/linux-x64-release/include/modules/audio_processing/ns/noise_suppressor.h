/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_NS_NOISE_SUPPRESSOR_H_
#define MODULES_AUDIO_PROCESSING_NS_NOISE_SUPPRESSOR_H_

#include <memory>
#include <vector>

#include "api/array_view.h"
#include "modules/audio_processing/audio_buffer.h"
#include "modules/audio_processing/ns/noise_estimator.h"
#include "modules/audio_processing/ns/ns_common.h"
#include "modules/audio_processing/ns/ns_config.h"
#include "modules/audio_processing/ns/ns_fft.h"
#include "modules/audio_processing/ns/speech_probability_estimator.h"
#include "modules/audio_processing/ns/wiener_filter.h"

namespace webrtc {

// Class for suppressing noise in a signal.
class NoiseSuppressor {
 public:
  NoiseSuppressor(const NsConfig& config,
                  size_t sample_rate_hz,
                  size_t num_channels);
  NoiseSuppressor(const NoiseSuppressor&) = delete;
  NoiseSuppressor& operator=(const NoiseSuppressor&) = delete;

  // Analyses the signal (typically applied before the AEC to avoid analyzing
  // any comfort noise signal).
  void Analyze(const AudioBuffer& audio);

  // Applies noise suppression.
  void Process(AudioBuffer* audio);

  // Specifies whether the capture output will be used. The purpose of this is
  // to allow the noise suppressor to deactivate some of the processing when the
  // resulting output is anyway not used, for instance when the endpoint is
  // muted.
  void SetCaptureOutputUsage(bool capture_output_used) {
    capture_output_used_ = capture_output_used;
  }

 private:
  const size_t num_bands_;
  const size_t num_channels_;
  const SuppressionParams suppression_params_;
  int32_t num_analyzed_frames_ = -1;
  NrFft fft_;
  bool capture_output_used_ = true;

  struct ChannelState {
    ChannelState(const SuppressionParams& suppression_params, size_t num_bands);

    SpeechProbabilityEstimator speech_probability_estimator;
    WienerFilter wiener_filter;
    NoiseEstimator noise_estimator;
    std::array<float, kFftSizeBy2Plus1> prev_analysis_signal_spectrum;
    std::array<float, kFftSize - kNsFrameSize> analyze_analysis_memory;
    std::array<float, kOverlapSize> process_analysis_memory;
    std::array<float, kOverlapSize> process_synthesis_memory;
    std::vector<std::array<float, kOverlapSize>> process_delay_memory;
  };

  struct FilterBankState {
    std::array<float, kFftSize> real;
    std::array<float, kFftSize> imag;
    std::array<float, kFftSize> extended_frame;
  };

  std::vector<FilterBankState> filter_bank_states_heap_;
  std::vector<float> upper_band_gains_heap_;
  std::vector<float> energies_before_filtering_heap_;
  std::vector<float> gain_adjustments_heap_;
  std::vector<std::unique_ptr<ChannelState>> channels_;

  // Aggregates the Wiener filters into a single filter to use.
  void AggregateWienerFilters(ArrayView<float, kFftSizeBy2Plus1> filter) const;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_NS_NOISE_SUPPRESSOR_H_
