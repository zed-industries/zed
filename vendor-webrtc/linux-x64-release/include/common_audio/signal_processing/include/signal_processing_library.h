/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

/*
 * This header file includes all of the fix point signal processing library
 * (SPL) function descriptions and declarations. For specific function calls,
 * see bottom of file.
 */

#ifndef COMMON_AUDIO_SIGNAL_PROCESSING_INCLUDE_SIGNAL_PROCESSING_LIBRARY_H_
#define COMMON_AUDIO_SIGNAL_PROCESSING_INCLUDE_SIGNAL_PROCESSING_LIBRARY_H_

#include <string.h>

#include "common_audio/signal_processing/dot_product_with_scale.h"

// Macros specific for the fixed point implementation
#define WEBRTC_SPL_WORD16_MAX 32767
#define WEBRTC_SPL_WORD16_MIN -32768
#define WEBRTC_SPL_WORD32_MAX (int32_t)0x7fffffff
#define WEBRTC_SPL_WORD32_MIN (int32_t)0x80000000
#define WEBRTC_SPL_MAX_LPC_ORDER 14
#define WEBRTC_SPL_MIN(A, B) (A < B ? A : B)  // Get min value
#define WEBRTC_SPL_MAX(A, B) (A > B ? A : B)  // Get max value
// TODO(kma/bjorn): For the next two macros, investigate how to correct the code
// for inputs of a = WEBRTC_SPL_WORD16_MIN or WEBRTC_SPL_WORD32_MIN.
#define WEBRTC_SPL_ABS_W16(a) (((int16_t)a >= 0) ? ((int16_t)a) : -((int16_t)a))
#define WEBRTC_SPL_ABS_W32(a) (((int32_t)a >= 0) ? ((int32_t)a) : -((int32_t)a))

#define WEBRTC_SPL_MUL(a, b) ((int32_t)((int32_t)(a) * (int32_t)(b)))
#define WEBRTC_SPL_UMUL(a, b) ((uint32_t)((uint32_t)(a) * (uint32_t)(b)))
#define WEBRTC_SPL_UMUL_32_16(a, b) ((uint32_t)((uint32_t)(a) * (uint16_t)(b)))
#define WEBRTC_SPL_MUL_16_U16(a, b) ((int32_t)(int16_t)(a) * (uint16_t)(b))

// clang-format off
// clang-format would choose some indentation
// leading to presubmit error (cpplint.py)
#ifndef WEBRTC_ARCH_ARM_V7
// For ARMv7 platforms, these are inline functions in spl_inl_armv7.h
#ifndef MIPS32_LE
// For MIPS platforms, these are inline functions in spl_inl_mips.h
#define WEBRTC_SPL_MUL_16_16(a, b) ((int32_t)(((int16_t)(a)) * ((int16_t)(b))))
#define WEBRTC_SPL_MUL_16_32_RSFT16(a, b) \
        (WEBRTC_SPL_MUL_16_16(a, b >> 16) +     \
        ((WEBRTC_SPL_MUL_16_16(a, (b & 0xffff) >> 1) + 0x4000) >> 15))
#endif
#endif

#define WEBRTC_SPL_MUL_16_32_RSFT11(a, b)          \
        (WEBRTC_SPL_MUL_16_16(a, (b) >> 16) * (1 << 5) + \
        (((WEBRTC_SPL_MUL_16_U16(a, (uint16_t)(b)) >> 1) + 0x0200) >> 10))
#define WEBRTC_SPL_MUL_16_32_RSFT14(a, b)          \
        (WEBRTC_SPL_MUL_16_16(a, (b) >> 16) * (1 << 2) + \
        (((WEBRTC_SPL_MUL_16_U16(a, (uint16_t)(b)) >> 1) + 0x1000) >> 13))
#define WEBRTC_SPL_MUL_16_32_RSFT15(a, b)            \
        ((WEBRTC_SPL_MUL_16_16(a, (b) >> 16) * (1 << 1)) + \
        (((WEBRTC_SPL_MUL_16_U16(a, (uint16_t)(b)) >> 1) + 0x2000) >> 14))
// clang-format on

#define WEBRTC_SPL_MUL_16_16_RSFT(a, b, c) (WEBRTC_SPL_MUL_16_16(a, b) >> (c))

#define WEBRTC_SPL_MUL_16_16_RSFT_WITH_ROUND(a, b, c) \
  ((WEBRTC_SPL_MUL_16_16(a, b) + ((int32_t)(((int32_t)1) << ((c)-1)))) >> (c))

// C + the 32 most significant bits of A * B
#define WEBRTC_SPL_SCALEDIFF32(A, B, C) \
  (C + (B >> 16) * A + (((uint32_t)(B & 0x0000FFFF) * A) >> 16))

#define WEBRTC_SPL_SAT(a, b, c) (b > a ? a : b < c ? c : b)

// Shifting with negative numbers allowed
// Positive means left shift
#define WEBRTC_SPL_SHIFT_W32(x, c) ((c) >= 0 ? (x) * (1 << (c)) : (x) >> -(c))

// Shifting with negative numbers not allowed
// We cannot do casting here due to signed/unsigned problem
#define WEBRTC_SPL_LSHIFT_W32(x, c) ((x) << (c))

#define WEBRTC_SPL_RSHIFT_U32(x, c) ((uint32_t)(x) >> (c))

#define WEBRTC_SPL_RAND(a) ((int16_t)((((int16_t)a * 18816) >> 7) & 0x00007fff))

