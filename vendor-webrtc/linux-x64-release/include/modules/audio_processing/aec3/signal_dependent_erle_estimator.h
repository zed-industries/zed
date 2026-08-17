/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_SIGNAL_DEPENDENT_ERLE_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_AEC3_SIGNAL_DEPENDENT_ERLE_ESTIMATOR_H_

#include <memory>
#include <vector>

#include "api/array_view.h"
#include "api/audio/echo_canceller3_config.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/aec3/render_buffer.h"
#include "modules/audio_processing/logging/apm_data_dumper.h"

namespace webrtc {

// This class estimates the dependency of the Erle to the input signal. By
// looking at the input signal, an estimation on whether the current echo
// estimate is due to the direct path or to a more reverberant one is performed.
// Once that estimation is done, it is possible to refine the average Erle that
// this class receive as an input.
class SignalDependentErleEstimator {
 public:
  SignalDependentErleEstimator(const EchoCanceller3Config& config,
                               size_t num_capture_channels);

  ~SignalDependentErleEstimator();

  void Reset();

  // Returns the Erle per frequency subband.
  ArrayView<const std::array<float, kFftLengthBy2Plus1>> Erle(
      bool onset_compensated) const {
    return onset_compensated && use_onset_detection_ ? erle_onset_compensated_
                                                     : erle_;
  }

  // Updates the Erle estimate. The Erle that is passed as an input is required
  // to be an estimation of the average Erle achieved by the linear filter.
  void Update(
      const RenderBuffer& render_buffer,
      ArrayView<const std::vector<std::array<float, kFftLengthBy2Plus1>>>
          filter_frequency_response,
      ArrayView<const float, kFftLengthBy2Plus1> X2,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> Y2,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> E2,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> average_erle,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>>
          average_erle_onset_compensated,
      const std::vector<bool>& converged_filters);

  void Dump(const std::unique_ptr<ApmDataDumper>& data_dumper) const;

  static constexpr size_t kSubbands = 6;

 private:
  void ComputeNumberOfActiveFilterSections(
      const RenderBuffer& render_buffer,
      ArrayView<const std::vector<std::array<float, kFftLengthBy2Plus1>>>
          filter_frequency_responses);

  void UpdateCorrectionFactors(
      ArrayView<const float, kFftLengthBy2Plus1> X2,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> Y2,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> E2,
      const std::vector<bool>& converged_filters);

  void ComputeEchoEstimatePerFilterSection(
      const RenderBuffer& render_buffer,
      ArrayView<const std::vector<std::array<float, kFftLengthBy2Plus1>>>
          filter_frequency_responses);

  void ComputeActiveFilterSections();

  const float min_erle_;
  const size_t num_sections_;
  const size_t num_blocks_;
  const size_t delay_headroom_blocks_;
  const std::array<size_t, kFftLengthBy2Plus1> band_to_subband_;
  const std::array<float, kSubbands> max_erle_;
  const std::vector<size_t> section_boundaries_blocks_;
  const bool use_onset_detection_;
  std::vector<std::array<float, kFftLengthBy2Plus1>> erle_;
  std::vector<std::array<float, kFftLengthBy2Plus1>> erle_onset_compensated_;
  std::vector<std::vector<std::array<float, kFftLengthBy2Plus1>>>
      S2_section_accum_;
  std::vector<std::vector<std::array<float, kSubbands>>> erle_estimators_;
  std::vector<std::array<float, kSubbands>> erle_ref_;
  std::vector<std::vector<std::array<float, kSubbands>>> correction_factors_;
  std::vector<std::array<int, kSubbands>> num_updates_;
  std::vector<std::array<size_t, kFftLengthBy2Plus1>> n_active_sections_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_SIGNAL_DEPENDENT_ERLE_ESTIMATOR_H_
