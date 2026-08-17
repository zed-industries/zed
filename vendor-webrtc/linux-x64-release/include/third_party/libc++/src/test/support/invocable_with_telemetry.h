//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_INVOCABLE_WITH_TELEMETRY_H
#define TEST_SUPPORT_INVOCABLE_WITH_TELEMETRY_H

#include <cassert>
#include <concepts>
#include <functional>
#include <utility>

#if TEST_STD_VER < 20
#  error invocable_with_telemetry requires C++20
#else
struct invocable_telemetry {
  int invocations;
  int moves;
  int copies;
};

template <class F>
class invocable_with_telemetry {
public:
  constexpr invocable_with_telemetry(F f, invocable_telemetry& telemetry) : f_(f), telemetry_(&telemetry) {}

  constexpr invocable_with_telemetry(invocable_with_telemetry&& other)
    requires std::move_constructible<F>
      : f_(std::move(other.f_)),
        telemetry_((assert(other.telemetry_ != nullptr), std::exchange(other.telemetry_, nullptr))) {
    ++telemetry_->moves;
  }

  constexpr invocable_with_telemetry(invocable_with_telemetry const& other)
    requires std::copy_constructible<F>
      : f_(other.f_), telemetry_((assert(other.telemetry_ != nullptr), other.telemetry_)) {
    ++telemetry_->copies;
  }

  constexpr invocable_with_telemetry& operator=(invocable_with_telemetry&& other)
    requires std::movable<F>
  {
    // Not using move-and-swap idiom to ensure that copies and moves remain accurate.
    assert(&other != this);
    assert(other.telemetry_ != nullptr);

    f_         = std::move(other.f_);
    telemetry_ = std::exchange(other.telemetry_, nullptr);

    ++telemetry_->moves;
    return *this;
  }

  constexpr invocable_with_telemetry& operator=(invocable_with_telemetry const& other)
    requires std::copyable<F>
  {
    // Not using copy-and-swap idiom to ensure that copies and moves remain accurate.
    assert(&other != this);
    assert(other.telemetry_ != nullptr);

    f_         = other.f_;
    telemetry_ = other.telemetry_;

    ++telemetry_->copies;
    return *this;
  }

  template <class... Args>
    requires std::invocable<F&, Args...>
  constexpr decltype(auto) operator()(Args&&... args) noexcept(std::is_nothrow_invocable_v<F&, Args...>) {
    assert(telemetry_);
    ++telemetry_->invocations;
    return std::invoke(f_, std::forward<Args>(args)...);
  }

private:
  F f_                            = F();
  invocable_telemetry* telemetry_ = nullptr;
};

template <class F>
invocable_with_telemetry(F f, int& invocations, int& moves, int& copies) -> invocable_with_telemetry<F>;

#endif // TEST_STD_VER < 20
#endif // TEST_SUPPORT_INVOCABLE_WITH_TELEMETRY_H
