// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_VIRTUAL_TIME_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_VIRTUAL_TIME_CONTROLLER_H_

#include "base/functional/callback.h"
#include "base/time/time.h"
#include "third_party/blink/public/platform/scheduler/web_scoped_virtual_time_pauser.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// Exposes methods to enable and control Virtual Time mode for associated
// thread scheduler. When virtual time is enabled, the system doesn't
// actually sleep for the delays between tasks before executing them.
//
// E.g: A-E are delayed tasks
//
// |    A   B C  D           E  (normal)
// |-----------------------------> time
//
// |ABCDE                       (virtual time)
// |-----------------------------> time
//
class VirtualTimeController {
 public:
  enum class VirtualTimePolicy {
    // In this policy virtual time is allowed to advance. If the blink scheduler
    // runs out of immediate work, the virtual timebase will be incremented so
    // that the next sceduled timer may fire.  NOTE Tasks will be run in time
    // order (as usual).
    kAdvance,

    // In this policy virtual time is not allowed to advance. Delayed tasks
    // posted to task runners owned by any child FrameSchedulers will be
    // paused, unless their scheduled run time is less than or equal to the
    // current virtual time.  Note non-delayed tasks will run as normal.
    kPause,

    // In this policy virtual time is allowed to advance unless there are
    // pending network fetches associated any child FrameScheduler, or a
    // document is being parsed on a background thread. Initially virtual time
    // is not allowed to advance until we have seen at least one load. The aim
    // being to try and make loading (more) deterministic.
    kDeterministicLoading,
  };

  // Enables virtual time for associated thread scheduler.
  // |initial_time| sets the initial value for Date.now()
  // Returns the base::TimeTicks that virtual time offsets will be relative to.
  virtual base::TimeTicks EnableVirtualTime(base::Time initial_time) = 0;

  // Disables virtual time. Note that this is only used for testing, because
  // there's no reason to do this in production.
  virtual void DisableVirtualTimeForTesting() = 0;

  // Returns true if virtual time is currently allowed to advance.
  virtual bool VirtualTimeAllowedToAdvance() const = 0;

  // Sets the virtual time policy, which is applied imemdiatly to all child
  // FrameSchedulers.
  // May only be called after virtual time was enabled with EnableVirtualTime().
  virtual void SetVirtualTimePolicy(VirtualTimePolicy) = 0;

  // Set the remaining virtual time budget to |budget|. Once the budget runs
  // out, |budget_exhausted_callback| is called. Note that the virtual time
  // policy is not affected when the budget expires.
  // May only be called after virtual time was enabled with EnableVirtualTime().
  virtual void GrantVirtualTimeBudget(
      base::TimeDelta budget,
      base::OnceClosure budget_exhausted_callback) = 0;

  // It's possible for pages to send infinite messages which can arbitrarily
  // block virtual time.  We can prevent this by setting an upper limit on the
  // number of tasks that can run before virtual time is advanced.
  // NB this anti-starvation logic doesn't apply to VirtualTimePolicy::kPause.
  // May only be called after virtual time was enabled with EnableVirtualTime().
  virtual void SetMaxVirtualTimeTaskStarvationCount(
      int max_task_starvation_count) = 0;

  // Returns a WebScopedVirtualTimePauser which can be used to vote for pausing
  // virtual time. Virtual time will be paused if any WebScopedVirtualTimePauser
  // votes to pause it, and only unpaused only if all
  // WebScopedVirtualTimePausers are either destroyed or vote to unpause.  Note
  // the WebScopedVirtualTimePauser returned by this method is initially
  // unpaused.
  virtual WebScopedVirtualTimePauser CreateWebScopedVirtualTimePauser(
      const WTF::String& name,
      WebScopedVirtualTimePauser::VirtualTaskDuration) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_VIRTUAL_TIME_CONTROLLER_H_
