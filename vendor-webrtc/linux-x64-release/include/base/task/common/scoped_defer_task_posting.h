// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_COMMON_SCOPED_DEFER_TASK_POSTING_H_
#define BASE_TASK_COMMON_SCOPED_DEFER_TASK_POSTING_H_

#include <vector>

#include "base/base_export.h"
#include "base/location.h"
#include "base/task/sequenced_task_runner.h"
#include "base/time/time.h"

namespace base {

// Tracing wants to post tasks from within a trace event within PostTask, but
// this can lead to a deadlock. Create a scope to ensure that we are posting
// the tasks in question outside of the scope of the lock.
// NOTE: This scope affects only the thread it is created on. All other threads
// still can post tasks.
//
// TODO(altimin): It should be possible to get rid of this scope, but this
// requires refactoring TimeDomain to ensure that TimeDomain never changes and
// we can read current time without grabbing a lock.
class BASE_EXPORT [[maybe_unused, nodiscard]] ScopedDeferTaskPosting {
 public:
  static void PostOrDefer(scoped_refptr<SequencedTaskRunner> task_runner,
                          const Location& from_here,
                          OnceClosure task,
                          base::TimeDelta delay);

  static bool IsPresent();

  ScopedDeferTaskPosting();

  ScopedDeferTaskPosting(const ScopedDeferTaskPosting&) = delete;
  ScopedDeferTaskPosting& operator=(const ScopedDeferTaskPosting&) = delete;

  ~ScopedDeferTaskPosting();

 private:
  static ScopedDeferTaskPosting* Get();
  // Returns whether the |scope| was set as active, which happens only
  // when the scope wasn't set before.
  static bool Set(ScopedDeferTaskPosting* scope);

  void DeferTaskPosting(scoped_refptr<SequencedTaskRunner> task_runner,
                        const Location& from_here,
                        OnceClosure task,
                        base::TimeDelta delay);

  struct DeferredTask {
    DeferredTask(scoped_refptr<SequencedTaskRunner> task_runner,
                 Location from_here,
                 OnceClosure task,
                 base::TimeDelta delay);

    DeferredTask(const DeferredTask&) = delete;
    DeferredTask& operator=(const DeferredTask&) = delete;

    DeferredTask(DeferredTask&& task);

    ~DeferredTask();

    scoped_refptr<SequencedTaskRunner> task_runner;
    Location from_here;
    OnceClosure task;
    base::TimeDelta delay;
  };

  std::vector<DeferredTask> deferred_tasks_;

  // Scopes can be nested (e.g. ScheduleWork inside PostTasks can post a task
  // to another task runner), so we want to know whether the scope is top-level
  // or not.
  bool top_level_scope_ = false;
};

}  // namespace base

#endif  // BASE_TASK_COMMON_SCOPED_DEFER_TASK_POSTING_H_
