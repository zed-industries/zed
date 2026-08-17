//===-- A template class for testing strfrom functions ----------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "src/__support/CPP/type_traits.h"
#include "src/__support/FPUtil/FPBits.h"
#include "test/UnitTest/Test.h"

#define ASSERT_STREQ_LEN(actual_written, actual_str, expected_str)             \
  EXPECT_EQ(actual_written, static_cast<int>(sizeof(expected_str) - 1));       \
  EXPECT_STREQ(actual_str, expected_str);

template <typename InputT>
class StrfromTest : public LIBC_NAMESPACE::testing::Test {

  static constexpr bool is_single_prec =
      LIBC_NAMESPACE::cpp::is_same<InputT, float>::value;
  static constexpr bool is_double_prec =
      LIBC_NAMESPACE::cpp::is_same<InputT, double>::value;

  using FunctionT = int (*)(char *, size_t, const char *, InputT fp);

public:
  void floatDecimalFormat(FunctionT func) {
    if constexpr (is_single_prec)
      floatDecimalSinglePrec(func);
    else if constexpr (is_double_prec)
      floatDecimalDoublePrec(func);
    else
      floatDecimalLongDoublePrec(func);
  }

  void floatHexExpFormat(FunctionT func) {
    if constexpr (is_single_prec)
      floatHexExpSinglePrec(func);
    else if constexpr (is_double_prec)
      floatHexExpDoublePrec(func);
    else
      floatHexExpLongDoublePrec(func);
  }

  void floatDecimalExpFormat(FunctionT func) {
    if constexpr (is_single_prec)
      floatDecimalExpSinglePrec(func);
    else if constexpr (is_double_prec)
      floatDecimalExpDoublePrec(func);
    else
      floatDecimalExpLongDoublePrec(func);
  }

  void floatDecimalAutoFormat(FunctionT func) {
    if constexpr (is_single_prec)
      floatDecimalAutoSinglePrec(func);
    else if constexpr (is_double_prec)
      floatDecimalAutoDoublePrec(func);
    else
      floatDecimalAutoLongDoublePrec(func);
  }

