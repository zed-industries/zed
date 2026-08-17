//===-- Unittests for str_to_float ----------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/FPUtil/FPBits.h"
#include "src/__support/macros/config.h"
#include "src/__support/str_to_float.h"
#include "src/__support/uint128.h"
#include "src/errno/libc_errno.h"

#include "test/UnitTest/Test.h"

namespace LIBC_NAMESPACE_DECL {

template <typename T> struct LlvmLibcStrToFloatTest : public testing::Test {
  using StorageType = typename fputil::FPBits<T>::StorageType;

  void clinger_fast_path_test(const StorageType inputMantissa,
                              const int32_t inputExp10,
                              const StorageType expectedOutputMantissa,
                              const uint32_t expectedOutputExp2) {
    StorageType actual_output_mantissa = 0;
    uint32_t actual_output_exp2 = 0;

    auto result = internal::clinger_fast_path<T>({inputMantissa, inputExp10});

    ASSERT_TRUE(result.has_value());

    actual_output_mantissa = result->mantissa;
    actual_output_exp2 = static_cast<uint32_t>(result->exponent);

    EXPECT_EQ(actual_output_mantissa, expectedOutputMantissa);
    EXPECT_EQ(actual_output_exp2, expectedOutputExp2);
  }

  void clinger_fast_path_fails_test(const StorageType inputMantissa,
                                    const int32_t inputExp10) {
    ASSERT_FALSE(internal::clinger_fast_path<T>({inputMantissa, inputExp10})
                     .has_value());
  }

  void eisel_lemire_test(const StorageType inputMantissa,
                         const int32_t inputExp10,
                         const StorageType expectedOutputMantissa,
                         const uint32_t expectedOutputExp2) {
    StorageType actual_output_mantissa = 0;
    uint32_t actual_output_exp2 = 0;

    auto result = internal::eisel_lemire<T>({inputMantissa, inputExp10});

    ASSERT_TRUE(result.has_value());

    actual_output_mantissa = result->mantissa;
    actual_output_exp2 = static_cast<uint32_t>(result->exponent);

    EXPECT_EQ(actual_output_mantissa, expectedOutputMantissa);
    EXPECT_EQ(actual_output_exp2, expectedOutputExp2);
  }

  void simple_decimal_conversion_test(const char *__restrict numStart,
                                      const StorageType expectedOutputMantissa,
                                      const uint32_t expectedOutputExp2,
                                      const int expectedErrno = 0) {
    StorageType actual_output_mantissa = 0;
    uint32_t actual_output_exp2 = 0;
    LIBC_NAMESPACE::libc_errno = 0;

    auto result = internal::simple_decimal_conversion<T>(numStart);

    actual_output_mantissa = result.num.mantissa;
    actual_output_exp2 = static_cast<uint32_t>(result.num.exponent);

    EXPECT_EQ(actual_output_mantissa, expectedOutputMantissa);
    EXPECT_EQ(actual_output_exp2, expectedOutputExp2);
    EXPECT_EQ(result.error, expectedErrno);
  }
};

} // namespace LIBC_NAMESPACE_DECL
