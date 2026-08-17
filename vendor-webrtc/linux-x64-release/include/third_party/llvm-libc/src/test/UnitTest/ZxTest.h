//===-- Header for using Fuchsia's zxtest framework ------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#ifndef LLVM_LIBC_UTILS_UNITTEST_ZXTEST_H
#define LLVM_LIBC_UTILS_UNITTEST_ZXTEST_H

#include "src/__support/macros/config.h"
#include <zxtest/zxtest.h>

#define WITH_SIGNAL(X) #X

#ifndef EXPECT_DEATH
// Since zxtest has ASSERT_DEATH but not EXPECT_DEATH, wrap calling it
// in a lambda returning void to swallow any early returns so that this
// can be used in a function that itself returns non-void.
#define EXPECT_DEATH(FUNC, SIG) ([&] { ASSERT_DEATH(FUNC, SIG); }())
#endif

namespace LIBC_NAMESPACE_DECL {
namespace testing {

using Test = ::zxtest::Test;

} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

// zxtest does not have gmock-style matchers.
#define LIBC_TEST_HAS_MATCHERS() (0)

#endif // LLVM_LIBC_UTILS_UNITTEST_ZXTEST_H