  void improperFormatString(FunctionT func) {
    char buff[100];
    int written;
    const bool is_long_double = !is_single_prec && !is_double_prec;

    written = func(buff, 37, "A simple string with no conversions.", 1.0);
    ASSERT_STREQ_LEN(written, buff, "A simple string with no conversions.");

    written =
        func(buff, 37,
             "%A simple string with one conversion, should overwrite.", 1.0);
    if (is_long_double) {
#if defined(LIBC_TYPES_LONG_DOUBLE_IS_X86_FLOAT80)
      ASSERT_STREQ_LEN(written, buff, "0X8P-3");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT64)
      ASSERT_STREQ_LEN(written, buff, "0X1P+0");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT128)
      ASSERT_STREQ_LEN(written, buff, "0X1P+0");
#endif
    } else {
      // not long double
      ASSERT_STREQ_LEN(written, buff, "0X1P+0");
    }
    written = func(buff, 74,
                   "A simple string with one conversion in %A "
                   "between, writes string as it is",
                   1.0);
    ASSERT_STREQ_LEN(written, buff,
                     "A simple string with one conversion in %A between, "
                     "writes string as it is");

    written = func(buff, 36, "A simple string with one conversion", 1.0);
    ASSERT_STREQ_LEN(written, buff, "A simple string with one conversion");

    written = func(buff, 20, "%1f", static_cast<InputT>(1234567890.0));
    ASSERT_STREQ_LEN(written, buff, "%1f");
  }

  void insufficentBufsize(FunctionT func) {
    char buff[20];
    int written;

    written = func(buff, 5, "%f", static_cast<InputT>(1234567890.0));
    EXPECT_EQ(written, 17);
    ASSERT_STREQ(buff, "1234");

    written = func(buff, 5, "%.5f", static_cast<InputT>(1.05));
    EXPECT_EQ(written, 7);
    ASSERT_STREQ(buff, "1.05");

    written = func(buff, 0, "%g", static_cast<InputT>(1.0));
    EXPECT_EQ(written, 1);
    ASSERT_STREQ(buff, "1.05"); // Make sure that buff has not changed
  }

  void infNanValues(FunctionT func) {
    if constexpr (is_double_prec)
      doublePrecInfNan(func);
    else if constexpr (!is_single_prec)
      longDoublePrecInfNan(func);
  }

  void floatDecimalSinglePrec(FunctionT func) {
    char buff[70];
    int written;

    written = func(buff, 16, "%f", 1.0f);
    ASSERT_STREQ_LEN(written, buff, "1.000000");

    written = func(buff, 20, "%f", 1234567890.0f);
    ASSERT_STREQ_LEN(written, buff, "1234567936.000000");

    written = func(buff, 67, "%.3f", 1.0f);
    ASSERT_STREQ_LEN(written, buff, "1.000");
  }

  void floatDecimalDoublePrec(FunctionT func) {
    char buff[500];
    int written;

    written = func(buff, 99, "%f", 1.0);
    ASSERT_STREQ_LEN(written, buff, "1.000000");

    written = func(buff, 99, "%F", -1.0);
    ASSERT_STREQ_LEN(written, buff, "-1.000000");

    written = func(buff, 99, "%f", -1.234567);
    ASSERT_STREQ_LEN(written, buff, "-1.234567");

    written = func(buff, 99, "%f", 0.0);
    ASSERT_STREQ_LEN(written, buff, "0.000000");

    written = func(buff, 99, "%f", 1.5);
    ASSERT_STREQ_LEN(written, buff, "1.500000");

// Dyadic float is only accurate to ~50 digits, so skip this 300 digit test.
// TODO: Create way to test just the first ~50 digits of a number.
#ifndef LIBC_COPT_FLOAT_TO_STR_REDUCED_PRECISION
    written = func(buff, 499, "%f", 1e300);
    ASSERT_STREQ_LEN(written, buff,
                     "100000000000000005250476025520442024870446858110815915491"
                     "585411551180245"
                     "798890819578637137508044786404370444383288387817694252323"
                     "536043057564479"
                     "218478670698284838720092657580373783023379478809005936895"
                     "323497079994508"
                     "111903896764088007465274278014249457925878882005684283811"
                     "566947219638686"
                     "5459400540160.000000");
#endif // DLIBC_COPT_FLOAT_TO_STR_REDUCED_PRECISION

    written = func(buff, 99, "%f", 0.1);
    ASSERT_STREQ_LEN(written, buff, "0.100000");

    written = func(buff, 99, "%f", 1234567890123456789.0);
    ASSERT_STREQ_LEN(written, buff, "1234567890123456768.000000");

    written = func(buff, 99, "%f", 9999999999999.99);
    ASSERT_STREQ_LEN(written, buff, "9999999999999.990234");

    written = func(buff, 99, "%f", 0.1);
    ASSERT_STREQ_LEN(written, buff, "0.100000");

    written = func(buff, 99, "%f", 1234567890123456789.0);
    ASSERT_STREQ_LEN(written, buff, "1234567890123456768.000000");

    written = func(buff, 99, "%f", 9999999999999.99);
    ASSERT_STREQ_LEN(written, buff, "9999999999999.990234");

    // Precision Tests
    written = func(buff, 100, "%.2f", 9999999999999.99);
    ASSERT_STREQ_LEN(written, buff, "9999999999999.99");

    written = func(buff, 100, "%.1f", 9999999999999.99);
    ASSERT_STREQ_LEN(written, buff, "10000000000000.0");

    written = func(buff, 100, "%.5f", 1.25);
    ASSERT_STREQ_LEN(written, buff, "1.25000");

    written = func(buff, 100, "%.0f", 1.25);
    ASSERT_STREQ_LEN(written, buff, "1");

    written = func(buff, 100, "%.20f", 1.234e-10);
    ASSERT_STREQ_LEN(written, buff, "0.00000000012340000000");
  }

  void floatDecimalLongDoublePrec(FunctionT func) {
    char buff[45];
    int written;

    written = func(buff, 40, "%f", 1.0L);
    ASSERT_STREQ_LEN(written, buff, "1.000000");

    written = func(buff, 10, "%.f", -2.5L);
    ASSERT_STREQ_LEN(written, buff, "-2");
  }

  void floatHexExpSinglePrec(FunctionT func) {
    char buff[25];
    int written;

    written = func(buff, 0, "%a", 1234567890.0f);
    EXPECT_EQ(written, 14);

    written = func(buff, 20, "%a", 1234567890.0f);
    EXPECT_EQ(written, 14);
    ASSERT_STREQ(buff, "0x1.26580cp+30");

    written = func(buff, 20, "%A", 1234567890.0f);
    EXPECT_EQ(written, 14);
    ASSERT_STREQ(buff, "0X1.26580CP+30");
  }

  void floatHexExpDoublePrec(FunctionT func) {
    char buff[60];
    int written;

    written = func(buff, 10, "%a", 1.0);
    ASSERT_STREQ_LEN(written, buff, "0x1p+0");

    written = func(buff, 10, "%A", -1.0);
    ASSERT_STREQ_LEN(written, buff, "-0X1P+0");

    written = func(buff, 30, "%a", -0x1.abcdef12345p0);
    ASSERT_STREQ_LEN(written, buff, "-0x1.abcdef12345p+0");

    written = func(buff, 50, "%A", 0x1.abcdef12345p0);
    ASSERT_STREQ_LEN(written, buff, "0X1.ABCDEF12345P+0");

    written = func(buff, 10, "%a", 0.0);
    ASSERT_STREQ_LEN(written, buff, "0x0p+0");

    written = func(buff, 40, "%a", 1.0e100);
    ASSERT_STREQ_LEN(written, buff, "0x1.249ad2594c37dp+332");

    written = func(buff, 30, "%a", 0.1);
    ASSERT_STREQ_LEN(written, buff, "0x1.999999999999ap-4");
  }

  void floatHexExpLongDoublePrec(FunctionT func) {
    char buff[55];
    int written;

    written = func(buff, 50, "%a", 0.1L);
#if defined(LIBC_TYPES_LONG_DOUBLE_IS_X86_FLOAT80)
    ASSERT_STREQ_LEN(written, buff, "0xc.ccccccccccccccdp-7");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT64)
    ASSERT_STREQ_LEN(written, buff, "0x1.999999999999ap-4");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT128)
    ASSERT_STREQ_LEN(written, buff, "0x1.999999999999999999999999999ap-4");
