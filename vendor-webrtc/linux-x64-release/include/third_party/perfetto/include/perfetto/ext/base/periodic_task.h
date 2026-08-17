/*
 * Copyright (C) 2021 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_PERIODIC_TASK_H_
#define INCLUDE_PERFETTO_EXT_BASE_PERIODIC_TASK_H_

#include <functional>

#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/base/thread_checker.h"
#include "perfetto/ext/base/weak_ptr.h"

namespace perfetto {
namespace base {

class TaskRunner;

// A periodic task utility class. It wraps the logic necessary to do periodic
// tasks using a TaskRunner, taking care of subtleties like ensuring that
// outstanding tasks are cancelled after reset/dtor.
// Tasks are aligned on wall time (unless they are |one_shot|). This is to
// ensure that when using multiple periodic tasks, they happen at the same time,
// minimizing context switches.
// On Linux/Android it also supports suspend-aware mode (via timerfd). On other
// operating systems it falls back to PostDelayedTask, which is not
// suspend-aware.
// TODO(primiano): this should probably become a periodic timer scheduler, so we
// can use one FD for everything rather than one FD per task. For now we take
// the hit of a FD-per-task to keep this low-risk.
// TODO(primiano): consider renaming this class to TimerTask. When |one_shot|
// is set, the "Periodic" part of the class name becomes a lie.
class PeriodicTask {
 public:
  explicit PeriodicTask(base::TaskRunner*);
  ~PeriodicTask();  // Calls Reset().

  struct Args {
    uint32_t period_ms = 0;
    std::function<void()> task = nullptr;
    bool start_first_task_immediately = false;
    bool use_suspend_aware_timer = false;
    bool one_shot = false;
  };

  void Start(Args);

  // Safe to be called multiple times, even without calling Start():
  void Reset();

  // No copy or move. WeakPtr-wrapped pointers to |this| are posted on the
  // task runner, this class is not easily movable.
  PeriodicTask(const PeriodicTask&) = delete;
  PeriodicTask& operator=(const PeriodicTask&) = delete;
  PeriodicTask(PeriodicTask&&) = delete;
  PeriodicTask& operator=(PeriodicTask&&) = delete;

  base::PlatformHandle timer_fd_for_testing() { return *timer_fd_; }

 private:
  static void RunTaskAndPostNext(base::WeakPtr<PeriodicTask>,
                                 uint32_t generation);
  void PostNextTask();
  void ResetTimerFd();

  base::TaskRunner* const task_runner_;
  Args args_;
  uint32_t generation_ = 0;
  base::ScopedPlatformHandle timer_fd_;

  PERFETTO_THREAD_CHECKER(thread_checker_)
  base::WeakPtrFactory<PeriodicTask> weak_ptr_factory_;  // Keep last.
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_PERIODIC_TASK_H_
