// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_SAMPLING_EVENT_SOURCE_H_
#define BASE_POWER_MONITOR_SAMPLING_EVENT_SOURCE_H_

#include "base/base_export.h"
#include "base/functional/callback_forward.h"
#include "base/time/time.h"

namespace base {

// Invokes a callback when a Sample should be requested from all Samplers.
class BASE_EXPORT SamplingEventSource {
 public:
  using SamplingEventCallback = RepeatingClosure;

  virtual ~SamplingEventSource() = 0;

  // Starts generating sampling events. Returns whether the operation succeeded.
  // |callback| is invoked for every sampling event.
  virtual bool Start(SamplingEventCallback callback) = 0;

  // Returns the expected time between each call to the SamplingEventCallback.
  virtual base::TimeDelta GetSampleInterval() = 0;
};

}  // namespace base

#endif  // BASE_POWER_MONITOR_SAMPLING_EVENT_SOURCE_H_
