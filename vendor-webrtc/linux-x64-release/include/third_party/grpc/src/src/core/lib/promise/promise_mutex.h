// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_PROMISE_MUTEX_H
#define GRPC_SRC_CORE_LIB_PROMISE_PROMISE_MUTEX_H

#include <grpc/support/port_platform.h>

#include <utility>

#include "absl/log/check.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/poll.h"

namespace grpc_core {

// A mutex that can be used to synchronize access to a value within one
// activity.
template <typename T>
class PromiseMutex {
 public:
  class Lock {
   public:
    Lock() {}
    ~Lock() {
      if (mutex_ != nullptr) {
        CHECK(mutex_->locked_);
        mutex_->locked_ = false;
        mutex_->waiter_.Wake();
      }
    }

    Lock(Lock&& other) noexcept
        : mutex_(std::exchange(other.mutex_, nullptr)) {}
    Lock& operator=(Lock&& other) noexcept {
      std::swap(mutex_, other.mutex_);
      return *this;
    }

    Lock(const Lock&) = delete;
    Lock& operator=(const Lock&) noexcept = delete;

    T* operator->() {
      DCHECK_NE(mutex_, nullptr);
      return &mutex_->value_;
    }
    T& operator*() {
      DCHECK_NE(mutex_, nullptr);
      return mutex_->value_;
    }

   private:
    friend class PromiseMutex;
    explicit Lock(PromiseMutex* mutex) : mutex_(mutex) {
      DCHECK(!mutex_->locked_);
      mutex_->locked_ = true;
    }
    PromiseMutex* mutex_ = nullptr;
  };

  PromiseMutex() = default;
  explicit PromiseMutex(T value) : value_(std::move(value)) {}
  ~PromiseMutex() { DCHECK(!locked_); }

  auto Acquire() {
    return [this]() -> Poll<Lock> {
      if (locked_) return waiter_.pending();
      return Lock(this);
    };
  }

 private:
  bool locked_ = false;
  IntraActivityWaiter waiter_;
  GPR_NO_UNIQUE_ADDRESS T value_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_PROMISE_MUTEX_H