#endif

    written = func(buff, 20, "%.1a", 0.1L);
#if defined(LIBC_TYPES_LONG_DOUBLE_IS_X86_FLOAT80)
    ASSERT_STREQ_LEN(written, buff, "0xc.dp-7");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT64)
    ASSERT_STREQ_LEN(written, buff, "0x1.ap-4");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT128)
    ASSERT_STREQ_LEN(written, buff, "0x1.ap-4");
#endif

    written = func(buff, 50, "%a", 1.0e1000L);
#if defined(LIBC_TYPES_LONG_DOUBLE_IS_X86_FLOAT80)
    ASSERT_STREQ_LEN(written, buff, "0xf.38db1f9dd3dac05p+3318");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT64)
    ASSERT_STREQ_LEN(written, buff, "inf");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT128)
    ASSERT_STREQ_LEN(written, buff, "0x1.e71b63f3ba7b580af1a52d2a7379p+3321");
#endif

    written = func(buff, 50, "%a", 1.0e-1000L);
#if defined(LIBC_TYPES_LONG_DOUBLE_IS_X86_FLOAT80)
    ASSERT_STREQ_LEN(written, buff, "0x8.68a9188a89e1467p-3325");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT64)
    ASSERT_STREQ_LEN(written, buff, "0x0p+0");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT128)
    ASSERT_STREQ_LEN(written, buff, "0x1.0d152311513c28ce202627c06ec2p-3322");
#endif

    written = func(buff, 50, "%.1a", 0xf.fffffffffffffffp16380L);
#if defined(LIBC_TYPES_LONG_DOUBLE_IS_X86_FLOAT80)
    ASSERT_STREQ_LEN(written, buff, "0x1.0p+16384");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT64)
    ASSERT_STREQ_LEN(written, buff, "inf");
