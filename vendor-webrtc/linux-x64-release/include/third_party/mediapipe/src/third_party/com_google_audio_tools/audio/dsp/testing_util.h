/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef AUDIO_DSP_TESTING_UTIL_H_
#define AUDIO_DSP_TESTING_UTIL_H_

#include <complex>
#include <string>

#include "glog/logging.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/types/span.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// gMock's FloatEq() only handles real-valued types. This matcher works for both
// real and complex valued types. It applies FloatEq() separately to the real
// and imaginary parts of complex numbers. For real types, std::imag() returns
// zero, so imaginary parts always match.
MATCHER_P(ComplexFloatEq, expected, "is approximately " +
          testing::PrintToString(expected)) {
  ::testing::Matcher<float> real_part_matcher =
      ::testing::FloatEq(std::real(expected));
  ::testing::Matcher<float> imaginary_part_matcher =
      ::testing::FloatEq(std::imag(expected));
  return real_part_matcher.Matches(std::real(arg)) &&
      imaginary_part_matcher.Matches(std::imag(arg));
}

// Extension of gMock's DoubleEq to complex-valued types.
MATCHER_P(ComplexDoubleEq, expected, "is approximately " +
          testing::PrintToString(expected)) {
  ::testing::Matcher<double> real_part_matcher =
      ::testing::DoubleEq(std::real(expected));
  ::testing::Matcher<double> imaginary_part_matcher =
      ::testing::DoubleEq(std::imag(expected));
  return real_part_matcher.Matches(std::real(arg)) &&
      imaginary_part_matcher.Matches(std::imag(arg));
}

namespace internal {

MATCHER_P(TupleIsNear, tolerance, "is near") {
  return ::std::abs(::testing::get<0>(arg) - ::testing::get<1>(arg)) <=
         tolerance;
}

MATCHER(TupleFloatEq, "is almost equal to") {
  typedef decltype(::testing::get<1>(arg)) RhsType;
  ::testing::Matcher<RhsType> matcher =
      ComplexFloatEq(::testing::get<1>(arg));
  return matcher.Matches(::testing::get<0>(arg));
}

// Convert nested Span to a 2D Eigen Array. Spans are implicitly
// constructable from initializer_lists and vectors, so this conversion is used
// in EigenArrayNear and EigenArrayEq to support syntaxes like
// EXPECT_THAT(array2d, EigenArrayNear<int>({{1, 2}, {3, 4}}, tolerance);
// This conversion creates a copy of the slice data, so it is safe to use the
// result even after the original slices vanish.
template <typename T>
Eigen::Array<T, Eigen::Dynamic, Eigen::Dynamic>
EigenArray2DFromNestedSpans(
    absl::Span<const absl::Span<const T>> rows) {
  Eigen::Array<T, Eigen::Dynamic, Eigen::Dynamic> result(0, rows.size());
  if (!rows.empty()) {
    result.resize(rows.size(), rows[0].size());
  }
  for (int i = 0; i < rows.size(); ++i) {
    ABSL_CHECK_EQ(rows[0].size(), rows[i].size());
    result.row(i) = Eigen::Map<const Eigen::Array<T, Eigen::Dynamic, 1>>(
        &rows[i][0], rows[i].size());
  }
  return result;
}

// Get a matcher's description as a string. To produce the description for
// EigenEach(inner_matcher), this function is called to get the description of
// inner_matcher.
template <typename LhsType>
std::string GetMatcherDescriptionAsString(
    const ::testing::Matcher<LhsType>& matcher, bool negation) {
  ::std::stringstream ss;
  if (negation) {
    matcher.DescribeNegationTo(&ss);
  } else {
    matcher.DescribeTo(&ss);
  }
  return ss.str();
}

}  // namespace internal

// Defines a gMock matcher that tests whether two numeric arrays are
// approximately equal in the sense of maximum absolute difference. The element
// value type may be any type for which std::abs() is defined, including
// float, double, complex<float>, complex<double>, and integral types.
//
// Example:
// vector<double> output = ComputeVector();
// vector<double> expected({-1.5333, sqrt(2), M_PI});
// EXPECT_THAT(output, FloatArrayNear(expected, 1e-3));
template <typename ContainerType>
decltype(testing::Pointwise(internal::TupleIsNear(0.0), ContainerType()))
    FloatArrayNear(const ContainerType& container, double tolerance) {
  return testing::Pointwise(internal::TupleIsNear(tolerance), container);
}

