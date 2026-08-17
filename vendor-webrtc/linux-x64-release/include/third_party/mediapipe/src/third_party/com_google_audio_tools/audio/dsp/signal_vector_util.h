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

// Functions for operating on 1D (like audio) signals represented as vectors
// or other compatible container types.

#ifndef AUDIO_DSP_SIGNAL_VECTOR_UTIL_H_
#define AUDIO_DSP_SIGNAL_VECTOR_UTIL_H_

#include <algorithm>
#include <cmath>
#include <complex>
#include <limits>
#include <numeric>
#include <type_traits>
#include <vector>

#include "audio/dsp/types.h"
#include "glog/logging.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// These functions were originally written to work on vectors of float or
// doubles, but have been templated to work on more general containers and
// more general value_type.  For some a Forward Container is enough; some
// require Reversible Container.  Complex value_type should work where it
// makes sense.  Limitations of applicability should show up at compile time.

template <typename ValueType>
constexpr inline ValueType Square(const ValueType& x) {
  return x * x;
}

template <typename ValueType>
constexpr inline typename RealType<ValueType>::Type SquareMagnitude(
    const ValueType& z) {
  // For real ValueType the compiler will optimize out the second term as zero.
  return Square(std::real(z)) + Square(std::imag(z));
}

template <typename ValueType, typename ContainerType>
void MultiplyVectorByScalar(ValueType multiplier, ContainerType* target) {
  ABSL_DCHECK(target);
  for (auto& x : *target) {
    x *= multiplier;
  }
}

template <typename ContainerType>
typename ContainerType::value_type Sum(const ContainerType& in) {
  typedef typename ContainerType::value_type ValueType;
  return std::accumulate(in.cbegin(), in.cend(), ValueType());
}

// Compute mean and variance of a container of real or complex-valued elements.
// The container must be nonempty. For a container of size N with real-valued
// elements (x_n), the mean and variance are defined as
//   mean = sum_n x_n / N,
//   variance = sum_n (x_n - mean)^2 / N.
// The variance is computed in one pass from the first and second moments as
// variance = (sum_n x_n^2) / N - mean^2. For complex-valued elements (z_n),
// the mean and variance are defined as
//   mean = sum_n z_n / N,
//   variance = sum_n |z_n - mean|^2 / N,
// where |.| denotes complex magnitude. Similar to the real case, variance is
// computed as variance = (sum_n |x_n|^2) / N - |mean|^2.
//
// Statistics are computed in a single pass through the data using by default
// double-precision accumulators, which is adequate for almost all signal
// processing applications, but perhaps not for extremely large sample sizes
// or when the variance is small compared to the square of the mean.
// TODO: Experiments on accuracy vs. sample size and magnitudes.
// TODO: Implement and compare with the assumed mean algorithm.
//
// WARNING: The single-pass algorithm used here is numerically unreliable in
// general. Do not use for large samples or when variance is small compared to
// the square of the mean. For example, the 2-element array {1 - epsilon, 1} has
//   mean^2 = 1 - epsilon + epsilon^2 / 4,
//   variance = epsilon^2 / 4,
// which for epsilon << 1 differ substantially in magnitude. For this data, the
// single-pass algorithm subtracts two nearly equal quantities,
//   (second moment) - mean^2
//   = (1 - epsilon + epsilon^2 / 2) - (1 - epsilon + epsilon^2 / 4),
// resulting in a large loss of significance.
//
// It is recommended to use the default AccumulatorType of double or
// complex<double> so that accumulation is in double precision. Single precision
// can be used by setting AccumulatorType to float or complex<float>.
template <typename ContainerType, typename AccumulatorType =
    // Set the default type for the accumulator based on whether the container
    // has complex-valued elements.
    typename std::conditional<HoldsComplex<ContainerType>::Value,
      // If elements are complex, use complex<double>.
      std::complex<double>,
      // Otherwise if elements are real, use double.
      double>::type>
void ComputeSecondOrderStats(
    const ContainerType& in, AccumulatorType* mean,
    typename RealType<AccumulatorType>::Type* variance) {
  static_assert(
      static_cast<bool>(HoldsComplex<ContainerType>::Value) ==
      static_cast<bool>(IsComplex<AccumulatorType>::Value),
      "complexness of elements and AccumulatorType must match");
  ABSL_DCHECK(mean);
  ABSL_DCHECK(variance);
  typedef typename RealType<AccumulatorType>::Type RealAccumulatorType;
  // Use RealAccumulatorType for the normalization below, since the division
  // complex<double> / size_t is undefined.
  const RealAccumulatorType num_samples = in.size();
  ABSL_CHECK_GT(num_samples, 0);
  AccumulatorType first_sum = 0;
  RealAccumulatorType second_sum = 0;
  for (const auto& value : in) {
    const AccumulatorType value_accumulator_type(value);
    first_sum += value_accumulator_type;
    second_sum += SquareMagnitude(value_accumulator_type);
  }
  *mean = first_sum / num_samples;
  *variance = second_sum / num_samples - SquareMagnitude(*mean);
}