#ifdef __cplusplus
extern "C" {
#endif

#define WEBRTC_SPL_MEMCPY_W16(v1, v2, length) \
  memcpy(v1, v2, (length) * sizeof(int16_t))

// inline functions:
#include "common_audio/signal_processing/include/spl_inl.h"

// third party math functions
#include "common_audio/third_party/spl_sqrt_floor/spl_sqrt_floor.h"

int16_t WebRtcSpl_GetScalingSquare(int16_t* in_vector,
                                   size_t in_vector_length,
                                   size_t times);

// Copy and set operations. Implementation in copy_set_operations.c.
// Descriptions at bottom of file.
void WebRtcSpl_MemSetW16(int16_t* vector,
                         int16_t set_value,
                         size_t vector_length);
void WebRtcSpl_MemSetW32(int32_t* vector,
                         int32_t set_value,
                         size_t vector_length);
void WebRtcSpl_MemCpyReversedOrder(int16_t* out_vector,
                                   int16_t* in_vector,
                                   size_t vector_length);
void WebRtcSpl_CopyFromEndW16(const int16_t* in_vector,
                              size_t in_vector_length,
                              size_t samples,
                              int16_t* out_vector);
void WebRtcSpl_ZerosArrayW16(int16_t* vector, size_t vector_length);
void WebRtcSpl_ZerosArrayW32(int32_t* vector, size_t vector_length);
// End: Copy and set operations.

// Minimum and maximum operation functions and their pointers.
// Implementation in min_max_operations.c.

// Returns the largest absolute value in a signed 16-bit vector.
//
// Input:
//      - vector : 16-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Maximum absolute value in vector.
typedef int16_t (*MaxAbsValueW16)(const int16_t* vector, size_t length);
extern const MaxAbsValueW16 WebRtcSpl_MaxAbsValueW16;
int16_t WebRtcSpl_MaxAbsValueW16C(const int16_t* vector, size_t length);
#if defined(WEBRTC_HAS_NEON)
int16_t WebRtcSpl_MaxAbsValueW16Neon(const int16_t* vector, size_t length);
#endif
#if defined(MIPS32_LE)
int16_t WebRtcSpl_MaxAbsValueW16_mips(const int16_t* vector, size_t length);
#endif

// Returns the largest absolute value in a signed 32-bit vector.
//
// Input:
//      - vector : 32-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Maximum absolute value in vector.
typedef int32_t (*MaxAbsValueW32)(const int32_t* vector, size_t length);
extern const MaxAbsValueW32 WebRtcSpl_MaxAbsValueW32;
int32_t WebRtcSpl_MaxAbsValueW32C(const int32_t* vector, size_t length);
#if defined(WEBRTC_HAS_NEON)
int32_t WebRtcSpl_MaxAbsValueW32Neon(const int32_t* vector, size_t length);
#endif
#if defined(MIPS_DSP_R1_LE)
int32_t WebRtcSpl_MaxAbsValueW32_mips(const int32_t* vector, size_t length);
#endif

// Returns the maximum value of a 16-bit vector.
//
// Input:
//      - vector : 16-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Maximum sample value in `vector`.
typedef int16_t (*MaxValueW16)(const int16_t* vector, size_t length);
extern const MaxValueW16 WebRtcSpl_MaxValueW16;
int16_t WebRtcSpl_MaxValueW16C(const int16_t* vector, size_t length);
#if defined(WEBRTC_HAS_NEON)
int16_t WebRtcSpl_MaxValueW16Neon(const int16_t* vector, size_t length);
#endif
#if defined(MIPS32_LE)
int16_t WebRtcSpl_MaxValueW16_mips(const int16_t* vector, size_t length);
#endif

// Returns the maximum value of a 32-bit vector.
//
// Input:
//      - vector : 32-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Maximum sample value in `vector`.
typedef int32_t (*MaxValueW32)(const int32_t* vector, size_t length);
extern const MaxValueW32 WebRtcSpl_MaxValueW32;
int32_t WebRtcSpl_MaxValueW32C(const int32_t* vector, size_t length);
#if defined(WEBRTC_HAS_NEON)
int32_t WebRtcSpl_MaxValueW32Neon(const int32_t* vector, size_t length);
#endif
#if defined(MIPS32_LE)
int32_t WebRtcSpl_MaxValueW32_mips(const int32_t* vector, size_t length);
#endif

// Returns the minimum value of a 16-bit vector.
//
// Input:
//      - vector : 16-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Minimum sample value in `vector`.
typedef int16_t (*MinValueW16)(const int16_t* vector, size_t length);
extern const MinValueW16 WebRtcSpl_MinValueW16;
int16_t WebRtcSpl_MinValueW16C(const int16_t* vector, size_t length);
#if defined(WEBRTC_HAS_NEON)
int16_t WebRtcSpl_MinValueW16Neon(const int16_t* vector, size_t length);
#endif
#if defined(MIPS32_LE)
int16_t WebRtcSpl_MinValueW16_mips(const int16_t* vector, size_t length);
#endif

// Returns the minimum value of a 32-bit vector.
//
// Input:
//      - vector : 32-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Minimum sample value in `vector`.
typedef int32_t (*MinValueW32)(const int32_t* vector, size_t length);
extern const MinValueW32 WebRtcSpl_MinValueW32;
int32_t WebRtcSpl_MinValueW32C(const int32_t* vector, size_t length);
#if defined(WEBRTC_HAS_NEON)
int32_t WebRtcSpl_MinValueW32Neon(const int32_t* vector, size_t length);
#endif
#if defined(MIPS32_LE)
int32_t WebRtcSpl_MinValueW32_mips(const int32_t* vector, size_t length);
#endif

// Returns both the minimum and maximum values of a 16-bit vector.
//
// Input:
//      - vector : 16-bit input vector.
//      - length : Number of samples in vector.
// Ouput:
//      - max_val : Maximum sample value in `vector`.
//      - min_val : Minimum sample value in `vector`.
void WebRtcSpl_MinMaxW16(const int16_t* vector,
                         size_t length,
                         int16_t* min_val,
                         int16_t* max_val);
#if defined(WEBRTC_HAS_NEON)
void WebRtcSpl_MinMaxW16Neon(const int16_t* vector,
                             size_t length,
                             int16_t* min_val,
                             int16_t* max_val);
#endif

// Returns the vector index to the largest absolute value of a 16-bit vector.
//
// Input:
//      - vector : 16-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Index to the maximum absolute value in vector.
//                 If there are multiple equal maxima, return the index of the
//                 first. -32768 will always have precedence over 32767 (despite
//                 -32768 presenting an int16 absolute value of 32767).
size_t WebRtcSpl_MaxAbsIndexW16(const int16_t* vector, size_t length);

// Returns the element with the largest absolute value of a 16-bit vector. Note
// that this function can return a negative value.
//
// Input:
//      - vector : 16-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : The element with the largest absolute value. Note that this
//                 may be a negative value.
int16_t WebRtcSpl_MaxAbsElementW16(const int16_t* vector, size_t length);

// Returns the vector index to the maximum sample value of a 16-bit vector.
//
// Input:
//      - vector : 16-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Index to the maximum value in vector (if multiple
//                 indexes have the maximum, return the first).
size_t WebRtcSpl_MaxIndexW16(const int16_t* vector, size_t length);

// Returns the vector index to the maximum sample value of a 32-bit vector.
//
// Input:
//      - vector : 32-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Index to the maximum value in vector (if multiple
//                 indexes have the maximum, return the first).
size_t WebRtcSpl_MaxIndexW32(const int32_t* vector, size_t length);

// Returns the vector index to the minimum sample value of a 16-bit vector.
//
// Input:
//      - vector : 16-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Index to the mimimum value in vector  (if multiple
//                 indexes have the minimum, return the first).
size_t WebRtcSpl_MinIndexW16(const int16_t* vector, size_t length);

// Returns the vector index to the minimum sample value of a 32-bit vector.
//
// Input:
//      - vector : 32-bit input vector.
//      - length : Number of samples in vector.
//
// Return value  : Index to the mimimum value in vector  (if multiple
//                 indexes have the minimum, return the first).
size_t WebRtcSpl_MinIndexW32(const int32_t* vector, size_t length);

// End: Minimum and maximum operations.

// Vector scaling operations. Implementation in vector_scaling_operations.c.
// Description at bottom of file.
void WebRtcSpl_VectorBitShiftW16(int16_t* out_vector,
                                 size_t vector_length,
                                 const int16_t* in_vector,
                                 int16_t right_shifts);
void WebRtcSpl_VectorBitShiftW32(int32_t* out_vector,
                                 size_t vector_length,
                                 const int32_t* in_vector,
                                 int16_t right_shifts);
void WebRtcSpl_VectorBitShiftW32ToW16(int16_t* out_vector,
                                      size_t vector_length,
                                      const int32_t* in_vector,
                                      int right_shifts);
void WebRtcSpl_ScaleVector(const int16_t* in_vector,
                           int16_t* out_vector,
                           int16_t gain,
                           size_t vector_length,
                           int16_t right_shifts);
void WebRtcSpl_ScaleVectorWithSat(const int16_t* in_vector,
                                  int16_t* out_vector,
                                  int16_t gain,
                                  size_t vector_length,
                                  int16_t right_shifts);
void WebRtcSpl_ScaleAndAddVectors(const int16_t* in_vector1,
                                  int16_t gain1,
                                  int right_shifts1,
                                  const int16_t* in_vector2,
                                  int16_t gain2,
                                  int right_shifts2,
                                  int16_t* out_vector,
                                  size_t vector_length);

// The functions (with related pointer) perform the vector operation:
//   out_vector[k] = ((scale1 * in_vector1[k]) + (scale2 * in_vector2[k])
//        + round_value) >> right_shifts,
//   where  round_value = (1 << right_shifts) >> 1.
//
// Input:
//      - in_vector1       : Input vector 1
//      - in_vector1_scale : Gain to be used for vector 1
//      - in_vector2       : Input vector 2
//      - in_vector2_scale : Gain to be used for vector 2
//      - right_shifts     : Number of right bit shifts to be applied
//      - length           : Number of elements in the input vectors
//
// Output:
//      - out_vector       : Output vector
// Return value            : 0 if OK, -1 if (in_vector1 == null
//                           || in_vector2 == null || out_vector == null
//                           || length <= 0 || right_shift < 0).
typedef int (*ScaleAndAddVectorsWithRound)(const int16_t* in_vector1,
                                           int16_t in_vector1_scale,
                                           const int16_t* in_vector2,
                                           int16_t in_vector2_scale,
                                           int right_shifts,
                                           int16_t* out_vector,
                                           size_t length);
extern const ScaleAndAddVectorsWithRound WebRtcSpl_ScaleAndAddVectorsWithRound;
int WebRtcSpl_ScaleAndAddVectorsWithRoundC(const int16_t* in_vector1,
                                           int16_t in_vector1_scale,
                                           const int16_t* in_vector2,
                                           int16_t in_vector2_scale,
                                           int right_shifts,
                                           int16_t* out_vector,
                                           size_t length);
#if defined(MIPS_DSP_R1_LE)
int WebRtcSpl_ScaleAndAddVectorsWithRound_mips(const int16_t* in_vector1,
                                               int16_t in_vector1_scale,
                                               const int16_t* in_vector2,
                                               int16_t in_vector2_scale,
                                               int right_shifts,
                                               int16_t* out_vector,
                                               size_t length);
#endif
// End: Vector scaling operations.

//
// WebRtcSpl_ReverseOrderMultArrayElements(...)
//
// Performs the vector operation:
//  out_vector[n] = (in_vector[n]*window[-n])>>right_shifts
//
// Input:
//      - in_vector     : Input vector
//      - window        : Window vector (should be reversed). The pointer
//                        should be set to the last value in the vector
//      - right_shifts  : Number of right bit shift to be applied after the
//                        multiplication
//      - vector_length : Number of elements in `in_vector`
//
// Output:
//      - out_vector    : Output vector (can be same as `in_vector`)
//
void WebRtcSpl_ReverseOrderMultArrayElements(int16_t* out_vector,
                                             const int16_t* in_vector,
                                             const int16_t* window,
                                             size_t vector_length,
                                             int16_t right_shifts);

//
// WebRtcSpl_ElementwiseVectorMult(...)
//
// Performs the vector operation:
//  out_vector[n] = (in_vector[n]*window[n])>>right_shifts
//
// Input:
//      - in_vector     : Input vector
//      - window        : Window vector.
//      - right_shifts  : Number of right bit shift to be applied after the
//                        multiplication
//      - vector_length : Number of elements in `in_vector`
//
// Output:
//      - out_vector    : Output vector (can be same as `in_vector`)
//
void WebRtcSpl_ElementwiseVectorMult(int16_t* out_vector,
                                     const int16_t* in_vector,
                                     const int16_t* window,
                                     size_t vector_length,
                                     int16_t right_shifts);

//
// WebRtcSpl_AddVectorsAndShift(...)
//
// Performs the vector operation:
//  out_vector[k] = (in_vector1[k] + in_vector2[k])>>right_shifts
//
// Input:
//      - in_vector1    : Input vector 1
//      - in_vector2    : Input vector 2
//      - right_shifts  : Number of right bit shift to be applied after the
//                        multiplication
//      - vector_length : Number of elements in `in_vector1` and `in_vector2`
//
// Output:
//      - out_vector    : Output vector (can be same as `in_vector1`)
//
void WebRtcSpl_AddVectorsAndShift(int16_t* out_vector,
                                  const int16_t* in_vector1,
                                  const int16_t* in_vector2,
                                  size_t vector_length,
                                  int16_t right_shifts);

//
// WebRtcSpl_AddAffineVectorToVector(...)
//
// Adds an affine transformed vector to another vector `out_vector`, i.e,
// performs
//  out_vector[k] += (in_vector[k]*gain+add_constant)>>right_shifts
//
// Input:
//      - in_vector     : Input vector
//      - gain          : Gain value, used to multiply the in vector with
//      - add_constant  : Constant value to add (usually 1<<(right_shifts-1),
//                        but others can be used as well
//      - right_shifts  : Number of right bit shifts (0-16)
//      - vector_length : Number of samples in `in_vector` and `out_vector`
//
// Output:
//      - out_vector    : Vector with the output
//
void WebRtcSpl_AddAffineVectorToVector(int16_t* out_vector,
                                       const int16_t* in_vector,
                                       int16_t gain,
                                       int32_t add_constant,
                                       int16_t right_shifts,
                                       size_t vector_length);

//
// WebRtcSpl_AffineTransformVector(...)
//
// Affine transforms a vector, i.e, performs
//  out_vector[k] = (in_vector[k]*gain+add_constant)>>right_shifts
//
// Input:
//      - in_vector     : Input vector
//      - gain          : Gain value, used to multiply the in vector with
//      - add_constant  : Constant value to add (usually 1<<(right_shifts-1),
//                        but others can be used as well
//      - right_shifts  : Number of right bit shifts (0-16)
//      - vector_length : Number of samples in `in_vector` and `out_vector`
//
// Output:
//      - out_vector    : Vector with the output
//
void WebRtcSpl_AffineTransformVector(int16_t* out_vector,
                                     const int16_t* in_vector,
                                     int16_t gain,
                                     int32_t add_constant,
                                     int16_t right_shifts,
                                     size_t vector_length);

// Signal processing operations.

// A 32-bit fix-point implementation of auto-correlation computation
//
// Input:
//      - in_vector        : Vector to calculate autocorrelation upon
//      - in_vector_length : Length (in samples) of `vector`
//      - order            : The order up to which the autocorrelation should be
//                           calculated
//
// Output:
//      - result           : auto-correlation values (values should be seen
//                           relative to each other since the absolute values
//                           might have been down shifted to avoid overflow)
//
//      - scale            : The number of left shifts required to obtain the
//                           auto-correlation in Q0
//
// Return value            : Number of samples in `result`, i.e. (order+1)
size_t WebRtcSpl_AutoCorrelation(const int16_t* in_vector,
                                 size_t in_vector_length,
                                 size_t order,
                                 int32_t* result,
                                 int* scale);

// A 32-bit fix-point implementation of the Levinson-Durbin algorithm that
// does NOT use the 64 bit class
//
// Input:
//      - auto_corr : Vector with autocorrelation values of length >= `order`+1
//      - order     : The LPC filter order (support up to order 20)
//
// Output:
//      - lpc_coef  : lpc_coef[0..order] LPC coefficients in Q12
//      - refl_coef : refl_coef[0...order-1]| Reflection coefficients in Q15
//
// Return value     : 1 for stable 0 for unstable
int16_t WebRtcSpl_LevinsonDurbin(const int32_t* auto_corr,
                                 int16_t* lpc_coef,
                                 int16_t* refl_coef,
                                 size_t order);

// Converts reflection coefficients `refl_coef` to LPC coefficients `lpc_coef`.
// This version is a 16 bit operation.
//
// NOTE: The 16 bit refl_coef -> lpc_coef conversion might result in a
// "slightly unstable" filter (i.e., a pole just outside the unit circle) in
// "rare" cases even if the reflection coefficients are stable.
//
// Input:
//      - refl_coef : Reflection coefficients in Q15 that should be converted
//                    to LPC coefficients
//      - use_order : Number of coefficients in `refl_coef`
//
// Output:
//      - lpc_coef  : LPC coefficients in Q12
void WebRtcSpl_ReflCoefToLpc(const int16_t* refl_coef,
                             int use_order,
                             int16_t* lpc_coef);

// Converts LPC coefficients `lpc_coef` to reflection coefficients `refl_coef`.
// This version is a 16 bit operation.
// The conversion is implemented by the step-down algorithm.
//
// Input:
//      - lpc_coef  : LPC coefficients in Q12, that should be converted to
//                    reflection coefficients
//      - use_order : Number of coefficients in `lpc_coef`
//
// Output:
//      - refl_coef : Reflection coefficients in Q15.
void WebRtcSpl_LpcToReflCoef(int16_t* lpc_coef,
                             int use_order,
                             int16_t* refl_coef);

// Calculates reflection coefficients (16 bit) from auto-correlation values
//
// Input:
//      - auto_corr : Auto-correlation values
//      - use_order : Number of coefficients wanted be calculated
//
// Output:
//      - refl_coef : Reflection coefficients in Q15.
void WebRtcSpl_AutoCorrToReflCoef(const int32_t* auto_corr,
                                  int use_order,
                                  int16_t* refl_coef);

// The functions (with related pointer) calculate the cross-correlation between
// two sequences `seq1` and `seq2`.
// `seq1` is fixed and `seq2` slides as the pointer is increased with the
// amount `step_seq2`. Note the arguments should obey the relationship:
// `dim_seq` - 1 + `step_seq2` * (`dim_cross_correlation` - 1) <
//      buffer size of `seq2`
//
// Input:
//      - seq1           : First sequence (fixed throughout the correlation)
//      - seq2           : Second sequence (slides `step_vector2` for each
//                            new correlation)
//      - dim_seq        : Number of samples to use in the cross-correlation
//      - dim_cross_correlation : Number of cross-correlations to calculate (the
//                            start position for `vector2` is updated for each
//                            new one)
//      - right_shifts   : Number of right bit shifts to use. This will
//                            become the output Q-domain.
//      - step_seq2      : How many (positive or negative) steps the
//                            `vector2` pointer should be updated for each new
//                            cross-correlation value.
//
// Output:
//      - cross_correlation : The cross-correlation in Q(-right_shifts)
typedef void (*CrossCorrelation)(int32_t* cross_correlation,
                                 const int16_t* seq1,
                                 const int16_t* seq2,
                                 size_t dim_seq,
                                 size_t dim_cross_correlation,
                                 int right_shifts,
                                 int step_seq2);
extern const CrossCorrelation WebRtcSpl_CrossCorrelation;
void WebRtcSpl_CrossCorrelationC(int32_t* cross_correlation,
                                 const int16_t* seq1,
                                 const int16_t* seq2,
                                 size_t dim_seq,
                                 size_t dim_cross_correlation,
                                 int right_shifts,
                                 int step_seq2);
#if defined(WEBRTC_HAS_NEON)
void WebRtcSpl_CrossCorrelationNeon(int32_t* cross_correlation,
                                    const int16_t* seq1,
                                    const int16_t* seq2,
                                    size_t dim_seq,
                                    size_t dim_cross_correlation,
                                    int right_shifts,
                                    int step_seq2);
#endif
#if defined(MIPS32_LE)
void WebRtcSpl_CrossCorrelation_mips(int32_t* cross_correlation,
                                     const int16_t* seq1,
                                     const int16_t* seq2,
                                     size_t dim_seq,
                                     size_t dim_cross_correlation,
                                     int right_shifts,
                                     int step_seq2);
#endif

// Creates (the first half of) a Hanning window. Size must be at least 1 and
// at most 512.
//
// Input:
//      - size      : Length of the requested Hanning window (1 to 512)
//
// Output:
//      - window    : Hanning vector in Q14.
void WebRtcSpl_GetHanningWindow(int16_t* window, size_t size);

// Calculates y[k] = sqrt(1 - x[k]^2) for each element of the input vector
// `in_vector`. Input and output values are in Q15.
//
// Inputs:
//      - in_vector     : Values to calculate sqrt(1 - x^2) of
//      - vector_length : Length of vector `in_vector`
//
// Output:
//      - out_vector    : Output values in Q15
void WebRtcSpl_SqrtOfOneMinusXSquared(int16_t* in_vector,
                                      size_t vector_length,
                                      int16_t* out_vector);
// End: Signal processing operations.

// Randomization functions. Implementations collected in
// randomization_functions.c and descriptions at bottom of this file.
int16_t WebRtcSpl_RandU(uint32_t* seed);
int16_t WebRtcSpl_RandN(uint32_t* seed);
int16_t WebRtcSpl_RandUArray(int16_t* vector,
                             int16_t vector_length,
                             uint32_t* seed);
// End: Randomization functions.

// Math functions
int32_t WebRtcSpl_Sqrt(int32_t value);

// Divisions. Implementations collected in division_operations.c and
// descriptions at bottom of this file.
uint32_t WebRtcSpl_DivU32U16(uint32_t num, uint16_t den);
int32_t WebRtcSpl_DivW32W16(int32_t num, int16_t den);
int16_t WebRtcSpl_DivW32W16ResW16(int32_t num, int16_t den);
int32_t WebRtcSpl_DivResultInQ31(int32_t num, int32_t den);
int32_t WebRtcSpl_DivW32HiLow(int32_t num, int16_t den_hi, int16_t den_low);
// End: Divisions.

int32_t WebRtcSpl_Energy(int16_t* vector,
                         size_t vector_length,
                         int* scale_factor);

// Filter operations.
size_t WebRtcSpl_FilterAR(const int16_t* ar_coef,
                          size_t ar_coef_length,
                          const int16_t* in_vector,
                          size_t in_vector_length,
                          int16_t* filter_state,
                          size_t filter_state_length,
                          int16_t* filter_state_low,
                          int16_t* out_vector,
                          int16_t* out_vector_low);

// WebRtcSpl_FilterMAFastQ12(...)
//
// Performs a MA filtering on a vector in Q12
//
// Input:
//      - in_vector         : Input samples (state in positions
//                            in_vector[-order] .. in_vector[-1])
//      - ma_coef           : Filter coefficients (in Q12)
//      - ma_coef_length    : Number of B coefficients (order+1)
//      - vector_length     : Number of samples to be filtered
//
// Output:
//      - out_vector        : Filtered samples
//
void WebRtcSpl_FilterMAFastQ12(const int16_t* in_vector,
                               int16_t* out_vector,
                               const int16_t* ma_coef,
                               size_t ma_coef_length,
                               size_t vector_length);

// Performs a AR filtering on a vector in Q12
// Input:
//      - data_in            : Input samples
//      - data_out           : State information in positions
//                               data_out[-order] .. data_out[-1]
//      - coefficients       : Filter coefficients (in Q12)
//      - coefficients_length: Number of coefficients (order+1)
//      - data_length        : Number of samples to be filtered
// Output:
//      - data_out           : Filtered samples
void WebRtcSpl_FilterARFastQ12(const int16_t* data_in,
                               int16_t* data_out,
                               const int16_t* __restrict coefficients,
                               size_t coefficients_length,
                               size_t data_length);

// The functions (with related pointer) perform a MA down sampling filter
// on a vector.
// Input:
//      - data_in            : Input samples (state in positions
//                               data_in[-order] .. data_in[-1])
//      - data_in_length     : Number of samples in `data_in` to be filtered.
//                               This must be at least
//                               `delay` + `factor`*(`out_vector_length`-1) + 1)
//      - data_out_length    : Number of down sampled samples desired
//      - coefficients       : Filter coefficients (in Q12)
//      - coefficients_length: Number of coefficients (order+1)
//      - factor             : Decimation factor
//      - delay              : Delay of filter (compensated for in out_vector)
// Output:
//      - data_out           : Filtered samples
// Return value              : 0 if OK, -1 if `in_vector` is too short
typedef int (*DownsampleFast)(const int16_t* data_in,
                              size_t data_in_length,
                              int16_t* data_out,
                              size_t data_out_length,
                              const int16_t* __restrict coefficients,
                              size_t coefficients_length,
                              int factor,
                              size_t delay);
extern const DownsampleFast WebRtcSpl_DownsampleFast;
int WebRtcSpl_DownsampleFastC(const int16_t* data_in,
                              size_t data_in_length,
                              int16_t* data_out,
                              size_t data_out_length,
                              const int16_t* __restrict coefficients,
                              size_t coefficients_length,
                              int factor,
                              size_t delay);
#if defined(WEBRTC_HAS_NEON)
int WebRtcSpl_DownsampleFastNeon(const int16_t* data_in,
                                 size_t data_in_length,
                                 int16_t* data_out,
                                 size_t data_out_length,
                                 const int16_t* __restrict coefficients,
                                 size_t coefficients_length,
                                 int factor,
                                 size_t delay);
#endif
#if defined(MIPS32_LE)
int WebRtcSpl_DownsampleFast_mips(const int16_t* data_in,
                                  size_t data_in_length,
                                  int16_t* data_out,
                                  size_t data_out_length,
                                  const int16_t* __restrict coefficients,
                                  size_t coefficients_length,
                                  int factor,
                                  size_t delay);
#endif

// End: Filter operations.

// FFT operations

int WebRtcSpl_ComplexFFT(int16_t vector[], int stages, int mode);
int WebRtcSpl_ComplexIFFT(int16_t vector[], int stages, int mode);

// Treat a 16-bit complex data buffer `complex_data` as an array of 32-bit
// values, and swap elements whose indexes are bit-reverses of each other.
//
// Input:
//      - complex_data  : Complex data buffer containing 2^`stages` real
//                        elements interleaved with 2^`stages` imaginary
//                        elements: [Re Im Re Im Re Im....]
//      - stages        : Number of FFT stages. Must be at least 3 and at most
//                        10, since the table WebRtcSpl_kSinTable1024[] is 1024
//                        elements long.
//
// Output:
//      - complex_data  : The complex data buffer.

void WebRtcSpl_ComplexBitReverse(int16_t* __restrict complex_data, int stages);

// End: FFT operations

/************************************************************
 *
 * RESAMPLING FUNCTIONS AND THEIR STRUCTS ARE DEFINED BELOW
 *
 ************************************************************/

/*******************************************************************
 * resample.c
 *
 * Includes the following resampling combinations
 * 22 kHz -> 16 kHz
 * 16 kHz -> 22 kHz
 * 22 kHz ->  8 kHz
 *  8 kHz -> 22 kHz
 *
 ******************************************************************/

// state structure for 22 -> 16 resampler
typedef struct {
  int32_t S_22_44[8];
  int32_t S_44_32[8];
  int32_t S_32_16[8];
} WebRtcSpl_State22khzTo16khz;

void WebRtcSpl_Resample22khzTo16khz(const int16_t* in,
                                    int16_t* out,
                                    WebRtcSpl_State22khzTo16khz* state,
                                    int32_t* tmpmem);

void WebRtcSpl_ResetResample22khzTo16khz(WebRtcSpl_State22khzTo16khz* state);

// state structure for 16 -> 22 resampler
typedef struct {
  int32_t S_16_32[8];
  int32_t S_32_22[8];
} WebRtcSpl_State16khzTo22khz;

void WebRtcSpl_Resample16khzTo22khz(const int16_t* in,
                                    int16_t* out,
                                    WebRtcSpl_State16khzTo22khz* state,
                                    int32_t* tmpmem);

void WebRtcSpl_ResetResample16khzTo22khz(WebRtcSpl_State16khzTo22khz* state);

// state structure for 22 -> 8 resampler
typedef struct {
  int32_t S_22_22[16];
  int32_t S_22_16[8];
  int32_t S_16_8[8];
} WebRtcSpl_State22khzTo8khz;

void WebRtcSpl_Resample22khzTo8khz(const int16_t* in,
                                   int16_t* out,
                                   WebRtcSpl_State22khzTo8khz* state,
                                   int32_t* tmpmem);

void WebRtcSpl_ResetResample22khzTo8khz(WebRtcSpl_State22khzTo8khz* state);

// state structure for 8 -> 22 resampler
typedef struct {
  int32_t S_8_16[8];
  int32_t S_16_11[8];
  int32_t S_11_22[8];
} WebRtcSpl_State8khzTo22khz;

void WebRtcSpl_Resample8khzTo22khz(const int16_t* in,
                                   int16_t* out,
                                   WebRtcSpl_State8khzTo22khz* state,
                                   int32_t* tmpmem);

void WebRtcSpl_ResetResample8khzTo22khz(WebRtcSpl_State8khzTo22khz* state);

/*******************************************************************
 * resample_fractional.c
 * Functions for internal use in the other resample functions
 *
 * Includes the following resampling combinations
 * 48 kHz -> 32 kHz
 * 32 kHz -> 24 kHz
 * 44 kHz -> 32 kHz
 *
 ******************************************************************/

void WebRtcSpl_Resample48khzTo32khz(const int32_t* In, int32_t* Out, size_t K);

void WebRtcSpl_Resample32khzTo24khz(const int32_t* In, int32_t* Out, size_t K);

void WebRtcSpl_Resample44khzTo32khz(const int32_t* In, int32_t* Out, size_t K);

/*******************************************************************
 * resample_48khz.c
 *
 * Includes the following resampling combinations
 * 48 kHz -> 16 kHz
 * 16 kHz -> 48 kHz
 * 48 kHz ->  8 kHz
 *  8 kHz -> 48 kHz
 *
 ******************************************************************/

typedef struct {
  int32_t S_48_48[16];
  int32_t S_48_32[8];
  int32_t S_32_16[8];
} WebRtcSpl_State48khzTo16khz;

void WebRtcSpl_Resample48khzTo16khz(const int16_t* in,
                                    int16_t* out,
                                    WebRtcSpl_State48khzTo16khz* state,
                                    int32_t* tmpmem);

void WebRtcSpl_ResetResample48khzTo16khz(WebRtcSpl_State48khzTo16khz* state);

typedef struct {
  int32_t S_16_32[8];
  int32_t S_32_24[8];
  int32_t S_24_48[8];
} WebRtcSpl_State16khzTo48khz;

void WebRtcSpl_Resample16khzTo48khz(const int16_t* in,
                                    int16_t* out,
                                    WebRtcSpl_State16khzTo48khz* state,
                                    int32_t* tmpmem);

void WebRtcSpl_ResetResample16khzTo48khz(WebRtcSpl_State16khzTo48khz* state);

typedef struct {
  int32_t S_48_24[8];
  int32_t S_24_24[16];
  int32_t S_24_16[8];
  int32_t S_16_8[8];
} WebRtcSpl_State48khzTo8khz;

void WebRtcSpl_Resample48khzTo8khz(const int16_t* in,
                                   int16_t* out,
                                   WebRtcSpl_State48khzTo8khz* state,
                                   int32_t* tmpmem);

void WebRtcSpl_ResetResample48khzTo8khz(WebRtcSpl_State48khzTo8khz* state);

typedef struct {
  int32_t S_8_16[8];
  int32_t S_16_12[8];
  int32_t S_12_24[8];
  int32_t S_24_48[8];
} WebRtcSpl_State8khzTo48khz;

void WebRtcSpl_Resample8khzTo48khz(const int16_t* in,
                                   int16_t* out,
                                   WebRtcSpl_State8khzTo48khz* state,
                                   int32_t* tmpmem);

void WebRtcSpl_ResetResample8khzTo48khz(WebRtcSpl_State8khzTo48khz* state);

/*******************************************************************
 * resample_by_2.c
 *
 * Includes down and up sampling by a factor of two.
 *
 ******************************************************************/

void WebRtcSpl_DownsampleBy2(const int16_t* in,
                             size_t len,
                             int16_t* out,
                             int32_t* filtState);

void WebRtcSpl_UpsampleBy2(const int16_t* in,
                           size_t len,
                           int16_t* out,
                           int32_t* filtState);

/************************************************************
 * END OF RESAMPLING FUNCTIONS
 ************************************************************/
void WebRtcSpl_AnalysisQMF(const int16_t* in_data,
                           size_t in_data_length,
                           int16_t* low_band,
                           int16_t* high_band,
                           int32_t* filter_state1,
                           int32_t* filter_state2);
void WebRtcSpl_SynthesisQMF(const int16_t* low_band,
                            const int16_t* high_band,
                            size_t band_length,
                            int16_t* out_data,
                            int32_t* filter_state1,
                            int32_t* filter_state2);

#ifdef __cplusplus
}
#endif  // __cplusplus
#endif  // COMMON_AUDIO_SIGNAL_PROCESSING_INCLUDE_SIGNAL_PROCESSING_LIBRARY_H_

