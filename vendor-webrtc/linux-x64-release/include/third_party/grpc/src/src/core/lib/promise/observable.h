// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_OBSERVABLE_H
#define GRPC_SRC_CORE_LIB_PROMISE_OBSERVABLE_H

#include <grpc/support/port_platform.h>

#include "absl/container/flat_hash_set.h"
#include "absl/functional/any_invocable.h"
#include "absl/log/check.h"
#include "src/core/lib/promise/activity.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/util/sync.h"

namespace grpc_core {

// Observable allows broadcasting a value to multiple interested observers.
template <typename T>
class Observable {
 public:
  // We need to assign a value initially.
  explicit Observable(T initial)
      : state_(MakeRefCounted<State>(std::move(initial))) {}

  // Update the value to something new. Awakes any waiters.
  void Set(T value) { state_->Set(std::move(value)); }

  // Returns a promise that resolves to a T when is_acceptable returns true for
  // that value.
  // is_acceptable is any invocable that takes a `const T&` and returns a bool.
  template <typename F>
  auto NextWhen(F is_acceptable) {
    return ObserverWhen<F>(state_, std::move(is_acceptable));
  }

  // Returns a promise that resolves to a T when the value becomes != current.
  auto Next(T current) {
    return NextWhen([current = std::move(current)](const T& value) {
      return value != current;
    });
  }

 private:
  // Forward declaration so we can form pointers to Observer in State.
  class Observer;

  // State keeps track of all observable state.
  // It's a refcounted object so that promises reading the state are not tied
  // to the lifetime of the Observable.
  class State : public RefCounted<State> {
   public:
    explicit State(T value) : value_(std::move(value)) {}

    // Update the value and wake all observers.
    void Set(T value) {
      MutexLock lock(&mu_);
      std::swap(value_, value);
      WakeAll();
    }

    // Export our mutex so that Observer can use it.
    Mutex* mu() ABSL_LOCK_RETURNED(mu_) { return &mu_; }

    // Fetch a ref to the current value.
    const T& current() const ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
      return value_;
    }

    // Remove an observer from the set (it no longer needs updates).
    void Remove(Observer* observer) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
      observers_.erase(observer);
    }

    // Add an observer to the set (it needs updates).
    GRPC_MUST_USE_RESULT Waker Add(Observer* observer)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
      observers_.insert(observer);
      return GetContext<Activity>()->MakeNonOwningWaker();
    }

   private:
    // Wake all observers.
    void WakeAll() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
      for (auto* observer : observers_) {
        observer->Wakeup();
      }
    }

    Mutex mu_;
    // All observers that may need an update.
    absl::flat_hash_set<Observer*> observers_ ABSL_GUARDED_BY(mu_);
    // The current value.
    T value_ ABSL_GUARDED_BY(mu_);
  };

  // A promise that resolves to a T when ShouldReturn() returns true.
  // Subclasses must implement ShouldReturn().
  class Observer {
   public:
    explicit Observer(RefCountedPtr<State> state) : state_(std::move(state)) {}

    virtual ~Observer() {
      // If we saw a pending at all then we *may* be in the set of observers.
      // If not we're definitely not and we can avoid taking the lock at all.
      if (!saw_pending_) return;
      MutexLock lock(state_->mu());
      auto w = std::move(waker_);
      state_->Remove(this);
    }

    Observer(const Observer&) = delete;
    Observer& operator=(const Observer&) = delete;
    Observer(Observer&& other) noexcept : state_(std::move(other.state_)) {
      CHECK(other.waker_.is_unwakeable());
      DCHECK(waker_.is_unwakeable());
      CHECK(!other.saw_pending_);
    }
    Observer& operator=(Observer&& other) noexcept = delete;

    void Wakeup() { waker_.WakeupAsync(); }

    virtual bool ShouldReturn(const T& current) = 0;

    Poll<T> operator()() {
      MutexLock lock(state_->mu());
      // Check if the value has changed yet.
      if (ShouldReturn(state_->current())) {
        if (saw_pending_ && !waker_.is_unwakeable()) state_->Remove(this);
        return state_->current();
      }
      // Record that we saw at least one pending and then register for wakeup.
      saw_pending_ = true;
      if (waker_.is_unwakeable()) waker_ = state_->Add(this);
      return Pending{};
    }

   private:
    RefCountedPtr<State> state_;
    Waker waker_;
    bool saw_pending_ = false;
  };

  // A promise that resolves to a T when is_acceptable returns true for
  // the current value.
  template <typename F>
  class ObserverWhen : public Observer {
   public:
    ObserverWhen(RefCountedPtr<State> state, F is_acceptable)
        : Observer(std::move(state)),
          is_acceptable_(std::move(is_acceptable)) {}

    ObserverWhen(ObserverWhen&& other) noexcept
        : Observer(std::move(other)),
          is_acceptable_(std::move(other.is_acceptable_)) {}

    bool ShouldReturn(const T& current) override {
      return is_acceptable_(current);
    }

   private:
    F is_acceptable_;
  };

  RefCountedPtr<State> state_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_OBSERVABLE_H
