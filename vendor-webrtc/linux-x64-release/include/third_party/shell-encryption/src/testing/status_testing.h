/*
 * Copyright 2020 Google LLC
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

#ifndef RLWE_TESTING_STATUS_TESTING_H_
#define RLWE_TESTING_STATUS_TESTING_H_

#include "status_macros.h"

#undef ASSERT_OK
#define ASSERT_OK(expr) \
  RLWE_ASSERT_OK_IMPL_(RLWE_STATUS_MACROS_IMPL_CONCAT_(_status, __LINE__), expr)

#define RLWE_ASSERT_OK_IMPL_(status, expr) \
  auto status = (expr);                    \
  ASSERT_THAT(status.ok(), ::testing::Eq(true));

#undef EXPECT_OK
#define EXPECT_OK(expr) \
  RLWE_EXPECT_OK_IMPL_(RLWE_STATUS_MACROS_IMPL_CONCAT_(_status, __LINE__), expr)

#define RLWE_EXPECT_OK_IMPL_(status, expr) \
  auto status = (expr);                    \
  EXPECT_THAT(status.ok(), ::testing::Eq(true));

#undef ASSERT_OK_AND_ASSIGN
#define ASSERT_OK_AND_ASSIGN(lhs, rexpr) \
  RLWE_ASSERT_OK_AND_ASSIGN_IMPL_(       \
      RLWE_STATUS_MACROS_IMPL_CONCAT_(_status_or_value, __LINE__), lhs, rexpr)

#define RLWE_ASSERT_OK_AND_ASSIGN_IMPL_(statusor, lhs, rexpr) \
  auto statusor = (rexpr);                                    \
  ASSERT_THAT(statusor.ok(), ::testing::Eq(true));            \
  lhs = std::move(statusor).value()

#endif  // RLWE_TESTING_STATUS_TESTING_H_
