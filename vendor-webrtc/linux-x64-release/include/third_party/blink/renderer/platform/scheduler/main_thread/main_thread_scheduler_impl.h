// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_SCHEDULER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_SCHEDULER_IMPL_H_

#include <atomic>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <stack>

#include "base/dcheck_is_on.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/metrics/single_sample_metrics.h"
#include "base/observer_list.h"
#include "base/profiler/sample_metadata.h"
#include "base/synchronization/lock.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/task/sequence_manager/task_time_observer.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/scoped_thread_priority.h"
#include "base/time/time.h"
#include "base/trace_event/trace_log.h"
#include "build/build_config.h"
#include "third_party/blink/public/common/features.h"
#include "third_party/blink/public/platform/scheduler/web_thread_scheduler.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/common/features.h"
#include "third_party/blink/renderer/platform/scheduler/common/idle_helper.h"
#include "third_party/blink/renderer/platform/scheduler/common/pollable_thread_safe_flag.h"
#include "third_party/blink/renderer/platform/scheduler/common/task_priority.h"
#include "third_party/blink/renderer/platform/scheduler/common/thread_scheduler_base.h"
#include "third_party/blink/renderer/platform/scheduler/common/tracing_helper.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/deadline_task_runner.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/find_in_page_budget_pool_controller.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/idle_time_estimator.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/main_thread_metrics_helper.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/main_thread_scheduler_helper.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/main_thread_task_queue.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/memory_purge_manager.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/page_scheduler_impl.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/pending_user_input.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/render_widget_signals.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/use_case.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/user_model.h"
#include "third_party/blink/renderer/platform/scheduler/main_thread/widget_scheduler_impl.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/main_thread_scheduler.h"
#include "third_party/blink/renderer/platform/scheduler/public/rail_mode_observer.h"
#include "third_party/blink/renderer/platform/scheduler/public/widget_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace base {
class LazyNow;
class TaskObserver;
}  // namespace base

