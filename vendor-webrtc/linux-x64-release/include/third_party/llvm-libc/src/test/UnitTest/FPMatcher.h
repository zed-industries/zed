//===-- FPMatchers.h --------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_TEST_UNITTEST_FPMATCHER_H
#define LLVM_LIBC_TEST_UNITTEST_FPMATCHER_H

#include "src/__support/CPP/array.h"
#include "src/__support/CPP/type_traits.h"
#include "src/__support/FPUtil/FEnvImpl.h"
#include "src/__support/FPUtil/FPBits.h"
#include "src/__support/FPUtil/fpbits_str.h"
#include "src/__support/macros/config.h"
#include "src/__support/macros/properties/architectures.h"
#include "test/UnitTest/RoundingModeUtils.h"
#include "test/UnitTest/StringUtils.h"
#include "test/UnitTest/Test.h"

#include "hdr/math_macros.h"

using LIBC_NAMESPACE::Sign;

namespace LIBC_NAMESPACE_DECL {
namespace testing {

template <typename T, TestCond Condition> class FPMatcher : public Matcher<T> {
  static_assert(cpp::is_floating_point_v<T>,
                "FPMatcher can only be used with floating point values.");
  static_assert(Condition == TestCond::EQ || Condition == TestCond::NE,
                "Unsupported FPMatcher test condition.");

  T expected;
  T actual;

public:
  FPMatcher(T expectedValue) : expected(expectedValue) {}

  bool match(T actualValue) {
    actual = actualValue;
    fputil::FPBits<T> actualBits(actual), expectedBits(expected);
    if (Condition == TestCond::EQ)
      return (actualBits.is_nan() && expectedBits.is_nan()) ||
             (actualBits.uintval() == expectedBits.uintval());

    // If condition == TestCond::NE.
    if (actualBits.is_nan())
      return !expectedBits.is_nan();
    return expectedBits.is_nan() ||
           (actualBits.uintval() != expectedBits.uintval());
  }

  void explainError() override {
    tlog << "Expected floating point value: "
         << str(fputil::FPBits<T>(expected)) << '\n';
    tlog << "Actual floating point value: " << str(fputil::FPBits<T>(actual))
         << '\n';
  }
};

template <typename T, TestCond Condition> class CFPMatcher : public Matcher<T> {
  static_assert(
      cpp::is_complex_v<T>,
      "CFPMatcher can only be used with complex floating point values.");
  static_assert(Condition == TestCond::EQ || Condition == TestCond::NE,
                "Unsupported CFPMatcher test condition.");

  T expected;
  T actual;

public:
  CFPMatcher(T expectedValue) : expected(expectedValue) {}

  template <typename CFT> bool matchComplex() {
    CFT *actualCmplxPtr = reinterpret_cast<CFT *>(&actual);
    CFT *expectedCmplxPtr = reinterpret_cast<CFT *>(&expected);
    CFT actualReal = actualCmplxPtr[0];
    CFT actualImag = actualCmplxPtr[1];
    CFT expectedReal = expectedCmplxPtr[0];
    CFT expectedImag = expectedCmplxPtr[1];
    fputil::FPBits<CFT> actualRealBits(actualReal),
        expectedRealBits(expectedReal);
    fputil::FPBits<CFT> actualImagBits(actualImag),
        expectedImagBits(expectedImag);
    if (Condition == TestCond::EQ)
      return ((actualRealBits.is_nan() && expectedRealBits.is_nan()) ||
              (actualRealBits.uintval() == expectedRealBits.uintval())) &&
             ((actualImagBits.is_nan() && expectedImagBits.is_nan()) ||
              (actualImagBits.uintval() == expectedImagBits.uintval()));

    // If condition == TestCond::NE.
    if (actualRealBits.is_nan() && expectedRealBits.is_nan())
      return !expectedRealBits.is_nan() && !expectedImagBits.is_nan();
    if (actualRealBits.is_nan())
      return !expectedRealBits.is_nan();
    if (actualImagBits.is_nan())
      return !expectedImagBits.is_nan();
    return (expectedRealBits.is_nan() ||
            actualRealBits.uintval() != expectedRealBits.uintval()) &&
           (expectedImagBits.is_nan() ||
            actualImagBits.uintval() != expectedImagBits.uintval());
  }

  template <typename CFT> void explainErrorComplex() {
    CFT *actualCmplxPtr = reinterpret_cast<CFT *>(&actual);
    CFT *expectedCmplxPtr = reinterpret_cast<CFT *>(&expected);
    CFT actualReal = actualCmplxPtr[0];
    CFT actualImag = actualCmplxPtr[1];
    CFT expectedReal = expectedCmplxPtr[0];
    CFT expectedImag = expectedCmplxPtr[1];
    tlog << "Expected complex floating point value: "
         << str(fputil::FPBits<CFT>(expectedReal)) + " + " +
                str(fputil::FPBits<CFT>(expectedImag)) + "i"
         << '\n';
    tlog << "Actual complex floating point value: "
         << str(fputil::FPBits<CFT>(actualReal)) + " + " +
                str(fputil::FPBits<CFT>(actualImag)) + "i"
         << '\n';
  }

  bool match(T actualValue) {
    actual = actualValue;
    if constexpr (cpp::is_complex_type_same<T, _Complex float>())
      return matchComplex<float>();
    else if constexpr (cpp::is_complex_type_same<T, _Complex double>())
      return matchComplex<double>();
    else if constexpr (cpp::is_complex_type_same<T, _Complex long double>())
      return matchComplex<long double>();
#ifdef LIBC_TYPES_HAS_CFLOAT16
    else if constexpr (cpp::is_complex_type_same<T, cfloat16>())
      return matchComplex<float16>();
#endif
#ifdef LIBC_TYPES_HAS_CFLOAT128
    else if constexpr (cpp::is_complex_type_same<T, cfloat128>())
      return matchComplex<float128>();
#endif
  }

  void explainError() override {
    if constexpr (cpp::is_complex_type_same<T, _Complex float>())
      return explainErrorComplex<float>();
    else if constexpr (cpp::is_complex_type_same<T, _Complex double>())
      return explainErrorComplex<double>();
    else if constexpr (cpp::is_complex_type_same<T, _Complex long double>())
      return explainErrorComplex<long double>();
#ifdef LIBC_TYPES_HAS_CFLOAT16
    else if constexpr (cpp::is_complex_type_same<T, cfloat16>())
      return explainErrorComplex<float16>();
#endif
#ifdef LIBC_TYPES_HAS_CFLOAT128
    else if constexpr (cpp::is_complex_type_same<T, cfloat128>())
      return explainErrorComplex<float128>();
#endif
  }
};

template <TestCond C, typename T> FPMatcher<T, C> getMatcher(T expectedValue) {
  return FPMatcher<T, C>(expectedValue);
}

template <TestCond C, typename T>
CFPMatcher<T, C> getMatcherComplex(T expectedValue) {
  return CFPMatcher<T, C>(expectedValue);
}

template <typename T> struct FPTest : public Test {
  using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;
  using StorageType = typename FPBits::StorageType;
  static constexpr StorageType STORAGE_MAX =
      LIBC_NAMESPACE::cpp::numeric_limits<StorageType>::max();
  static constexpr T zero = FPBits::zero(Sign::POS).get_val();
  static constexpr T neg_zero = FPBits::zero(Sign::NEG).get_val();
  static constexpr T aNaN = FPBits::quiet_nan(Sign::POS).get_val();
  static constexpr T neg_aNaN = FPBits::quiet_nan(Sign::NEG).get_val();
  static constexpr T sNaN = FPBits::signaling_nan().get_val();
  static constexpr T inf = FPBits::inf(Sign::POS).get_val();
  static constexpr T neg_inf = FPBits::inf(Sign::NEG).get_val();
  static constexpr T min_normal = FPBits::min_normal().get_val();
  static constexpr T max_normal = FPBits::max_normal(Sign::POS).get_val();
  static constexpr T neg_max_normal = FPBits::max_normal(Sign::NEG).get_val();
  static constexpr T min_denormal = FPBits::min_subnormal().get_val();
  static constexpr T max_denormal = FPBits::max_subnormal().get_val();

  static constexpr int N_ROUNDING_MODES = 4;
  static constexpr fputil::testing::RoundingMode ROUNDING_MODES[4] = {
      fputil::testing::RoundingMode::Nearest,
      fputil::testing::RoundingMode::Upward,
      fputil::testing::RoundingMode::Downward,
      fputil::testing::RoundingMode::TowardZero,
  };
};

// Add facility to test Flush-Denormal-To-Zero (FTZ) and Denormal-As-Zero (DAZ)
// modes.
// These tests to ensure that our implementations will not crash under these
// modes.
#if defined(LIBC_TARGET_ARCH_IS_X86_64) && __has_builtin(__builtin_ia32_stmxcsr)

#define LIBC_TEST_FTZ_DAZ

static constexpr unsigned FTZ = 0x8000; // Flush denormal to zero
static constexpr unsigned DAZ = 0x0040; // Denormal as zero

struct ModifyMXCSR {
  ModifyMXCSR(unsigned flags) {
    old_mxcsr = __builtin_ia32_stmxcsr();
    __builtin_ia32_ldmxcsr(old_mxcsr | flags);
  }