//
// WebRtcSpl_AddSatW16(...)
// WebRtcSpl_AddSatW32(...)
//
// Returns the result of a saturated 16-bit, respectively 32-bit, addition of
// the numbers specified by the `var1` and `var2` parameters.
//
// Input:
//      - var1      : Input variable 1
//      - var2      : Input variable 2
//
// Return value     : Added and saturated value
//

//
// WebRtcSpl_SubSatW16(...)
// WebRtcSpl_SubSatW32(...)
//
// Returns the result of a saturated 16-bit, respectively 32-bit, subtraction
// of the numbers specified by the `var1` and `var2` parameters.
//
// Input:
//      - var1      : Input variable 1
//      - var2      : Input variable 2
//
// Returned value   : Subtracted and saturated value
//

//
// WebRtcSpl_GetSizeInBits(...)
//
// Returns the # of bits that are needed at the most to represent the number
// specified by the `value` parameter.
//
// Input:
//      - value     : Input value
//
// Return value     : Number of bits needed to represent `value`
//

//
// WebRtcSpl_NormW32(...)
//
// Norm returns the # of left shifts required to 32-bit normalize the 32-bit
// signed number specified by the `value` parameter.
//
// Input:
//      - value     : Input value
//
// Return value     : Number of bit shifts needed to 32-bit normalize `value`
//