template <typename ContainerType>
double SumSquaredMagnitudes(const ContainerType& in) {
  double total = 0.0;
  for (const auto& x : in) {
    double real_part = std::real(x);
    // For real value_type the compiler will recognize this as zero and skip it.
    double imag_part = std::imag(x);
    total += real_part * real_part + imag_part * imag_part;
  }
  return total;
}

// Compute the maximum magnitude (L^infinity norm) of a container. The return
// type is double for containers of integral or complex-valued elements.
// Otherwise, the return type is ContainerType::value_type.
template <typename ContainerType>  // Implementation for real-valued data.
typename std::enable_if<!HoldsComplex<ContainerType>::Value,
    typename FloatPromotion<typename ContainerType::value_type>::Type>::type
    ComputeMaxMagnitude(const ContainerType& in) {
  if (in.empty()) {
    return 0.0;
  }
  // Negation of INT_MIN has undefined behavior since the result cannot be
  // represented as an int, and similarly for other integral types. Therefore,
  // we promote integral-typed values to double before negating. Floating point
  // types are not promoted.
  typedef typename FloatPromotion<
      typename ContainerType::value_type>::Type PromotedType;
  // TODO: Reduce the number of comparisons in finding the min and max
  // from 2N to 3N/2 by processing pairs of elements at a time.
  return std::max(
      -static_cast<PromotedType>(*std::min_element(in.cbegin(), in.cend())),
      static_cast<PromotedType>(*std::max_element(in.cbegin(), in.cend())));
}
template <typename ContainerType>  // Implementation for complex-valued data.
typename std::enable_if<HoldsComplex<ContainerType>::Value, double>::type
    ComputeMaxMagnitude(const ContainerType& in) {
  double max_square_magnitude = 0.0;
  // Find the largest square magnitude element value.
  for (const auto& x : in) {
    double real_part = std::real(x);
    double imag_part = std::imag(x);
    max_square_magnitude = std::max(
        max_square_magnitude, real_part * real_part + imag_part * imag_part);
  }
  return std::sqrt(max_square_magnitude);
}

// target[i] += addend[i].
template <typename ValueType>
void AddVectorIntoVector(const std::vector<ValueType>& addend,
                         std::vector<ValueType>* target) {
  ABSL_DCHECK(target);
  const int n = target->size();
  ABSL_DCHECK_EQ(n, addend.size());
  for (int i = 0; i < n; ++i) {
    (*target)[i] += addend[i];
  }
}

// target[i] += weight * addend[i].
template <typename ValueType>
void AddWeightedVectorIntoVector(const std::vector<ValueType>& addend,
                                 float weight,
                                 std::vector<ValueType>* target) {
  ABSL_DCHECK(target);
  const int n = target->size();
  ABSL_DCHECK_EQ(n, addend.size());
  for (int i = 0; i < n; ++i) {
    (*target)[i] += weight * addend[i];
  }
}

// target[i] *= multiplier[i].
// TODO: Consider renaming this function.
template <typename ContainerType1, typename ContainerType2>
void MultiplyVector(const ContainerType1& multiplier,
                    ContainerType2* target) {
  ABSL_DCHECK(target);
  const int n = target->size();
  ABSL_DCHECK_EQ(n, multiplier.size());
  for (int i = 0; i < n; ++i) {
    (*target)[i] *= multiplier[i];
  }
}

// target[i] *= conj(other[i]).
template <typename ValueType>
void MultiplyComplexVectorByConjugate(
    const std::vector<std::complex<ValueType>>& other,
    std::vector<std::complex<ValueType>>* target) {
  ABSL_DCHECK(target);
  const int n = target->size();
  ABSL_DCHECK_EQ(n, other.size());
  for (int i = 0; i < n; ++i) {
    // NOTE: c1 *= conj(c2) is net 20% slower, due to complex over-engineering.
    // In --copt="-ffast-math" mode, both are equally slow, not much faster.
    // TODO: Make benchmark and report as compiler bug.
    std::complex<ValueType>& c1 = (*target)[i];
    const std::complex<ValueType>& c2 = other[i];
    ValueType r1 = c1.real();
    ValueType i1 = c1.imag();
    ValueType r2 = c2.real();
    ValueType i2 = c2.imag();
    c1 = std::complex<ValueType>(r1 * r2 + i1 * i2, r2 * i1 - r1 * i2);
  }
}

// Zero pads the shorter input to match the size of the longer one.
template <typename ContainerType1, typename ContainerType2>
void MakeTheSameSize(ContainerType1* in1, ContainerType2* in2) {
  ABSL_CHECK(in1);
  ABSL_CHECK(in2);
  if (in1->size() > in2->size()) {
    in2->resize(in1->size());
  } else {
    in1->resize(in2->size());
  }
}

