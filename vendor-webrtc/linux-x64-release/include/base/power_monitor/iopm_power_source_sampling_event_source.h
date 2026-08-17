// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_IOPM_POWER_SOURCE_SAMPLING_EVENT_SOURCE_H_
#define BASE_POWER_MONITOR_IOPM_POWER_SOURCE_SAMPLING_EVENT_SOURCE_H_

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/mac/scoped_ionotificationportref.h"
#include "base/mac/scoped_ioobject.h"
#include "base/power_monitor/sampling_event_source.h"

namespace base {

// Generates a sampling event when a state change notification is dispatched by
// the IOPMPowerSource service.
class BASE_EXPORT IOPMPowerSourceSamplingEventSource
    : public SamplingEventSource {
 public:
  IOPMPowerSourceSamplingEventSource();

  ~IOPMPowerSourceSamplingEventSource() override;

  // SamplingEventSource:
  bool Start(SamplingEventCallback callback) override;
  TimeDelta GetSampleInterval() override;

 private:
  static void OnNotification(void* context,
                             io_service_t service,
                             natural_t message_type,
                             void* message_argument);

  mac::ScopedIONotificationPortRef notify_port_;
  mac::ScopedIOObject<io_service_t> service_;
  mac::ScopedIOObject<io_object_t> notification_;
  SamplingEventCallback callback_;
};

}  // namespace base

#endif  // BASE_POWER_MONITOR_IOPM_POWER_SOURCE_SAMPLING_EVENT_SOURCE_H_