//
// WebRtcSpl_NormW16(...)
//
// Norm returns the # of left shifts required to 16-bit normalize the 16-bit
// signed number specified by the `value` parameter.
//
// Input:
//      - value     : Input value
//
// Return value     : Number of bit shifts needed to 32-bit normalize `value`
//

//
// WebRtcSpl_NormU32(...)
//
// Norm returns the # of left shifts required to 32-bit normalize the unsigned
// 32-bit number specified by the `value` parameter.
//
// Input:
//      - value     : Input value
//
// Return value     : Number of bit shifts needed to 32-bit normalize `value`
//

//
// WebRtcSpl_GetScalingSquare(...)
//
// Returns the # of bits required to scale the samples specified in the
// `in_vector` parameter so that, if the squares of the samples are added the
// # of times specified by the `times` parameter, the 32-bit addition will not
// overflow (result in int32_t).
//
// Input:
//      - in_vector         : Input vector to check scaling on
//      - in_vector_length  : Samples in `in_vector`
//      - times             : Number of additions to be performed
//
// Return value             : Number of right bit shifts needed to avoid
//                            overflow in the addition calculation
//

//
// WebRtcSpl_MemSetW16(...)
//
// Sets all the values in the int16_t vector `vector` of length
// `vector_length` to the specified value `set_value`
//
// Input:
//      - vector        : Pointer to the int16_t vector
//      - set_value     : Value specified
//      - vector_length : Length of vector
//

