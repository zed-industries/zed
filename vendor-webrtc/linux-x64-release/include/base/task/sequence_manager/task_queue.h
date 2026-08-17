// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_TASK_QUEUE_H_
#define BASE_TASK_SEQUENCE_MANAGER_TASK_QUEUE_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <type_traits>

#include "base/base_export.h"
#include "base/check.h"
#include "base/task/common/checked_lock.h"
#include "base/task/common/lazy_now.h"
#include "base/task/sequence_manager/tasks.h"
#include "base/task/single_thread_task_runner.h"
#include "base/task/task_observer.h"
#include "base/threading/platform_thread.h"
#include "base/time/time.h"
#include "base/trace_event/base_tracing.h"
#include "base/trace_event/base_tracing_forward.h"

namespace perfetto {
class EventContext;
}

namespace base {

class TaskObserver;

namespace sequence_manager {

using QueueName = ::perfetto::protos::pbzero::SequenceManagerTask::QueueName;

namespace internal {
class SequenceManagerImpl;
class TaskQueueImpl;
}  // namespace internal

// A `TaskQueue` represents an ordered list of tasks sharing common properties,
// e.g. priority, throttling, etc. `TaskQueue`s are associated with a
// `SequenceManager` instance, which chooses the next task from its set of
// queues. `TaskQueue`s should typically be used on a single thread since most
// methods are not thread safe (enforeced via CHECKs), but cross-thread task
// posting is supported with thread-safe task runners.
//
// A `TaskQueue` is unregistered (stops accepting and running tasks) when either
// its associated `TaskQueue::Handle` or `SequenceManager` is destroyed. If the
// handle is destroyed while the `SequenceManager` is still alive, the
// `SequenceManager` takes ownership of the queue and schedules it for deletion
// after the current task finishes. Otherwise, if the handle outlives the
// sequence manager, the queue is destroyed when the handle is destroyed.
class BASE_EXPORT TaskQueue {
 public:
  // Interface that lets a task queue be throttled by changing the wake up time
  // and optionally, by inserting fences. A wake up in this context is a
  // notification at a given time that lets this TaskQueue know of newly ripe
  // delayed tasks if it's enabled. By delaying the desired wake up time to a
  // different allowed wake up time, the Throttler can hold off delayed tasks
  // that would otherwise by allowed to run sooner.
  class BASE_EXPORT Throttler {
   public:
    // Invoked when the TaskQueue's next allowed wake up time is reached and is
    // enabled, even if blocked by a fence. That wake up is defined by the last
    // value returned from GetNextAllowedWakeUp().
    // This is always called on the thread this TaskQueue is associated with.
    virtual void OnWakeUp(LazyNow* lazy_now) = 0;

    // Invoked when the TaskQueue newly gets a pending immediate task and is
    // enabled, even if blocked by a fence. Redundant calls are possible when
    // the TaskQueue already had a pending immediate task.
    // The implementation may use this to:
    // - Restrict task execution by inserting/updating a fence.
    // - Update the TaskQueue's next delayed wake up via UpdateWakeUp().
    //   This allows the Throttler to perform additional operations later from
    //   OnWakeUp().
    // This is always called on the thread this TaskQueue is associated with.
    virtual void OnHasImmediateTask() = 0;

    // Invoked when the TaskQueue is enabled and wants to know when to schedule
    // the next delayed wake-up (which happens at least every time this queue is
    // about to cause the next wake up) provided |next_desired_wake_up|, the
    // wake-up for the next pending delayed task in this queue (pending delayed
    // tasks that are ripe may be ignored), or nullopt if there's no pending
    // delayed task. |has_ready_task| indicates whether there are immediate
    // tasks or ripe delayed tasks. The implementation should return the next
    // allowed wake up, or nullopt if no future wake-up is necessary.
    // This is always called on the thread this TaskQueue is associated with.
    virtual std::optional<WakeUp> GetNextAllowedWakeUp(
        LazyNow* lazy_now,
        std::optional<WakeUp> next_desired_wake_up,
        bool has_ready_task) = 0;

   protected:
    ~Throttler() = default;
  };

  // Wrapper around a `TaskQueue`, exposed by `SequenceManager` when creating a
  // task queue. The handle owns the underlying queue and exposes it through a
  // unique_ptr-like interface, and it's responsible for managing the queue's
  // lifetime, ensuring the queue is properly unregistered with the queue's
  // `SequenceManager` when the handle is destroyed.
  class BASE_EXPORT Handle {
   public:
    Handle();

