//===-- Simple checkers for integrations tests ------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_UTILS_INTEGRATION_TEST_TEST_H
#define LLVM_LIBC_UTILS_INTEGRATION_TEST_TEST_H

#include "src/__support/OSUtil/exit.h"
#include "src/__support/OSUtil/io.h"

#define __AS_STRING(val) #val
#define __CHECK_TRUE(file, line, val, should_exit)                             \
  if (!(val)) {                                                                \
    LIBC_NAMESPACE::write_to_stderr(file ":" __AS_STRING(                      \
        line) ": Expected '" #val "' to be true, but is false\n");             \
    if (should_exit)                                                           \
      LIBC_NAMESPACE::internal::exit(127);                                     \
  }

#define __CHECK_FALSE(file, line, val, should_exit)                            \
  if (val) {                                                                   \
    LIBC_NAMESPACE::write_to_stderr(file ":" __AS_STRING(                      \
        line) ": Expected '" #val "' to be false, but is true\n");             \
    if (should_exit)                                                           \
      LIBC_NAMESPACE::internal::exit(127);                                     \
  }

#define __CHECK_EQ(file, line, val1, val2, should_exit)                        \
  if ((val1) != (val2)) {                                                      \
    LIBC_NAMESPACE::write_to_stderr(file ":" __AS_STRING(                      \
        line) ": Expected '" #val1 "' to be equal to '" #val2 "'\n");          \
    if (should_exit)                                                           \
      LIBC_NAMESPACE::internal::exit(127);                                     \
  }

#define __CHECK_NE(file, line, val1, val2, should_exit)                        \
  if ((val1) == (val2)) {                                                      \
    LIBC_NAMESPACE::write_to_stderr(file ":" __AS_STRING(                      \
        line) ": Expected '" #val1 "' to not be equal to '" #val2 "'\n");      \
    if (should_exit)                                                           \
      LIBC_NAMESPACE::internal::exit(127);                                     \
  }

////////////////////////////////////////////////////////////////////////////////
// Boolean checks are handled as comparison to the true / false values.

#define EXPECT_TRUE(val) __CHECK_TRUE(__FILE__, __LINE__, val, false)
#define ASSERT_TRUE(val) __CHECK_TRUE(__FILE__, __LINE__, val, true)
#define EXPECT_FALSE(val) __CHECK_FALSE(__FILE__, __LINE__, val, false)
#define ASSERT_FALSE(val) __CHECK_FALSE(__FILE__, __LINE__, val, true)

////////////////////////////////////////////////////////////////////////////////
// Binary equality / inequality.

#define EXPECT_EQ(val1, val2)                                                  \
  __CHECK_EQ(__FILE__, __LINE__, (val1), (val2), false)
#define ASSERT_EQ(val1, val2)                                                  \
  __CHECK_EQ(__FILE__, __LINE__, (val1), (val2), true)
#define EXPECT_NE(val1, val2)                                                  \
  __CHECK_NE(__FILE__, __LINE__, (val1), (val2), false)
#define ASSERT_NE(val1, val2)                                                  \
  __CHECK_NE(__FILE__, __LINE__, (val1), (val2), true)

////////////////////////////////////////////////////////////////////////////////
// Errno checks.

#define ASSERT_ERRNO_EQ(VAL)                                                   \
  ASSERT_EQ(VAL, static_cast<int>(LIBC_NAMESPACE::libc_errno))
#define ASSERT_ERRNO_SUCCESS()                                                 \
  ASSERT_EQ(0, static_cast<int>(LIBC_NAMESPACE::libc_errno))
#define ASSERT_ERRNO_FAILURE()                                                 \
  ASSERT_NE(0, static_cast<int>(LIBC_NAMESPACE::libc_errno))

// Integration tests are compiled with -ffreestanding which stops treating
// the main function as a non-overloadable special function. Hence, we use a
// convenience macro which declares it 'extern "C"'.
//
// When we are able to reuse the unit test infrastructure for integration
// tests, then we should not need to explicitly declare/define the main
// function in individual integration tests. We will not need this macro
// then.
#define TEST_MAIN extern "C" int main

#endif // LLVM_LIBC_UTILS_INTEGRATION_TEST_TEST_H
