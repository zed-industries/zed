//===----- hlsl_intrinsics.h - HLSL definitions for intrinsics ----------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _HLSL_HLSL_INTRINSICS_H_
#define _HLSL_HLSL_INTRINSICS_H_

#include "hlsl/hlsl_intrinsic_helpers.h"

namespace hlsl {

//===----------------------------------------------------------------------===//
// asfloat builtins
//===----------------------------------------------------------------------===//

/// \fn float asfloat(T Val)
/// \brief Interprets the bit pattern of x as float point number.
/// \param Val The input value.

template <typename T, int N>
constexpr vector<float, N> asfloat(vector<T, N> V) {
  return __detail::bit_cast<float, T, N>(V);
}

template <typename T> constexpr float asfloat(T F) {
  return __detail::bit_cast<float, T>(F);
}

//===----------------------------------------------------------------------===//
// asint builtins
//===----------------------------------------------------------------------===//

/// \fn int asint(T Val)
/// \brief Interprets the bit pattern of x as an integer.
/// \param Val The input value.

template <typename T, int N> constexpr vector<int, N> asint(vector<T, N> V) {
  return __detail::bit_cast<int, T, N>(V);
}

template <typename T> constexpr int asint(T F) {
  return __detail::bit_cast<int, T>(F);
}

//===----------------------------------------------------------------------===//
// asint16 builtins
//===----------------------------------------------------------------------===//

/// \fn int16_t asint16(T X)
/// \brief Interprets the bit pattern of \a X as an 16-bit integer.
/// \param X The input value.

#ifdef __HLSL_ENABLE_16_BIT

template <typename T, int N>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
constexpr __detail::enable_if_t<__detail::is_same<int16_t, T>::value ||
                                    __detail::is_same<uint16_t, T>::value ||
                                    __detail::is_same<half, T>::value,
                                vector<int16_t, N>> asint16(vector<T, N> V) {
  return __detail::bit_cast<int16_t, T, N>(V);
}

template <typename T>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
constexpr __detail::enable_if_t<__detail::is_same<int16_t, T>::value ||
                                    __detail::is_same<uint16_t, T>::value ||
                                    __detail::is_same<half, T>::value,
                                int16_t> asint16(T F) {
  return __detail::bit_cast<int16_t, T>(F);
}
#endif

//===----------------------------------------------------------------------===//
// asuint builtins
//===----------------------------------------------------------------------===//

/// \fn uint asuint(T Val)
/// \brief Interprets the bit pattern of x as an unsigned integer.
/// \param Val The input value.

template <typename T, int N> constexpr vector<uint, N> asuint(vector<T, N> V) {
  return __detail::bit_cast<uint, T, N>(V);
}

template <typename T> constexpr uint asuint(T F) {
  return __detail::bit_cast<uint, T>(F);
}

//===----------------------------------------------------------------------===//
// asuint splitdouble builtins
//===----------------------------------------------------------------------===//

/// \fn void asuint(double D, out uint lowbits, out int highbits)
/// \brief Split and interprets the lowbits and highbits of double D into uints.
/// \param D The input double.
/// \param lowbits The output lowbits of D.
/// \param highbits The output highbits of D.
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_splitdouble)
void asuint(double, out uint, out uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_splitdouble)
void asuint(double2, out uint2, out uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_splitdouble)
void asuint(double3, out uint3, out uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_splitdouble)
void asuint(double4, out uint4, out uint4);

//===----------------------------------------------------------------------===//
// asuint16 builtins
//===----------------------------------------------------------------------===//

/// \fn uint16_t asuint16(T X)
/// \brief Interprets the bit pattern of \a X as an 16-bit unsigned integer.
/// \param X The input value.

#ifdef __HLSL_ENABLE_16_BIT

template <typename T, int N>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
constexpr __detail::enable_if_t<__detail::is_same<int16_t, T>::value ||
                                    __detail::is_same<uint16_t, T>::value ||
                                    __detail::is_same<half, T>::value,
                                vector<uint16_t, N>> asuint16(vector<T, N> V) {
  return __detail::bit_cast<uint16_t, T, N>(V);
}

template <typename T>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
constexpr __detail::enable_if_t<__detail::is_same<int16_t, T>::value ||
                                    __detail::is_same<uint16_t, T>::value ||
                                    __detail::is_same<half, T>::value,
                                uint16_t> asuint16(T F) {
  return __detail::bit_cast<uint16_t, T>(F);
}
#endif

//===----------------------------------------------------------------------===//
// distance builtins
//===----------------------------------------------------------------------===//

/// \fn K distance(T X, T Y)
/// \brief Returns a distance scalar between \a X and \a Y.
/// \param X The X input value.
/// \param Y The Y input value.

template <typename T>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline __detail::enable_if_t<__detail::is_arithmetic<T>::Value &&
                                       __detail::is_same<half, T>::value,
                                   T> distance(T X, T Y) {
  return __detail::distance_impl(X, Y);
}

