// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_BIND_H_
#define BASE_TEST_BIND_H_

#include <string_view>
#include <type_traits>
#include <utility>

#include "base/functional/bind.h"

namespace base {

class Location;

namespace internal {

template <typename Signature, typename R, typename F, typename... Args>
static constexpr bool kHasConstCallOperator = false;
template <typename R, typename F, typename... Args>
static constexpr bool
    kHasConstCallOperator<R (F::*)(Args...) const, R, F, Args...> = true;

// Implementation of `BindLambdaForTesting()`, which checks preconditions before
// handing off to `Bind{Once,Repeating}()`.
template <typename Lambda,
          typename = ExtractCallableRunType<std::decay_t<Lambda>>>
struct BindLambdaForTestingHelper;

template <typename Lambda, typename R, typename... Args>
struct BindLambdaForTestingHelper<Lambda, R(Args...)> {
 private:
  using F = std::decay_t<Lambda>;

  // For context on this "templated struct with a lambda that asserts" pattern,
  // see comments in `Invoker<>`.
  template <bool v = std::is_rvalue_reference_v<Lambda&&> &&
                     !std::is_const_v<std::remove_reference_t<Lambda>>>
  struct IsNonConstRvalueRef {
    static constexpr bool value = [] {
      static_assert(
          v,
          "BindLambdaForTesting() requires non-const rvalue for mutable lambda "
          "binding, i.e. base::BindLambdaForTesting(std::move(lambda)).");
      return v;
    }();
  };

  static R Run(const F& f, Args... args) {
    return f(std::forward<Args>(args)...);
  }

  static R RunOnce(F&& f, Args... args) {
    return f(std::forward<Args>(args)...);
  }

 public:
  static auto BindLambdaForTesting(Lambda&& lambda) {
    if constexpr (kHasConstCallOperator<decltype(&F::operator()), R, F,
                                        Args...>) {
      // If WTF::BindRepeating is available, and a callback argument is in WTF,
      // then this call is ambiguous without the full namespace path.
      return ::base::BindRepeating(&Run, std::forward<Lambda>(lambda));
    } else if constexpr (IsNonConstRvalueRef<>::value) {
      // Since a mutable lambda potentially can invalidate its state after being
      // run once, this method returns a `OnceCallback` instead of a
      // `RepeatingCallback`.
      return BindOnce(&RunOnce, std::move(lambda));
    }
  }
};

}  // namespace internal

// A variant of `Bind{Once,Repeating}()` that can bind capturing lambdas for
// testing. This doesn't support extra arguments binding as the lambda itself
// can do.
template <typename Lambda>
auto BindLambdaForTesting(Lambda&& lambda) {
  return internal::BindLambdaForTestingHelper<Lambda>::BindLambdaForTesting(
      std::forward<Lambda>(lambda));
}

// Returns a closure that fails on destruction if it hasn't been run.
OnceClosure MakeExpectedRunClosure(
    const Location& location,
    std::string_view message = std::string_view());
RepeatingClosure MakeExpectedRunAtLeastOnceClosure(
    const Location& location,
    std::string_view message = std::string_view());

// Returns a closure that fails the test if run.
RepeatingClosure MakeExpectedNotRunClosure(
    const Location& location,
    std::string_view message = std::string_view());

}  // namespace base

#endif  // BASE_TEST_BIND_H_
