/*  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_EXPAND_UMA_LOGGER_H_
#define MODULES_AUDIO_CODING_NETEQ_EXPAND_UMA_LOGGER_H_

#include <stdint.h>

#include <memory>
#include <optional>
#include <string>

#include "absl/strings/string_view.h"
#include "api/neteq/tick_timer.h"

namespace webrtc {

// This class is used to periodically log values to a UMA histogram. The caller
// is expected to update this class with an incremental sample counter which
// counts expand samples. At the end of each logging period, the class will
// calculate the fraction of samples that were expand samples during that period
// and report that in percent. The logging period must be strictly positive.
// Does not take ownership of tick_timer and the pointer must refer to a valid
// object that outlives the one constructed.
class ExpandUmaLogger {
 public:
  ExpandUmaLogger(absl::string_view uma_name,
                  int logging_period_s,
                  const TickTimer* tick_timer);

  ~ExpandUmaLogger();

  ExpandUmaLogger(const ExpandUmaLogger&) = delete;
  ExpandUmaLogger& operator=(const ExpandUmaLogger&) = delete;

  // In this call, value should be an incremental sample counter. The sample
  // rate must be strictly positive.
  void UpdateSampleCounter(uint64_t value, int sample_rate_hz);

 private:
  const std::string uma_name_;
  const int logging_period_s_;
  const TickTimer& tick_timer_;
  std::unique_ptr<TickTimer::Countdown> timer_;
  std::optional<uint64_t> last_logged_value_;
  uint64_t last_value_ = 0;
  int sample_rate_hz_ = 0;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_EXPAND_UMA_LOGGER_H_
