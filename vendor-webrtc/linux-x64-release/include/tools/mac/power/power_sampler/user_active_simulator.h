// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_MAC_POWER_POWER_SAMPLER_USER_ACTIVE_SIMULATOR_H_
#define TOOLS_MAC_POWER_POWER_SAMPLER_USER_ACTIVE_SIMULATOR_H_

#include <IOKit/pwr_mgt/IOPMLib.h>

#include "base/timer/timer.h"

namespace power_sampler {

// Pretends that the user is active.
//
// On macOS, scheduling policies change when the user is not active. This class
// simulates user activity to allow running benchmarks with the same scheduling
// policies as when the user is active. When this class is used, the
// UserIdleLevelSampler should report that the "user idle level" is always "0".
class UserActiveSimulator {
 public:
  UserActiveSimulator();
  ~UserActiveSimulator();

  void Start();

 private:
  void OnTimer();

  base::RepeatingTimer timer_;

  // To report continuous user activity, the same id must be provided to each
  // call to IOPMAssertionDeclareUserActivity().
  IOPMAssertionID assertion_id_ = kIOPMNullAssertionID;
};

}  // namespace power_sampler

#endif  // TOOLS_MAC_POWER_POWER_SAMPLER_USER_ACTIVE_SIMULATOR_H_
