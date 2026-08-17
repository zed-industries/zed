// Copyright 2018 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_UTIL_FUCHSIA_SCOPED_TASK_SUSPEND_H_
#define CRASHPAD_UTIL_FUCHSIA_SCOPED_TASK_SUSPEND_H_

#include <lib/zx/process.h>
#include <lib/zx/suspend_token.h>
#include <lib/zx/thread.h>

#include <vector>


namespace crashpad {

//! \brief Manages the suspension of another task.
//!
//! Suspending a process is asynchronous, and may take an arbitrary amount of
//! time. As a result, this class is limited to being a best-effort, and
//! correct suspension/resumption cannot be relied upon.
//!
//! Callers should not attempt to suspend the current task as obtained via
//! `zx_process_self()`.
class ScopedTaskSuspend {
 public:
  explicit ScopedTaskSuspend(const zx::process& process);

  ScopedTaskSuspend(const ScopedTaskSuspend&) = delete;
  ScopedTaskSuspend& operator=(const ScopedTaskSuspend&) = delete;

  ~ScopedTaskSuspend() = default;

 private:
  zx::suspend_token suspend_token_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_FUCHSIA_SCOPED_TASK_SUSPEND_H_
