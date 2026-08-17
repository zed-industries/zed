// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_NULL_TASK_RUNNER_H_
#define BASE_TEST_NULL_TASK_RUNNER_H_

#include "base/compiler_specific.h"
#include "base/functional/callback.h"
#include "base/task/single_thread_task_runner.h"

namespace base {

// ATTENTION: Prefer SingleThreadTaskEnvironment or TaskEnvironment w/
// ThreadPoolExecutionMode::QUEUED over this class. A NullTaskRunner might seem
// appealing, but not running tasks is under-testing the potential side-effects
// of the code under tests. All tests should be okay if tasks born from their
// actions are run or deleted at a later point.
//
// Helper class for tests that need to provide an implementation of a
// *TaskRunner class but don't actually care about tasks being run.
class NullTaskRunner : public base::SingleThreadTaskRunner {
 public:
  NullTaskRunner();

  NullTaskRunner(const NullTaskRunner&) = delete;
  NullTaskRunner& operator=(const NullTaskRunner&) = delete;

  bool PostDelayedTask(const Location& from_here,
                       base::OnceClosure task,
                       base::TimeDelta delay) override;
  bool PostNonNestableDelayedTask(const Location& from_here,
                                  base::OnceClosure task,
                                  base::TimeDelta delay) override;
  // Always returns true to avoid triggering DCHECKs.
  bool RunsTasksInCurrentSequence() const override;

 protected:
  ~NullTaskRunner() override;
};

}  // namespace base

#endif  // BASE_TEST_NULL_TASK_RUNNER_H_
