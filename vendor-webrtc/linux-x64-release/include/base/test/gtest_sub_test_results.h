// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_GTEST_SUB_TEST_RESULTS_H_
#define BASE_TEST_GTEST_SUB_TEST_RESULTS_H_

#include <optional>
#include <string_view>

#include "testing/gtest/include/gtest/gtest.h"

namespace base {

// Add a SubTestResult in the GTest XML output. This can be used to report
// additional test results within a single GTest.
//
// Arguments:
//
// - `name` may only contain alphanumeric characters or underscore (_).
// - `name` may not be an empty string.
// - `failure_message` has no character limitations.
// - If no `failure_message` is passed, the SubTestResult is considered
//   successful.
//
// Caveats:
//
// - Must be called on the thread where GTest is running the test case.
// - Only works on desktop, which uses the test launcher.
void AddSubTestResult(std::string_view name,
                      testing::TimeInMillis elapsed_time,
                      std::optional<std::string_view> failure_message);

}  // namespace base

#endif  // BASE_TEST_GTEST_SUB_TEST_RESULTS_H_
