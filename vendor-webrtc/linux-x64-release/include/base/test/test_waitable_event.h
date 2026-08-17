// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_WAITABLE_EVENT_H_
#define BASE_TEST_TEST_WAITABLE_EVENT_H_

#include "base/synchronization/waitable_event.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/scoped_handle.h"
#endif

namespace base {

// A WaitableEvent for use in tests, it has the same API as WaitableEvent with
// the following two distinctions:
//   1) ScopedAllowBaseSyncPrimitivesForTesting is not required to block on it.
//   2) It doesn't instantiate a ScopedBlockingCallWithBaseSyncPrimitives in
//      Wait() (important in some //base tests that are thrown off when the
//      WaitableEvents used to drive the test add additional ScopedBlockingCalls
//      to the mix of monitored calls).
class TestWaitableEvent : public WaitableEvent {
 public:
  TestWaitableEvent(ResetPolicy reset_policy = ResetPolicy::MANUAL,
                    InitialState initial_state = InitialState::NOT_SIGNALED);

#if BUILDFLAG(IS_WIN)
  explicit TestWaitableEvent(win::ScopedHandle event_handle);
#endif
};

static_assert(sizeof(TestWaitableEvent) == sizeof(WaitableEvent),
              "WaitableEvent is non-virtual, TestWaitableEvent must be usable "
              "interchangeably.");

}  // namespace base

#endif  // BASE_TEST_TEST_WAITABLE_EVENT_H_