namespace blink {
namespace scheduler {
namespace frame_scheduler_impl_unittest {
class FrameSchedulerImplTest;
}  // namespace frame_scheduler_impl_unittest
namespace main_thread_scheduler_impl_unittest {
class MainThreadSchedulerImplForTest;
class MainThreadSchedulerImplTest;
class MockPageSchedulerImpl;
FORWARD_DECLARE_TEST(MainThreadSchedulerImplTest, Tracing);
FORWARD_DECLARE_TEST(MainThreadSchedulerImplTest,
                     LogIpcsPostedToDocumentsInBackForwardCache);
FORWARD_DECLARE_TEST(MainThreadSchedulerImplTest,
                     CanExceedIdleDeadlineIfRequired);
}  // namespace main_thread_scheduler_impl_unittest
class AgentGroupSchedulerImpl;
class CPUTimeBudgetPool;
class FrameSchedulerImpl;
class PageSchedulerImpl;
class WebRenderWidgetSchedulingState;
class WidgetSchedulerImpl;

class PLATFORM_EXPORT MainThreadSchedulerImpl
    : public ThreadSchedulerBase,
      public MainThreadScheduler,
      public WebThreadScheduler,
      public IdleHelper::Delegate,
      public RenderWidgetSignals::Observer,
      public base::trace_event::TraceLog::AsyncEnabledStateObserver {
 public:
  // Duration after which rendering is considered starved, in which case the
  // compositor task queues will have an increased priority until the next
  // BeginMainFrame. Note that the elevated priority is lower than that of
  // render-blocking tasks, and a separate higher threshold is used to prevent
  // starvation by those tasks (see `kRenderBlockingStarvationThreshold`).
  static constexpr base::TimeDelta kDefaultRenderingStarvationThreshold =
      base::Milliseconds(100);

  // Duration after which rendering is considered starved by render-blocking
  // tasks, which is a safeguard against pathological cases for render-blocking
  // loading task prioritization.
  static constexpr base::TimeDelta kRenderBlockingStarvationThreshold =
      base::Milliseconds(500);

  // Tracks prioritization of the next frame. This is used in conjunction with
  // `UseCase` and other signals to compute the priority of the compositor task
  // queue.
  enum class RenderingPrioritizationState {
    // No prioritization for a specific frame.
    kNone,
    // A frame has not been produced after a certain threshold, so prioritize
    // the next frame (but don't block input).
    kRenderingStarved,
    // The total duration of render blocking tasks since the last frame exceeds
    // a certain threshold, so prioritize the next frame (matching
    // render-blocking priority).
    kRenderingStarvedByRenderBlocking,
    // The user is waiting for the result of a discrete input event, e.g. clicks
    // or typing. The next frame is prioritized at highest priority (matching
    // input).
    kWaitingForInputResponse,
  };

  // Don't use except for tracing.
  struct TaskDescriptionForTracing {
    TaskType task_type;
    std::optional<MainThreadTaskQueue::QueueType> queue_type;

    // Required in order to wrap in TraceableState.
    constexpr bool operator!=(const TaskDescriptionForTracing& rhs) const {
      return task_type != rhs.task_type || queue_type != rhs.queue_type;
    }
  };

  struct SchedulingSettings {
    SchedulingSettings();

    // If enabled, base::SingleThreadTaskRunner::GetCurrentDefault() and
    // base::SequencedTaskRunner::GetCurrentDefault() returns the current active
    // per-ASG task runner instead of the per-thread task runner.
    bool mbi_override_task_runner_handle;

    // The policy to use for discrete input-based task deferral. If
    // `features::kDeferRendererTasksAfterInput` is enabled, this is set to the
    // policy set in the associated feature param, otherwise this is
    // std::nullopt.
    std::optional<features::TaskDeferralPolicy>
        discrete_input_task_deferral_policy;

    bool input_scenario_priority_boost_enabled;
    bool input_scenario_priority_boost_includes_loading;
  };

  static const char* RAILModeToString(RAILMode rail_mode);

  explicit MainThreadSchedulerImpl(
      std::unique_ptr<base::sequence_manager::SequenceManager>
          sequence_manager);
  explicit MainThreadSchedulerImpl(
      base::sequence_manager::SequenceManager* sequence_manager);
  MainThreadSchedulerImpl(const MainThreadSchedulerImpl&) = delete;
  MainThreadSchedulerImpl& operator=(const MainThreadSchedulerImpl&) = delete;

  ~MainThreadSchedulerImpl() override;

  // WebThreadScheduler implementation:
  scoped_refptr<base::SingleThreadTaskRunner> DeprecatedDefaultTaskRunner()
      override;
  std::unique_ptr<MainThread> CreateMainThread() override;
  std::unique_ptr<WebAgentGroupScheduler> CreateWebAgentGroupScheduler()
      override;
  void SetRendererHidden(bool hidden) override;
  void SetRendererBackgrounded(bool backgrounded) override;
#if BUILDFLAG(IS_ANDROID)
  void PauseTimersForAndroidWebView() override;
  void ResumeTimersForAndroidWebView() override;
#endif
  void OnUrgentMessageReceived() override;
  void OnUrgentMessageProcessed() override;

  // WebThreadScheduler and ThreadScheduler implementation:
  void Shutdown() override;

  // MainThreadScheduler implementation:
  scoped_refptr<base::SingleThreadTaskRunner> NonWakingTaskRunner() override;
  [[nodiscard]] std::unique_ptr<MainThreadScheduler::RendererPauseHandle>
  PauseScheduler() override;
  v8::Isolate* Isolate() override;
  AgentGroupScheduler* CreateAgentGroupScheduler() override;
  AgentGroupScheduler* GetCurrentAgentGroupScheduler() override;
  void AddRAILModeObserver(RAILModeObserver* observer) override;
  void RemoveRAILModeObserver(RAILModeObserver const* observer) override;
  void ForEachMainThreadIsolate(
      base::RepeatingCallback<void(v8::Isolate* isolate)> callback) override;
  Vector<WebInputEventAttribution> GetPendingUserInputInfo(
      bool include_continuous) const override;
  void ExecuteAfterCurrentTaskForTesting(
      base::OnceClosure on_completion_task,
      ExecuteAfterCurrentTaskRestricted) override;
  void StartIdlePeriodForTesting() override;
  void SetRendererBackgroundedForTesting(bool backgrounded) override;

  // ThreadScheduler implementation:
  bool ShouldYieldForHighPriorityWork() override;
  void PostIdleTask(const base::Location&, Thread::IdleTask) override;
  void PostDelayedIdleTask(const base::Location&,
                           base::TimeDelta delay,
                           Thread::IdleTask) override;
  void RemoveCancelledIdleTasks() override;
  scoped_refptr<base::SingleThreadTaskRunner> V8TaskRunner() override;
  scoped_refptr<base::SingleThreadTaskRunner> V8UserVisibleTaskRunner()
      override;
  scoped_refptr<base::SingleThreadTaskRunner> V8BestEffortTaskRunner() override;
  scoped_refptr<base::SingleThreadTaskRunner> CleanupTaskRunner() override;
  base::TimeTicks MonotonicallyIncreasingVirtualTime() override;
  void AddTaskObserver(base::TaskObserver* task_observer) override;
  void RemoveTaskObserver(base::TaskObserver* task_observer) override;
  void SetV8Isolate(v8::Isolate* isolate) override;
  blink::MainThreadScheduler* ToMainThreadScheduler() override;

  // ThreadSchedulerBase implementation:
  scoped_refptr<base::SingleThreadTaskRunner> ControlTaskRunner() override;
  const base::TickClock* GetTickClock() const override;
  MainThreadSchedulerHelper& GetHelper() override { return helper_; }

  // RenderWidgetSignals::Observer implementation:
  void SetAllRenderWidgetsHidden(bool hidden) override;

  scoped_refptr<WidgetScheduler> CreateWidgetScheduler(
      WidgetScheduler::Delegate* delegate);
  void WillBeginFrame(const viz::BeginFrameArgs& args);
  void BeginFrameNotExpectedSoon();
  void BeginMainFrameNotExpectedUntil(base::TimeTicks time);
  void DidCommitFrameToCompositor();
  void DidHandleInputEventOnCompositorThread(
      const WebInputEvent& web_input_event,
      WidgetScheduler::InputEventState event_state);
  void WillPostInputEventToMainThread(
      WebInputEvent::Type web_input_event_type,
      const WebInputEventAttribution& web_input_event_attribution);
  void WillHandleInputEventOnMainThread(
      WebInputEvent::Type web_input_event_type,
      const WebInputEventAttribution& web_input_event_attribution);
  void DidHandleInputEventOnMainThread(const WebInputEvent& web_input_event,
                                       WebInputEventResult result,
                                       bool frame_requested);

  // Use a separate task runner so that IPC tasks are not logged via the same
  // task queue that executes them. Otherwise this would result in an infinite
  // loop of posting and logging to a single queue.
  scoped_refptr<base::SingleThreadTaskRunner>
  BackForwardCacheIpcTrackingTaskRunner() {
    return back_forward_cache_ipc_tracking_task_runner_;
  }

  scoped_refptr<base::SingleThreadTaskRunner> DefaultTaskRunner();

  scoped_refptr<SingleThreadIdleTaskRunner> IdleTaskRunner();
  base::TimeTicks NowTicks() const;

  // Returns a new task queue created with given params.
  scoped_refptr<MainThreadTaskQueue> NewTaskQueue(
      const MainThreadTaskQueue::QueueCreationParams& params);

  // Returns a new throttleable task queue to be used for tests.
  scoped_refptr<MainThreadTaskQueue> NewThrottleableTaskQueueForTest(
      FrameSchedulerImpl* frame_scheduler);

  scoped_refptr<base::sequence_manager::TaskQueue> NewTaskQueueForTest();

  void RemoveAgentGroupScheduler(AgentGroupSchedulerImpl*);
  void AddPageScheduler(PageSchedulerImpl*);
  void RemovePageScheduler(PageSchedulerImpl*);

  // Called by an associated PageScheduler when frozen or resumed.
  void OnPageFrozen(base::MemoryReductionTaskContext called_from);
  void OnPageResumed();

  void AddTaskTimeObserver(base::sequence_manager::TaskTimeObserver*);
  void RemoveTaskTimeObserver(base::sequence_manager::TaskTimeObserver*);

  // Snapshots this MainThreadSchedulerImpl for tracing.
  void CreateTraceEventObjectSnapshot() const;

  // Called when one of associated page schedulers has changed audio state.
  void OnAudioStateChanged();

  // Tells the scheduler that a provisional load has committed. Must be called
  // from the main thread.
  void DidStartProvisionalLoad(bool is_outermost_main_frame);

  // Tells the scheduler that a provisional load has committed. The scheduler
  // may reset the task cost estimators and the UserModel. Must be called from
  // the main thread.
  void DidCommitProvisionalLoad(bool is_web_history_inert_commit,
                                bool is_reload,
                                bool is_outermost_main_frame);

  // Note that the main's thread policy should be upto date to compute
  // the correct priority.
  TaskPriority ComputePriority(MainThreadTaskQueue* task_queue) const;

  // Test helpers.
  MainThreadSchedulerHelper* GetSchedulerHelperForTesting();
  IdleTimeEstimator* GetIdleTimeEstimatorForTesting();
  base::TimeTicks CurrentIdleTaskDeadlineForTesting() const;
  void EndIdlePeriodForTesting(base::TimeTicks time_remaining);
  bool PolicyNeedsUpdateForTesting();
  const IdleHelper& GetIdleHelperForTesting() const;

  std::unique_ptr<CPUTimeBudgetPool> CreateCPUTimeBudgetPoolForTesting(
      const char* name);

  // Virtual for test.
  virtual void OnMainFramePaint();

  void OnShutdownTaskQueue(const scoped_refptr<MainThreadTaskQueue>& queue);
  void OnDetachTaskQueue(MainThreadTaskQueue&);

  // TODO(crbug.com/1143007): Pass `queue` by reference now that the queue is
  // guaranteed to be alive.
  void OnTaskStarted(
      MainThreadTaskQueue* queue,
      const base::sequence_manager::Task& task,
      const base::sequence_manager::TaskQueue::TaskTiming& task_timing);

  // TODO(crbug.com/1143007): Pass `queue` by reference now that the queue is
  // guaranteed to be alive.
  void OnTaskCompleted(
      base::WeakPtr<MainThreadTaskQueue> queue,
      const base::sequence_manager::Task& task,
      base::sequence_manager::TaskQueue::TaskTiming* task_timing,
      base::LazyNow* lazy_now);

  void UpdateIpcTracking();
  void SetOnIPCTaskPostedWhileInBackForwardCacheIfNeeded();
  void OnIPCTaskPostedWhileInAllPagesBackForwardCache(
      uint32_t ipc_hash,
      const char* ipc_interface_name);
  void DetachOnIPCTaskPostedWhileInBackForwardCacheHandler();

  bool IsAudioPlaying() const;

  // base::trace_event::TraceLog::EnabledStateObserver implementation:
  void OnTraceLogEnabled() override;
  void OnTraceLogDisabled() override;

  UseCase current_use_case() const;

  const SchedulingSettings& scheduling_settings() const;

  void OnWebSchedulingTaskQueuePriorityChanged(MainThreadTaskQueue*);
  void OnWidgetSchedulerWillShutdown(WidgetSchedulerImpl*);

  base::WeakPtr<MainThreadSchedulerImpl> GetWeakPtr();

  TaskPriority compositor_priority() const {
    return main_thread_only().compositor_priority;
  }

  bool main_thread_compositing_is_fast() const {
    return main_thread_only().main_thread_compositing_is_fast;
  }

  TaskPriority find_in_page_priority() const {
    return main_thread_only().current_policy.find_in_page_priority;
  }

  base::TimeTicks CurrentTaskStartTime() const {
    return main_thread_only().current_task_start_time;
  }

 protected:
  // ThreadSchedulerBase implementation:
  WTF::Vector<base::OnceClosure>& GetOnTaskCompletionCallbacks() override;

  scoped_refptr<MainThreadTaskQueue> ControlTaskQueue();
  scoped_refptr<MainThreadTaskQueue> DefaultTaskQueue();
  scoped_refptr<MainThreadTaskQueue> V8TaskQueue();

  // `current_use_case` will be overwritten by the next call to UpdatePolicy.
  // Thus, this function should be only used for testing purposes.
  void SetCurrentUseCaseForTest(UseCase use_case) {
    main_thread_only().current_use_case = use_case;
  }

  virtual void PerformMicrotaskCheckpoint();

 private:
  friend class WebRenderWidgetSchedulingState;
  friend class MainThreadMetricsHelper;

  friend class MainThreadMetricsHelperTest;
  friend class frame_scheduler_impl_unittest::FrameSchedulerImplTest;
  friend class main_thread_scheduler_impl_unittest::
      MainThreadSchedulerImplForTest;
  friend class main_thread_scheduler_impl_unittest::MockPageSchedulerImpl;
  friend class main_thread_scheduler_impl_unittest::MainThreadSchedulerImplTest;

  friend class FindInPageBudgetPoolController;

  FRIEND_TEST_ALL_PREFIXES(
      main_thread_scheduler_impl_unittest::MainThreadSchedulerImplTest,
      Tracing);
  FRIEND_TEST_ALL_PREFIXES(
      main_thread_scheduler_impl_unittest::MainThreadSchedulerImplTest,
      LogIpcsPostedToDocumentsInBackForwardCache);
  FRIEND_TEST_ALL_PREFIXES(
      main_thread_scheduler_impl_unittest::MainThreadSchedulerImplTest,
      CanExceedIdleDeadlineIfRequired);

  enum class TimeDomainType {
    kReal,
    kVirtual,
  };

  // WebThreadScheduler private implementation:
  WebThreadScheduler* ToWebMainThreadScheduler() override;

  // ThreadSchedulerBase overrides
  base::SequencedTaskRunner* GetVirtualTimeTaskRunner() override;
  void OnVirtualTimeEnabled() override;
  void OnVirtualTimeDisabled() override;
  void OnVirtualTimePaused() override;
  void OnVirtualTimeResumed() override;

  static const char* TimeDomainTypeToString(TimeDomainType domain_type);

  void AddAgentGroupScheduler(AgentGroupSchedulerImpl*);

  struct AgentGroupSchedulerScope {
    std::unique_ptr<base::SingleThreadTaskRunner::CurrentDefaultHandle>
        single_thread_task_runner_current_handle_override;
    WeakPersistent<AgentGroupScheduler> previous_agent_group_scheduler;
    WeakPersistent<AgentGroupScheduler> current_agent_group_scheduler;
    scoped_refptr<base::SingleThreadTaskRunner> previous_task_runner;
    scoped_refptr<base::SingleThreadTaskRunner> current_task_runner;
    const char* trace_event_scope_name;
    raw_ptr<void> trace_event_scope_id;
  };

  void BeginAgentGroupSchedulerScope(
      AgentGroupScheduler* next_agent_group_scheduler);
  void EndAgentGroupSchedulerScope();

  bool IsAnyOrdinaryMainFrameWaitingForFirstContentfulPaint() const;
  bool IsAnyOrdinaryMainFrameWaitingForFirstMeaningfulPaint() const;
  bool IsAnyOrdinaryMainFrameLoading() const;

  struct Policy {
    DISALLOW_NEW();

   public:
    RAILMode rail_mode = RAILMode::kDefault;
    bool should_freeze_compositor_task_queue = false;
    bool should_pause_task_queues = false;
    bool should_pause_task_queues_for_android_webview = false;
    bool should_prioritize_ipc_tasks = false;
    TaskPriority find_in_page_priority =
        FindInPageBudgetPoolController::kFindInPageBudgetNotExhaustedPriority;
    UseCase use_case = UseCase::kNone;

    Policy() = default;
    ~Policy() = default;

    bool operator==(const Policy& other) const = default;

    bool IsQueueEnabled(MainThreadTaskQueue*, const SchedulingSettings&) const;
    void WriteIntoTrace(perfetto::TracedValue context) const;
  };

  class TaskDurationMetricTracker;

  class RendererPauseHandleImpl
      : public MainThreadScheduler::RendererPauseHandle {
   public:
    explicit RendererPauseHandleImpl(MainThreadSchedulerImpl* scheduler);
    ~RendererPauseHandleImpl() override;

   private:
    raw_ptr<MainThreadSchedulerImpl> scheduler_;  // NOT OWNED
  };

  // IdleHelper::Delegate implementation:
  bool CanEnterLongIdlePeriod(
      base::TimeTicks now,
      base::TimeDelta* next_long_idle_period_delay_out) override;
  void IsNotQuiescent() override {}
  void OnPendingTasksChanged(bool has_tasks) override;

  // Enables or disables the BeginMainFrameNotExpected signals from all widgets.
  void DispatchRequestBeginMainFrameNotExpected(bool has_tasks);

  // Requests the BeginMainFrameNotExpected signals for a single widget, which
  // is done asynchronously during initialization if needed.
  void InitializeRequestBeginMainFrameNotExpected(
      scoped_refptr<WidgetSchedulerImpl>);

  void EndIdlePeriod();

  // Update a policy which increases priority for the next beginMainFrame after
  // an input event.
  void UpdatePrioritizeCompositingAfterInputAfterTaskCompleted(
      MainThreadTaskQueue* queue);

  // Returns the serialized scheduler state for tracing.
  void WriteIntoTraceLocked(perfetto::TracedValue context,
                            base::TimeTicks optional_now) const;
  void CreateTraceEventObjectSnapshotLocked() const;

  // Shuts down empty detached task queues, which are being kept alive to run
  // pending tasks.
  void ShutdownEmptyDetachedTaskQueues();

  static bool ShouldPrioritizeInputEvent(const WebInputEvent& web_input_event);

  // The amount of time which idle periods can continue being scheduled when the
  // renderer has been hidden, before going to sleep for good.
  static const int kEndIdleWhenHiddenDelayMillis = 10000;

  // Schedules an immediate PolicyUpdate, if there isn't one already pending and
  // sets |policy_may_need_update_|. Note |any_thread_lock_| must be
  // locked.
  void EnsureUrgentPolicyUpdatePostedOnMainThread(
      const base::Location& from_here);

  // Update the policy if a new signal has arrived. Must be called from the main
  // thread.
  void MaybeUpdatePolicy();

  // Locks |any_thread_lock_| and updates the scheduler policy.  May early
  // out if the policy is unchanged. Must be called from the main thread.
  void UpdatePolicy();

  // Like UpdatePolicy, except it doesn't early out.
  void ForceUpdatePolicy();

  enum class UpdateType {
    kMayEarlyOutIfPolicyUnchanged,
    kForceUpdate,
  };

  // The implementation of UpdatePolicy & ForceUpdatePolicy.  It is allowed to
  // early out if |update_type| is kMayEarlyOutIfPolicyUnchanged.
  virtual void UpdatePolicyLocked(UpdateType update_type);

  // Helper for computing the use case. |expected_usecase_duration| will be
  // filled with the amount of time after which the use case should be updated
  // again. If the duration is zero, a new use case update should not be
  // scheduled. Must be called with |any_thread_lock_| held. Can be called from
  // any thread.
  UseCase ComputeCurrentUseCase(
      base::TimeTicks now,
      base::TimeDelta* expected_use_case_duration) const;

  // Helper for computing the RAILMode based on the given UseCase and current
  // scheduler state.
  RAILMode ComputeCurrentRAILMode(UseCase) const;

  // An input event of some sort happened, the policy may need updating.
  void UpdateForInputEventOnCompositorThread(
      const WebInputEvent& event,
      WidgetScheduler::InputEventState input_event_state);

  // The task cost estimators and the UserModel need to be reset upon page
  // nagigation. This function does that. Must be called from the main thread.
  void ResetForNavigationLocked();

  // Trigger an update to all task queues' priorities, throttling, and
  // enabled/disabled state based on current policy. When triggered from a
  // policy update, |previous_policy| should be populated with the pre-update
  // policy.
  void UpdateStateForAllTaskQueues(std::optional<Policy> previous_policy);

  void UpdateTaskQueueState(
      MainThreadTaskQueue* task_queue,
      base::sequence_manager::TaskQueue::QueueEnabledVoter*
          task_queue_enabled_voter,
      const Policy& old_policy,
      const Policy& new_policy,
      bool should_update_priority) const;

  void PauseRendererImpl();
  void ResumeRendererImpl();

  // Returns true if there is a change in the main thread's policy that should
  // trigger a priority update.
  bool ShouldUpdateTaskQueuePriorities(Policy new_policy) const;

  // Computes compositor priority based on various experiments and
  // the use case. Defaults to kNormalPriority.
  TaskPriority ComputeCompositorPriority() const;

  // Used to update the compositor priority on the main thread.
  void UpdateCompositorTaskQueuePriority();

  // Updates the policy state associated with the current task and updates the
  // policy if necessary.
  void MaybeUpdatePolicyOnTaskCompleted(
      MainThreadTaskQueue*,
      const base::sequence_manager::TaskQueue::TaskTiming&);

  // Updates the current `RenderingPrioritizationState` used to set the
  // compositor task queue priority after the current task has finished.
  void UpdateRenderingPrioritizationStateOnTaskCompleted(
      MainThreadTaskQueue*,
      const base::sequence_manager::TaskQueue::TaskTiming&);

  // Computes the priority for compositing based on the current use case.
  // Returns nullopt if the use case does not need to set the priority.
  std::optional<TaskPriority> ComputeCompositorPriorityFromUseCase() const;

  // Computes the compositor task queue priority for the next main frame based
  // on the current `RenderingPrioritizationState`.
  std::optional<TaskPriority> ComputeCompositorPriorityForMainFrame() const;

  void ShutdownAllQueues();

  bool AllPagesFrozen() const;

  // Indicates that scheduler has been shutdown.
  // It should be accessed only on the main thread, but couldn't be a member
  // of MainThreadOnly struct because last might be destructed before we
  // have to check this flag during scheduler's destruction.
  bool was_shutdown_ = false;

  bool has_ipc_callback_set_ = false;
  bool IsIpcTrackingEnabledForAllPages();

  // This controller should be initialized before any TraceableVariables
  // because they require one to initialize themselves.
  TraceableVariableController tracing_controller_;

  // Used for experiments on finch. On main thread instantiation, we cache
  // the values of base::Feature flags using this struct, since calling
  // base::Feature::IsEnabled is a relatively expensive operation.
  //
  // Note that it is important to keep this as the first member to ensure it is
  // initialized first and can be used everywhere.
  const SchedulingSettings scheduling_settings_;

  raw_ptr<base::sequence_manager::SequenceManager> sequence_manager_;
  std::unique_ptr<base::sequence_manager::SequenceManager>
      owned_sequence_manager_;
  MainThreadSchedulerHelper helper_;
  scoped_refptr<MainThreadTaskQueue> idle_helper_queue_;
  std::unique_ptr<base::sequence_manager::TaskQueue::QueueEnabledVoter>
      idle_queue_voter_;
  IdleHelper idle_helper_;
  RenderWidgetSignals render_widget_scheduler_signals_;

  std::unique_ptr<FindInPageBudgetPoolController>
      find_in_page_budget_pool_controller_;

  const scoped_refptr<MainThreadTaskQueue> control_task_queue_;
  scoped_refptr<MainThreadTaskQueue> virtual_time_control_task_queue_;
  scoped_refptr<MainThreadTaskQueue>
      back_forward_cache_ipc_tracking_task_queue_;

  using TaskQueueVoterMap ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327)") =
      std::map<scoped_refptr<MainThreadTaskQueue>,
               std::unique_ptr<
                   base::sequence_manager::TaskQueue::QueueEnabledVoter>>;

