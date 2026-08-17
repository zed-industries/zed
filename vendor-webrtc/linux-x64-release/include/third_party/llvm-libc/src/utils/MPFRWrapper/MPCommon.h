//===-- MPCommon.h ----------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_UTILS_MPFRWRAPPER_MPCOMMON_H
#define LLVM_LIBC_UTILS_MPFRWRAPPER_MPCOMMON_H

#include "src/__support/CPP/string.h"
#include "src/__support/CPP/type_traits.h"
#include "src/__support/FPUtil/FPBits.h"
#include "src/__support/macros/config.h"
#include "test/UnitTest/RoundingModeUtils.h"

#include <stdint.h>

#include "mpfr_inc.h"

#ifdef LIBC_TYPES_FLOAT128_IS_NOT_LONG_DOUBLE
extern "C" {
int mpfr_set_float128(mpfr_ptr, float128, mpfr_rnd_t);
float128 mpfr_get_float128(mpfr_srcptr, mpfr_rnd_t);
}
#endif

namespace LIBC_NAMESPACE_DECL {
namespace testing {
namespace mpfr {

template <typename T> using FPBits = LIBC_NAMESPACE::fputil::FPBits<T>;
using LIBC_NAMESPACE::fputil::testing::RoundingMode;

// A precision value which allows sufficiently large additional
// precision compared to the floating point precision.
template <typename T> struct ExtraPrecision;

#ifdef LIBC_TYPES_HAS_FLOAT16
template <> struct ExtraPrecision<float16> {
  static constexpr unsigned int VALUE = 128;
};
#endif

template <> struct ExtraPrecision<float> {
  static constexpr unsigned int VALUE = 128;
};

template <> struct ExtraPrecision<double> {
  static constexpr unsigned int VALUE = 256;
};

template <> struct ExtraPrecision<long double> {
#ifdef LIBC_TYPES_LONG_DOUBLE_IS_FLOAT128
  static constexpr unsigned int VALUE = 512;
#else
  static constexpr unsigned int VALUE = 256;
#endif
};

#if defined(LIBC_TYPES_FLOAT128_IS_NOT_LONG_DOUBLE)
template <> struct ExtraPrecision<float128> {
  static constexpr unsigned int VALUE = 512;
};
#endif // LIBC_TYPES_FLOAT128_IS_NOT_LONG_DOUBLE

// If the ulp tolerance is less than or equal to 0.5, we would check that the
// result is rounded correctly with respect to the rounding mode by using the
// same precision as the inputs.
template <typename T>
static inline unsigned int get_precision(double ulp_tolerance) {
  if (ulp_tolerance <= 0.5) {
    return LIBC_NAMESPACE::fputil::FPBits<T>::FRACTION_LEN + 1;
  } else {
    return ExtraPrecision<T>::VALUE;
  }
}

static inline mpfr_rnd_t get_mpfr_rounding_mode(RoundingMode mode) {
  switch (mode) {
  case RoundingMode::Upward:
    return MPFR_RNDU;
    break;
  case RoundingMode::Downward:
    return MPFR_RNDD;
    break;
  case RoundingMode::TowardZero:
    return MPFR_RNDZ;
    break;
  case RoundingMode::Nearest:
    return MPFR_RNDN;
    break;
  }
  __builtin_unreachable();
}

class MPFRNumber {
  unsigned int mpfr_precision;
  mpfr_rnd_t mpfr_rounding;
  mpfr_t value;

public:
  MPFRNumber();
  // We use explicit EnableIf specializations to disallow implicit
  // conversions. Implicit conversions can potentially lead to loss of
  // precision. We exceptionally allow implicit conversions from float16
  // to float, as the MPFR API does not support float16, thus requiring
  // conversion to a higher-precision format.
  template <typename XType,
            cpp::enable_if_t<cpp::is_same_v<float, XType>
#ifdef LIBC_TYPES_HAS_FLOAT16
                                 || cpp::is_same_v<float16, XType>
#endif
                             ,
                             int> = 0>
  explicit MPFRNumber(XType x,
                      unsigned int precision = ExtraPrecision<XType>::VALUE,
                      RoundingMode rounding = RoundingMode::Nearest)
      : mpfr_precision(precision),
        mpfr_rounding(get_mpfr_rounding_mode(rounding)) {
    mpfr_init2(value, mpfr_precision);
    mpfr_set_flt(value, x, mpfr_rounding);
  }

  template <typename XType,
            cpp::enable_if_t<cpp::is_same_v<double, XType>, int> = 0>
  explicit MPFRNumber(XType x,
                      unsigned int precision = ExtraPrecision<XType>::VALUE,
                      RoundingMode rounding = RoundingMode::Nearest)
      : mpfr_precision(precision),
        mpfr_rounding(get_mpfr_rounding_mode(rounding)) {
    mpfr_init2(value, mpfr_precision);
    mpfr_set_d(value, x, mpfr_rounding);
  }

