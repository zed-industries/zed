/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_CLOCK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_CLOCK_H_

#include <limits>

#include "base/time/default_tick_clock.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Maintains a stationary clock time during script execution. Tracks the glass
// time of the beginning of the current animation frame (i.e. the moment photons
// left the screen for the previous frame).
class CORE_EXPORT AnimationClock {
  DISALLOW_NEW();

 public:
  AnimationClock()
      : time_(),
        can_dynamically_update_time_(false),
        clock_(base::DefaultTickClock::GetInstance()),
        task_for_which_time_was_calculated_(
            std::numeric_limits<unsigned>::max()) {}
  AnimationClock(const AnimationClock&) = delete;
  AnimationClock& operator=(const AnimationClock&) = delete;

  void UpdateTime(base::TimeTicks time);
  base::TimeTicks CurrentTime();

  // The HTML spec says that the clock for animations is only updated once per
  // rendering lifecycle, at the start. However the spec also assumes that the
  // user agent runs rendering lifecycles constantly, back-to-back. In Blink we
  // attempt to *not* run rendering lifecycles as much as possible, to avoid
  // unnecessary CPU usage.
  //
  // As such, when outside a rendering lifecycle (for example, if a setInterval
  // triggers) we allow the AnimationClock to dynamically adjust its time to
  // look like it is being updated by the rendering lifecycles that never
  // happened.
  //
  // TODO(crbug.com/995806): Allowing the AnimationClock to update itself is
  // error prone. We should instead get the latest impl-frame time from the
  // compositor when outside of a Blink rendering lifecycle (whilst still
  // not changing within the same task).
  void SetAllowedToDynamicallyUpdateTime(bool can_dynamically_update_time) {
    can_dynamically_update_time_ = can_dynamically_update_time;
  }

  // When using our dynamically update behavior outside rendering lifecycles, we
  // still do not want the time to move forward within the same task (e.g.
  // within a single setInterval callback). To achieve this we track the task in
  // which the time was last updated, and don't update it again until we are in
  // a new task.
  static void NotifyTaskStart() { ++currently_running_task_; }

  void ResetTimeForTesting();
  // The caller owns the passed in clock, which must outlive the AnimationClock.
  void OverrideDynamicClockForTesting(const base::TickClock*);

 private:
  base::TimeTicks time_;

  // See |SetAllowedToDynamicallyUpdateTime| documentation for these members.
  bool can_dynamically_update_time_;
  const base::TickClock* clock_;

  // See |NotifyTaskStart| documentation for these members.
  unsigned task_for_which_time_was_calculated_;
  static unsigned currently_running_task_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_ANIMATION_CLOCK_H_