  TaskQueueVoterMap task_runners_;

  scoped_refptr<MainThreadTaskQueue> v8_task_queue_;
  scoped_refptr<MainThreadTaskQueue> v8_user_visible_task_queue_;
  scoped_refptr<MainThreadTaskQueue> v8_best_effort_task_queue_;
  scoped_refptr<MainThreadTaskQueue> memory_purge_task_queue_;
  scoped_refptr<MainThreadTaskQueue> non_waking_task_queue_;

  scoped_refptr<base::SingleThreadTaskRunner> v8_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> v8_user_visible_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> v8_best_effort_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> control_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> non_waking_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner>
      back_forward_cache_ipc_tracking_task_runner_;

  MemoryPurgeManager memory_purge_manager_;

  base::RepeatingClosure update_policy_closure_;
  DeadlineTaskRunner delayed_update_policy_runner_;
  CancelableClosureHolder end_renderer_hidden_idle_period_closure_;

  // We have decided to improve thread safety at the cost of some boilerplate
  // (the accessors) for the following data members.
  struct MainThreadOnly {
    MainThreadOnly(
        MainThreadSchedulerImpl* main_thread_scheduler_impl,
        const base::TickClock* time_source,
        base::TimeTicks now);
    ~MainThreadOnly();

    IdleTimeEstimator idle_time_estimator;
    TraceableState<UseCase, "renderer.scheduler"> current_use_case;
    Policy current_policy;
    base::TimeTicks current_policy_expiration_time;
    base::TimeTicks estimated_next_frame_begin;
    base::TimeTicks current_task_start_time;
    base::TimeTicks discrete_input_response_start_time;
    base::TimeDelta compositor_frame_interval;
    TraceableCounter<int, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        renderer_pause_count;  // Renderer is paused if non-zero.

