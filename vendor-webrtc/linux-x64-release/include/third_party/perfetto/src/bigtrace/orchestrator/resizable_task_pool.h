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

#ifndef SRC_BIGTRACE_ORCHESTRATOR_RESIZABLE_TASK_POOL_H_
#define SRC_BIGTRACE_ORCHESTRATOR_RESIZABLE_TASK_POOL_H_

#include <functional>
#include <mutex>
#include <thread>

#include <grpcpp/client_context.h>

namespace perfetto::bigtrace {

// This struct maps a thread to a context in order to allow for the cancellation
// of the thread's current gRPC call through ClientContext's TryCancel
struct ThreadWithContext {
  explicit ThreadWithContext(std::function<void(ThreadWithContext*)> fn)
      : thread(fn, this) {}

  // Cancels the gRPC call through ClientContext as well as signalling a stop to
  // the thread
  void Cancel() {
    client_context->TryCancel();
    std::lock_guard<std::mutex> lk(mutex);
    is_thread_cancelled = true;
  }

  // Returns whether the thread has been cancelled
  bool IsCancelled() {
    std::lock_guard<std::mutex> lk(mutex);
    return is_thread_cancelled;
  }

  std::mutex mutex;
  std::unique_ptr<grpc::ClientContext> client_context;
  std::thread thread;
  bool is_thread_cancelled = false;
};

// This pool manages a set of running tasks for a given query, and provides the
// ability to resize in order to fairly distribute an equal number of workers
// for each user through preemption
class ResizableTaskPool {
 public:
  explicit ResizableTaskPool(std::function<void(ThreadWithContext*)> fn);
  void Resize(uint32_t new_size);
  void JoinAll();

 private:
  std::function<void(ThreadWithContext*)> fn_;
  std::vector<std::unique_ptr<ThreadWithContext>> contextual_threads_;
};
}  // namespace perfetto::bigtrace

#endif  // SRC_BIGTRACE_ORCHESTRATOR_RESIZABLE_TASK_POOL_H_