//
// WebRtcSpl_MemSetW32(...)
//
// Sets all the values in the int32_t vector `vector` of length
// `vector_length` to the specified value `set_value`
//
// Input:
//      - vector        : Pointer to the int16_t vector
//      - set_value     : Value specified
//      - vector_length : Length of vector
//

//
// WebRtcSpl_MemCpyReversedOrder(...)
//
// Copies all the values from the source int16_t vector `in_vector` to a
// destination int16_t vector `out_vector`. It is done in reversed order,
// meaning that the first sample of `in_vector` is copied to the last sample of
// the `out_vector`. The procedure continues until the last sample of
// `in_vector` has been copied to the first sample of `out_vector`. This
// creates a reversed vector.
//
// Input:
//      - in_vector     : Pointer to the first sample in a int16_t vector
//                        of length `length`
//      - vector_length : Number of elements to copy
//
// Output:
//      - out_vector    : Pointer to the last sample in a int16_t vector
//                        of length `length`
//

//
// WebRtcSpl_CopyFromEndW16(...)
//
// Copies the rightmost `samples` of `in_vector` (of length `in_vector_length`)
// to the vector `out_vector`.
//
// Input:
//      - in_vector         : Input vector
//      - in_vector_length  : Number of samples in `in_vector`
//      - samples           : Number of samples to extract (from right side)
//                            from `in_vector`
//
// Output:
//      - out_vector        : Vector with the requested samples
//