// Defines a gMock matcher acting as an elementwise version of FloatEq() for
// arrays of real and complex floating point types. It tests whether two arrays
// are pointwise equal within 4 units in the last place (ULP) in float precision
// [http://en.wikipedia.org/wiki/Unit_in_the_last_place]. For complex types, the
// matcher checks that real components and imaginary components are each within
// 4 ULPs. Roughly, 4 ULPs is 2^-21 times the absolute value, or 0.00005%.
// Exceptionally, zero matches values with magnitude less than 5.6e-45 (2^-147),
// infinities match infinities of the same sign, and NaNs don't match anything.
//
// Example:
// vector<float> output = ComputeVector();
// vector<float> expected({-1.5333, sqrt(2), M_PI});
// EXPECT_THAT(output, FloatArrayEq(expected));
template <typename ContainerType>
decltype(testing::Pointwise(internal::TupleFloatEq(), ContainerType()))
    FloatArrayEq(const ContainerType& container) {
  return testing::Pointwise(internal::TupleFloatEq(), container);
}

// Call .eval() on input and convert it to a column major representation.
template <typename EigenType>
Eigen::Array<typename EigenType::Scalar, Eigen::Dynamic,
             Eigen::Dynamic, Eigen::ColMajor>
             EvalAsColMajorEigenArray(const EigenType& input) {
  return input.eval();
}

// Wrap a column major Eigen Array as a Span.
template <typename Scalar>
absl::Span<const Scalar> EigenArrayAsSpan(
    const Eigen::Array<Scalar, Eigen::Dynamic,
                       Eigen::Dynamic, Eigen::ColMajor>& array) {
  return absl::Span<const Scalar>(array.data(), array.size());
}

// Gmock matcher to test whether all elements in an array match expected_array
// within the specified tolerance, and print a detailed error message pointing
// to the first mismatched element if they do not.  Essentially an elementwise
// version of testing::DoubleNear for Eigen arrays.
//
// Example:
// Eigen::ArrayXf expected = ...
// EXPECT_THAT(actual_arrayxf, EigenArrayNear(expected, 1e-5));
MATCHER_P2(EigenArrayNear, expected_array, tolerance,
           "array is near " + ::testing::PrintToString(expected_array) +
               " within tolerance " + ::testing::PrintToString(tolerance)) {
  if (arg.rows() != expected_array.rows() ||
      arg.cols() != expected_array.cols()) {
    *result_listener << "where shape (" << expected_array.rows() << ", "
                     << expected_array.cols() << ") doesn't match ("
                     << arg.rows() << ", " << arg.cols() << ")";
    return false;
  }
  // Call .eval() to allow callers to pass in Eigen expressions and possibly
  // noncontiguous objects, e.g. Eigen::ArrayXf::Zero(10) or Map with a stride.
  // Arrays are represented in column major order for consistent comparison.
  auto realized_expected_array = EvalAsColMajorEigenArray(expected_array);
  auto realized_actual_array = EvalAsColMajorEigenArray(arg);
  return ExplainMatchResult(
      FloatArrayNear(EigenArrayAsSpan(realized_expected_array),
                     tolerance),
      EigenArrayAsSpan(realized_actual_array),
      result_listener);
}

// Gmock matcher to test whether all elements in an array match expected_array
// within 4 units of least precision (ULP) in float precision. Essentially an
// elementwise version of testing::FloatEq for Eigen arrays.
//
// Example:
// Eigen::ArrayXf expected = ...
// EXPECT_THAT(actual_arrayxf, EigenArrayEq(expected));
MATCHER_P(EigenArrayEq, expected_array,
          "array is almost equal to "
          + ::testing::PrintToString(expected_array)) {
  if (arg.rows() != expected_array.rows() ||
      arg.cols() != expected_array.cols()) {
    *result_listener << "where shape (" << expected_array.rows() << ", "
                     << expected_array.cols() << ") doesn't match ("
                     << arg.rows() << ", " << arg.cols() << ")";
    return false;
  }
  // Call .eval() to allow callers to pass in Eigen expressions and possibly
  // noncontiguous objects, e.g. Eigen::ArrayXf::Zero(10) or Map with a stride.
  // Arrays are represented in column major order for consistent comparison.
  auto realized_expected_array = EvalAsColMajorEigenArray(expected_array);
  auto realized_actual_array = EvalAsColMajorEigenArray(arg);
  return ExplainMatchResult(
      FloatArrayEq(EigenArrayAsSpan(realized_expected_array)),
      EigenArrayAsSpan(realized_actual_array),
      result_listener);
}

