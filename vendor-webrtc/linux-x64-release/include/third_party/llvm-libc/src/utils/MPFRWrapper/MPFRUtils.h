//===-- MPFRUtils.h ---------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_UTILS_MPFRWRAPPER_MPFRUTILS_H
#define LLVM_LIBC_UTILS_MPFRWRAPPER_MPFRUTILS_H

#include "src/__support/CPP/type_traits.h"
#include "src/__support/macros/config.h"
#include "test/UnitTest/RoundingModeUtils.h"
#include "test/UnitTest/Test.h"

#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {
namespace testing {
namespace mpfr {

enum class Operation : int {
  // Operations which take a single floating point number as input
  // and produce a single floating point number as output. The input
  // and output floating point numbers are of the same kind.
  BeginUnaryOperationsSingleOutput,
  Abs,
  Acos,
  Acosh,
  Acospi,
  Asin,
  Asinh,
  Atan,
  Atanh,
  Cbrt,
  Ceil,
  Cos,
  Cosh,
  Cospi,
  Erf,
  Exp,
  Exp2,
  Exp2m1,
  Exp10,
  Exp10m1,
  Expm1,
  Floor,
  Log,
  Log2,
  Log10,
  Log1p,
  Mod2PI,
  ModPIOver2,
  ModPIOver4,
  Round,
  RoundEven,
  Sin,
  Sinpi,
  Sinh,
  Sqrt,
  Tan,
  Tanh,
  Tanpi,
  Trunc,
  EndUnaryOperationsSingleOutput,

  // Operations which take a single floating point nubmer as input
  // but produce two outputs. The first ouput is a floating point
  // number of the same type as the input. The second output is of type
  // 'int'.
  BeginUnaryOperationsTwoOutputs,
  Frexp, // Floating point output, the first output, is the fractional part.
  EndUnaryOperationsTwoOutputs,

  // Operations wich take two floating point nubmers of the same type as
  // input and produce a single floating point number of the same type as
  // output.
  BeginBinaryOperationsSingleOutput,
  Add,
  Atan2,
  Div,
  Fmod,
  Hypot,
  Mul,
  Pow,
  Sub,
  EndBinaryOperationsSingleOutput,

  // Operations which take two floating point numbers of the same type as
  // input and produce two outputs. The first output is a floating point number
  // of the same type as the inputs. The second output is of type 'int'.
  BeginBinaryOperationsTwoOutputs,
  RemQuo, // The first output(floating point) is the remainder.
  EndBinaryOperationsTwoOutputs,