  template <typename XType,
            cpp::enable_if_t<cpp::is_same_v<long double, XType>, int> = 0>
  explicit MPFRNumber(XType x,
                      unsigned int precision = ExtraPrecision<XType>::VALUE,
                      RoundingMode rounding = RoundingMode::Nearest)
      : mpfr_precision(precision),
        mpfr_rounding(get_mpfr_rounding_mode(rounding)) {
    mpfr_init2(value, mpfr_precision);
    mpfr_set_ld(value, x, mpfr_rounding);
  }

#ifdef LIBC_TYPES_FLOAT128_IS_NOT_LONG_DOUBLE
  template <typename XType,
            cpp::enable_if_t<cpp::is_same_v<float128, XType>, int> = 0>
  explicit MPFRNumber(XType x,
                      unsigned int precision = ExtraPrecision<XType>::VALUE,
                      RoundingMode rounding = RoundingMode::Nearest)
      : mpfr_precision(precision),
        mpfr_rounding(get_mpfr_rounding_mode(rounding)) {
    mpfr_init2(value, mpfr_precision);
    mpfr_set_float128(value, x, mpfr_rounding);
  }
#endif // LIBC_TYPES_FLOAT128_IS_NOT_LONG_DOUBLE

  template <typename XType,
            cpp::enable_if_t<cpp::is_integral_v<XType>, int> = 0>
  explicit MPFRNumber(XType x,
                      unsigned int precision = ExtraPrecision<float>::VALUE,
                      RoundingMode rounding = RoundingMode::Nearest)
      : mpfr_precision(precision),
        mpfr_rounding(get_mpfr_rounding_mode(rounding)) {
    mpfr_init2(value, mpfr_precision);
    mpfr_set_sj(value, x, mpfr_rounding);
  }

  MPFRNumber(const MPFRNumber &other);
  MPFRNumber(const MPFRNumber &other, unsigned int precision);
  MPFRNumber(const mpfr_t x, unsigned int precision, RoundingMode rounding);

  ~MPFRNumber();

  MPFRNumber &operator=(const MPFRNumber &rhs);

  bool is_nan() const;
  MPFRNumber abs() const;
  MPFRNumber acos() const;
  MPFRNumber acosh() const;
  MPFRNumber acospi() const;
  MPFRNumber add(const MPFRNumber &b) const;
  MPFRNumber asin() const;
  MPFRNumber asinh() const;
  MPFRNumber atan() const;
  MPFRNumber atan2(const MPFRNumber &b);
  MPFRNumber atanh() const;
  MPFRNumber cbrt() const;
  MPFRNumber ceil() const;
  MPFRNumber cos() const;
  MPFRNumber cosh() const;
  MPFRNumber cospi() const;
  MPFRNumber erf() const;
  MPFRNumber exp() const;
  MPFRNumber exp2() const;
  MPFRNumber exp2m1() const;
  MPFRNumber exp10() const;
  MPFRNumber exp10m1() const;
  MPFRNumber expm1() const;
  MPFRNumber div(const MPFRNumber &b) const;
  MPFRNumber floor() const;
  MPFRNumber fmod(const MPFRNumber &b);
  MPFRNumber frexp(int &exp);
  MPFRNumber hypot(const MPFRNumber &b);
  MPFRNumber log() const;
  MPFRNumber log2() const;
  MPFRNumber log10() const;
  MPFRNumber log1p() const;
  MPFRNumber pow(const MPFRNumber &b);
  MPFRNumber remquo(const MPFRNumber &divisor, int &quotient);
  MPFRNumber round() const;
  MPFRNumber roundeven() const;
  bool round_to_long(long &result) const;
  bool round_to_long(mpfr_rnd_t rnd, long &result) const;
  MPFRNumber rint(mpfr_rnd_t rnd) const;
  MPFRNumber mod_2pi() const;
  MPFRNumber mod_pi_over_2() const;
  MPFRNumber mod_pi_over_4() const;
  MPFRNumber sin() const;
  MPFRNumber sinpi() const;
  MPFRNumber sinh() const;
  MPFRNumber sqrt() const;
  MPFRNumber sub(const MPFRNumber &b) const;
  MPFRNumber tan() const;
  MPFRNumber tanh() const;
  MPFRNumber tanpi() const;
  MPFRNumber trunc() const;
  MPFRNumber fma(const MPFRNumber &b, const MPFRNumber &c);
  MPFRNumber mul(const MPFRNumber &b);
  cpp::string str() const;

  template <typename T> T as() const;
  void dump(const char *msg) const;

