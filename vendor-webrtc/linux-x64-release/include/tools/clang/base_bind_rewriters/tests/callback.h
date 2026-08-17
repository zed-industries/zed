// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_BASE_BIND_REWRITERS_TESTS_CALLBACK_H_
#define TOOLS_CLANG_BASE_BIND_REWRITERS_TESTS_CALLBACK_H_

#include <type_traits>
#include <utility>

namespace base {
namespace internal {

template <typename T>
class PassedWrapper {
 public:
  explicit PassedWrapper(T&& scoper) {}
  PassedWrapper(PassedWrapper&& other) {}
};

}  // namespace internal

template <typename T,
          std::enable_if_t<!std::is_lvalue_reference<T>::value>* = nullptr>
internal::PassedWrapper<T> Passed(T&& scoper) {
  return internal::PassedWrapper<T>(std::move(scoper));
}

template <typename T>
internal::PassedWrapper<T> Passed(T* scoper) {
  return internal::PassedWrapper<T>(std::move(*scoper));
}

template <typename Signature>
class OnceCallback;

template <typename Signature>
class RepeatingCallback;

template <typename Signature>
using Callback = RepeatingCallback<Signature>;

using OnceClosure = OnceCallback<void()>;
using RepeatingClosure = RepeatingCallback<void()>;
using Closure = Callback<void()>;

template <typename R, typename... Args>
class OnceCallback<R(Args...)> {
 public:
  OnceCallback() {}

  OnceCallback(OnceCallback&&) = default;
  OnceCallback& operator=(OnceCallback&&) = default;

  OnceCallback(const OnceCallback&) = delete;
  OnceCallback& operator=(const OnceCallback&) = delete;

  OnceCallback(RepeatingCallback<R(Args...)> other) {}
  OnceCallback& operator=(RepeatingCallback<R(Args...)> other) { return *this; }

  R Run(Args... args) const & {
    static_assert(!sizeof(*this), "");
    return R();
  }
  R Run(Args... args) && { return R(); }
};

template <typename R, typename... Args>
class RepeatingCallback<R(Args...)> {
 public:
  RepeatingCallback() {}

  RepeatingCallback(const RepeatingCallback&) = default;
  RepeatingCallback& operator=(const RepeatingCallback&) = default;

  RepeatingCallback(RepeatingCallback&&) = default;
  RepeatingCallback& operator=(RepeatingCallback&&) = default;

  R Run(Args... args) const & { return R(); }
  R Run(Args... args) && { return R(); }
};

template <typename Functor, typename... Args>
Callback<void()> Bind(Functor, Args&&...) {
  return Callback<void()>();
}

template <typename Functor, typename... Args>
OnceCallback<void()> BindOnce(Functor, Args&&...) {
  return OnceCallback<void()>();
}

template <typename Functor, typename... Args>
RepeatingCallback<void()> BindRepeating(Functor, Args&&...) {
  return RepeatingCallback<void()>();
}

}  // namespace base

#endif  // TOOLS_CLANG_BASE_BIND_REWRITERS_TESTS_CALLBACK_H_