  // Operations which take three floating point nubmers of the same type as
  // input and produce a single floating point number of the same type as
  // output.
  BeginTernaryOperationsSingleOuput,
  Fma,
  EndTernaryOperationsSingleOutput,
};

using LIBC_NAMESPACE::fputil::testing::ForceRoundingMode;
using LIBC_NAMESPACE::fputil::testing::RoundingMode;

template <typename T> struct BinaryInput {
  static_assert(
      LIBC_NAMESPACE::cpp::is_floating_point_v<T>,
      "Template parameter of BinaryInput must be a floating point type.");

  using Type = T;
  T x, y;
};

template <typename T> struct TernaryInput {
  static_assert(
      LIBC_NAMESPACE::cpp::is_floating_point_v<T>,
      "Template parameter of TernaryInput must be a floating point type.");

  using Type = T;
  T x, y, z;
};

template <typename T> struct BinaryOutput {
  T f;
  int i;
};

namespace internal {

template <typename T1, typename T2>
struct AreMatchingBinaryInputAndBinaryOutput {
  static constexpr bool VALUE = false;
};

template <typename T>
struct AreMatchingBinaryInputAndBinaryOutput<BinaryInput<T>, BinaryOutput<T>> {
  static constexpr bool VALUE = cpp::is_floating_point_v<T>;
};

template <typename T> struct IsBinaryInput {
  static constexpr bool VALUE = false;
};

template <typename T> struct IsBinaryInput<BinaryInput<T>> {
  static constexpr bool VALUE = true;
};

template <typename T> struct IsTernaryInput {
  static constexpr bool VALUE = false;
};

template <typename T> struct IsTernaryInput<TernaryInput<T>> {
  static constexpr bool VALUE = true;
};

template <typename T> struct MakeScalarInput : cpp::type_identity<T> {};

template <typename T>
struct MakeScalarInput<BinaryInput<T>> : cpp::type_identity<T> {};

template <typename T>
struct MakeScalarInput<TernaryInput<T>> : cpp::type_identity<T> {};

template <typename InputType, typename OutputType>
bool compare_unary_operation_single_output(Operation op, InputType input,
                                           OutputType libc_output,
                                           double ulp_tolerance,
                                           RoundingMode rounding);
template <typename T>
bool compare_unary_operation_two_outputs(Operation op, T input,
                                         const BinaryOutput<T> &libc_output,
                                         double ulp_tolerance,
                                         RoundingMode rounding);
template <typename T>
bool compare_binary_operation_two_outputs(Operation op,
                                          const BinaryInput<T> &input,
                                          const BinaryOutput<T> &libc_output,
                                          double ulp_tolerance,
                                          RoundingMode rounding);

template <typename InputType, typename OutputType>
bool compare_binary_operation_one_output(Operation op,
                                         const BinaryInput<InputType> &input,
                                         OutputType libc_output,
                                         double ulp_tolerance,
                                         RoundingMode rounding);

template <typename InputType, typename OutputType>
bool compare_ternary_operation_one_output(Operation op,
                                          const TernaryInput<InputType> &input,
                                          OutputType libc_output,
                                          double ulp_tolerance,
                                          RoundingMode rounding);

template <typename InputType, typename OutputType>
void explain_unary_operation_single_output_error(Operation op, InputType input,
                                                 OutputType match_value,
                                                 double ulp_tolerance,
                                                 RoundingMode rounding);
template <typename T>
void explain_unary_operation_two_outputs_error(
    Operation op, T input, const BinaryOutput<T> &match_value,
    double ulp_tolerance, RoundingMode rounding);
template <typename T>
void explain_binary_operation_two_outputs_error(
    Operation op, const BinaryInput<T> &input,
    const BinaryOutput<T> &match_value, double ulp_tolerance,
    RoundingMode rounding);

template <typename InputType, typename OutputType>
void explain_binary_operation_one_output_error(
    Operation op, const BinaryInput<InputType> &input, OutputType match_value,
    double ulp_tolerance, RoundingMode rounding);

template <typename InputType, typename OutputType>
void explain_ternary_operation_one_output_error(
    Operation op, const TernaryInput<InputType> &input, OutputType match_value,
    double ulp_tolerance, RoundingMode rounding);

template <Operation op, bool silent, typename InputType, typename OutputType>
class MPFRMatcher : public testing::Matcher<OutputType> {
  InputType input;
  OutputType match_value;
  double ulp_tolerance;
  RoundingMode rounding;

public:
  MPFRMatcher(InputType testInput, double ulp_tolerance, RoundingMode rounding)
      : input(testInput), ulp_tolerance(ulp_tolerance), rounding(rounding) {}

  bool match(OutputType libcResult) {
    match_value = libcResult;
    return match(input, match_value);
  }

  // This method is marked with NOLINT because the name `explainError` does not
  // conform to the coding style.
  void explainError() override { // NOLINT
    explain_error(input, match_value);
  }

  // Whether the `explainError` step is skipped or not.
  bool is_silent() const override { return silent; }

private:
  template <typename InType, typename OutType>
  bool match(InType in, OutType out) {
    return compare_unary_operation_single_output(op, in, out, ulp_tolerance,
                                                 rounding);
  }

  template <typename T> bool match(T in, const BinaryOutput<T> &out) {
    return compare_unary_operation_two_outputs(op, in, out, ulp_tolerance,
                                               rounding);
  }

  template <typename T, typename U>
  bool match(const BinaryInput<T> &in, U out) {
    return compare_binary_operation_one_output(op, in, out, ulp_tolerance,
                                               rounding);
  }