//
// WebRtcSpl_ZerosArrayW16(...)
// WebRtcSpl_ZerosArrayW32(...)
//
// Inserts the value "zero" in all positions of a w16 and a w32 vector
// respectively.
//
// Input:
//      - vector_length : Number of samples in vector
//
// Output:
//      - vector        : Vector containing all zeros
//

//
// WebRtcSpl_VectorBitShiftW16(...)
// WebRtcSpl_VectorBitShiftW32(...)
//
// Bit shifts all the values in a vector up or downwards. Different calls for
// int16_t and int32_t vectors respectively.
//
// Input:
//      - vector_length : Length of vector
//      - in_vector     : Pointer to the vector that should be bit shifted
//      - right_shifts  : Number of right bit shifts (negative value gives left
//                        shifts)
//
// Output:
//      - out_vector    : Pointer to the result vector (can be the same as
//                        `in_vector`)
//

//
// WebRtcSpl_VectorBitShiftW32ToW16(...)
//
// Bit shifts all the values in a int32_t vector up or downwards and
// stores the result as an int16_t vector. The function will saturate the
// signal if needed, before storing in the output vector.
//
// Input:
//      - vector_length : Length of vector
//      - in_vector     : Pointer to the vector that should be bit shifted
//      - right_shifts  : Number of right bit shifts (negative value gives left
//                        shifts)
//
// Output:
//      - out_vector    : Pointer to the result vector (can be the same as
//                        `in_vector`)
//

