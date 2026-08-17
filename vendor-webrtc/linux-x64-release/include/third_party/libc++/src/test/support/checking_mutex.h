//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_CHECKING_MUTEX_H
#define TEST_SUPPORT_CHECKING_MUTEX_H

#include <cassert>
#include <chrono>

struct checking_mutex {
  enum state {
    locked_via_lock,
    locked_via_try_lock,
    locked_via_try_lock_for,
    locked_via_try_lock_until,
    unlocked,
    none,
  };

  state current_state = unlocked;
  state last_try      = none;
  bool reject         = false;

  checking_mutex()                      = default;
  checking_mutex(const checking_mutex&) = delete;
  ~checking_mutex() { assert(current_state == unlocked); }

  void lock() {
    assert(current_state == unlocked);
    assert(!reject);
    current_state = locked_via_lock;
    last_try      = locked_via_lock;
    reject        = true;
  }

  void unlock() {
    assert(current_state != unlocked && current_state != none);
    last_try      = unlocked;
    current_state = unlocked;
    reject        = false;
  }

  bool try_lock() {
    last_try = locked_via_try_lock;
    if (reject)
      return false;
    current_state = locked_via_try_lock;
    return true;
  }

  template <class Rep, class Period>
  bool try_lock_for(const std::chrono::duration<Rep, Period>&) {
    last_try = locked_via_try_lock_for;
    if (reject)
      return false;
    current_state = locked_via_try_lock_for;
    return true;
  }

  template <class Clock, class Duration>
  bool try_lock_until(const std::chrono::time_point<Clock, Duration>&) {
    last_try = locked_via_try_lock_until;
    if (reject)
      return false;
    current_state = locked_via_try_lock_until;
    return true;
  }

  checking_mutex* operator&() = delete;

  template <class T>
  void operator,(const T&) = delete;
};

#endif // TEST_SUPPORT_CHECKING_MUTEX_H
