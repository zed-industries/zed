// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_SEQUENCE_MANAGER_SEQUENCE_MANAGER_H_
#define BASE_TASK_SEQUENCE_MANAGER_SEQUENCE_MANAGER_H_

#include <memory>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "base/message_loop/message_pump_type.h"
#include "base/task/sequence_manager/task_queue_impl.h"
#include "base/task/sequence_manager/task_time_observer.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/default_tick_clock.h"

namespace base {

class MessagePump;
class TaskObserver;

namespace sequence_manager {
class TimeDomain;

// SequenceManager manages TaskQueues which have different properties
// (e.g. priority, common task type) multiplexing all posted tasks into
// a single backing sequence (currently bound to a single thread, which is
// refererred as *main thread* in the comments below). SequenceManager
// implementation can be used in a various ways to apply scheduling logic.
class BASE_EXPORT SequenceManager {
 public:
  class Observer {
   public:
    virtual ~Observer() = default;
    // Called back on the main thread.
    virtual void OnBeginNestedRunLoop() = 0;
    virtual void OnExitNestedRunLoop() = 0;
  };

  class BASE_EXPORT PrioritySettings {
   public:
    // This limit is based on an implementation detail of `TaskQueueSelector`'s
    // `ActivePriorityTracker`, which can be refactored if more priorities are
    // needed.
    static constexpr size_t kMaxPriorities = sizeof(size_t) * 8 - 1;

    static PrioritySettings CreateDefault();

    template <typename T>
      requires(std::is_enum_v<T>)
    PrioritySettings(T priority_count, T default_priority)
        : PrioritySettings(
              static_cast<TaskQueue::QueuePriority>(priority_count),
              static_cast<TaskQueue::QueuePriority>(default_priority)) {
      static_assert(
          std::is_same_v<std::underlying_type_t<T>, TaskQueue::QueuePriority>,
          "Enumerated priorites must have the same underlying type as "
          "TaskQueue::QueuePriority");
    }

    PrioritySettings(TaskQueue::QueuePriority priority_count,
                     TaskQueue::QueuePriority default_priority);

    ~PrioritySettings();

    PrioritySettings(PrioritySettings&&) noexcept;
    PrioritySettings& operator=(PrioritySettings&&);

    TaskQueue::QueuePriority priority_count() const { return priority_count_; }

    TaskQueue::QueuePriority default_priority() const {
      return default_priority_;
    }

#if BUILDFLAG(ENABLE_BASE_TRACING)
    void SetProtoPriorityConverter(
        perfetto::protos::pbzero::SequenceManagerTask::Priority (
            *proto_priority_converter)(TaskQueue::QueuePriority)) {
      proto_priority_converter_ = proto_priority_converter;
    }

    perfetto::protos::pbzero::SequenceManagerTask::Priority TaskPriorityToProto(
        TaskQueue::QueuePriority priority) const;
#endif

   private:
    TaskQueue::QueuePriority priority_count_;
    TaskQueue::QueuePriority default_priority_;

#if BUILDFLAG(ENABLE_BASE_TRACING)
    perfetto::protos::pbzero::SequenceManagerTask::Priority (
        *proto_priority_converter_)(TaskQueue::QueuePriority) = nullptr;
#endif

#if DCHECK_IS_ON()
   public:
    PrioritySettings(
        TaskQueue::QueuePriority priority_count,
        TaskQueue::QueuePriority default_priority,
        std::vector<TimeDelta> per_priority_cross_thread_task_delay,
        std::vector<TimeDelta> per_priority_same_thread_task_delay);

    const std::vector<TimeDelta>& per_priority_cross_thread_task_delay() const
        LIFETIME_BOUND {
      return per_priority_cross_thread_task_delay_;
    }

    const std::vector<TimeDelta>& per_priority_same_thread_task_delay() const
        LIFETIME_BOUND {
      return per_priority_same_thread_task_delay_;
    }

   private:
    // Scheduler policy induced raciness is an area of concern. This lets us
    // apply an extra delay per priority for cross thread posting.
    std::vector<TimeDelta> per_priority_cross_thread_task_delay_;

    // Like the above but for same thread posting.
    std::vector<TimeDelta> per_priority_same_thread_task_delay_;
#endif
  };

  // Settings defining the desired SequenceManager behaviour.
  struct BASE_EXPORT Settings {
    class Builder;

    Settings();
    Settings(const Settings&) = delete;
    Settings& operator=(const Settings&) = delete;
    // In the future MessagePump (which is move-only) will also be a setting,
    // so we are making Settings move-only in preparation.
    Settings(Settings&& move_from) noexcept;

