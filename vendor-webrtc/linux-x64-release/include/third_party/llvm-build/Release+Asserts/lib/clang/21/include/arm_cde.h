/*===---- arm_cde.h - ARM CDE intrinsics -----------------------------------===
 *
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */

#ifndef __ARM_CDE_H
#define __ARM_CDE_H

#if !__ARM_FEATURE_CDE
#error "CDE support not enabled"
#endif

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx1)))
uint32_t __arm_cx1(int, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx1a)))
uint32_t __arm_cx1a(int, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx1d)))
uint64_t __arm_cx1d(int, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx1da)))
uint64_t __arm_cx1da(int, uint64_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx2)))
uint32_t __arm_cx2(int, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx2a)))
uint32_t __arm_cx2a(int, uint32_t, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx2d)))
uint64_t __arm_cx2d(int, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx2da)))
uint64_t __arm_cx2da(int, uint64_t, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx3)))
uint32_t __arm_cx3(int, uint32_t, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx3a)))
uint32_t __arm_cx3a(int, uint32_t, uint32_t, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx3d)))
uint64_t __arm_cx3d(int, uint32_t, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_cx3da)))
uint64_t __arm_cx3da(int, uint64_t, uint32_t, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx1_u32)))
uint32_t __arm_vcx1_u32(int, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx1a_u32)))
uint32_t __arm_vcx1a_u32(int, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx1d_u64)))
uint64_t __arm_vcx1d_u64(int, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx1da_u64)))
uint64_t __arm_vcx1da_u64(int, uint64_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx2_u32)))
uint32_t __arm_vcx2_u32(int, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx2a_u32)))
uint32_t __arm_vcx2a_u32(int, uint32_t, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx2d_u64)))
uint64_t __arm_vcx2d_u64(int, uint64_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx2da_u64)))
uint64_t __arm_vcx2da_u64(int, uint64_t, uint64_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx3_u32)))
uint32_t __arm_vcx3_u32(int, uint32_t, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx3a_u32)))
uint32_t __arm_vcx3a_u32(int, uint32_t, uint32_t, uint32_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx3d_u64)))
uint64_t __arm_vcx3d_u64(int, uint64_t, uint64_t, uint32_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx3da_u64)))
uint64_t __arm_vcx3da_u64(int, uint64_t, uint64_t, uint64_t, uint32_t);

#if __ARM_FEATURE_MVE

typedef uint16_t mve_pred16_t;
typedef __attribute__((__neon_vector_type__(8), __clang_arm_mve_strict_polymorphism)) int16_t int16x8_t;
typedef __attribute__((__neon_vector_type__(4), __clang_arm_mve_strict_polymorphism)) int32_t int32x4_t;
typedef __attribute__((__neon_vector_type__(2), __clang_arm_mve_strict_polymorphism)) int64_t int64x2_t;
typedef __attribute__((__neon_vector_type__(16), __clang_arm_mve_strict_polymorphism)) int8_t int8x16_t;
typedef __attribute__((__neon_vector_type__(8), __clang_arm_mve_strict_polymorphism)) uint16_t uint16x8_t;
typedef __attribute__((__neon_vector_type__(4), __clang_arm_mve_strict_polymorphism)) uint32_t uint32x4_t;
typedef __attribute__((__neon_vector_type__(2), __clang_arm_mve_strict_polymorphism)) uint64_t uint64x2_t;
typedef __attribute__((__neon_vector_type__(16), __clang_arm_mve_strict_polymorphism)) uint8_t uint8x16_t;