// The next few functions are syntactic sugar for EigenArrayNear and
// EigenArrayEq to allow callers to pass in non-Eigen types that can be
// statically initialized like (nested in the 2D case) initializer_lists, or
// vectors, etc. For example this specialization lets one make calls inlining
// expected_array like:
//   EXPECT_THAT(array1d, EigenArrayNear<float>({0.1, 0.2}, tolerance));
// or in the 2D case:
//   EXPECT_THAT(array2d, EigenArrayNear<int>({{1, 2}, {3, 4}}, tolerance);

template <typename T>
EigenArrayNearMatcherP2<Eigen::Array<T, Eigen::Dynamic, 1>, double>
EigenArrayNear(absl::Span<const T> data, double tolerance) {
  Eigen::Array<T, Eigen::Dynamic, 1> temp_array =
      Eigen::Map<const Eigen::Array<T, Eigen::Dynamic, 1>>(&data[0],
                                                           data.size());
  return EigenArrayNear(temp_array, tolerance);
}

template <typename T>
EigenArrayNearMatcherP2<Eigen::Array<T, Eigen::Dynamic, Eigen::Dynamic>, double>
EigenArrayNear(absl::Span<const absl::Span<const T>> rows,
               double tolerance) {
  return EigenArrayNear(internal::EigenArray2DFromNestedSpans(rows),
                        tolerance);
}

template <typename T>
EigenArrayEqMatcherP<Eigen::Array<T, Eigen::Dynamic, 1>> EigenArrayEq(
    absl::Span<const T> data) {
  Eigen::Array<T, Eigen::Dynamic, 1> temp_array =
      Eigen::Map<const Eigen::Array<T, Eigen::Dynamic, 1>>(&data[0],
                                                           data.size());
  return EigenArrayEq(temp_array);
}

template <typename T>
EigenArrayEqMatcherP<Eigen::Array<T, Eigen::Dynamic, Eigen::Dynamic>>
EigenArrayEq(absl::Span<const absl::Span<const T>> rows) {
  return EigenArrayEq(internal::EigenArray2DFromNestedSpans(rows));
}

// Defines a gMock matcher like Each that works on Eigen types having contiguous
// memory. EigenEach(inner_matcher) applies inner_matcher to each element, and
// matches if inner_matcher matches for all elements.
// NOTE: Like Each, EigenEach matches an empty array.
//
// Examples:
// EXPECT_THAT(arrayxf, EigenEach(Gt(0.0)));
// EXPECT_THAT(arrayxf, EigenEach(FloatEq(1.0)));
MATCHER_P(EigenEach, inner_matcher,
           (negation ? "contains some elements that " :
                       "only contains elements that ") +
           internal::GetMatcherDescriptionAsString(
               static_cast<::testing::Matcher<typename
               std::remove_reference<arg_type>::type::Scalar>>(inner_matcher),
               negation)) {
  // Call .eval() to allow callers to pass in Eigen expressions and possibly
  // noncontiguous objects.
  auto realized_actual_array = EvalAsColMajorEigenArray(arg);
  return ExplainMatchResult(
      ::testing::Each(inner_matcher),
      EigenArrayAsSpan(realized_actual_array),
      result_listener);
}

}  // namespace audio_dsp

namespace Eigen {
template <typename Scalar, int Rows, int Cols, int Options, int MaxRows,
          int MaxCols>
void PrintTo(const Array<Scalar, Rows, Cols, Options, MaxRows, MaxCols>& array,
    ::std::ostream* os) {
  IOFormat format(StreamPrecision, 0, ", ", ",\n", "[", "]", "[", "]");
  *os << "\n" << array.format(format);
}
}  // namespace Eigen

#endif  // AUDIO_DSP_TESTING_UTIL_H_
