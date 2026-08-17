/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_REMOTE_ESTIMATE_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_REMOTE_ESTIMATE_H_

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "api/units/data_rate.h"
#include "api/units/timestamp.h"
#include "logging/rtc_event_log/events/rtc_event_log_parse_status.h"

namespace webrtc {

struct LoggedRemoteEstimateEvent {
  LoggedRemoteEstimateEvent() = default;

  int64_t log_time_us() const { return timestamp.us(); }
  int64_t log_time_ms() const { return timestamp.ms(); }
  Timestamp log_time() const { return timestamp; }

  Timestamp timestamp = Timestamp::MinusInfinity();
  std::optional<DataRate> link_capacity_lower;
  std::optional<DataRate> link_capacity_upper;
};

class RtcEventRemoteEstimate final : public RtcEvent {
 public:
  static constexpr Type kType = Type::RemoteEstimateEvent;

  RtcEventRemoteEstimate(DataRate link_capacity_lower,
                         DataRate link_capacity_upper)
      : link_capacity_lower_(link_capacity_lower),
        link_capacity_upper_(link_capacity_upper) {}

  Type GetType() const override { return kType; }
  bool IsConfigEvent() const override { return false; }

  static std::string Encode(ArrayView<const RtcEvent*> /* batch */) {
    // TODO(terelius): Implement
    return "";
  }

  static RtcEventLogParseStatus Parse(
      absl::string_view /* encoded_bytes */,
      bool /* batched */,
      std::vector<LoggedRemoteEstimateEvent>& /* output */) {
    // TODO(terelius): Implement
    return RtcEventLogParseStatus::Error("Not Implemented", __FILE__, __LINE__);
  }

  const DataRate link_capacity_lower_;
  const DataRate link_capacity_upper_;
};

}  // namespace webrtc
#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_REMOTE_ESTIMATE_H_
