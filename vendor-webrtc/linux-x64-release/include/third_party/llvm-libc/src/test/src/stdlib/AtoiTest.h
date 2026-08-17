//===-- A template class for testing ato* functions -------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/CPP/limits.h" // INT_MAX, INT_MIN, LLONG_MAX, LLONG_MIN
#include "src/__support/CPP/type_traits.h"
#include "test/UnitTest/Test.h"

using LIBC_NAMESPACE::cpp::is_same_v;

template <typename ReturnT>
struct AtoTest : public LIBC_NAMESPACE::testing::Test {
  using FunctionT = ReturnT (*)(const char *);

  void validNumbers(FunctionT func) {
    const char *zero = "0";
    ASSERT_EQ(func(zero), static_cast<ReturnT>(0));

    const char *ten = "10";
    ASSERT_EQ(func(ten), static_cast<ReturnT>(10));

    const char *negative_hundred = "-100";
    ASSERT_EQ(func(negative_hundred), static_cast<ReturnT>(-100));

    const char *positive_thousand = "+1000";
    ASSERT_EQ(func(positive_thousand), static_cast<ReturnT>(1000));

    const char *spaces_before = "     12345";
    ASSERT_EQ(func(spaces_before), static_cast<ReturnT>(12345));

    const char *tabs_before = "\t\t\t\t67890";
    ASSERT_EQ(func(tabs_before), static_cast<ReturnT>(67890));

    const char *letters_after = "123abc";
    ASSERT_EQ(func(letters_after), static_cast<ReturnT>(123));

    const char *letters_between = "456def789";
    ASSERT_EQ(func(letters_between), static_cast<ReturnT>(456));

    const char *all_together = "\t   110 times 5 = 550";
    ASSERT_EQ(func(all_together), static_cast<ReturnT>(110));

    const char *biggest_int = "2147483647";
    ASSERT_EQ(func(biggest_int), static_cast<ReturnT>(INT_MAX));

    const char *smallest_int = "-2147483648";
    ASSERT_EQ(func(smallest_int), static_cast<ReturnT>(INT_MIN));

    if constexpr (sizeof(ReturnT) >= 8) {
      const char *biggest_long_long = "9223372036854775807";
      ASSERT_EQ(func(biggest_long_long), static_cast<ReturnT>(LLONG_MAX));

      const char *smallest_long_long = "-9223372036854775808";
      ASSERT_EQ(func(smallest_long_long), static_cast<ReturnT>(LLONG_MIN));
    }

    // If this is atoi and the size of int is less than the size of long, then
    // we parse as long and cast to int to match existing behavior. This only
    // matters for cases where the result would be outside of the int range, and
    // those cases are undefined, so we can choose whatever output value we
    // want. In this case we have chosen to cast since that matches existing
    // implementations and makes differential fuzzing easier, but no user should
    // rely on this behavior.
    if constexpr (is_same_v<ReturnT, int> && sizeof(ReturnT) < sizeof(long)) {

      static_assert(sizeof(int) == 4);

      const char *bigger_than_biggest_int = "2147483649";
      ASSERT_EQ(func(bigger_than_biggest_int),
                static_cast<ReturnT>(2147483649));

      const char *smaller_than_smallest_int = "-2147483649";
      ASSERT_EQ(func(smaller_than_smallest_int),
                static_cast<ReturnT>(-2147483649));
    }
  }

  void nonBaseTenWholeNumbers(FunctionT func) {
    const char *hexadecimal = "0x10";
    ASSERT_EQ(func(hexadecimal), static_cast<ReturnT>(0));

    const char *octal = "010";
    ASSERT_EQ(func(octal), static_cast<ReturnT>(10));

    const char *decimal_point = "5.9";
    ASSERT_EQ(func(decimal_point), static_cast<ReturnT>(5));
  }

  void notNumbers(FunctionT func) {
    const char *ten_as_word = "ten";
    ASSERT_EQ(func(ten_as_word), static_cast<ReturnT>(0));

    const char *lots_of_letters =
        "wtragsdhfgjykutjdyfhgnchgmjhkyurktfgjhlu;po7urtdjyfhgklyk";
    ASSERT_EQ(func(lots_of_letters), static_cast<ReturnT>(0));
  }
};

template <typename ReturnType>
AtoTest(ReturnType (*)(const char *)) -> AtoTest<ReturnType>;

#define ATOI_TEST(name, func)                                                  \
  using LlvmLibc##name##Test = AtoTest<decltype(func(""))>;                    \
  TEST_F(LlvmLibc##name##Test, ValidNumbers) { validNumbers(func); }           \
  TEST_F(LlvmLibc##name##Test, NonBaseTenWholeNumbers) {                       \
    nonBaseTenWholeNumbers(func);                                              \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, NotNumbers) { notNumbers(func); }
