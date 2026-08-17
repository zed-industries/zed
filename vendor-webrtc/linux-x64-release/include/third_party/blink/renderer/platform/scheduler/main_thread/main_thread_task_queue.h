// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_TASK_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_TASK_QUEUE_H_

#include <bit>
#include <memory>
#include <optional>

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/common/lazy_now.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/task/sequence_manager/task_queue_impl.h"
#include "base/task/sequence_manager/time_domain.h"
#include "base/task/single_thread_task_runner.h"
#include "net/base/request_priority.h"
#include "third_party/blink/renderer/platform/scheduler/common/blink_scheduler_single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/common/task_priority.h"
#include "third_party/blink/renderer/platform/scheduler/common/throttling/budget_pool.h"
#include "third_party/blink/renderer/platform/scheduler/common/throttling/task_queue_throttler.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/agent_group_scheduler_impl.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_priority.h"
#include "third_party/blink/renderer/platform/scheduler/public/web_scheduling_queue_type.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace base::sequence_manager {
class SequenceManager;
}  // namespace base::sequence_manager

namespace blink::scheduler {

using TaskQueue = base::sequence_manager::TaskQueue;

namespace main_thread_scheduler_impl_unittest {
class MainThreadSchedulerImplTest;
}

class FrameSchedulerImpl;
class MainThreadSchedulerImpl;
class WakeUpBudgetPool;

// TODO(crbug.com/1143007): Remove ref-counting of MainThreadTaskQueues as it's
// no longer needed.
class PLATFORM_EXPORT MainThreadTaskQueue
    : public ThreadSafeRefCounted<MainThreadTaskQueue> {
 public:
  enum class QueueType {
    // Keep MainThreadTaskQueue::NameForQueueType in sync.
    // This enum is used for a histogram and it should not be re-numbered.
    // TODO(altimin): Clean up obsolete names and use a new histogram when
    // the situation settles.
    kControl = 0,
    kDefault = 1,

    // 2 was used for default loading task runner but this was deprecated.

    // 3 was used for default timer task runner but this was deprecated.

    // 4: kUnthrottled, obsolete.

    kFrameLoading = 5,
    // 6 : kFrameThrottleable, replaced with FRAME_THROTTLEABLE.
    // 7 : kFramePausable, replaced with kFramePausable
    kCompositor = 8,
    kIdle = 9,
    kTest = 10,
    kFrameLoadingControl = 11,
    kFrameThrottleable = 12,
    kFrameDeferrable = 13,
    kFramePausable = 14,
    kFrameUnpausable = 15,
    kV8 = 16,
    kV8UserVisible = 27,
    kV8BestEffort = 28,
    // 17 : kIPC, obsolete
    kInput = 18,

    // Detached is used in histograms for tasks which are run after frame
    // is detached and task queue is gracefully shutdown.
    // TODO(altimin): Move to the top when histogram is renumbered.
    kDetached = 19,

    // 20 : kCleanup, obsolete.
    // 21 : kWebSchedulingUserInteraction, obsolete.
    // 22 : kWebSchedulingBestEffort, obsolete.

    kWebScheduling = 24,
    kNonWaking = 25,

    kIPCTrackingForCachedPages = 26,

    // Used to group multiple types when calculating Expected Queueing Time.
    kOther = 23,
    kCount = 29
  };

  // The ThrottleHandle controls throttling and unthrottling the queue. When
  // a caller requests a queue to be throttled, this handle is returned and
  // the queue will remain throttled as long as the handle is alive.
  class ThrottleHandle {
   public:
    explicit ThrottleHandle(MainThreadTaskQueue& task_queue)
        : task_queue_(task_queue.AsWeakPtr()) {
      // The throttler is reset for detached task queues, which we shouldn't be
      // attempting to throttle.
      CHECK(task_queue_->throttler_);
      task_queue_->throttler_->IncreaseThrottleRefCount();
    }

    ~ThrottleHandle() {
      // The throttler is reset for detached task queues.
      if (task_queue_ && task_queue_->throttler_) {
        task_queue_->throttler_->DecreaseThrottleRefCount();
      }
    }

    // Move-only.
    ThrottleHandle(ThrottleHandle&& other)
        : task_queue_(std::move(other.task_queue_)) {
      other.task_queue_ = nullptr;
    }
    ThrottleHandle& operator=(ThrottleHandle&&);

   private:
    base::WeakPtr<MainThreadTaskQueue> task_queue_;
  };

  // Returns name of the given queue type. Returned string has application
  // lifetime.
  static base::sequence_manager::QueueName NameForQueueType(
      QueueType queue_type);

  using QueueTraitsKeyType = int;

  // QueueTraits represent the deferrable, throttleable, pausable, and freezable
  // properties of a MainThreadTaskQueue. For non-loading task queues, there
  // will be at most one task queue with a specific set of QueueTraits, and the
  // the QueueTraits determine which queues should be used to run which task
  // types.
  struct QueueTraits {
    QueueTraits() = default;

    // Separate enum class for handling prioritisation decisions in task queues.
    enum class PrioritisationType {
      kInternalScriptContinuation = 0,
      kBestEffort = 1,
      kRegular = 2,
      kLoading = 3,
      kLoadingControl = 4,
      kFindInPage = 5,
      kExperimentalDatabase = 6,
      kJavaScriptTimer = 7,
      kHighPriorityLocalFrame = 8,
      kCompositor = 9,  // Main-thread only.
      kInput = 10,
      kPostMessageForwarding = 11,
      kInternalNavigationCancellation = 12,
      kRenderBlocking = 13,
      kLow = 14,
      kAsyncScript = 15,

      kMaxValue = kAsyncScript
    };

    // Bit width required for the PrioritisationType enumeration
    static constexpr unsigned kPrioritisationTypeWidthBits =
        std::bit_width(static_cast<unsigned>(PrioritisationType::kMaxValue));

    // Ensure that the count of the enumeration does not exceed the
    // representable range
    static_assert(static_cast<unsigned>(PrioritisationType::kMaxValue) <
                      (1u << kPrioritisationTypeWidthBits),
                  "PrioritisationType count exceeds the bit width range");

    // Ensure that the count of the enumeration is not less than half the
    // representable range
    static_assert(
        static_cast<unsigned>(PrioritisationType::kMaxValue) >=
            (1u << (kPrioritisationTypeWidthBits - 1)),
        "PrioritisationType count is less than half the bit width range");

    QueueTraits(const QueueTraits&) = default;
    QueueTraits& operator=(const QueueTraits&) = default;

    QueueTraits SetCanBeDeferred(bool value) {
      can_be_deferred = value;
      return *this;
    }

    QueueTraits SetCanBeDeferredForRendering(bool value) {
      can_be_deferred_for_rendering = value;
      return *this;
    }

    QueueTraits SetCanBeThrottled(bool value) {
      can_be_throttled = value;
      return *this;
    }

    QueueTraits SetCanBeIntensivelyThrottled(bool value) {
      can_be_intensively_throttled = value;
      return *this;
    }

    QueueTraits SetCanBePaused(bool value) {
      can_be_paused = value;
      return *this;
    }

    QueueTraits SetCanBeFrozen(bool value) {
      can_be_frozen = value;
      return *this;
    }

    QueueTraits SetCanRunInBackground(bool value) {
      can_run_in_background = value;
      return *this;
    }

    QueueTraits SetCanRunWhenVirtualTimePaused(bool value) {
      can_run_when_virtual_time_paused = value;
      return *this;
    }

    QueueTraits SetPrioritisationType(PrioritisationType type) {
      prioritisation_type = type;
      return *this;
    }

    QueueTraits SetCanBePausedForAndroidWebview(bool value) {
      can_be_paused_for_android_webview = value;
      return *this;
    }

    bool operator==(const QueueTraits& other) const = default;

    // Return a key suitable for WTF::HashMap.
    QueueTraitsKeyType Key() const {
      // offset for shifting bits to compute |key|.
      // |key| starts at 1 since 0 and -1 are used for empty/deleted values.
      int offset = 0;
      int key = 1 << (offset++);
      key |= can_be_deferred << (offset++);
      key |= can_be_deferred_for_rendering << (offset++);
      key |= can_be_throttled << (offset++);
      key |= can_be_intensively_throttled << (offset++);
      key |= can_be_paused << (offset++);
      key |= can_be_frozen << (offset++);
      key |= can_run_in_background << (offset++);
      key |= can_run_when_virtual_time_paused << (offset++);
      key |= can_be_paused_for_android_webview << (offset++);
      key |= static_cast<int>(prioritisation_type) << offset;
      offset += kPrioritisationTypeWidthBits;
      return key;
    }

    void WriteIntoTrace(perfetto::TracedValue context) const;

    bool can_be_deferred : 1 = false;
    bool can_be_deferred_for_rendering : 1 = false;
    bool can_be_throttled : 1 = false;
    bool can_be_intensively_throttled : 1 = false;
    bool can_be_paused : 1 = false;
    bool can_be_frozen : 1 = false;
    bool can_run_in_background : 1 = true;
    bool can_run_when_virtual_time_paused : 1 = true;
    bool can_be_paused_for_android_webview : 1 = false;
    PrioritisationType prioritisation_type = PrioritisationType::kRegular;
  };

  struct QueueCreationParams {
    explicit QueueCreationParams(QueueType queue_type)
        : queue_type(queue_type), spec(NameForQueueType(queue_type)) {}

    QueueCreationParams SetWebSchedulingQueueType(
        std::optional<WebSchedulingQueueType> type) {
      web_scheduling_queue_type = type;
      return *this;
    }

    QueueCreationParams SetWebSchedulingPriority(
        std::optional<WebSchedulingPriority> priority) {
      web_scheduling_priority = priority;
      return *this;
    }

    QueueCreationParams SetAgentGroupScheduler(
        AgentGroupSchedulerImpl* scheduler) {
      agent_group_scheduler = scheduler;
      return *this;
    }

    QueueCreationParams SetFrameScheduler(FrameSchedulerImpl* scheduler) {
      frame_scheduler = scheduler;
      return *this;
    }

    // Forwarded calls to |queue_traits|

    QueueCreationParams SetCanBeDeferred(bool value) {
      queue_traits = queue_traits.SetCanBeDeferred(value);
      ApplyQueueTraitsToSpec();
      return *this;
    }

    QueueCreationParams SetCanBeDeferredForRendering(bool value) {
      queue_traits = queue_traits.SetCanBeDeferredForRendering(value);
      ApplyQueueTraitsToSpec();
      return *this;
    }

    QueueCreationParams SetCanBeThrottled(bool value) {
      queue_traits = queue_traits.SetCanBeThrottled(value);
      ApplyQueueTraitsToSpec();
      return *this;
    }

    QueueCreationParams SetCanBePaused(bool value) {
      queue_traits = queue_traits.SetCanBePaused(value);
      ApplyQueueTraitsToSpec();
      return *this;
    }

    QueueCreationParams SetCanBeFrozen(bool value) {
      queue_traits = queue_traits.SetCanBeFrozen(value);
      ApplyQueueTraitsToSpec();
      return *this;
    }

    QueueCreationParams SetCanRunInBackground(bool value) {
      queue_traits = queue_traits.SetCanRunInBackground(value);
      ApplyQueueTraitsToSpec();
      return *this;
    }

    QueueCreationParams SetCanRunWhenVirtualTimePaused(bool value) {
      queue_traits = queue_traits.SetCanRunWhenVirtualTimePaused(value);
      ApplyQueueTraitsToSpec();
      return *this;
    }

    QueueCreationParams SetPrioritisationType(
        QueueTraits::PrioritisationType type) {
      queue_traits = queue_traits.SetPrioritisationType(type);
      ApplyQueueTraitsToSpec();
      return *this;
    }

    QueueCreationParams SetQueueTraits(QueueTraits value) {
      queue_traits = value;
      ApplyQueueTraitsToSpec();
      return *this;
    }

    // Forwarded calls to |spec|.

    QueueCreationParams SetShouldMonitorQuiescence(bool should_monitor) {
      spec = spec.SetShouldMonitorQuiescence(should_monitor);
      return *this;
    }

    QueueCreationParams SetShouldNotifyObservers(bool run_observers) {
      spec = spec.SetShouldNotifyObservers(run_observers);
      return *this;
    }

    QueueCreationParams SetNonWaking(bool non_waking) {
      spec = spec.SetNonWaking(non_waking);
      return *this;
    }

    QueueType queue_type;
    TaskQueue::Spec spec;
    WeakPersistent<AgentGroupSchedulerImpl> agent_group_scheduler;
    raw_ptr<FrameSchedulerImpl> frame_scheduler = nullptr;
    QueueTraits queue_traits;
    std::optional<WebSchedulingQueueType> web_scheduling_queue_type;
    std::optional<WebSchedulingPriority> web_scheduling_priority;

   private:
    void ApplyQueueTraitsToSpec() {
      spec = spec.SetDelayedFencesAllowed(queue_traits.can_be_throttled);
    }
  };

  MainThreadTaskQueue(base::sequence_manager::SequenceManager& sequence_manager,
                      const TaskQueue::Spec& spec,
                      const QueueCreationParams& params,
                      MainThreadSchedulerImpl* main_thread_scheduler);

  QueueType queue_type() const { return queue_type_; }

  bool CanBeDeferred() const { return queue_traits_.can_be_deferred; }

  bool CanBeDeferredForRendering() const {
    return queue_traits_.can_be_deferred_for_rendering;
  }

  bool CanBeThrottled() const { return queue_traits_.can_be_throttled; }

  bool CanBeIntensivelyThrottled() const {
    return queue_traits_.can_be_intensively_throttled;
  }

  bool CanBePaused() const { return queue_traits_.can_be_paused; }

  // Used for WebView's pauseTimers API. This API expects layout, parsing, and
  // Javascript timers to be paused. Though this suggests we should pause
  // loading (where parsing happens) as well, there are some expectations of JS
  // still being able to run during pause. Because of this we only pause timers
  // as well as any other pausable frame task queue.
  // https://developer.android.com/reference/android/webkit/WebView#pauseTimers()
  bool CanBePausedForAndroidWebview() const {
    return queue_traits_.can_be_paused_for_android_webview;
  }

  bool CanBeFrozen() const { return queue_traits_.can_be_frozen; }

  bool CanRunInBackground() const {
    return queue_traits_.can_run_in_background;
  }

  bool CanRunWhenVirtualTimePaused() const {
    return queue_traits_.can_run_when_virtual_time_paused;
  }

  QueueTraits GetQueueTraits() const { return queue_traits_; }

  QueueTraits::PrioritisationType GetPrioritisationType() const {
    return queue_traits_.prioritisation_type;
  }

  void OnTaskStarted(const base::sequence_manager::Task& task,
                     const TaskQueue::TaskTiming& task_timing);

  void OnTaskCompleted(const base::sequence_manager::Task& task,
                       TaskQueue::TaskTiming* task_timing,
                       base::LazyNow* lazy_now);

  void LogTaskExecution(perfetto::EventContext& ctx,
                        const base::sequence_manager::Task& task);

  void SetOnIPCTaskPosted(
      base::RepeatingCallback<void(const base::sequence_manager::Task&)>
          on_ipc_task_posted_callback);
  void DetachOnIPCTaskPostedWhileInBackForwardCache();

  // Called when the underlying scheduler is destroyed. Tasks in this queue will
  // continue to run until the queue becomes empty.
  void DetachTaskQueue();

  // Shuts down the task queue. No tasks will run after this is called.
  void ShutdownTaskQueue();

  AgentGroupScheduler* GetAgentGroupScheduler();

  FrameSchedulerImpl* GetFrameScheduler() const;

  scoped_refptr<base::SingleThreadTaskRunner> CreateTaskRunner(
      TaskType task_type);

  std::optional<WebSchedulingQueueType> GetWebSchedulingQueueType() const {
    return web_scheduling_queue_type_;
  }

  std::optional<WebSchedulingPriority> GetWebSchedulingPriority() const {
    return web_scheduling_priority_;
  }

  void SetWebSchedulingPriority(WebSchedulingPriority priority);

  void OnWebSchedulingTaskQueueDestroyed();

  // TODO(crbug.com/1143007): Improve MTTQ API surface so that we no longer
  // need to expose the raw pointer to the queue.
  TaskQueue* GetTaskQueue() { return task_queue_.get(); }

  // This method returns the default task runner with task type kTaskTypeNone
  // and is mostly used for tests. For most use cases, you'll want a more
  // specific task runner and should use the 'CreateTaskRunner' method and pass
  // the desired task type.
  const scoped_refptr<base::SingleThreadTaskRunner>&
  GetTaskRunnerWithDefaultTaskType() {
    return task_runner_with_default_task_type_;
  }

  bool IsThrottled() const;

  // Throttles the task queue as long as the handle is kept alive.
  MainThreadTaskQueue::ThrottleHandle Throttle();

  // Called when a task finished running to update cpu-based throttling.
  void OnTaskRunTimeReported(TaskQueue::TaskTiming* task_timing);

  // Methods for setting and resetting budget pools for this task queue.
  // Note that a task queue can be in multiple budget pools so a pool must
  // be specified when resetting.
  void AddToBudgetPool(base::TimeTicks now, BudgetPool* pool);
  void RemoveFromBudgetPool(base::TimeTicks now, BudgetPool* pool);

  void SetWakeUpBudgetPool(WakeUpBudgetPool* wake_up_budget_pool);
  WakeUpBudgetPool* GetWakeUpBudgetPool() const { return wake_up_budget_pool_; }

  void SetQueuePriority(TaskPriority priority) {
    task_queue_->SetQueuePriority(priority);
  }

  TaskPriority GetQueuePriority() const {
    return static_cast<TaskPriority>(task_queue_->GetQueuePriority());
  }

  bool IsQueueEnabled() const { return task_queue_->IsQueueEnabled(); }
  bool IsEmpty() const { return task_queue_->IsEmpty(); }

  bool HasTaskToRunImmediatelyOrReadyDelayedTask() const {
    return task_queue_->HasTaskToRunImmediatelyOrReadyDelayedTask();
  }

  void SetShouldReportPostedTasksWhenDisabled(bool should_report) {
    task_queue_->SetShouldReportPostedTasksWhenDisabled(should_report);
  }

  std::unique_ptr<TaskQueue::QueueEnabledVoter> CreateQueueEnabledVoter() {
    return task_queue_->CreateQueueEnabledVoter();
  }

  void AddTaskObserver(base::TaskObserver* task_observer) {
    task_queue_->AddTaskObserver(task_observer);
  }
  void RemoveTaskObserver(base::TaskObserver* task_observer) {
    task_queue_->RemoveTaskObserver(task_observer);
  }

  base::WeakPtr<MainThreadTaskQueue> AsWeakPtr() {
    return weak_ptr_factory_.GetWeakPtr();
  }

  void WriteIntoTrace(perfetto::TracedValue context) const;

 protected:
  void SetFrameSchedulerForTest(FrameSchedulerImpl* frame_scheduler);

  MainThreadTaskQueue(const MainThreadTaskQueue&) = delete;
  MainThreadTaskQueue& operator=(const MainThreadTaskQueue&) = delete;

  ~MainThreadTaskQueue();

 private:
  friend class ThreadSafeRefCounted<MainThreadTaskQueue>;
  friend class blink::scheduler::main_thread_scheduler_impl_unittest::
      MainThreadSchedulerImplTest;

  scoped_refptr<BlinkSchedulerSingleThreadTaskRunner> WrapTaskRunner(
      scoped_refptr<base::SingleThreadTaskRunner>);

  TaskQueue::Handle task_queue_;
  scoped_refptr<base::SingleThreadTaskRunner>
      task_runner_with_default_task_type_;
  std::optional<TaskQueueThrottler> throttler_;

  const QueueType queue_type_;
  const QueueTraits queue_traits_;

  // Set if this is queue is used for the web-exposed scheduling API. Used to
  // differentiate initial tasks from continuations for prioritization.
  const std::optional<WebSchedulingQueueType> web_scheduling_queue_type_;

  // |web_scheduling_priority_| is the priority of the task queue within the web
  // scheduling API. This priority is used in conjunction with the frame
  // scheduling policy to determine the task queue priority.
  std::optional<WebSchedulingPriority> web_scheduling_priority_;

  // Needed to notify renderer scheduler about completed tasks.
  raw_ptr<MainThreadSchedulerImpl> main_thread_scheduler_;  // NOT OWNED

  WeakPersistent<AgentGroupSchedulerImpl> agent_group_scheduler_;

  // Set in the constructor. Cleared in `DetachTaskQueue()` and
  // `ShutdownTaskQueue()`. Can never be set to a different value afterwards
  // (except in tests).
  raw_ptr<FrameSchedulerImpl, DanglingUntriaged> frame_scheduler_;  // NOT OWNED

  // The WakeUpBudgetPool for this TaskQueue, if any.
  raw_ptr<WakeUpBudgetPool> wake_up_budget_pool_{nullptr};  // NOT OWNED

  std::unique_ptr<TaskQueue::OnTaskPostedCallbackHandle>
      on_ipc_task_posted_callback_handle_;

  base::WeakPtrFactory<MainThreadTaskQueue> weak_ptr_factory_{this};
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_TASK_QUEUE_H_