  template <typename T>
  bool match(BinaryInput<T> in, const BinaryOutput<T> &out) {
    return compare_binary_operation_two_outputs(op, in, out, ulp_tolerance,
                                                rounding);
  }

  template <typename InType, typename OutType>
  bool match(const TernaryInput<InType> &in, OutType out) {
    return compare_ternary_operation_one_output(op, in, out, ulp_tolerance,
                                                rounding);
  }

  template <typename InType, typename OutType>
  void explain_error(InType in, OutType out) {
    explain_unary_operation_single_output_error(op, in, out, ulp_tolerance,
                                                rounding);
  }

  template <typename T> void explain_error(T in, const BinaryOutput<T> &out) {
    explain_unary_operation_two_outputs_error(op, in, out, ulp_tolerance,
                                              rounding);
  }

  template <typename T>
  void explain_error(const BinaryInput<T> &in, const BinaryOutput<T> &out) {
    explain_binary_operation_two_outputs_error(op, in, out, ulp_tolerance,
                                               rounding);
  }

  template <typename T, typename U>
  void explain_error(const BinaryInput<T> &in, U out) {
    explain_binary_operation_one_output_error(op, in, out, ulp_tolerance,
                                              rounding);
  }

  template <typename InType, typename OutType>
  void explain_error(const TernaryInput<InType> &in, OutType out) {
    explain_ternary_operation_one_output_error(op, in, out, ulp_tolerance,
                                               rounding);
  }
};

} // namespace internal

// Return true if the input and ouput types for the operation op are valid
// types.
template <Operation op, typename InputType, typename OutputType>
constexpr bool is_valid_operation() {
  constexpr bool IS_NARROWING_OP =
      (op == Operation::Sqrt && cpp::is_floating_point_v<InputType> &&
       cpp::is_floating_point_v<OutputType> &&
       sizeof(OutputType) <= sizeof(InputType)) ||
      (Operation::BeginBinaryOperationsSingleOutput < op &&
       op < Operation::EndBinaryOperationsSingleOutput &&
       internal::IsBinaryInput<InputType>::VALUE &&
       cpp::is_floating_point_v<
           typename internal::MakeScalarInput<InputType>::type> &&
       cpp::is_floating_point_v<OutputType>) ||
      (op == Operation::Fma && internal::IsTernaryInput<InputType>::VALUE &&
       cpp::is_floating_point_v<
           typename internal::MakeScalarInput<InputType>::type> &&
       cpp::is_floating_point_v<OutputType>);
  if (IS_NARROWING_OP)
    return true;
  return (Operation::BeginUnaryOperationsSingleOutput < op &&
          op < Operation::EndUnaryOperationsSingleOutput &&
          cpp::is_same_v<InputType, OutputType> &&
          cpp::is_floating_point_v<InputType>) ||
         (Operation::BeginUnaryOperationsTwoOutputs < op &&
          op < Operation::EndUnaryOperationsTwoOutputs &&
          cpp::is_floating_point_v<InputType> &&
          cpp::is_same_v<OutputType, BinaryOutput<InputType>>) ||
         (Operation::BeginBinaryOperationsSingleOutput < op &&
          op < Operation::EndBinaryOperationsSingleOutput &&
          cpp::is_floating_point_v<OutputType> &&
          cpp::is_same_v<InputType, BinaryInput<OutputType>>) ||
         (Operation::BeginBinaryOperationsTwoOutputs < op &&
          op < Operation::EndBinaryOperationsTwoOutputs &&
          internal::AreMatchingBinaryInputAndBinaryOutput<InputType,
                                                          OutputType>::VALUE) ||
         (Operation::BeginTernaryOperationsSingleOuput < op &&
          op < Operation::EndTernaryOperationsSingleOutput &&
          cpp::is_floating_point_v<OutputType> &&
          cpp::is_same_v<InputType, TernaryInput<OutputType>>);
}

template <Operation op, typename InputType, typename OutputType>
__attribute__((no_sanitize("address"))) cpp::enable_if_t<
    is_valid_operation<op, InputType, OutputType>(),
    internal::MPFRMatcher<op, /*is_silent*/ false, InputType, OutputType>>
get_mpfr_matcher(InputType input, OutputType output_unused,
                 double ulp_tolerance, RoundingMode rounding) {
  return internal::MPFRMatcher<op, /*is_silent*/ false, InputType, OutputType>(
      input, ulp_tolerance, rounding);
}

template <Operation op, typename InputType, typename OutputType>
__attribute__((no_sanitize("address"))) cpp::enable_if_t<
    is_valid_operation<op, InputType, OutputType>(),
    internal::MPFRMatcher<op, /*is_silent*/ true, InputType, OutputType>>
get_silent_mpfr_matcher(InputType input, OutputType output_unused,
                        double ulp_tolerance, RoundingMode rounding) {
  return internal::MPFRMatcher<op, /*is_silent*/ true, InputType, OutputType>(
      input, ulp_tolerance, rounding);
}

template <typename T> T round(T x, RoundingMode mode);

template <typename T> bool round_to_long(T x, long &result);
template <typename T> bool round_to_long(T x, RoundingMode mode, long &result);

} // namespace mpfr
} // namespace testing
} // namespace LIBC_NAMESPACE_DECL

