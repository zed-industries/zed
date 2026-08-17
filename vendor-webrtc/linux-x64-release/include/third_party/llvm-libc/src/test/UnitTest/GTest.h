//===-- Header for using the gtest framework -------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#ifndef LLVM_LIBC_UTILS_UNITTEST_GTEST_H
#define LLVM_LIBC_UTILS_UNITTEST_GTEST_H

#include "src/__support/macros/config.h"
#include <gtest/gtest.h>

namespace LIBC_NAMESPACE_DECL {
namespace testing {

using ::testing::Matcher;
using ::testing::Test;

} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

#define LIBC_TEST_HAS_MATCHERS() (1)

#endif // LLVM_LIBC_UTILS_UNITTEST_GTEST_H
