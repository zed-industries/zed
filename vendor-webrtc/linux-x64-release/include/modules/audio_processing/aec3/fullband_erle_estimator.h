/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_FULLBAND_ERLE_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_AEC3_FULLBAND_ERLE_ESTIMATOR_H_

#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/audio/echo_canceller3_config.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/logging/apm_data_dumper.h"

namespace webrtc {

// Estimates the echo return loss enhancement using the energy of all the
// freuquency bands.
class FullBandErleEstimator {
 public:
  FullBandErleEstimator(const EchoCanceller3Config::Erle& config,
                        size_t num_capture_channels);
  ~FullBandErleEstimator();
  // Resets the ERLE estimator.
  void Reset();

  // Updates the ERLE estimator.
  void Update(ArrayView<const float> X2,
              ArrayView<const std::array<float, kFftLengthBy2Plus1>> Y2,
              ArrayView<const std::array<float, kFftLengthBy2Plus1>> E2,
              const std::vector<bool>& converged_filters);

  // Returns the fullband ERLE estimates in log2 units.
  float FullbandErleLog2() const {
    float min_erle = erle_time_domain_log2_[0];
    for (size_t ch = 1; ch < erle_time_domain_log2_.size(); ++ch) {
      min_erle = std::min(min_erle, erle_time_domain_log2_[ch]);
    }
    return min_erle;
  }

  // Returns an estimation of the current linear filter quality. It returns a
  // float number between 0 and 1 mapping 1 to the highest possible quality.
  ArrayView<const std::optional<float>> GetInstLinearQualityEstimates() const {
    return linear_filters_qualities_;
  }

  void Dump(const std::unique_ptr<ApmDataDumper>& data_dumper) const;

 private:
  void UpdateQualityEstimates();

  class ErleInstantaneous {
   public:
    explicit ErleInstantaneous(const EchoCanceller3Config::Erle& config);
    ~ErleInstantaneous();

    // Updates the estimator with a new point, returns true
    // if the instantaneous ERLE was updated due to having enough
    // points for performing the estimate.
    bool Update(float Y2_sum, float E2_sum);
    // Resets the instantaneous ERLE estimator to its initial state.
    void Reset();
    // Resets the members related with an instantaneous estimate.
    void ResetAccumulators();
    // Returns the instantaneous ERLE in log2 units.
    std::optional<float> GetInstErleLog2() const { return erle_log2_; }
    // Gets an indication between 0 and 1 of the performance of the linear
    // filter for the current time instant.
    std::optional<float> GetQualityEstimate() const {
      if (erle_log2_) {
        float value = inst_quality_estimate_;
        if (clamp_inst_quality_to_zero_) {
          value = std::max(0.f, value);
        }
        if (clamp_inst_quality_to_one_) {
          value = std::min(1.f, value);
        }
        return std::optional<float>(value);
      }
      return std::nullopt;
    }
    void Dump(const std::unique_ptr<ApmDataDumper>& data_dumper) const;

   private:
    void UpdateMaxMin();
    void UpdateQualityEstimate();
    const bool clamp_inst_quality_to_zero_;
    const bool clamp_inst_quality_to_one_;
    std::optional<float> erle_log2_;
    float inst_quality_estimate_;
    float max_erle_log2_;
    float min_erle_log2_;
    float Y2_acum_;
    float E2_acum_;
    int num_points_;
  };

  const float min_erle_log2_;
  const float max_erle_lf_log2_;
  std::vector<int> hold_counters_instantaneous_erle_;
  std::vector<float> erle_time_domain_log2_;
  std::vector<ErleInstantaneous> instantaneous_erle_;
  std::vector<std::optional<float>> linear_filters_qualities_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_FULLBAND_ERLE_ESTIMATOR_H_