static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_m_s16)))
int16x8_t __arm_vcx1q_m(int, int16x8_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_m_s32)))
int32x4_t __arm_vcx1q_m(int, int32x4_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_m_s64)))
int64x2_t __arm_vcx1q_m(int, int64x2_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_m_s8)))
int8x16_t __arm_vcx1q_m(int, int8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_m_u16)))
uint16x8_t __arm_vcx1q_m(int, uint16x8_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_m_u32)))
uint32x4_t __arm_vcx1q_m(int, uint32x4_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_m_u64)))
uint64x2_t __arm_vcx1q_m(int, uint64x2_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_m_u8)))
uint8x16_t __arm_vcx1q_m(int, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_u8)))
uint8x16_t __arm_vcx1q_u8(int, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_m_s16)))
int16x8_t __arm_vcx1qa_m(int, int16x8_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_m_s32)))
int32x4_t __arm_vcx1qa_m(int, int32x4_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_m_s64)))
int64x2_t __arm_vcx1qa_m(int, int64x2_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_m_s8)))
int8x16_t __arm_vcx1qa_m(int, int8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_m_u16)))
uint16x8_t __arm_vcx1qa_m(int, uint16x8_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_m_u32)))
uint32x4_t __arm_vcx1qa_m(int, uint32x4_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_m_u64)))
uint64x2_t __arm_vcx1qa_m(int, uint64x2_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_m_u8)))
uint8x16_t __arm_vcx1qa_m(int, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_s16)))
int16x8_t __arm_vcx1qa(int, int16x8_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_s32)))
int32x4_t __arm_vcx1qa(int, int32x4_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_s64)))
int64x2_t __arm_vcx1qa(int, int64x2_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_s8)))
int8x16_t __arm_vcx1qa(int, int8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_u16)))
uint16x8_t __arm_vcx1qa(int, uint16x8_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_u32)))
uint32x4_t __arm_vcx1qa(int, uint32x4_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_u64)))
uint64x2_t __arm_vcx1qa(int, uint64x2_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_u8)))
uint8x16_t __arm_vcx1qa(int, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_m_impl_s16)))
int16x8_t __arm_vcx2q_m_impl(int, int16x8_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_m_impl_s32)))
int32x4_t __arm_vcx2q_m_impl(int, int32x4_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_m_impl_s64)))
int64x2_t __arm_vcx2q_m_impl(int, int64x2_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_m_impl_s8)))
int8x16_t __arm_vcx2q_m_impl(int, int8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_m_impl_u16)))
uint16x8_t __arm_vcx2q_m_impl(int, uint16x8_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_m_impl_u32)))
uint32x4_t __arm_vcx2q_m_impl(int, uint32x4_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_m_impl_u64)))
uint64x2_t __arm_vcx2q_m_impl(int, uint64x2_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_m_impl_u8)))
uint8x16_t __arm_vcx2q_m_impl(int, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_s16)))
int16x8_t __arm_vcx2q(int, int16x8_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_s32)))
int32x4_t __arm_vcx2q(int, int32x4_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_s64)))
int64x2_t __arm_vcx2q(int, int64x2_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_s8)))
int8x16_t __arm_vcx2q(int, int8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u16)))
uint16x8_t __arm_vcx2q(int, uint16x8_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u32)))
uint32x4_t __arm_vcx2q(int, uint32x4_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u64)))
uint64x2_t __arm_vcx2q(int, uint64x2_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8)))
uint8x16_t __arm_vcx2q(int, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8_s16)))
uint8x16_t __arm_vcx2q_u8(int, int16x8_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8_s32)))
uint8x16_t __arm_vcx2q_u8(int, int32x4_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8_s64)))
uint8x16_t __arm_vcx2q_u8(int, int64x2_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8_s8)))
uint8x16_t __arm_vcx2q_u8(int, int8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8_u16)))
uint8x16_t __arm_vcx2q_u8(int, uint16x8_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8_u32)))
uint8x16_t __arm_vcx2q_u8(int, uint32x4_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8_u64)))
uint8x16_t __arm_vcx2q_u8(int, uint64x2_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8_u8)))
uint8x16_t __arm_vcx2q_u8(int, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_impl_s16)))
int16x8_t __arm_vcx2qa_impl(int, int16x8_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_impl_s32)))
int32x4_t __arm_vcx2qa_impl(int, int32x4_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_impl_s64)))
int64x2_t __arm_vcx2qa_impl(int, int64x2_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_impl_s8)))
int8x16_t __arm_vcx2qa_impl(int, int8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_impl_u16)))
uint16x8_t __arm_vcx2qa_impl(int, uint16x8_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_impl_u32)))
uint32x4_t __arm_vcx2qa_impl(int, uint32x4_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_impl_u64)))
uint64x2_t __arm_vcx2qa_impl(int, uint64x2_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_impl_u8)))
uint8x16_t __arm_vcx2qa_impl(int, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_m_impl_s16)))
int16x8_t __arm_vcx2qa_m_impl(int, int16x8_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_m_impl_s32)))
int32x4_t __arm_vcx2qa_m_impl(int, int32x4_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_m_impl_s64)))
int64x2_t __arm_vcx2qa_m_impl(int, int64x2_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_m_impl_s8)))
int8x16_t __arm_vcx2qa_m_impl(int, int8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_m_impl_u16)))
uint16x8_t __arm_vcx2qa_m_impl(int, uint16x8_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_m_impl_u32)))
uint32x4_t __arm_vcx2qa_m_impl(int, uint32x4_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_m_impl_u64)))
uint64x2_t __arm_vcx2qa_m_impl(int, uint64x2_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_m_impl_u8)))
uint8x16_t __arm_vcx2qa_m_impl(int, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_impl_s16)))
int16x8_t __arm_vcx3q_impl(int, int16x8_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_impl_s32)))
int32x4_t __arm_vcx3q_impl(int, int32x4_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_impl_s64)))
int64x2_t __arm_vcx3q_impl(int, int64x2_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_impl_s8)))
int8x16_t __arm_vcx3q_impl(int, int8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_impl_u16)))
uint16x8_t __arm_vcx3q_impl(int, uint16x8_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_impl_u32)))
uint32x4_t __arm_vcx3q_impl(int, uint32x4_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_impl_u64)))
uint64x2_t __arm_vcx3q_impl(int, uint64x2_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_impl_u8)))
uint8x16_t __arm_vcx3q_impl(int, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_m_impl_s16)))
int16x8_t __arm_vcx3q_m_impl(int, int16x8_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_m_impl_s32)))
int32x4_t __arm_vcx3q_m_impl(int, int32x4_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_m_impl_s64)))
int64x2_t __arm_vcx3q_m_impl(int, int64x2_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_m_impl_s8)))
int8x16_t __arm_vcx3q_m_impl(int, int8x16_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_m_impl_u16)))
uint16x8_t __arm_vcx3q_m_impl(int, uint16x8_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_m_impl_u32)))
uint32x4_t __arm_vcx3q_m_impl(int, uint32x4_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_m_impl_u64)))
uint64x2_t __arm_vcx3q_m_impl(int, uint64x2_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_m_impl_u8)))
uint8x16_t __arm_vcx3q_m_impl(int, uint8x16_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_u8_impl_s16)))
uint8x16_t __arm_vcx3q_u8_impl(int, int16x8_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_u8_impl_s32)))
uint8x16_t __arm_vcx3q_u8_impl(int, int32x4_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_u8_impl_s64)))
uint8x16_t __arm_vcx3q_u8_impl(int, int64x2_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_u8_impl_s8)))
uint8x16_t __arm_vcx3q_u8_impl(int, int8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_u8_impl_u16)))
uint8x16_t __arm_vcx3q_u8_impl(int, uint16x8_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_u8_impl_u32)))
uint8x16_t __arm_vcx3q_u8_impl(int, uint32x4_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_u8_impl_u64)))
uint8x16_t __arm_vcx3q_u8_impl(int, uint64x2_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_u8_impl_u8)))
uint8x16_t __arm_vcx3q_u8_impl(int, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_impl_s16)))
int16x8_t __arm_vcx3qa_impl(int, int16x8_t, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_impl_s32)))
int32x4_t __arm_vcx3qa_impl(int, int32x4_t, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_impl_s64)))
int64x2_t __arm_vcx3qa_impl(int, int64x2_t, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_impl_s8)))
int8x16_t __arm_vcx3qa_impl(int, int8x16_t, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_impl_u16)))
uint16x8_t __arm_vcx3qa_impl(int, uint16x8_t, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_impl_u32)))
uint32x4_t __arm_vcx3qa_impl(int, uint32x4_t, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_impl_u64)))
uint64x2_t __arm_vcx3qa_impl(int, uint64x2_t, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_impl_u8)))
uint8x16_t __arm_vcx3qa_impl(int, uint8x16_t, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_m_impl_s16)))
int16x8_t __arm_vcx3qa_m_impl(int, int16x8_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_m_impl_s32)))
int32x4_t __arm_vcx3qa_m_impl(int, int32x4_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_m_impl_s64)))
int64x2_t __arm_vcx3qa_m_impl(int, int64x2_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_m_impl_s8)))
int8x16_t __arm_vcx3qa_m_impl(int, int8x16_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_m_impl_u16)))
uint16x8_t __arm_vcx3qa_m_impl(int, uint16x8_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_m_impl_u32)))
uint32x4_t __arm_vcx3qa_m_impl(int, uint32x4_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_m_impl_u64)))
uint64x2_t __arm_vcx3qa_m_impl(int, uint64x2_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_m_impl_u8)))
uint8x16_t __arm_vcx3qa_m_impl(int, uint8x16_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_s16_u8)))
int16x8_t __arm_vreinterpretq_s16_u8(uint8x16_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_s32_u8)))
int32x4_t __arm_vreinterpretq_s32_u8(uint8x16_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_s64_u8)))
int64x2_t __arm_vreinterpretq_s64_u8(uint8x16_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_s8_u8)))
int8x16_t __arm_vreinterpretq_s8_u8(uint8x16_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u16_u8)))
uint16x8_t __arm_vreinterpretq_u16_u8(uint8x16_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u32_u8)))
uint32x4_t __arm_vreinterpretq_u32_u8(uint8x16_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u64_u8)))
uint64x2_t __arm_vreinterpretq_u64_u8(uint8x16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u8_s16)))
uint8x16_t __arm_vreinterpretq_u8(int16x8_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u8_s32)))
uint8x16_t __arm_vreinterpretq_u8(int32x4_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u8_s64)))
uint8x16_t __arm_vreinterpretq_u8(int64x2_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u8_s8)))
uint8x16_t __arm_vreinterpretq_u8(int8x16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u8_u16)))
uint8x16_t __arm_vreinterpretq_u8(uint16x8_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u8_u32)))
uint8x16_t __arm_vreinterpretq_u8(uint32x4_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u8_u64)))
uint8x16_t __arm_vreinterpretq_u8(uint64x2_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vreinterpretq_u8_u8)))
uint8x16_t __arm_vreinterpretq_u8(uint8x16_t);
#define __arm_vcx2q_m(cp, inactive, n, imm, pred) __arm_vcx2q_m_impl((cp), (inactive), __arm_vreinterpretq_u8(n), (imm), (pred))
#define __arm_vcx2qa(cp, acc, n, imm) __arm_vcx2qa_impl((cp), (acc), __arm_vreinterpretq_u8(n), (imm))
#define __arm_vcx2qa_m(cp, acc, n, imm, pred) __arm_vcx2qa_m_impl((cp), (acc), __arm_vreinterpretq_u8(n), (imm), (pred))
#define __arm_vcx3q(cp, n, m, imm) __arm_vcx3q_impl((cp), (n), __arm_vreinterpretq_u8(m), (imm))
#define __arm_vcx3q_m(cp, inactive, n, m, imm, pred) __arm_vcx3q_m_impl((cp), (inactive), __arm_vreinterpretq_u8(n), __arm_vreinterpretq_u8(m), (imm), (pred))
#define __arm_vcx3q_u8(cp, n, m, imm) __arm_vcx3q_u8_impl((cp), (n), __arm_vreinterpretq_u8(m), (imm))
#define __arm_vcx3qa(cp, acc, n, m, imm) __arm_vcx3qa_impl((cp), (acc), __arm_vreinterpretq_u8(n), __arm_vreinterpretq_u8(m), (imm))
#define __arm_vcx3qa_m(cp, acc, n, m, imm, pred) __arm_vcx3qa_m_impl((cp), (acc), __arm_vreinterpretq_u8(n), __arm_vreinterpretq_u8(m), (imm), (pred))

