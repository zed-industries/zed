/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_END_LOG_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_END_LOG_H_

#include <cstdint>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "api/units/timestamp.h"
#include "logging/rtc_event_log/events/rtc_event_field_encoding.h"
#include "logging/rtc_event_log/events/rtc_event_log_parse_status.h"

namespace webrtc {

struct LoggedStopEvent {
  LoggedStopEvent() = default;

  explicit LoggedStopEvent(Timestamp timestamp) : timestamp(timestamp) {}

  int64_t log_time_us() const { return timestamp.us(); }
  int64_t log_time_ms() const { return timestamp.ms(); }
  Timestamp log_time() const { return timestamp; }

  Timestamp timestamp = Timestamp::PlusInfinity();
};

class RtcEventEndLog final : public RtcEvent {
 public:
  static constexpr Type kType = Type::EndV3Log;

  explicit RtcEventEndLog(Timestamp timestamp);
  ~RtcEventEndLog() override;

  Type GetType() const override { return kType; }
  bool IsConfigEvent() const override { return false; }

  static std::string Encode(ArrayView<const RtcEvent*> batch);

  static RtcEventLogParseStatus Parse(absl::string_view encoded_bytes,
                                      bool batched,
                                      std::vector<LoggedStopEvent>& output);

 private:
  RtcEventEndLog(const RtcEventEndLog& other);

  static constexpr EventParameters event_params_{"EndLog",
                                                 RtcEventEndLog::kType};
};

}  // namespace webrtc
#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_END_LOG_H_