    bool renderer_hidden = false;
    std::optional<base::ScopedSampleMetadata> renderer_hidden_metadata;
    std::optional<base::ScopedSampleMetadata> renderer_frozen_metadata;
    bool renderer_backgrounded = kLaunchingProcessIsBackgrounded;
    TraceableState<bool, "renderer.scheduler"> blocking_input_expected_soon;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler.debug")>
        in_idle_period_for_testing;
    TraceableState<bool, "renderer"> is_audio_playing;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler.debug")>
        compositor_will_send_main_frame_not_expected;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler.debug")>
        has_navigated;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler.debug")>
        pause_timers_for_webview;
    base::TimeTicks background_status_changed_at;
    HashSet<PageSchedulerImpl*> page_schedulers;  // Not owned.
    base::ObserverList<RAILModeObserver>::Unchecked
        rail_mode_observers;  // Not owned.
    MainThreadMetricsHelper metrics_helper;
    TraceableState<std::optional<TaskDescriptionForTracing>,
                   TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        task_description_for_tracing;  // Don't use except for tracing.
    TraceableState<std::optional<TaskPriority>,
                   TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        task_priority_for_tracing;  // Only used for tracing.

    // Holds task queues that are currently running.
    // The queue for the inmost task is at the top of stack when there are
    // nested RunLoops.
    std::stack<scoped_refptr<MainThreadTaskQueue>,
               std::vector<scoped_refptr<MainThreadTaskQueue>>>
        running_queues;