//
// WebRtcSpl_ScaleVector(...)
//
// Performs the vector operation:
//  out_vector[k] = (gain*in_vector[k])>>right_shifts
//
// Input:
//      - in_vector     : Input vector
//      - gain          : Scaling gain
//      - vector_length : Elements in the `in_vector`
//      - right_shifts  : Number of right bit shifts applied
//
// Output:
//      - out_vector    : Output vector (can be the same as `in_vector`)
//

//
// WebRtcSpl_ScaleVectorWithSat(...)
//
// Performs the vector operation:
//  out_vector[k] = SATURATE( (gain*in_vector[k])>>right_shifts )
//
// Input:
//      - in_vector     : Input vector
//      - gain          : Scaling gain
//      - vector_length : Elements in the `in_vector`
//      - right_shifts  : Number of right bit shifts applied
//
// Output:
//      - out_vector    : Output vector (can be the same as `in_vector`)
//

//
// WebRtcSpl_ScaleAndAddVectors(...)
//
// Performs the vector operation:
//  out_vector[k] = (gain1*in_vector1[k])>>right_shifts1
//                  + (gain2*in_vector2[k])>>right_shifts2
//
// Input:
//      - in_vector1    : Input vector 1
//      - gain1         : Gain to be used for vector 1
//      - right_shifts1 : Right bit shift to be used for vector 1
//      - in_vector2    : Input vector 2
//      - gain2         : Gain to be used for vector 2
//      - right_shifts2 : Right bit shift to be used for vector 2
//      - vector_length : Elements in the input vectors
//
// Output:
//      - out_vector    : Output vector
//

//
// WebRtcSpl_IncreaseSeed(...)
//
// Increases the seed (and returns the new value)
//
// Input:
//      - seed      : Seed for random calculation
//
// Output:
//      - seed      : Updated seed value
//
// Return value     : The new seed value
//

//
// WebRtcSpl_RandU(...)
//
// Produces a uniformly distributed value in the int16_t range
//
// Input:
//      - seed      : Seed for random calculation
//
// Output:
//      - seed      : Updated seed value
//
// Return value     : Uniformly distributed value in the range
//                    [Word16_MIN...Word16_MAX]
//

//
// WebRtcSpl_RandN(...)
//
// Produces a normal distributed value in the int16_t range
//
// Input:
//      - seed      : Seed for random calculation
//
// Output:
//      - seed      : Updated seed value
//
// Return value     : N(0,1) value in the Q13 domain
//

//
// WebRtcSpl_RandUArray(...)
//
// Produces a uniformly distributed vector with elements in the int16_t
// range
//
// Input:
//      - vector_length : Samples wanted in the vector
//      - seed          : Seed for random calculation
//
// Output:
//      - vector        : Vector with the uniform values
//      - seed          : Updated seed value
//
// Return value         : Number of samples in vector, i.e., `vector_length`
//

//
// WebRtcSpl_Sqrt(...)
//
// Returns the square root of the input value `value`. The precision of this
// function is integer precision, i.e., sqrt(8) gives 2 as answer.
// If `value` is a negative number then 0 is returned.
//
// Algorithm:
//
// A sixth order Taylor Series expansion is used here to compute the square
// root of a number y^0.5 = (1+x)^0.5
// where
// x = y-1
//   = 1+(x/2)-0.5*((x/2)^2+0.5*((x/2)^3-0.625*((x/2)^4+0.875*((x/2)^5)
// 0.5 <= x < 1
//
// Input:
//      - value     : Value to calculate sqrt of
//
// Return value     : Result of the sqrt calculation
//

//
// WebRtcSpl_DivU32U16(...)
//
// Divides a uint32_t `num` by a uint16_t `den`.
//
// If `den`==0, (uint32_t)0xFFFFFFFF is returned.
//
// Input:
//      - num       : Numerator
//      - den       : Denominator
//
// Return value     : Result of the division (as a uint32_t), i.e., the
//                    integer part of num/den.
//

//
// WebRtcSpl_DivW32W16(...)
//
// Divides a int32_t `num` by a int16_t `den`.
//
// If `den`==0, (int32_t)0x7FFFFFFF is returned.
//
// Input:
//      - num       : Numerator
//      - den       : Denominator
//
// Return value     : Result of the division (as a int32_t), i.e., the
//                    integer part of num/den.
//

//
// WebRtcSpl_DivW32W16ResW16(...)
//
// Divides a int32_t `num` by a int16_t `den`, assuming that the
// result is less than 32768, otherwise an unpredictable result will occur.
//
// If `den`==0, (int16_t)0x7FFF is returned.
//
// Input:
//      - num       : Numerator
//      - den       : Denominator
//
// Return value     : Result of the division (as a int16_t), i.e., the
//                    integer part of num/den.
//

//
// WebRtcSpl_DivResultInQ31(...)
//
// Divides a int32_t `num` by a int16_t `den`, assuming that the
// absolute value of the denominator is larger than the numerator, otherwise
// an unpredictable result will occur.
//
// Input:
//      - num       : Numerator
//      - den       : Denominator
//
// Return value     : Result of the division in Q31.
//

