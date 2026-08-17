/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_MATCHED_FILTER_H_
#define MODULES_AUDIO_PROCESSING_AEC3_MATCHED_FILTER_H_

#include <stddef.h>

#include <optional>
#include <vector>

#include "api/array_view.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "rtc_base/gtest_prod_util.h"
#include "rtc_base/system/arch.h"

namespace webrtc {

class ApmDataDumper;
struct DownsampledRenderBuffer;

namespace aec3 {

#if defined(WEBRTC_HAS_NEON)

// Filter core for the matched filter that is optimized for NEON.
void MatchedFilterCore_NEON(size_t x_start_index,
                            float x2_sum_threshold,
                            float smoothing,
                            webrtc::ArrayView<const float> x,
                            webrtc::ArrayView<const float> y,
                            webrtc::ArrayView<float> h,
                            bool* filters_updated,
                            float* error_sum,
                            bool compute_accumulation_error,
                            webrtc::ArrayView<float> accumulated_error,
                            webrtc::ArrayView<float> scratch_memory);

#endif

#if defined(WEBRTC_ARCH_X86_FAMILY)

// Filter core for the matched filter that is optimized for SSE2.
void MatchedFilterCore_SSE2(size_t x_start_index,
                            float x2_sum_threshold,
                            float smoothing,
                            ArrayView<const float> x,
                            ArrayView<const float> y,
                            ArrayView<float> h,
                            bool* filters_updated,
                            float* error_sum,
                            bool compute_accumulated_error,
                            ArrayView<float> accumulated_error,
                            ArrayView<float> scratch_memory);

// Filter core for the matched filter that is optimized for AVX2.
void MatchedFilterCore_AVX2(size_t x_start_index,
                            float x2_sum_threshold,
                            float smoothing,
                            ArrayView<const float> x,
                            ArrayView<const float> y,
                            ArrayView<float> h,
                            bool* filters_updated,
                            float* error_sum,
                            bool compute_accumulated_error,
                            ArrayView<float> accumulated_error,
                            ArrayView<float> scratch_memory);

#endif

// Filter core for the matched filter.
void MatchedFilterCore(size_t x_start_index,
                       float x2_sum_threshold,
                       float smoothing,
                       ArrayView<const float> x,
                       ArrayView<const float> y,
                       ArrayView<float> h,
                       bool* filters_updated,
                       float* error_sum,
                       bool compute_accumulation_error,
                       ArrayView<float> accumulated_error);

// Find largest peak of squared values in array.
size_t MaxSquarePeakIndex(ArrayView<const float> h);

}  // namespace aec3

// Produces recursively updated cross-correlation estimates for several signal
// shifts where the intra-shift spacing is uniform.
class MatchedFilter {
 public:
  // Stores properties for the lag estimate corresponding to a particular signal
  // shift.
  struct LagEstimate {
    LagEstimate() = default;
    LagEstimate(size_t lag, size_t pre_echo_lag)
        : lag(lag), pre_echo_lag(pre_echo_lag) {}
    size_t lag = 0;
    size_t pre_echo_lag = 0;
  };

  MatchedFilter(ApmDataDumper* data_dumper,
                Aec3Optimization optimization,
                size_t sub_block_size,
                size_t window_size_sub_blocks,
                int num_matched_filters,
                size_t alignment_shift_sub_blocks,
                float excitation_limit,
                float smoothing_fast,
                float smoothing_slow,
                float matching_filter_threshold,
                bool detect_pre_echo);

  MatchedFilter() = delete;
  MatchedFilter(const MatchedFilter&) = delete;
  MatchedFilter& operator=(const MatchedFilter&) = delete;

  ~MatchedFilter();

  // Updates the correlation with the values in the capture buffer.
  void Update(const DownsampledRenderBuffer& render_buffer,
              ArrayView<const float> capture,
              bool use_slow_smoothing);

  // Resets the matched filter.
  void Reset(bool full_reset);

  // Returns the current lag estimates.
  std::optional<const MatchedFilter::LagEstimate> GetBestLagEstimate() const {
    return reported_lag_estimate_;
  }

  // Returns the maximum filter lag.
  size_t GetMaxFilterLag() const {
    return filters_.size() * filter_intra_lag_shift_ + filters_[0].size();
  }

  // Log matched filter properties.
  void LogFilterProperties(int sample_rate_hz,
                           size_t shift,
                           size_t downsampling_factor) const;

 private:
  void Dump();

  ApmDataDumper* const data_dumper_;
  const Aec3Optimization optimization_;
  const size_t sub_block_size_;
  const size_t filter_intra_lag_shift_;
  std::vector<std::vector<float>> filters_;
  std::vector<std::vector<float>> accumulated_error_;
  std::vector<float> instantaneous_accumulated_error_;
  std::vector<float> scratch_memory_;
  std::optional<MatchedFilter::LagEstimate> reported_lag_estimate_;
  std::optional<size_t> winner_lag_;
  int last_detected_best_lag_filter_ = -1;
  std::vector<size_t> filters_offsets_;
  int number_pre_echo_updates_ = 0;
  const float excitation_limit_;
  const float smoothing_fast_;
  const float smoothing_slow_;
  const float matching_filter_threshold_;
  const bool detect_pre_echo_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_MATCHED_FILTER_H_
