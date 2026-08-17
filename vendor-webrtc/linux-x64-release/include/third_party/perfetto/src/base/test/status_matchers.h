/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_BASE_TEST_STATUS_MATCHERS_H_
#define SRC_BASE_TEST_STATUS_MATCHERS_H_

#include <ostream>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "test/gtest_and_gmock.h"

namespace perfetto::base {
namespace gtest_matchers {

// Returns a gMock matcher that matches a Status or StatusOr<> which is OK.
MATCHER(IsOk, negation ? "is not OK" : "is OK") {
  return arg.ok();
}

// Returns a gMock matcher that matches a Status or StatusOr<> which is an
// error.
MATCHER(IsError, negation ? "is not error" : "is error") {
  return !arg.ok();
}

// Macros for testing the results of function returning base::Status*.
#define PERFETTO_TEST_STATUS_MATCHER_CONCAT(x, y) x##y
#define PERFETTO_TEST_STATUS_MATCHER_CONCAT2(x, y) \
  PERFETTO_TEST_STATUS_MATCHER_CONCAT(x, y)

// Macros for testing the results of functions that return base::Status or
// base::StatusOr<T> (for any type T).
#define EXPECT_OK(expression) \
  EXPECT_THAT(expression, ::perfetto::base::gtest_matchers::IsOk())
#define ASSERT_OK(expression) \
  ASSERT_THAT(expression, ::perfetto::base::gtest_matchers::IsOk())

#define ASSERT_OK_AND_ASSIGN(lhs, rhs)                                      \
  PERFETTO_TEST_STATUS_MATCHER_CONCAT2(auto status_or, __LINE__) = rhs;     \
  lhs = PERFETTO_TEST_STATUS_MATCHER_CONCAT2(status_or, __LINE__).ok()      \
            ? std::move(                                                    \
                  PERFETTO_TEST_STATUS_MATCHER_CONCAT2(status_or, __LINE__) \
                      .value())                                             \
            : decltype(rhs)::value_type{};                                  \
  ASSERT_OK(PERFETTO_TEST_STATUS_MATCHER_CONCAT2(status_or, __LINE__).status())

}  // namespace gtest_matchers

// Add a |PrintTo| function to allow easily determining what the cause of the
// failure is.
inline void PrintTo(const Status& status, std::ostream* os) {
  if (status.ok()) {
    *os << "OK";
  } else {
    *os << "Error(message=" << status.message() << ")";
  }
}
template <typename T>
inline void PrintTo(const StatusOr<T>& status, std::ostream* os) {
  if (status.ok()) {
    *os << testing::PrintToString(status.value());
  } else {
    *os << "Error(message=" << status.status().message() << ")";
  }
}

}  // namespace perfetto::base

#endif  // SRC_BASE_TEST_STATUS_MATCHERS_H_
