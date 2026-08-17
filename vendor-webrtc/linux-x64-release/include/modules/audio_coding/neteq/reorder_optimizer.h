/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_REORDER_OPTIMIZER_H_
#define MODULES_AUDIO_CODING_NETEQ_REORDER_OPTIMIZER_H_

#include <optional>

#include "modules/audio_coding/neteq/histogram.h"

namespace webrtc {

// Calculates an optimal delay to reduce the chance of missing reordered
// packets. The delay/loss trade-off can be tune using the `ms_per_loss_percent`
// parameter.
class ReorderOptimizer {
 public:
  ReorderOptimizer(int forget_factor,
                   int ms_per_loss_percent,
                   std::optional<int> start_forget_weight);

  void Update(int relative_delay_ms, bool reordered, int base_delay_ms);

  std::optional<int> GetOptimalDelayMs() const { return optimal_delay_ms_; }

  void Reset();

 private:
  int MinimizeCostFunction(int base_delay_ms) const;

  Histogram histogram_;
  const int ms_per_loss_percent_;
  std::optional<int> optimal_delay_ms_;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_REORDER_OPTIMIZER_H_
