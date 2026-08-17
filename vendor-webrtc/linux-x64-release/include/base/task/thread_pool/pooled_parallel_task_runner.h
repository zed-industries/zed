// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_POOLED_PARALLEL_TASK_RUNNER_H_
#define BASE_TASK_THREAD_POOL_POOLED_PARALLEL_TASK_RUNNER_H_

#include "base/base_export.h"
#include "base/functional/callback_forward.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/task/task_runner.h"
#include "base/task/task_traits.h"
#include "base/time/time.h"

namespace base {
namespace internal {

class PooledTaskRunnerDelegate;

// A task runner that runs tasks in parallel.
class BASE_EXPORT PooledParallelTaskRunner : public TaskRunner {
 public:
  // Constructs a PooledParallelTaskRunner which can be used to post tasks.
  PooledParallelTaskRunner(
      const TaskTraits& traits,
      PooledTaskRunnerDelegate* pooled_task_runner_delegate);
  PooledParallelTaskRunner(const PooledParallelTaskRunner&) = delete;
  PooledParallelTaskRunner& operator=(const PooledParallelTaskRunner&) = delete;

  // TaskRunner:
  bool PostDelayedTask(const Location& from_here,
                       OnceClosure closure,
                       TimeDelta delay) override;

 private:
  ~PooledParallelTaskRunner() override;

  const TaskTraits traits_;
  const raw_ptr<PooledTaskRunnerDelegate, DanglingUntriaged>
      pooled_task_runner_delegate_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_POOLED_PARALLEL_TASK_RUNNER_H_