    // List of callbacks to execute after the current task.
    WTF::Vector<base::OnceClosure> on_task_completion_callbacks;

    bool main_thread_compositing_is_fast;

    // Priority given to the main thread's compositor task queue. Defaults to
    // kNormalPriority and is updated via UpdateCompositorTaskQueuePriority().
    // After 100ms with nothing running from this queue, the compositor will
    // be set to kVeryHighPriority until a frame is run.
    TraceableState<TaskPriority, "renderer.scheduler"> compositor_priority;

    TraceableState<RenderingPrioritizationState, "renderer.scheduler">
        main_frame_prioritization_state;
    // Signals needed to compute the `main_frame_prioritization_state`.
    base::TimeTicks last_frame_time;
    bool is_current_task_main_frame = false;
    // Set when a discrete input event is handled on the main thread. This is
    // used by the kPrioritizeCompositingAfterInput experiment to determine if
    // the next frame should be prioritized.
    bool is_current_task_discrete_input = false;
    // Set when a frame is known to be requested when handling an input event on
    // the main thread.
    bool is_frame_requested_after_discrete_input = false;
    // Cumulative non-continuous time spent running render-blocking tasks since
    // the last frame.
    base::TimeDelta rendering_blocking_duration_since_last_frame;

    WTF::Vector<AgentGroupSchedulerScope> agent_group_scheduler_scope_stack;

