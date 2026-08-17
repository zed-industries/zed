// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_NAVIGATION_TIMING_ACTIVATION_START_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_NAVIGATION_TIMING_ACTIVATION_START_H_

#include "third_party/blink/renderer/core/timing/performance_navigation_timing.h"

namespace blink {

class PerformanceNavigationTimingActivationStart final {
 public:
  static DOMHighResTimeStamp activationStart(
      const PerformanceNavigationTiming& performance_navigation_timing);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_NAVIGATION_TIMING_ACTIVATION_START_H_
