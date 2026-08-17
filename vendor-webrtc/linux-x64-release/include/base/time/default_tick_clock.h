// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TIME_DEFAULT_TICK_CLOCK_H_
#define BASE_TIME_DEFAULT_TICK_CLOCK_H_

#include "base/base_export.h"
#include "base/time/tick_clock.h"

namespace base {

// DefaultTickClock is a TickClock implementation that uses TimeTicks::Now().
// This is typically used by components that expose a SetTickClockForTesting().
// Note: Overriding Time/TimeTicks altogether via
// TaskEnvironment::TimeSource::MOCK_TIME is now the preferred way of overriding
// time in unit tests. As such, there shouldn't be many new use cases for
// TickClock/DefaultTickClock anymore.
class BASE_EXPORT DefaultTickClock : public TickClock {
 public:
  ~DefaultTickClock() override;

  // Simply returns TimeTicks::Now().
  TimeTicks NowTicks() const override;

  // Returns a shared instance of DefaultTickClock. This is thread-safe.
  static const DefaultTickClock* GetInstance();
};

}  // namespace base

#endif  // BASE_TIME_DEFAULT_TICK_CLOCK_H_
