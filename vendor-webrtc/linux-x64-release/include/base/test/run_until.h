// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_RUN_UNTIL_H_
#define BASE_TEST_RUN_UNTIL_H_

#include "base/functional/function_ref.h"

namespace base::test {

// Waits until `condition` evaluates to `true`, by evaluating `condition`
// whenever the current thread becomes idle.
//
// Note: "something" (e.g. a task) must wake the current thread once the
// condition is true. As such testing global conditions which won't wake the
// current thread is flaky.
//
// Returns true if `condition` became true, or false if a timeout happens.
//
// Example usage:
//
//   ChangeColorAsyncTo(object_under_tests, Color::Red);
//
//   // Waits until the color is red, or aborts the tests otherwise.
//   ASSERT_TRUE(RunUntil([&](){
//     return object_under_test.Color() == Color::Red;
//   })) << "Timeout waiting for the color to turn red";
//
//   // When we come here `Color()` is guaranteed to be `Color::Red`.
//
// TODO (crbug.com/376085325): Implement timeout handling for MOCK_TIME to
// manage unmet conditions.
[[nodiscard]] bool RunUntil(base::FunctionRef<bool(void)> condition);

}  // namespace base::test

#endif  // BASE_TEST_RUN_UNTIL_H_