#elif defined(LIBC_TYPES_LONG_DOUBLE_IS_FLOAT128)
    ASSERT_STREQ_LEN(written, buff, "0x2.0p+16383");
#endif
  }

  void floatDecimalExpSinglePrec(FunctionT func) {
    char buff[25];
    int written;

    written = func(buff, 20, "%.9e", 1234567890.0f);
    ASSERT_STREQ_LEN(written, buff, "1.234567936e+09");

    written = func(buff, 20, "%.9E", 1234567890.0f);
    ASSERT_STREQ_LEN(written, buff, "1.234567936E+09");
  }

  void floatDecimalExpDoublePrec(FunctionT func) {
    char buff[101];
    int written;

    written = func(buff, 100, "%e", 1.0);
    ASSERT_STREQ_LEN(written, buff, "1.000000e+00");

    written = func(buff, 100, "%E", -1.0);
    ASSERT_STREQ_LEN(written, buff, "-1.000000E+00");

    written = func(buff, 100, "%e", -1.234567);
    ASSERT_STREQ_LEN(written, buff, "-1.234567e+00");

    written = func(buff, 100, "%e", 0.0);
    ASSERT_STREQ_LEN(written, buff, "0.000000e+00");

    written = func(buff, 100, "%e", 1.5);
    ASSERT_STREQ_LEN(written, buff, "1.500000e+00");

    written = func(buff, 100, "%e", 1e300);
    ASSERT_STREQ_LEN(written, buff, "1.000000e+300");

    written = func(buff, 100, "%e", 1234567890123456789.0);
    ASSERT_STREQ_LEN(written, buff, "1.234568e+18");

    // Precision Tests
    written = func(buff, 100, "%.1e", 1.0);
    ASSERT_STREQ_LEN(written, buff, "1.0e+00");

    written = func(buff, 100, "%.1e", 1.99);
    ASSERT_STREQ_LEN(written, buff, "2.0e+00");

    written = func(buff, 100, "%.1e", 9.99);
    ASSERT_STREQ_LEN(written, buff, "1.0e+01");
  }

  void floatDecimalExpLongDoublePrec(FunctionT func) {
    // Mark as maybe_unused to silence unused variable
    // warning when long double is not 80-bit
    [[maybe_unused]] char buff[100];
    [[maybe_unused]] int written;

#if defined(LIBC_TYPES_LONG_DOUBLE_IS_X86_FLOAT80)
    written = func(buff, 90, "%.9e", 1000000000500000000.1L);
    ASSERT_STREQ_LEN(written, buff, "1.000000001e+18");

    written = func(buff, 90, "%.9e", 1000000000500000000.0L);
    ASSERT_STREQ_LEN(written, buff, "1.000000000e+18");

    written = func(buff, 90, "%e", 0xf.fffffffffffffffp+16380L);
    ASSERT_STREQ_LEN(written, buff, "1.189731e+4932");
#endif // LIBC_TYPES_LONG_DOUBLE_IS_X86_FLOAT80
  }

  void floatDecimalAutoSinglePrec(FunctionT func) {
    char buff[25];
    int written;

    written = func(buff, 20, "%.9g", 1234567890.0f);
    ASSERT_STREQ_LEN(written, buff, "1.23456794e+09");

    written = func(buff, 20, "%.9G", 1234567890.0f);
    ASSERT_STREQ_LEN(written, buff, "1.23456794E+09");
  }

  void floatDecimalAutoDoublePrec(FunctionT func) {
    char buff[120];
    int written;

    written = func(buff, 100, "%g", 1234567890123456789.0);
    ASSERT_STREQ_LEN(written, buff, "1.23457e+18");

    written = func(buff, 100, "%g", 9999990000000.00);
    ASSERT_STREQ_LEN(written, buff, "9.99999e+12");

    written = func(buff, 100, "%g", 9999999000000.00);
    ASSERT_STREQ_LEN(written, buff, "1e+13");

    written = func(buff, 100, "%g", 0xa.aaaaaaaaaaaaaabp-7);
    ASSERT_STREQ_LEN(written, buff, "0.0833333");

    written = func(buff, 100, "%g", 0.00001);
    ASSERT_STREQ_LEN(written, buff, "1e-05");

    // Precision Tests
    written = func(buff, 100, "%.0g", 0.0);
    ASSERT_STREQ_LEN(written, buff, "0");

    written = func(buff, 100, "%.2g", 0.1);
    ASSERT_STREQ_LEN(written, buff, "0.1");

    written = func(buff, 100, "%.2g", 1.09);
    ASSERT_STREQ_LEN(written, buff, "1.1");

    written = func(buff, 100, "%.15g", 22.25);
    ASSERT_STREQ_LEN(written, buff, "22.25");

    written = func(buff, 100, "%.20g", 1.234e-10);
    ASSERT_STREQ_LEN(written, buff, "1.2340000000000000814e-10");
  }

  void floatDecimalAutoLongDoublePrec(FunctionT func) {
    // Mark as maybe_unused to silence unused variable
    // warning when long double is not 80-bit
    [[maybe_unused]] char buff[100];
    [[maybe_unused]] int written;

#if defined(LIBC_TYPES_LONG_DOUBLE_IS_X86_FLOAT80)
    written = func(buff, 99, "%g", 0xf.fffffffffffffffp+16380L);
    ASSERT_STREQ_LEN(written, buff, "1.18973e+4932");

    written = func(buff, 99, "%g", 0xa.aaaaaaaaaaaaaabp-7L);
    ASSERT_STREQ_LEN(written, buff, "0.0833333");

    written = func(buff, 99, "%g", 9.99999999999e-100L);
    ASSERT_STREQ_LEN(written, buff, "1e-99");
#endif // LIBC_TYPES_LONG_DOUBLE_IS_X86_FLOAT80
  }

  void doublePrecInfNan(FunctionT func) {
    char buff[15];
    int written;

    double inf = LIBC_NAMESPACE::fputil::FPBits<double>::inf().get_val();
    double nan = LIBC_NAMESPACE::fputil::FPBits<double>::quiet_nan().get_val();

    written = func(buff, 10, "%f", inf);
    ASSERT_STREQ_LEN(written, buff, "inf");

    written = func(buff, 10, "%A", -inf);
    ASSERT_STREQ_LEN(written, buff, "-INF");

    written = func(buff, 10, "%f", nan);
    ASSERT_STREQ_LEN(written, buff, "nan");

    written = func(buff, 10, "%A", -nan);
    ASSERT_STREQ_LEN(written, buff, "-NAN");
  }

  void longDoublePrecInfNan(FunctionT func) {
    char buff[15];
    int written;

    long double ld_inf =
        LIBC_NAMESPACE::fputil::FPBits<long double>::inf().get_val();
    long double ld_nan =
        LIBC_NAMESPACE::fputil::FPBits<long double>::quiet_nan().get_val();

    written = func(buff, 10, "%f", ld_inf);
    ASSERT_STREQ_LEN(written, buff, "inf");

    written = func(buff, 10, "%A", -ld_inf);
    ASSERT_STREQ_LEN(written, buff, "-INF");

    written = func(buff, 10, "%f", ld_nan);
    ASSERT_STREQ_LEN(written, buff, "nan");

    written = func(buff, 10, "%A", -ld_nan);
    ASSERT_STREQ_LEN(written, buff, "-NAN");
  }
};

#define STRFROM_TEST(InputType, name, func)                                    \
  using LlvmLibc##name##Test = StrfromTest<InputType>;                         \
  TEST_F(LlvmLibc##name##Test, FloatDecimalFormat) {                           \
    floatDecimalFormat(func);                                                  \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, FloatHexExpFormat) { floatHexExpFormat(func); } \
  TEST_F(LlvmLibc##name##Test, FloatDecimalAutoFormat) {                       \
    floatDecimalAutoFormat(func);                                              \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, FloatDecimalExpFormat) {                        \
    floatDecimalExpFormat(func);                                               \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, ImproperFormatString) {                         \
    improperFormatString(func);                                                \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, InsufficientBufferSize) {                       \
    insufficentBufsize(func);                                                  \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, InfAndNanValues) { infNanValues(func); }