// Copies in to out, zero padding to length output_size.
template <typename ContainerType1, typename ContainerType2>
void ZeroPad(const ContainerType1& in, ContainerType2* out, int output_size) {
  typedef typename ContainerType2::value_type ValueType;
  ABSL_DCHECK(out);
  ABSL_DCHECK(output_size >= in.size());
  out->assign(in.cbegin(), in.cend());
  out->resize(output_size, ValueType(0));
}

// Appends suffix to container v.
template <typename ContainerType1, typename ContainerType2>
void VectorAppend(ContainerType1* v, const ContainerType2& suffix) {
  static_assert(std::is_same<typename ContainerType1::value_type,
                typename ContainerType2::value_type>::value,
                "Element types must be the same.");
  v->insert(v->end(), suffix.begin(), suffix.end());
}

// Computes the root mean square error between two real-valued vectors.
// Dies if the vectors have different lengths.
template <typename ContainerType1, typename ContainerType2>
double RootMeanSquareError(const ContainerType1& in1,
                           const ContainerType2& in2) {
  int n_elements = in1.size();
  ABSL_CHECK_EQ(n_elements, in2.size());
  double sum_squared_error = 0.0;
  auto it1 = in1.begin();
  auto it2 = in2.begin();
  for (int i = 0; i < n_elements; ++i) {
    // Promote to double before taking the difference.
    auto val1 = *it1++;
    auto val2 = *it2++;
    double difference_real = (static_cast<double>(std::real(val1)) -
                              std::real(val2));
    // For real args, the compiler will recognize this as zero and skip it.
    double difference_imag = (static_cast<double>(std::imag(val1)) -
                              std::imag(val2));
    sum_squared_error += (difference_real * difference_real +
                          difference_imag * difference_imag);
  }
  return std::sqrt(sum_squared_error / n_elements);
}

// This function is not for complex value_type, since values need to
// be ordered to find a max.  The container needs to be a Random
// Access Container for the index calculation by iterator subtraction
// to make sense.
template <typename ContainerType, typename SampleType>
void FindMaxIndexAndValue(const ContainerType& container,
                          int* max_index, SampleType* max_value) {
  ABSL_DCHECK(max_index && max_value);
  ABSL_CHECK(!container.empty());
  auto max_iterator = std::max_element(container.begin(), container.end());
  *max_value = *max_iterator;
  *max_index = max_iterator - container.begin();
}

// Given two containers of the same size of real-valued elements, update *target
// so that it is the elementwise max.
template <typename ContainerType1, typename ContainerType2>
void ElementwiseMax(const ContainerType1& other, ContainerType2* target) {
  ABSL_DCHECK(target);
  ABSL_CHECK_EQ(other.size(), target->size());
  auto it_target = target->begin();
  for (const auto& other_val : other) {
    if (other_val > *it_target) *it_target = other_val;
    ++it_target;
  }
}

// Normalize a container of values as z-scores based on the mean and standard
// deviation over the container, (value - mean) / (stddev_floor + stddev).
template <typename ContainerType>
void ZScoreNormalize(double stddev_floor, ContainerType* v) {
  ABSL_DCHECK(v);
  if (!v->empty()) {
    // Get the float type compatible with the container elements.
    typedef typename FloatPromotion<
        typename ContainerType::value_type>::Type FloatType;
    // Get the real type associated with FloatType.
    typedef typename RealType<FloatType>::Type RealFloatType;

    typename std::conditional<
        HoldsComplex<ContainerType>::Value,
          std::complex<double>,
          double>::type mean;
    double variance;
    ComputeSecondOrderStats(*v, &mean, &variance);
    const FloatType mean_float_type = mean;
    const RealFloatType denom = stddev_floor + std::sqrt(variance);
    for (auto& value : *v) {
      value = (value - mean_float_type) / denom;
    }
  }
}

// Computes the coefficient used by the smoothers, where scale specifies the
// standard deviation in units of samples of the approximately-Gaussian impulse
// response.
//
// A description of the filter strategy and coefficient formula is at
// http://en.wikipedia.org/wiki/Scale_space_implementation#Recursive_filters
// The 2/t is unlike the 1/t in dicklyon's matlab smooth1d.m, because
// this coefficient is for a 4-pass version compared to that 2-pass
// version.  With four passes, the corresponding continuous-time
// impulse response has continuous first and second derivatives,
// unlike the 2-pass or double- exponential smoothing filter, whose
// impulse repsonse has a cusp (a discontinuity of first derivative)
// at the time origin.  The more smooth shape of the 4-pass smoother
// makes it more "Gaussian-like".
inline float SmootherCoefficientFromScale(float scale) {
  if (scale <= 0.01) return 1.0;  // Negligible smoothing requested.
  const float t = scale * scale;  // Kernel variance, TP Lindeberg's t notation.
  const float coefficient = std::sqrt(Square(1.0 + 2.0/t) - 1.0) - 2.0/t;
  return coefficient;
}

