/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_ALR_STATE_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_ALR_STATE_H_

#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "api/units/timestamp.h"
#include "logging/rtc_event_log/events/rtc_event_definition.h"
#include "logging/rtc_event_log/events/rtc_event_field_encoding.h"
#include "logging/rtc_event_log/events/rtc_event_log_parse_status.h"

namespace webrtc {

struct LoggedAlrStateEvent {
  LoggedAlrStateEvent() = default;
  LoggedAlrStateEvent(Timestamp timestamp, bool in_alr)
      : timestamp(timestamp), in_alr(in_alr) {}

  int64_t log_time_us() const { return timestamp.us(); }
  int64_t log_time_ms() const { return timestamp.ms(); }
  Timestamp log_time() const { return timestamp; }

  Timestamp timestamp = Timestamp::MinusInfinity();
  bool in_alr;
};

class RtcEventAlrState final : public RtcEvent {
 public:
  static constexpr Type kType = Type::AlrStateEvent;

  explicit RtcEventAlrState(bool in_alr);
  ~RtcEventAlrState() override;

  Type GetType() const override { return kType; }
  bool IsConfigEvent() const override { return false; }

  std::unique_ptr<RtcEventAlrState> Copy() const;

  bool in_alr() const { return in_alr_; }

  static std::string Encode(ArrayView<const RtcEvent*> batch) {
    return RtcEventAlrState::definition_.EncodeBatch(batch);
  }

  static RtcEventLogParseStatus Parse(absl::string_view s,
                                      bool batched,
                                      std::vector<LoggedAlrStateEvent>& output);

 private:
  RtcEventAlrState(const RtcEventAlrState& other);

  const bool in_alr_;

  static constexpr RtcEventDefinition<RtcEventAlrState,
                                      LoggedAlrStateEvent,
                                      bool>
      definition_{{"AlrState", RtcEventAlrState::kType},
                  {&RtcEventAlrState::in_alr_,
                   &LoggedAlrStateEvent::in_alr,
                   {"in_alr", /*id=*/1, FieldType::kFixed8, /*width=*/1}}};
};

}  // namespace webrtc
#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_ALR_STATE_H_
