// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_RAIL_MODE_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_RAIL_MODE_OBSERVER_H_

namespace blink {

// A subset of RAIL mode as defined in [1].
// [1] https://developers.google.com/web/fundamentals/performance/rail
enum class RAILMode {
  // Covers all modes except Load.
  kDefault,
  kLoad,
};

class RAILModeObserver {
 public:
  virtual ~RAILModeObserver() = default;
  virtual void OnRAILModeChanged(RAILMode rail_mode) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_RAIL_MODE_OBSERVER_H_