  ~ModifyMXCSR() { __builtin_ia32_ldmxcsr(old_mxcsr); }

private:
  unsigned old_mxcsr;
};

#endif

} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

#define DECLARE_SPECIAL_CONSTANTS(T)                                           \
  using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;                            \
  using StorageType = typename FPBits::StorageType;                            \
                                                                               \
  static constexpr StorageType STORAGE_MAX =                                   \
      LIBC_NAMESPACE::cpp::numeric_limits<StorageType>::max();                 \
  const T zero = FPBits::zero(Sign::POS).get_val();                            \
  const T neg_zero = FPBits::zero(Sign::NEG).get_val();                        \
  const T aNaN = FPBits::quiet_nan(Sign::POS).get_val();                       \
  const T neg_aNaN = FPBits::quiet_nan(Sign::NEG).get_val();                   \
  const T sNaN = FPBits::signaling_nan(Sign::POS).get_val();                   \
  const T neg_sNaN = FPBits::signaling_nan(Sign::NEG).get_val();               \
  const T inf = FPBits::inf(Sign::POS).get_val();                              \
  const T neg_inf = FPBits::inf(Sign::NEG).get_val();                          \
  const T min_normal = FPBits::min_normal().get_val();                         \
  const T max_normal = FPBits::max_normal(Sign::POS).get_val();                \
  const T neg_max_normal = FPBits::max_normal(Sign::NEG).get_val();            \
  const T min_denormal = FPBits::min_subnormal(Sign::POS).get_val();           \
  const T neg_min_denormal = FPBits::min_subnormal(Sign::NEG).get_val();       \
  const T max_denormal = FPBits::max_subnormal().get_val();                    \
  static constexpr int UNKNOWN_MATH_ROUNDING_DIRECTION = 99;                   \
  static constexpr LIBC_NAMESPACE::cpp::array<int, 6>                          \
      MATH_ROUNDING_DIRECTIONS_INCLUDING_UNKNOWN = {                           \
          FP_INT_UPWARD,     FP_INT_DOWNWARD,                                  \
          FP_INT_TOWARDZERO, FP_INT_TONEARESTFROMZERO,                         \
          FP_INT_TONEAREST,  UNKNOWN_MATH_ROUNDING_DIRECTION,                  \
  };

#define EXPECT_FP_EQ(expected, actual)                                         \
  EXPECT_THAT(actual, LIBC_NAMESPACE::testing::getMatcher<                     \
                          LIBC_NAMESPACE::testing::TestCond::EQ>(expected))

#define EXPECT_CFP_EQ(expected, actual)                                        \
  EXPECT_THAT(actual, LIBC_NAMESPACE::testing::getMatcherComplex<              \
                          LIBC_NAMESPACE::testing::TestCond::EQ>(expected))

#define TEST_FP_EQ(expected, actual)                                           \
  LIBC_NAMESPACE::testing::getMatcher<LIBC_NAMESPACE::testing::TestCond::EQ>(  \
      expected)                                                                \
      .match(actual)

#define EXPECT_FP_IS_NAN(actual) EXPECT_TRUE((actual) != (actual))

#define ASSERT_FP_EQ(expected, actual)                                         \
  ASSERT_THAT(actual, LIBC_NAMESPACE::testing::getMatcher<                     \
                          LIBC_NAMESPACE::testing::TestCond::EQ>(expected))

#define EXPECT_FP_NE(expected, actual)                                         \
  EXPECT_THAT(actual, LIBC_NAMESPACE::testing::getMatcher<                     \
                          LIBC_NAMESPACE::testing::TestCond::NE>(expected))

#define ASSERT_FP_NE(expected, actual)                                         \
  ASSERT_THAT(actual, LIBC_NAMESPACE::testing::getMatcher<                     \
                          LIBC_NAMESPACE::testing::TestCond::NE>(expected))

#define EXPECT_MATH_ERRNO(expected)                                            \
  do {                                                                         \
    if (math_errhandling & MATH_ERRNO) {                                       \
      int actual = LIBC_NAMESPACE::libc_errno;                                 \
      LIBC_NAMESPACE::libc_errno = 0;                                          \
      EXPECT_EQ(actual, expected);                                             \
    }                                                                          \
  } while (0)

#define ASSERT_MATH_ERRNO(expected)                                            \
  do {                                                                         \
    if (math_errhandling & MATH_ERRNO) {                                       \
      int actual = LIBC_NAMESPACE::libc_errno;                                 \
      LIBC_NAMESPACE::libc_errno = 0;                                          \
      ASSERT_EQ(actual, expected);                                             \
    }                                                                          \
  } while (0)

#define EXPECT_FP_EXCEPTION(expected)                                          \
  do {                                                                         \
    if (math_errhandling & MATH_ERREXCEPT) {                                   \
      EXPECT_EQ(                                                               \
          LIBC_NAMESPACE::fputil::test_except(                                 \
              static_cast<int>(FE_ALL_EXCEPT)) &                               \
              ((expected) ? (expected) : static_cast<int>(FE_ALL_EXCEPT)),     \
          (expected));                                                         \
    }                                                                          \
  } while (0)

#define ASSERT_FP_EXCEPTION(expected)                                          \
  do {                                                                         \
    if (math_errhandling & MATH_ERREXCEPT) {                                   \
      ASSERT_EQ(                                                               \
          LIBC_NAMESPACE::fputil::test_except(                                 \
              static_cast<int>(FE_ALL_EXCEPT)) &                               \
              ((expected) ? (expected) : static_cast<int>(FE_ALL_EXCEPT)),     \
          (expected));                                                         \
    }                                                                          \
  } while (0)

#define EXPECT_FP_EQ_WITH_EXCEPTION(expected_val, actual_val, expected_except) \
  do {                                                                         \
    LIBC_NAMESPACE::fputil::clear_except(static_cast<int>(FE_ALL_EXCEPT));     \
    EXPECT_FP_EQ(expected_val, actual_val);                                    \
    EXPECT_FP_EXCEPTION(expected_except);                                      \
  } while (0)

#define EXPECT_FP_IS_NAN_WITH_EXCEPTION(actual_val, expected_except)           \
  do {                                                                         \
    LIBC_NAMESPACE::fputil::clear_except(static_cast<int>(FE_ALL_EXCEPT));     \
    EXPECT_FP_IS_NAN(actual_val);                                              \
    EXPECT_FP_EXCEPTION(expected_except);                                      \
  } while (0)

#define EXPECT_FP_EQ_ROUNDING_MODE(expected, actual, rounding_mode)            \
  do {                                                                         \
    using namespace LIBC_NAMESPACE::fputil::testing;                           \
    ForceRoundingMode __r((rounding_mode));                                    \
    if (__r.success) {                                                         \
      EXPECT_FP_EQ((expected), (actual));                                      \
    }                                                                          \
  } while (0)

#define EXPECT_FP_EQ_ROUNDING_NEAREST(expected, actual)                        \
  EXPECT_FP_EQ_ROUNDING_MODE((expected), (actual), RoundingMode::Nearest)

#define EXPECT_FP_EQ_ROUNDING_UPWARD(expected, actual)                         \
  EXPECT_FP_EQ_ROUNDING_MODE((expected), (actual), RoundingMode::Upward)

#define EXPECT_FP_EQ_ROUNDING_DOWNWARD(expected, actual)                       \
  EXPECT_FP_EQ_ROUNDING_MODE((expected), (actual), RoundingMode::Downward)

#define EXPECT_FP_EQ_ROUNDING_TOWARD_ZERO(expected, actual)                    \
  EXPECT_FP_EQ_ROUNDING_MODE((expected), (actual), RoundingMode::TowardZero)

#define EXPECT_FP_EQ_ALL_ROUNDING_1(expected, actual)                          \
  do {                                                                         \
    EXPECT_FP_EQ_ROUNDING_NEAREST((expected), (actual));                       \
    EXPECT_FP_EQ_ROUNDING_UPWARD((expected), (actual));                        \
    EXPECT_FP_EQ_ROUNDING_DOWNWARD((expected), (actual));                      \
    EXPECT_FP_EQ_ROUNDING_TOWARD_ZERO((expected), (actual));                   \
  } while (0)

#define EXPECT_FP_EQ_ALL_ROUNDING_4(expected_nearest, expected_upward,         \
                                    expected_downward, expected_toward_zero,   \
                                    actual)                                    \
  do {                                                                         \
    EXPECT_FP_EQ_ROUNDING_NEAREST((expected_nearest), (actual));               \
    EXPECT_FP_EQ_ROUNDING_UPWARD((expected_upward), (actual));                 \
    EXPECT_FP_EQ_ROUNDING_DOWNWARD((expected_downward), (actual));             \
    EXPECT_FP_EQ_ROUNDING_TOWARD_ZERO((expected_toward_zero), (actual));       \
  } while (0)

