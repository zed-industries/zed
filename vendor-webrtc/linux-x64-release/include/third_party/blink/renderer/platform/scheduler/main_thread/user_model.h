// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_USER_MODEL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_USER_MODEL_H_

#include "base/time/time.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace blink {
namespace scheduler {

class PLATFORM_EXPORT UserModel {
  USING_FAST_MALLOC(UserModel);

 public:
  // The time we should stay in a priority-escalated mode after an input event.
  static constexpr base::TimeDelta kGestureEstimationLimit =
      base::Milliseconds(100);

  // This is based on two weeks of Android usage data.
  static constexpr base::TimeDelta kMedianGestureDuration =
      base::Milliseconds(300);

  // We consider further gesture start events to be likely if the user has
  // interacted with the device in the past two seconds.
  // Based on Android usage data, 2000ms between gestures is the 75th percentile
  // with 700ms being the 50th.
  static constexpr base::TimeDelta kExpectSubsequentGestureDeadline =
      base::Milliseconds(2000);

  // The maximum amount of time tasks will be deferred in response to discrete
  // input.
  static constexpr base::TimeDelta kDiscreteInputResponseDeadline =
      base::Milliseconds(50);

  UserModel();
  UserModel(const UserModel&) = delete;
  UserModel& operator=(const UserModel&) = delete;

  // Tells us that the system started processing an input event. Must be paired
  // with a call to DidFinishProcessingInputEvent. Called on the compositor
  // thread. This is only called for inputs that are potentially related to
  // gestures.
  void DidStartProcessingInputEvent(WebInputEvent::Type type,
                                    const base::TimeTicks now);

  // Tells us that the system finished processing an input event. Called on
  // either the compositor thread or main thread, depending on where this input
  // is handled. This is only called for inputs that are potentially related to
  // gestures.
  void DidFinishProcessingInputEvent(const base::TimeTicks now);

  // Tells us that the system finished processing a discrete input event, like a
  // click or keyboard event. Only called if a visual response is expected in a
  // subsequent task.
  void DidProcessDiscreteInputEvent(const base::TimeTicks now);

  // Tells us that the UI response for any recent discrete input events was
  // processed.
  void DidProcessDiscreteInputResponse();

  // Returns the estimated amount of time left in the current user gesture, to a
  // maximum of |kGestureEstimationLimitMillis|.  After that time has elapased
  // this function should be called again.
  base::TimeDelta TimeLeftInContinuousUserGesture(base::TimeTicks now) const;

  // Tries to guess if a user gesture is expected soon. Currently this is
  // very simple, but one day I hope to do something more sophisticated here.
  // The prediction may change after |prediction_valid_duration| has elapsed.
  bool IsGestureExpectedSoon(const base::TimeTicks now,
                             base::TimeDelta* prediction_valid_duration);

  // Returns true if a gesture has been in progress for less than the median
  // gesture duration. The prediction may change after
  // |prediction_valid_duration| has elapsed.
  bool IsGestureExpectedToContinue(
      const base::TimeTicks now,
      base::TimeDelta* prediction_valid_duration) const;

  // Returns the time left before the deadline for UI response of the most
  // recent discrete input event.
  base::TimeDelta TimeLeftUntilDiscreteInputResponseDeadline(
      base::TimeTicks now) const;

  void WriteIntoTrace(perfetto::TracedValue context) const;

  // Clears input signals.
  void Reset(base::TimeTicks now);

 private:
  bool IsGestureExpectedSoonImpl(
      const base::TimeTicks now,
      base::TimeDelta* prediction_valid_duration) const;

  int pending_input_event_count_ = 0;
  base::TimeTicks last_input_signal_time_;
  base::TimeTicks last_gesture_start_time_;
  // This only includes continuous events like scrolls, flings, or pinches.
  base::TimeTicks last_continuous_gesture_time_;
  // This only includes discrete events like taps or keyboard events.
  base::TimeTicks last_discrete_input_time_;
  base::TimeTicks last_gesture_expected_start_time_;
  base::TimeTicks last_reset_time_;
  // This typically means the user's finger is down.
  bool is_gesture_active_ = false;
  bool is_gesture_expected_ = false;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_USER_MODEL_H_
