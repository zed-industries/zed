// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_TASK_RUNNER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_TASK_RUNNER_H_

#include "base/synchronization/condition_variable.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "v8/include/v8.h"

namespace blink {

// This class manages a queue of tasks posted from any threads, and interrupts
// v8::Isolate to run the tasks.
//
// The tasks will be run on the isolate's thread after interruption or via
// posting to its task runner.
class CORE_EXPORT InspectorTaskRunner final
    : public ThreadSafeRefCounted<InspectorTaskRunner> {
  USING_FAST_MALLOC(InspectorTaskRunner);

 public:
  // Can be created on any thread.
  static scoped_refptr<InspectorTaskRunner> Create(
      scoped_refptr<base::SingleThreadTaskRunner> isolate_task_runner) {
    return base::AdoptRef(new InspectorTaskRunner(isolate_task_runner));
  }

  InspectorTaskRunner(const InspectorTaskRunner&) = delete;
  InspectorTaskRunner& operator=(const InspectorTaskRunner&) = delete;

  // Must be called on the isolate's thread.
  void InitIsolate(v8::Isolate*) LOCKS_EXCLUDED(lock_);
  // Can be disposed from any thread.
  void Dispose() LOCKS_EXCLUDED(lock_);

  // Can be called from any thread other than isolate's thread.
  // This method appends a task, and both posts to the isolate's task runner
  // and requests interrupt. Whatever comes first - executes the task.
  // Returns if the task has been appended or discarded if this runner has
  // already been disposed. Note that successfully appending a task does not
  // guarantee that it'll run, e.g. if Dispose() is called before it runs.
  using Task = CrossThreadOnceClosure;
  bool AppendTask(Task) LOCKS_EXCLUDED(lock_);

  // Can be called from any thread other than isolate's thread.
  // This method appends a task and posts to the isolate's task runner to
  // request that the next task be executed, but does not interrupt V8
  // execution.
  // Returns if the task has been appended or discarded if this runner has
  // already been disposed. Note that successfully appending a task does not
  // guarantee that it'll run, e.g. if Dispose() is called before it runs.
  bool AppendTaskDontInterrupt(Task) LOCKS_EXCLUDED(lock_);

  scoped_refptr<base::SingleThreadTaskRunner> isolate_task_runner() {
    return isolate_task_runner_;
  }

  void ProcessInterruptingTasks();
  void RequestQuitProcessingInterruptingTasks();

 private:
  friend ThreadSafeRefCounted<InspectorTaskRunner>;
  explicit InspectorTaskRunner(
      scoped_refptr<base::SingleThreadTaskRunner> isolate_task_runner);
  ~InspectorTaskRunner();

  // All these methods are run on the isolate's thread.
  Task TakeNextInterruptingTask() LOCKS_EXCLUDED(lock_);
  Task WaitForNextInterruptingTaskOrQuitRequest() LOCKS_EXCLUDED(lock_);
  void PerformSingleInterruptingTaskDontWait() LOCKS_EXCLUDED(lock_);
  static void V8InterruptCallback(v8::Isolate*, void* data);

  base::Lock lock_;
  scoped_refptr<base::SingleThreadTaskRunner> isolate_task_runner_;
  v8::Isolate* isolate_ GUARDED_BY(lock_) = nullptr;
  bool quit_requested_ = false;
  base::ConditionVariable task_queue_cv_;
  Deque<Task> interrupting_task_queue_;
  bool disposed_ GUARDED_BY(lock_) = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_TASK_RUNNER_H_
