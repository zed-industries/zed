/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_BWE_UPDATE_DELAY_BASED_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_BWE_UPDATE_DELAY_BASED_H_

#include <stdint.h>

#include <limits>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "api/transport/bandwidth_usage.h"
#include "api/units/timestamp.h"
#include "logging/rtc_event_log/events/rtc_event_definition.h"
#include "logging/rtc_event_log/events/rtc_event_field_encoding.h"
#include "logging/rtc_event_log/events/rtc_event_field_extraction.h"
#include "logging/rtc_event_log/events/rtc_event_log_parse_status.h"
#include "rtc_base/checks.h"

namespace webrtc {

// Separate the event log encoding from the enum values.
// As long as the enum values are the same as the encodings,
// the two conversion functions can be compiled to (roughly)
// a range check each.
template <>
class RtcEventLogEnum<BandwidthUsage> {
  static constexpr uint64_t kBwNormal = 0;
  static constexpr uint64_t kBwUnderusing = 1;
  static constexpr uint64_t kBwOverusing = 2;

 public:
  static uint64_t Encode(BandwidthUsage x) {
    switch (x) {
      case BandwidthUsage::kBwNormal:
        return kBwNormal;
      case BandwidthUsage::kBwUnderusing:
        return kBwUnderusing;
      case BandwidthUsage::kBwOverusing:
        return kBwOverusing;
      case BandwidthUsage::kLast:
        RTC_DCHECK_NOTREACHED();
    }
    RTC_DCHECK_NOTREACHED();
    return std::numeric_limits<uint64_t>::max();
  }
  static RtcEventLogParseStatusOr<BandwidthUsage> Decode(uint64_t x) {
    switch (x) {
      case kBwNormal:
        return BandwidthUsage::kBwNormal;
      case kBwUnderusing:
        return BandwidthUsage::kBwUnderusing;
      case kBwOverusing:
        return BandwidthUsage::kBwOverusing;
    }
    return RtcEventLogParseStatus::Error("Failed to decode BandwidthUsage enum",
                                         __FILE__, __LINE__);
  }
};

struct LoggedBweDelayBasedUpdate {
  LoggedBweDelayBasedUpdate() = default;
  LoggedBweDelayBasedUpdate(Timestamp timestamp,
                            int32_t bitrate_bps,
                            BandwidthUsage detector_state)
      : timestamp(timestamp),
        bitrate_bps(bitrate_bps),
        detector_state(detector_state) {}

  int64_t log_time_us() const { return timestamp.us(); }
  int64_t log_time_ms() const { return timestamp.ms(); }
  Timestamp log_time() const { return timestamp; }

  Timestamp timestamp = Timestamp::MinusInfinity();
  int32_t bitrate_bps;
  BandwidthUsage detector_state;
};

class RtcEventBweUpdateDelayBased final : public RtcEvent {
 public:
  static constexpr Type kType = Type::BweUpdateDelayBased;

  RtcEventBweUpdateDelayBased(int32_t bitrate_bps,
                              BandwidthUsage detector_state);
  ~RtcEventBweUpdateDelayBased() override;

  Type GetType() const override { return kType; }
  bool IsConfigEvent() const override { return false; }

  std::unique_ptr<RtcEventBweUpdateDelayBased> Copy() const;

  int32_t bitrate_bps() const { return bitrate_bps_; }
  BandwidthUsage detector_state() const { return detector_state_; }

  static std::string Encode(ArrayView<const RtcEvent*> batch) {
    return RtcEventBweUpdateDelayBased::definition_.EncodeBatch(batch);
  }

  static RtcEventLogParseStatus Parse(
      absl::string_view encoded_bytes,
      bool batched,
      std::vector<LoggedBweDelayBasedUpdate>& output) {
    return RtcEventBweUpdateDelayBased::definition_.ParseBatch(encoded_bytes,
                                                               batched, output);
  }

 private:
  RtcEventBweUpdateDelayBased(const RtcEventBweUpdateDelayBased& other);

  const int32_t bitrate_bps_;
  const BandwidthUsage detector_state_;

  static constexpr RtcEventDefinition<RtcEventBweUpdateDelayBased,
                                      LoggedBweDelayBasedUpdate,
                                      int32_t,
                                      BandwidthUsage>
      definition_{
          {"BweDelayBased", RtcEventBweUpdateDelayBased::kType},
          {&RtcEventBweUpdateDelayBased::bitrate_bps_,
           &LoggedBweDelayBasedUpdate::bitrate_bps,
           {"bitrate_bps", /*id=*/1, FieldType::kVarInt, /*width=*/32}},
          {&RtcEventBweUpdateDelayBased::detector_state_,
           &LoggedBweDelayBasedUpdate::detector_state,
           {"detector_state", /*id=*/2, FieldType::kVarInt, /*width=*/64}}};
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_BWE_UPDATE_DELAY_BASED_H_
