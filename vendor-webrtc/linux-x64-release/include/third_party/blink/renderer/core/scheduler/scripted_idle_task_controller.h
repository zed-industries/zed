// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_SCRIPTED_IDLE_TASK_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_SCRIPTED_IDLE_TASK_CONTROLLER_H_

#include "base/feature_list.h"
#include "base/memory/ref_counted.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/delayed_task_handle.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/core/probe/async_task_context.h"
#include "third_party/blink/renderer/core/scheduler/idle_deadline.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

CORE_EXPORT BASE_DECLARE_FEATURE(kRemoveCancelledScriptedIdleTasks);

class IdleRequestOptions;
class ScriptedIdleTaskController;
class ThreadScheduler;

// Terminology used in this file:
//
// - `IdleTask`: An externally provided task to run on idle time or after a
//   deadline
// - "Scheduler idle task": A task posted to the scheduler which runs on idle
//   time. May run an `IdleTask` when scheduled.
// - "Scheduler timeout task": A task posted to the scheduler which runs after a
//   delay. May run an `IdleTask` when scheduled.

// A task to run on idle time or after a deadline. Subclasses must define what
// to do when the task runs in `invoke`.
class CORE_EXPORT IdleTask : public GarbageCollected<IdleTask>,
                             public NameClient {
 public:
  virtual void Trace(Visitor* visitor) const {}
  const char* NameInHeapSnapshot() const override { return "IdleTask"; }
  ~IdleTask() override;
  virtual void invoke(IdleDeadline*) = 0;
  probe::AsyncTaskContext* async_task_context() { return &async_task_context_; }

 private:
  friend class ScriptedIdleTaskController;
  probe::AsyncTaskContext async_task_context_;
  // Handle to the associated "scheduler timeout task".
  base::DelayedTaskHandle delayed_task_handle_;
  bool has_scheduler_idle_task_ = false;
};

// `ScriptedIdleTaskController` manages scheduling and running `IdleTask`s. This
// provides some higher level functionality on top of the thread scheduler's
// idle tasks, e.g. timeouts and providing an `IdleDeadline` to callbacks, which
// is used both by the requestIdleCallback API and internally in blink.
class CORE_EXPORT ScriptedIdleTaskController
    : public GarbageCollected<ScriptedIdleTaskController>,
      public ExecutionContextLifecycleStateObserver,
      public Supplement<ExecutionContext>,
      public NameClient {
  USING_PRE_FINALIZER(ScriptedIdleTaskController, Dispose);

 public:
  using RefCountedCounter = scoped_refptr<base::RefCountedData<size_t>>;

  // A move-only type which decrements a ref-counted counter on deletion or
  // on DecrementNow().
  class DecrementOnDelete {
   public:
    explicit DecrementOnDelete(RefCountedCounter counter);
    ~DecrementOnDelete();

    DecrementOnDelete(DecrementOnDelete&&);
    DecrementOnDelete& operator=(DecrementOnDelete&&);
    DecrementOnDelete(const DecrementOnDelete&) = delete;
    DecrementOnDelete& operator=(const DecrementOnDelete&) = delete;

    void DecrementNow();

   private:
    RefCountedCounter counter_;
  };

  static const char kSupplementName[];

  static ScriptedIdleTaskController& From(ExecutionContext& context);

  explicit ScriptedIdleTaskController(ExecutionContext*);
  ~ScriptedIdleTaskController() override;

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override {
    return "ScriptedIdleTaskController";
  }
  void Dispose();

  using CallbackId = int;

  int RegisterCallback(IdleTask*, const IdleRequestOptions*);
  void CancelCallback(CallbackId);

  // Returns true iff there is a registered callback with `id`.
  bool HasCallback(CallbackId id) const;

  // ExecutionContextLifecycleStateObserver interface.
  void ContextDestroyed() override;
  void ContextLifecycleStateChanged(mojom::FrameLifecycleState) override;

  // Invoked when IsCancelled() is called on a "scheduler idle task" from this.
  // TODO(crbug.com/394266102): Remove after the bug is understood and fixed.
  void OnCheckSchedulerIdleTaskIsCancelled();

 private:
  using IdleTaskMap = HeapHashMap<CallbackId, Member<IdleTask>>;

  // Posts a "scheduler idle task" and a "scheduler timeout task" to run the
  // `IdleTask` identified by `id`.
  void PostSchedulerIdleAndTimeoutTasks(CallbackId id, uint32_t timeout_millis);

  // Posts a "scheduler idle task" to run the `IdleTask` identified by `it`.
  void PostSchedulerIdleTask(IdleTaskMap::iterator it);

  void SchedulerIdleTask(CallbackId id,
                         DecrementOnDelete decrement_on_delete,
                         base::TimeTicks deadline);

  void SchedulerTimeoutTask(CallbackId id);

  void RunIdleTask(CallbackId id,
                   base::TimeTicks deadline,
                   IdleDeadline::CallbackType callback_type);

  // Removes an/all `IdleTask`(s) from `idle_tasks_`.
  void RemoveIdleTask(CallbackId id);
  void RemoveAllIdleTasks();

  // Removes cancelled "scheduler idle tasks" from the scheduler queue if more
  // than 1000 are accumulated. This should be invoked whenever the delta
  // between the number of `IdleTask`s and "scheduler idle tasks" increases.
  void CleanupSchedulerIdleTasks();

  void ContextPaused();
  void ContextUnpaused();

  int NextCallbackId();

  static bool IsValidCallbackId(int id) {
    using Traits = HashTraits<CallbackId>;
    return !WTF::IsHashTraitsEmptyOrDeletedValue<Traits, CallbackId>(id);
  }

  // Not owned.
  ThreadScheduler* scheduler_;

  // Pending `IdleTask`s.
  IdleTaskMap idle_tasks_;

  // `IdleTask`s for which `SchedulerIdleTask` ran while paused. They'll be
  // rescheduled when unpaused.
  Vector<CallbackId> idle_tasks_to_reschedule_;

  // Id that will be assigned to the `IdleTask` registered with this.
  CallbackId next_callback_id_ = 0;

  // Whether the execution context is paused.
  bool paused_ = false;

  // Number of outstanding "scheduler idle tasks".
  scoped_refptr<base::RefCountedData<size_t>> num_scheduler_idle_tasks_ =
      base::MakeRefCounted<base::RefCountedData<size_t>>(0);

  // Number of calls to `IsCancelled()` on "scheduler idle tasks" from this.
  // TODO(crbug.com/394266102): Remove after the bug is understood and fixed.
  uint64_t num_is_cancelled_checks_ = 0;

  // Whether `next_callback_id_` wrapped around.
  // TODO(crbug.com/394266102): Remove after the bug is understood and fixed.
  bool next_callback_id_wrapped_around_ = false;

 public:
  // Type of SchedulerIdleTask(), used to define callback cancellation traits in
  // the implementation file.
  using SchedulerIdleTaskDeclType =
      decltype(&ScriptedIdleTaskController::SchedulerIdleTask);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_SCRIPTED_IDLE_TASK_CONTROLLER_H_
