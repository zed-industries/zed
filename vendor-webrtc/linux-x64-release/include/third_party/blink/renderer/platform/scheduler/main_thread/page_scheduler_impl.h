// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_PAGE_SCHEDULER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_PAGE_SCHEDULER_IMPL_H_

#include <memory>
#include <optional>

#include "base/memory/post_delayed_memory_reduction_task.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/common/lazy_now.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/common/cancelable_closure_holder.h"
#include "third_party/blink/renderer/platform/scheduler/common/throttling/task_queue_throttler.h"
#include "third_party/blink/renderer/platform/scheduler/common/throttling/type.h"
#include "third_party/blink/renderer/platform/scheduler/common/tracing_helper.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/agent_group_scheduler_impl.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/frame_origin_type.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/page_visibility_state.h"
#include "third_party/blink/renderer/platform/scheduler/public/page_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/widget_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace blink {
namespace scheduler {
namespace page_scheduler_impl_unittest {
class PageSchedulerImplTest;
}  // namespace page_scheduler_impl_unittest

class CPUTimeBudgetPool;
class FrameSchedulerImpl;
class MainThreadSchedulerImpl;
class MainThreadTaskQueue;
class PolicyUpdater;
class WakeUpBudgetPool;

class PLATFORM_EXPORT PageSchedulerImpl : public PageScheduler {
 public:
  // Interval between throttled wake ups, without intensive throttling.
  static constexpr base::TimeDelta kDefaultThrottledWakeUpInterval =
      base::Seconds(1);

  // Interval between throttled wake ups, with intensive throttling.
  static constexpr base::TimeDelta kIntensiveThrottledWakeUpInterval =
      base::Minutes(1);

  PageSchedulerImpl(PageScheduler::Delegate*, AgentGroupSchedulerImpl&);
  PageSchedulerImpl(const PageSchedulerImpl&) = delete;
  PageSchedulerImpl& operator=(const PageSchedulerImpl&) = delete;

  ~PageSchedulerImpl() override;

  // PageScheduler implementation:
  void OnTitleOrFaviconUpdated() override;
  void SetPageVisible(bool page_visible) override;
  bool IsPageVisible() const override;
  void SetPageFrozen(bool) override;
  void SetPageBackForwardCached(bool) override;
  bool IsMainFrameLocal() const override;
  void SetIsMainFrameLocal(bool is_local) override;
  base::TimeTicks GetStoredInBackForwardCacheTimestamp() {
    return stored_in_back_forward_cache_timestamp_;
  }
  bool IsInBackForwardCache() const override {
    return is_stored_in_back_forward_cache_;
  }
  bool has_ipc_detection_enabled() { return has_ipc_detection_enabled_; }

  std::unique_ptr<FrameScheduler> CreateFrameScheduler(
      FrameScheduler::Delegate* delegate,
      bool is_in_embedded_frame_tree,
      FrameScheduler::FrameType) override;
  void AudioStateChanged(bool is_audio_playing) override;
  bool IsAudioPlaying() const override;
  bool IsExemptFromBudgetBasedThrottling() const override;
  bool OptedOutFromAggressiveThrottlingForTest() const override;
  bool RequestBeginMainFrameNotExpected(bool new_state) override;
  scoped_refptr<WidgetScheduler> CreateWidgetScheduler(
      WidgetScheduler::Delegate*) override;

  bool IsFrozen() const;
  bool OptedOutFromAggressiveThrottling() const;
  // Returns whether CPU time is throttled for the page. Note: This is
  // independent from wake up rate throttling.
  bool IsCPUTimeThrottled() const;

  bool IsLoading() const;

  // An "ordinary" PageScheduler is responsible for a fully-featured page
  // owned by a web view.
  virtual bool IsOrdinary() const;

  MainThreadSchedulerImpl* GetMainThreadScheduler() const;
  AgentGroupSchedulerImpl& GetAgentGroupScheduler() override;
  VirtualTimeController* GetVirtualTimeController() override;

  void Unregister(FrameSchedulerImpl*);

  void OnThrottlingStatusUpdated();

  void OnVirtualTimeEnabled();

  void OnTraceLogEnabled();

  // Virtual for testing.
  virtual bool IsWaitingForMainFrameContentfulPaint() const;
  virtual bool IsWaitingForMainFrameMeaningfulPaint() const;
  virtual bool IsMainFrameLoading() const;

