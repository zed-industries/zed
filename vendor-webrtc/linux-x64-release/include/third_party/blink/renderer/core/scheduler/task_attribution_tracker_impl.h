// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_TASK_ATTRIBUTION_TRACKER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_TASK_ATTRIBUTION_TRACKER_IMPL_H_

#include <optional>

#include "base/containers/contains.h"
#include "base/memory/raw_ptr.h"
#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/scheduler/public/task_attribution_tracker.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {
class SchedulerTaskContext;
class SoftNavigationContext;
}  // namespace blink

namespace v8 {
class Isolate;
}  // namespace v8

namespace blink::scheduler {
class TaskAttributionInfo;

// This class is used to keep track of tasks posted on the main thread and their
// ancestry. It assigns an incerementing ID per task, and gets notified when a
// task is posted, started or ended, and using that, it keeps track of which
// task is the parent of the current task, and stores that info for later. It
// then enables callers to determine if a certain task ID is an ancestor of the
// current task.
class CORE_EXPORT TaskAttributionTrackerImpl : public TaskAttributionTracker {
 public:
  static std::unique_ptr<TaskAttributionTracker> Create(v8::Isolate*);

  TaskAttributionInfo* RunningTask() const override;

  TaskScope CreateTaskScope(ScriptState* script_state,
                            TaskAttributionInfo* task_state,
                            TaskScopeType type) override;

  TaskScope CreateTaskScope(ScriptState* script_state,
                            SoftNavigationContext*) override;

  TaskScope CreateTaskScope(
      ScriptState* script_state,
      TaskAttributionInfo* task_state,
      TaskScopeType type,
      SchedulerTaskContext* continuation_context) override;

  std::optional<TaskScope> MaybeCreateTaskScopeForCallback(
      ScriptState*,
      TaskAttributionInfo* task_state) override;

  ObserverScope RegisterObserver(Observer* observer) override;
  void AddSameDocumentNavigationTask(TaskAttributionInfo* task) override;
  void ResetSameDocumentNavigationTasks() override;
  TaskAttributionInfo* CommitSameDocumentNavigation(TaskAttributionId) override;

 private:
  explicit TaskAttributionTrackerImpl(v8::Isolate*);

  void OnTaskScopeDestroyed(const TaskScope&) override;
  void OnObserverScopeDestroyed(const ObserverScope&) override;

  TaskAttributionId next_task_id_;
  Persistent<Observer> observer_ = nullptr;

  // A queue of TaskAttributionInfo objects representing tasks that initiated a
  // same-document navigation that was sent to the browser side. They are kept
  // here to ensure the relevant object remains alive (and hence properly
  // tracked through task attribution).
  WTF::Deque<Persistent<TaskAttributionInfo>> same_document_navigation_tasks_;

  // The lifetime of this class is tied to the `isolate_`.
  v8::Isolate* isolate_;
};

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCHEDULER_TASK_ATTRIBUTION_TRACKER_IMPL_H_