    Handle(Handle&&);
    Handle& operator=(Handle&&);

    ~Handle();

    void reset();
    TaskQueue* get() const;
    TaskQueue* operator->() const;

    explicit operator bool() const { return !!task_queue_; }

   private:
    friend class internal::SequenceManagerImpl;
    explicit Handle(std::unique_ptr<internal::TaskQueueImpl> task_queue);

    std::unique_ptr<internal::TaskQueueImpl> task_queue_;
    WeakPtr<internal::SequenceManagerImpl> sequence_manager_;
  };

  // Queues with higher priority (smaller number) are selected to run before
  // queues of lower priority. Note that there is no starvation protection,
  // i.e., a constant stream of high priority work can mean that tasks in lower
  // priority queues won't get to run.
  using QueuePriority = uint8_t;

  // By default there is only a single priority. Sequences making use of
  // priorities should parameterize the `SequenceManager` with the appropriate
  // `SequenceManager::PrioritySettings`.
  enum class DefaultQueuePriority : QueuePriority {
    kNormalPriority = 0,

    // Must be the last entry.
    kQueuePriorityCount = 1,
  };

  // Options for constructing a TaskQueue.
  struct Spec {
    explicit Spec(QueueName name) : name(name) {}

    Spec SetShouldMonitorQuiescence(bool should_monitor) {
      should_monitor_quiescence = should_monitor;
      return *this;
    }

    Spec SetShouldNotifyObservers(bool run_observers) {
      should_notify_observers = run_observers;
      return *this;
    }

    // Delayed fences require Now() to be sampled when posting immediate tasks
    // which is not free.
    Spec SetDelayedFencesAllowed(bool allow_delayed_fences) {
      delayed_fence_allowed = allow_delayed_fences;
      return *this;
    }

    Spec SetNonWaking(bool non_waking_in) {
      non_waking = non_waking_in;
      return *this;
    }

    QueueName name;
    bool should_monitor_quiescence = false;
    bool should_notify_observers = true;
    bool delayed_fence_allowed = false;
    bool non_waking = false;
  };

  // Information about task execution.
  //
  // Wall-time related methods (start_time, end_time, wall_duration) can be
  // called only when |has_wall_time()| is true.
  //
  // start_* should be called after RecordTaskStart.
  // end_* and *_duration should be called after RecordTaskEnd.
  class BASE_EXPORT TaskTiming {
   public:
    enum class State { NotStarted, Running, Finished };
    enum class TimeRecordingPolicy { DoRecord, DoNotRecord };

    explicit TaskTiming(bool has_wall_time);

    bool has_wall_time() const { return has_wall_time_; }
    bool has_thread_time() const { return has_thread_time_; }

    base::TimeTicks start_time() const {
      DCHECK(has_wall_time());
      return start_time_;
    }
    base::TimeTicks end_time() const {
      DCHECK(has_wall_time());
      return end_time_;
    }
    base::TimeDelta wall_duration() const {
      DCHECK(has_wall_time());
      return end_time_ - start_time_;
    }

    State state() const { return state_; }

    void RecordTaskStart(LazyNow* now);
    void RecordTaskEnd(LazyNow* now);

    // Protected for tests.
   protected:
    State state_ = State::NotStarted;

    bool has_wall_time_;
    bool has_thread_time_;

    base::TimeTicks start_time_;
    base::TimeTicks end_time_;
  };

  // An interface that lets the owner vote on whether or not the associated
  // TaskQueue should be enabled.
  class BASE_EXPORT QueueEnabledVoter {
   public:
    ~QueueEnabledVoter();

    QueueEnabledVoter(const QueueEnabledVoter&) = delete;
    const QueueEnabledVoter& operator=(const QueueEnabledVoter&) = delete;

    // Votes to enable or disable the associated TaskQueue. The TaskQueue will
    // only be enabled if all the voters agree it should be enabled, or if there
    // are no voters. Voters don't keep the queue alive.
    // NOTE this must be called on the thread the associated TaskQueue was
    // created on.
    void SetVoteToEnable(bool enabled);

    bool IsVotingToEnable() const { return enabled_; }

   private:
    friend class internal::TaskQueueImpl;
    explicit QueueEnabledVoter(WeakPtr<internal::TaskQueueImpl> task_queue);

