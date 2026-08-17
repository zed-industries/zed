//===-- Header selector for libc unittests ----------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_UNITTEST_TEST_H
#define LLVM_LIBC_TEST_UNITTEST_TEST_H

// This macro takes a file name and returns a value implicitly castable to
// a const char*. That const char* is the path to a file with the provided name
// in a directory where the test is allowed to write. By default it writes
// directly to the filename provided, but implementations are allowed to
// redefine it as necessary.
#define libc_make_test_file_path(file_name) (file_name)

// The LIBC_COPT_TEST_USE_* macros can select either of two alternate test
// frameworks:
//  * gtest, the well-known model for them all
//  * zxtest, the gtest workalike subset sometimes used in the Fuchsia build
// The default is to use llvm-libc's own gtest workalike framework.
//
// All the frameworks provide the basic EXPECT_* and ASSERT_* macros that gtest
// does.  The wrapper headers below define LIBC_NAMESPACE::testing::Test as the
// base class for test fixture classes.  Each also provides a definition of the
// macro LIBC_TEST_HAS_MATCHERS() for use in `#if` conditionals to guard use of
// gmock-style matchers, which zxtest does not support.

#if defined(LIBC_COPT_TEST_USE_ZXTEST)
#include "ZxTest.h"
// TODO: Migrate Pigweed to setting LIBC_COPT_TEST_USE_GTEST instead.
#elif defined(LIBC_COPT_TEST_USE_GTEST) || defined(LIBC_COPT_TEST_USE_PIGWEED)
#include "GTest.h"
#else
#include "LibcTest.h"
#endif

// These are defined the same way for each framework, in terms of the macros
// they all provide.

#define ASSERT_ERRNO_EQ(VAL)                                                   \
  do {                                                                         \
    ASSERT_EQ(VAL, static_cast<int>(LIBC_NAMESPACE::libc_errno));              \
    LIBC_NAMESPACE::libc_errno = 0;                                            \
  } while (0)
#define ASSERT_ERRNO_SUCCESS()                                                 \
  ASSERT_EQ(0, static_cast<int>(LIBC_NAMESPACE::libc_errno))
#define ASSERT_ERRNO_FAILURE()                                                 \
  do {                                                                         \
    ASSERT_NE(0, static_cast<int>(LIBC_NAMESPACE::libc_errno));                \
    LIBC_NAMESPACE::libc_errno = 0;                                            \
  } while (0)

#endif // LLVM_LIBC_TEST_UNITTEST_TEST_H
