// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_THREAD_POOL_EXECUTOR_H_
#define MEDIAPIPE_FRAMEWORK_THREAD_POOL_EXECUTOR_H_

#include "mediapipe/framework/deps/thread_options.h"
#include "mediapipe/framework/executor.h"
#include "mediapipe/framework/port/statusor.h"
#include "mediapipe/framework/port/threadpool.h"

namespace mediapipe {

// A multithreaded executor based on a thread pool.
class ThreadPoolExecutor : public Executor {
 public:
  static absl::StatusOr<Executor*> Create(
      const MediaPipeOptions& extendable_options);

  explicit ThreadPoolExecutor(int num_threads);
  ~ThreadPoolExecutor() override;
  void Schedule(std::function<void()> task) override;

  // For testing.
  int num_threads() const { return thread_pool_.num_threads(); }
  // Returns the thread stack size (in bytes).
  size_t stack_size() const { return stack_size_; }

 private:
  ThreadPoolExecutor(const ThreadOptions& thread_options, int num_threads);

  // Saves the value of the stack size option and starts the thread pool.
  void Start();

  mediapipe::ThreadPool thread_pool_;

  // Records the stack size in ThreadOptions right before we call
  // thread_pool_.StartWorkers().
  //
  // The actual stack size passed to pthread_attr_setstacksize() for the
  // worker threads differs from the stack size we specified. It includes the
  // guard size and space for thread-local storage. (See Thread::Start() in
  // thread/thread.cc.) So the unit tests check the stack size in
  // ThreadOptions, in addition to trying to recover the specified stack
  // size from the stack size returned by pthread_getattr_np(),
  // pthread_attr_getstacksize(), and pthread_attr_getguardsize().
  size_t stack_size_ = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_THREAD_POOL_EXECUTOR_H_