// GET_MPFR_DUMMY_ARG is going to be added to the end of GET_MPFR_MACRO as a
// simple way to avoid the compiler warning `gnu-zero-variadic-macro-arguments`.
#define GET_MPFR_DUMMY_ARG(...) 0

#define GET_MPFR_MACRO(__1, __2, __3, __4, __5, __NAME, ...) __NAME

#define EXPECT_MPFR_MATCH_DEFAULT(op, input, match_value, ulp_tolerance)       \
  EXPECT_THAT(match_value,                                                     \
              LIBC_NAMESPACE::testing::mpfr::get_mpfr_matcher<op>(             \
                  input, match_value, ulp_tolerance,                           \
                  LIBC_NAMESPACE::testing::mpfr::RoundingMode::Nearest))

#define EXPECT_MPFR_MATCH_ROUNDING(op, input, match_value, ulp_tolerance,      \
                                   rounding)                                   \
  EXPECT_THAT(match_value,                                                     \
              LIBC_NAMESPACE::testing::mpfr::get_mpfr_matcher<op>(             \
                  input, match_value, ulp_tolerance, rounding))

#define EXPECT_MPFR_MATCH(...)                                                 \
  GET_MPFR_MACRO(__VA_ARGS__, EXPECT_MPFR_MATCH_ROUNDING,                      \
                 EXPECT_MPFR_MATCH_DEFAULT, GET_MPFR_DUMMY_ARG)                \
  (__VA_ARGS__)

#define TEST_MPFR_MATCH_ROUNDING(op, input, match_value, ulp_tolerance,        \
                                 rounding)                                     \
  LIBC_NAMESPACE::testing::mpfr::get_mpfr_matcher<op>(input, match_value,      \
                                                      ulp_tolerance, rounding) \
      .match(match_value)

#define TEST_MPFR_MATCH(...)                                                   \
  GET_MPFR_MACRO(__VA_ARGS__, TEST_MPFR_MATCH_ROUNDING,                        \
                 EXPECT_MPFR_MATCH_DEFAULT, GET_MPFR_DUMMY_ARG)                \
  (__VA_ARGS__)

