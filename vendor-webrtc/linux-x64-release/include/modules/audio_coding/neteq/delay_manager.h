/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_DELAY_MANAGER_H_
#define MODULES_AUDIO_CODING_NETEQ_DELAY_MANAGER_H_

#include <memory>
#include <optional>

#include "api/field_trials_view.h"
#include "api/neteq/tick_timer.h"
#include "modules/audio_coding/neteq/reorder_optimizer.h"
#include "modules/audio_coding/neteq/underrun_optimizer.h"

namespace webrtc {

class DelayManager {
 public:
  struct Config {
    explicit Config(const FieldTrialsView& field_trials);
    void Log();

    // Options that can be configured via field trial.
    double quantile = 0.95;
    double forget_factor = 0.983;
    std::optional<double> start_forget_weight = 2;
    std::optional<int> resample_interval_ms = 500;

    bool use_reorder_optimizer = true;
    double reorder_forget_factor = 0.9993;
    int ms_per_loss_percent = 20;
  };

  DelayManager(const Config& config, const TickTimer* tick_timer);

  virtual ~DelayManager();

  DelayManager(const DelayManager&) = delete;
  DelayManager& operator=(const DelayManager&) = delete;

  // Updates the delay manager that a new packet arrived with delay
  // `arrival_delay_ms`. This updates the statistics and a new target buffer
  // level is calculated. The `reordered` flag indicates if the packet was
  // reordered.
  virtual void Update(int arrival_delay_ms, bool reordered);

  // Resets all state.
  virtual void Reset();

  // Gets the target buffer level in milliseconds. If a minimum or maximum delay
  // has been set, the target delay reported here also respects the configured
  // min/max delay.
  virtual int TargetDelayMs() const;

 private:
  UnderrunOptimizer underrun_optimizer_;
  std::unique_ptr<ReorderOptimizer> reorder_optimizer_;
  int target_level_ms_ = 0;  // Currently preferred buffer level.
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_DELAY_MANAGER_H_