  // Return a number of child web frame schedulers for this PageScheduler.
  size_t FrameCount() const;

  void SetUpIPCTaskDetection();
  // This flag tracks whether or not IPC tasks are tracked if they are posted to
  // frames or pages that are stored in the back-forward cache
  bool has_ipc_detection_enabled_ = false;

  // Update policy for all frames.
  void UpdatePolicy();

  void WriteIntoTrace(perfetto::TracedValue context, base::TimeTicks now) const;

  base::WeakPtr<PageSchedulerImpl> GetWeakPtr() {
    return weak_factory_.GetWeakPtr();
  }

 private:
  friend class AgentGroupSchedulerImpl;
  friend class FrameSchedulerImpl;
  friend class page_scheduler_impl_unittest::PageSchedulerImplTest;

  enum class AudioState {
    kSilent,
    kAudible,
    kRecentlyAudible,
  };

  void RegisterFrameSchedulerImpl(FrameSchedulerImpl* frame_scheduler);

  // A page cannot be throttled or frozen 30 seconds after playing audio.
  //
  // This used to be 5 seconds, which was barely enough to cover the time of
  // silence during which a logo and button are shown after a YouTube ad. Since
  // most pages don't play audio in background, it was decided that the delay
  // can be increased to 30 seconds without significantly affecting performance.
  static constexpr base::TimeDelta kRecentAudioDelay = base::Seconds(30);

  // Support not issuing a notification to frames when we disable freezing as
  // a part of foregrounding the page.
  void SetPageFrozenImpl(bool frozen,
                         PolicyUpdater& policy_updater,
                         base::MemoryReductionTaskContext called_from =
                             base::MemoryReductionTaskContext::kDelayExpired);

  // Adds `task_queue` to `wake_up_budget_pool`.
  void AddQueueToWakeUpBudgetPool(MainThreadTaskQueue* task_queue,
                                  WakeUpBudgetPool* wake_up_budget_pool,
                                  base::LazyNow* lazy_now);
  // Removes `task_queue` from its current `WakeUpBudgetPool`, if any.
  void RemoveQueueFromWakeUpBudgetPool(MainThreadTaskQueue* task_queue,
                                       base::LazyNow* lazy_now);
  // Returns the WakeUpBudgetPool to use for a `task_queue` on a frame of type
  // `frame_origin_type` which allows throttling of type `throttling_type`.
  WakeUpBudgetPool* GetWakeUpBudgetPool(MainThreadTaskQueue* task_queue,
                                        FrameOriginType frame_origin_type,
                                        ThrottlingType throttling_type);

  // Initializes WakeUpBudgetPools, if not already initialized.
  void MaybeInitializeWakeUpBudgetPools(base::LazyNow* lazy_now);

  CPUTimeBudgetPool* background_cpu_time_budget_pool();
  void MaybeInitializeBackgroundCPUTimeBudgetPool(base::LazyNow* lazy_now);

  // Depending on page visibility, either turns throttling off, or schedules a
  // call to enable it after a grace period.
  void UpdatePolicyOnVisibilityChange(PolicyUpdater& policy_updater);

  // Adjusts settings of budget pools depending on current state of the page.
  void UpdateCPUTimeBudgetPool(base::LazyNow* lazy_now);
  void UpdateWakeUpBudgetPools(base::LazyNow* lazy_now);
  base::TimeDelta GetIntensiveWakeUpThrottlingInterval(
      bool is_same_origin) const;

  // Callback for marking page is silent after a delay since last audible
  // signal.
  void OnAudioSilent();

  // Callbacks for adjusting the settings of a budget pool after a delay.
  // TODO(altimin): Trigger throttling depending on the loading state
  // of the page.
  void DoThrottleCPUTime();
  void DoIntensivelyThrottleWakeUps();
  void ResetHadRecentTitleOrFaviconUpdate();

  // Returns true if the page is backgrounded, false otherwise. A page is
  // considered backgrounded if it is not visible, not playing audio and
  // virtual time is disabled.
  bool IsBackgrounded() const;

  // Returns true if WakeUpBudgetPools were initialized.
  bool HasWakeUpBudgetPools() const;

  // Determines when this page's frozen state should change. If it should change
  // now, perform the state transition. Otherwise, schedules another call to
  // this method at the time when it should change.
  void UpdateFrozenState(PolicyUpdater& policy_updater);
  void UpdateFrozenState(PolicyUpdater& policy_updater,
                         base::MemoryReductionTaskContext called_from);