template <typename T>
const inline __detail::enable_if_t<
    __detail::is_arithmetic<T>::Value && __detail::is_same<float, T>::value, T>
distance(T X, T Y) {
  return __detail::distance_impl(X, Y);
}

template <int N>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline half distance(__detail::HLSL_FIXED_VECTOR<half, N> X,
                           __detail::HLSL_FIXED_VECTOR<half, N> Y) {
  return __detail::distance_vec_impl(X, Y);
}

template <int N>
const inline float distance(__detail::HLSL_FIXED_VECTOR<float, N> X,
                            __detail::HLSL_FIXED_VECTOR<float, N> Y) {
  return __detail::distance_vec_impl(X, Y);
}

//===----------------------------------------------------------------------===//
// dot2add builtins
//===----------------------------------------------------------------------===//

/// \fn float dot2add(half2 A, half2 B, float C)
/// \brief Dot product of 2 vector of type half and add a float scalar value.
/// \param A The first input value to dot product.
/// \param B The second input value to dot product.
/// \param C The input value added to the dot product.

_HLSL_AVAILABILITY(shadermodel, 6.4)
const inline float dot2add(half2 A, half2 B, float C) {
  return __detail::dot2add_impl(A, B, C);
}

//===----------------------------------------------------------------------===//
// dst builtins
//===----------------------------------------------------------------------===//

/// \fn vector<T, 4> dst(vector<T, 4>, vector<T, 4>)
/// \brief Calculates a distance vector.
/// \param Src0 [in] Contains the squared distance
/// \param Src1 [in] Contains the reciprocal distance
///
/// Return the computed distance vector

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline half4 dst(half4 Src0, half4 Src1) {
  return __detail::dst_impl(Src0, Src1);
}

const inline float4 dst(float4 Src0, float4 Src1) {
  return __detail::dst_impl(Src0, Src1);
}

const inline double4 dst(double4 Src0, double4 Src1) {
  return __detail::dst_impl(Src0, Src1);
}

//===----------------------------------------------------------------------===//
// fmod builtins
//===----------------------------------------------------------------------===//

/// \fn T fmod(T x, T y)
/// \brief Returns the linear interpolation of x to y.
/// \param x [in] The dividend.
/// \param y [in] The divisor.
///
/// Return the floating-point remainder of the x parameter divided by the y
/// parameter.

template <typename T>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline __detail::enable_if_t<__detail::is_arithmetic<T>::Value &&
                                       __detail::is_same<half, T>::value,
                                   T> fmod(T X, T Y) {
  return __detail::fmod_impl(X, Y);
}

template <typename T>
const inline __detail::enable_if_t<
    __detail::is_arithmetic<T>::Value && __detail::is_same<float, T>::value, T>
fmod(T X, T Y) {
  return __detail::fmod_impl(X, Y);
}

template <int N>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline __detail::HLSL_FIXED_VECTOR<half, N> fmod(
    __detail::HLSL_FIXED_VECTOR<half, N> X,
    __detail::HLSL_FIXED_VECTOR<half, N> Y) {
  return __detail::fmod_vec_impl(X, Y);
}

template <int N>
const inline __detail::HLSL_FIXED_VECTOR<float, N>
fmod(__detail::HLSL_FIXED_VECTOR<float, N> X,
     __detail::HLSL_FIXED_VECTOR<float, N> Y) {
  return __detail::fmod_vec_impl(X, Y);
}

//===----------------------------------------------------------------------===//
// length builtins
//===----------------------------------------------------------------------===//

/// \fn T length(T x)
/// \brief Returns the length of the specified floating-point vector.
/// \param x [in] The vector of floats, or a scalar float.
///
/// Length is based on the following formula: sqrt(x[0]^2 + x[1]^2 + ...).

template <typename T>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline __detail::enable_if_t<__detail::is_arithmetic<T>::Value &&
                                       __detail::is_same<half, T>::value,
                                   T> length(T X) {
  return __detail::length_impl(X);
}

template <typename T>
const inline __detail::enable_if_t<
    __detail::is_arithmetic<T>::Value && __detail::is_same<float, T>::value, T>
length(T X) {
  return __detail::length_impl(X);
}

template <int N>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline half length(__detail::HLSL_FIXED_VECTOR<half, N> X) {
  return __detail::length_vec_impl(X);
}

template <int N>
const inline float length(__detail::HLSL_FIXED_VECTOR<float, N> X) {
  return __detail::length_vec_impl(X);
}

//===----------------------------------------------------------------------===//
// lit builtins
//===----------------------------------------------------------------------===//

/// \fn vector<T, 4> lit(T NDotL, T NDotH, T M)
/// \brief Returns a lighting coefficient vector.
/// \param NDotL The dot product of the normalized surface normal and the
/// light vector.
/// \param NDotH The dot product of the half-angle vector and the surface
/// normal.
/// \param M A specular exponent.
///
/// This function returns a lighting coefficient vector (ambient, diffuse,
/// specular, 1).

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline half4 lit(half NDotL, half NDotH, half M) {
  return __detail::lit_impl(NDotL, NDotH, M);
}

