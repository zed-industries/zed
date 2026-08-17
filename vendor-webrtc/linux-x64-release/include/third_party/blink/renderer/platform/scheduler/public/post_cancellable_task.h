// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_POST_CANCELLABLE_TASK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_POST_CANCELLABLE_TASK_H_

#include "base/functional/callback.h"
#include "base/location.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/sequenced_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// TaskHandle is associated to a task posted by PostCancellableTask() or
// PostCancellableDelayedTask() and cancels the associated task on
// TaskHandle::cancel() call or on TaskHandle destruction.
class PLATFORM_EXPORT TaskHandle {
  DISALLOW_NEW();

 public:
  class Runner;

  TaskHandle();
  ~TaskHandle();

  TaskHandle(TaskHandle&&);
  TaskHandle& operator=(TaskHandle&&);

  // Returns true if the task will run later. Returns false if the task is
  // cancelled or the task is run already.
  // This function is not thread safe. Call this on the thread that has posted
  // the task.
  bool IsActive() const;

  // Cancels the task invocation. Do nothing if the task is cancelled or run
  // already.
  // This function is not thread safe. Call this on the thread that has posted
  // the task.
  void Cancel();

 private:
  TaskHandle(const TaskHandle&) = delete;
  TaskHandle& operator=(const TaskHandle&) = delete;

  friend PLATFORM_EXPORT TaskHandle
  PostCancellableTask(base::SequencedTaskRunner&,
                      const base::Location&,
                      base::OnceClosure);
  friend PLATFORM_EXPORT TaskHandle
  PostDelayedCancellableTask(base::SequencedTaskRunner&,
                             const base::Location&,
                             base::OnceClosure,
                             base::TimeDelta delay);
  friend PLATFORM_EXPORT TaskHandle
  PostNonNestableCancellableTask(base::SequencedTaskRunner&,
                                 const base::Location&,
                                 base::OnceClosure);
  friend PLATFORM_EXPORT TaskHandle
  PostNonNestableDelayedCancellableTask(base::SequencedTaskRunner&,
                                        const base::Location&,
                                        base::OnceClosure,
                                        base::TimeDelta delay);

  explicit TaskHandle(scoped_refptr<Runner>);
  scoped_refptr<Runner> runner_;
};

// For same-thread cancellable task posting. Returns a TaskHandle object for
// cancellation.
[[nodiscard]] PLATFORM_EXPORT TaskHandle
PostCancellableTask(base::SequencedTaskRunner&,
                    const base::Location&,
                    base::OnceClosure);
[[nodiscard]] PLATFORM_EXPORT TaskHandle
PostDelayedCancellableTask(base::SequencedTaskRunner&,
                           const base::Location&,
                           base::OnceClosure,
                           base::TimeDelta delay);
[[nodiscard]] PLATFORM_EXPORT TaskHandle
PostNonNestableCancellableTask(base::SequencedTaskRunner&,
                               const base::Location&,
                               base::OnceClosure);
[[nodiscard]] PLATFORM_EXPORT TaskHandle
PostNonNestableDelayedCancellableTask(base::SequencedTaskRunner&,
                                      const base::Location&,
                                      base::OnceClosure,
                                      base::TimeDelta delay);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_POST_CANCELLABLE_TASK_H_
