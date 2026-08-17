// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_GMOCK_CALLBACK_SUPPORT_H_
#define BASE_TEST_GMOCK_CALLBACK_SUPPORT_H_

#include <ostream>
#include <tuple>
#include <utility>

#include "base/check.h"
#include "base/functional/callback.h"
#include "base/memory/ref_counted.h"
#include "base/memory/scoped_refptr.h"
#include "testing/gmock/include/gmock/gmock.h"

namespace base {

namespace gmock_callback_support_internal {

// Small helper to get the `I`th argument.
template <size_t I, typename... Args>
decltype(auto) get(Args&&... args) {
  return std::get<I>(std::forward_as_tuple(std::forward<Args>(args)...));
}

// Wraps `tuple` inside RefCountedData<std::unique_ptr<Tuple>> to allow creating
// shallow copies in lambda return of RunOnceCallback<>.
// Since RefCountedData<Tuple> stores Tuple directly, the indirection via
// std::unique_ptr<Tuple> is necessary to be able to CHECK() on second
// invocation instead of running the callback with a default-constructed tuple.
template <typename Tuple>
auto WrapTupleAsRefCountedData(Tuple&& tuple) {
  return MakeRefCounted<RefCountedData<std::unique_ptr<Tuple>>>(
      std::make_unique<Tuple>(std::forward<Tuple>(tuple)));
}

// Invokes `cb` with the arguments stored in `tuple`. Both `cb` and `tuple` are
// perfectly forwarded, allowing callers to specify whether they should be
// passed by move or copy.
template <typename Callback, typename Tuple>
decltype(auto) RunImpl(Callback&& cb, Tuple&& tuple) {
  return std::apply(
      [&](auto&&... args) -> decltype(auto) {
        return std::forward<Callback>(cb).Run(
            std::forward<decltype(args)>(args)...);
      },
      std::forward<Tuple>(tuple));
}

}  // namespace gmock_callback_support_internal

namespace test {

// Matchers for base::{Once,Repeating}Callback and
// base::{Once,Repeating}Closure.
MATCHER(IsNullCallback, "a null callback") {
  return (arg.is_null());
}

MATCHER(IsNotNullCallback, "a non-null callback") {
  return (!arg.is_null());
}

// The Run[Once]Closure() action invokes the Run() method on the closure
// provided when the action is constructed. Function arguments passed when the
// action is run will be ignored.
ACTION_P(RunClosure, closure) {
  closure.Run();
}

// This action can be invoked at most once. Any further invocation will trigger
// a CHECK failure.
inline auto RunOnceClosure(OnceClosure cb) {
  // Mock actions need to be copyable, but OnceClosure is not. Wrap the closure
  // in a base::RefCountedData<> to allow it to be copied.
  using RefCountedOnceClosure = RefCountedData<OnceClosure>;
  scoped_refptr<RefCountedOnceClosure> copyable_cb =
      MakeRefCounted<RefCountedOnceClosure>(std::move(cb));
  return [copyable_cb](auto&&...) {
    CHECK(copyable_cb->data);
    std::move(copyable_cb->data).Run();
  };
}

// The Run[Once]Closure<N>() action invokes the Run() method on the N-th
// (0-based) argument of the mock function.
template <size_t I>
auto RunClosure() {
  return [](auto&&... args) -> decltype(auto) {
    return gmock_callback_support_internal::get<I>(args...).Run();
  };
}

template <size_t I>
auto RunOnceClosure() {
  return [](auto&&... args) -> decltype(auto) {
    return std::move(gmock_callback_support_internal::get<I>(args...)).Run();
  };
}

// The Run[Once]Callback<N>(p1, p2, ..., p_k) action invokes the Run() method on
// the N-th (0-based) argument of the mock function, with arguments p1, p2, ...,
// p_k.
//
// Notes:
//
//   1. The arguments are passed by value by default.  If you need to
//   pass an argument by reference, wrap it inside ByRef().  For example,
//
//     RunCallback<1>(5, string("Hello"), ByRef(foo))
//
//   passes 5 and string("Hello") by value, and passes foo by reference.
//
//   2. If the callback takes an argument by reference but ByRef() is
//   not used, it will receive the reference to a copy of the value,
//   instead of the original value.  For example, when the 0-th
//   argument of the callback takes a const string&, the action
//
//     RunCallback<0>(string("Hello"))
//
//   makes a copy of the temporary string("Hello") object and passes a
//   reference of the copy, instead of the original temporary object,
//   to the callback.  This makes it easy for a user to define an
//   RunCallback action from temporary values and have it performed later.
//
//   3. There are two separate APIs for interacting with OnceCallback<> --
//   RunOnceCallback<> and RunOnceCallbackRepeatedly<>. In the former, arguments
//   are copies during each run; in the latter, they are passed by move.
//   Note that RunOnceCallback<> cannot be used with WillRepeatedly() since its
//   arguments are moved out upon first invocation -- the code doing so will
//   crash with a CHECK().
//   Using move-only arguments with `RunCallback()` is not supported.
template <size_t I, typename... RunArgs>
auto RunOnceCallback(RunArgs&&... run_args) {
  // Mock actions have to be copyable. However, since this action is only
  // supposed to be invoked once and might contain move-only arguments, the arg
  // tuple is explicitly wrapped as RefCountedData<> to allow shallow copies.
  return
      [tuple_ptr = gmock_callback_support_internal::WrapTupleAsRefCountedData(
           std::make_tuple(std::forward<RunArgs>(run_args)...))](
          auto&&... args) -> decltype(auto) {
        CHECK(tuple_ptr->data)
            << "A RunOnceCallback() action must be called at most once. "
               "Use RunOnceCallbackRepeatedly() for invoking a "
               "OnceCallback<> more than once.";
        auto data = std::exchange(tuple_ptr->data, nullptr);
        return gmock_callback_support_internal::RunImpl(
            std::move(gmock_callback_support_internal::get<I>(args...)),
            std::move(*data));
      };
}

template <size_t I, typename... RunArgs>
auto RunOnceCallbackRepeatedly(RunArgs&&... run_args) {
  return [tuple = std::make_tuple(std::forward<RunArgs>(run_args)...)](
             auto&&... args) -> decltype(auto) {
    return gmock_callback_support_internal::RunImpl(
        std::move(gmock_callback_support_internal::get<I>(args...)), tuple);
  };
}

template <size_t I, typename... RunArgs>
auto RunCallback(RunArgs&&... run_args) {
  return [tuple = std::make_tuple(std::forward<RunArgs>(run_args)...)](
             auto&&... args) -> decltype(auto) {
    return gmock_callback_support_internal::RunImpl(
        gmock_callback_support_internal::get<I>(args...), tuple);
  };
}

}  // namespace test
}  // namespace base

#endif  // BASE_TEST_GMOCK_CALLBACK_SUPPORT_H_