    WeakPtr<internal::TaskQueueImpl> task_queue_;
    bool enabled_ = true;
  };

  TaskQueue(const TaskQueue&) = delete;
  TaskQueue& operator=(const TaskQueue&) = delete;
  virtual ~TaskQueue() = default;

  // Returns an interface that allows the caller to vote on whether or not this
  // TaskQueue is enabled. The TaskQueue will be enabled if there are no voters
  // or if all agree it should be enabled.
  // NOTE this must be called on the thread this TaskQueue was created by.
  virtual std::unique_ptr<QueueEnabledVoter> CreateQueueEnabledVoter() = 0;

  // NOTE this must be called on the thread this TaskQueue was created by.
  virtual bool IsQueueEnabled() const = 0;

  // Returns true if the queue is completely empty.
  virtual bool IsEmpty() const = 0;

  // Returns the number of pending tasks in the queue.
  virtual size_t GetNumberOfPendingTasks() const = 0;

  // Returns true iff this queue has immediate tasks or delayed tasks that are
  // ripe for execution. Ignores the queue's enabled state and fences.
  // NOTE: this must be called on the thread this TaskQueue was created by.
  // TODO(etiennep): Rename to HasReadyTask() and add LazyNow parameter.
  virtual bool HasTaskToRunImmediatelyOrReadyDelayedTask() const = 0;

  // Returns a wake-up for the next pending delayed task (pending delayed tasks
  // that are ripe may be ignored), ignoring Throttler is any. If there are no
  // such tasks (immediate tasks don't count) or the queue is disabled it
  // returns nullopt.
  // NOTE: this must be called on the thread this TaskQueue was created by.
  virtual std::optional<WakeUp> GetNextDesiredWakeUp() = 0;

  // Can be called on any thread.
  virtual const char* GetName() const = 0;

  // Set the priority of the queue to |priority|. NOTE this must be called on
  // the thread this TaskQueue was created by.
  virtual void SetQueuePriority(QueuePriority priority) = 0;

  // Same as above but with an enum value as the priority.
  template <typename T>
    requires(std::is_enum_v<T>)
  void SetQueuePriority(T priority) {
    static_assert(std::is_same_v<std::underlying_type_t<T>, QueuePriority>,
                  "Enumerated priorites must have the same underlying type as "
                  "TaskQueue::QueuePriority");
    SetQueuePriority(static_cast<QueuePriority>(priority));
  }

  // Returns the current queue priority.
  virtual QueuePriority GetQueuePriority() const = 0;

  // These functions can only be called on the same thread that the task queue
  // manager executes its tasks on.
  virtual void AddTaskObserver(TaskObserver* task_observer) = 0;
  virtual void RemoveTaskObserver(TaskObserver* task_observer) = 0;

  enum class InsertFencePosition {
    kNow,  // Tasks posted on the queue up till this point further may run.
           // All further tasks are blocked.
    kBeginningOfTime,  // No tasks posted on this queue may run.
  };

  // Inserts a barrier into the task queue which prevents tasks with an enqueue
  // order greater than the fence from running until either the fence has been
  // removed or a subsequent fence has unblocked some tasks within the queue.
  // Note: delayed tasks get their enqueue order set once their delay has
  // expired, and non-delayed tasks get their enqueue order set when posted.
  //
  // Fences come in three flavours:
  // - Regular (InsertFence(NOW)) - all tasks posted after this moment
  //   are blocked.
  // - Fully blocking (InsertFence(kBeginningOfTime)) - all tasks including
  //   already posted are blocked.
  // - Delayed (InsertFenceAt(timestamp)) - blocks all tasks posted after given
  //   point in time (must be in the future).
  //
  // Only one fence can be scheduled at a time. Inserting a new fence
  // will automatically remove the previous one, regardless of fence type.
  virtual void InsertFence(InsertFencePosition position) = 0;

  // Delayed fences are only allowed for queues created with
  // SetDelayedFencesAllowed(true) because this feature implies sampling Now()
  // (which isn't free) for every PostTask, even those with zero delay.
  virtual void InsertFenceAt(TimeTicks time) = 0;

  // Removes any previously added fence and unblocks execution of any tasks
  // blocked by it.
  virtual void RemoveFence() = 0;

  // Returns true if the queue has a fence but it isn't necessarily blocking
  // execution of tasks (it may be the case if tasks enqueue order hasn't
  // reached the number set for a fence).
  virtual bool HasActiveFence() = 0;

