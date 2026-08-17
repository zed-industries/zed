//===--- hlsl_alias_intrinsics.h - HLSL alias definitions for intrinsics --===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _HLSL_HLSL_ALIAS_INTRINSICS_H_
#define _HLSL_HLSL_ALIAS_INTRINSICS_H_

namespace hlsl {

// Note: Functions in this file are sorted alphabetically, then grouped by base
// element type, and the element types are sorted by size, then singed integer,
// unsigned integer and floating point. Keeping this ordering consistent will
// help keep this file manageable as it grows.

#define _HLSL_BUILTIN_ALIAS(builtin)                                           \
  __attribute__((clang_builtin_alias(builtin)))
#define _HLSL_AVAILABILITY(platform, version)                                  \
  __attribute__((availability(platform, introduced = version)))
#define _HLSL_AVAILABILITY_STAGE(platform, version, stage)                     \
  __attribute__((                                                              \
      availability(platform, introduced = version, environment = stage)))

#ifdef __HLSL_ENABLE_16_BIT
#define _HLSL_16BIT_AVAILABILITY(platform, version)                            \
  __attribute__((availability(platform, introduced = version)))
#define _HLSL_16BIT_AVAILABILITY_STAGE(platform, version, stage)               \
  __attribute__((                                                              \
      availability(platform, introduced = version, environment = stage)))
#else
#define _HLSL_16BIT_AVAILABILITY(environment, version)
#define _HLSL_16BIT_AVAILABILITY_STAGE(environment, version, stage)
#endif

//===----------------------------------------------------------------------===//
// abs builtins
//===----------------------------------------------------------------------===//

/// \fn T abs(T Val)
/// \brief Returns the absolute value of the input value, \a Val.
/// \param Val The input value.

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int16_t abs(int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int16_t2 abs(int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int16_t3 abs(int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int16_t4 abs(int16_t4);

_HLSL_AVAILABILITY(shadermodel, 6.2)
constexpr uint16_t abs(uint16_t V) { return V; }
_HLSL_AVAILABILITY(shadermodel, 6.2)
constexpr uint16_t2 abs(uint16_t2 V) { return V; }
_HLSL_AVAILABILITY(shadermodel, 6.2)
constexpr uint16_t3 abs(uint16_t3 V) { return V; }
_HLSL_AVAILABILITY(shadermodel, 6.2)
constexpr uint16_t4 abs(uint16_t4 V) { return V; }
#endif

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
half abs(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
half2 abs(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
half3 abs(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
half4 abs(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int abs(int);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int2 abs(int2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int3 abs(int3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int4 abs(int4);

constexpr uint abs(uint V) { return V; }
constexpr uint2 abs(uint2 V) { return V; }
constexpr uint3 abs(uint3 V) { return V; }
constexpr uint4 abs(uint4 V) { return V; }

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
float abs(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
float2 abs(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
float3 abs(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
float4 abs(float4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int64_t abs(int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int64_t2 abs(int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int64_t3 abs(int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
int64_t4 abs(int64_t4);

constexpr uint64_t abs(uint64_t V) { return V; }
constexpr uint64_t2 abs(uint64_t2 V) { return V; }
constexpr uint64_t3 abs(uint64_t3 V) { return V; }
constexpr uint64_t4 abs(uint64_t4 V) { return V; }

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
double abs(double);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
double2 abs(double2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
double3 abs(double3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_abs)
double4 abs(double4);

//===----------------------------------------------------------------------===//
// acos builtins
//===----------------------------------------------------------------------===//

/// \fn T acos(T Val)
/// \brief Returns the arccosine of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_acos)
half acos(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_acos)
half2 acos(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_acos)
half3 acos(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_acos)
half4 acos(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_acos)
float acos(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_acos)
float2 acos(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_acos)
float3 acos(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_acos)
float4 acos(float4);

//===----------------------------------------------------------------------===//
// AddUint64 builtins
//===----------------------------------------------------------------------===//

/// \fn T AddUint64(T a, T b)
/// \brief Implements unsigned 64-bit integer addition using pairs of unsigned
/// 32-bit integers.
/// \param x [in] The first unsigned 32-bit integer pair(s)
/// \param y [in] The second unsigned 32-bit integer pair(s)
///
/// This function takes one or two pairs (low, high) of unsigned 32-bit integer
/// values and returns pairs (low, high) of unsigned 32-bit integer
/// values representing the result of unsigned 64-bit integer addition.

_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_adduint64)
uint32_t2 AddUint64(uint32_t2, uint32_t2);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_adduint64)
uint32_t4 AddUint64(uint32_t4, uint32_t4);

//===----------------------------------------------------------------------===//
// all builtins
//===----------------------------------------------------------------------===//

/// \fn bool all(T x)
/// \brief Returns True if all components of the \a x parameter are non-zero;
/// otherwise, false. \param x The input value.

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int16_t4);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint16_t4);
#endif

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(bool);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(bool2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(bool3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(bool4);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(float4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(uint64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(double);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(double2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(double3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_all)
bool all(double4);

//===----------------------------------------------------------------------===//
// and builtins
//===----------------------------------------------------------------------===//

/// \fn bool and(bool x, bool y)
/// \brief Logically ands two boolean vectors elementwise and produces a bool
/// vector output.

// TODO: Clean up clang-format marker once we've resolved
//       https://github.com/llvm/llvm-project/issues/127851
//
// clang-format off
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_and)
bool and(bool x, bool y);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_and)
bool2 and(bool2 x, bool2 y);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_and)
bool3 and(bool3 x, bool3 y);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_and)
bool4 and(bool4 x, bool4 y);
// clang-format on

//===----------------------------------------------------------------------===//
// any builtins
//===----------------------------------------------------------------------===//

/// \fn bool any(T x)
/// \brief Returns True if any components of the \a x parameter are non-zero;
/// otherwise, false. \param x The input value.

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int16_t4);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint16_t4);
#endif

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(bool);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(bool2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(bool3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(bool4);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(float4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(uint64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(double);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(double2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(double3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_any)
bool any(double4);

//===----------------------------------------------------------------------===//
// asdouble builtins
//===----------------------------------------------------------------------===//

/// \fn double asdouble(uint LowBits, uint HighBits)
/// \brief Reinterprets a cast value (two 32-bit values) into a double.
/// \param LowBits The low 32-bit pattern of the input value.
/// \param HighBits The high 32-bit pattern of the input value.

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_asdouble)
double asdouble(uint, uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_asdouble)
double2 asdouble(uint2, uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_asdouble)
double3 asdouble(uint3, uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_asdouble)
double4 asdouble(uint4, uint4);

//===----------------------------------------------------------------------===//
// asin builtins
//===----------------------------------------------------------------------===//

/// \fn T asin(T Val)
/// \brief Returns the arcsine of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_asin)
half asin(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_asin)
half2 asin(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_asin)
half3 asin(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_asin)
half4 asin(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_asin)
float asin(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_asin)
float2 asin(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_asin)
float3 asin(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_asin)
float4 asin(float4);

//===----------------------------------------------------------------------===//
// atan builtins
//===----------------------------------------------------------------------===//

/// \fn T atan(T Val)
/// \brief Returns the arctangent of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan)
half atan(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan)
half2 atan(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan)
half3 atan(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan)
half4 atan(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan)
float atan(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan)
float2 atan(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan)
float3 atan(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan)
float4 atan(float4);

//===----------------------------------------------------------------------===//
// atan2 builtins
//===----------------------------------------------------------------------===//

/// \fn T atan2(T y, T x)
/// \brief Returns the arctangent of y/x, using the signs of the arguments to
/// determine the correct quadrant.
/// \param y The y-coordinate.
/// \param x The x-coordinate.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan2)
half atan2(half y, half x);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan2)
half2 atan2(half2 y, half2 x);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan2)
half3 atan2(half3 y, half3 x);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan2)
half4 atan2(half4 y, half4 x);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan2)
float atan2(float y, float x);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan2)
float2 atan2(float2 y, float2 x);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan2)
float3 atan2(float3 y, float3 x);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_atan2)
float4 atan2(float4 y, float4 x);

//===----------------------------------------------------------------------===//
// ceil builtins
//===----------------------------------------------------------------------===//

/// \fn T ceil(T Val)
/// \brief Returns the smallest integer value that is greater than or equal to
/// the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_ceil)
half ceil(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_ceil)
half2 ceil(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_ceil)
half3 ceil(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_ceil)
half4 ceil(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_ceil)
float ceil(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_ceil)
float2 ceil(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_ceil)
float3 ceil(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_ceil)
float4 ceil(float4);

//===----------------------------------------------------------------------===//
// clamp builtins
//===----------------------------------------------------------------------===//

/// \fn T clamp(T X, T Min, T Max)
/// \brief Clamps the specified value \a X to the specified
/// minimum ( \a Min) and maximum ( \a Max) range.
/// \param X A value to clamp.
/// \param Min The specified minimum range.
/// \param Max The specified maximum range.
///
/// Returns The clamped value for the \a X parameter.
/// For values of -INF or INF, clamp will behave as expected.
/// However for values of NaN, the results are undefined.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
half clamp(half, half, half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
half2 clamp(half2, half2, half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
half3 clamp(half3, half3, half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
half4 clamp(half4, half4, half4);

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int16_t clamp(int16_t, int16_t, int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int16_t2 clamp(int16_t2, int16_t2, int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int16_t3 clamp(int16_t3, int16_t3, int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int16_t4 clamp(int16_t4, int16_t4, int16_t4);

_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint16_t clamp(uint16_t, uint16_t, uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint16_t2 clamp(uint16_t2, uint16_t2, uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint16_t3 clamp(uint16_t3, uint16_t3, uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint16_t4 clamp(uint16_t4, uint16_t4, uint16_t4);
#endif

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int clamp(int, int, int);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int2 clamp(int2, int2, int2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int3 clamp(int3, int3, int3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int4 clamp(int4, int4, int4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint clamp(uint, uint, uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint2 clamp(uint2, uint2, uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint3 clamp(uint3, uint3, uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint4 clamp(uint4, uint4, uint4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int64_t clamp(int64_t, int64_t, int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int64_t2 clamp(int64_t2, int64_t2, int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int64_t3 clamp(int64_t3, int64_t3, int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
int64_t4 clamp(int64_t4, int64_t4, int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint64_t clamp(uint64_t, uint64_t, uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint64_t2 clamp(uint64_t2, uint64_t2, uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint64_t3 clamp(uint64_t3, uint64_t3, uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
uint64_t4 clamp(uint64_t4, uint64_t4, uint64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
float clamp(float, float, float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
float2 clamp(float2, float2, float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
float3 clamp(float3, float3, float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
float4 clamp(float4, float4, float4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
double clamp(double, double, double);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
double2 clamp(double2, double2, double2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
double3 clamp(double3, double3, double3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clamp)
double4 clamp(double4, double4, double4);

//===----------------------------------------------------------------------===//
// clip builtins
//===----------------------------------------------------------------------===//

/// \fn void clip(T Val)
/// \brief Discards the current pixel if the specified value is less than zero.
/// \param Val The input value.

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clip)
void clip(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clip)
void clip(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clip)
void clip(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_clip)
void clip(float4);

//===----------------------------------------------------------------------===//
// cos builtins
//===----------------------------------------------------------------------===//

/// \fn T cos(T Val)
/// \brief Returns the cosine of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cos)
half cos(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cos)
half2 cos(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cos)
half3 cos(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cos)
half4 cos(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cos)
float cos(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cos)
float2 cos(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cos)
float3 cos(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cos)
float4 cos(float4);

//===----------------------------------------------------------------------===//
// cosh builtins
//===----------------------------------------------------------------------===//

/// \fn T cosh(T Val)
/// \brief Returns the hyperbolic cosine of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cosh)
half cosh(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cosh)
half2 cosh(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cosh)
half3 cosh(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cosh)
half4 cosh(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cosh)
float cosh(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cosh)
float2 cosh(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cosh)
float3 cosh(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_cosh)
float4 cosh(float4);

//===----------------------------------------------------------------------===//
// count bits builtins
//===----------------------------------------------------------------------===//

/// \fn T countbits(T Val)
/// \brief Return the number of bits (per component) set in the input integer.
/// \param Val The input value.

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
const inline uint countbits(int16_t x) {
  return __builtin_elementwise_popcount(x);
}
_HLSL_AVAILABILITY(shadermodel, 6.2)
const inline uint2 countbits(int16_t2 x) {
  return __builtin_elementwise_popcount(x);
}
_HLSL_AVAILABILITY(shadermodel, 6.2)
const inline uint3 countbits(int16_t3 x) {
  return __builtin_elementwise_popcount(x);
}
_HLSL_AVAILABILITY(shadermodel, 6.2)
const inline uint4 countbits(int16_t4 x) {
  return __builtin_elementwise_popcount(x);
}
_HLSL_AVAILABILITY(shadermodel, 6.2)
const inline uint countbits(uint16_t x) {
  return __builtin_elementwise_popcount(x);
}
_HLSL_AVAILABILITY(shadermodel, 6.2)
const inline uint2 countbits(uint16_t2 x) {
  return __builtin_elementwise_popcount(x);
}
_HLSL_AVAILABILITY(shadermodel, 6.2)
const inline uint3 countbits(uint16_t3 x) {
  return __builtin_elementwise_popcount(x);
}
_HLSL_AVAILABILITY(shadermodel, 6.2)
const inline uint4 countbits(uint16_t4 x) {
  return __builtin_elementwise_popcount(x);
}
#endif

const inline uint countbits(int x) { return __builtin_elementwise_popcount(x); }
const inline uint2 countbits(int2 x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint3 countbits(int3 x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint4 countbits(int4 x) {
  return __builtin_elementwise_popcount(x);
}

const inline uint countbits(uint x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint2 countbits(uint2 x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint3 countbits(uint3 x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint4 countbits(uint4 x) {
  return __builtin_elementwise_popcount(x);
}

const inline uint countbits(int64_t x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint2 countbits(int64_t2 x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint3 countbits(int64_t3 x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint4 countbits(int64_t4 x) {
  return __builtin_elementwise_popcount(x);
}

const inline uint countbits(uint64_t x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint2 countbits(uint64_t2 x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint3 countbits(uint64_t3 x) {
  return __builtin_elementwise_popcount(x);
}
const inline uint4 countbits(uint64_t4 x) {
  return __builtin_elementwise_popcount(x);
}

//===----------------------------------------------------------------------===//
// degrees builtins
//===----------------------------------------------------------------------===//

/// \fn T degrees(T x)
/// \brief Converts the specified value from radians to degrees.
/// \param x The specified input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_degrees)
half degrees(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_degrees)
half2 degrees(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_degrees)
half3 degrees(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_degrees)
half4 degrees(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_degrees)
float degrees(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_degrees)
float2 degrees(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_degrees)
float3 degrees(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_degrees)
float4 degrees(float4);

//===----------------------------------------------------------------------===//
// dot product builtins
//===----------------------------------------------------------------------===//

/// \fn K dot(T X, T Y)
/// \brief Return the dot product (a scalar value) of \a X and \a Y.
/// \param X The X input value.
/// \param Y The Y input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
half dot(half, half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
half dot(half2, half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
half dot(half3, half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
half dot(half4, half4);

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int16_t dot(int16_t, int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int16_t dot(int16_t2, int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int16_t dot(int16_t3, int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int16_t dot(int16_t4, int16_t4);

_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint16_t dot(uint16_t, uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint16_t dot(uint16_t2, uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint16_t dot(uint16_t3, uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint16_t dot(uint16_t4, uint16_t4);
#endif

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
float dot(float, float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
float dot(float2, float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
float dot(float3, float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
float dot(float4, float4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
double dot(double, double);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int dot(int, int);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int dot(int2, int2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int dot(int3, int3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int dot(int4, int4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint dot(uint, uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint dot(uint2, uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint dot(uint3, uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint dot(uint4, uint4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int64_t dot(int64_t, int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int64_t dot(int64_t2, int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int64_t dot(int64_t3, int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
int64_t dot(int64_t4, int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint64_t dot(uint64_t, uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint64_t dot(uint64_t2, uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint64_t dot(uint64_t3, uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot)
uint64_t dot(uint64_t4, uint64_t4);

//===----------------------------------------------------------------------===//
// dot4add builtins
//===----------------------------------------------------------------------===//

/// \fn int dot4add_i8packed(uint A, uint B, int C)

_HLSL_AVAILABILITY(shadermodel, 6.4)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot4add_i8packed)
int dot4add_i8packed(uint, uint, int);

/// \fn uint dot4add_u8packed(uint A, uint B, uint C)

_HLSL_AVAILABILITY(shadermodel, 6.4)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_dot4add_u8packed)
uint dot4add_u8packed(uint, uint, uint);

//===----------------------------------------------------------------------===//
// exp builtins
//===----------------------------------------------------------------------===//

/// \fn T exp(T x)
/// \brief Returns the base-e exponential, or \a e**x, of the specified value.
/// \param x The specified input value.
///
/// The return value is the base-e exponential of the \a x parameter.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp)
half exp(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp)
half2 exp(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp)
half3 exp(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp)
half4 exp(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp)
float exp(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp)
float2 exp(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp)
float3 exp(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp)
float4 exp(float4);

//===----------------------------------------------------------------------===//
// exp2 builtins
//===----------------------------------------------------------------------===//

/// \fn T exp2(T x)
/// \brief Returns the base 2 exponential, or \a 2**x, of the specified value.
/// \param x The specified input value.
///
/// The base 2 exponential of the \a x parameter.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp2)
half exp2(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp2)
half2 exp2(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp2)
half3 exp2(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp2)
half4 exp2(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp2)
float exp2(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp2)
float2 exp2(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp2)
float3 exp2(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_exp2)
float4 exp2(float4);

//===----------------------------------------------------------------------===//
// firstbithigh builtins
//===----------------------------------------------------------------------===//

/// \fn T firstbithigh(T Val)
/// \brief Returns the location of the first set bit starting from the highest
/// order bit and working downward, per component.
/// \param Val the input value.

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint firstbithigh(int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint2 firstbithigh(int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint3 firstbithigh(int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint4 firstbithigh(int16_t4);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint firstbithigh(uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint2 firstbithigh(uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint3 firstbithigh(uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint4 firstbithigh(uint16_t4);
#endif

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint firstbithigh(int);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint2 firstbithigh(int2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint3 firstbithigh(int3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint4 firstbithigh(int4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint firstbithigh(uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint2 firstbithigh(uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint3 firstbithigh(uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint4 firstbithigh(uint4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint firstbithigh(int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint2 firstbithigh(int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint3 firstbithigh(int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint4 firstbithigh(int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint firstbithigh(uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint2 firstbithigh(uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint3 firstbithigh(uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbithigh)
uint4 firstbithigh(uint64_t4);

//===----------------------------------------------------------------------===//
// firstbitlow builtins
//===----------------------------------------------------------------------===//

/// \fn T firstbitlow(T Val)
/// \brief Returns the location of the first set bit starting from the lowest
/// order bit and working upward, per component.
/// \param Val the input value.

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint firstbitlow(int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint2 firstbitlow(int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint3 firstbitlow(int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint4 firstbitlow(int16_t4);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint firstbitlow(uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint2 firstbitlow(uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint3 firstbitlow(uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint4 firstbitlow(uint16_t4);
#endif

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint firstbitlow(int);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint2 firstbitlow(int2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint3 firstbitlow(int3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint4 firstbitlow(int4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint firstbitlow(uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint2 firstbitlow(uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint3 firstbitlow(uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint4 firstbitlow(uint4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint firstbitlow(int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint2 firstbitlow(int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint3 firstbitlow(int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint4 firstbitlow(int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint firstbitlow(uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint2 firstbitlow(uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint3 firstbitlow(uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_firstbitlow)
uint4 firstbitlow(uint64_t4);

//===----------------------------------------------------------------------===//
// floor builtins
//===----------------------------------------------------------------------===//

/// \fn T floor(T Val)
/// \brief Returns the largest integer that is less than or equal to the input
/// value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_floor)
half floor(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_floor)
half2 floor(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_floor)
half3 floor(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_floor)
half4 floor(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_floor)
float floor(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_floor)
float2 floor(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_floor)
float3 floor(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_floor)
float4 floor(float4);

//===----------------------------------------------------------------------===//
// frac builtins
//===----------------------------------------------------------------------===//

/// \fn T frac(T x)
/// \brief Returns the fractional (or decimal) part of x. \a x parameter.
/// \param x The specified input value.
///
/// If \a the return value is greater than or equal to 0 and less than 1.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_frac)
half frac(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_frac)
half2 frac(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_frac)
half3 frac(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_frac)
half4 frac(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_frac)
float frac(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_frac)
float2 frac(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_frac)
float3 frac(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_frac)
float4 frac(float4);

//===----------------------------------------------------------------------===//
// isinf builtins
//===----------------------------------------------------------------------===//

/// \fn T isinf(T x)
/// \brief Determines if the specified value \a x  is infinite.
/// \param x The specified input value.
///
/// Returns a value of the same size as the input, with a value set
/// to True if the x parameter is +INF or -INF. Otherwise, False.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_isinf)
bool isinf(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_isinf)
bool2 isinf(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_isinf)
bool3 isinf(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_isinf)
bool4 isinf(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_isinf)
bool isinf(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_isinf)
bool2 isinf(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_isinf)
bool3 isinf(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_isinf)
bool4 isinf(float4);

//===----------------------------------------------------------------------===//
// lerp builtins
//===----------------------------------------------------------------------===//

/// \fn T lerp(T x, T y, T s)
/// \brief Returns the linear interpolation of x to y by s.
/// \param x [in] The first-floating point value.
/// \param y [in] The second-floating point value.
/// \param s [in] A value that linearly interpolates between the x parameter and
/// the y parameter.
///
/// Linear interpolation is based on the following formula: x*(1-s) + y*s which
/// can equivalently be written as x + s(y-x).

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_lerp)
half lerp(half, half, half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_lerp)
half2 lerp(half2, half2, half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_lerp)
half3 lerp(half3, half3, half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_lerp)
half4 lerp(half4, half4, half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_lerp)
float lerp(float, float, float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_lerp)
float2 lerp(float2, float2, float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_lerp)
float3 lerp(float3, float3, float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_lerp)
float4 lerp(float4, float4, float4);

//===----------------------------------------------------------------------===//
// log builtins
//===----------------------------------------------------------------------===//

/// \fn T log(T Val)
/// \brief The base-e logarithm of the input value, \a Val parameter.
/// \param Val The input value.
///
/// If \a Val is negative, this result is undefined. If \a Val is 0, this
/// function returns negative infinity.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log)
half log(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log)
half2 log(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log)
half3 log(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log)
half4 log(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log)
float log(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log)
float2 log(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log)
float3 log(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log)
float4 log(float4);

//===----------------------------------------------------------------------===//
// log10 builtins
//===----------------------------------------------------------------------===//

/// \fn T log10(T Val)
/// \brief The base-10 logarithm of the input value, \a Val parameter.
/// \param Val The input value.
///
/// If \a Val is negative, this result is undefined. If \a Val is 0, this
/// function returns negative infinity.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log10)
half log10(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log10)
half2 log10(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log10)
half3 log10(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log10)
half4 log10(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log10)
float log10(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log10)
float2 log10(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log10)
float3 log10(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log10)
float4 log10(float4);

//===----------------------------------------------------------------------===//
// log2 builtins
//===----------------------------------------------------------------------===//

/// \fn T log2(T Val)
/// \brief The base-2 logarithm of the input value, \a Val parameter.
/// \param Val The input value.
///
/// If \a Val is negative, this result is undefined. If \a Val is 0, this
/// function returns negative infinity.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log2)
half log2(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log2)
half2 log2(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log2)
half3 log2(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log2)
half4 log2(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log2)
float log2(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log2)
float2 log2(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log2)
float3 log2(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_log2)
float4 log2(float4);

//===----------------------------------------------------------------------===//
// mad builtins
//===----------------------------------------------------------------------===//

/// \fn T mad(T M, T A, T B)
/// \brief The result of \a M * \a A + \a B.
/// \param M The multiplication value.
/// \param A The first addition value.
/// \param B The second addition value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
half mad(half, half, half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
half2 mad(half2, half2, half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
half3 mad(half3, half3, half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
half4 mad(half4, half4, half4);

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int16_t mad(int16_t, int16_t, int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int16_t2 mad(int16_t2, int16_t2, int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int16_t3 mad(int16_t3, int16_t3, int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int16_t4 mad(int16_t4, int16_t4, int16_t4);

_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint16_t mad(uint16_t, uint16_t, uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint16_t2 mad(uint16_t2, uint16_t2, uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint16_t3 mad(uint16_t3, uint16_t3, uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint16_t4 mad(uint16_t4, uint16_t4, uint16_t4);
#endif

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int mad(int, int, int);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int2 mad(int2, int2, int2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int3 mad(int3, int3, int3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int4 mad(int4, int4, int4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint mad(uint, uint, uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint2 mad(uint2, uint2, uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint3 mad(uint3, uint3, uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint4 mad(uint4, uint4, uint4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int64_t mad(int64_t, int64_t, int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int64_t2 mad(int64_t2, int64_t2, int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int64_t3 mad(int64_t3, int64_t3, int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
int64_t4 mad(int64_t4, int64_t4, int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint64_t mad(uint64_t, uint64_t, uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint64_t2 mad(uint64_t2, uint64_t2, uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint64_t3 mad(uint64_t3, uint64_t3, uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
uint64_t4 mad(uint64_t4, uint64_t4, uint64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
float mad(float, float, float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
float2 mad(float2, float2, float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
float3 mad(float3, float3, float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
float4 mad(float4, float4, float4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
double mad(double, double, double);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
double2 mad(double2, double2, double2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
double3 mad(double3, double3, double3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_mad)
double4 mad(double4, double4, double4);

//===----------------------------------------------------------------------===//
// max builtins
//===----------------------------------------------------------------------===//

/// \fn T max(T X, T Y)
/// \brief Return the greater of \a X and \a Y.
/// \param X The X input value.
/// \param Y The Y input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
half max(half, half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
half2 max(half2, half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
half3 max(half3, half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
half4 max(half4, half4);

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int16_t max(int16_t, int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int16_t2 max(int16_t2, int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int16_t3 max(int16_t3, int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int16_t4 max(int16_t4, int16_t4);

_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint16_t max(uint16_t, uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint16_t2 max(uint16_t2, uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint16_t3 max(uint16_t3, uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint16_t4 max(uint16_t4, uint16_t4);
#endif

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int max(int, int);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int2 max(int2, int2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int3 max(int3, int3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int4 max(int4, int4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint max(uint, uint);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint2 max(uint2, uint2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint3 max(uint3, uint3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint4 max(uint4, uint4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int64_t max(int64_t, int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int64_t2 max(int64_t2, int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int64_t3 max(int64_t3, int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
int64_t4 max(int64_t4, int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint64_t max(uint64_t, uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint64_t2 max(uint64_t2, uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint64_t3 max(uint64_t3, uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
uint64_t4 max(uint64_t4, uint64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
float max(float, float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
float2 max(float2, float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
float3 max(float3, float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
float4 max(float4, float4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
double max(double, double);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
double2 max(double2, double2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
double3 max(double3, double3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_max)
double4 max(double4, double4);

//===----------------------------------------------------------------------===//
// min builtins
//===----------------------------------------------------------------------===//

/// \fn T min(T X, T Y)
/// \brief Return the lesser of \a X and \a Y.
/// \param X The X input value.
/// \param Y The Y input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
half min(half, half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
half2 min(half2, half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
half3 min(half3, half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
half4 min(half4, half4);

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int16_t min(int16_t, int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int16_t2 min(int16_t2, int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int16_t3 min(int16_t3, int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int16_t4 min(int16_t4, int16_t4);

_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint16_t min(uint16_t, uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint16_t2 min(uint16_t2, uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint16_t3 min(uint16_t3, uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint16_t4 min(uint16_t4, uint16_t4);
#endif

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int min(int, int);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int2 min(int2, int2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int3 min(int3, int3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int4 min(int4, int4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint min(uint, uint);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint2 min(uint2, uint2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint3 min(uint3, uint3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint4 min(uint4, uint4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
float min(float, float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
float2 min(float2, float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
float3 min(float3, float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
float4 min(float4, float4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int64_t min(int64_t, int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int64_t2 min(int64_t2, int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int64_t3 min(int64_t3, int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
int64_t4 min(int64_t4, int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint64_t min(uint64_t, uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint64_t2 min(uint64_t2, uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint64_t3 min(uint64_t3, uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
uint64_t4 min(uint64_t4, uint64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
double min(double, double);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
double2 min(double2, double2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
double3 min(double3, double3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_min)
double4 min(double4, double4);

//===----------------------------------------------------------------------===//
// normalize builtins
//===----------------------------------------------------------------------===//

/// \fn T normalize(T x)
/// \brief Returns the normalized unit vector of the specified floating-point
/// vector. \param x [in] The vector of floats.
///
/// Normalize is based on the following formula: x / length(x).

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_normalize)
half normalize(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_normalize)
half2 normalize(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_normalize)
half3 normalize(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_normalize)
half4 normalize(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_normalize)
float normalize(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_normalize)
float2 normalize(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_normalize)
float3 normalize(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_normalize)
float4 normalize(float4);

//===----------------------------------------------------------------------===//
// or builtins
//===----------------------------------------------------------------------===//

/// \fn bool or(bool x, bool y)
/// \brief Logically ors two boolean vectors elementwise and produces a bool
/// vector output.

// TODO: Clean up clang-format marker once we've resolved
//       https://github.com/llvm/llvm-project/issues/127851
//
// clang-format off
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_or)
bool or(bool, bool);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_or)
bool2 or(bool2, bool2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_or)
bool3 or(bool3, bool3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_or)
bool4 or(bool4, bool4);
// clang-format on

//===----------------------------------------------------------------------===//
// pow builtins
//===----------------------------------------------------------------------===//

/// \fn T pow(T Val, T Pow)
/// \brief Return the value \a Val, raised to the power \a Pow.
/// \param Val The input value.
/// \param Pow The specified power.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_pow)
half pow(half, half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_pow)
half2 pow(half2, half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_pow)
half3 pow(half3, half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_pow)
half4 pow(half4, half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_pow)
float pow(float, float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_pow)
float2 pow(float2, float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_pow)
float3 pow(float3, float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_pow)
float4 pow(float4, float4);

//===----------------------------------------------------------------------===//
// reversebits builtins
//===----------------------------------------------------------------------===//

/// \fn T reversebits(T Val)
/// \brief Return the value \a Val with the bit order reversed.
/// \param Val The input value.

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint16_t reversebits(uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint16_t2 reversebits(uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint16_t3 reversebits(uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint16_t4 reversebits(uint16_t4);
#endif

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint reversebits(uint);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint2 reversebits(uint2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint3 reversebits(uint3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint4 reversebits(uint4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint64_t reversebits(uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint64_t2 reversebits(uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint64_t3 reversebits(uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_bitreverse)
uint64_t4 reversebits(uint64_t4);

//===----------------------------------------------------------------------===//
// cross builtins
//===----------------------------------------------------------------------===//

/// \fn T cross(T x, T y)
/// \brief Returns the cross product of two floating-point, 3D vectors.
/// \param x [in] The first floating-point, 3D vector.
/// \param y [in] The second floating-point, 3D vector.
///
/// Result is the cross product of x and y, i.e., the resulting
/// components are, in order :
/// x[1] * y[2] - y[1] * x[2]
/// x[2] * y[0] - y[2] * x[0]
/// x[0] * y[1] - y[0] * x[1]

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_cross)
half3 cross(half3, half3);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_cross)
float3 cross(float3, float3);

//===----------------------------------------------------------------------===//
// rcp builtins
//===----------------------------------------------------------------------===//

/// \fn T rcp(T x)
/// \brief Calculates a fast, approximate, per-component reciprocal ie 1 / \a x.
/// \param x The specified input value.
///
/// The return value is the reciprocal of the \a x parameter.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
half rcp(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
half2 rcp(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
half3 rcp(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
half4 rcp(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
float rcp(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
float2 rcp(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
float3 rcp(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
float4 rcp(float4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
double rcp(double);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
double2 rcp(double2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
double3 rcp(double3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rcp)
double4 rcp(double4);

//===----------------------------------------------------------------------===//
// rsqrt builtins
//===----------------------------------------------------------------------===//

/// \fn T rsqrt(T x)
/// \brief Returns the reciprocal of the square root of the specified value.
/// ie 1 / sqrt( \a x).
/// \param x The specified input value.
///
/// This function uses the following formula: 1 / sqrt(x).

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rsqrt)
half rsqrt(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rsqrt)
half2 rsqrt(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rsqrt)
half3 rsqrt(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rsqrt)
half4 rsqrt(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rsqrt)
float rsqrt(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rsqrt)
float2 rsqrt(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rsqrt)
float3 rsqrt(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_rsqrt)
float4 rsqrt(float4);

//===----------------------------------------------------------------------===//
// round builtins
//===----------------------------------------------------------------------===//

/// \fn T round(T x)
/// \brief Rounds the specified value \a x to the nearest integer.
/// \param x The specified input value.
///
/// The return value is the \a x parameter, rounded to the nearest integer
/// within a floating-point type. Halfway cases are
/// rounded to the nearest even value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_roundeven)
half round(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_roundeven)
half2 round(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_roundeven)
half3 round(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_roundeven)
half4 round(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_roundeven)
float round(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_roundeven)
float2 round(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_roundeven)
float3 round(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_roundeven)
float4 round(float4);

//===----------------------------------------------------------------------===//
// saturate builtins
//===----------------------------------------------------------------------===//

/// \fn T saturate(T Val)
/// \brief Returns input value, \a Val, clamped within the range of 0.0f
/// to 1.0f. \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
half saturate(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
half2 saturate(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
half3 saturate(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
half4 saturate(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
float saturate(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
float2 saturate(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
float3 saturate(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
float4 saturate(float4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
double saturate(double);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
double2 saturate(double2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
double3 saturate(double3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_saturate)
double4 saturate(double4);

//===----------------------------------------------------------------------===//
// select builtins
//===----------------------------------------------------------------------===//

/// \fn T select(bool Cond, T TrueVal, T FalseVal)
/// \brief ternary operator.
/// \param Cond The Condition input value.
/// \param TrueVal The Value returned if Cond is true.
/// \param FalseVal The Value returned if Cond is false.

template <typename T>
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_select)
T select(bool, T, T);

/// \fn vector<T,Sz> select(vector<bool,Sz> Conds, vector<T,Sz> TrueVals,
///                         vector<T,Sz> FalseVals)
/// \brief ternary operator for vectors. All vectors must be the same size.
/// \param Conds The Condition input values.
/// \param TrueVals The vector values are chosen from when conditions are true.
/// \param FalseVals The vector values are chosen from when conditions are
/// false.

template <typename T, int Sz>
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_select)
vector<T, Sz> select(vector<bool, Sz>, vector<T, Sz>, vector<T, Sz>);

/// \fn vector<T,Sz> select(vector<bool,Sz> Conds, T TrueVal,
///                         vector<T,Sz> FalseVals)
/// \brief ternary operator for vectors. All vectors must be the same size.
/// \param Conds The Condition input values.
/// \param TrueVal The scalar value to splat from when conditions are true.
/// \param FalseVals The vector values are chosen from when conditions are
/// false.

template <typename T, int Sz>
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_select)
vector<T, Sz> select(vector<bool, Sz>, T, vector<T, Sz>);

/// \fn vector<T,Sz> select(vector<bool,Sz> Conds, vector<T,Sz> TrueVals,
///                         T FalseVal)
/// \brief ternary operator for vectors. All vectors must be the same size.
/// \param Conds The Condition input values.
/// \param TrueVals The vector values are chosen from when conditions are true.
/// \param FalseVal The scalar value to splat from when conditions are false.

template <typename T, int Sz>
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_select)
vector<T, Sz> select(vector<bool, Sz>, vector<T, Sz>, T);

/// \fn vector<T,Sz> select(vector<bool,Sz> Conds, vector<T,Sz> TrueVals,
///                         T FalseVal)
/// \brief ternary operator for vectors. All vectors must be the same size.
/// \param Conds The Condition input values.
/// \param TrueVal The scalar value to splat from when conditions are true.
/// \param FalseVal The scalar value to splat from when conditions are false.

template <typename T, int Sz>
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_select)
__detail::enable_if_t<__detail::is_arithmetic<T>::Value, vector<T, Sz>> select(
    vector<bool, Sz>, T, T);

//===----------------------------------------------------------------------===//
// sin builtins
//===----------------------------------------------------------------------===//

/// \fn T sin(T Val)
/// \brief Returns the sine of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sin)
half sin(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sin)
half2 sin(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sin)
half3 sin(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sin)
half4 sin(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sin)
float sin(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sin)
float2 sin(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sin)
float3 sin(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sin)
float4 sin(float4);

//===----------------------------------------------------------------------===//
// sinh builtins
//===----------------------------------------------------------------------===//

/// \fn T sinh(T Val)
/// \brief Returns the hyperbolic sine of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sinh)
half sinh(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sinh)
half2 sinh(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sinh)
half3 sinh(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sinh)
half4 sinh(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sinh)
float sinh(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sinh)
float2 sinh(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sinh)
float3 sinh(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sinh)
float4 sinh(float4);

//===----------------------------------------------------------------------===//
// sqrt builtins
//===----------------------------------------------------------------------===//

/// \fn T sqrt(T Val)
/// \brief Returns the square root of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sqrt)
half sqrt(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sqrt)
half2 sqrt(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sqrt)
half3 sqrt(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sqrt)
half4 sqrt(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sqrt)
float sqrt(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sqrt)
float2 sqrt(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sqrt)
float3 sqrt(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_sqrt)
float4 sqrt(float4);

//===----------------------------------------------------------------------===//
// step builtins
//===----------------------------------------------------------------------===//

/// \fn T step(T x, T y)
/// \brief Returns 1 if the x parameter is greater than or equal to the y
/// parameter; otherwise, 0. vector. \param x [in] The first floating-point
/// value to compare. \param y [in] The first floating-point value to compare.
///
/// Step is based on the following formula: (x >= y) ? 1 : 0

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_step)
half step(half, half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_step)
half2 step(half2, half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_step)
half3 step(half3, half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_step)
half4 step(half4, half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_step)
float step(float, float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_step)
float2 step(float2, float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_step)
float3 step(float3, float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_step)
float4 step(float4, float4);

//===----------------------------------------------------------------------===//
// tan builtins
//===----------------------------------------------------------------------===//

/// \fn T tan(T Val)
/// \brief Returns the tangent of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tan)
half tan(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tan)
half2 tan(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tan)
half3 tan(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tan)
half4 tan(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tan)
float tan(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tan)
float2 tan(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tan)
float3 tan(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tan)
float4 tan(float4);

//===----------------------------------------------------------------------===//
// tanh builtins
//===----------------------------------------------------------------------===//

/// \fn T tanh(T Val)
/// \brief Returns the hyperbolic tangent of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tanh)
half tanh(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tanh)
half2 tanh(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tanh)
half3 tanh(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tanh)
half4 tanh(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tanh)
float tanh(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tanh)
float2 tanh(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tanh)
float3 tanh(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_tanh)
float4 tanh(float4);

//===----------------------------------------------------------------------===//
// trunc builtins
//===----------------------------------------------------------------------===//

/// \fn T trunc(T Val)
/// \brief Returns the truncated integer value of the input value, \a Val.
/// \param Val The input value.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_trunc)
half trunc(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_trunc)
half2 trunc(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_trunc)
half3 trunc(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_trunc)
half4 trunc(half4);

_HLSL_BUILTIN_ALIAS(__builtin_elementwise_trunc)
float trunc(float);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_trunc)
float2 trunc(float2);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_trunc)
float3 trunc(float3);
_HLSL_BUILTIN_ALIAS(__builtin_elementwise_trunc)
float4 trunc(float4);

//===----------------------------------------------------------------------===//
// Wave* builtins
//===----------------------------------------------------------------------===//

/// \brief Returns true if the expression is true in all active lanes in the
/// current wave.
///
/// \param Val The boolean expression to evaluate.
/// \return True if the expression is true in all lanes.
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_all_true)
__attribute__((convergent)) bool WaveActiveAllTrue(bool Val);

/// \brief Returns true if the expression is true in any active lane in the
/// current wave.
///
/// \param Val The boolean expression to evaluate.
/// \return True if the expression is true in any lane.
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_any_true)
__attribute__((convergent)) bool WaveActiveAnyTrue(bool Val);

/// \brief Counts the number of boolean variables which evaluate to true across
/// all active lanes in the current wave.
///
/// \param Val The input boolean value.
/// \return The number of lanes for which the boolean variable evaluates to
/// true, across all active lanes in the current wave.
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_count_bits)
__attribute__((convergent)) uint WaveActiveCountBits(bool Val);

/// \brief Returns the index of the current lane within the current wave.
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_get_lane_index)
__attribute__((convergent)) uint WaveGetLaneIndex();

_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_is_first_lane)
__attribute__((convergent)) bool WaveIsFirstLane();

//===----------------------------------------------------------------------===//
// WaveReadLaneAt builtins
//===----------------------------------------------------------------------===//

// \brief Returns the value of the expression for the given lane index within
// the specified wave.

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) bool WaveReadLaneAt(bool, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) bool2 WaveReadLaneAt(bool2, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) bool3 WaveReadLaneAt(bool3, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) bool4 WaveReadLaneAt(bool4, uint32_t);

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int16_t WaveReadLaneAt(int16_t, uint32_t);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int16_t2 WaveReadLaneAt(int16_t2, uint32_t);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int16_t3 WaveReadLaneAt(int16_t3, uint32_t);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int16_t4 WaveReadLaneAt(int16_t4, uint32_t);

_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint16_t WaveReadLaneAt(uint16_t, uint32_t);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint16_t2 WaveReadLaneAt(uint16_t2, uint32_t);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint16_t3 WaveReadLaneAt(uint16_t3, uint32_t);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint16_t4 WaveReadLaneAt(uint16_t4, uint32_t);
#endif

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) half WaveReadLaneAt(half, uint32_t);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) half2 WaveReadLaneAt(half2, uint32_t);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) half3 WaveReadLaneAt(half3, uint32_t);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) half4 WaveReadLaneAt(half4, uint32_t);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int WaveReadLaneAt(int, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int2 WaveReadLaneAt(int2, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int3 WaveReadLaneAt(int3, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int4 WaveReadLaneAt(int4, uint32_t);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint WaveReadLaneAt(uint, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint2 WaveReadLaneAt(uint2, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint3 WaveReadLaneAt(uint3, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint4 WaveReadLaneAt(uint4, uint32_t);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) float WaveReadLaneAt(float, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) float2 WaveReadLaneAt(float2, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) float3 WaveReadLaneAt(float3, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) float4 WaveReadLaneAt(float4, uint32_t);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int64_t WaveReadLaneAt(int64_t, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int64_t2 WaveReadLaneAt(int64_t2, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int64_t3 WaveReadLaneAt(int64_t3, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) int64_t4 WaveReadLaneAt(int64_t4, uint32_t);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint64_t WaveReadLaneAt(uint64_t, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint64_t2 WaveReadLaneAt(uint64_t2, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint64_t3 WaveReadLaneAt(uint64_t3, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) uint64_t4 WaveReadLaneAt(uint64_t4, uint32_t);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) double WaveReadLaneAt(double, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) double2 WaveReadLaneAt(double2, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) double3 WaveReadLaneAt(double3, uint32_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_read_lane_at)
__attribute__((convergent)) double4 WaveReadLaneAt(double4, uint32_t);

//===----------------------------------------------------------------------===//
// WaveActiveMax builtins
//===----------------------------------------------------------------------===//

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) half WaveActiveMax(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) half2 WaveActiveMax(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) half3 WaveActiveMax(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) half4 WaveActiveMax(half4);

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int16_t WaveActiveMax(int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int16_t2 WaveActiveMax(int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int16_t3 WaveActiveMax(int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int16_t4 WaveActiveMax(int16_t4);

_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint16_t WaveActiveMax(uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint16_t2 WaveActiveMax(uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint16_t3 WaveActiveMax(uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint16_t4 WaveActiveMax(uint16_t4);
#endif

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int WaveActiveMax(int);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int2 WaveActiveMax(int2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int3 WaveActiveMax(int3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int4 WaveActiveMax(int4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint WaveActiveMax(uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint2 WaveActiveMax(uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint3 WaveActiveMax(uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint4 WaveActiveMax(uint4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int64_t WaveActiveMax(int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int64_t2 WaveActiveMax(int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int64_t3 WaveActiveMax(int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) int64_t4 WaveActiveMax(int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint64_t WaveActiveMax(uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint64_t2 WaveActiveMax(uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint64_t3 WaveActiveMax(uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) uint64_t4 WaveActiveMax(uint64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) float WaveActiveMax(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) float2 WaveActiveMax(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) float3 WaveActiveMax(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) float4 WaveActiveMax(float4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) double WaveActiveMax(double);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) double2 WaveActiveMax(double2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) double3 WaveActiveMax(double3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_max)
__attribute__((convergent)) double4 WaveActiveMax(double4);

//===----------------------------------------------------------------------===//
// WaveActiveSum builtins
//===----------------------------------------------------------------------===//

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) half WaveActiveSum(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) half2 WaveActiveSum(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) half3 WaveActiveSum(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) half4 WaveActiveSum(half4);

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int16_t WaveActiveSum(int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int16_t2 WaveActiveSum(int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int16_t3 WaveActiveSum(int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int16_t4 WaveActiveSum(int16_t4);

_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint16_t WaveActiveSum(uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint16_t2 WaveActiveSum(uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint16_t3 WaveActiveSum(uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.0)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint16_t4 WaveActiveSum(uint16_t4);
#endif

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int WaveActiveSum(int);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int2 WaveActiveSum(int2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int3 WaveActiveSum(int3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int4 WaveActiveSum(int4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint WaveActiveSum(uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint2 WaveActiveSum(uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint3 WaveActiveSum(uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint4 WaveActiveSum(uint4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int64_t WaveActiveSum(int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int64_t2 WaveActiveSum(int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int64_t3 WaveActiveSum(int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) int64_t4 WaveActiveSum(int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint64_t WaveActiveSum(uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint64_t2 WaveActiveSum(uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint64_t3 WaveActiveSum(uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) uint64_t4 WaveActiveSum(uint64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) float WaveActiveSum(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) float2 WaveActiveSum(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) float3 WaveActiveSum(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) float4 WaveActiveSum(float4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) double WaveActiveSum(double);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) double2 WaveActiveSum(double2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) double3 WaveActiveSum(double3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_wave_active_sum)
__attribute__((convergent)) double4 WaveActiveSum(double4);

//===----------------------------------------------------------------------===//
// sign builtins
//===----------------------------------------------------------------------===//

/// \fn T sign(T Val)
/// \brief Returns -1 if \a Val is less than zero; 0 if \a Val equals zero; and
/// 1 if \a Val is greater than zero. \param Val The input value.

#ifdef __HLSL_ENABLE_16_BIT
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int sign(int16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int2 sign(int16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int3 sign(int16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int4 sign(int16_t4);

_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int sign(uint16_t);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int2 sign(uint16_t2);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int3 sign(uint16_t3);
_HLSL_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int4 sign(uint16_t4);
#endif

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int sign(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int2 sign(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int3 sign(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int4 sign(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int sign(int);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int2 sign(int2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int3 sign(int3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int4 sign(int4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int sign(uint);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int2 sign(uint2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int3 sign(uint3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int4 sign(uint4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int sign(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int2 sign(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int3 sign(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int4 sign(float4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int sign(int64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int2 sign(int64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int3 sign(int64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int4 sign(int64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int sign(uint64_t);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int2 sign(uint64_t2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int3 sign(uint64_t3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int4 sign(uint64_t4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int sign(double);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int2 sign(double2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int3 sign(double3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_sign)
int4 sign(double4);

//===----------------------------------------------------------------------===//
// radians builtins
//===----------------------------------------------------------------------===//

/// \fn T radians(T Val)
/// \brief Converts the specified value from degrees to radians.

_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_radians)
half radians(half);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_radians)
half2 radians(half2);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_radians)
half3 radians(half3);
_HLSL_16BIT_AVAILABILITY(shadermodel, 6.2)
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_radians)
half4 radians(half4);

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_radians)
float radians(float);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_radians)
float2 radians(float2);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_radians)
float3 radians(float3);
_HLSL_BUILTIN_ALIAS(__builtin_hlsl_elementwise_radians)
float4 radians(float4);

//===----------------------------------------------------------------------===//
// GroupMemoryBarrierWithGroupSync builtins
//===----------------------------------------------------------------------===//

/// \fn void GroupMemoryBarrierWithGroupSync(void)
/// \brief Blocks execution of all threads in a group until all group shared
/// accesses have been completed and all threads in the group have reached this
/// call.

_HLSL_BUILTIN_ALIAS(__builtin_hlsl_group_memory_barrier_with_group_sync)
void GroupMemoryBarrierWithGroupSync(void);

} // namespace hlsl
#endif //_HLSL_HLSL_ALIAS_INTRINSICS_H_
