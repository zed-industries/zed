/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_ICE_CANDIDATE_PAIR_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_ICE_CANDIDATE_PAIR_H_

#include <stdint.h>

#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "api/units/timestamp.h"
#include "logging/rtc_event_log/events/rtc_event_log_parse_status.h"

namespace webrtc {

enum class IceCandidatePairEventType {
  kCheckSent,
  kCheckReceived,
  kCheckResponseSent,
  kCheckResponseReceived,
  kNumValues,
};

struct LoggedIceCandidatePairEvent {
  LoggedIceCandidatePairEvent() = default;
  LoggedIceCandidatePairEvent(Timestamp timestamp,
                              IceCandidatePairEventType type,
                              uint32_t candidate_pair_id,
                              uint32_t transaction_id)
      : timestamp(timestamp),
        type(type),
        candidate_pair_id(candidate_pair_id),
        transaction_id(transaction_id) {}

  int64_t log_time_us() const { return timestamp.us(); }
  int64_t log_time_ms() const { return timestamp.ms(); }
  Timestamp log_time() const { return timestamp; }

  Timestamp timestamp = Timestamp::MinusInfinity();
  IceCandidatePairEventType type;
  uint32_t candidate_pair_id;
  uint32_t transaction_id;
};

class RtcEventIceCandidatePair final : public RtcEvent {
 public:
  static constexpr Type kType = Type::IceCandidatePairEvent;

  RtcEventIceCandidatePair(IceCandidatePairEventType type,
                           uint32_t candidate_pair_id,
                           uint32_t transaction_id);

  ~RtcEventIceCandidatePair() override;

  Type GetType() const override { return kType; }
  bool IsConfigEvent() const override { return false; }

  std::unique_ptr<RtcEventIceCandidatePair> Copy() const;

  IceCandidatePairEventType type() const { return type_; }
  uint32_t candidate_pair_id() const { return candidate_pair_id_; }
  uint32_t transaction_id() const { return transaction_id_; }

  static std::string Encode(ArrayView<const RtcEvent*> /* batch */) {
    // TODO(terelius): Implement
    return "";
  }

  static RtcEventLogParseStatus Parse(
      absl::string_view /* encoded_bytes */,
      bool /* batched */,
      std::vector<LoggedIceCandidatePairEvent>& /* output */) {
    // TODO(terelius): Implement
    return RtcEventLogParseStatus::Error("Not Implemented", __FILE__, __LINE__);
  }

 private:
  RtcEventIceCandidatePair(const RtcEventIceCandidatePair& other);

  const IceCandidatePairEventType type_;
  const uint32_t candidate_pair_id_;
  const uint32_t transaction_id_;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_ICE_CANDIDATE_PAIR_H_