// Except for ForwardSmoothVector, which is a "causal" smoothing filter, the
// smoothing functions all require Reversible Containers.  They work on real
// and complex value_types.

template <typename ContainerType, typename SampleType>
void ForwardSmoothVector(float coefficient, SampleType* state,
                         ContainerType* signal) {
  ABSL_DCHECK(state && signal);
  SampleType local_state = *state;
  for (auto it = signal->begin(); it != signal->end(); ++it) {
    local_state += coefficient * (*it - local_state);
    *it = local_state;
  }
  *state = local_state;
}

template <typename ContainerType, typename SampleType>
void BackwardSmoothVector(float coefficient, SampleType* state,
                          ContainerType* signal) {
  ABSL_DCHECK(state && signal);
  SampleType local_state = *state;
  for (auto it = signal->rbegin(); it != signal->rend(); ++it) {  // Reversed.
    local_state += coefficient * (*it - local_state);
    *it = local_state;
  }
  *state = local_state;
}

// A Gaussian-like smoother, made by cascading four one-pole smoothers, two
// in forward direction and two backward, for net zero phase.
template <typename ContainerType>
void SmoothVector(float coefficient, ContainerType* signal) {
  ABSL_DCHECK(signal);
  // Two passes, each a forward and a backward one-pole smoother.
  auto state = *(signal->begin());
  for (int count = 0; count < 2; ++count) {
    state *= (1.0f - coefficient);  // A compromise starting edge state.
    ForwardSmoothVector(coefficient, &state, signal);
    state *= (1.0f - coefficient);  // A compromise ending edge state.
    BackwardSmoothVector(coefficient, &state, signal);
  }
}

namespace internal {
// Wrapper around std::conj that should always return a real
// value if the input is real.
template <typename ScalarType>
std::complex<ScalarType> Conjugate(std::complex<ScalarType> in) {
  return std::conj(in);
}
template <typename ScalarType>
ScalarType Conjugate(ScalarType in) {
  return in;
}
}  // namespace internal

// Smooth a vector that represents half of a discrete Fourier transform, with
// circular Hermitian symmetry, whether real or complex.  The coefficient is
// typically derived from scale (the intended smoothing standard deviation in
// samples) via SmootherCoefficientFromScale; both scale and coefficient
// must be non-negative.
template <typename SampleType>  // Typically float or complex<float>.
void HermitianSmoothVector(float coefficient, float scale,
                           std::vector<SampleType>* signal) {
  ABSL_CHECK(signal);
  ABSL_CHECK_GE(scale, 0);
  ABSL_CHECK_GE(coefficient, 0);
  const int n = signal->size();
  if (n < 2) return;
  // Number of values to flip above end for filtering continuity.
  int extra_n = std::floor(3 * (scale + 1));  // Estimate; at least 3.
  if (extra_n >= n) {
    extra_n = n - 1;  // Reflect all but last value above the last value.
  }
  std::vector<SampleType> extra_vals(extra_n);
  // Fill in extra_vals by reflecting end, with Hermitian symmetry.
  auto signal_iter = signal->rbegin();
  for (auto& v : extra_vals) {
    v = internal::Conjugate(
        *(++signal_iter));  // Preincrement; skips last element.
  }
  // Two forward/backward passes makes it more Gaussian-like.
  for (int count = 0; count < 2; ++count) {
    SampleType average = 0.0;  // Filter state variable, exponential average.
    // Backward state initialization for forward pass.
    for (auto signal_it = signal->rend() - (extra_n + 1);
         signal_it != signal->rend() - 1; ++signal_it) {
      average += coefficient * (*signal_it - average);
    }
    average = internal::Conjugate(average);  // Adjust state for start.
    ForwardSmoothVector(coefficient, &average, signal);
    // Keep going forward through the extra values.
    ForwardSmoothVector(coefficient, &average, &extra_vals);
    // Backward pass, extras first to get to good state.
    average = 0.0;  // It's sort of arbitrary what to do here after the extras.
    BackwardSmoothVector(coefficient, &average, &extra_vals);
    // Backward pass continues with newly perfected state.
    BackwardSmoothVector(coefficient, &average, signal);
  }
  // Keep endpoints real to enforce Hermitian symmetry and keep the inverse
  // transform real.  They will be very close to real already.
  *(signal->begin()) = std::real(*(signal->begin()));
  *(signal->rbegin()) = std::real(*(signal->rbegin()));
}

