/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_EVENT_H_
#define RTC_BASE_EVENT_H_

#include "api/units/time_delta.h"

#if defined(WEBRTC_WIN)
#include <windows.h>
#elif defined(WEBRTC_POSIX)
#include <pthread.h>
#else
#error "Must define either WEBRTC_WIN or WEBRTC_POSIX."
#endif

#include "rtc_base/synchronization/yield_policy.h"

namespace webrtc {

// RTC_DISALLOW_WAIT() utility
//
// Sets a stack-scoped flag that disallows use of `webrtc::Event::Wait` by means
// of raising a DCHECK when a call to `webrtc::Event::Wait()` is made..
// This is useful to guard synchronization-free scopes against regressions.
//
// Example of what this would catch (`ScopeToProtect` calls `Foo`):
//
//  void Foo(TaskQueue* tq) {
//    Event event;
//    tq->PostTask([&event]() {
//      event.Set();
//    });
//    event.Wait(Event::kForever);  // <- Will trigger a DCHECK.
//  }
//
//  void ScopeToProtect() {
//    TaskQueue* tq = GetSomeTaskQueue();
//    RTC_DISALLOW_WAIT();  // Policy takes effect.
//    Foo(tq);
//  }
//
#if RTC_DCHECK_IS_ON
#define RTC_DISALLOW_WAIT() ScopedDisallowWait disallow_wait_##__LINE__
#else
#define RTC_DISALLOW_WAIT()
#endif

class Event {
 public:
  // TODO(bugs.webrtc.org/14366): Consider removing this redundant alias.
  static constexpr TimeDelta kForever = TimeDelta::PlusInfinity();
  static constexpr TimeDelta kDefaultWarnDuration = TimeDelta::Seconds(3);

  Event();
  Event(bool manual_reset, bool initially_signaled);
  Event(const Event&) = delete;
  Event& operator=(const Event&) = delete;
  ~Event();

  void Set();
  void Reset();

  // Waits for the event to become signaled, but logs a warning if it takes more
  // than `warn_after`, and gives up completely if it takes more than
  // `give_up_after`. (If `warn_after >= give_up_after`, no warning will be
  // logged.) Either or both may be `kForever`, which means wait indefinitely.
  //
  // Care is taken so that the underlying OS wait call isn't requested to sleep
  // shorter than `give_up_after`.
  //
  // Returns true if the event was signaled, false if there was a timeout or
  // some other error.
  bool Wait(TimeDelta give_up_after, TimeDelta warn_after);

  // Waits with the given timeout and a reasonable default warning timeout.
  bool Wait(TimeDelta give_up_after) {
    return Wait(give_up_after, give_up_after.IsPlusInfinity()
                                   ? kDefaultWarnDuration
                                   : kForever);
  }

 private:
#if defined(WEBRTC_WIN)
  HANDLE event_handle_;
#elif defined(WEBRTC_POSIX)
  pthread_mutex_t event_mutex_;
  pthread_cond_t event_cond_;
  const bool is_manual_reset_;
  bool event_status_;
#endif
};

// These classes are provided for compatibility with Chromium.
// The webrtc::Event implementation is overriden inside of Chromium for the
// purposes of detecting when threads are blocked that shouldn't be as well as
// to use the more accurate event implementation that's there than is provided
// by default on some platforms (e.g. Windows).
// When building with standalone WebRTC, this class is a noop.
// For further information, please see the
// ScopedAllowBaseSyncPrimitives(ForTesting) classes in Chromium.
class ScopedAllowBaseSyncPrimitives {
 public:
  ScopedAllowBaseSyncPrimitives() {}
  ~ScopedAllowBaseSyncPrimitives() {}
};

class ScopedAllowBaseSyncPrimitivesForTesting {
 public:
  ScopedAllowBaseSyncPrimitivesForTesting() {}
  ~ScopedAllowBaseSyncPrimitivesForTesting() {}
};

#if RTC_DCHECK_IS_ON
class ScopedDisallowWait {
 public:
  ScopedDisallowWait() = default;

 private:
  class DisallowYieldHandler : public YieldInterface {
   public:
    void YieldExecution() override { RTC_DCHECK_NOTREACHED(); }
  } handler_;
  webrtc::ScopedYieldPolicy policy{&handler_};
};
#endif

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::Event;
using ::webrtc::ScopedAllowBaseSyncPrimitives;
using ::webrtc::ScopedAllowBaseSyncPrimitivesForTesting;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_EVENT_H_