    Persistent<GCedHeapHashSet<WeakMember<AgentGroupSchedulerImpl>>>
        agent_group_schedulers;
    // Task queues that have been detached from their scheduler and may have
    // pending tasks that need to run.
    WTF::HashSet<scoped_refptr<MainThreadTaskQueue>> detached_task_queues;

    // Temporarily boosts the main thread priority. Only used if
    // kInputScenarioPriorityBoost is enabled.
    std::optional<base::ScopedBoostPriority> main_thread_priority_boost;

    // `WidgetScheduler`s that have not been shut down.
    WTF::HashSet<scoped_refptr<WidgetSchedulerImpl>> widget_schedulers;
  };

  struct AnyThread {
    explicit AnyThread(MainThreadSchedulerImpl* main_thread_scheduler_impl);
    ~AnyThread();

    PendingUserInput::Monitor pending_input_monitor;
    UserModel user_model;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        awaiting_touch_start_response;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        awaiting_discrete_input_response;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        begin_main_frame_on_critical_path;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        last_gesture_was_compositor_driven;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        default_gesture_prevented;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        have_seen_a_blocking_gesture;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        waiting_for_any_main_frame_contentful_paint;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        waiting_for_any_main_frame_meaningful_paint;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        is_any_main_frame_loading;
    TraceableState<bool, TRACE_DISABLED_BY_DEFAULT("renderer.scheduler")>
        have_seen_input_since_navigation;
  };

