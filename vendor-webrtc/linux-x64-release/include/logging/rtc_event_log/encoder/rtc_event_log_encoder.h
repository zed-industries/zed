/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_H_
#define LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_H_

#include <cstdint>
#include <deque>
#include <memory>
#include <string>

#include "api/rtc_event_log/rtc_event.h"

namespace webrtc {
class RtcEventLogEncoder {
 public:
  virtual ~RtcEventLogEncoder() = default;

  virtual std::string EncodeLogStart(int64_t timestamp_us,
                                     int64_t utc_time_us) = 0;
  virtual std::string EncodeLogEnd(int64_t timestamp_us) = 0;

  virtual std::string EncodeBatch(
      std::deque<std::unique_ptr<RtcEvent>>::const_iterator begin,
      std::deque<std::unique_ptr<RtcEvent>>::const_iterator end) = 0;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_ENCODER_RTC_EVENT_LOG_ENCODER_H_
