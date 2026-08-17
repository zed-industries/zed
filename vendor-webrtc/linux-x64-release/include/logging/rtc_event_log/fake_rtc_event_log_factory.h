/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_FAKE_RTC_EVENT_LOG_FACTORY_H_
#define LOGGING_RTC_EVENT_LOG_FAKE_RTC_EVENT_LOG_FACTORY_H_

#include <memory>

#include "absl/base/nullability.h"
#include "api/environment/environment.h"
#include "api/rtc_event_log/rtc_event_log_factory_interface.h"
#include "logging/rtc_event_log/fake_rtc_event_log.h"

namespace webrtc {

class FakeRtcEventLogFactory : public RtcEventLogFactoryInterface {
 public:
  FakeRtcEventLogFactory() = default;
  ~FakeRtcEventLogFactory() override = default;

  absl_nonnull std::unique_ptr<RtcEventLog> Create(
      const Environment& env) const override;

  FakeRtcEventLog* last_log_created() { return last_log_created_; }

 private:
  FakeRtcEventLog* last_log_created_ = nullptr;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_FAKE_RTC_EVENT_LOG_FACTORY_H_
