/*
 * Copyright 2020-2021 Google LLC
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

// Utilities for working generically with types.

#ifndef AUDIO_DSP_TYPES_H_
#define AUDIO_DSP_TYPES_H_

#include <complex>
#include <memory>
#include <type_traits>

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// Determine whether a scalar type is complex-valued,
// IsComplex<ValueType>::Value is true if and only if ValueType is complex.
template <typename ValueType>
struct IsComplex {
  // Set result to false by default.
  enum { Value = false };
};

// Use template specializations to set true for IsComplex<complex<T>>.
template <>
struct IsComplex<std::complex<float>> {
  enum { Value = true };
};

template <>
struct IsComplex<std::complex<double>> {
  enum { Value = true };
};

template <>
struct IsComplex<std::complex<long double>> {
  enum { Value = true };
};

// HoldsComplex<ContainerType>::Value is true if and only if the type of its
// elements is complex-valued.
template <typename ContainerType>
struct HoldsComplex {
  enum { Value = IsComplex<typename ContainerType::value_type>::Value };
};

// RealType<ValueType>::Type is equal to T if ValueType is complex<T>, and
// otherwise it is equal to ValueType.
template <typename ValueType, bool = IsComplex<ValueType>::Value>
struct RealType {
  // If ValueType is complex, get the T from complex<T>.
  typedef typename ValueType::value_type Type;
};

template <typename ValueType>
struct RealType<ValueType, false> {
  // Otherwise, just return ValueType.
  typedef ValueType Type;
};

// ComplexType<ValueType>::Type is equal to complex<ValueType>, except when
// ValueType is already a complex type (according to IsComplex) in which case
// ComplexType<ValueType>::Type is ValueType.
template <typename ValueType>
struct ComplexType {
  typedef typename std::complex<typename RealType<ValueType>::Type> Type;
};

// Possibly promote a given scalar type ValueType to a floating-point type. If
// ValueType is an integral type, FloatPromotion<ValueType>::Type is double,
// otherwise it is ValueType. Has no effect on real and complex floating-point
// types.
template<class ValueType>
struct FloatPromotion {
  typedef typename std::conditional<std::is_integral<ValueType>::value,
      // If ValueType is integral, return double.
      double,
      // Otherwise just return ValueType.
      ValueType>::type Type;
};

// If `x` is a pointer, dereference it. Otherwise, return it unchanged:
//
//   std::vector<float> x;
//   DerefIfPointer(x) == x
//
//   std::vector<float>* y;
//   DerefIfPointer(y) == *y
//
//   std::unique_ptr<std::vector<float>> z;
//   DerefIfPointer(z) == *z
//
// At most one level of indirection is dereferenced. If `x` is a pointer to a
// pointer T**, then DerefIfPointer(x) returns a T*.
//
// NOTE: This template works with std::unique_ptr<T>, but not with other smart
// pointers.
template <typename T>
T&& DerefIfPointer(T&& x) { return std::forward<T>(x); }

template <typename T>
T& DerefIfPointer(T* x) { return *x; }

template <typename T>
T& DerefIfPointer(std::unique_ptr<T>& x) { return *x; }

}  // namespace audio_dsp

#endif  // AUDIO_DSP_TYPES_H_