#define EXPECT_FP_EQ_ALL_ROUNDING_UNSUPPORTED(...)                             \
  static_assert(false, "Unsupported number of arguments")

#define EXPECT_FP_EQ_ALL_ROUNDING_GET_6TH_ARG(ARG1, ARG2, ARG3, ARG4, ARG5,    \
                                              ARG6, ...)                       \
  ARG6

#define EXPECT_FP_EQ_ALL_ROUNDING_SELECTION(...)                               \
  EXPECT_FP_EQ_ALL_ROUNDING_GET_6TH_ARG(                                       \
      __VA_ARGS__, EXPECT_FP_EQ_ALL_ROUNDING_4,                                \
      EXPECT_FP_EQ_ALL_ROUNDING_UNSUPPORTED,                                   \
      EXPECT_FP_EQ_ALL_ROUNDING_UNSUPPORTED, EXPECT_FP_EQ_ALL_ROUNDING_1)

#define EXPECT_FP_EQ_ALL_ROUNDING(...)                                         \
  EXPECT_FP_EQ_ALL_ROUNDING_SELECTION(__VA_ARGS__)(__VA_ARGS__)

#define ASSERT_FP_EQ_ROUNDING_MODE(expected, actual, rounding_mode)            \
  do {                                                                         \
    using namespace LIBC_NAMESPACE::fputil::testing;                           \
    ForceRoundingMode __r((rounding_mode));                                    \
    if (__r.success) {                                                         \
      ASSERT_FP_EQ((expected), (actual));                                      \
    }                                                                          \
  } while (0)

