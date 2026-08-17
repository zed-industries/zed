/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_SUBBAND_ERLE_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_AEC3_SUBBAND_ERLE_ESTIMATOR_H_

#include <stddef.h>

#include <array>
#include <memory>
#include <vector>

#include "api/array_view.h"
#include "api/audio/echo_canceller3_config.h"
#include "api/environment/environment.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/logging/apm_data_dumper.h"

namespace webrtc {

// Estimates the echo return loss enhancement for each frequency subband.
class SubbandErleEstimator {
 public:
  SubbandErleEstimator(const Environment& env,
                       const EchoCanceller3Config& config,
                       size_t num_capture_channels);
  ~SubbandErleEstimator();

  // Resets the ERLE estimator.
  void Reset();

  // Updates the ERLE estimate.
  void Update(ArrayView<const float, kFftLengthBy2Plus1> X2,
              ArrayView<const std::array<float, kFftLengthBy2Plus1>> Y2,
              ArrayView<const std::array<float, kFftLengthBy2Plus1>> E2,
              const std::vector<bool>& converged_filters);

  // Returns the ERLE estimate.
  ArrayView<const std::array<float, kFftLengthBy2Plus1>> Erle(
      bool onset_compensated) const {
    return onset_compensated && use_onset_detection_ ? erle_onset_compensated_
                                                     : erle_;
  }

  // Returns the non-capped ERLE estimate.
  ArrayView<const std::array<float, kFftLengthBy2Plus1>> ErleUnbounded() const {
    return erle_unbounded_;
  }

  // Returns the ERLE estimate at onsets (only used for testing).
  ArrayView<const std::array<float, kFftLengthBy2Plus1>> ErleDuringOnsets()
      const {
    return erle_during_onsets_;
  }

  void Dump(const std::unique_ptr<ApmDataDumper>& data_dumper) const;

 private:
  struct AccumulatedSpectra {
    explicit AccumulatedSpectra(size_t num_capture_channels)
        : Y2(num_capture_channels),
          E2(num_capture_channels),
          low_render_energy(num_capture_channels),
          num_points(num_capture_channels) {}
    std::vector<std::array<float, kFftLengthBy2Plus1>> Y2;
    std::vector<std::array<float, kFftLengthBy2Plus1>> E2;
    std::vector<std::array<bool, kFftLengthBy2Plus1>> low_render_energy;
    std::vector<int> num_points;
  };

  void UpdateAccumulatedSpectra(
      ArrayView<const float, kFftLengthBy2Plus1> X2,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> Y2,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> E2,
      const std::vector<bool>& converged_filters);

  void ResetAccumulatedSpectra();

  void UpdateBands(const std::vector<bool>& converged_filters);
  void DecreaseErlePerBandForLowRenderSignals();

  const bool use_onset_detection_;
  const float min_erle_;
  const std::array<float, kFftLengthBy2Plus1> max_erle_;
  const bool use_min_erle_during_onsets_;
  AccumulatedSpectra accum_spectra_;
  // ERLE without special handling of render onsets.
  std::vector<std::array<float, kFftLengthBy2Plus1>> erle_;
  // ERLE lowered during render onsets.
  std::vector<std::array<float, kFftLengthBy2Plus1>> erle_onset_compensated_;
  std::vector<std::array<float, kFftLengthBy2Plus1>> erle_unbounded_;
  // Estimation of ERLE during render onsets.
  std::vector<std::array<float, kFftLengthBy2Plus1>> erle_during_onsets_;
  std::vector<std::array<bool, kFftLengthBy2Plus1>> coming_onset_;
  std::vector<std::array<int, kFftLengthBy2Plus1>> hold_counters_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_SUBBAND_ERLE_ESTIMATOR_H_
