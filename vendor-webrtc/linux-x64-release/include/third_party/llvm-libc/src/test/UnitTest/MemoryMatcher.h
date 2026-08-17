//===-- MemoryMatcher.h -----------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_UNITTEST_MEMORYMATCHER_H
#define LLVM_LIBC_TEST_UNITTEST_MEMORYMATCHER_H

#include "src/__support/CPP/span.h"

#include "src/__support/macros/config.h"
#include "test/UnitTest/Test.h"

namespace LIBC_NAMESPACE_DECL {
namespace testing {

using MemoryView = LIBC_NAMESPACE::cpp::span<const char>;

} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

#if !LIBC_TEST_HAS_MATCHERS()

#define EXPECT_MEM_EQ(expected, actual)                                        \
  do {                                                                         \
    LIBC_NAMESPACE::testing::MemoryView e = (expected);                        \
    LIBC_NAMESPACE::testing::MemoryView a = (actual);                          \
    ASSERT_EQ(e.size(), a.size());                                             \
    EXPECT_BYTES_EQ(e.data(), a.data(), e.size());                             \
  } while (0)

#define ASSERT_MEM_EQ(expected, actual)                                        \
  do {                                                                         \
    LIBC_NAMESPACE::testing::MemoryView e = (expected);                        \
    LIBC_NAMESPACE::testing::MemoryView a = (actual);                          \
    ASSERT_EQ(e.size(), a.size());                                             \
    ASSERT_BYTES_EQ(e.data(), a.data(), e.size());                             \
  } while (0)

#else // LIBC_TEST_HAS_MATCHERS()

namespace LIBC_NAMESPACE_DECL {
namespace testing {

class MemoryMatcher : public Matcher<MemoryView> {
  MemoryView expected;
  MemoryView actual;
  bool mismatch_size = false;
  size_t mismatch_index = cpp::numeric_limits<size_t>::max();

public:
  MemoryMatcher(MemoryView expectedValue) : expected(expectedValue) {}

  bool match(MemoryView actualValue);

  void explainError() override;
};

} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

#define EXPECT_MEM_EQ(expected, actual)                                        \
  EXPECT_THAT(actual, LIBC_NAMESPACE::testing::MemoryMatcher(expected))
#define ASSERT_MEM_EQ(expected, actual)                                        \
  ASSERT_THAT(actual, LIBC_NAMESPACE::testing::MemoryMatcher(expected))

#endif // !LIBC_TEST_HAS_MATCHERS()

#endif // LLVM_LIBC_TEST_UNITTEST_MEMORYMATCHER_H