//
// WebRtcSpl_DivW32HiLow(...)
//
// Divides a int32_t `num` by a denominator in hi, low format. The
// absolute value of the denominator has to be larger (or equal to) the
// numerator.
//
// Input:
//      - num       : Numerator
//      - den_hi    : High part of denominator
//      - den_low   : Low part of denominator
//
// Return value     : Divided value in Q31
//

//
// WebRtcSpl_Energy(...)
//
// Calculates the energy of a vector
//
// Input:
//      - vector        : Vector which the energy should be calculated on
//      - vector_length : Number of samples in vector
//
// Output:
//      - scale_factor  : Number of left bit shifts needed to get the physical
//                        energy value, i.e, to get the Q0 value
//
// Return value         : Energy value in Q(-`scale_factor`)
//

//
// WebRtcSpl_FilterAR(...)
//
// Performs a 32-bit AR filtering on a vector in Q12
//
// Input:
//  - ar_coef                   : AR-coefficient vector (values in Q12),
//                                ar_coef[0] must be 4096.
//  - ar_coef_length            : Number of coefficients in `ar_coef`.
//  - in_vector                 : Vector to be filtered.
//  - in_vector_length          : Number of samples in `in_vector`.
//  - filter_state              : Current state (higher part) of the filter.
//  - filter_state_length       : Length (in samples) of `filter_state`.
//  - filter_state_low          : Current state (lower part) of the filter.
//
// Output:
//  - filter_state              : Updated state (upper part) vector.
//  - filter_state_low          : Updated state (lower part) vector.
//  - out_vector                : Vector containing the upper part of the
//                                filtered values.
//  - out_vector_low            : Vector containing the lower part of the
//                                filtered values.
//
// Return value                 : Number of samples in the `out_vector`.
//

//
// WebRtcSpl_ComplexIFFT(...)
//
// Complex Inverse FFT
//
// Computes an inverse complex 2^`stages`-point FFT on the input vector, which
// is in bit-reversed order. The original content of the vector is destroyed in
// the process, since the input is overwritten by the output, normal-ordered,
// FFT vector. With X as the input complex vector, y as the output complex
// vector and with M = 2^`stages`, the following is computed:
//
//        M-1
// y(k) = sum[X(i)*[cos(2*pi*i*k/M) + j*sin(2*pi*i*k/M)]]
//        i=0
//
// The implementations are optimized for speed, not for code size. It uses the
// decimation-in-time algorithm with radix-2 butterfly technique.
//
// Input:
//      - vector    : In pointer to complex vector containing 2^`stages`
//                    real elements interleaved with 2^`stages` imaginary
//                    elements.
//                    [ReImReImReIm....]
//                    The elements are in Q(-scale) domain, see more on Return
//                    Value below.
//
//      - stages    : Number of FFT stages. Must be at least 3 and at most 10,
//                    since the table WebRtcSpl_kSinTable1024[] is 1024
//                    elements long.
//
//      - mode      : This parameter gives the user to choose how the FFT
//                    should work.
//                    mode==0: Low-complexity and Low-accuracy mode
//                    mode==1: High-complexity and High-accuracy mode
//
// Output:
//      - vector    : Out pointer to the FFT vector (the same as input).
//
// Return Value     : The scale value that tells the number of left bit shifts
//                    that the elements in the `vector` should be shifted with
//                    in order to get Q0 values, i.e. the physically correct
//                    values. The scale parameter is always 0 or positive,
//                    except if N>1024 (`stages`>10), which returns a scale
//                    value of -1, indicating error.
//

//
// WebRtcSpl_ComplexFFT(...)
//
// Complex FFT
//
// Computes a complex 2^`stages`-point FFT on the input vector, which is in
// bit-reversed order. The original content of the vector is destroyed in
// the process, since the input is overwritten by the output, normal-ordered,
// FFT vector. With x as the input complex vector, Y as the output complex
// vector and with M = 2^`stages`, the following is computed:
//
//              M-1
// Y(k) = 1/M * sum[x(i)*[cos(2*pi*i*k/M) + j*sin(2*pi*i*k/M)]]
//              i=0
//
// The implementations are optimized for speed, not for code size. It uses the
// decimation-in-time algorithm with radix-2 butterfly technique.
//
// This routine prevents overflow by scaling by 2 before each FFT stage. This is
// a fixed scaling, for proper normalization - there will be log2(n) passes, so
// this results in an overall factor of 1/n, distributed to maximize arithmetic
// accuracy.
//
// Input:
//      - vector    : In pointer to complex vector containing 2^`stages` real
//                    elements interleaved with 2^`stages` imaginary elements.
//                    [ReImReImReIm....]
//                    The output is in the Q0 domain.
//
//      - stages    : Number of FFT stages. Must be at least 3 and at most 10,
//                    since the table WebRtcSpl_kSinTable1024[] is 1024
//                    elements long.
//
//      - mode      : This parameter gives the user to choose how the FFT
//                    should work.
//                    mode==0: Low-complexity and Low-accuracy mode
//                    mode==1: High-complexity and High-accuracy mode
//
// Output:
//      - vector    : The output FFT vector is in the Q0 domain.
//
// Return value     : The scale parameter is always 0, except if N>1024,
//                    which returns a scale value of -1, indicating error.
//

//
// WebRtcSpl_AnalysisQMF(...)
//
// Splits a 0-2*F Hz signal into two sub bands: 0-F Hz and F-2*F Hz. The
// current version has F = 8000, therefore, a super-wideband audio signal is
// split to lower-band 0-8 kHz and upper-band 8-16 kHz.
//
// Input:
//      - in_data       : Wide band speech signal, 320 samples (10 ms)
//
// Input & Output:
//      - filter_state1 : Filter state for first All-pass filter
//      - filter_state2 : Filter state for second All-pass filter
//
// Output:
//      - low_band      : Lower-band signal 0-8 kHz band, 160 samples (10 ms)
//      - high_band     : Upper-band signal 8-16 kHz band (flipped in frequency
//                        domain), 160 samples (10 ms)
//

//
// WebRtcSpl_SynthesisQMF(...)
//
// Combines the two sub bands (0-F and F-2*F Hz) into a signal of 0-2*F
// Hz, (current version has F = 8000 Hz). So the filter combines lower-band
// (0-8 kHz) and upper-band (8-16 kHz) channels to obtain super-wideband 0-16
// kHz audio.
//
// Input:
//      - low_band      : The signal with the 0-8 kHz band, 160 samples (10 ms)
//      - high_band     : The signal with the 8-16 kHz band, 160 samples (10 ms)
//
// Input & Output:
//      - filter_state1 : Filter state for first All-pass filter
//      - filter_state2 : Filter state for second All-pass filter
//
// Output:
//      - out_data      : Super-wideband speech signal, 0-16 kHz
//

// int16_t WebRtcSpl_SatW32ToW16(...)
//
// This function saturates a 32-bit word into a 16-bit word.
//
// Input:
//      - value32   : The value of a 32-bit word.
//
// Output:
//      - out16     : the saturated 16-bit word.
//

// int32_t WebRtc_MulAccumW16(...)
//
// This function multiply a 16-bit word by a 16-bit word, and accumulate this
// value to a 32-bit integer.
//
// Input:
//      - a    : The value of the first 16-bit word.
//      - b    : The value of the second 16-bit word.
//      - c    : The value of an 32-bit integer.
//
// Return Value: The value of a * b + c.
//
