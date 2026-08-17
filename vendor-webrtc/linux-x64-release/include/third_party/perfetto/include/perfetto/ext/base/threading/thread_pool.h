/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_BASE_THREADING_THREAD_POOL_H_
#define INCLUDE_PERFETTO_EXT_BASE_THREADING_THREAD_POOL_H_

#include <condition_variable>
#include <functional>
#include <list>
#include <mutex>
#include <optional>
#include <thread>
#include <vector>

#include "perfetto/base/task_runner.h"
#include "perfetto/base/thread_annotations.h"

namespace perfetto {
namespace base {

// Bounded thread pool designed for CPU-bound tasks.
//
// This is a classic bounded thread pool designed for running jobs which fully
// occupy the CPU without blocking. IO bound tasks which block for long periods
// of times will cause starvation for any other tasks which are waiting.
// IO-heavy tasks should use base::TaskRunner and async-IO instead of using this
// class.
//
// Threads are created when the thread pool is created and persist for the
// lifetime of the ThreadPool. No new threads are created after construction.
// When the ThreadPool is destroyed, any active tasks are completed and every
// thread joined before returning from the destructor.
//
// Tasks are executed in a FIFO order without any notion of priority. If a
// thread in the pool is free, it will be used to execute the task immediately.
// Otherwise, it will be queued for execution when any thread becomes available.
class ThreadPool {
 public:
  // Initializes this thread_pool |thread_count| threads.
  explicit ThreadPool(uint32_t thread_count);
  ~ThreadPool();

  // Submits a task for execution by any thread in this thread pool.
  //
  // This task should not block for IO as this can cause starvation.
  void PostTask(std::function<void()>);

 private:
  void RunThreadLoop();

  ThreadPool(ThreadPool&&) = delete;
  ThreadPool& operator=(ThreadPool&&) = delete;

  std::mutex mutex_;
  std::list<std::function<void()>> pending_tasks_ PERFETTO_GUARDED_BY(mutex_);
  std::condition_variable thread_waiter_ PERFETTO_GUARDED_BY(mutex_);
  uint32_t thread_waiting_count_ PERFETTO_GUARDED_BY(mutex_) = 0;
  bool quit_ PERFETTO_GUARDED_BY(mutex_) = false;

  std::vector<std::thread> threads_;
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_THREADING_THREAD_POOL_H_
