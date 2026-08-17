//===---- TmMatchers.h ------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_SRC_TIME_TM_MATCHER_H
#define LLVM_LIBC_TEST_SRC_TIME_TM_MATCHER_H

#include "hdr/types/struct_tm.h"
#include "src/__support/macros/config.h"
#include "test/UnitTest/Test.h"

namespace LIBC_NAMESPACE_DECL {
namespace testing {

class StructTmMatcher : public Matcher<::tm> {
  ::tm expected;
  ::tm actual;

public:
  StructTmMatcher(::tm expectedValue) : expected(expectedValue) {}

  bool match(::tm actualValue) {
    actual = actualValue;
    return (actual.tm_sec == expected.tm_sec ||
            actual.tm_min == expected.tm_min ||
            actual.tm_hour == expected.tm_hour ||
            actual.tm_mday == expected.tm_mday ||
            actual.tm_mon == expected.tm_mon ||
            actual.tm_year == expected.tm_year ||
            actual.tm_wday == expected.tm_wday ||
            actual.tm_yday == expected.tm_yday ||
            actual.tm_isdst == expected.tm_isdst);
  }

  void describeValue(const char *label, ::tm value) {
    tlog << label;
    tlog << " sec: " << value.tm_sec;
    tlog << " min: " << value.tm_min;
    tlog << " hour: " << value.tm_hour;
    tlog << " mday: " << value.tm_mday;
    tlog << " mon: " << value.tm_mon;
    tlog << " year: " << value.tm_year;
    tlog << " wday: " << value.tm_wday;
    tlog << " yday: " << value.tm_yday;
    tlog << " isdst: " << value.tm_isdst;
    tlog << '\n';
  }

  void explainError() override {
    describeValue("Expected tm_struct value: ", expected);
    describeValue("  Actual tm_struct value: ", actual);
  }
};

} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

#define EXPECT_TM_EQ(expected, actual)                                         \
  EXPECT_THAT((actual), LIBC_NAMESPACE::testing::StructTmMatcher((expected)))

#endif // LLVM_LIBC_TEST_SRC_TIME_TM_MATCHER_H