  // Returns true if the queue has a fence which is blocking execution of tasks.
  virtual bool BlockedByFence() const = 0;

  // Associates |throttler| to this queue. Only one throttler can be associated
  // with this queue. |throttler| must outlive this TaskQueue, or remain valid
  // until ResetThrottler().
  virtual void SetThrottler(Throttler* throttler) = 0;
  // Disassociates the current throttler from this queue, if any.
  virtual void ResetThrottler() = 0;

  // Updates the task queue's next wake up time in its time domain, taking into
  // account the desired run time of queued tasks and policies enforced by the
  // throttler if any.
  virtual void UpdateWakeUp(LazyNow* lazy_now) = 0;

  // Controls whether or not the queue will emit traces events when tasks are
  // posted to it while disabled. This only applies for the current or next
  // period during which the queue is disabled. When the queue is re-enabled
  // this will revert back to the default value of false.
  virtual void SetShouldReportPostedTasksWhenDisabled(bool should_report) = 0;

  // Create a task runner for this TaskQueue which will annotate all
  // posted tasks with the given task type.
  // Must be called on the thread this task queue is associated with.
  //
  // NOTE: Task runners don't keep the TaskQueue alive, so task queues can be
  // deleted with valid task runners. Posting a task in that case will fail.
  virtual scoped_refptr<SingleThreadTaskRunner> CreateTaskRunner(
      TaskType task_type) const = 0;

  // Default task runner which doesn't annotate tasks with a task type.
  virtual const scoped_refptr<SingleThreadTaskRunner>& task_runner() const = 0;

  using OnTaskStartedHandler =
      RepeatingCallback<void(const Task&, const TaskQueue::TaskTiming&)>;
  using OnTaskCompletedHandler =
      RepeatingCallback<void(const Task&, TaskQueue::TaskTiming*, LazyNow*)>;
  using OnTaskPostedHandler = RepeatingCallback<void(const Task&)>;
  using TaskExecutionTraceLogger =
      RepeatingCallback<void(perfetto::EventContext&, const Task&)>;

  // Sets a handler to subscribe for notifications about started and completed
  // tasks.
  virtual void SetOnTaskStartedHandler(OnTaskStartedHandler handler) = 0;

  // |task_timing| may be passed in Running state and may not have the end time,
  // so that the handler can run an additional task that is counted as a part of
  // the main task.
  // The handler can call TaskTiming::RecordTaskEnd, which is optional, to
  // finalize the task, and use the resulting timing.
  virtual void SetOnTaskCompletedHandler(OnTaskCompletedHandler handler) = 0;

  // RAII handle associated with an OnTaskPostedHandler. Unregisters the handler
  // upon destruction.
  class OnTaskPostedCallbackHandle {
   public:
    OnTaskPostedCallbackHandle(const OnTaskPostedCallbackHandle&) = delete;
    OnTaskPostedCallbackHandle& operator=(const OnTaskPostedCallbackHandle&) =
        delete;
    virtual ~OnTaskPostedCallbackHandle() = default;

   protected:
    OnTaskPostedCallbackHandle() = default;
  };

  // Add a callback for adding custom functionality for processing posted task.
  // Callback will be dispatched while holding a scheduler lock. As a result,
  // callback should not call scheduler APIs directly, as this can lead to
  // deadlocks. For example, PostTask should not be called directly and
  // ScopedDeferTaskPosting::PostOrDefer should be used instead. `handler` must
  // not be a null callback. Must be called on the thread this task queue is
  // associated with, and the handle returned must be destroyed on the same
  // thread.
  [[nodiscard]] virtual std::unique_ptr<OnTaskPostedCallbackHandle>
  AddOnTaskPostedHandler(OnTaskPostedHandler handler) = 0;

  // Set a callback to fill trace event arguments associated with the task
  // execution.
  virtual void SetTaskExecutionTraceLogger(TaskExecutionTraceLogger logger) = 0;

  // Removes immediate cancelled tasks from the queue. Call this method when the
  // queue is expected to contain a significant number of canceled tasks
  // (>1000), making it worthwhile to traverse it to reclaim memory. Should only
  // be called in a context where it's safe to call the destructor of tasks.
  virtual void RemoveCancelledTasks() = 0;

 protected:
  TaskQueue() = default;
};

}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_TASK_QUEUE_H_
