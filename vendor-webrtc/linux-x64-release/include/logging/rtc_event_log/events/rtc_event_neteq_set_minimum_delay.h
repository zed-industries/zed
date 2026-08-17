/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_NETEQ_SET_MINIMUM_DELAY_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_NETEQ_SET_MINIMUM_DELAY_H_

#include <stdint.h>

#include <memory>

#include "absl/memory/memory.h"
#include "api/rtc_event_log/rtc_event.h"
#include "api/units/timestamp.h"

namespace webrtc {

struct LoggedNetEqSetMinimumDelayEvent {
  LoggedNetEqSetMinimumDelayEvent() = default;
  LoggedNetEqSetMinimumDelayEvent(Timestamp timestamp,
                                  uint32_t remote_ssrc,
                                  int minimum_delay_ms)
      : timestamp(timestamp),
        remote_ssrc(remote_ssrc),
        minimum_delay_ms(minimum_delay_ms) {}

  int64_t log_time_us() const { return timestamp.us(); }
  int64_t log_time_ms() const { return timestamp.ms(); }
  Timestamp log_time() const { return timestamp; }

  Timestamp timestamp = Timestamp::MinusInfinity();
  uint32_t remote_ssrc;
  int minimum_delay_ms;
};

class RtcEventNetEqSetMinimumDelay final : public RtcEvent {
 public:
  static constexpr Type kType = Type::NetEqSetMinimumDelay;

  explicit RtcEventNetEqSetMinimumDelay(uint32_t remote_ssrc, int delay_ms);
  ~RtcEventNetEqSetMinimumDelay() override;

  Type GetType() const override { return kType; }
  bool IsConfigEvent() const override { return false; }
  uint32_t remote_ssrc() const { return remote_ssrc_; }
  int minimum_delay_ms() const { return minimum_delay_ms_; }

  std::unique_ptr<RtcEventNetEqSetMinimumDelay> Copy() const {
    return absl::WrapUnique<RtcEventNetEqSetMinimumDelay>(
        new RtcEventNetEqSetMinimumDelay(*this));
  }

 private:
  uint32_t remote_ssrc_;
  int minimum_delay_ms_;
};

}  // namespace webrtc
#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_NETEQ_SET_MINIMUM_DELAY_H_