    ~Settings();

    MessagePumpType message_loop_type = MessagePumpType::DEFAULT;
    raw_ptr<const TickClock, DanglingUntriaged> clock =
        DefaultTickClock::GetInstance();

    // Whether or not queueing timestamp will be added to tasks.
    bool add_queue_time_to_tasks = false;

    // Whether many tasks may run between each check for native work.
    bool can_run_tasks_by_batches = false;

    PrioritySettings priority_settings = PrioritySettings::CreateDefault();

#if DCHECK_IS_ON()
    // TODO(alexclarke): Consider adding command line flags to control these.
    enum class TaskLogging {
      kNone,
      kEnabled,
      kEnabledWithBacktrace,

      // Logs high priority tasks and the lower priority tasks they skipped
      // past.  Useful for debugging test failures caused by scheduler policy
      // changes.
      kReorderedOnly,
    };
    TaskLogging task_execution_logging = TaskLogging::kNone;

    // If true PostTask will emit a debug log.
    bool log_post_task = false;

    // If true debug logs will be emitted when a delayed task becomes eligible
    // to run.
    bool log_task_delay_expiry = false;

    // If not zero this seeds a PRNG used by the task selection logic to choose
    // a random TaskQueue for a given priority rather than the TaskQueue with
    // the oldest EnqueueOrder.
    uint64_t random_task_selection_seed = 0;
#endif  // DCHECK_IS_ON()
  };

  virtual ~SequenceManager() = default;

  // Binds the SequenceManager and its TaskQueues to the current thread. Should
  // only be called once. Note that CreateSequenceManagerOnCurrentThread()
  // performs this initialization automatically.
  virtual void BindToCurrentThread() = 0;

  // Returns the task runner the current task was posted on. Returns null if no
  // task is currently running. Must be called on the bound thread.
  virtual scoped_refptr<SequencedTaskRunner> GetTaskRunnerForCurrentTask() = 0;

  // Finishes the initialization for a SequenceManager created via
  // CreateUnboundSequenceManager(). Must not be called in any other
  // circumstances. The ownership of the pump is transferred to SequenceManager.
  virtual void BindToMessagePump(std::unique_ptr<MessagePump> message_pump) = 0;

  // Must be called on the main thread.
  // Can be called only once, before creating TaskQueues.
  // Observer must outlive the SequenceManager.
  virtual void SetObserver(Observer* observer) = 0;

  // Must be called on the main thread.
  virtual void AddTaskTimeObserver(TaskTimeObserver* task_time_observer) = 0;
  virtual void RemoveTaskTimeObserver(TaskTimeObserver* task_time_observer) = 0;

  // Sets `time_domain` to be used by this scheduler and associated task queues.
  // Only one time domain can be set at a time. `time_domain` must outlive this
  // SequenceManager, even if ResetTimeDomain() is called. This has no effect on
  // previously scheduled tasks and it is recommended that `time_domain` be set
  // before posting any task to avoid inconsistencies in time. Otherwise,
  // replacing `time_domain` is very subtle and should be reserved for developer
  // only use cases (e.g. virtual time in devtools) where any flakiness caused
  // by a racy time update isn't surprising.
  virtual void SetTimeDomain(TimeDomain* time_domain) = 0;
  // Disassociates the current `time_domain` and reverts to using
  // RealTimeDomain.
  virtual void ResetTimeDomain() = 0;

  virtual const TickClock* GetTickClock() const = 0;
  virtual TimeTicks NowTicks() const = 0;

  // Returns a wake-up for the next delayed task which is not ripe for
  // execution. If there are no such tasks (immediate tasks don't count),
  // returns nullopt.
  virtual std::optional<WakeUp> GetNextDelayedWakeUp() const = 0;

  // Sets the SingleThreadTaskRunner that will be returned by
  // SingleThreadTaskRunner::GetCurrentDefault on the main thread.
  virtual void SetDefaultTaskRunner(
      scoped_refptr<SingleThreadTaskRunner> task_runner) = 0;

  // Removes all canceled delayed tasks, and considers resizing to fit all
  // internal queues.
  virtual void ReclaimMemory() = 0;

  // Returns true if no tasks were executed in TaskQueues that monitor
  // quiescence since the last call to this method.
  virtual bool GetAndClearSystemIsQuiescentBit() = 0;

  // Set the number of tasks executed in a single SequenceManager invocation.
  // Increasing this number reduces the overhead of the tasks dispatching
  // logic at the cost of a potentially worse latency. 1 by default.
  virtual void SetWorkBatchSize(int work_batch_size) = 0;