// This function performs a "DC-blocking" or "AC-coupling" filter, which is
// just the original signal minus a first-order forward-smoothed signal, with
// a few ms smoothing time constant (like 0.002 second for a 500 radian/s
// or 80 Hz corner frequency below which there is a 6 dB/octave rolloff).
template <typename ContainerType>
void BlockDC(float scale_samples, ContainerType* signal) {
  ABSL_DCHECK(signal);
  typedef typename ContainerType::value_type ValueType;
  ValueType local_state = ValueType();  // Zero.
  float coefficient = 1.0 / scale_samples;
  for (auto it = signal->begin(); it != signal->end(); ++it) {
    local_state += coefficient * (*it - local_state);
    *it -= local_state;  // Like forward smoother, but -= instead of =.
  }
}

// Interpolates a quadratic polynomial about index x0 and its two neighbors
// and finds the maximum of this interpolant. This allows us to refine a
// discrete maximum location x0 to subsample precision. The returned value is
// the interpolated maximum value and *max_location is the interpolated
// location. It is guaranteed that 0 <= *max_location <= signal.size() - 1 and
// |x0 - *max_location| <= 1.
template <typename ContainerType>
float QuadraticInterpolateMax(const ContainerType& signal, int x0,
                              float *max_location) {
  ABSL_CHECK_GE(x0, 0);
  ABSL_CHECK_LT(x0, signal.size());
  ABSL_CHECK(max_location);
  if (x0 == 0 || x0 == signal.size() - 1) {
    *max_location = x0;
    return signal[x0];
  }
  const float kTolerance = 4 * std::numeric_limits<float>::epsilon();
  // Construct polynomial P(x) = signal[x0] + b (x - x0) + a (x - x0)^2
  // such that P interpolates signal at {x0 - 1, x0, x0 + 1}.
  float a = (signal[x0 - 1] - 2 * signal[x0] + signal[x0 + 1]) / 2.0f;
  float b = (signal[x0 + 1] - signal[x0 - 1]) / 2.0f;
  // If polynomial is nearly symmetric about x0, just return x0.
  if (fabs(b) < kTolerance) {
    *max_location = x0;
    return signal[x0];
  }
  // Find the maximum based on the local concavity of the signal. If
  // signal[x0] > max(signal[x0 - 1], signal[x0 + 1]), then a < 0.
  if (a < 0.0f) {
    // Find x such that P'(x) = 0 and clamp it to [x0 - 1, x0 + 1], i.e.,
    // don't extrapolate.
    // NOTE: If signal[x0] > max(signal[x0 - 1], signal[x0 + 1]), then
    // 2 signal[x0] > signal[x0 - 1] + signal[x0 + 1]
    //                + |signal[x0 - 1] - signal[x0 + 1]| / 2,
    // which implies that -2 a > |b| and clamping has no effect.
    float delta = std::max(-1.0f, std::min(-b / (2 * a), 1.0f));
    *max_location = x0 + delta;
    return signal[x0] + (b + a * delta) * delta;
  } else {
    // signal is locally convex, select the larger endpoint.
    x0 += std::copysign(1, b);
    *max_location = x0;
    return signal[x0];
  }
}

// Compute the power of a complex vector.  InputValueType must be a
// complex type.  OutputValueType may be either a real or complex
// type.  It's fine if the input and output are the same vector.
template <typename InputValueType, typename OutputValueType>
void ConvertComplexVectorToPower(
    const std::vector<InputValueType>& complex_vector,
    std::vector<OutputValueType>* real_or_complex_power) {
  ABSL_DCHECK(real_or_complex_power);
  const int n = complex_vector.size();
  real_or_complex_power->resize(n);
  for (int i = 0; i < n; ++i) {
    // Because norm is known to be slow, handle real and imag explicitly.
    double re = complex_vector[i].real();
    double im = complex_vector[i].imag();
    (*real_or_complex_power)[i] = re * re + im * im;
  }
}

// Compute the dot product of a pair of real vectors.  Note that if
// either vector is complex, this won't compile.  Integer types will
// work, although the computations will be performed in double
// precision.
template <typename ContainerType1, typename ContainerType2>
double RealDotProduct(
    const ContainerType1& in1, const ContainerType2& in2) {
  int n_elements = in1.size();
  ABSL_CHECK_EQ(n_elements, in2.size());
  double dot_product = 0.0;
  for (int i = 0; i < n_elements; ++i) {
    dot_product += in1[i] * static_cast<double>(in2[i]);
  }
  return dot_product;
}

// Compute the dot product of a pair of complex vectors where the
// conjugation is on the second vector.
template <typename ContainerType1, typename ContainerType2>
typename ContainerType1::value_type ComplexDotProduct(
    const ContainerType1& in1, const ContainerType2& in2) {
  int n_elements = in1.size();
  ABSL_CHECK_EQ(n_elements, in2.size());
  typename ContainerType1::value_type dot_product = 0;
  for (int i = 0; i < n_elements; ++i) {
    dot_product += in1[i] *
        static_cast<typename ContainerType1::value_type>(std::conj(in2[i]));
  }
  return dot_product;
}

