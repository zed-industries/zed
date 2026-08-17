/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_BEGIN_LOG_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_BEGIN_LOG_H_

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

struct LoggedStartEvent {
  LoggedStartEvent() = default;

  explicit LoggedStartEvent(Timestamp timestamp)
      : LoggedStartEvent(timestamp, timestamp) {}

  LoggedStartEvent(Timestamp timestamp, Timestamp utc_start_time)
      : timestamp(timestamp), utc_start_time(utc_start_time) {}

  int64_t log_time_us() const { return timestamp.us(); }
  int64_t log_time_ms() const { return timestamp.ms(); }
  Timestamp log_time() const { return timestamp; }

  Timestamp utc_time() const { return utc_start_time; }

  Timestamp timestamp = Timestamp::PlusInfinity();
  Timestamp utc_start_time = Timestamp::PlusInfinity();
};

class RtcEventBeginLog final : public RtcEvent {
 public:
  static constexpr Type kType = Type::BeginV3Log;

  RtcEventBeginLog(Timestamp timestamp, Timestamp utc_start_time);
  ~RtcEventBeginLog() override;

  Type GetType() const override { return kType; }
  bool IsConfigEvent() const override { return false; }

  static std::string Encode(ArrayView<const RtcEvent*> batch);

  static RtcEventLogParseStatus Parse(absl::string_view encoded_bytes,
                                      bool batched,
                                      std::vector<LoggedStartEvent>& output);

 private:
  RtcEventBeginLog(const RtcEventBeginLog& other);

  int64_t utc_start_time_ms_;

  static constexpr EventParameters event_params_{"BeginLog",
                                                 RtcEventBeginLog::kType};
  static constexpr FieldParameters utc_start_time_params_{
      "utc_start_time_ms", /*id=*/1, FieldType::kVarInt, /*width=*/64};
};

}  // namespace webrtc
#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_BEGIN_LOG_H_
