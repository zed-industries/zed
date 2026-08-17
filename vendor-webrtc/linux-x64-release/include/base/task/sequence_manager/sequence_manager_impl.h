// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_SEQUENCE_MANAGER_IMPL_H_
#define BASE_TASK_SEQUENCE_MANAGER_SEQUENCE_MANAGER_IMPL_H_

#include <atomic>
#include <deque>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>

#include "base/atomic_sequence_num.h"
#include "base/base_export.h"
#include "base/callback_list.h"
#include "base/compiler_specific.h"
#include "base/containers/circular_deque.h"
#include "base/debug/crash_logging.h"
#include "base/feature_list.h"
#include "base/functional/callback_forward.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/message_loop/message_pump_type.h"
#include "base/observer_list.h"
#include "base/pending_task.h"
#include "base/rand_util.h"
#include "base/run_loop.h"
#include "base/synchronization/lock.h"
#include "base/task/current_thread.h"
#include "base/task/sequence_manager/associated_thread_id.h"
#include "base/task/sequence_manager/enqueue_order.h"
#include "base/task/sequence_manager/enqueue_order_generator.h"
#include "base/task/sequence_manager/sequence_manager.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/task/sequence_manager/task_queue_impl.h"
#include "base/task/sequence_manager/task_queue_selector.h"
#include "base/task/sequence_manager/thread_controller.h"
#include "base/task/sequence_manager/work_tracker.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/default_tick_clock.h"
#include "base/types/pass_key.h"
#include "base/values.h"
#include "build/build_config.h"