// Compute the dot product of a complex vector in1 with a real vector in2.
template <typename ContainerType1, typename ContainerType2>
typename ContainerType1::value_type ComplexRealDotProduct(
    const ContainerType1& in1, const ContainerType2& in2) {
  int n_elements = in1.size();
  ABSL_CHECK_EQ(n_elements, in2.size());
  typename ContainerType1::value_type dot_product = 0;
  for (int i = 0; i < n_elements; ++i) {
    dot_product += in1[i] *
        static_cast<typename ContainerType1::value_type::value_type>(in2[i]);
  }
  return dot_product;
}

// Compute a real elementwise dot product out = in1.*in2.  Note that
// if either input vector is complex, this won't compile.  Integer
// types will work, although the computations will be performed in
// double precision, so it's not maximally efficient for floats or
// integers.  in1, in2 and out need not be distinct.
template <
  typename InContainerType1, typename InContainerType2,
  typename OutContainerType>
void RealElementwiseProduct(
    const InContainerType1& in1, const InContainerType2& in2,
    OutContainerType* out) {
  ABSL_DCHECK(out);
  int n_elements = in1.size();
  ABSL_CHECK_EQ(n_elements, in2.size());
  out->resize(n_elements);
  for (int i = 0; i < n_elements; ++i) {
    (*out)[i] = static_cast<double>(in1[i]) * static_cast<double>(in2[i]);
  }
}

// Compute a complex elementwise dot product out = in1.*conj(in2).
// This method will work for real or integer inputs, but all values
// are cast to complex<double> before multiplying, so it's not
// maximally efficient for integers, floats, doubles, or
// complex<float>s.  in1, in2 and out need not be distinct.  Note that
// this function is in general not symmetric in its inputs.
template <
  typename InContainerType1, typename InContainerType2,
  typename OutContainerType>
void ComplexElementwiseProduct(
    const InContainerType1& in1, const InContainerType2& in2,
    OutContainerType* out) {
  ABSL_DCHECK(out);
  int n_elements = in1.size();
  ABSL_CHECK_EQ(n_elements, in2.size());
  out->resize(n_elements);
  for (int i = 0; i < n_elements; ++i) {
    (*out)[i] = static_cast<std::complex<double> >(in1[i]) *
        std::conj(static_cast<std::complex<double> >(in2[i]));
  }
}

template<typename ContainerType1, typename ContainerType2>
void GetRealPartOfVector(const ContainerType1& in,
                         ContainerType2* out) {
  ABSL_DCHECK(out);
  out->clear();
  out->reserve(in.size());
  for (const auto& element : in) {
    out->push_back(element.real());
  }
}

template <typename ContainerType>
void ComputePhasorVector(double phase_step, int num_samples,
                         ContainerType* signal) {
  ABSL_DCHECK(signal != nullptr);
  signal->resize(num_samples);
  const std::complex<double> rotator = std::exp(
      std::complex<double>(0, phase_step));
  std::complex<double> phasor = std::complex<double>(1, 0);
  // Repeatedly multiply phasor by rotator to compute {rotator^0, rotator^1,
  // ..., rotator^n, ...} so that the nth element is exp(i radians * n).
  for (auto& element : *signal) {
    element = static_cast<typename ContainerType::value_type>(phasor);
    phasor *= rotator;
  }
}

// Fills the provided 'signal' container with 'num_samples' samples of a sine
// wave with the provided frequency (Hz), sample rate (Hz) and phase (radians).
template <typename ContainerType>
void ComputeSineWaveVector(const double frequency_hz,
                           const double sample_rate_hz,
                           const double phase_radians,
                           const size_t num_samples,
                           ContainerType* signal) {
  ABSL_DCHECK(signal != nullptr);
  signal->resize(num_samples);
  const double phase_step = 2 * M_PI * frequency_hz / sample_rate_hz;
  const std::complex<double> rotator = std::exp(
      std::complex<double>(0, phase_step));
  std::complex<double> phasor =
      std::exp(std::complex<double>(0, phase_radians));
  // Repeatedly multiply phasor by rotator to compute {rotator^0, rotator^1,
  // ..., rotator^n, ...} so that the nth element is
  // exp(i (radians * n + phase_radians)).
  for (auto& element : *signal) {
    element = static_cast<typename ContainerType::value_type>(phasor.imag());
    phasor *= rotator;
  }
}

// Split a complex vector into a vector of real parts and a vector
// of imaginary parts.
template <typename ValueType>
void SplitComplexVectorIntoRealAndImaginaryParts(
    const std::vector<std::complex<ValueType>>& complex_vector,
    std::vector<ValueType>* real_part,
    std::vector<ValueType>* imaginary_part) {
  real_part->clear();
  imaginary_part->clear();
  real_part->reserve(complex_vector.size());
  imaginary_part->reserve(complex_vector.size());
  for (const auto& sample : complex_vector) {
    real_part->push_back(sample.real());
    imaginary_part->push_back(sample.imag());
  }
}

