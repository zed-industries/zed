// Copyright 2022 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_SYNCHRONIZATION_SCOPED_SPIN_GUARD_H_
#define CRASHPAD_UTIL_SYNCHRONIZATION_SCOPED_SPIN_GUARD_H_

#include <atomic>
#include <optional>
#include <utility>

#include "base/check.h"
#include "base/notreached.h"
#include "util/misc/clock.h"

namespace crashpad {

//! \brief Spinlock state for `ScopedSpinGuard`.
struct SpinGuardState final {
  //! \brief A `ScopedSpinGuard` in an unlocked state.
  constexpr SpinGuardState() : locked(false) {}

  SpinGuardState(const SpinGuardState&) = delete;
  SpinGuardState& operator=(const SpinGuardState&) = delete;

  //! \brief `true` if the `ScopedSpinGuard` is locked, `false` otherwise.
  std::atomic<bool> locked;
  static_assert(std::atomic<bool>::is_always_lock_free,
                "std::atomic<bool> may not be signal-safe");
  static_assert(sizeof(std::atomic<bool>) == sizeof(bool),
                "std::atomic<bool> adds size to bool");
};

//! \brief A scoped mutual-exclusion guard wrapping a `SpinGuardState` with RAII
//!     semantics.
class ScopedSpinGuard final {
  //! \brief The duration in nanoseconds between attempts to lock the spinlock.
  static constexpr uint64_t kSpinGuardSleepTimeNanos = 10;

 public:
  ScopedSpinGuard(const ScopedSpinGuard&) = delete;
  ScopedSpinGuard& operator=(const ScopedSpinGuard&) = delete;
  ScopedSpinGuard(ScopedSpinGuard&& other) noexcept : state_(nullptr) {
    std::swap(state_, other.state_);
  }
  ScopedSpinGuard& operator=(ScopedSpinGuard&& other) {
    std::swap(state_, other.state_);
    return *this;
  }

  //! \brief Spins up to `timeout_nanos` nanoseconds trying to lock `state`.
  //! \param[in] timeout_nanos The timeout in nanoseconds after which this gives
  //!     up trying to lock the spinlock and returns `std::nullopt`.
  //! \param[in,out] state The spinlock state to attempt to lock. This method
  //!     holds a pointer to `state`, so `state` must outlive the lifetime of
  //!     this object.
  //! \return The locked `ScopedSpinGuard` on success, or `std::nullopt` on
  //!     timeout.
  static std::optional<ScopedSpinGuard> TryCreateScopedSpinGuard(
      uint64_t timeout_nanos,
      SpinGuardState& state) {
    const uint64_t clock_end_time_nanos =
        ClockMonotonicNanoseconds() + timeout_nanos;
    while (true) {
      bool expected_current_value = false;
      // `std::atomic::compare_exchange_weak()` is allowed to spuriously fail on
      // architectures like ARM, which can cause timeouts even for
      // single-threaded cases (https://crbug.com/340980960,
      // http://b/296082201).
      //
      // Use `std::compare_exchange_strong()` instead to avoid spurious failures
      // in the common case.
      if (state.locked.compare_exchange_strong(expected_current_value,
                                               true,
                                               std::memory_order_acquire,
                                               std::memory_order_relaxed)) {
        return std::make_optional<ScopedSpinGuard>(state);
      }
      if (ClockMonotonicNanoseconds() >= clock_end_time_nanos) {
        return std::nullopt;
      }
      SleepNanoseconds(kSpinGuardSleepTimeNanos);
    }

    NOTREACHED();
  }

  ~ScopedSpinGuard() {
    if (state_) {
#ifdef NDEBUG
      state_->locked.store(false, std::memory_order_release);
#else
      bool old = state_->locked.exchange(false, std::memory_order_release);
      DCHECK(old);
#endif
    }
  }

  //! \brief A `ScopedSpinGuard` wrapping a locked `SpinGuardState`.
  //! \param[in,out] locked_state A locked `SpinGuardState`. This method
  //!     holds a pointer to `state`, so `state` must outlive the lifetime of
  //!     this object.
  ScopedSpinGuard(SpinGuardState& locked_state) : state_(&locked_state) {
    DCHECK(locked_state.locked);
  }

 private:
  // \brief Optional spinlock state, unlocked when this object goes out of
  //     scope.
  //
  // If this is `nullptr`, then this object has been moved from, and the state
  // is no longer valid. In that case, nothing will be unlocked when this object
  // is destroyed.
  SpinGuardState* state_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_SYNCHRONIZATION_SCOPED_SPIN_GUARD_H_
