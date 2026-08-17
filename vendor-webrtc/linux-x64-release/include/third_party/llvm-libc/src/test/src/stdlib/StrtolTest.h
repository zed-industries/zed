//===-- A template class for testing strto* functions -----------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/CPP/limits.h"
#include "src/__support/CPP/type_traits.h"
#include "src/__support/ctype_utils.h"
#include "src/__support/macros/properties/architectures.h"
#include "src/errno/libc_errno.h"
#include "test/UnitTest/Test.h"

#include <stddef.h>

using LIBC_NAMESPACE::cpp::is_signed_v;

template <typename ReturnT>
struct StrtoTest : public LIBC_NAMESPACE::testing::Test {
  using FunctionT = ReturnT (*)(const char *, char **, int);

  static constexpr ReturnT T_MAX =
      LIBC_NAMESPACE::cpp::numeric_limits<ReturnT>::max();
  static constexpr ReturnT T_MIN =
      LIBC_NAMESPACE::cpp::numeric_limits<ReturnT>::min();

  void InvalidBase(FunctionT func) {
    const char *ten = "10";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(ten, nullptr, -1), ReturnT(0));
    ASSERT_ERRNO_EQ(EINVAL);
  }

  void CleanBaseTenDecode(FunctionT func) {
    char *str_end = nullptr;

    // TODO: Look into collapsing these repeated segments.
    const char *ten = "10";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(ten, &str_end, 10), ReturnT(10));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - ten, ptrdiff_t(2));

    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(ten, nullptr, 10), ReturnT(10));
    ASSERT_ERRNO_SUCCESS();

    const char *hundred = "100";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(hundred, &str_end, 10), ReturnT(100));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - hundred, ptrdiff_t(3));

    const char *big_number = "1234567890";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(big_number, &str_end, 10), ReturnT(1234567890));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - big_number, ptrdiff_t(10));

    // This number is larger than 2^32, meaning that if long is only 32 bits
    // wide, strtol will return LONG_MAX.
    const char *bigger_number = "12345678900";
    LIBC_NAMESPACE::libc_errno = 0;
    if constexpr (sizeof(ReturnT) < 8) {
      ASSERT_EQ(func(bigger_number, &str_end, 10), T_MAX);
      ASSERT_ERRNO_EQ(ERANGE);
    } else {
      ASSERT_EQ(func(bigger_number, &str_end, 10), ReturnT(12345678900));
      ASSERT_ERRNO_SUCCESS();
    }
    EXPECT_EQ(str_end - bigger_number, ptrdiff_t(11));

    const char *too_big_number = "123456789012345678901";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(too_big_number, &str_end, 10), T_MAX);
    ASSERT_ERRNO_EQ(ERANGE);
    EXPECT_EQ(str_end - too_big_number, ptrdiff_t(21));

    const char *long_number_range_test =
        "10000000000000000000000000000000000000000000000000";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(long_number_range_test, &str_end, 10), T_MAX);
    ASSERT_ERRNO_EQ(ERANGE);
    EXPECT_EQ(str_end - long_number_range_test, ptrdiff_t(50));

    // For most negative numbers, the unsigned functions treat it the same as
    // casting a negative variable to an unsigned type.
    const char *negative = "-100";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(negative, &str_end, 10), ReturnT(-100));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - negative, ptrdiff_t(4));

    const char *big_negative_number = "-1234567890";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(big_negative_number, &str_end, 10), ReturnT(-1234567890));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - big_negative_number, ptrdiff_t(11));

    const char *too_big_negative_number = "-123456789012345678901";
    LIBC_NAMESPACE::libc_errno = 0;
    // If the number is signed, it should return the smallest negative number
    // for the current type, but if it's unsigned it should max out and return
    // the largest positive number for the current type. From the standard:
    // "If the correct value is outside the range of representable values,
    // LONG_MIN, LONG_MAX, LLONG_MIN, LLONG_MAX, ULONG_MAX, or ULLONG_MAX is
    // returned"
    // Note that 0 is not on that list.
    ASSERT_EQ(func(too_big_negative_number, &str_end, 10),
              (is_signed_v<ReturnT> ? T_MIN : T_MAX));
    ASSERT_ERRNO_EQ(ERANGE);
    EXPECT_EQ(str_end - too_big_negative_number, ptrdiff_t(22));
  }

  void MessyBaseTenDecode(FunctionT func) {
    char *str_end = nullptr;

    const char *spaces_before = "     10";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(spaces_before, &str_end, 10), ReturnT(10));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - spaces_before, ptrdiff_t(7));

    const char *spaces_after = "10      ";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(spaces_after, &str_end, 10), ReturnT(10));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - spaces_after, ptrdiff_t(2));

    const char *word_before = "word10";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(word_before, &str_end, 10), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - word_before, ptrdiff_t(0));

    const char *word_after = "10word";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(word_after, &str_end, 10), ReturnT(10));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - word_after, ptrdiff_t(2));

    const char *two_numbers = "10 999";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(two_numbers, &str_end, 10), ReturnT(10));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - two_numbers, ptrdiff_t(2));

    const char *two_signs = "--10 999";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(two_signs, &str_end, 10), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - two_signs, ptrdiff_t(0));

    const char *sign_before = "+2=4";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(sign_before, &str_end, 10), ReturnT(2));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - sign_before, ptrdiff_t(2));

    const char *sign_after = "2+2=4";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(sign_after, &str_end, 10), ReturnT(2));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - sign_after, ptrdiff_t(1));

    const char *tab_before = "\t10";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(tab_before, &str_end, 10), ReturnT(10));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - tab_before, ptrdiff_t(3));

    const char *all_together = "\t  -12345and+67890";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(all_together, &str_end, 10), ReturnT(-12345));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - all_together, ptrdiff_t(9));

    const char *just_spaces = "  ";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(just_spaces, &str_end, 10), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - just_spaces, ptrdiff_t(0));

    const char *just_space_and_sign = " +";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(just_space_and_sign, &str_end, 10), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - just_space_and_sign, ptrdiff_t(0));
  }

  void DecodeInOtherBases(FunctionT func) {
    // This test is excessively slow on the GPU, so we limit the innermost loop.
#if defined(LIBC_TARGET_ARCH_IS_GPU)
    constexpr int limit = 0;
#else
    constexpr int limit = 36;
#endif
    char small_string[4] = {'\0', '\0', '\0', '\0'};
    for (int base = 2; base <= 36; ++base) {
      for (int first_digit = 0; first_digit <= 36; ++first_digit) {
        small_string[0] = static_cast<char>(
            LIBC_NAMESPACE::internal::int_to_b36_char(first_digit));
        if (first_digit < base) {
          LIBC_NAMESPACE::libc_errno = 0;
          ASSERT_EQ(func(small_string, nullptr, base),
                    static_cast<ReturnT>(first_digit));
          ASSERT_ERRNO_SUCCESS();
        } else {
          LIBC_NAMESPACE::libc_errno = 0;
          ASSERT_EQ(func(small_string, nullptr, base), ReturnT(0));
          ASSERT_ERRNO_SUCCESS();
        }
      }
    }

    for (int base = 2; base <= 36; ++base) {
      for (int first_digit = 0; first_digit <= 36; ++first_digit) {
        small_string[0] = static_cast<char>(
            LIBC_NAMESPACE::internal::int_to_b36_char(first_digit));
        for (int second_digit = 0; second_digit <= 36; ++second_digit) {
          small_string[1] = static_cast<char>(
              LIBC_NAMESPACE::internal::int_to_b36_char(second_digit));
          if (first_digit < base && second_digit < base) {
            LIBC_NAMESPACE::libc_errno = 0;
            ASSERT_EQ(
                func(small_string, nullptr, base),
                static_cast<ReturnT>(second_digit + (first_digit * base)));
            ASSERT_ERRNO_SUCCESS();
          } else if (first_digit < base) {
            LIBC_NAMESPACE::libc_errno = 0;
            ASSERT_EQ(func(small_string, nullptr, base),
                      static_cast<ReturnT>(first_digit));
            ASSERT_ERRNO_SUCCESS();
          } else {
            LIBC_NAMESPACE::libc_errno = 0;
            ASSERT_EQ(func(small_string, nullptr, base), ReturnT(0));
            ASSERT_ERRNO_SUCCESS();
          }
        }
      }
    }

    for (int base = 2; base <= 36; ++base) {
      for (int first_digit = 0; first_digit <= 36; ++first_digit) {
        small_string[0] = static_cast<char>(
            LIBC_NAMESPACE::internal::int_to_b36_char(first_digit));
        for (int second_digit = 0; second_digit <= 36; ++second_digit) {
          small_string[1] = static_cast<char>(
              LIBC_NAMESPACE::internal::int_to_b36_char(second_digit));
          for (int third_digit = 0; third_digit <= limit; ++third_digit) {
            small_string[2] = static_cast<char>(
                LIBC_NAMESPACE::internal::int_to_b36_char(third_digit));

            if (first_digit < base && second_digit < base &&
                third_digit < base) {
              LIBC_NAMESPACE::libc_errno = 0;
              ASSERT_EQ(func(small_string, nullptr, base),
                        static_cast<ReturnT>(third_digit +
                                             (second_digit * base) +
                                             (first_digit * base * base)));
              ASSERT_ERRNO_SUCCESS();
            } else if (first_digit < base && second_digit < base) {
              LIBC_NAMESPACE::libc_errno = 0;
              ASSERT_EQ(
                  func(small_string, nullptr, base),
                  static_cast<ReturnT>(second_digit + (first_digit * base)));
              ASSERT_ERRNO_SUCCESS();
            } else if (first_digit < base) {
              // if the base is 16 there is a special case for the prefix 0X.
              // The number is treated as a one digit hexadecimal.
              if (base == 16 && first_digit == 0 && second_digit == 33) {
                if (third_digit < base) {
                  LIBC_NAMESPACE::libc_errno = 0;
                  ASSERT_EQ(func(small_string, nullptr, base),
                            static_cast<ReturnT>(third_digit));
                  ASSERT_ERRNO_SUCCESS();
                } else {
                  LIBC_NAMESPACE::libc_errno = 0;
                  ASSERT_EQ(func(small_string, nullptr, base), ReturnT(0));
                  ASSERT_ERRNO_SUCCESS();
                }
              } else {
                LIBC_NAMESPACE::libc_errno = 0;
                ASSERT_EQ(func(small_string, nullptr, base),
                          static_cast<ReturnT>(first_digit));
                ASSERT_ERRNO_SUCCESS();
              }
            } else {
              LIBC_NAMESPACE::libc_errno = 0;
              ASSERT_EQ(func(small_string, nullptr, base), ReturnT(0));
              ASSERT_ERRNO_SUCCESS();
            }
          }
        }
      }
    }
  }

  void CleanBaseSixteenDecode(FunctionT func) {
    char *str_end = nullptr;

    const char *no_prefix = "123abc";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(no_prefix, &str_end, 16), ReturnT(0x123abc));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - no_prefix, ptrdiff_t(6));

    const char *yes_prefix = "0x456def";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(yes_prefix, &str_end, 16), ReturnT(0x456def));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - yes_prefix, ptrdiff_t(8));

    const char *letter_after_prefix = "0xabc123";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(letter_after_prefix, &str_end, 16), ReturnT(0xabc123));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - letter_after_prefix, ptrdiff_t(8));

    // These tests check what happens when the number passed is exactly the max
    // value for the conversion.

    // Max size for unsigned 32 bit numbers

    const char *max_32_bit_value = "0xFFFFFFFF";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(max_32_bit_value, &str_end, 0),
              ((is_signed_v<ReturnT> && sizeof(ReturnT) == 4)
                   ? T_MAX
                   : ReturnT(0xFFFFFFFF)));
    ASSERT_ERRNO_EQ(is_signed_v<ReturnT> && sizeof(ReturnT) == 4 ? ERANGE : 0);
    EXPECT_EQ(str_end - max_32_bit_value, ptrdiff_t(10));

    const char *negative_max_32_bit_value = "-0xFFFFFFFF";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(negative_max_32_bit_value, &str_end, 0),
              ((is_signed_v<ReturnT> && sizeof(ReturnT) == 4)
                   ? T_MIN
                   : -ReturnT(0xFFFFFFFF)));
    ASSERT_ERRNO_EQ(is_signed_v<ReturnT> && sizeof(ReturnT) == 4 ? ERANGE : 0);
    EXPECT_EQ(str_end - negative_max_32_bit_value, ptrdiff_t(11));

    // Max size for signed 32 bit numbers

    const char *max_31_bit_value = "0x7FFFFFFF";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(max_31_bit_value, &str_end, 0), ReturnT(0x7FFFFFFF));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - max_31_bit_value, ptrdiff_t(10));

    const char *negative_max_31_bit_value = "-0x7FFFFFFF";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(negative_max_31_bit_value, &str_end, 0),
              -ReturnT(0x7FFFFFFF));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - negative_max_31_bit_value, ptrdiff_t(11));

    // Max size for unsigned 64 bit numbers

    const char *max_64_bit_value = "0xFFFFFFFFFFFFFFFF";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(max_64_bit_value, &str_end, 0),
              (is_signed_v<ReturnT> || sizeof(ReturnT) < 8
                   ? T_MAX
                   : ReturnT(0xFFFFFFFFFFFFFFFF)));
    ASSERT_ERRNO_EQ((is_signed_v<ReturnT> || sizeof(ReturnT) < 8 ? ERANGE : 0));
    EXPECT_EQ(str_end - max_64_bit_value, ptrdiff_t(18));

    // See the end of CleanBase10Decode for an explanation of how this large
    // negative number can end up as T_MAX.
    const char *negative_max_64_bit_value = "-0xFFFFFFFFFFFFFFFF";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(
        func(negative_max_64_bit_value, &str_end, 0),
        (is_signed_v<ReturnT>
             ? T_MIN
             : (sizeof(ReturnT) < 8 ? T_MAX : -ReturnT(0xFFFFFFFFFFFFFFFF))));
    ASSERT_ERRNO_EQ((is_signed_v<ReturnT> || sizeof(ReturnT) < 8 ? ERANGE : 0));
    EXPECT_EQ(str_end - negative_max_64_bit_value, ptrdiff_t(19));

    // Max size for signed 64 bit numbers

    const char *max_63_bit_value = "0x7FFFFFFFFFFFFFFF";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(max_63_bit_value, &str_end, 0),
              (sizeof(ReturnT) < 8 ? T_MAX : ReturnT(0x7FFFFFFFFFFFFFFF)));
    ASSERT_ERRNO_EQ(sizeof(ReturnT) < 8 ? ERANGE : 0);
    EXPECT_EQ(str_end - max_63_bit_value, ptrdiff_t(18));

    const char *negative_max_63_bit_value = "-0x7FFFFFFFFFFFFFFF";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(negative_max_63_bit_value, &str_end, 0),
              (sizeof(ReturnT) >= 8 ? -ReturnT(0x7FFFFFFFFFFFFFFF)
                                    : (is_signed_v<ReturnT> ? T_MIN : T_MAX)));
    ASSERT_ERRNO_EQ(sizeof(ReturnT) < 8 ? ERANGE : 0);
    EXPECT_EQ(str_end - negative_max_63_bit_value, ptrdiff_t(19));
  }

  void MessyBaseSixteenDecode(FunctionT func) {
    char *str_end = nullptr;

    const char *just_prefix = "0x";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(just_prefix, &str_end, 16), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - just_prefix, ptrdiff_t(1));

    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(just_prefix, &str_end, 0), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - just_prefix, ptrdiff_t(1));

    const char *prefix_with_x_after = "0xx";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(prefix_with_x_after, &str_end, 16), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - prefix_with_x_after, ptrdiff_t(1));

    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(prefix_with_x_after, &str_end, 0), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - prefix_with_x_after, ptrdiff_t(1));
  }

  void AutomaticBaseSelection(FunctionT func) {
    char *str_end = nullptr;

    const char *base_ten = "12345";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(base_ten, &str_end, 0), ReturnT(12345));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - base_ten, ptrdiff_t(5));

    const char *base_sixteen_no_prefix = "123abc";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(base_sixteen_no_prefix, &str_end, 0), ReturnT(123));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - base_sixteen_no_prefix, ptrdiff_t(3));

    const char *base_sixteen_with_prefix = "0x456def";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(base_sixteen_with_prefix, &str_end, 0), ReturnT(0x456def));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - base_sixteen_with_prefix, ptrdiff_t(8));

    const char *base_eight_with_prefix = "012345";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(base_eight_with_prefix, &str_end, 0), ReturnT(012345));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - base_eight_with_prefix, ptrdiff_t(6));

    const char *just_zero = "0";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(just_zero, &str_end, 0), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - just_zero, ptrdiff_t(1));

    const char *just_zero_x = "0x";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(just_zero_x, &str_end, 0), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - just_zero_x, ptrdiff_t(1));

    const char *just_zero_eight = "08";
    LIBC_NAMESPACE::libc_errno = 0;
    ASSERT_EQ(func(just_zero_eight, &str_end, 0), ReturnT(0));
    ASSERT_ERRNO_SUCCESS();
    EXPECT_EQ(str_end - just_zero_eight, ptrdiff_t(1));
  }
};

template <typename ReturnType>
StrtoTest(ReturnType (*)(const char *)) -> StrtoTest<ReturnType>;

#define STRTOL_TEST(name, func)                                                \
  using LlvmLibc##name##Test = StrtoTest<decltype(func("", nullptr, 0))>;      \
  TEST_F(LlvmLibc##name##Test, InvalidBase) { InvalidBase(func); }             \
  TEST_F(LlvmLibc##name##Test, CleanBaseTenDecode) {                           \
    CleanBaseTenDecode(func);                                                  \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, MessyBaseTenDecode) {                           \
    MessyBaseTenDecode(func);                                                  \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, DecodeInOtherBases) {                           \
    DecodeInOtherBases(func);                                                  \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, CleanBaseSixteenDecode) {                       \
    CleanBaseSixteenDecode(func);                                              \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, MessyBaseSixteenDecode) {                       \
    MessyBaseSixteenDecode(func);                                              \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, AutomaticBaseSelection) {                       \
    AutomaticBaseSelection(func);                                              \
  }
