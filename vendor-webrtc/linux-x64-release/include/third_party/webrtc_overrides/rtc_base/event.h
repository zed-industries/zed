// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_RTC_BASE_EVENT_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_RTC_BASE_EVENT_H_

#include "base/synchronization/waitable_event.h"
#include "base/threading/thread_restrictions.h"
#include "third_party/webrtc/api/units/time_delta.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"

namespace webrtc {

// Overrides WebRTC's internal event implementation to use Chromium's.
class RTC_EXPORT Event {
 public:
  // TODO(bugs.webrtc.org/14366): Consider removing this redundant alias.
  static constexpr webrtc::TimeDelta kForever =
      webrtc::TimeDelta::PlusInfinity();

  Event();
  Event(bool manual_reset, bool initially_signaled);

  Event(const Event&) = delete;
  Event& operator=(const Event&) = delete;

  ~Event();

  void Set();
  void Reset();

  // Wait for the event to become signaled, for the specified duration. To wait
  // indefinitely, pass kForever.
  bool Wait(webrtc::TimeDelta give_up_after);
  bool Wait(webrtc::TimeDelta give_up_after, webrtc::TimeDelta /*warn_after*/) {
    return Wait(give_up_after);
  }

 private:
  base::WaitableEvent event_;
};

// Pull ScopedAllowBaseSyncPrimitives(ForTesting) into the rtc namespace.
// Managing what types in WebRTC are allowed to use
// ScopedAllowBaseSyncPrimitives, is done via thread_restrictions.h.
using ScopedAllowBaseSyncPrimitives = base::ScopedAllowBaseSyncPrimitives;
using ScopedAllowBaseSyncPrimitivesForTesting =
    base::ScopedAllowBaseSyncPrimitivesForTesting;

}  // namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
namespace rtc {
using ::webrtc::Event;
using ::webrtc::ScopedAllowBaseSyncPrimitives;
using ::webrtc::ScopedAllowBaseSyncPrimitivesForTesting;
}  // namespace rtc

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_RTC_BASE_EVENT_H_