#define EXPECT_MPFR_MATCH_ALL_ROUNDING(op, input, match_value, ulp_tolerance)  \
  {                                                                            \
    namespace mpfr = LIBC_NAMESPACE::testing::mpfr;                            \
    mpfr::ForceRoundingMode __r1(mpfr::RoundingMode::Nearest);                 \
    if (__r1.success) {                                                        \
      EXPECT_MPFR_MATCH(op, input, match_value, ulp_tolerance,                 \
                        mpfr::RoundingMode::Nearest);                          \
    }                                                                          \
    mpfr::ForceRoundingMode __r2(mpfr::RoundingMode::Upward);                  \
    if (__r2.success) {                                                        \
      EXPECT_MPFR_MATCH(op, input, match_value, ulp_tolerance,                 \
                        mpfr::RoundingMode::Upward);                           \
    }                                                                          \
    mpfr::ForceRoundingMode __r3(mpfr::RoundingMode::Downward);                \
    if (__r3.success) {                                                        \
      EXPECT_MPFR_MATCH(op, input, match_value, ulp_tolerance,                 \
                        mpfr::RoundingMode::Downward);                         \
    }                                                                          \
    mpfr::ForceRoundingMode __r4(mpfr::RoundingMode::TowardZero);              \
    if (__r4.success) {                                                        \
      EXPECT_MPFR_MATCH(op, input, match_value, ulp_tolerance,                 \
                        mpfr::RoundingMode::TowardZero);                       \
    }                                                                          \
  }

#define TEST_MPFR_MATCH_ROUNDING_SILENTLY(op, input, match_value,              \
                                          ulp_tolerance, rounding)             \
  LIBC_NAMESPACE::testing::mpfr::get_silent_mpfr_matcher<op>(                  \
      input, match_value, ulp_tolerance, rounding)                             \
      .match(match_value)

#define ASSERT_MPFR_MATCH_DEFAULT(op, input, match_value, ulp_tolerance)       \
  ASSERT_THAT(match_value,                                                     \
              LIBC_NAMESPACE::testing::mpfr::get_mpfr_matcher<op>(             \
                  input, match_value, ulp_tolerance,                           \
                  LIBC_NAMESPACE::testing::mpfr::RoundingMode::Nearest))

#define ASSERT_MPFR_MATCH_ROUNDING(op, input, match_value, ulp_tolerance,      \
                                   rounding)                                   \
  ASSERT_THAT(match_value,                                                     \
              LIBC_NAMESPACE::testing::mpfr::get_mpfr_matcher<op>(             \
                  input, match_value, ulp_tolerance, rounding))

#define ASSERT_MPFR_MATCH(...)                                                 \
  GET_MPFR_MACRO(__VA_ARGS__, ASSERT_MPFR_MATCH_ROUNDING,                      \
                 ASSERT_MPFR_MATCH_DEFAULT, GET_MPFR_DUMMY_ARG)                \
  (__VA_ARGS__)

#define ASSERT_MPFR_MATCH_ALL_ROUNDING(op, input, match_value, ulp_tolerance)  \
  {                                                                            \
    namespace mpfr = LIBC_NAMESPACE::testing::mpfr;                            \
    mpfr::ForceRoundingMode __r1(mpfr::RoundingMode::Nearest);                 \
    if (__r1.success) {                                                        \
      ASSERT_MPFR_MATCH(op, input, match_value, ulp_tolerance,                 \
                        mpfr::RoundingMode::Nearest);                          \
    }                                                                          \
    mpfr::ForceRoundingMode __r2(mpfr::RoundingMode::Upward);                  \
    if (__r2.success) {                                                        \
      ASSERT_MPFR_MATCH(op, input, match_value, ulp_tolerance,                 \
                        mpfr::RoundingMode::Upward);                           \
    }                                                                          \
    mpfr::ForceRoundingMode __r3(mpfr::RoundingMode::Downward);                \
    if (__r3.success) {                                                        \
      ASSERT_MPFR_MATCH(op, input, match_value, ulp_tolerance,                 \
                        mpfr::RoundingMode::Downward);                         \
    }                                                                          \
    mpfr::ForceRoundingMode __r4(mpfr::RoundingMode::TowardZero);              \
    if (__r4.success) {                                                        \
      ASSERT_MPFR_MATCH(op, input, match_value, ulp_tolerance,                 \
                        mpfr::RoundingMode::TowardZero);                       \
    }                                                                          \
  }

#endif // LLVM_LIBC_UTILS_MPFRWRAPPER_MPFRUTILS_H
