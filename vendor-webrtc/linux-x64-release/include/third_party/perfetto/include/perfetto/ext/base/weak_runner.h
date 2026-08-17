/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_BASE_WEAK_RUNNER_H_
#define INCLUDE_PERFETTO_EXT_BASE_WEAK_RUNNER_H_

#include <stdint.h>

#include <functional>
#include <memory>

namespace perfetto::base {

class TaskRunner;

// This is a wrapper around a `base::TaskRunner*`. It is intended to be used by
// classes that want to post tasks on themselves. When the object is destroyed,
// all posted tasks become noops.
//
// A class that embeds a WeakRunner can safely capture `this` on the posted
// tasks.
class WeakRunner {
 public:
  explicit WeakRunner(base::TaskRunner* task_runner);
  ~WeakRunner();
  base::TaskRunner* task_runner() const { return task_runner_; }

  // Schedules `f` for immediate execution. `f` will not be executed is `*this`
  // is destroyed.
  //
  // Can be called from any thread, but the caller needs to make sure that
  // `*this` is alive while `PostTask` is running: this is not obvious when
  // multiple threads are involved.
  void PostTask(std::function<void()> f) const;

  // Schedules `f` for execution after |delay_ms|.
  // Can be called from any thread, but the caller needs to make sure that
  // `*this` is alive while `PostDelayedTask` is running: this is not obvious
  // when multiple threads are involved.
  void PostDelayedTask(std::function<void()> f, uint32_t delay_ms) const;

 private:
  base::TaskRunner* const task_runner_;
  std::shared_ptr<bool> destroyed_;
};

}  // namespace perfetto::base

#endif  // INCLUDE_PERFETTO_EXT_BASE_WEAK_RUNNER_H_
