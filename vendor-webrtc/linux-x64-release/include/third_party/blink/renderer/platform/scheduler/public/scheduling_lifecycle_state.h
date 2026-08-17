// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_SCHEDULING_LIFECYCLE_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_SCHEDULING_LIFECYCLE_STATE_H_

namespace blink {
namespace scheduler {

// The scheduling state of the frame which is communicated to observers.
// It's closely related to Web Lifecycle[1] states and should be distingushed
// from DocumentLifecycle.
//
// [1] https://wicg.github.io/page-lifecycle/spec.html.
enum class SchedulingLifecycleState {
  // Frame is active and should not be throttled.
  kNotThrottled,
  // Frame has just been backgrounded and can be throttled non-aggressively.
  kHidden,
  // Frame spent some time in background and can be fully throttled.
  kThrottled,
  // Frame is stopped, no tasks associated with it can run.
  kStopped,
};

const char* SchedulingLifecycleStateToString(SchedulingLifecycleState state);

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_SCHEDULING_LIFECYCLE_STATE_H_