  // Return the ULP (units-in-the-last-place) difference between the
  // stored MPFR and a floating point number.
  //
  // We define ULP difference as follows:
  //   If exponents of this value and the |input| are same, then:
  //     ULP(this_value, input) = abs(this_value - input) / eps(input)
  //   else:
  //     max = max(abs(this_value), abs(input))
  //     min = min(abs(this_value), abs(input))
  //     maxExponent = exponent(max)
  //     ULP(this_value, input) = (max - 2^maxExponent) / eps(max) +
  //                              (2^maxExponent - min) / eps(min)
  //
  // Remarks:
  // 1. A ULP of 0.0 will imply that the value is correctly rounded.
  // 2. We expect that this value and the value to be compared (the [input]
  //    argument) are reasonable close, and we will provide an upper bound
  //    of ULP value for testing.  Morever, most of the fractional parts of
  //    ULP value do not matter much, so using double as the return type
  //    should be good enough.
  // 3. For close enough values (values which don't diff in their exponent by
  //    not more than 1), a ULP difference of N indicates a bit distance
  //    of N between this number and [input].
  // 4. A values of +0.0 and -0.0 are treated as equal.
  template <typename T>
  cpp::enable_if_t<cpp::is_floating_point_v<T>, MPFRNumber>
  ulp_as_mpfr_number(T input) {
    T thisAsT = as<T>();
    if (thisAsT == input)
      return MPFRNumber(0.0);

    if (is_nan()) {
      if (FPBits<T>(input).is_nan())
        return MPFRNumber(0.0);
      return MPFRNumber(FPBits<T>::inf().get_val());
    }

    int thisExponent = FPBits<T>(thisAsT).get_exponent();
    int inputExponent = FPBits<T>(input).get_exponent();
    // Adjust the exponents for denormal numbers.
    if (FPBits<T>(thisAsT).is_subnormal())
      ++thisExponent;
    if (FPBits<T>(input).is_subnormal())
      ++inputExponent;

    if (thisAsT * input < 0 || thisExponent == inputExponent) {
      MPFRNumber inputMPFR(input);
      mpfr_sub(inputMPFR.value, value, inputMPFR.value, MPFR_RNDN);
      mpfr_abs(inputMPFR.value, inputMPFR.value, MPFR_RNDN);
      mpfr_mul_2si(inputMPFR.value, inputMPFR.value,
                   -thisExponent + FPBits<T>::FRACTION_LEN, MPFR_RNDN);
      return inputMPFR;
    }

    // If the control reaches here, it means that this number and input are
    // of the same sign but different exponent. In such a case, ULP error is
    // calculated as sum of two parts.
    thisAsT = FPBits<T>(thisAsT).abs().get_val();
    input = FPBits<T>(input).abs().get_val();
    T min = thisAsT > input ? input : thisAsT;
    T max = thisAsT > input ? thisAsT : input;
    int minExponent = FPBits<T>(min).get_exponent();
    int maxExponent = FPBits<T>(max).get_exponent();
    // Adjust the exponents for denormal numbers.
    if (FPBits<T>(min).is_subnormal())
      ++minExponent;
    if (FPBits<T>(max).is_subnormal())
      ++maxExponent;

    MPFRNumber minMPFR(min);
    MPFRNumber maxMPFR(max);

    MPFRNumber pivot(uint32_t(1));
    mpfr_mul_2si(pivot.value, pivot.value, maxExponent, MPFR_RNDN);

    mpfr_sub(minMPFR.value, pivot.value, minMPFR.value, MPFR_RNDN);
    mpfr_mul_2si(minMPFR.value, minMPFR.value,
                 -minExponent + FPBits<T>::FRACTION_LEN, MPFR_RNDN);

    mpfr_sub(maxMPFR.value, maxMPFR.value, pivot.value, MPFR_RNDN);
    mpfr_mul_2si(maxMPFR.value, maxMPFR.value,
                 -maxExponent + FPBits<T>::FRACTION_LEN, MPFR_RNDN);

    mpfr_add(minMPFR.value, minMPFR.value, maxMPFR.value, MPFR_RNDN);
    return minMPFR;
  }

  template <typename T>
  cpp::enable_if_t<cpp::is_floating_point_v<T>, cpp::string>
  ulp_as_string(T input) {
    MPFRNumber num = ulp_as_mpfr_number(input);
    return num.str();
  }

  template <typename T>
  cpp::enable_if_t<cpp::is_floating_point_v<T>, double> ulp(T input) {
    MPFRNumber num = ulp_as_mpfr_number(input);
    return num.as<double>();
  }
};

} // namespace mpfr
} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_UTILS_MPFRWRAPPER_MPCOMMON_H
