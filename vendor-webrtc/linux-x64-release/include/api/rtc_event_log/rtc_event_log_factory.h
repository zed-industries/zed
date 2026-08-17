/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_RTC_EVENT_LOG_RTC_EVENT_LOG_FACTORY_H_
#define API_RTC_EVENT_LOG_RTC_EVENT_LOG_FACTORY_H_

#include <memory>

#include "absl/base/nullability.h"
#include "api/environment/environment.h"
#include "api/rtc_event_log/rtc_event_log.h"
#include "api/rtc_event_log/rtc_event_log_factory_interface.h"
#include "api/task_queue/task_queue_factory.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class RTC_EXPORT RtcEventLogFactory : public RtcEventLogFactoryInterface {
 public:
  RtcEventLogFactory() = default;

  [[deprecated("Use default constructor")]]  //
  explicit RtcEventLogFactory(TaskQueueFactory* /* task_queue_factory */) {}

  ~RtcEventLogFactory() override = default;

  absl_nonnull std::unique_ptr<RtcEventLog> Create(
      const Environment& env) const override;
};

}  // namespace webrtc

#endif  // API_RTC_EVENT_LOG_RTC_EVENT_LOG_FACTORY_H_
