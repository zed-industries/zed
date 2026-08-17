/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_UNDERRUN_OPTIMIZER_H_
#define MODULES_AUDIO_CODING_NETEQ_UNDERRUN_OPTIMIZER_H_

#include <memory>
#include <optional>

#include "api/neteq/tick_timer.h"
#include "modules/audio_coding/neteq/histogram.h"

namespace webrtc {

// Estimates probability of buffer underrun due to late packet arrival.
// The optimal delay is decided such that the probability of underrun is lower
// than 1 - `histogram_quantile`.
class UnderrunOptimizer {
 public:
  UnderrunOptimizer(const TickTimer* tick_timer,
                    int histogram_quantile,
                    int forget_factor,
                    std::optional<int> start_forget_weight,
                    std::optional<int> resample_interval_ms);

  void Update(int relative_delay_ms);

  std::optional<int> GetOptimalDelayMs() const { return optimal_delay_ms_; }

  void Reset();

 private:
  const TickTimer* tick_timer_;
  Histogram histogram_;
  const int histogram_quantile_;  // In Q30.
  const std::optional<int> resample_interval_ms_;
  std::unique_ptr<TickTimer::Stopwatch> resample_stopwatch_;
  int max_delay_in_interval_ms_ = 0;
  std::optional<int> optimal_delay_ms_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_UNDERRUN_OPTIMIZER_H_
