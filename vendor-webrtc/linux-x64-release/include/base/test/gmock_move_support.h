// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_GMOCK_MOVE_SUPPORT_H_
#define BASE_TEST_GMOCK_MOVE_SUPPORT_H_

#include <cstddef>
#include <tuple>
#include <utility>

// Moves the `I`th argument to `out`. Analogous to `testing::SaveArg()`, which
// copies instead.
//
// Example:
//
//   std::unique_ptr<int> result;
//   EXPECT_CALL(mocked_object, Method).WillOnce(MoveArg<0>(&result));
//   mocked_object.Method(std::make_unique<int>(123));
//   EXPECT_THAT(result, Pointee(Eq(123)));
//
// Important: it is not possible to use multiple `MoveArg()` actions in a single
// `DoAll()`: per the googlemock documentation, all but the last action receive
// a read-only view of the arguments. Allowing an intermediate action to consume
// the arguments would leave the original arguments in an unspecified state for
// invoking subsequent actions, which is dubious.
//
// A simple workaround is to use `Invoke()` with a lambda instead, e.g.
//
//   std::unique_ptr<int> int_result;
//   std::unique_ptr<double> double_result;
//   EXPECT_CALL(mocked_object, Method).WillOnce(Invoke(
//       [&] (auto&& arg1, auto&& arg2) {
//         int_result = std::move(arg1);
//         double_result = std::move(arg2);
//         return 42;
//       }));
//   EXPECT_EQ(42, mocked_object.Method(std::make_unique<int>(123),
//                                      std::make_unique<double>(0.5)));
//   EXPECT_THAT(int_result, Pointee(Eq(123)));
//   EXPECT_THAT(double_result, Pointee(DoubleEq(0.5)));

template <size_t I = 0, typename T>
auto MoveArg(T* out) {
  return [out](auto&&... args) {
    *out = std::move(std::get<I>(std::tie(args...)));
  };
}

// Moves the `I`th argument to `*out` and returns `return_value`.
//
// This is a convenience helper for code that wants to write the following:
//
//   DoAll(MoveArg<N>(&saved_arg), Return(value))
//
// The above is not actually possible to write because:
// - `DoAll()` requires that its final action has a return type that matches the
//   mocked call's return type. So `Return()` must be last.
// - But any actions before the last receive a read-only view of the arguments.
//   So `MoveArg()` receives a read-only view and cannot move out of it.
//
// Example:
//
//   std::unique_ptr<int> result;
//   EXPECT_CALL(mocked_object, Method).WillOnce(
//       MoveArgAndReturn<0>(&result, true));
//   EXPECT_TRUE(mocked_object.Method(std::make_unique<int>(123)));
//   EXPECT_THAT(int_result, Pointee(Eq(123)));
//
template <size_t I = 0, typename T1, typename T2>
auto MoveArgAndReturn(T1* out, T2&& return_value) {
  return [out, value = std::forward<T2>(return_value)](auto&&... args) mutable {
    *out = std::move(std::get<I>(std::tie(args...)));
    return std::forward<T2>(value);
  };
}

#endif  // BASE_TEST_GMOCK_MOVE_SUPPORT_H_