  // Enables crash keys that can be set in the scope of a task which help
  // to identify the culprit if upcoming work results in a crash.
  // Key names must be thread-specific to avoid races and corrupted crash dumps.
  virtual void EnableCrashKeys(const char* async_stack_crash_key) = 0;

  virtual TaskQueue::QueuePriority GetPriorityCount() const = 0;

  // Creates a `TaskQueue` and returns a `TaskQueue::Handle`for it. The queue is
  // owned by the handle and shut down when the handle is destroyed. Must be
  // called on the main thread.
  virtual TaskQueue::Handle CreateTaskQueue(const TaskQueue::Spec& spec) = 0;

  // Returns true iff this SequenceManager has no immediate work to do. I.e.
  // there are no pending non-delayed tasks or delayed tasks that are due to
  // run. This method ignores any pending delayed tasks that might have become
  // eligible to run since the last task was executed. This is important because
  // if it did tests would become flaky depending on the exact timing of this
  // call. This is moderately expensive.
  virtual bool IsIdleForTesting() = 0;

  // The total number of posted tasks that haven't executed yet.
  virtual size_t GetPendingTaskCountForTesting() const = 0;

  // Returns a JSON string which describes all pending tasks.
  virtual std::string DescribeAllPendingTasks() const = 0;

  // Adds an observer which reports task execution. Can only be called on the
  // same thread that `this` is running on.
  virtual void AddTaskObserver(TaskObserver* task_observer) = 0;

  // Removes an observer which reports task execution. Can only be called on the
  // same thread that `this` is running on.
  virtual void RemoveTaskObserver(TaskObserver* task_observer) = 0;
};

class BASE_EXPORT SequenceManager::Settings::Builder {
 public:
  Builder();
  ~Builder();

  // Sets the MessagePumpType which is used to create a MessagePump.
  Builder& SetMessagePumpType(MessagePumpType message_loop_type);

  // Sets the TickClock the SequenceManager uses to obtain Now.
  Builder& SetTickClock(const TickClock* clock);

  // Whether or not queueing timestamp will be added to tasks.
  Builder& SetAddQueueTimeToTasks(bool add_queue_time_to_tasks);

  // Whether many tasks may run between each check for native work.
  Builder& SetCanRunTasksByBatches(bool can_run_tasks_by_batches);

  Builder& SetPrioritySettings(PrioritySettings settings);

#if DCHECK_IS_ON()
  // Controls task execution logging.
  Builder& SetTaskLogging(TaskLogging task_execution_logging);

  // Whether or not PostTask will emit a debug log.
  Builder& SetLogPostTask(bool log_post_task);

  // Whether or not debug logs will be emitted when a delayed task becomes
  // eligible to run.
  Builder& SetLogTaskDelayExpiry(bool log_task_delay_expiry);

  // If not zero this seeds a PRNG used by the task selection logic to choose a
  // random TaskQueue for a given priority rather than the TaskQueue with the
  // oldest EnqueueOrder.
  Builder& SetRandomTaskSelectionSeed(uint64_t random_task_selection_seed);
#endif  // DCHECK_IS_ON()

  Settings Build();

 private:
  Settings settings_;
};

// Create SequenceManager using MessageLoop on the current thread.
// Implementation is located in sequence_manager_impl.cc.
// TODO(scheduler-dev): Remove after every thread has a SequenceManager.
BASE_EXPORT std::unique_ptr<SequenceManager>
CreateSequenceManagerOnCurrentThread(SequenceManager::Settings settings);

// Create a SequenceManager using the given MessagePump on the current thread.
// MessagePump instances can be created with
// MessagePump::CreateMessagePumpForType().
BASE_EXPORT std::unique_ptr<SequenceManager>
CreateSequenceManagerOnCurrentThreadWithPump(
    std::unique_ptr<MessagePump> message_pump,
    SequenceManager::Settings settings = SequenceManager::Settings());

// Create an unbound SequenceManager (typically for a future thread or because
// additional setup is required before binding). The SequenceManager can be
// initialized on the current thread and then needs to be bound and initialized
// on the target thread by calling one of the Bind*() methods.
BASE_EXPORT std::unique_ptr<SequenceManager> CreateUnboundSequenceManager(
    SequenceManager::Settings settings = SequenceManager::Settings());

}  // namespace sequence_manager
}  // namespace base

#endif  // BASE_TASK_SEQUENCE_MANAGER_SEQUENCE_MANAGER_H_
