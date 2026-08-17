// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_LATCH_H
#define GRPC_SRC_CORE_LIB_PROMISE_LATCH_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <atomic>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/strings/str_cat.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/poll.h"

namespace grpc_core {

// Latches only work correctly within a single activity or a single party. If
// you need something that works across activities/parties refer to
// inter_activity_latch.h
//
// Latch provides a single set waitable object. We use Latch when we want to
// wait for a particular object to be set. This object would typically be set by
// some promise getting resolved.
//
// Initially the value Latch is unset.
//
// We can wait for Latch to be set using either the Wait or WaitAndCopy method.
// These two methods produce Promises that resolves when the Latch is value Set
// to a value of type T.
template <typename T>
class Latch {
 public:
  Latch() = default;
  Latch(const Latch&) = delete;
  explicit Latch(T value) : value_(std::move(value)), has_value_(true) {}
  Latch& operator=(const Latch&) = delete;
  Latch(Latch&& other) noexcept
      : value_(std::move(other.value_)), has_value_(other.has_value_) {
#ifndef NDEBUG
    DCHECK(!other.has_had_waiters_);
#endif
  }
  Latch& operator=(Latch&& other) noexcept {
#ifndef NDEBUG
    DCHECK(!other.has_had_waiters_);
#endif
    value_ = std::move(other.value_);
    has_value_ = other.has_value_;
    return *this;
  }

  // Produce a promise to wait for a value from this latch.
  // Moves the result out of the latch.
  auto Wait() {
#ifndef NDEBUG
    has_had_waiters_ = true;
#endif
    return [this]() -> Poll<T> {
      GRPC_TRACE_LOG(promise_primitives, INFO)
          << DebugTag() << "Wait " << StateString();
      if (has_value_) {
        return std::move(value_);
      } else {
        return waiter_.pending();
      }
    };
  }

  // Produce a promise to wait for a value from this latch.
  // Copies the result out of the latch.
  auto WaitAndCopy() {
#ifndef NDEBUG
    has_had_waiters_ = true;
#endif
    return [this]() -> Poll<T> {
      GRPC_TRACE_LOG(promise_primitives, INFO)
          << DebugTag() << "WaitAndCopy " << StateString();
      if (has_value_) {
        return value_;
      } else {
        return waiter_.pending();
      }
    };
  }

  // Set the value of the latch. Can only be called once.
  void Set(T value) {
    GRPC_TRACE_LOG(promise_primitives, INFO)
        << DebugTag() << "Set " << StateString();
    DCHECK(!has_value_);
    value_ = std::move(value);
    has_value_ = true;
    waiter_.Wake();
  }

  bool is_set() const { return has_value_; }

 private:
  std::string DebugTag() {
    return absl::StrCat(GetContext<Activity>()->DebugTag(), " LATCH[0x",
                        reinterpret_cast<uintptr_t>(this), "]: ");
  }

  std::string StateString() {
    return absl::StrCat("has_value:", has_value_ ? "true" : "false",
                        " waiter:", waiter_.DebugString());
  }

  // The value stored (if has_value_ is true), otherwise some random value, we
  // don't care.
  // Why not std::optional<>? Writing things this way lets us compress
  // has_value_ with waiter_ and leads to some significant memory savings for
  // some scenarios.
  GPR_NO_UNIQUE_ADDRESS T value_;
  // True if we have a value set, false otherwise.
  bool has_value_ = false;
#ifndef NDEBUG
  // Has this latch ever had waiters.
  bool has_had_waiters_ = false;
#endif
  IntraActivityWaiter waiter_;
};

// Specialization for void.
template <>
class Latch<void> {
 public:
  Latch() = default;
  Latch(const Latch&) = delete;
  Latch& operator=(const Latch&) = delete;
  Latch(Latch&& other) noexcept : is_set_(other.is_set_) {
#ifndef NDEBUG
    DCHECK(!other.has_had_waiters_);
#endif
  }
  Latch& operator=(Latch&& other) noexcept {
#ifndef NDEBUG
    DCHECK(!other.has_had_waiters_);
#endif
    is_set_ = other.is_set_;
    return *this;
  }

  // Produce a promise to wait for this latch.
  auto Wait() {
#ifndef NDEBUG
    has_had_waiters_ = true;
#endif
    return [this]() -> Poll<Empty> {
      GRPC_TRACE_LOG(promise_primitives, INFO)
          << DebugTag() << "PollWait " << StateString();
      if (is_set_) {
        return Empty{};
      } else {
        return waiter_.pending();
      }
    };
  }

  // Set the latch. Can only be called once.
  void Set() {
    GRPC_TRACE_LOG(promise_primitives, INFO)
        << DebugTag() << "Set " << StateString();
    DCHECK(!is_set_);
    is_set_ = true;
    waiter_.Wake();
  }

  bool is_set() const { return is_set_; }

 private:
  std::string DebugTag() {
    return absl::StrCat(GetContext<Activity>()->DebugTag(), " LATCH(void)[0x",
                        reinterpret_cast<uintptr_t>(this), "]: ");
  }

  std::string StateString() {
    return absl::StrCat("is_set:", is_set_ ? "true" : "false",
                        " waiter:", waiter_.DebugString());
  }

  // True if we have a value set, false otherwise.
  bool is_set_ = false;
#ifndef NDEBUG
  // Has this latch ever had waiters.
  bool has_had_waiters_ = false;
#endif
  IntraActivityWaiter waiter_;
};

template <typename T>
using LatchWaitPromise = decltype(std::declval<Latch<T>>().Wait());

// A Latch that can have its value observed by outside threads, but only waited
// upon from inside a single activity.
template <typename T>
class ExternallyObservableLatch;

template <>
class ExternallyObservableLatch<void> {
 public:
  ExternallyObservableLatch() = default;
  ExternallyObservableLatch(const ExternallyObservableLatch&) = delete;
  ExternallyObservableLatch& operator=(const ExternallyObservableLatch&) =
      delete;

  // Produce a promise to wait for this latch.
  auto Wait() {
    return [this]() -> Poll<Empty> {
      GRPC_TRACE_LOG(promise_primitives, INFO)
          << DebugTag() << "PollWait " << StateString();
      if (IsSet()) {
        return Empty{};
      } else {
        return waiter_.pending();
      }
    };
  }

  // Set the latch.
  void Set() {
    GRPC_TRACE_LOG(promise_primitives, INFO)
        << DebugTag() << "Set " << StateString();
    is_set_.store(true, std::memory_order_relaxed);
    waiter_.Wake();
  }

  bool IsSet() const { return is_set_.load(std::memory_order_relaxed); }

  void Reset() {
    GRPC_TRACE_LOG(promise_primitives, INFO)
        << DebugTag() << "Reset " << StateString();
    is_set_.store(false, std::memory_order_relaxed);
  }

 private:
  std::string DebugTag() {
    return absl::StrCat(GetContext<Activity>()->DebugTag(), " LATCH(void)[0x",
                        reinterpret_cast<uintptr_t>(this), "]: ");
  }

  std::string StateString() {
    return absl::StrCat(
        "is_set:", is_set_.load(std::memory_order_relaxed) ? "true" : "false",
        " waiter:", waiter_.DebugString());
  }

  // True if we have a value set, false otherwise.
  std::atomic<bool> is_set_{false};
  IntraActivityWaiter waiter_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_LATCH_H