#define ASSERT_FP_EQ_ROUNDING_NEAREST(expected, actual)                        \
  ASSERT_FP_EQ_ROUNDING_MODE((expected), (actual), RoundingMode::Nearest)

#define ASSERT_FP_EQ_ROUNDING_UPWARD(expected, actual)                         \
  ASSERT_FP_EQ_ROUNDING_MODE((expected), (actual), RoundingMode::Upward)

#define ASSERT_FP_EQ_ROUNDING_DOWNWARD(expected, actual)                       \
  ASSERT_FP_EQ_ROUNDING_MODE((expected), (actual), RoundingMode::Downward)

#define ASSERT_FP_EQ_ROUNDING_TOWARD_ZERO(expected, actual)                    \
  ASSERT_FP_EQ_ROUNDING_MODE((expected), (actual), RoundingMode::TowardZero)

#define EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_MODE(                             \
    expected, actual, expected_except, rounding_mode)                          \
  do {                                                                         \
    using namespace LIBC_NAMESPACE::fputil::testing;                           \
    ForceRoundingMode __r((rounding_mode));                                    \
    if (__r.success) {                                                         \
      LIBC_NAMESPACE::fputil::clear_except(static_cast<int>(FE_ALL_EXCEPT));   \
      EXPECT_FP_EQ((expected), (actual));                                      \
      EXPECT_FP_EXCEPTION(expected_except);                                    \
    }                                                                          \
  } while (0)

