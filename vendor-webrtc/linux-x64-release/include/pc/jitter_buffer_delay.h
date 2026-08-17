/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_JITTER_BUFFER_DELAY_H_
#define PC_JITTER_BUFFER_DELAY_H_

#include <stdint.h>

#include <optional>

#include "api/sequence_checker.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// JitterBufferDelay converts delay from seconds to milliseconds for the
// underlying media channel. It also handles cases when user sets delay before
// the start of media_channel by caching its request.
class JitterBufferDelay {
 public:
  JitterBufferDelay() = default;

  void Set(std::optional<double> delay_seconds);
  int GetMs() const;

 private:
  RTC_NO_UNIQUE_ADDRESS SequenceChecker worker_thread_checker_{
      SequenceChecker::kDetached};
  std::optional<double> cached_delay_seconds_
      RTC_GUARDED_BY(&worker_thread_checker_);
};

}  // namespace webrtc

#endif  // PC_JITTER_BUFFER_DELAY_H_
