// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_TASK_ATTRIBUTION_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_TASK_ATTRIBUTION_TRACKER_H_

#include <optional>
#include <utility>

#include "base/functional/function_ref.h"
#include "base/memory/stack_allocated.h"
#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/renderer/platform/bindings/v8_per_isolate_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {
class SchedulerTaskContext;
class ScriptState;
class ScriptWrappableTaskState;
class SoftNavigationContext;
}  // namespace blink

namespace v8 {
class Isolate;
}  // namespace v8

namespace blink::scheduler {

class TaskAttributionInfo;
class TaskAttributionTrackerImpl;

// This public interface enables platform/ and core/ callers to create a task
// scope on the one hand, and check on the ID of the currently running task as
// well as its ancestry on the other.
class PLATFORM_EXPORT TaskAttributionTracker {
 public:
  enum class TaskScopeType {
    kCallback,
    kScheduledAction,
    kScriptExecution,
    kPostMessage,
    kPopState,
    kSchedulerPostTask,
    kRequestIdleCallback,
    kXMLHttpRequest,
    kSoftNavigation,
  };

  // `TaskScope` stores state for the current task, which is propagated to tasks
  // and promise reactions created with in the scope. `TaskScope`s are meant
  // to be only used for JavaScript execution, and "task" here approximately
  // means "the current JavaScript execution, excluding microtasks", which
  // roughly aligns with a top-level JS callback.
  class TaskScope final {
    STACK_ALLOCATED();

   public:
    ~TaskScope() {
      if (task_tracker_) {
        task_tracker_->OnTaskScopeDestroyed(*this);
      }
    }

    TaskScope(TaskScope&& other)
        : task_tracker_(std::exchange(other.task_tracker_, nullptr)),
          script_state_(other.script_state_),
          previous_task_state_(other.previous_task_state_) {}

    TaskScope& operator=(TaskScope&& other) {
      task_tracker_ = std::exchange(other.task_tracker_, nullptr);
      script_state_ = other.script_state_;
      previous_task_state_ = other.previous_task_state_;
      return *this;
    }

   private:
    // Only `TaskAttributionTrackerImpl` can create `TaskScope`s.
    friend class TaskAttributionTrackerImpl;

    TaskScope(TaskAttributionTracker* tracker,
              ScriptState* script_state,
              ScriptWrappableTaskState* previous_task_state)
        : task_tracker_(tracker),
          script_state_(script_state),
          previous_task_state_(previous_task_state) {}

    // `task_tracker_` is tied to the lifetime of the isolate, which will
    // outlive the current task.
    TaskAttributionTracker* task_tracker_;

    // The rest are on the Oilpan heap, so these are stored as raw pointers
    // since the class is stack allocated.
    ScriptState* script_state_;
    ScriptWrappableTaskState* previous_task_state_;
  };

  class Observer : public GarbageCollectedMixin {
   public:
    virtual void OnCreateTaskScope(TaskAttributionInfo&) = 0;
  };

  class PLATFORM_EXPORT ObserverScope {
    STACK_ALLOCATED();

   public:
    ObserverScope(ObserverScope&& other)
        : task_tracker_(std::exchange(other.task_tracker_, nullptr)),
          previous_observer_(std::exchange(other.previous_observer_, nullptr)) {
    }

    ObserverScope& operator=(ObserverScope&& other) {
      task_tracker_ = std::exchange(other.task_tracker_, nullptr);
      previous_observer_ = std::exchange(other.previous_observer_, nullptr);
      return *this;
    }

    ~ObserverScope() {
      if (task_tracker_) {
        task_tracker_->OnObserverScopeDestroyed(*this);
      }
    }

   private:
    friend class TaskAttributionTrackerImpl;

    ObserverScope(TaskAttributionTracker* tracker,
                  Observer* observer,
                  Observer* previous_observer)
        : task_tracker_(tracker), previous_observer_(previous_observer) {}

    Observer* PreviousObserver() const { return previous_observer_; }

    TaskAttributionTracker* task_tracker_;
    Observer* previous_observer_;
  };

  static TaskAttributionTracker* From(v8::Isolate* isolate) {
    return V8PerIsolateData::From(isolate)->GetTaskAttributionTracker();
  }

  virtual ~TaskAttributionTracker() = default;

  // Creates a new `TaskScope` to propagate `task_state` to descendant tasks and
  // continuations.
  virtual TaskScope CreateTaskScope(ScriptState*,
                                    TaskAttributionInfo* task_state,
                                    TaskScopeType type) = 0;

  // Create a new `TaskScope` to propagate the given `SoftNavigationContext`,
  // initiating propagation for the context.
  virtual TaskScope CreateTaskScope(ScriptState*, SoftNavigationContext*) = 0;

  // Creates a new `TaskScope` with web scheduling context. `task_state` will be
  // propagated to descendant tasks and continuations; `continuation_context`
  // will only be propagated to continuations.
  virtual TaskScope CreateTaskScope(
      ScriptState*,
      TaskAttributionInfo* task_state,
      TaskScopeType type,
      SchedulerTaskContext* continuation_context) = 0;

  // Conditionally create a `TaskScope` for a generic v8 callback. A `TaskScope`
  // is always created if `task_state` is non-null, and one is additionally
  // created if there isn't an active `TaskScope`.
  virtual std::optional<TaskScope> MaybeCreateTaskScopeForCallback(
      ScriptState*,
      TaskAttributionInfo* task_state) = 0;

  // Get the `TaskAttributionInfo` for the currently running task.
  virtual TaskAttributionInfo* RunningTask() const = 0;

  // Registers an observer to be notified when a `TaskScope` has been created.
  // Multiple `Observer`s can be registered, but only the innermost one will
  // receive callbacks.
  virtual ObserverScope RegisterObserver(Observer* observer) = 0;

  // Setter and getter for a pointer to a pending same-document navigation task,
  // to ensure the task's lifetime.
  virtual void AddSameDocumentNavigationTask(TaskAttributionInfo* task) = 0;
  virtual void ResetSameDocumentNavigationTasks() = 0;
  virtual TaskAttributionInfo* CommitSameDocumentNavigation(
      TaskAttributionId) = 0;

 protected:
  virtual void OnTaskScopeDestroyed(const TaskScope&) = 0;
  virtual void OnObserverScopeDestroyed(const ObserverScope&) = 0;
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_TASK_ATTRIBUTION_TRACKER_H_
