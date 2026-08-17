/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_ICE_LOGGER_H_
#define LOGGING_RTC_EVENT_LOG_ICE_LOGGER_H_

#include <cstdint>
#include <unordered_map>

#include "logging/rtc_event_log/events/rtc_event_ice_candidate_pair.h"
#include "logging/rtc_event_log/events/rtc_event_ice_candidate_pair_config.h"

namespace webrtc {

class RtcEventLog;

// IceEventLog wraps RtcEventLog and provides structural logging of ICE-specific
// events. The logged events are serialized with other RtcEvent's if protobuf is
// enabled in the build.
class IceEventLog {
 public:
  IceEventLog();
  ~IceEventLog();

  void set_event_log(RtcEventLog* event_log) { event_log_ = event_log; }

  void LogCandidatePairConfig(
      IceCandidatePairConfigType type,
      uint32_t candidate_pair_id,
      const IceCandidatePairDescription& candidate_pair_desc);

  void LogCandidatePairEvent(IceCandidatePairEventType type,
                             uint32_t candidate_pair_id,
                             uint32_t transaction_id);

  // This method constructs a config event for each candidate pair with their
  // description and logs these config events. It is intended to be called when
  // logging starts to ensure that we have at least one config for each
  // candidate pair id.
  void DumpCandidatePairDescriptionToMemoryAsConfigEvents() const;

 private:
  RtcEventLog* event_log_ = nullptr;
  std::unordered_map<uint32_t, IceCandidatePairDescription>
      candidate_pair_desc_by_id_;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_ICE_LOGGER_H_