namespace base {

namespace internal {
class SequenceManagerThreadDelegate;
}

namespace trace_event {
class ConvertableToTraceFormat;
}  // namespace trace_event

namespace sequence_manager {

class SequenceManagerForTest;
class TaskQueue;
class TaskTimeObserver;
class TimeDomain;

namespace internal {

class TaskQueueImpl;
class DefaultWakeUpQueue;
class SequenceManagerImpl;
class ThreadControllerImpl;

// A private factory method for SequenceManagerThreadDelegate which is
// equivalent to sequence_manager::CreateUnboundSequenceManager() but returns
// the underlying impl.
std::unique_ptr<SequenceManagerImpl> CreateUnboundSequenceManagerImpl(
    PassKey<base::internal::SequenceManagerThreadDelegate>,
    SequenceManager::Settings settings);

// The task queue manager provides N task queues and a selector interface for
// choosing which task queue to service next. Each task queue consists of two
// sub queues:
//
// 1. Incoming task queue. Tasks that are posted get immediately appended here.
//    When a task is appended into an empty incoming queue, the task manager
//    work function (DoWork()) is scheduled to run on the main task runner.
//
// 2. Work queue. If a work queue is empty when DoWork() is entered, tasks from
//    the incoming task queue (if any) are moved here. The work queues are
//    registered with the selector as input to the scheduling decision.
//
class BASE_EXPORT SequenceManagerImpl
    : public SequenceManager,
      public internal::SequencedTaskSource,
      public internal::TaskQueueSelector::Observer,
      public RunLoop::NestingObserver {
 public:
  using Observer = SequenceManager::Observer;

  SequenceManagerImpl(const SequenceManagerImpl&) = delete;
  SequenceManagerImpl& operator=(const SequenceManagerImpl&) = delete;
  ~SequenceManagerImpl() override;

  // Initializes features for this class. See `base::features::Init()`.
  static void InitializeFeatures();

  // SequenceManager implementation:
  void BindToCurrentThread() override;
  scoped_refptr<SequencedTaskRunner> GetTaskRunnerForCurrentTask() override;
  void BindToMessagePump(std::unique_ptr<MessagePump> message_pump) override;
  void SetObserver(Observer* observer) override;
  void AddTaskTimeObserver(TaskTimeObserver* task_time_observer) override;
  void RemoveTaskTimeObserver(TaskTimeObserver* task_time_observer) override;
  void SetTimeDomain(TimeDomain* time_domain) override;
  void ResetTimeDomain() override;
  const TickClock* GetTickClock() const override;
  TimeTicks NowTicks() const override;
  void SetDefaultTaskRunner(
      scoped_refptr<SingleThreadTaskRunner> task_runner) override;
  void ReclaimMemory() override;
  bool GetAndClearSystemIsQuiescentBit() override;
  void SetWorkBatchSize(int work_batch_size) override;
  void EnableCrashKeys(const char* async_stack_crash_key) override;
  size_t GetPendingTaskCountForTesting() const override;
  TaskQueue::Handle CreateTaskQueue(const TaskQueue::Spec& spec) override;
  std::string DescribeAllPendingTasks() const override;
  void AddTaskObserver(TaskObserver* task_observer) override;
  void RemoveTaskObserver(TaskObserver* task_observer) override;
  std::optional<WakeUp> GetNextDelayedWakeUp() const override;
  TaskQueue::QueuePriority GetPriorityCount() const override;

  // SequencedTaskSource implementation:
  void SetRunTaskSynchronouslyAllowed(
      bool can_run_tasks_synchronously) override;
  using internal::SequencedTaskSource::SelectNextTask;
  std::optional<SelectedTask> SelectNextTask(LazyNow& lazy_now,
                                             SelectTaskOption option) override;
  void DidRunTask(LazyNow& lazy_now) override;
  using internal::SequencedTaskSource::GetPendingWakeUp;
  std::optional<WakeUp> GetPendingWakeUp(LazyNow* lazy_now,
                                         SelectTaskOption option) override;
#if BUILDFLAG(IS_WIN)
  bool NextWakeUpNeedsHighRes() override;
#endif
  void OnBeginWork() override;
  bool OnIdle() override;
  void MaybeEmitTaskDetails(
      perfetto::EventContext& ctx,
      const SequencedTaskSource::SelectedTask& selected_task) const override;

  void AddDestructionObserver(
      CurrentThread::DestructionObserver* destruction_observer);
  void RemoveDestructionObserver(
      CurrentThread::DestructionObserver* destruction_observer);
  [[nodiscard]] CallbackListSubscription RegisterOnNextIdleCallback(
      OnceClosure on_next_idle_callback);

  // Sets / returns the default TaskRunner. Thread-safe.
  void SetTaskRunner(scoped_refptr<SingleThreadTaskRunner> task_runner);
  scoped_refptr<SingleThreadTaskRunner> GetTaskRunner();

  bool IsBoundToCurrentThread() const;
  MessagePump* GetMessagePump() const;
  bool IsType(MessagePumpType type) const;
  void SetAddQueueTimeToTasks(bool enable);
  void SetTaskExecutionAllowedInNativeNestedLoop(bool allowed);
  bool IsTaskExecutionAllowedInNativeNestedLoop() const;
#if BUILDFLAG(IS_IOS)
  void AttachToMessagePump();
#endif
  bool IsIdleForTesting() override;
  void EnableMessagePumpTimeKeeperMetrics(
      const char* thread_name,
      bool wall_time_based_metrics_enabled_for_testing = false);

  // Requests that a task to process work is scheduled.
  void ScheduleWork();

  // Returns the currently executing TaskQueue if any. Must be called on the
  // thread this class was created on.
  internal::TaskQueueImpl* currently_executing_task_queue() const;

  // Unregisters a TaskQueue previously created by |NewTaskQueue()|.
  // No tasks will run on this queue after this call.
  void UnregisterTaskQueueImpl(
      std::unique_ptr<internal::TaskQueueImpl> task_queue);

  scoped_refptr<AssociatedThreadId> associated_thread() const {
    return associated_thread_;
  }

  const Settings& settings() const LIFETIME_BOUND { return settings_; }

  WeakPtr<SequenceManagerImpl> GetWeakPtr();

  // How frequently to perform housekeeping tasks (sweeping canceled tasks etc).
  static constexpr TimeDelta kReclaimMemoryInterval = Seconds(30);

 protected:
  static std::unique_ptr<ThreadControllerImpl>
  CreateThreadControllerImplForCurrentThread(const TickClock* clock);

  // Create a task queue manager where |controller| controls the thread
  // on which the tasks are eventually run.
  SequenceManagerImpl(std::unique_ptr<internal::ThreadController> controller,
                      SequenceManager::Settings settings = Settings());

  friend class internal::TaskQueueImpl;
  friend class internal::DefaultWakeUpQueue;
  friend class ::base::sequence_manager::SequenceManagerForTest;

 private:
  // Returns the SequenceManager running the
  // current thread. It must only be used on the thread it was obtained.
  // Only to be used by CurrentThread for the moment
  static SequenceManagerImpl* GetCurrent();
  friend class ::base::CurrentThread;

  // Factory friends to call into private creation methods.
  friend std::unique_ptr<SequenceManager>
      sequence_manager::CreateSequenceManagerOnCurrentThread(
          SequenceManager::Settings);
  friend std::unique_ptr<SequenceManager>
  sequence_manager::CreateSequenceManagerOnCurrentThreadWithPump(
      std::unique_ptr<MessagePump> message_pump,
      SequenceManager::Settings);
  friend std::unique_ptr<SequenceManager>
      sequence_manager::CreateUnboundSequenceManager(SequenceManager::Settings);
  friend std::unique_ptr<SequenceManagerImpl>
      sequence_manager::internal::CreateUnboundSequenceManagerImpl(
          PassKey<base::internal::SequenceManagerThreadDelegate>,
          SequenceManager::Settings);

  // Assume direct control over current thread and create a SequenceManager.
  // This function should be called only once per thread.
  // This function assumes that a task execution environment is already
  // initialized for the current thread.
  static std::unique_ptr<SequenceManagerImpl> CreateOnCurrentThread(
      SequenceManager::Settings settings);

  // Create an unbound SequenceManager (typically for a future thread). The
  // SequenceManager can be initialized on the current thread and then needs to
  // be bound and initialized on the target thread by calling one of the Bind*()
  // methods.
  static std::unique_ptr<SequenceManagerImpl> CreateUnbound(
      SequenceManager::Settings settings);

  enum class ProcessTaskResult {
    kDeferred,
    kExecuted,
    kSequenceManagerDeleted,
  };

  // SequenceManager maintains a queue of non-nestable tasks since they're
  // uncommon and allocating an extra deque per TaskQueue will waste the memory.
  using NonNestableTaskDeque =
      circular_deque<internal::TaskQueueImpl::DeferredNonNestableTask>;

  // We have to track rentrancy because we support nested runloops but the
  // selector interface is unaware of those.  This struct keeps track off all
  // task related state needed to make pairs of SelectNextTask() / DidRunTask()
  // work.
  struct ExecutingTask {
    ExecutingTask(Task&& task,
                  internal::TaskQueueImpl* task_queue,
                  TaskQueue::TaskTiming task_timing)
        : pending_task(std::move(task)),
          task_queue(task_queue),
          task_queue_name(task_queue->GetProtoName()),
          task_timing(task_timing),
          priority(task_queue->GetQueuePriority()),
          task_type(pending_task.task_type) {}

    Task pending_task;

    // `task_queue` is not a raw_ptr<...> for performance reasons (based on
    // analysis of sampling profiler data and tab_search:top100:2020).
    RAW_PTR_EXCLUSION internal::TaskQueueImpl* task_queue = nullptr;
    // Save task_queue_name as the task queue can be deleted within the task.
    QueueName task_queue_name;
    TaskQueue::TaskTiming task_timing;
    // Save priority as it might change after running a task.
    TaskQueue::QueuePriority priority;
    // Save task metadata to use in after running a task as |pending_task|
    // won't be available then.
    int task_type;
  };

  struct MainThreadOnly {
    explicit MainThreadOnly(
        SequenceManagerImpl* sequence_manager,
        const scoped_refptr<AssociatedThreadId>& associated_thread,
        const SequenceManager::Settings& settings,
        const base::TickClock* clock);
    ~MainThreadOnly();

    int nesting_depth = 0;
    NonNestableTaskDeque non_nestable_task_queue;
    // TODO(altimin): Switch to instruction pointer crash key when it's
    // available.
    raw_ptr<debug::CrashKeyString> file_name_crash_key = nullptr;
    raw_ptr<debug::CrashKeyString> function_name_crash_key = nullptr;
    raw_ptr<debug::CrashKeyString> async_stack_crash_key = nullptr;
    std::array<char, static_cast<size_t>(debug::CrashKeySize::Size64)>
        async_stack_buffer = {};

    internal::TaskQueueSelector selector;
    // RAW_PTR_EXCLUSION: Performance reasons(based on analysis of
    // speedometer3).
    ObserverList<TaskObserver>::UncheckedAndRawPtrExcluded task_observers;
    ObserverList<TaskTimeObserver> task_time_observers;
    const raw_ptr<const base::TickClock> default_clock;
    raw_ptr<TimeDomain> time_domain = nullptr;

    std::unique_ptr<WakeUpQueue> wake_up_queue;
    std::unique_ptr<WakeUpQueue> non_waking_wake_up_queue;

    // If true MaybeReclaimMemory will attempt to reclaim memory.
    bool memory_reclaim_scheduled = false;

    // Used to ensure we don't perform expensive housekeeping too frequently.
    TimeTicks next_time_to_reclaim_memory;

    // List of task queues managed by this SequenceManager.
    // - active_queues contains queues that are still running tasks, which are
    //   are owned by relevant TaskQueues.
    // - queues_to_delete contains soon-to-be-deleted queues, because some
    //   internal scheduling code does not expect queues to be pulled
    //   from underneath.

    // RAW_PTR_EXCLUSION: Performance reasons (based on analysis of
    // speedometer3).
    RAW_PTR_EXCLUSION std::set<internal::TaskQueueImpl*> active_queues;

    std::map<internal::TaskQueueImpl*, std::unique_ptr<internal::TaskQueueImpl>>
        queues_to_delete;

    bool task_was_run_on_quiescence_monitored_queue = false;
    bool nesting_observer_registered_ = false;

    // Use std::deque() so that references returned by SelectNextTask() remain
    // valid until the matching call to DidRunTask(), even when nested RunLoops
    // cause tasks to be pushed on the stack in-between. This is needed because
    // references are kept in local variables by calling code between
    // SelectNextTask()/DidRunTask().
    std::deque<ExecutingTask> task_execution_stack;

    raw_ptr<Observer> observer = nullptr;  // NOT OWNED

    ObserverList<CurrentThread::DestructionObserver>::
        UncheckedAndDanglingUntriaged destruction_observers;

    // Notified the next time `OnIdle()` completes without scheduling additional
    // work.
    OnceClosureList on_next_idle_callbacks;
  };

  void CompleteInitializationOnBoundThread();

  // TaskQueueSelector::Observer:
  void OnTaskQueueEnabled(internal::TaskQueueImpl* queue) override;
  void OnWorkAvailable() override;

  // RunLoop::NestingObserver:
  void OnBeginNestedRunLoop() override;
  void OnExitNestedRunLoop() override;

  // Schedules next wake-up at the given time, canceling any previous requests.
  // Use std::nullopt to cancel a wake-up. Must be called on the thread this
  // class was created on.
  void SetNextWakeUp(LazyNow* lazy_now, std::optional<WakeUp> wake_up);

  // Called before TaskQueue requests to reload its empty immediate work queue.
  void WillRequestReloadImmediateWorkQueue();

  // Returns a valid `SyncWorkAuthorization` if a call to `RunOrPostTask` on a
  // `SequencedTaskRunner` bound to this `SequenceManager` may run its task
  // synchronously.
  SyncWorkAuthorization TryAcquireSyncWorkAuthorization();

  // Called when a task is about to be queued. May add metadata to the task and
  // emit trace events.
  void WillQueueTask(Task* pending_task);

  // Enqueues onto delayed WorkQueues all delayed tasks which must run now
  // (cannot be postponed) and possibly some delayed tasks which can run now but
  // could be postponed (due to how tasks are stored, it is not possible to
  // retrieve all such tasks efficiently) and reloads any empty work queues.
  void MoveReadyDelayedTasksToWorkQueues(LazyNow* lazy_now);

  void NotifyWillProcessTask(ExecutingTask* task, LazyNow* time_before_task);
  void NotifyDidProcessTask(ExecutingTask* task, LazyNow* time_after_task);

  EnqueueOrder GetNextSequenceNumber();

  bool GetAddQueueTimeToTasks();

  std::unique_ptr<trace_event::ConvertableToTraceFormat>
  AsValueWithSelectorResultForTracing(internal::WorkQueue* selected_work_queue,
                                      bool force_verbose) const;
  Value::Dict AsValueWithSelectorResult(
      internal::WorkQueue* selected_work_queue,
      bool force_verbose) const;

  // Used in construction of TaskQueueImpl to obtain an AtomicFlag which it can
  // use to request reload by ReloadEmptyWorkQueues. The lifetime of
  // TaskQueueImpl is managed by this class and the handle will be released by
  // TaskQueueImpl::UnregisterTaskQueue which is always called before the
  // queue's destruction.
  AtomicFlagSet::AtomicFlag GetFlagToRequestReloadForEmptyQueue(
      TaskQueueImpl* task_queue);

  // Calls |TakeImmediateIncomingQueueTasks| on all queues with their reload
  // flag set in |empty_queues_to_reload_|.
  void ReloadEmptyWorkQueues();

  std::unique_ptr<internal::TaskQueueImpl> CreateTaskQueueImpl(
      const TaskQueue::Spec& spec);

  // Periodically reclaims memory by sweeping away canceled tasks and shrinking
  // buffers.
  void MaybeReclaimMemory();

  // Deletes queues marked for deletion and empty queues marked for shutdown.
  void CleanUpQueues();

  // Removes canceled delayed tasks from the front of wake up queue.
  void RemoveAllCanceledDelayedTasksFromFront(LazyNow* lazy_now);

  TaskQueue::TaskTiming::TimeRecordingPolicy ShouldRecordTaskTiming(
      const internal::TaskQueueImpl* task_queue);

  // Write the async stack trace onto a crash key as whitespace-delimited hex
  // addresses.
  void RecordCrashKeys(const PendingTask&);

  // Helper to terminate all scoped trace events to allow starting new ones
  // in SelectNextTask().
  std::optional<SelectedTask> SelectNextTaskImpl(LazyNow& lazy_now,
                                                 SelectTaskOption option);

  // Returns a wake-up for the next delayed task which is not ripe for
  // execution, or nullopt if `option` is `kSkipDelayedTask` or there
  // are no such tasks (immediate tasks don't count).
  std::optional<WakeUp> GetNextDelayedWakeUpWithOption(
      SelectTaskOption option) const;

  // Given a `wake_up` describing when the next delayed task should run, returns
  // a wake up that should be scheduled on the thread. `is_immediate()` if the
  // wake up should run immediately. `nullopt` if no wake up is required because
  // `wake_up` is `nullopt` or a `time_domain` is used.
  std::optional<WakeUp> AdjustWakeUp(std::optional<WakeUp> wake_up,
                                     LazyNow* lazy_now) const;

  void MaybeAddLeewayToTask(Task& task) const;

#if DCHECK_IS_ON()
  void LogTaskDebugInfo(const internal::WorkQueue* work_queue) const;
#endif

  // Determines if wall time or thread time should be recorded for the next
  // task.
  TaskQueue::TaskTiming InitializeTaskTiming(
      internal::TaskQueueImpl* task_queue);

  const scoped_refptr<AssociatedThreadId> associated_thread_;

  EnqueueOrderGenerator enqueue_order_generator_;

  const std::unique_ptr<internal::ThreadController> controller_;
  const Settings settings_;

  WorkTracker work_tracker_;

  // Whether to add the queue time to tasks.
  std::atomic<bool> add_queue_time_to_tasks_;

  AtomicFlagSet empty_queues_to_reload_;

  MainThreadOnly main_thread_only_;
  MainThreadOnly& main_thread_only() {
    DCHECK_CALLED_ON_VALID_THREAD(associated_thread_->thread_checker);
    return main_thread_only_;
  }
  const MainThreadOnly& main_thread_only() const LIFETIME_BOUND {
    DCHECK_CALLED_ON_VALID_THREAD(associated_thread_->thread_checker);
    return main_thread_only_;
  }

  // |clock_| either refers to the TickClock representation of |time_domain|
  // (same object) if any, or to |default_clock| otherwise. It is maintained as
  // an atomic pointer here for multi-threaded usage.
  std::atomic<const base::TickClock*> clock_;
  const base::TickClock* main_thread_clock() const {
    DCHECK_CALLED_ON_VALID_THREAD(associated_thread_->thread_checker);
    return clock_.load(std::memory_order_relaxed);
  }
  const base::TickClock* any_thread_clock() const {
    // |memory_order_acquire| matched by |memory_order_release| in
    // SetTimeDomain() to ensure all data used by |clock_| is visible when read
    // from the current thread. A thread might try to access a stale |clock_|
    // but that's not an issue since |time_domain| contractually outlives
    // SequenceManagerImpl even if it's reset.
    return clock_.load(std::memory_order_acquire);
  }

  WeakPtrFactory<SequenceManagerImpl> weak_factory_{this};
};

}  // namespace internal
}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_SEQUENCE_MANAGER_IMPL_H_
