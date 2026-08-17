//===-- ErrnoCheckingTest.h ------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_UNITTEST_ERRNOCHECKINGTEST_H
#define LLVM_LIBC_TEST_UNITTEST_ERRNOCHECKINGTEST_H

#include "src/__support/macros/config.h"
#include "src/errno/libc_errno.h"
#include "test/UnitTest/Test.h"

namespace LIBC_NAMESPACE_DECL {
namespace testing {

// Provides a test fixture for tests that validate modifications of the errno.
// It clears out the errno at the beginning of the test (e.g. in case it
// contained the value pre-set by the system), and confirms it's still zero
// at the end of the test, forcing the test author to explicitly account for all
// non-zero values.
class ErrnoCheckingTest : public Test {
public:
  void SetUp() override {
    Test::SetUp();
    LIBC_NAMESPACE::libc_errno = 0;
  }

  void TearDown() override {
    ASSERT_ERRNO_SUCCESS();
    Test::TearDown();
  }
};

} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_TEST_UNITTEST_ERRNOCHECKINGTEST_H