const inline float4 lit(float NDotL, float NDotH, float M) {
  return __detail::lit_impl(NDotL, NDotH, M);
}

//===----------------------------------------------------------------------===//
// D3DCOLORtoUBYTE4 builtin
//===----------------------------------------------------------------------===//

/// \fn T D3DCOLORtoUBYTE4(T x)
/// \brief Converts a floating-point, 4D vector set by a D3DCOLOR to a UBYTE4.
/// \param x [in] The floating-point vector4 to convert.
///
/// The return value is the UBYTE4 representation of the \a x parameter.
///
/// This function swizzles and scales components of the \a x parameter. Use this
/// function to compensate for the lack of UBYTE4 support in some hardware.

constexpr vector<uint, 4> D3DCOLORtoUBYTE4(vector<float, 4> V) {
  return __detail::d3d_color_to_ubyte4_impl(V);
}

//===----------------------------------------------------------------------===//
// reflect builtin
//===----------------------------------------------------------------------===//

/// \fn T reflect(T I, T N)
/// \brief Returns a reflection using an incident ray, \a I, and a surface
/// normal, \a N.
/// \param I The incident ray.
/// \param N The surface normal.
///
/// The return value is a floating-point vector that represents the reflection
/// of the incident ray, \a I, off a surface with the normal \a N.
///
/// This function calculates the reflection vector using the following formula:
/// V = I - 2 * N * dot(I N) .
///
/// N must already be normalized in order to achieve the desired result.
///
/// The operands must all be a scalar or vector whose component type is
/// floating-point.
///
/// Result type and the type of all operands must be the same type.

template <typename T>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline __detail::enable_if_t<__detail::is_arithmetic<T>::Value &&
                                       __detail::is_same<half, T>::value,
                                   T> reflect(T I, T N) {
  return __detail::reflect_impl(I, N);
}

template <typename T>
const inline __detail::enable_if_t<
    __detail::is_arithmetic<T>::Value && __detail::is_same<float, T>::value, T>
reflect(T I, T N) {
  return __detail::reflect_impl(I, N);
}

template <int L>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline __detail::HLSL_FIXED_VECTOR<half, L> reflect(
    __detail::HLSL_FIXED_VECTOR<half, L> I,
    __detail::HLSL_FIXED_VECTOR<half, L> N) {
  return __detail::reflect_vec_impl(I, N);
}

template <int L>
const inline __detail::HLSL_FIXED_VECTOR<float, L>
reflect(__detail::HLSL_FIXED_VECTOR<float, L> I,
        __detail::HLSL_FIXED_VECTOR<float, L> N) {
  return __detail::reflect_vec_impl(I, N);
}

//===----------------------------------------------------------------------===//
// smoothstep builtin
//===----------------------------------------------------------------------===//

/// \fn T smoothstep(T Min, T Max, T X)
/// \brief Returns a smooth Hermite interpolation between 0 and 1, if \a X is in
/// the range [\a Min, \a Max].
/// \param Min The minimum range of the x parameter.
/// \param Max The maximum range of the x parameter.
/// \param X The specified value to be interpolated.
///
/// The return value is 0.0 if \a X ≤ \a Min and 1.0 if \a X ≥ \a Max. When \a
/// Min < \a X < \a Max, the function performs smooth Hermite interpolation
/// between 0 and 1.

template <typename T>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline __detail::enable_if_t<__detail::is_arithmetic<T>::Value &&
                                       __detail::is_same<half, T>::value,
                                   T> smoothstep(T Min, T Max, T X) {
  return __detail::smoothstep_impl(Min, Max, X);
}

template <typename T>
const inline __detail::enable_if_t<
    __detail::is_arithmetic<T>::Value && __detail::is_same<float, T>::value, T>
smoothstep(T Min, T Max, T X) {
  return __detail::smoothstep_impl(Min, Max, X);
}

template <int N>
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
const inline __detail::HLSL_FIXED_VECTOR<half, N> smoothstep(
    __detail::HLSL_FIXED_VECTOR<half, N> Min,
    __detail::HLSL_FIXED_VECTOR<half, N> Max,
    __detail::HLSL_FIXED_VECTOR<half, N> X) {
  return __detail::smoothstep_vec_impl(Min, Max, X);
}

template <int N>
const inline __detail::HLSL_FIXED_VECTOR<float, N>
smoothstep(__detail::HLSL_FIXED_VECTOR<float, N> Min,
           __detail::HLSL_FIXED_VECTOR<float, N> Max,
           __detail::HLSL_FIXED_VECTOR<float, N> X) {
  return __detail::smoothstep_vec_impl(Min, Max, X);
}

} // namespace hlsl
#endif //_HLSL_HLSL_INTRINSICS_H_