  struct CompositorThreadOnly {
    CompositorThreadOnly();
    ~CompositorThreadOnly();

    WebInputEvent::Type last_input_type;
    std::unique_ptr<base::ThreadChecker> compositor_thread_checker;

    void CheckOnValidThread() {
#if DCHECK_IS_ON()
      // We don't actually care which thread this called from, just so long as
      // its consistent.
      if (!compositor_thread_checker)
        compositor_thread_checker.reset(new base::ThreadChecker());
      DCHECK(compositor_thread_checker->CalledOnValidThread());
#endif
    }
  };

  // Don't access main_thread_only_, instead use main_thread_only().
  MainThreadOnly main_thread_only_;
  MainThreadOnly& main_thread_only() {
    helper_.CheckOnValidThread();
    return main_thread_only_;
  }
  const struct MainThreadOnly& main_thread_only() const {
    helper_.CheckOnValidThread();
    return main_thread_only_;
  }

  mutable base::Lock any_thread_lock_;
  // Don't access any_thread_, instead use any_thread().
  AnyThread any_thread_;
  AnyThread& any_thread() {
    any_thread_lock_.AssertAcquired();
    return any_thread_;
  }
  const struct AnyThread& any_thread() const {
    any_thread_lock_.AssertAcquired();
    return any_thread_;
  }

  // Don't access compositor_thread_only_, instead use
  // |GetCompositorThreadOnly()|.
  CompositorThreadOnly compositor_thread_only_;
  CompositorThreadOnly& GetCompositorThreadOnly() {
    compositor_thread_only_.CheckOnValidThread();
    return compositor_thread_only_;
  }

  PollableThreadSafeFlag policy_may_need_update_;
  WeakPersistent<AgentGroupScheduler> current_agent_group_scheduler_;

  // This is accessed from both the main and IO (IPC) threads. It's incremented
  // when an urgent IPC task is posted and decremented when that IPC task runs
  // (or doesn't, e.g. if the interface is closed). This gets checked at the end
  // of every task to determine if the policy should be updated.
  std::atomic<uint64_t> num_pending_urgent_ipc_messages_{0};

  base::WeakPtrFactory<MainThreadSchedulerImpl> weak_factory_{this};
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_MAIN_THREAD_SCHEDULER_IMPL_H_
