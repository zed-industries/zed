/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef TENSORFLOW_LITE_SUPPORT_CC_PORT_DEFAULT_STATUS_MATCHERS_H_
#define TENSORFLOW_LITE_SUPPORT_CC_PORT_DEFAULT_STATUS_MATCHERS_H_

#include "gmock/gmock.h"
#include "gtest/gtest.h"

#define SUPPORT_STATUS_MACROS_IMPL_CONCAT_INNER_(x, y) x##y
#define SUPPORT_STATUS_MACROS_IMPL_CONCAT_(x, y) \
  SUPPORT_STATUS_MACROS_IMPL_CONCAT_INNER_(x, y)

#undef SUPPORT_ASSERT_OK
#define SUPPORT_ASSERT_OK(expr) \
  SUPPORT_ASSERT_OK_IMPL_(      \
      SUPPORT_STATUS_MACROS_IMPL_CONCAT_(_status, __LINE__), expr)

#define SUPPORT_ASSERT_OK_IMPL_(status, expr) \
  auto status = (expr);                       \
  ASSERT_TRUE(status.ok());

#undef SUPPORT_EXPECT_OK
#define SUPPORT_EXPECT_OK(expr) \
  SUPPORT_EXPECT_OK_IMPL_(      \
      SUPPORT_STATUS_MACROS_IMPL_CONCAT_(_status, __LINE__), expr)

#define SUPPORT_EXPECT_OK_IMPL_(status, expr) \
  auto status = (expr);                       \
  EXPECT_TRUE(status.ok());

#undef SUPPORT_ASSERT_OK_AND_ASSIGN
#define SUPPORT_ASSERT_OK_AND_ASSIGN(lhs, rexpr)                           \
  SUPPORT_ASSERT_OK_AND_ASSIGN_IMPL_(                                      \
      SUPPORT_STATUS_MACROS_IMPL_CONCAT_(_status_or_value, __LINE__), lhs, \
      rexpr)

#define SUPPORT_ASSERT_OK_AND_ASSIGN_IMPL_(statusor, lhs, rexpr) \
  auto statusor = (rexpr);                                       \
  ASSERT_TRUE(statusor.ok());                                    \
  lhs = std::move(statusor.value())

#endif  // TENSORFLOW_LITE_SUPPORT_CC_PORT_DEFAULT_STATUS_MATCHERS_H_