// The inverse of SplitComplexVector: combines a vector of real parts and a
// vector of imaginary parts into a complex vector.
template <typename ValueType>
void CombineRealAndImaginaryPartsIntoComplexVector(
    const std::vector<ValueType>& real_part,
    const std::vector<ValueType>& imaginary_part,
    std::vector<std::complex<ValueType>>* combined) {
  const int size = real_part.size();
  ABSL_CHECK_EQ(size, imaginary_part.size());
  combined->clear();
  combined->reserve(size);
  for (int i = 0; i < size; ++i) {
    combined->push_back(std::complex<ValueType>(real_part[i],
                                                imaginary_part[i]));
  }
}

// Compute the circular or angular average of a set of values modulo a specified
// period [http://en.wikipedia.org/wiki/Mean_of_circular_quantities]. The result
// is a value in [0, period]. The input container must be nonempty and the
// period must be positive.
//
// NOTE: This function is discontinuous. A small change in the input values can
// cause a jump in the output between 0 and period. This is an unavoidable
// artifact of representing angles on the real line, the circular topology has
// to be cut somewhere. Calculations with angles should take wrapping into
// account, e.g., instead of comparing angles with arithmetic difference, use
// the angular difference [http://stackoverflow.com/a/7869457].
template <typename ContainerType>
float ComputeCircularAverage(const ContainerType& values, float period) {
  static_assert(static_cast<bool>(HoldsComplex<ContainerType>::Value) == false,
      "elements must be real");
  ABSL_CHECK(!values.empty());
  ABSL_CHECK_GT(period, 0.0);
  const float radians_factor = (2 * M_PI) / period;
  std::complex<float> sum = 0;
  for (const auto& value : values) {
    const float phase = radians_factor * value;
    sum += std::polar(1.0f, phase);
  }
  // Compute the circular average and convert from radians to cycles. The value
  // is in the range [-0.5, 0.5].
  float average_phase = std::arg(sum) / (2 * M_PI);
  if (average_phase < 0.0f) {  // Adjust range to [0, 1].
    average_phase += 1.0f;
  }
  // Convert from cycles to the original units.
  average_phase *= period;
  return average_phase;
}

// Compute the weighted circular average of a set of values modulo a specified
// period. Same as ComputeCircularAverage, but with weights on the values.
template <typename ContainerType1, typename ContainerType2>
float ComputeWeightedCircularAverage(const ContainerType1& values,
                                     const ContainerType2& weights,
                                     float period) {
  static_assert(static_cast<bool>(HoldsComplex<ContainerType1>::Value) == false,
      "values must be real");
  static_assert(static_cast<bool>(HoldsComplex<ContainerType2>::Value) == false,
      "weights must be real");
  ABSL_CHECK(!values.empty());
  ABSL_CHECK_EQ(values.size(), weights.size());
  ABSL_CHECK_GT(period, 0.0);
  const float radians_factor = (2 * M_PI) / period;
  std::complex<float> sum = 0;
  auto weights_it = weights.cbegin();
  for (const auto& value : values) {
    const float phase = radians_factor * value;
    sum += std::polar<float>(*weights_it, phase);
    ++weights_it;
  }
  float average = std::arg(sum) / (2 * M_PI);
  if (average < 0.0) {
    average += 1.0;
  }
  average *= period;
  return average;
}

// Class template for causal Finite Impulse Response (FIR) filtering, with
// filter coefficients of datatype CoefficientType and internal state of
// datatype StateType. The filter remembers an internal state so that filtering
// is streamable.
template <typename CoefficientType, typename StateType>
class FIRFilter {
 public:
  // Construct an FIR filter with impulse response impulse_response and an
  // initial state vector of zero.
  // NOTE: impulse_response must not be empty.
  explicit FIRFilter(const std::vector<CoefficientType>& impulse_response)
      : impulse_response_(impulse_response) {
    ABSL_CHECK(!impulse_response_.empty());
    Reset();
  }

  // Get the filter's impulse response.
  const std::vector<CoefficientType>& GetImpulseResponse() const {
    return impulse_response_;
  }

  // Get the filter's current state.
  const std::vector<StateType>& GetState() const {
    return state_;
  }

  // Reset the state of the filter to zero. This is equivalent to zero padded
  // boundary handling.
  void Reset() {
    state_.assign(impulse_response_.size() - 1, StateType(0));
  }