#define EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_NEAREST(expected, actual,         \
                                                     expected_except)          \
  EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_MODE(                                   \
      (expected), (actual), (expected_except), RoundingMode::Nearest)

#define EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_UPWARD(expected, actual,          \
                                                    expected_except)           \
  EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_MODE(                                   \
      (expected), (actual), (expected_except), RoundingMode::Upward)

#define EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_DOWNWARD(expected, actual,        \
                                                      expected_except)         \
  EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_MODE(                                   \
      (expected), (actual), (expected_except), RoundingMode::Downward)

#define EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_TOWARD_ZERO(expected, actual,     \
                                                         expected_except)      \
  EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_MODE(                                   \
      (expected), (actual), (expected_except), RoundingMode::TowardZero)

#define EXPECT_FP_EQ_WITH_EXCEPTION_ALL_ROUNDING(expected, actual,             \
                                                 expected_except)              \
  do {                                                                         \
    EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_NEAREST((expected), (actual),         \
                                                 (expected_except));           \
    EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_UPWARD((expected), (actual),          \
                                                (expected_except));            \
    EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_DOWNWARD((expected), (actual),        \
                                                  (expected_except));          \
    EXPECT_FP_EQ_WITH_EXCEPTION_ROUNDING_TOWARD_ZERO((expected), (actual),     \
                                                     (expected_except));       \
  } while (0)

#endif // LLVM_LIBC_TEST_UNITTEST_FPMATCHER_H