#endif /* __ARM_FEATURE_MVE */

#if __ARM_FEATURE_MVE & 2

typedef __fp16 float16_t;
typedef float float32_t;
typedef __attribute__((__neon_vector_type__(8), __clang_arm_mve_strict_polymorphism)) float16_t float16x8_t;
typedef __attribute__((__neon_vector_type__(4), __clang_arm_mve_strict_polymorphism)) float32_t float32x4_t;

static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_m_f16)))
float16x8_t __arm_vcx1q_m(int, float16x8_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1q_m_f32)))
float32x4_t __arm_vcx1q_m(int, float32x4_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_f16)))
float16x8_t __arm_vcx1qa(int, float16x8_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_f32)))
float32x4_t __arm_vcx1qa(int, float32x4_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_m_f16)))
float16x8_t __arm_vcx1qa_m(int, float16x8_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx1qa_m_f32)))
float32x4_t __arm_vcx1qa_m(int, float32x4_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_f16)))
float16x8_t __arm_vcx2q(int, float16x8_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_f32)))
float32x4_t __arm_vcx2q(int, float32x4_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_m_impl_f16)))
float16x8_t __arm_vcx2q_m_impl(int, float16x8_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_m_impl_f32)))
float32x4_t __arm_vcx2q_m_impl(int, float32x4_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8_f16)))
uint8x16_t __arm_vcx2q_u8(int, float16x8_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2q_u8_f32)))
uint8x16_t __arm_vcx2q_u8(int, float32x4_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_impl_f16)))
float16x8_t __arm_vcx2qa_impl(int, float16x8_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_impl_f32)))
float32x4_t __arm_vcx2qa_impl(int, float32x4_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_m_impl_f16)))
float16x8_t __arm_vcx2qa_m_impl(int, float16x8_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx2qa_m_impl_f32)))
float32x4_t __arm_vcx2qa_m_impl(int, float32x4_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_impl_f16)))
float16x8_t __arm_vcx3q_impl(int, float16x8_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_impl_f32)))
float32x4_t __arm_vcx3q_impl(int, float32x4_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_m_impl_f16)))
float16x8_t __arm_vcx3q_m_impl(int, float16x8_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_m_impl_f32)))
float32x4_t __arm_vcx3q_m_impl(int, float32x4_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_u8_impl_f16)))
uint8x16_t __arm_vcx3q_u8_impl(int, float16x8_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3q_u8_impl_f32)))
uint8x16_t __arm_vcx3q_u8_impl(int, float32x4_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_impl_f16)))
float16x8_t __arm_vcx3qa_impl(int, float16x8_t, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_impl_f32)))
float32x4_t __arm_vcx3qa_impl(int, float32x4_t, uint8x16_t, uint8x16_t, uint32_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_m_impl_f16)))
float16x8_t __arm_vcx3qa_m_impl(int, float16x8_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_cde_vcx3qa_m_impl_f32)))
float32x4_t __arm_vcx3qa_m_impl(int, float32x4_t, uint8x16_t, uint8x16_t, uint32_t, mve_pred16_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_f16_u8)))
float16x8_t __arm_vreinterpretq_f16_u8(uint8x16_t);
static __inline__ __attribute__((__clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_f32_u8)))
float32x4_t __arm_vreinterpretq_f32_u8(uint8x16_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u8_f16)))
uint8x16_t __arm_vreinterpretq_u8(float16x8_t);
static __inline__ __attribute__((__overloadable__, __clang_arm_builtin_alias(__builtin_arm_mve_vreinterpretq_u8_f32)))
uint8x16_t __arm_vreinterpretq_u8(float32x4_t);

#endif /* __ARM_FEATURE_MVE & 2 */

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* __ARM_CDE_H */