  // Apply FIR filter to signal,
  //
  //    result[i] = signal[i] * impulse_response[0] +
  //                signal[i - 1] * impulse_response[1] + ...
  //                signal[i - n] * impulse_response[n].
  //
  // The result has the same size as signal. In-place computation is allowed
  // with result = &signal. This function is streamable, meaning that filtering
  // can be performed in chunks. The input should be passed in non-overlapping
  // chunks. There is no filtering artifact around chunk boundaries because the
  // filter remembers previous samples through its internal state vector. For
  // example,
  //
  //    FIRFilter<T,T> filter(h);
  //    vector<T> x(x1);
  //    x.insert(x.end(), x2.begin(), x2.end());  // Concatenate x = [x1, x2].
  //    vectior<T> y;
  //    filter.Filter(x, &y);  // y = h * x.
  //
  // is equivalent to
  //
  //    FIRFilter<T,T> filter(h);
  //    vector<T> y1;
  //    filter.Filter(x1, &y1);  // Filter first chunk, y1 = h * x1.
  //    vector<T> y2;
  //    filter.Filter(x2, &y2);  // Filter second chunk, y2 = h * x2.
  //    vector<T> y(y1);
  //    y.insert(y.end(), y2.begin(), y2.end());  // Concatenate y = [y1, y2].
  //
  // (Multiplication of ContainerType1::value_type and CoefficientType muat be
  // defined.)
  template <typename ContainerType1, typename ContainerType2>
  void Filter(const ContainerType1& signal, ContainerType2* result) {
    ABSL_CHECK(result);
    result->resize(signal.size());
    if (impulse_response_.size() > 1) {
      for (int i = 0; i < signal.size(); ++i) {
        const typename ContainerType1::value_type sample = signal[i];
        (*result)[i] = sample * impulse_response_[0] + state_[0];
        for (int k = 1; k < state_.size(); ++k) {
          state_[k - 1] = sample * impulse_response_[k] + state_[k];
        }
        state_.back() = sample * impulse_response_.back();
      }
    } else {  // Special case for an order zero filter.
      for (int i = 0; i < signal.size(); ++i) {
        (*result)[i] = signal[i] * impulse_response_[0];
      }
    }
  }

 private:
  const std::vector<CoefficientType> impulse_response_;
  std::vector<StateType> state_;
};

// Design an FIR lowpass filter by computing the sinc impulse response of the
// ideal brickwall filter and windowing it with the given window,
//   filter[i] = window[i] * factor * sinc(pi * factor * (i - radius)),
// where sinc(x) := sin(x) / x, radius = (window.size() - 1) / 2, and
// factor = 2 * high_band_edge / sample_rate. The filter has zero phase provided
// that the window is symmetric.
// NOTE: The window must have odd size.
template <typename ContainerType>
std::vector<typename ContainerType::value_type> DesignFirLowpassFilter(
    float high_band_edge, float sample_rate, const ContainerType& window) {
  typedef typename ContainerType::value_type ValueType;
  ABSL_CHECK_GE(high_band_edge, 0.0);
  ABSL_CHECK_LE(high_band_edge, sample_rate / 2);
  ABSL_CHECK_EQ(window.size() % 2, 1);  // Window size must be odd.
  const int radius = (window.size() - 1) / 2;
  std::vector<ValueType> filter(window.size());
  const ValueType factor = 2 * high_band_edge / sample_rate;
  filter[radius] = factor;
  for (int i = 1; i <= radius; ++i) {
    ValueType value = sin(M_PI * factor * i) / (M_PI * i);
    filter[radius + i] = value;
    filter[radius - i] = value;
  }
  MultiplyVector(window, &filter);
  return filter;
}

// Design an FIR highpass filter computing the impulse response of the ideal
// brickwall filter and windowing it with the given window. The filter has zero
// phase provided that the window is symmetric.
// NOTE: The window must have odd size.
template <typename ContainerType>
std::vector<typename ContainerType::value_type> DesignFirHighpassFilter(
    float low_band_edge, float sample_rate, const ContainerType& window) {
  // Design the highpass filter by making the complementary lowpass filter and
  // subtracting it from a delta.
  std::vector<typename ContainerType::value_type> filter =
      DesignFirLowpassFilter(low_band_edge, sample_rate, window);
  MultiplyVectorByScalar(-1, &filter);
  const int radius = (window.size() - 1) / 2;
  filter[radius] += window[radius];
  return filter;
}

// Design an FIR bandpass filter computing the impulse response of the ideal
// brickwall filter and windowing it with the given window. The filter has zero
// phase provided that the window is symmetric.
// NOTE: The window must have odd size.
template <typename ContainerType>
std::vector<typename ContainerType::value_type> DesignFirBandpassFilter(
    float low_band_edge, float high_band_edge, float sample_rate,
    const ContainerType& window) {
  ABSL_CHECK_LE(low_band_edge, high_band_edge);
  // Design the bandpass filter as the difference of two lowpass filters.
  std::vector<typename ContainerType::value_type> filter1 =
      DesignFirLowpassFilter(low_band_edge, sample_rate, window);
  std::vector<typename ContainerType::value_type> filter2 =
      DesignFirLowpassFilter(high_band_edge, sample_rate, window);
  for (int i = 0; i < filter1.size(); ++i) {
    filter2[i] -= filter1[i];
  }
  return filter2;
}

}  // namespace audio_dsp

#endif  // AUDIO_DSP_SIGNAL_VECTOR_UTIL_H_
