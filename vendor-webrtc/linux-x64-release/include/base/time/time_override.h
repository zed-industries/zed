// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TIME_TIME_OVERRIDE_H_
#define BASE_TIME_TIME_OVERRIDE_H_

#include <atomic>
#include <optional>

#include "base/base_export.h"
#include "base/time/time.h"
#include "build/build_config.h"

namespace base {

using TimeNowFunction = decltype(&Time::Now);
using TimeTicksNowFunction = decltype(&TimeTicks::Now);
using TimeTicksLowResolutionNowFunction =
    decltype(&TimeTicks::LowResolutionNow);
using LiveTicksNowFunction = decltype(&LiveTicks::Now);
using ThreadTicksNowFunction = decltype(&ThreadTicks::Now);

// Time overrides should be used with extreme caution. Discuss with //base/time
// OWNERS before adding a new one.
namespace subtle {

// Override the return value of Time::Now and Time::NowFromSystemTime /
// TimeTicks::Now / LiveTicks::Now / ThreadTicks::Now to emulate time, e.g. for
// tests or to modify progression of time. It is recommended that the override
// be set while single-threaded and before the first call to Now() to avoid
// threading issues and inconsistencies in returned values. Overriding time
// while other threads are running is very subtle and should be reserved for
// developer only use cases (e.g. virtual time in devtools) where any flakiness
// caused by a racy time update isn't surprising. Instantiating a
// ScopedTimeClockOverrides while other threads are running might break their
// expectation that TimeTicks and ThreadTicks increase monotonically. Nested
// overrides are not allowed.
class BASE_EXPORT ScopedTimeClockOverrides {
 public:
  // Pass |nullptr| for any override if it shouldn't be overriden.
  ScopedTimeClockOverrides(TimeNowFunction time_override,
                           TimeTicksNowFunction time_ticks_override,
                           ThreadTicksNowFunction thread_ticks_override,
                           LiveTicksNowFunction live_ticks_override = nullptr,
                           TimeTicksLowResolutionNowFunction
                               time_ticks_low_resolution_override = nullptr);

  ScopedTimeClockOverrides(const ScopedTimeClockOverrides&) = delete;
  ScopedTimeClockOverrides& operator=(const ScopedTimeClockOverrides&) = delete;

  // Restores the platform default Now() functions.
  ~ScopedTimeClockOverrides();

  static bool overrides_active() { return overrides_active_; }

 private:
  static bool overrides_active_;
};

// These methods return the platform default Time::Now / TimeTicks::Now /
// ThreadTicks::Now values even while an override is in place. These methods
// should only be used in places where emulated time should be disregarded. For
// example, they can be used to implement test timeouts for tests that may
// override time.
BASE_EXPORT Time TimeNowIgnoringOverride();
BASE_EXPORT Time TimeNowFromSystemTimeIgnoringOverride();
BASE_EXPORT TimeTicks TimeTicksNowIgnoringOverride();
BASE_EXPORT LiveTicks LiveTicksNowIgnoringOverride();
BASE_EXPORT ThreadTicks ThreadTicksNowIgnoringOverride();
BASE_EXPORT TimeTicks TimeTicksLowResolutionNowIgnoringOverride();

#if BUILDFLAG(IS_POSIX)
// Equivalent to TimeTicksNowIgnoringOverride(), but is allowed to fail and
// return std::nullopt. This may safely be used in a signal handler.
BASE_EXPORT std::optional<TimeTicks> MaybeTimeTicksNowIgnoringOverride();
#endif

}  // namespace subtle

namespace internal {

// These function pointers are used by platform-independent implementations of
// the Now() methods and ScopedTimeClockOverrides. They are set to point to the
// respective NowIgnoringOverride functions by default, but can also be set by
// platform-specific code to select a default implementation at runtime, thereby
// avoiding the indirection via the NowIgnoringOverride functions. Note that the
// pointers can be overridden and later reset to the NowIgnoringOverride
// functions by ScopedTimeClockOverrides.
extern std::atomic<TimeNowFunction> g_time_now_function;
extern std::atomic<TimeNowFunction> g_time_now_from_system_time_function;
extern std::atomic<TimeTicksNowFunction> g_time_ticks_now_function;
extern std::atomic<TimeTicksNowFunction>
    g_time_ticks_low_resolution_now_function;
extern std::atomic<LiveTicksNowFunction> g_live_ticks_now_function;
extern std::atomic<ThreadTicksNowFunction> g_thread_ticks_now_function;

}  // namespace internal

}  // namespace base

#endif  // BASE_TIME_TIME_OVERRIDE_H_
