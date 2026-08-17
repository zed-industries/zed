// Copyright 2023 The Centipede Authors.
// Copyright 2017 The Abseil Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_CENTIPEDE_THREAD_POOL_H_
#define THIRD_PARTY_CENTIPEDE_THREAD_POOL_H_

#include <cstddef>
#include <queue>
#include <thread>
#include <utility>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "absl/log/check.h"
#include "absl/synchronization/mutex.h"

namespace fuzztest::internal {

// A simple thread pool implementation based on `std::thread`.
class ThreadPool {
 public:
  // Initializes this ThreadPool by starting the requested number of worker
  // threads.
  explicit ThreadPool(int num_threads) {
    threads_.reserve(num_threads);
    for (int i = 0; i < num_threads; ++i) {
      threads_.push_back(std::thread{&ThreadPool::WorkLoop, this});
    }
  }

  ThreadPool(const ThreadPool &) = delete;
  ThreadPool &operator=(const ThreadPool &) = delete;

  // Shuts down this ThreadPool by sending shutdown signals to all the worker
  // threads and waiting for them to wrap up work and join.
  ~ThreadPool() {
    {
      absl::MutexLock l{&mu_};
      for (size_t i = 0; i < threads_.size(); ++i) {
        queue_.push(nullptr);  // Shutdown signal.
      }
    }
    for (auto &t : threads_) {
      t.join();
    }
  }

  // Schedules a function to be run on a ThreadPool thread immediately.
  void Schedule(absl::AnyInvocable<void()> func) {
    CHECK(func != nullptr);
    absl::MutexLock l{&mu_};
    queue_.push(std::move(func));
  }

 private:
  // Tells the waiting worker threads when new work becomes available in the
  // queue.
  bool WorkAvailable() const ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    return !queue_.empty();
  }

  // The work loop that every worker thread iterates over, waiting and
  // executing newly scheduled work.
  void WorkLoop() {
    while (true) {
      absl::AnyInvocable<void()> func;
      {
        absl::MutexLock l{&mu_};
        mu_.Await(absl::Condition{this, &ThreadPool::WorkAvailable});
        func = std::move(queue_.front());
        queue_.pop();
      }
      if (func == nullptr) {  // Shutdown signal.
        break;
      }
      func();
    }
  }

  absl::Mutex mu_;
  std::queue<absl::AnyInvocable<void()>> queue_ ABSL_GUARDED_BY(mu_);
  std::vector<std::thread> threads_;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_THREAD_POOL_H_