  // Returns all WakeUpBudgetPools owned by this PageSchedulerImpl.
  static constexpr int kNumWakeUpBudgetPools = 4;
  std::array<WakeUpBudgetPool*, kNumWakeUpBudgetPools> AllWakeUpBudgetPools();

  TraceableVariableController tracing_controller_;
  HashSet<FrameSchedulerImpl*> frame_schedulers_;
  raw_ptr<MainThreadSchedulerImpl, DanglingUntriaged> main_thread_scheduler_;
  Persistent<AgentGroupSchedulerImpl> agent_group_scheduler_;

  PageVisibilityState page_visibility_;
  base::TimeTicks page_visibility_changed_time_;
  AudioState audio_state_;
  base::TimeTicks audio_state_changed_time_;
  bool is_frozen_;
  bool opted_out_from_aggressive_throttling_;
  bool nested_runloop_;
  bool is_main_frame_local_;
  bool is_cpu_time_throttled_;
  bool are_wake_ups_intensively_throttled_;
  bool had_recent_title_or_favicon_update_;
  std::unique_ptr<CPUTimeBudgetPool> cpu_time_budget_pool_;

  // Wake up budget pools for each throttling scenario:
  //
  // For background pages:
  //                                    Same-origin frame    Cross-origin frame
  //   Normal throttling only           2                    2
  //   Normal and intensive throttling  3                    4
  //
  // For foreground pages:
  //   Visible small & non user activated cross-origin frame (unimportant)    1
  //   Hidden cross-origin frame                                              2
  //
  // Task queues attched to these pools will be updated when:
  //    * Page background state changes
  //    * Frame visibility changes
  //    * Frame origin changes
  //    * Cross-origin frame size proportion of page's visible area changes
  //    * Cross-origin frame user activation state changes
  //
  // 1: This pool allows |unimportant_timers_throttled_wake_up_interval_|
  //    aligned wake ups for unimportant frames (visible small and no user
  //    activated cross-origin frames) in foreground pages.
  std::unique_ptr<WakeUpBudgetPool> unimportant_wake_up_budget_pool_;
  // 2: This pool allows 1-second aligned wake ups for hidden cross-origin
  //    frames in foreground pages, or when the page is backgrounded.
  std::unique_ptr<WakeUpBudgetPool> hidden_wake_up_budget_pool_;
  // 3: This pool allows 1-second aligned wake ups if the page is not
  //    intensively throttled of if there hasn't been a wake up in the last
  //    minute. Otherwise, it allows 1-minute aligned wake ups.
  std::unique_ptr<WakeUpBudgetPool> same_origin_intensive_wake_up_budget_pool_;
  // 4: This pool allows 1-second aligned wake ups if the page is not
  //    intensively throttled. Otherwise, it allows 1-minute aligned wake ups.
  //
  //    Unlike |same_origin_intensive_wake_up_budget_pool_|, this pool does not
  //    allow a 1-second aligned wake up when there hasn't been a wake up in the
  //    last minute. This is to prevent frames from different origins from
  //    learning about each other. Concretely, this means that
  //    MaybeInitializeWakeUpBudgetPools() does not invoke
  //    AllowUnalignedWakeUpIfNoRecentWakeUp() on this pool.
  std::unique_ptr<WakeUpBudgetPool> cross_origin_intensive_wake_up_budget_pool_;

  raw_ptr<PageScheduler::Delegate> delegate_;
  CancelableClosureHolder do_throttle_cpu_time_callback_;
  CancelableClosureHolder do_intensively_throttle_wake_ups_callback_;
  CancelableClosureHolder reset_had_recent_title_or_favicon_update_;
  CancelableClosureHolder on_audio_silent_closure_;
  base::OneShotDelayedBackgroundTimer update_frozen_state_timer_;
  const base::TimeDelta delay_for_background_tab_freezing_;

  // Interval between throttled wake ups for unimportant frames (visible, small
  // and non user activated cross origin frames) on a foreground page.
  const base::TimeDelta unimportant_timers_throttled_wake_up_interval_;

  bool is_stored_in_back_forward_cache_ = false;
  TaskHandle set_ipc_posted_handler_task_;
  base::TimeTicks stored_in_back_forward_cache_timestamp_;

  base::WeakPtrFactory<PageSchedulerImpl> weak_factory_{this};
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_PAGE_SCHEDULER_IMPL_H_
