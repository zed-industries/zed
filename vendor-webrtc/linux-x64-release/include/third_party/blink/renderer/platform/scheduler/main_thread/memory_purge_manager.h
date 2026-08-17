// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MEMORY_PURGE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MEMORY_PURGE_MANAGER_H_

#include "base/memory/post_delayed_memory_reduction_task.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "base/timer/timer.h"
#include "third_party/blink/public/common/page/launching_process_state.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Manages process-wide proactive memory purging.
class PLATFORM_EXPORT MemoryPurgeManager {
 public:
  explicit MemoryPurgeManager(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  MemoryPurgeManager(const MemoryPurgeManager&) = delete;
  MemoryPurgeManager& operator=(const MemoryPurgeManager&) = delete;
  ~MemoryPurgeManager();

  // Called when a page is created or destroyed, to maintain the total count of
  // pages owned by a renderer. MemoryPurgeManager assumes that a page is *not*
  // frozen on creation.
  void OnPageCreated();
  void OnPageDestroyed(bool frozen);

  // Called when a page is frozen. If all pages are frozen or
  // |kFreezePurgeMemoryAllPagesFrozen| is disabled, and the renderer is
  // backgrounded, ensures that a delayed memory purge is scheduled. If the
  // timer is already running, uses the smallest requested delay.
  void OnPageFrozen(base::MemoryReductionTaskContext called_from =
                        base::MemoryReductionTaskContext::kDelayExpired);

  // Called when a page is resumed (unfrozen). Has the effect of unsuppressing
  // memory pressure notifications.
  void OnPageResumed();

  // Called when the renderer's process priority changes.
  void SetRendererBackgrounded(bool backgrounded);

  // Called when the renderer is backgrounded. Starts a timer responsible for
  // performing a memory purge upon expiry, if the
  // kPurgeRendererMemoryWhenBackgrounded feature is enabled. If the timer is
  // already running, uses the smallest requested delay.
  void OnRendererBackgrounded();

  // Called when the renderer is foregrounded. Has the effect of cancelling a
  // queued memory purge.
  void OnRendererForegrounded();

  void SetPurgeDisabledForTesting(bool disabled) {
    purge_disabled_for_testing_ = disabled;
  }

#if BUILDFLAG(IS_ANDROID)
  // Sets a callback called with |false| when transitioning from "all pages
  // frozen" to "not all pages frozen" or |true| when transitioning from "not
  // all pages frozen" to "all pages frozen".
  //
  // Currently only used on Android.
  void SetOnAllPagesFrozenCallback(
      base::RepeatingCallback<void(bool)> callback);
#endif

  // Disabled on Android, as it is not useful there. This is because we freeze
  // tabs, and trigger a critical memory pressure notification at that point.
  // This has been confirmed to not be necessary on Android in a field trial.
  // See https://bugs.chromium.org/p/chromium/issues/detail?id=1335069#c3 for
  // details.
  static constexpr bool kPurgeEnabled =
#if BUILDFLAG(IS_ANDROID)
      false
#else
      true
#endif
      ;

  // Default maximum time to purge after the renderer is backgrounded. Can be
  // modified by field trials. Exposed for testing.
  static constexpr base::TimeDelta kDefaultMaxTimeToPurgeAfterBackgrounded =
      base::Minutes(4);

  // Only one second, not to delay, but to make sure that it runs after the
  // non-delayed tasks.
  static constexpr base::TimeDelta kFreezePurgeDelay =
      base::TimeDelta(base::Seconds(1));

  // Recorded a metric with whether or not all pages are currently frozen.
  void RecordAreAllPagesFrozenMetric(std::string_view name);

 private:
  // Starts |purge_timer_| to trigger a delayed memory purge. If the timer is
  // already running, starts the timer with the smaller of the requested delay
  // and the remaining delay.
  void RequestMemoryPurgeWithDelay(base::TimeDelta delay);

  // Called when the timer expires. Simulates a critical memory pressure signal
  // to purge memory. Suppresses memory pressure notifications if all pages
  // are frozen.
  void PerformMemoryPurge();

  // Returns true if:
  // - The kPurgeRendererMemoryWhenBackgrounded feature is enabled, and the
  //   renderer is awaiting a purge after being backgrounded.
  //
  // or if all of the following are true:
  //
  // - The renderer is backgrounded.
  // - All pages are frozen or kFreezePurgeMemoryAllPagesFrozen is disabled.
  bool CanPurge() const;

  // If we transitioned between "all pages frozen" and "not all pages frozen",
  // run the callback. |were_all_frozen| indicates whether all pages were
  // previously frozen, before the potential state transition.
  void MaybeRunAllPagesFrozenCallback(bool were_all_frozen);

  // Returns true if |total_page_count_| == |frozen_page_count_|
  bool AreAllPagesFrozen() const;

  base::TimeDelta GetTimeToPurgeAfterBackgrounded() const;

  bool purge_disabled_for_testing_ = false;
  bool renderer_backgrounded_ = kLaunchingProcessIsBackgrounded;

  // Keeps track of whether a memory purge was requested as a consequence of the
  // renderer transitioning to a backgrounded state. Prevents purges from being
  // cancelled if a purge was requested upon backgrounding the renderer and the
  // renderer is still backgrounded. Supports the metrics collection associated
  // with the PurgeAndSuspend experiment.
  bool backgrounded_purge_pending_ = false;

  int total_page_count_ = 0;
  int frozen_page_count_ = 0;

  base::OneShotTimer purge_timer_;

#if BUILDFLAG(IS_ANDROID)
  base::RepeatingCallback<void(bool)> all_pages_frozen_callback_;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MEMORY_PURGE_MANAGER_H_
