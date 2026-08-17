/*===---- arm_sme.h - ARM SME intrinsics ------===
 *
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */

#ifndef __ARM_SME_H
#define __ARM_SME_H

#if !defined(__LITTLE_ENDIAN__)
#error "Big endian is currently not supported for arm_sme.h"
#endif
#include <arm_sve.h>

#include <stddef.h>

/* Function attributes */
#define __ai static __inline__ __attribute__((__always_inline__, __nodebug__))

#define __aio static __inline__ __attribute__((__always_inline__, __nodebug__, __overloadable__))

#ifdef  __cplusplus
extern "C" {
#endif

void __arm_za_disable(void) __arm_streaming_compatible;

__ai bool __arm_has_sme(void) __arm_streaming_compatible {
  uint64_t x0, x1;
  __builtin_arm_get_sme_state(&x0, &x1);
  return x0 & (1ULL << 63);
}

void *__arm_sc_memcpy(void *dest, const void *src, size_t n) __arm_streaming_compatible;
void *__arm_sc_memmove(void *dest, const void *src, size_t n) __arm_streaming_compatible;
void *__arm_sc_memset(void *s, int c, size_t n) __arm_streaming_compatible;
void *__arm_sc_memchr(void *s, int c, size_t n) __arm_streaming_compatible;

__ai __attribute__((target("sme"))) void svundef_za(void) __arm_streaming_compatible __arm_out("za") { }

__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme___arm_in_streaming_mode)))
bool __arm_in_streaming_mode(void);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddha_za32_u32_m)))
void svaddha_za32_u32_m(uint64_t, svbool_t, svbool_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddha_za32_s32_m)))
void svaddha_za32_s32_m(uint64_t, svbool_t, svbool_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddva_za32_u32_m)))
void svaddva_za32_u32_m(uint64_t, svbool_t, svbool_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddva_za32_s32_m)))
void svaddva_za32_s32_m(uint64_t, svbool_t, svbool_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svcntsb)))
uint64_t svcntsb(void);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svcntsd)))
uint64_t svcntsd(void);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svcntsh)))
uint64_t svcntsh(void);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svcntsw)))
uint64_t svcntsw(void);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_hor_vnum_za128)))
void svld1_hor_vnum_za128(uint64_t, uint32_t, svbool_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_hor_vnum_za16)))
void svld1_hor_vnum_za16(uint64_t, uint32_t, svbool_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_hor_vnum_za32)))
void svld1_hor_vnum_za32(uint64_t, uint32_t, svbool_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_hor_vnum_za64)))
void svld1_hor_vnum_za64(uint64_t, uint32_t, svbool_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_hor_vnum_za8)))
void svld1_hor_vnum_za8(uint64_t, uint32_t, svbool_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_hor_za128)))
void svld1_hor_za128(uint64_t, uint32_t, svbool_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_hor_za16)))
void svld1_hor_za16(uint64_t, uint32_t, svbool_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_hor_za32)))
void svld1_hor_za32(uint64_t, uint32_t, svbool_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_hor_za64)))
void svld1_hor_za64(uint64_t, uint32_t, svbool_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_hor_za8)))
void svld1_hor_za8(uint64_t, uint32_t, svbool_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_ver_vnum_za128)))
void svld1_ver_vnum_za128(uint64_t, uint32_t, svbool_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_ver_vnum_za16)))
void svld1_ver_vnum_za16(uint64_t, uint32_t, svbool_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_ver_vnum_za32)))
void svld1_ver_vnum_za32(uint64_t, uint32_t, svbool_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_ver_vnum_za64)))
void svld1_ver_vnum_za64(uint64_t, uint32_t, svbool_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_ver_vnum_za8)))
void svld1_ver_vnum_za8(uint64_t, uint32_t, svbool_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_ver_za128)))
void svld1_ver_za128(uint64_t, uint32_t, svbool_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_ver_za16)))
void svld1_ver_za16(uint64_t, uint32_t, svbool_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_ver_za32)))
void svld1_ver_za32(uint64_t, uint32_t, svbool_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_ver_za64)))
void svld1_ver_za64(uint64_t, uint32_t, svbool_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svld1_ver_za8)))
void svld1_ver_za8(uint64_t, uint32_t, svbool_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svldr_vnum_za)))
void svldr_vnum_za(uint32_t, void const *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svldr_za)))
void svldr_za(uint32_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_f16_m)))
void svmopa_za32_f16_m(uint64_t, svbool_t, svbool_t, svfloat16_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_bf16_m)))
void svmopa_za32_bf16_m(uint64_t, svbool_t, svbool_t, svbfloat16_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_f32_m)))
void svmopa_za32_f32_m(uint64_t, svbool_t, svbool_t, svfloat32_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_s8_m)))
void svmopa_za32_s8_m(uint64_t, svbool_t, svbool_t, svint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_u8_m)))
void svmopa_za32_u8_m(uint64_t, svbool_t, svbool_t, svuint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_f16_m)))
void svmops_za32_f16_m(uint64_t, svbool_t, svbool_t, svfloat16_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_bf16_m)))
void svmops_za32_bf16_m(uint64_t, svbool_t, svbool_t, svbfloat16_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_f32_m)))
void svmops_za32_f32_m(uint64_t, svbool_t, svbool_t, svfloat32_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_s8_m)))
void svmops_za32_s8_m(uint64_t, svbool_t, svbool_t, svint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_u8_m)))
void svmops_za32_u8_m(uint64_t, svbool_t, svbool_t, svuint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_u8_m)))
svuint8_t svread_hor_za128_u8_m(svuint8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_u32_m)))
svuint32_t svread_hor_za128_u32_m(svuint32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_u64_m)))
svuint64_t svread_hor_za128_u64_m(svuint64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_u16_m)))
svuint16_t svread_hor_za128_u16_m(svuint16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_bf16_m)))
svbfloat16_t svread_hor_za128_bf16_m(svbfloat16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_s8_m)))
svint8_t svread_hor_za128_s8_m(svint8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_f64_m)))
svfloat64_t svread_hor_za128_f64_m(svfloat64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_f32_m)))
svfloat32_t svread_hor_za128_f32_m(svfloat32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_f16_m)))
svfloat16_t svread_hor_za128_f16_m(svfloat16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_s32_m)))
svint32_t svread_hor_za128_s32_m(svint32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_s64_m)))
svint64_t svread_hor_za128_s64_m(svint64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_mf8_m)))
svmfloat8_t svread_hor_za128_mf8_m(svmfloat8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_s16_m)))
svint16_t svread_hor_za128_s16_m(svint16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_u16_m)))
svuint16_t svread_hor_za16_u16_m(svuint16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_bf16_m)))
svbfloat16_t svread_hor_za16_bf16_m(svbfloat16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_f16_m)))
svfloat16_t svread_hor_za16_f16_m(svfloat16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_s16_m)))
svint16_t svread_hor_za16_s16_m(svint16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_u32_m)))
svuint32_t svread_hor_za32_u32_m(svuint32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_f32_m)))
svfloat32_t svread_hor_za32_f32_m(svfloat32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_s32_m)))
svint32_t svread_hor_za32_s32_m(svint32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_u64_m)))
svuint64_t svread_hor_za64_u64_m(svuint64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_f64_m)))
svfloat64_t svread_hor_za64_f64_m(svfloat64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_s64_m)))
svint64_t svread_hor_za64_s64_m(svint64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_u8_m)))
svuint8_t svread_hor_za8_u8_m(svuint8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_s8_m)))
svint8_t svread_hor_za8_s8_m(svint8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_mf8_m)))
svmfloat8_t svread_hor_za8_mf8_m(svmfloat8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_u8_m)))
svuint8_t svread_ver_za128_u8_m(svuint8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_u32_m)))
svuint32_t svread_ver_za128_u32_m(svuint32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_u64_m)))
svuint64_t svread_ver_za128_u64_m(svuint64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_u16_m)))
svuint16_t svread_ver_za128_u16_m(svuint16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_bf16_m)))
svbfloat16_t svread_ver_za128_bf16_m(svbfloat16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_s8_m)))
svint8_t svread_ver_za128_s8_m(svint8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_f64_m)))
svfloat64_t svread_ver_za128_f64_m(svfloat64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_f32_m)))
svfloat32_t svread_ver_za128_f32_m(svfloat32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_f16_m)))
svfloat16_t svread_ver_za128_f16_m(svfloat16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_s32_m)))
svint32_t svread_ver_za128_s32_m(svint32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_s64_m)))
svint64_t svread_ver_za128_s64_m(svint64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_mf8_m)))
svmfloat8_t svread_ver_za128_mf8_m(svmfloat8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_s16_m)))
svint16_t svread_ver_za128_s16_m(svint16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_u16_m)))
svuint16_t svread_ver_za16_u16_m(svuint16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_bf16_m)))
svbfloat16_t svread_ver_za16_bf16_m(svbfloat16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_f16_m)))
svfloat16_t svread_ver_za16_f16_m(svfloat16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_s16_m)))
svint16_t svread_ver_za16_s16_m(svint16_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_u32_m)))
svuint32_t svread_ver_za32_u32_m(svuint32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_f32_m)))
svfloat32_t svread_ver_za32_f32_m(svfloat32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_s32_m)))
svint32_t svread_ver_za32_s32_m(svint32_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_u64_m)))
svuint64_t svread_ver_za64_u64_m(svuint64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_f64_m)))
svfloat64_t svread_ver_za64_f64_m(svfloat64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_s64_m)))
svint64_t svread_ver_za64_s64_m(svint64_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_u8_m)))
svuint8_t svread_ver_za8_u8_m(svuint8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_s8_m)))
svint8_t svread_ver_za8_s8_m(svint8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_mf8_m)))
svmfloat8_t svread_ver_za8_mf8_m(svmfloat8_t, svbool_t, uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_hor_vnum_za128)))
void svst1_hor_vnum_za128(uint64_t, uint32_t, svbool_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_hor_vnum_za16)))
void svst1_hor_vnum_za16(uint64_t, uint32_t, svbool_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_hor_vnum_za32)))
void svst1_hor_vnum_za32(uint64_t, uint32_t, svbool_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_hor_vnum_za64)))
void svst1_hor_vnum_za64(uint64_t, uint32_t, svbool_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_hor_vnum_za8)))
void svst1_hor_vnum_za8(uint64_t, uint32_t, svbool_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_hor_za128)))
void svst1_hor_za128(uint64_t, uint32_t, svbool_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_hor_za16)))
void svst1_hor_za16(uint64_t, uint32_t, svbool_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_hor_za32)))
void svst1_hor_za32(uint64_t, uint32_t, svbool_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_hor_za64)))
void svst1_hor_za64(uint64_t, uint32_t, svbool_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_hor_za8)))
void svst1_hor_za8(uint64_t, uint32_t, svbool_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_ver_vnum_za128)))
void svst1_ver_vnum_za128(uint64_t, uint32_t, svbool_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_ver_vnum_za16)))
void svst1_ver_vnum_za16(uint64_t, uint32_t, svbool_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_ver_vnum_za32)))
void svst1_ver_vnum_za32(uint64_t, uint32_t, svbool_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_ver_vnum_za64)))
void svst1_ver_vnum_za64(uint64_t, uint32_t, svbool_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_ver_vnum_za8)))
void svst1_ver_vnum_za8(uint64_t, uint32_t, svbool_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_ver_za128)))
void svst1_ver_za128(uint64_t, uint32_t, svbool_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_ver_za16)))
void svst1_ver_za16(uint64_t, uint32_t, svbool_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_ver_za32)))
void svst1_ver_za32(uint64_t, uint32_t, svbool_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_ver_za64)))
void svst1_ver_za64(uint64_t, uint32_t, svbool_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svst1_ver_za8)))
void svst1_ver_za8(uint64_t, uint32_t, svbool_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svstr_vnum_za)))
void svstr_vnum_za(uint32_t, void *, int64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svstr_za)))
void svstr_za(uint32_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumopa_za32_s8_m)))
void svsumopa_za32_s8_m(uint64_t, svbool_t, svbool_t, svint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumops_za32_s8_m)))
void svsumops_za32_s8_m(uint64_t, svbool_t, svbool_t, svint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmopa_za32_u8_m)))
void svusmopa_za32_u8_m(uint64_t, svbool_t, svbool_t, svuint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmops_za32_u8_m)))
void svusmops_za32_u8_m(uint64_t, svbool_t, svbool_t, svuint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_u8_m)))
void svwrite_hor_za128_u8_m(uint64_t, uint32_t, svbool_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_u32_m)))
void svwrite_hor_za128_u32_m(uint64_t, uint32_t, svbool_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_u64_m)))
void svwrite_hor_za128_u64_m(uint64_t, uint32_t, svbool_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_u16_m)))
void svwrite_hor_za128_u16_m(uint64_t, uint32_t, svbool_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_bf16_m)))
void svwrite_hor_za128_bf16_m(uint64_t, uint32_t, svbool_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_s8_m)))
void svwrite_hor_za128_s8_m(uint64_t, uint32_t, svbool_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_f64_m)))
void svwrite_hor_za128_f64_m(uint64_t, uint32_t, svbool_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_f32_m)))
void svwrite_hor_za128_f32_m(uint64_t, uint32_t, svbool_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_f16_m)))
void svwrite_hor_za128_f16_m(uint64_t, uint32_t, svbool_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_s32_m)))
void svwrite_hor_za128_s32_m(uint64_t, uint32_t, svbool_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_s64_m)))
void svwrite_hor_za128_s64_m(uint64_t, uint32_t, svbool_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_mf8_m)))
void svwrite_hor_za128_mf8_m(uint64_t, uint32_t, svbool_t, svmfloat8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_s16_m)))
void svwrite_hor_za128_s16_m(uint64_t, uint32_t, svbool_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_u16_m)))
void svwrite_hor_za16_u16_m(uint64_t, uint32_t, svbool_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_bf16_m)))
void svwrite_hor_za16_bf16_m(uint64_t, uint32_t, svbool_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_f16_m)))
void svwrite_hor_za16_f16_m(uint64_t, uint32_t, svbool_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_s16_m)))
void svwrite_hor_za16_s16_m(uint64_t, uint32_t, svbool_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_u32_m)))
void svwrite_hor_za32_u32_m(uint64_t, uint32_t, svbool_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_f32_m)))
void svwrite_hor_za32_f32_m(uint64_t, uint32_t, svbool_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_s32_m)))
void svwrite_hor_za32_s32_m(uint64_t, uint32_t, svbool_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_u64_m)))
void svwrite_hor_za64_u64_m(uint64_t, uint32_t, svbool_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_f64_m)))
void svwrite_hor_za64_f64_m(uint64_t, uint32_t, svbool_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_s64_m)))
void svwrite_hor_za64_s64_m(uint64_t, uint32_t, svbool_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_u8_m)))
void svwrite_hor_za8_u8_m(uint64_t, uint32_t, svbool_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_s8_m)))
void svwrite_hor_za8_s8_m(uint64_t, uint32_t, svbool_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_mf8_m)))
void svwrite_hor_za8_mf8_m(uint64_t, uint32_t, svbool_t, svmfloat8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_u8_m)))
void svwrite_ver_za128_u8_m(uint64_t, uint32_t, svbool_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_u32_m)))
void svwrite_ver_za128_u32_m(uint64_t, uint32_t, svbool_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_u64_m)))
void svwrite_ver_za128_u64_m(uint64_t, uint32_t, svbool_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_u16_m)))
void svwrite_ver_za128_u16_m(uint64_t, uint32_t, svbool_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_bf16_m)))
void svwrite_ver_za128_bf16_m(uint64_t, uint32_t, svbool_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_s8_m)))
void svwrite_ver_za128_s8_m(uint64_t, uint32_t, svbool_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_f64_m)))
void svwrite_ver_za128_f64_m(uint64_t, uint32_t, svbool_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_f32_m)))
void svwrite_ver_za128_f32_m(uint64_t, uint32_t, svbool_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_f16_m)))
void svwrite_ver_za128_f16_m(uint64_t, uint32_t, svbool_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_s32_m)))
void svwrite_ver_za128_s32_m(uint64_t, uint32_t, svbool_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_s64_m)))
void svwrite_ver_za128_s64_m(uint64_t, uint32_t, svbool_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_mf8_m)))
void svwrite_ver_za128_mf8_m(uint64_t, uint32_t, svbool_t, svmfloat8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_s16_m)))
void svwrite_ver_za128_s16_m(uint64_t, uint32_t, svbool_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_u16_m)))
void svwrite_ver_za16_u16_m(uint64_t, uint32_t, svbool_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_bf16_m)))
void svwrite_ver_za16_bf16_m(uint64_t, uint32_t, svbool_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_f16_m)))
void svwrite_ver_za16_f16_m(uint64_t, uint32_t, svbool_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_s16_m)))
void svwrite_ver_za16_s16_m(uint64_t, uint32_t, svbool_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_u32_m)))
void svwrite_ver_za32_u32_m(uint64_t, uint32_t, svbool_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_f32_m)))
void svwrite_ver_za32_f32_m(uint64_t, uint32_t, svbool_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_s32_m)))
void svwrite_ver_za32_s32_m(uint64_t, uint32_t, svbool_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_u64_m)))
void svwrite_ver_za64_u64_m(uint64_t, uint32_t, svbool_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_f64_m)))
void svwrite_ver_za64_f64_m(uint64_t, uint32_t, svbool_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_s64_m)))
void svwrite_ver_za64_s64_m(uint64_t, uint32_t, svbool_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_u8_m)))
void svwrite_ver_za8_u8_m(uint64_t, uint32_t, svbool_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_s8_m)))
void svwrite_ver_za8_s8_m(uint64_t, uint32_t, svbool_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_mf8_m)))
void svwrite_ver_za8_mf8_m(uint64_t, uint32_t, svbool_t, svmfloat8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_mask_za)))
void svzero_mask_za(uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_za)))
void svzero_za(void);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddha_za32_u32_m)))
void svaddha_za32_m(uint64_t, svbool_t, svbool_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddha_za32_s32_m)))
void svaddha_za32_m(uint64_t, svbool_t, svbool_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddva_za32_u32_m)))
void svaddva_za32_m(uint64_t, svbool_t, svbool_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddva_za32_s32_m)))
void svaddva_za32_m(uint64_t, svbool_t, svbool_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_f16_m)))
void svmopa_za32_m(uint64_t, svbool_t, svbool_t, svfloat16_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_bf16_m)))
void svmopa_za32_m(uint64_t, svbool_t, svbool_t, svbfloat16_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_f32_m)))
void svmopa_za32_m(uint64_t, svbool_t, svbool_t, svfloat32_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_s8_m)))
void svmopa_za32_m(uint64_t, svbool_t, svbool_t, svint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_u8_m)))
void svmopa_za32_m(uint64_t, svbool_t, svbool_t, svuint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_f16_m)))
void svmops_za32_m(uint64_t, svbool_t, svbool_t, svfloat16_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_bf16_m)))
void svmops_za32_m(uint64_t, svbool_t, svbool_t, svbfloat16_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_f32_m)))
void svmops_za32_m(uint64_t, svbool_t, svbool_t, svfloat32_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_s8_m)))
void svmops_za32_m(uint64_t, svbool_t, svbool_t, svint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_u8_m)))
void svmops_za32_m(uint64_t, svbool_t, svbool_t, svuint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_u8_m)))
svuint8_t svread_hor_za128_m(svuint8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_u32_m)))
svuint32_t svread_hor_za128_m(svuint32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_u64_m)))
svuint64_t svread_hor_za128_m(svuint64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_u16_m)))
svuint16_t svread_hor_za128_m(svuint16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_bf16_m)))
svbfloat16_t svread_hor_za128_m(svbfloat16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_s8_m)))
svint8_t svread_hor_za128_m(svint8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_f64_m)))
svfloat64_t svread_hor_za128_m(svfloat64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_f32_m)))
svfloat32_t svread_hor_za128_m(svfloat32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_f16_m)))
svfloat16_t svread_hor_za128_m(svfloat16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_s32_m)))
svint32_t svread_hor_za128_m(svint32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_s64_m)))
svint64_t svread_hor_za128_m(svint64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_mf8_m)))
svmfloat8_t svread_hor_za128_m(svmfloat8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za128_s16_m)))
svint16_t svread_hor_za128_m(svint16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_u16_m)))
svuint16_t svread_hor_za16_m(svuint16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_bf16_m)))
svbfloat16_t svread_hor_za16_m(svbfloat16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_f16_m)))
svfloat16_t svread_hor_za16_m(svfloat16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_s16_m)))
svint16_t svread_hor_za16_m(svint16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_u32_m)))
svuint32_t svread_hor_za32_m(svuint32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_f32_m)))
svfloat32_t svread_hor_za32_m(svfloat32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_s32_m)))
svint32_t svread_hor_za32_m(svint32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_u64_m)))
svuint64_t svread_hor_za64_m(svuint64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_f64_m)))
svfloat64_t svread_hor_za64_m(svfloat64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_s64_m)))
svint64_t svread_hor_za64_m(svint64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_u8_m)))
svuint8_t svread_hor_za8_m(svuint8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_s8_m)))
svint8_t svread_hor_za8_m(svint8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_mf8_m)))
svmfloat8_t svread_hor_za8_m(svmfloat8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_u8_m)))
svuint8_t svread_ver_za128_m(svuint8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_u32_m)))
svuint32_t svread_ver_za128_m(svuint32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_u64_m)))
svuint64_t svread_ver_za128_m(svuint64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_u16_m)))
svuint16_t svread_ver_za128_m(svuint16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_bf16_m)))
svbfloat16_t svread_ver_za128_m(svbfloat16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_s8_m)))
svint8_t svread_ver_za128_m(svint8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_f64_m)))
svfloat64_t svread_ver_za128_m(svfloat64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_f32_m)))
svfloat32_t svread_ver_za128_m(svfloat32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_f16_m)))
svfloat16_t svread_ver_za128_m(svfloat16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_s32_m)))
svint32_t svread_ver_za128_m(svint32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_s64_m)))
svint64_t svread_ver_za128_m(svint64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_mf8_m)))
svmfloat8_t svread_ver_za128_m(svmfloat8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za128_s16_m)))
svint16_t svread_ver_za128_m(svint16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_u16_m)))
svuint16_t svread_ver_za16_m(svuint16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_bf16_m)))
svbfloat16_t svread_ver_za16_m(svbfloat16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_f16_m)))
svfloat16_t svread_ver_za16_m(svfloat16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_s16_m)))
svint16_t svread_ver_za16_m(svint16_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_u32_m)))
svuint32_t svread_ver_za32_m(svuint32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_f32_m)))
svfloat32_t svread_ver_za32_m(svfloat32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_s32_m)))
svint32_t svread_ver_za32_m(svint32_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_u64_m)))
svuint64_t svread_ver_za64_m(svuint64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_f64_m)))
svfloat64_t svread_ver_za64_m(svfloat64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_s64_m)))
svint64_t svread_ver_za64_m(svint64_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_u8_m)))
svuint8_t svread_ver_za8_m(svuint8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_s8_m)))
svint8_t svread_ver_za8_m(svint8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_mf8_m)))
svmfloat8_t svread_ver_za8_m(svmfloat8_t, svbool_t, uint64_t, uint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumopa_za32_s8_m)))
void svsumopa_za32_m(uint64_t, svbool_t, svbool_t, svint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumops_za32_s8_m)))
void svsumops_za32_m(uint64_t, svbool_t, svbool_t, svint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmopa_za32_u8_m)))
void svusmopa_za32_m(uint64_t, svbool_t, svbool_t, svuint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmops_za32_u8_m)))
void svusmops_za32_m(uint64_t, svbool_t, svbool_t, svuint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_u8_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_u32_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_u64_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_u16_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_bf16_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_s8_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_f64_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_f32_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_f16_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_s32_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_s64_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_mf8_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svmfloat8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za128_s16_m)))
void svwrite_hor_za128_m(uint64_t, uint32_t, svbool_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_u16_m)))
void svwrite_hor_za16_m(uint64_t, uint32_t, svbool_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_bf16_m)))
void svwrite_hor_za16_m(uint64_t, uint32_t, svbool_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_f16_m)))
void svwrite_hor_za16_m(uint64_t, uint32_t, svbool_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_s16_m)))
void svwrite_hor_za16_m(uint64_t, uint32_t, svbool_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_u32_m)))
void svwrite_hor_za32_m(uint64_t, uint32_t, svbool_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_f32_m)))
void svwrite_hor_za32_m(uint64_t, uint32_t, svbool_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_s32_m)))
void svwrite_hor_za32_m(uint64_t, uint32_t, svbool_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_u64_m)))
void svwrite_hor_za64_m(uint64_t, uint32_t, svbool_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_f64_m)))
void svwrite_hor_za64_m(uint64_t, uint32_t, svbool_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_s64_m)))
void svwrite_hor_za64_m(uint64_t, uint32_t, svbool_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_u8_m)))
void svwrite_hor_za8_m(uint64_t, uint32_t, svbool_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_s8_m)))
void svwrite_hor_za8_m(uint64_t, uint32_t, svbool_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_mf8_m)))
void svwrite_hor_za8_m(uint64_t, uint32_t, svbool_t, svmfloat8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_u8_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_u32_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_u64_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_u16_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_bf16_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_s8_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_f64_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_f32_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_f16_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_s32_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_s64_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_mf8_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svmfloat8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za128_s16_m)))
void svwrite_ver_za128_m(uint64_t, uint32_t, svbool_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_u16_m)))
void svwrite_ver_za16_m(uint64_t, uint32_t, svbool_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_bf16_m)))
void svwrite_ver_za16_m(uint64_t, uint32_t, svbool_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_f16_m)))
void svwrite_ver_za16_m(uint64_t, uint32_t, svbool_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_s16_m)))
void svwrite_ver_za16_m(uint64_t, uint32_t, svbool_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_u32_m)))
void svwrite_ver_za32_m(uint64_t, uint32_t, svbool_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_f32_m)))
void svwrite_ver_za32_m(uint64_t, uint32_t, svbool_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_s32_m)))
void svwrite_ver_za32_m(uint64_t, uint32_t, svbool_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_u64_m)))
void svwrite_ver_za64_m(uint64_t, uint32_t, svbool_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_f64_m)))
void svwrite_ver_za64_m(uint64_t, uint32_t, svbool_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_s64_m)))
void svwrite_ver_za64_m(uint64_t, uint32_t, svbool_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_u8_m)))
void svwrite_ver_za8_m(uint64_t, uint32_t, svbool_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_s8_m)))
void svwrite_ver_za8_m(uint64_t, uint32_t, svbool_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_mf8_m)))
void svwrite_ver_za8_m(uint64_t, uint32_t, svbool_t, svmfloat8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za16_f16_vg1x2)))
void svadd_za16_f16_vg1x2(uint32_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za16_f16_vg1x4)))
void svadd_za16_f16_vg1x4(uint32_t, svfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za16_f16_vg1x2)))
void svsub_za16_f16_vg1x2(uint32_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za16_f16_vg1x4)))
void svsub_za16_f16_vg1x4(uint32_t, svfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za16_f16_vg1x2)))
void svadd_za16_vg1x2(uint32_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za16_f16_vg1x4)))
void svadd_za16_vg1x4(uint32_t, svfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za16_f16_vg1x2)))
void svsub_za16_vg1x2(uint32_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za16_f16_vg1x4)))
void svsub_za16_vg1x4(uint32_t, svfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za16_bf16_vg1x2)))
void svadd_za16_bf16_vg1x2(uint32_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za16_bf16_vg1x4)))
void svadd_za16_bf16_vg1x4(uint32_t, svbfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_bf16_vg1x2)))
void svmla_single_za16_bf16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_bf16_vg1x4)))
void svmla_single_za16_bf16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_bf16_vg1x2)))
void svmla_lane_za16_bf16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_bf16_vg1x4)))
void svmla_lane_za16_bf16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_bf16_vg1x2)))
void svmla_za16_bf16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_bf16_vg1x4)))
void svmla_za16_bf16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za16_bf16_vg1x2)))
void svmls_single_za16_bf16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za16_bf16_vg1x4)))
void svmls_single_za16_bf16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za16_bf16_vg1x2)))
void svmls_lane_za16_bf16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za16_bf16_vg1x4)))
void svmls_lane_za16_bf16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za16_bf16_vg1x2)))
void svmls_za16_bf16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za16_bf16_vg1x4)))
void svmls_za16_bf16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za16_bf16_m)))
void svmopa_za16_bf16_m(uint64_t, svbool_t, svbool_t, svbfloat16_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za16_bf16_m)))
void svmops_za16_bf16_m(uint64_t, svbool_t, svbool_t, svbfloat16_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za16_bf16_vg1x2)))
void svsub_za16_bf16_vg1x2(uint32_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za16_bf16_vg1x4)))
void svsub_za16_bf16_vg1x4(uint32_t, svbfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za16_bf16_vg1x2)))
void svadd_za16_vg1x2(uint32_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za16_bf16_vg1x4)))
void svadd_za16_vg1x4(uint32_t, svbfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_bf16_vg1x2)))
void svmla_za16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_bf16_vg1x4)))
void svmla_za16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_bf16_vg1x2)))
void svmla_lane_za16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_bf16_vg1x4)))
void svmla_lane_za16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_bf16_vg1x2)))
void svmla_za16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_bf16_vg1x4)))
void svmla_za16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za16_bf16_vg1x2)))
void svmls_za16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za16_bf16_vg1x4)))
void svmls_za16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za16_bf16_vg1x2)))
void svmls_lane_za16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za16_bf16_vg1x4)))
void svmls_lane_za16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za16_bf16_vg1x2)))
void svmls_za16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za16_bf16_vg1x4)))
void svmls_za16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za16_bf16_m)))
void svmopa_za16_m(uint64_t, svbool_t, svbool_t, svbfloat16_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za16_bf16_m)))
void svmops_za16_m(uint64_t, svbool_t, svbool_t, svbfloat16_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za16_bf16_vg1x2)))
void svsub_za16_vg1x2(uint32_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za16_bf16_vg1x4)))
void svsub_za16_vg1x4(uint32_t, svbfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_f16_vg1x2)))
void svmla_single_za16_f16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_f16_vg1x4)))
void svmla_single_za16_f16_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_f16_vg1x2)))
void svmla_lane_za16_f16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_f16_vg1x4)))
void svmla_lane_za16_f16_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_f16_vg1x2)))
void svmla_za16_f16_vg1x2(uint32_t, svfloat16x2_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_f16_vg1x4)))
void svmla_za16_f16_vg1x4(uint32_t, svfloat16x4_t, svfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za16_f16_vg1x2)))
void svmls_single_za16_f16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za16_f16_vg1x4)))
void svmls_single_za16_f16_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za16_f16_vg1x2)))
void svmls_lane_za16_f16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za16_f16_vg1x4)))
void svmls_lane_za16_f16_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za16_f16_vg1x2)))
void svmls_za16_f16_vg1x2(uint32_t, svfloat16x2_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za16_f16_vg1x4)))
void svmls_za16_f16_vg1x4(uint32_t, svfloat16x4_t, svfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za16_f16_m)))
void svmopa_za16_f16_m(uint64_t, svbool_t, svbool_t, svfloat16_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za16_f16_m)))
void svmops_za16_f16_m(uint64_t, svbool_t, svbool_t, svfloat16_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_f16_vg1x2)))
void svmla_za16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_f16_vg1x4)))
void svmla_za16_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_f16_vg1x2)))
void svmla_lane_za16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_f16_vg1x4)))
void svmla_lane_za16_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_f16_vg1x2)))
void svmla_za16_vg1x2(uint32_t, svfloat16x2_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_f16_vg1x4)))
void svmla_za16_vg1x4(uint32_t, svfloat16x4_t, svfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za16_f16_vg1x2)))
void svmls_za16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za16_f16_vg1x4)))
void svmls_za16_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za16_f16_vg1x2)))
void svmls_lane_za16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za16_f16_vg1x4)))
void svmls_lane_za16_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za16_f16_vg1x2)))
void svmls_za16_vg1x2(uint32_t, svfloat16x2_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za16_f16_vg1x4)))
void svmls_za16_vg1x4(uint32_t, svfloat16x4_t, svfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za16_f16_m)))
void svmopa_za16_m(uint64_t, svbool_t, svbool_t, svfloat16_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za16_f16_m)))
void svmops_za16_m(uint64_t, svbool_t, svbool_t, svfloat16_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za64_f64_m)))
void svmopa_za64_f64_m(uint64_t, svbool_t, svbool_t, svfloat64_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za64_f64_m)))
void svmops_za64_f64_m(uint64_t, svbool_t, svbool_t, svfloat64_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za64_f64_m)))
void svmopa_za64_m(uint64_t, svbool_t, svbool_t, svfloat64_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za64_f64_m)))
void svmops_za64_m(uint64_t, svbool_t, svbool_t, svfloat64_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za16_mf8_vg1x2_fpm)))
void svdot_single_za16_mf8_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za16_mf8_vg1x4_fpm)))
void svdot_single_za16_mf8_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za16_mf8_vg1x2_fpm)))
void svdot_lane_za16_mf8_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za16_mf8_vg1x4_fpm)))
void svdot_lane_za16_mf8_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za16_mf8_vg1x2_fpm)))
void svdot_za16_mf8_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8x2_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za16_mf8_vg1x4_fpm)))
void svdot_za16_mf8_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8x4_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_mf8_vg2x1_fpm)))
void svmla_single_za16_mf8_vg2x1_fpm(uint32_t, svmfloat8_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_mf8_vg2x2_fpm)))
void svmla_single_za16_mf8_vg2x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_mf8_vg2x4_fpm)))
void svmla_single_za16_mf8_vg2x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_mf8_vg2x1_fpm)))
void svmla_lane_za16_mf8_vg2x1_fpm(uint32_t, svmfloat8_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_mf8_vg2x2_fpm)))
void svmla_lane_za16_mf8_vg2x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_mf8_vg2x4_fpm)))
void svmla_lane_za16_mf8_vg2x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_mf8_vg2x2_fpm)))
void svmla_za16_mf8_vg2x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8x2_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_mf8_vg2x4_fpm)))
void svmla_za16_mf8_vg2x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8x4_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za16_mf8_m_fpm)))
void svmopa_za16_mf8_m_fpm(uint64_t, svbool_t, svbool_t, svmfloat8_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za16_mf8_vg1x2_fpm)))
void svvdot_lane_za16_mf8_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za16_mf8_vg1x2_fpm)))
void svdot_za16_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za16_mf8_vg1x4_fpm)))
void svdot_za16_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za16_mf8_vg1x2_fpm)))
void svdot_lane_za16_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za16_mf8_vg1x4_fpm)))
void svdot_lane_za16_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za16_mf8_vg1x2_fpm)))
void svdot_za16_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8x2_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za16_mf8_vg1x4_fpm)))
void svdot_za16_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8x4_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_mf8_vg2x1_fpm)))
void svmla_za16_vg2x1_fpm(uint32_t, svmfloat8_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_mf8_vg2x2_fpm)))
void svmla_za16_vg2x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za16_mf8_vg2x4_fpm)))
void svmla_za16_vg2x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_mf8_vg2x1_fpm)))
void svmla_lane_za16_vg2x1_fpm(uint32_t, svmfloat8_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_mf8_vg2x2_fpm)))
void svmla_lane_za16_vg2x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za16_mf8_vg2x4_fpm)))
void svmla_lane_za16_vg2x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_mf8_vg2x2_fpm)))
void svmla_za16_vg2x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8x2_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za16_mf8_vg2x4_fpm)))
void svmla_za16_vg2x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8x4_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za16_mf8_m_fpm)))
void svmopa_za16_m_fpm(uint64_t, svbool_t, svbool_t, svmfloat8_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za16_mf8_vg1x2_fpm)))
void svvdot_lane_za16_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_mf8_vg1x2_fpm)))
void svdot_single_za32_mf8_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_mf8_vg1x4_fpm)))
void svdot_single_za32_mf8_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_mf8_vg1x2_fpm)))
void svdot_lane_za32_mf8_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_mf8_vg1x4_fpm)))
void svdot_lane_za32_mf8_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_mf8_vg1x2_fpm)))
void svdot_za32_mf8_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8x2_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_mf8_vg1x4_fpm)))
void svdot_za32_mf8_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8x4_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_mf8_vg4x1_fpm)))
void svmla_single_za32_mf8_vg4x1_fpm(uint32_t, svmfloat8_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_mf8_vg4x2_fpm)))
void svmla_single_za32_mf8_vg4x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_mf8_vg4x4_fpm)))
void svmla_single_za32_mf8_vg4x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_mf8_vg4x1_fpm)))
void svmla_lane_za32_mf8_vg4x1_fpm(uint32_t, svmfloat8_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_mf8_vg4x2_fpm)))
void svmla_lane_za32_mf8_vg4x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_mf8_vg4x4_fpm)))
void svmla_lane_za32_mf8_vg4x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_mf8_vg4x2_fpm)))
void svmla_za32_mf8_vg4x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8x2_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_mf8_vg4x4_fpm)))
void svmla_za32_mf8_vg4x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8x4_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_mf8_m_fpm)))
void svmopa_za32_mf8_m_fpm(uint64_t, svbool_t, svbool_t, svmfloat8_t, svmfloat8_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdotb_lane_za32_mf8_vg1x4_fpm)))
void svvdotb_lane_za32_mf8_vg1x4_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdott_lane_za32_mf8_vg1x4_fpm)))
void svvdott_lane_za32_mf8_vg1x4_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_mf8_vg1x2_fpm)))
void svdot_za32_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_mf8_vg1x4_fpm)))
void svdot_za32_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_mf8_vg1x2_fpm)))
void svdot_lane_za32_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_mf8_vg1x4_fpm)))
void svdot_lane_za32_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_mf8_vg1x2_fpm)))
void svdot_za32_vg1x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8x2_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_mf8_vg1x4_fpm)))
void svdot_za32_vg1x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8x4_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_mf8_vg4x1_fpm)))
void svmla_za32_vg4x1_fpm(uint32_t, svmfloat8_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_mf8_vg4x2_fpm)))
void svmla_za32_vg4x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_mf8_vg4x4_fpm)))
void svmla_za32_vg4x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_mf8_vg4x1_fpm)))
void svmla_lane_za32_vg4x1_fpm(uint32_t, svmfloat8_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_mf8_vg4x2_fpm)))
void svmla_lane_za32_vg4x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_mf8_vg4x4_fpm)))
void svmla_lane_za32_vg4x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_mf8_vg4x2_fpm)))
void svmla_za32_vg4x2_fpm(uint32_t, svmfloat8x2_t, svmfloat8x2_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_mf8_vg4x4_fpm)))
void svmla_za32_vg4x4_fpm(uint32_t, svmfloat8x4_t, svmfloat8x4_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_mf8_m_fpm)))
void svmopa_za32_m_fpm(uint64_t, svbool_t, svbool_t, svmfloat8_t, svmfloat8_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdotb_lane_za32_mf8_vg1x4_fpm)))
void svvdotb_lane_za32_vg1x4_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdott_lane_za32_mf8_vg1x4_fpm)))
void svvdott_lane_za32_vg1x4_fpm(uint32_t, svmfloat8x2_t, svmfloat8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddha_za64_u64_m)))
void svaddha_za64_u64_m(uint64_t, svbool_t, svbool_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddha_za64_s64_m)))
void svaddha_za64_s64_m(uint64_t, svbool_t, svbool_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddva_za64_u64_m)))
void svaddva_za64_u64_m(uint64_t, svbool_t, svbool_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddva_za64_s64_m)))
void svaddva_za64_s64_m(uint64_t, svbool_t, svbool_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za64_s16_m)))
void svmopa_za64_s16_m(uint64_t, svbool_t, svbool_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za64_u16_m)))
void svmopa_za64_u16_m(uint64_t, svbool_t, svbool_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za64_s16_m)))
void svmops_za64_s16_m(uint64_t, svbool_t, svbool_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za64_u16_m)))
void svmops_za64_u16_m(uint64_t, svbool_t, svbool_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumopa_za64_s16_m)))
void svsumopa_za64_s16_m(uint64_t, svbool_t, svbool_t, svint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumops_za64_s16_m)))
void svsumops_za64_s16_m(uint64_t, svbool_t, svbool_t, svint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmopa_za64_u16_m)))
void svusmopa_za64_u16_m(uint64_t, svbool_t, svbool_t, svuint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmops_za64_u16_m)))
void svusmops_za64_u16_m(uint64_t, svbool_t, svbool_t, svuint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddha_za64_u64_m)))
void svaddha_za64_m(uint64_t, svbool_t, svbool_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddha_za64_s64_m)))
void svaddha_za64_m(uint64_t, svbool_t, svbool_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddva_za64_u64_m)))
void svaddva_za64_m(uint64_t, svbool_t, svbool_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svaddva_za64_s64_m)))
void svaddva_za64_m(uint64_t, svbool_t, svbool_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za64_s16_m)))
void svmopa_za64_m(uint64_t, svbool_t, svbool_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za64_u16_m)))
void svmopa_za64_m(uint64_t, svbool_t, svbool_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za64_s16_m)))
void svmops_za64_m(uint64_t, svbool_t, svbool_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za64_u16_m)))
void svmops_za64_m(uint64_t, svbool_t, svbool_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumopa_za64_s16_m)))
void svsumopa_za64_m(uint64_t, svbool_t, svbool_t, svint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumops_za64_s16_m)))
void svsumops_za64_m(uint64_t, svbool_t, svbool_t, svint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmopa_za64_u16_m)))
void svusmopa_za64_m(uint64_t, svbool_t, svbool_t, svuint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmops_za64_u16_m)))
void svusmops_za64_m(uint64_t, svbool_t, svbool_t, svuint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_zt_u8_x4)))
svuint8x4_t svluti4_zt_u8_x4(uint64_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_zt_s8_x4)))
svint8x4_t svluti4_zt_s8_x4(uint64_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_u8)))
void svwrite_lane_zt_u8(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_u32)))
void svwrite_lane_zt_u32(uint64_t, svuint32_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_u64)))
void svwrite_lane_zt_u64(uint64_t, svuint64_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_u16)))
void svwrite_lane_zt_u16(uint64_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_bf16)))
void svwrite_lane_zt_bf16(uint64_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_s8)))
void svwrite_lane_zt_s8(uint64_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_f64)))
void svwrite_lane_zt_f64(uint64_t, svfloat64_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_f32)))
void svwrite_lane_zt_f32(uint64_t, svfloat32_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_f16)))
void svwrite_lane_zt_f16(uint64_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_s32)))
void svwrite_lane_zt_s32(uint64_t, svint32_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_s64)))
void svwrite_lane_zt_s64(uint64_t, svint64_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_s16)))
void svwrite_lane_zt_s16(uint64_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_u8)))
void svwrite_zt_u8(uint64_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_u32)))
void svwrite_zt_u32(uint64_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_u64)))
void svwrite_zt_u64(uint64_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_u16)))
void svwrite_zt_u16(uint64_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_bf16)))
void svwrite_zt_bf16(uint64_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_s8)))
void svwrite_zt_s8(uint64_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_f64)))
void svwrite_zt_f64(uint64_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_f32)))
void svwrite_zt_f32(uint64_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_f16)))
void svwrite_zt_f16(uint64_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_s32)))
void svwrite_zt_s32(uint64_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_s64)))
void svwrite_zt_s64(uint64_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_s16)))
void svwrite_zt_s16(uint64_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_u8)))
void svwrite_lane_zt(uint64_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_u32)))
void svwrite_lane_zt(uint64_t, svuint32_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_u64)))
void svwrite_lane_zt(uint64_t, svuint64_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_u16)))
void svwrite_lane_zt(uint64_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_bf16)))
void svwrite_lane_zt(uint64_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_s8)))
void svwrite_lane_zt(uint64_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_f64)))
void svwrite_lane_zt(uint64_t, svfloat64_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_f32)))
void svwrite_lane_zt(uint64_t, svfloat32_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_f16)))
void svwrite_lane_zt(uint64_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_s32)))
void svwrite_lane_zt(uint64_t, svint32_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_s64)))
void svwrite_lane_zt(uint64_t, svint64_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_lane_zt_s16)))
void svwrite_lane_zt(uint64_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_u8)))
void svwrite_zt(uint64_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_u32)))
void svwrite_zt(uint64_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_u64)))
void svwrite_zt(uint64_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_u16)))
void svwrite_zt(uint64_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_bf16)))
void svwrite_zt(uint64_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_s8)))
void svwrite_zt(uint64_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_f64)))
void svwrite_zt(uint64_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_f32)))
void svwrite_zt(uint64_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_f16)))
void svwrite_zt(uint64_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_s32)))
void svwrite_zt(uint64_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_s64)))
void svwrite_zt(uint64_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_zt_s16)))
void svwrite_zt(uint64_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za32_u32_vg1x2)))
void svadd_write_single_za32_u32_vg1x2(uint32_t, svuint32x2_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za32_s32_vg1x2)))
void svadd_write_single_za32_s32_vg1x2(uint32_t, svint32x2_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za32_u32_vg1x4)))
void svadd_write_single_za32_u32_vg1x4(uint32_t, svuint32x4_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za32_s32_vg1x4)))
void svadd_write_single_za32_s32_vg1x4(uint32_t, svint32x4_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za32_u32_vg1x2)))
void svadd_write_za32_u32_vg1x2(uint32_t, svuint32x2_t, svuint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za32_s32_vg1x2)))
void svadd_write_za32_s32_vg1x2(uint32_t, svint32x2_t, svint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za32_u32_vg1x4)))
void svadd_write_za32_u32_vg1x4(uint32_t, svuint32x4_t, svuint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za32_s32_vg1x4)))
void svadd_write_za32_s32_vg1x4(uint32_t, svint32x4_t, svint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_u32_vg1x2)))
void svadd_za32_u32_vg1x2(uint32_t, svuint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_f32_vg1x2)))
void svadd_za32_f32_vg1x2(uint32_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_s32_vg1x2)))
void svadd_za32_s32_vg1x2(uint32_t, svint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_u32_vg1x4)))
void svadd_za32_u32_vg1x4(uint32_t, svuint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_f32_vg1x4)))
void svadd_za32_f32_vg1x4(uint32_t, svfloat32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_s32_vg1x4)))
void svadd_za32_s32_vg1x4(uint32_t, svint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svbmopa_za32_u32_m)))
void svbmopa_za32_u32_m(uint64_t, svbool_t, svbool_t, svuint32_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svbmopa_za32_s32_m)))
void svbmopa_za32_s32_m(uint64_t, svbool_t, svbool_t, svint32_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svbmops_za32_u32_m)))
void svbmops_za32_u32_m(uint64_t, svbool_t, svbool_t, svuint32_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svbmops_za32_s32_m)))
void svbmops_za32_s32_m(uint64_t, svbool_t, svbool_t, svint32_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_bf16_vg1x2)))
void svdot_single_za32_bf16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_f16_vg1x2)))
void svdot_single_za32_f16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_s8_vg1x2)))
void svdot_single_za32_s8_vg1x2(uint32_t, svint8x2_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_s16_vg1x2)))
void svdot_single_za32_s16_vg1x2(uint32_t, svint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_u8_vg1x2)))
void svdot_single_za32_u8_vg1x2(uint32_t, svuint8x2_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_u16_vg1x2)))
void svdot_single_za32_u16_vg1x2(uint32_t, svuint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_bf16_vg1x4)))
void svdot_single_za32_bf16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_f16_vg1x4)))
void svdot_single_za32_f16_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_s8_vg1x4)))
void svdot_single_za32_s8_vg1x4(uint32_t, svint8x4_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_s16_vg1x4)))
void svdot_single_za32_s16_vg1x4(uint32_t, svint16x4_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_u8_vg1x4)))
void svdot_single_za32_u8_vg1x4(uint32_t, svuint8x4_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_u16_vg1x4)))
void svdot_single_za32_u16_vg1x4(uint32_t, svuint16x4_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_bf16_vg1x2)))
void svdot_lane_za32_bf16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_f16_vg1x2)))
void svdot_lane_za32_f16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_s8_vg1x2)))
void svdot_lane_za32_s8_vg1x2(uint32_t, svint8x2_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_s16_vg1x2)))
void svdot_lane_za32_s16_vg1x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_u8_vg1x2)))
void svdot_lane_za32_u8_vg1x2(uint32_t, svuint8x2_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_u16_vg1x2)))
void svdot_lane_za32_u16_vg1x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_bf16_vg1x4)))
void svdot_lane_za32_bf16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_f16_vg1x4)))
void svdot_lane_za32_f16_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_s8_vg1x4)))
void svdot_lane_za32_s8_vg1x4(uint32_t, svint8x4_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_s16_vg1x4)))
void svdot_lane_za32_s16_vg1x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_u8_vg1x4)))
void svdot_lane_za32_u8_vg1x4(uint32_t, svuint8x4_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_u16_vg1x4)))
void svdot_lane_za32_u16_vg1x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_bf16_vg1x2)))
void svdot_za32_bf16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_f16_vg1x2)))
void svdot_za32_f16_vg1x2(uint32_t, svfloat16x2_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_s8_vg1x2)))
void svdot_za32_s8_vg1x2(uint32_t, svint8x2_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_s16_vg1x2)))
void svdot_za32_s16_vg1x2(uint32_t, svint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_u8_vg1x2)))
void svdot_za32_u8_vg1x2(uint32_t, svuint8x2_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_u16_vg1x2)))
void svdot_za32_u16_vg1x2(uint32_t, svuint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_bf16_vg1x4)))
void svdot_za32_bf16_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_f16_vg1x4)))
void svdot_za32_f16_vg1x4(uint32_t, svfloat16x4_t, svfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_s8_vg1x4)))
void svdot_za32_s8_vg1x4(uint32_t, svint8x4_t, svint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_s16_vg1x4)))
void svdot_za32_s16_vg1x4(uint32_t, svint16x4_t, svint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_u8_vg1x4)))
void svdot_za32_u8_vg1x4(uint32_t, svuint8x4_t, svuint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_u16_vg1x4)))
void svdot_za32_u16_vg1x4(uint32_t, svuint16x4_t, svuint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svldr_zt)))
void svldr_zt(uint64_t, void const *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_u8)))
svuint8_t svluti2_lane_zt_u8(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_u32)))
svuint32_t svluti2_lane_zt_u32(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_u16)))
svuint16_t svluti2_lane_zt_u16(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_bf16)))
svbfloat16_t svluti2_lane_zt_bf16(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_s8)))
svint8_t svluti2_lane_zt_s8(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_f32)))
svfloat32_t svluti2_lane_zt_f32(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_f16)))
svfloat16_t svluti2_lane_zt_f16(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_s32)))
svint32_t svluti2_lane_zt_s32(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_mf8)))
svmfloat8_t svluti2_lane_zt_mf8(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_s16)))
svint16_t svluti2_lane_zt_s16(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_u8_x2)))
svuint8x2_t svluti2_lane_zt_u8_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_u32_x2)))
svuint32x2_t svluti2_lane_zt_u32_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_u16_x2)))
svuint16x2_t svluti2_lane_zt_u16_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_bf16_x2)))
svbfloat16x2_t svluti2_lane_zt_bf16_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_s8_x2)))
svint8x2_t svluti2_lane_zt_s8_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_f32_x2)))
svfloat32x2_t svluti2_lane_zt_f32_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_f16_x2)))
svfloat16x2_t svluti2_lane_zt_f16_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_s32_x2)))
svint32x2_t svluti2_lane_zt_s32_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_mf8_x2)))
svmfloat8x2_t svluti2_lane_zt_mf8_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_s16_x2)))
svint16x2_t svluti2_lane_zt_s16_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_u8_x4)))
svuint8x4_t svluti2_lane_zt_u8_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_u32_x4)))
svuint32x4_t svluti2_lane_zt_u32_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_u16_x4)))
svuint16x4_t svluti2_lane_zt_u16_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_bf16_x4)))
svbfloat16x4_t svluti2_lane_zt_bf16_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_s8_x4)))
svint8x4_t svluti2_lane_zt_s8_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_f32_x4)))
svfloat32x4_t svluti2_lane_zt_f32_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_f16_x4)))
svfloat16x4_t svluti2_lane_zt_f16_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_s32_x4)))
svint32x4_t svluti2_lane_zt_s32_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_mf8_x4)))
svmfloat8x4_t svluti2_lane_zt_mf8_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti2_lane_zt_s16_x4)))
svint16x4_t svluti2_lane_zt_s16_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_u8)))
svuint8_t svluti4_lane_zt_u8(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_u32)))
svuint32_t svluti4_lane_zt_u32(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_u16)))
svuint16_t svluti4_lane_zt_u16(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_bf16)))
svbfloat16_t svluti4_lane_zt_bf16(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_s8)))
svint8_t svluti4_lane_zt_s8(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_f32)))
svfloat32_t svluti4_lane_zt_f32(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_f16)))
svfloat16_t svluti4_lane_zt_f16(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_s32)))
svint32_t svluti4_lane_zt_s32(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_mf8)))
svmfloat8_t svluti4_lane_zt_mf8(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_s16)))
svint16_t svluti4_lane_zt_s16(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_u8_x2)))
svuint8x2_t svluti4_lane_zt_u8_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_u32_x2)))
svuint32x2_t svluti4_lane_zt_u32_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_u16_x2)))
svuint16x2_t svluti4_lane_zt_u16_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_bf16_x2)))
svbfloat16x2_t svluti4_lane_zt_bf16_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_s8_x2)))
svint8x2_t svluti4_lane_zt_s8_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_f32_x2)))
svfloat32x2_t svluti4_lane_zt_f32_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_f16_x2)))
svfloat16x2_t svluti4_lane_zt_f16_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_s32_x2)))
svint32x2_t svluti4_lane_zt_s32_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_mf8_x2)))
svmfloat8x2_t svluti4_lane_zt_mf8_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_s16_x2)))
svint16x2_t svluti4_lane_zt_s16_x2(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_u32_x4)))
svuint32x4_t svluti4_lane_zt_u32_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_u16_x4)))
svuint16x4_t svluti4_lane_zt_u16_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_bf16_x4)))
svbfloat16x4_t svluti4_lane_zt_bf16_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_f32_x4)))
svfloat32x4_t svluti4_lane_zt_f32_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_f16_x4)))
svfloat16x4_t svluti4_lane_zt_f16_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_s32_x4)))
svint32x4_t svluti4_lane_zt_s32_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svluti4_lane_zt_s16_x4)))
svint16x4_t svluti4_lane_zt_s16_x4(uint64_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_f32_vg1x2)))
void svmla_single_za32_f32_vg1x2(uint32_t, svfloat32x2_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_f32_vg1x4)))
void svmla_single_za32_f32_vg1x4(uint32_t, svfloat32x4_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_bf16_vg2x2)))
void svmla_single_za32_bf16_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_f16_vg2x2)))
void svmla_single_za32_f16_vg2x2(uint32_t, svfloat16x2_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_s16_vg2x2)))
void svmla_single_za32_s16_vg2x2(uint32_t, svint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_u16_vg2x2)))
void svmla_single_za32_u16_vg2x2(uint32_t, svuint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_bf16_vg2x4)))
void svmla_single_za32_bf16_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_f16_vg2x4)))
void svmla_single_za32_f16_vg2x4(uint32_t, svfloat16x4_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_s16_vg2x4)))
void svmla_single_za32_s16_vg2x4(uint32_t, svint16x4_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_u16_vg2x4)))
void svmla_single_za32_u16_vg2x4(uint32_t, svuint16x4_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_s8_vg4x2)))
void svmla_single_za32_s8_vg4x2(uint32_t, svint8x2_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_u8_vg4x2)))
void svmla_single_za32_u8_vg4x2(uint32_t, svuint8x2_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_s8_vg4x4)))
void svmla_single_za32_s8_vg4x4(uint32_t, svint8x4_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_u8_vg4x4)))
void svmla_single_za32_u8_vg4x4(uint32_t, svuint8x4_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_f32_vg1x2)))
void svmla_lane_za32_f32_vg1x2(uint32_t, svfloat32x2_t, svfloat32_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_f32_vg1x4)))
void svmla_lane_za32_f32_vg1x4(uint32_t, svfloat32x4_t, svfloat32_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_bf16_vg2x1)))
void svmla_lane_za32_bf16_vg2x1(uint32_t, svbfloat16_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_f16_vg2x1)))
void svmla_lane_za32_f16_vg2x1(uint32_t, svfloat16_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s16_vg2x1)))
void svmla_lane_za32_s16_vg2x1(uint32_t, svint16_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u16_vg2x1)))
void svmla_lane_za32_u16_vg2x1(uint32_t, svuint16_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_bf16_vg2x2)))
void svmla_lane_za32_bf16_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_f16_vg2x2)))
void svmla_lane_za32_f16_vg2x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s16_vg2x2)))
void svmla_lane_za32_s16_vg2x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u16_vg2x2)))
void svmla_lane_za32_u16_vg2x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_bf16_vg2x4)))
void svmla_lane_za32_bf16_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_f16_vg2x4)))
void svmla_lane_za32_f16_vg2x4(uint32_t, svfloat16x4_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s16_vg2x4)))
void svmla_lane_za32_s16_vg2x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u16_vg2x4)))
void svmla_lane_za32_u16_vg2x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s8_vg4x1)))
void svmla_lane_za32_s8_vg4x1(uint32_t, svint8_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u8_vg4x1)))
void svmla_lane_za32_u8_vg4x1(uint32_t, svuint8_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s8_vg4x2)))
void svmla_lane_za32_s8_vg4x2(uint32_t, svint8x2_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u8_vg4x2)))
void svmla_lane_za32_u8_vg4x2(uint32_t, svuint8x2_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s8_vg4x4)))
void svmla_lane_za32_s8_vg4x4(uint32_t, svint8x4_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u8_vg4x4)))
void svmla_lane_za32_u8_vg4x4(uint32_t, svuint8x4_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_f32_vg1x2)))
void svmla_za32_f32_vg1x2(uint32_t, svfloat32x2_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_f32_vg1x4)))
void svmla_za32_f32_vg1x4(uint32_t, svfloat32x4_t, svfloat32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_bf16_vg2x1)))
void svmla_za32_bf16_vg2x1(uint32_t, svbfloat16_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_f16_vg2x1)))
void svmla_za32_f16_vg2x1(uint32_t, svfloat16_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s16_vg2x1)))
void svmla_za32_s16_vg2x1(uint32_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u16_vg2x1)))
void svmla_za32_u16_vg2x1(uint32_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_bf16_vg2x2)))
void svmla_za32_bf16_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_f16_vg2x2)))
void svmla_za32_f16_vg2x2(uint32_t, svfloat16x2_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s16_vg2x2)))
void svmla_za32_s16_vg2x2(uint32_t, svint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u16_vg2x2)))
void svmla_za32_u16_vg2x2(uint32_t, svuint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_bf16_vg2x4)))
void svmla_za32_bf16_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_f16_vg2x4)))
void svmla_za32_f16_vg2x4(uint32_t, svfloat16x4_t, svfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s16_vg2x4)))
void svmla_za32_s16_vg2x4(uint32_t, svint16x4_t, svint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u16_vg2x4)))
void svmla_za32_u16_vg2x4(uint32_t, svuint16x4_t, svuint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s8_vg4x1)))
void svmla_za32_s8_vg4x1(uint32_t, svint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u8_vg4x1)))
void svmla_za32_u8_vg4x1(uint32_t, svuint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s8_vg4x2)))
void svmla_za32_s8_vg4x2(uint32_t, svint8x2_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u8_vg4x2)))
void svmla_za32_u8_vg4x2(uint32_t, svuint8x2_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s8_vg4x4)))
void svmla_za32_s8_vg4x4(uint32_t, svint8x4_t, svint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u8_vg4x4)))
void svmla_za32_u8_vg4x4(uint32_t, svuint8x4_t, svuint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_f32_vg1x2)))
void svmls_single_za32_f32_vg1x2(uint32_t, svfloat32x2_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_f32_vg1x4)))
void svmls_single_za32_f32_vg1x4(uint32_t, svfloat32x4_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_bf16_vg2x2)))
void svmls_single_za32_bf16_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_f16_vg2x2)))
void svmls_single_za32_f16_vg2x2(uint32_t, svfloat16x2_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_s16_vg2x2)))
void svmls_single_za32_s16_vg2x2(uint32_t, svint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_u16_vg2x2)))
void svmls_single_za32_u16_vg2x2(uint32_t, svuint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_bf16_vg2x4)))
void svmls_single_za32_bf16_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_f16_vg2x4)))
void svmls_single_za32_f16_vg2x4(uint32_t, svfloat16x4_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_s16_vg2x4)))
void svmls_single_za32_s16_vg2x4(uint32_t, svint16x4_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_u16_vg2x4)))
void svmls_single_za32_u16_vg2x4(uint32_t, svuint16x4_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_s8_vg4x2)))
void svmls_single_za32_s8_vg4x2(uint32_t, svint8x2_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_u8_vg4x2)))
void svmls_single_za32_u8_vg4x2(uint32_t, svuint8x2_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_s8_vg4x4)))
void svmls_single_za32_s8_vg4x4(uint32_t, svint8x4_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_u8_vg4x4)))
void svmls_single_za32_u8_vg4x4(uint32_t, svuint8x4_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_f32_vg1x2)))
void svmls_lane_za32_f32_vg1x2(uint32_t, svfloat32x2_t, svfloat32_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_f32_vg1x4)))
void svmls_lane_za32_f32_vg1x4(uint32_t, svfloat32x4_t, svfloat32_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_bf16_vg2x1)))
void svmls_lane_za32_bf16_vg2x1(uint32_t, svbfloat16_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_f16_vg2x1)))
void svmls_lane_za32_f16_vg2x1(uint32_t, svfloat16_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s16_vg2x1)))
void svmls_lane_za32_s16_vg2x1(uint32_t, svint16_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u16_vg2x1)))
void svmls_lane_za32_u16_vg2x1(uint32_t, svuint16_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_bf16_vg2x2)))
void svmls_lane_za32_bf16_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_f16_vg2x2)))
void svmls_lane_za32_f16_vg2x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s16_vg2x2)))
void svmls_lane_za32_s16_vg2x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u16_vg2x2)))
void svmls_lane_za32_u16_vg2x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_bf16_vg2x4)))
void svmls_lane_za32_bf16_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_f16_vg2x4)))
void svmls_lane_za32_f16_vg2x4(uint32_t, svfloat16x4_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s16_vg2x4)))
void svmls_lane_za32_s16_vg2x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u16_vg2x4)))
void svmls_lane_za32_u16_vg2x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s8_vg4x1)))
void svmls_lane_za32_s8_vg4x1(uint32_t, svint8_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u8_vg4x1)))
void svmls_lane_za32_u8_vg4x1(uint32_t, svuint8_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s8_vg4x2)))
void svmls_lane_za32_s8_vg4x2(uint32_t, svint8x2_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u8_vg4x2)))
void svmls_lane_za32_u8_vg4x2(uint32_t, svuint8x2_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s8_vg4x4)))
void svmls_lane_za32_s8_vg4x4(uint32_t, svint8x4_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u8_vg4x4)))
void svmls_lane_za32_u8_vg4x4(uint32_t, svuint8x4_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_f32_vg1x2)))
void svmls_za32_f32_vg1x2(uint32_t, svfloat32x2_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_f32_vg1x4)))
void svmls_za32_f32_vg1x4(uint32_t, svfloat32x4_t, svfloat32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_bf16_vg2x1)))
void svmls_za32_bf16_vg2x1(uint32_t, svbfloat16_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_f16_vg2x1)))
void svmls_za32_f16_vg2x1(uint32_t, svfloat16_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s16_vg2x1)))
void svmls_za32_s16_vg2x1(uint32_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u16_vg2x1)))
void svmls_za32_u16_vg2x1(uint32_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_bf16_vg2x2)))
void svmls_za32_bf16_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_f16_vg2x2)))
void svmls_za32_f16_vg2x2(uint32_t, svfloat16x2_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s16_vg2x2)))
void svmls_za32_s16_vg2x2(uint32_t, svint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u16_vg2x2)))
void svmls_za32_u16_vg2x2(uint32_t, svuint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_bf16_vg2x4)))
void svmls_za32_bf16_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_f16_vg2x4)))
void svmls_za32_f16_vg2x4(uint32_t, svfloat16x4_t, svfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s16_vg2x4)))
void svmls_za32_s16_vg2x4(uint32_t, svint16x4_t, svint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u16_vg2x4)))
void svmls_za32_u16_vg2x4(uint32_t, svuint16x4_t, svuint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s8_vg4x1)))
void svmls_za32_s8_vg4x1(uint32_t, svint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u8_vg4x1)))
void svmls_za32_u8_vg4x1(uint32_t, svuint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s8_vg4x2)))
void svmls_za32_s8_vg4x2(uint32_t, svint8x2_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u8_vg4x2)))
void svmls_za32_u8_vg4x2(uint32_t, svuint8x2_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s8_vg4x4)))
void svmls_za32_s8_vg4x4(uint32_t, svint8x4_t, svint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u8_vg4x4)))
void svmls_za32_u8_vg4x4(uint32_t, svuint8x4_t, svuint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_s16_m)))
void svmopa_za32_s16_m(uint64_t, svbool_t, svbool_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_u16_m)))
void svmopa_za32_u16_m(uint64_t, svbool_t, svbool_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_s16_m)))
void svmops_za32_s16_m(uint64_t, svbool_t, svbool_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_u16_m)))
void svmops_za32_u16_m(uint64_t, svbool_t, svbool_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_u16_vg2)))
svuint16x2_t svread_hor_za16_u16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_bf16_vg2)))
svbfloat16x2_t svread_hor_za16_bf16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_f16_vg2)))
svfloat16x2_t svread_hor_za16_f16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_s16_vg2)))
svint16x2_t svread_hor_za16_s16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_u16_vg4)))
svuint16x4_t svread_hor_za16_u16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_bf16_vg4)))
svbfloat16x4_t svread_hor_za16_bf16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_f16_vg4)))
svfloat16x4_t svread_hor_za16_f16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za16_s16_vg4)))
svint16x4_t svread_hor_za16_s16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_u32_vg2)))
svuint32x2_t svread_hor_za32_u32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_f32_vg2)))
svfloat32x2_t svread_hor_za32_f32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_s32_vg2)))
svint32x2_t svread_hor_za32_s32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_u32_vg4)))
svuint32x4_t svread_hor_za32_u32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_f32_vg4)))
svfloat32x4_t svread_hor_za32_f32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za32_s32_vg4)))
svint32x4_t svread_hor_za32_s32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_u64_vg2)))
svuint64x2_t svread_hor_za64_u64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_f64_vg2)))
svfloat64x2_t svread_hor_za64_f64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_s64_vg2)))
svint64x2_t svread_hor_za64_s64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_u64_vg4)))
svuint64x4_t svread_hor_za64_u64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_f64_vg4)))
svfloat64x4_t svread_hor_za64_f64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za64_s64_vg4)))
svint64x4_t svread_hor_za64_s64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_u8_vg2)))
svuint8x2_t svread_hor_za8_u8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_s8_vg2)))
svint8x2_t svread_hor_za8_s8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_mf8_vg2)))
svmfloat8x2_t svread_hor_za8_mf8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_u8_vg4)))
svuint8x4_t svread_hor_za8_u8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_s8_vg4)))
svint8x4_t svread_hor_za8_s8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_hor_za8_mf8_vg4)))
svmfloat8x4_t svread_hor_za8_mf8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_u16_vg2)))
svuint16x2_t svread_ver_za16_u16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_bf16_vg2)))
svbfloat16x2_t svread_ver_za16_bf16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_f16_vg2)))
svfloat16x2_t svread_ver_za16_f16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_s16_vg2)))
svint16x2_t svread_ver_za16_s16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_u16_vg4)))
svuint16x4_t svread_ver_za16_u16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_bf16_vg4)))
svbfloat16x4_t svread_ver_za16_bf16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_f16_vg4)))
svfloat16x4_t svread_ver_za16_f16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za16_s16_vg4)))
svint16x4_t svread_ver_za16_s16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_u32_vg2)))
svuint32x2_t svread_ver_za32_u32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_f32_vg2)))
svfloat32x2_t svread_ver_za32_f32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_s32_vg2)))
svint32x2_t svread_ver_za32_s32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_u32_vg4)))
svuint32x4_t svread_ver_za32_u32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_f32_vg4)))
svfloat32x4_t svread_ver_za32_f32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za32_s32_vg4)))
svint32x4_t svread_ver_za32_s32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_u64_vg2)))
svuint64x2_t svread_ver_za64_u64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_f64_vg2)))
svfloat64x2_t svread_ver_za64_f64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_s64_vg2)))
svint64x2_t svread_ver_za64_s64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_u64_vg4)))
svuint64x4_t svread_ver_za64_u64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_f64_vg4)))
svfloat64x4_t svread_ver_za64_f64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za64_s64_vg4)))
svint64x4_t svread_ver_za64_s64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_u8_vg2)))
svuint8x2_t svread_ver_za8_u8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_s8_vg2)))
svint8x2_t svread_ver_za8_s8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_mf8_vg2)))
svmfloat8x2_t svread_ver_za8_mf8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_u8_vg4)))
svuint8x4_t svread_ver_za8_u8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_s8_vg4)))
svint8x4_t svread_ver_za8_s8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_ver_za8_mf8_vg4)))
svmfloat8x4_t svread_ver_za8_mf8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za16_u16_vg1x2)))
svuint16x2_t svread_za16_u16_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za16_bf16_vg1x2)))
svbfloat16x2_t svread_za16_bf16_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za16_f16_vg1x2)))
svfloat16x2_t svread_za16_f16_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za16_s16_vg1x2)))
svint16x2_t svread_za16_s16_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za16_u16_vg1x4)))
svuint16x4_t svread_za16_u16_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za16_bf16_vg1x4)))
svbfloat16x4_t svread_za16_bf16_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za16_f16_vg1x4)))
svfloat16x4_t svread_za16_f16_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za16_s16_vg1x4)))
svint16x4_t svread_za16_s16_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za32_u32_vg1x2)))
svuint32x2_t svread_za32_u32_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za32_f32_vg1x2)))
svfloat32x2_t svread_za32_f32_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za32_s32_vg1x2)))
svint32x2_t svread_za32_s32_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za32_u32_vg1x4)))
svuint32x4_t svread_za32_u32_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za32_f32_vg1x4)))
svfloat32x4_t svread_za32_f32_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za32_s32_vg1x4)))
svint32x4_t svread_za32_s32_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za64_u64_vg1x2)))
svuint64x2_t svread_za64_u64_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za64_f64_vg1x2)))
svfloat64x2_t svread_za64_f64_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za64_s64_vg1x2)))
svint64x2_t svread_za64_s64_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za64_u64_vg1x4)))
svuint64x4_t svread_za64_u64_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za64_f64_vg1x4)))
svfloat64x4_t svread_za64_f64_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za64_s64_vg1x4)))
svint64x4_t svread_za64_s64_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za8_u8_vg1x2)))
svuint8x2_t svread_za8_u8_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za8_s8_vg1x2)))
svint8x2_t svread_za8_s8_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za8_mf8_vg1x2)))
svmfloat8x2_t svread_za8_mf8_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za8_u8_vg1x4)))
svuint8x4_t svread_za8_u8_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za8_s8_vg1x4)))
svint8x4_t svread_za8_s8_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svread_za8_mf8_vg1x4)))
svmfloat8x4_t svread_za8_mf8_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svstr_zt)))
void svstr_zt(uint64_t, void *);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za32_u32_vg1x2)))
void svsub_write_single_za32_u32_vg1x2(uint32_t, svuint32x2_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za32_s32_vg1x2)))
void svsub_write_single_za32_s32_vg1x2(uint32_t, svint32x2_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za32_u32_vg1x4)))
void svsub_write_single_za32_u32_vg1x4(uint32_t, svuint32x4_t, svuint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za32_s32_vg1x4)))
void svsub_write_single_za32_s32_vg1x4(uint32_t, svint32x4_t, svint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za32_u32_vg1x2)))
void svsub_write_za32_u32_vg1x2(uint32_t, svuint32x2_t, svuint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za32_s32_vg1x2)))
void svsub_write_za32_s32_vg1x2(uint32_t, svint32x2_t, svint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za32_u32_vg1x4)))
void svsub_write_za32_u32_vg1x4(uint32_t, svuint32x4_t, svuint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za32_s32_vg1x4)))
void svsub_write_za32_s32_vg1x4(uint32_t, svint32x4_t, svint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_u32_vg1x2)))
void svsub_za32_u32_vg1x2(uint32_t, svuint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_f32_vg1x2)))
void svsub_za32_f32_vg1x2(uint32_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_s32_vg1x2)))
void svsub_za32_s32_vg1x2(uint32_t, svint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_u32_vg1x4)))
void svsub_za32_u32_vg1x4(uint32_t, svuint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_f32_vg1x4)))
void svsub_za32_f32_vg1x4(uint32_t, svfloat32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_s32_vg1x4)))
void svsub_za32_s32_vg1x4(uint32_t, svint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_single_za32_s8_vg1x2)))
void svsudot_single_za32_s8_vg1x2(uint32_t, svint8x2_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_single_za32_s8_vg1x4)))
void svsudot_single_za32_s8_vg1x4(uint32_t, svint8x4_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_lane_za32_s8_vg1x2)))
void svsudot_lane_za32_s8_vg1x2(uint32_t, svint8x2_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_lane_za32_s8_vg1x4)))
void svsudot_lane_za32_s8_vg1x4(uint32_t, svint8x4_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_za32_s8_vg1x2)))
void svsudot_za32_s8_vg1x2(uint32_t, svint8x2_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_za32_s8_vg1x4)))
void svsudot_za32_s8_vg1x4(uint32_t, svint8x4_t, svuint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_single_za32_s8_vg4x2)))
void svsumla_single_za32_s8_vg4x2(uint32_t, svint8x2_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_single_za32_s8_vg4x4)))
void svsumla_single_za32_s8_vg4x4(uint32_t, svint8x4_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_lane_za32_s8_vg4x1)))
void svsumla_lane_za32_s8_vg4x1(uint32_t, svint8_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_lane_za32_s8_vg4x2)))
void svsumla_lane_za32_s8_vg4x2(uint32_t, svint8x2_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_lane_za32_s8_vg4x4)))
void svsumla_lane_za32_s8_vg4x4(uint32_t, svint8x4_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_za32_s8_vg4x1)))
void svsumla_za32_s8_vg4x1(uint32_t, svint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_za32_s8_vg4x2)))
void svsumla_za32_s8_vg4x2(uint32_t, svint8x2_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_za32_s8_vg4x4)))
void svsumla_za32_s8_vg4x4(uint32_t, svint8x4_t, svuint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsuvdot_lane_za32_s8_vg1x4)))
void svsuvdot_lane_za32_s8_vg1x4(uint32_t, svint8x4_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_single_za32_u8_vg1x2)))
void svusdot_single_za32_u8_vg1x2(uint32_t, svuint8x2_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_single_za32_u8_vg1x4)))
void svusdot_single_za32_u8_vg1x4(uint32_t, svuint8x4_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_lane_za32_u8_vg1x2)))
void svusdot_lane_za32_u8_vg1x2(uint32_t, svuint8x2_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_lane_za32_u8_vg1x4)))
void svusdot_lane_za32_u8_vg1x4(uint32_t, svuint8x4_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_za32_u8_vg1x2)))
void svusdot_za32_u8_vg1x2(uint32_t, svuint8x2_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_za32_u8_vg1x4)))
void svusdot_za32_u8_vg1x4(uint32_t, svuint8x4_t, svint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_single_za32_u8_vg4x2)))
void svusmla_single_za32_u8_vg4x2(uint32_t, svuint8x2_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_single_za32_u8_vg4x4)))
void svusmla_single_za32_u8_vg4x4(uint32_t, svuint8x4_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_lane_za32_u8_vg4x1)))
void svusmla_lane_za32_u8_vg4x1(uint32_t, svuint8_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_lane_za32_u8_vg4x2)))
void svusmla_lane_za32_u8_vg4x2(uint32_t, svuint8x2_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_lane_za32_u8_vg4x4)))
void svusmla_lane_za32_u8_vg4x4(uint32_t, svuint8x4_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_za32_u8_vg4x1)))
void svusmla_za32_u8_vg4x1(uint32_t, svuint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_za32_u8_vg4x2)))
void svusmla_za32_u8_vg4x2(uint32_t, svuint8x2_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_za32_u8_vg4x4)))
void svusmla_za32_u8_vg4x4(uint32_t, svuint8x4_t, svint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusvdot_lane_za32_u8_vg1x4)))
void svusvdot_lane_za32_u8_vg1x4(uint32_t, svuint8x4_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_bf16_vg1x2)))
void svvdot_lane_za32_bf16_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_f16_vg1x2)))
void svvdot_lane_za32_f16_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_s16_vg1x2)))
void svvdot_lane_za32_s16_vg1x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_u16_vg1x2)))
void svvdot_lane_za32_u16_vg1x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_s8_vg1x4)))
void svvdot_lane_za32_s8_vg1x4(uint32_t, svint8x4_t, svint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_u8_vg1x4)))
void svvdot_lane_za32_u8_vg1x4(uint32_t, svuint8x4_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_u16_vg2)))
void svwrite_hor_za16_u16_vg2(uint64_t, uint32_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_bf16_vg2)))
void svwrite_hor_za16_bf16_vg2(uint64_t, uint32_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_f16_vg2)))
void svwrite_hor_za16_f16_vg2(uint64_t, uint32_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_s16_vg2)))
void svwrite_hor_za16_s16_vg2(uint64_t, uint32_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_u16_vg4)))
void svwrite_hor_za16_u16_vg4(uint64_t, uint32_t, svuint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_bf16_vg4)))
void svwrite_hor_za16_bf16_vg4(uint64_t, uint32_t, svbfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_f16_vg4)))
void svwrite_hor_za16_f16_vg4(uint64_t, uint32_t, svfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_s16_vg4)))
void svwrite_hor_za16_s16_vg4(uint64_t, uint32_t, svint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_u32_vg2)))
void svwrite_hor_za32_u32_vg2(uint64_t, uint32_t, svuint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_f32_vg2)))
void svwrite_hor_za32_f32_vg2(uint64_t, uint32_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_s32_vg2)))
void svwrite_hor_za32_s32_vg2(uint64_t, uint32_t, svint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_u32_vg4)))
void svwrite_hor_za32_u32_vg4(uint64_t, uint32_t, svuint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_f32_vg4)))
void svwrite_hor_za32_f32_vg4(uint64_t, uint32_t, svfloat32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_s32_vg4)))
void svwrite_hor_za32_s32_vg4(uint64_t, uint32_t, svint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_u64_vg2)))
void svwrite_hor_za64_u64_vg2(uint64_t, uint32_t, svuint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_f64_vg2)))
void svwrite_hor_za64_f64_vg2(uint64_t, uint32_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_s64_vg2)))
void svwrite_hor_za64_s64_vg2(uint64_t, uint32_t, svint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_u64_vg4)))
void svwrite_hor_za64_u64_vg4(uint64_t, uint32_t, svuint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_f64_vg4)))
void svwrite_hor_za64_f64_vg4(uint64_t, uint32_t, svfloat64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_s64_vg4)))
void svwrite_hor_za64_s64_vg4(uint64_t, uint32_t, svint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_u8_vg2)))
void svwrite_hor_za8_u8_vg2(uint64_t, uint32_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_s8_vg2)))
void svwrite_hor_za8_s8_vg2(uint64_t, uint32_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_mf8_vg2)))
void svwrite_hor_za8_mf8_vg2(uint64_t, uint32_t, svmfloat8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_u8_vg4)))
void svwrite_hor_za8_u8_vg4(uint64_t, uint32_t, svuint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_s8_vg4)))
void svwrite_hor_za8_s8_vg4(uint64_t, uint32_t, svint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_mf8_vg4)))
void svwrite_hor_za8_mf8_vg4(uint64_t, uint32_t, svmfloat8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_u16_vg2)))
void svwrite_ver_za16_u16_vg2(uint64_t, uint32_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_bf16_vg2)))
void svwrite_ver_za16_bf16_vg2(uint64_t, uint32_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_f16_vg2)))
void svwrite_ver_za16_f16_vg2(uint64_t, uint32_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_s16_vg2)))
void svwrite_ver_za16_s16_vg2(uint64_t, uint32_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_u16_vg4)))
void svwrite_ver_za16_u16_vg4(uint64_t, uint32_t, svuint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_bf16_vg4)))
void svwrite_ver_za16_bf16_vg4(uint64_t, uint32_t, svbfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_f16_vg4)))
void svwrite_ver_za16_f16_vg4(uint64_t, uint32_t, svfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_s16_vg4)))
void svwrite_ver_za16_s16_vg4(uint64_t, uint32_t, svint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_u32_vg2)))
void svwrite_ver_za32_u32_vg2(uint64_t, uint32_t, svuint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_f32_vg2)))
void svwrite_ver_za32_f32_vg2(uint64_t, uint32_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_s32_vg2)))
void svwrite_ver_za32_s32_vg2(uint64_t, uint32_t, svint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_u32_vg4)))
void svwrite_ver_za32_u32_vg4(uint64_t, uint32_t, svuint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_f32_vg4)))
void svwrite_ver_za32_f32_vg4(uint64_t, uint32_t, svfloat32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_s32_vg4)))
void svwrite_ver_za32_s32_vg4(uint64_t, uint32_t, svint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_u64_vg2)))
void svwrite_ver_za64_u64_vg2(uint64_t, uint32_t, svuint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_f64_vg2)))
void svwrite_ver_za64_f64_vg2(uint64_t, uint32_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_s64_vg2)))
void svwrite_ver_za64_s64_vg2(uint64_t, uint32_t, svint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_u64_vg4)))
void svwrite_ver_za64_u64_vg4(uint64_t, uint32_t, svuint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_f64_vg4)))
void svwrite_ver_za64_f64_vg4(uint64_t, uint32_t, svfloat64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_s64_vg4)))
void svwrite_ver_za64_s64_vg4(uint64_t, uint32_t, svint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_u8_vg2)))
void svwrite_ver_za8_u8_vg2(uint64_t, uint32_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_s8_vg2)))
void svwrite_ver_za8_s8_vg2(uint64_t, uint32_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_mf8_vg2)))
void svwrite_ver_za8_mf8_vg2(uint64_t, uint32_t, svmfloat8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_u8_vg4)))
void svwrite_ver_za8_u8_vg4(uint64_t, uint32_t, svuint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_s8_vg4)))
void svwrite_ver_za8_s8_vg4(uint64_t, uint32_t, svint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_mf8_vg4)))
void svwrite_ver_za8_mf8_vg4(uint64_t, uint32_t, svmfloat8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_u16_vg1x2)))
void svwrite_za16_u16_vg1x2(uint32_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_bf16_vg1x2)))
void svwrite_za16_bf16_vg1x2(uint32_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_f16_vg1x2)))
void svwrite_za16_f16_vg1x2(uint32_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_s16_vg1x2)))
void svwrite_za16_s16_vg1x2(uint32_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_u16_vg1x4)))
void svwrite_za16_u16_vg1x4(uint32_t, svuint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_bf16_vg1x4)))
void svwrite_za16_bf16_vg1x4(uint32_t, svbfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_f16_vg1x4)))
void svwrite_za16_f16_vg1x4(uint32_t, svfloat16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_s16_vg1x4)))
void svwrite_za16_s16_vg1x4(uint32_t, svint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_u32_vg1x2)))
void svwrite_za32_u32_vg1x2(uint32_t, svuint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_f32_vg1x2)))
void svwrite_za32_f32_vg1x2(uint32_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_s32_vg1x2)))
void svwrite_za32_s32_vg1x2(uint32_t, svint32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_u32_vg1x4)))
void svwrite_za32_u32_vg1x4(uint32_t, svuint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_f32_vg1x4)))
void svwrite_za32_f32_vg1x4(uint32_t, svfloat32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_s32_vg1x4)))
void svwrite_za32_s32_vg1x4(uint32_t, svint32x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_u64_vg1x2)))
void svwrite_za64_u64_vg1x2(uint32_t, svuint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_f64_vg1x2)))
void svwrite_za64_f64_vg1x2(uint32_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_s64_vg1x2)))
void svwrite_za64_s64_vg1x2(uint32_t, svint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_u64_vg1x4)))
void svwrite_za64_u64_vg1x4(uint32_t, svuint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_f64_vg1x4)))
void svwrite_za64_f64_vg1x4(uint32_t, svfloat64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_s64_vg1x4)))
void svwrite_za64_s64_vg1x4(uint32_t, svint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_u8_vg1x2)))
void svwrite_za8_u8_vg1x2(uint32_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_s8_vg1x2)))
void svwrite_za8_s8_vg1x2(uint32_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_mf8_vg1x2)))
void svwrite_za8_mf8_vg1x2(uint32_t, svmfloat8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_u8_vg1x4)))
void svwrite_za8_u8_vg1x4(uint32_t, svuint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_s8_vg1x4)))
void svwrite_za8_s8_vg1x4(uint32_t, svint8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_mf8_vg1x4)))
void svwrite_za8_mf8_vg1x4(uint32_t, svmfloat8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_zt)))
void svzero_zt(uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za32_u32_vg1x2)))
void svadd_write_za32_vg1x2(uint32_t, svuint32x2_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za32_s32_vg1x2)))
void svadd_write_za32_vg1x2(uint32_t, svint32x2_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za32_u32_vg1x4)))
void svadd_write_za32_vg1x4(uint32_t, svuint32x4_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za32_s32_vg1x4)))
void svadd_write_za32_vg1x4(uint32_t, svint32x4_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za32_u32_vg1x2)))
void svadd_write_za32_vg1x2(uint32_t, svuint32x2_t, svuint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za32_s32_vg1x2)))
void svadd_write_za32_vg1x2(uint32_t, svint32x2_t, svint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za32_u32_vg1x4)))
void svadd_write_za32_vg1x4(uint32_t, svuint32x4_t, svuint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za32_s32_vg1x4)))
void svadd_write_za32_vg1x4(uint32_t, svint32x4_t, svint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_u32_vg1x2)))
void svadd_za32_vg1x2(uint32_t, svuint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_f32_vg1x2)))
void svadd_za32_vg1x2(uint32_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_s32_vg1x2)))
void svadd_za32_vg1x2(uint32_t, svint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_u32_vg1x4)))
void svadd_za32_vg1x4(uint32_t, svuint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_f32_vg1x4)))
void svadd_za32_vg1x4(uint32_t, svfloat32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za32_s32_vg1x4)))
void svadd_za32_vg1x4(uint32_t, svint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svbmopa_za32_u32_m)))
void svbmopa_za32_m(uint64_t, svbool_t, svbool_t, svuint32_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svbmopa_za32_s32_m)))
void svbmopa_za32_m(uint64_t, svbool_t, svbool_t, svint32_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svbmops_za32_u32_m)))
void svbmops_za32_m(uint64_t, svbool_t, svbool_t, svuint32_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svbmops_za32_s32_m)))
void svbmops_za32_m(uint64_t, svbool_t, svbool_t, svint32_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_bf16_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_f16_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_s8_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svint8x2_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_s16_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_u8_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svuint8x2_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_u16_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svuint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_bf16_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_f16_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_s8_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svint8x4_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_s16_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svint16x4_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_u8_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svuint8x4_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za32_u16_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svuint16x4_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_bf16_vg1x2)))
void svdot_lane_za32_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_f16_vg1x2)))
void svdot_lane_za32_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_s8_vg1x2)))
void svdot_lane_za32_vg1x2(uint32_t, svint8x2_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_s16_vg1x2)))
void svdot_lane_za32_vg1x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_u8_vg1x2)))
void svdot_lane_za32_vg1x2(uint32_t, svuint8x2_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_u16_vg1x2)))
void svdot_lane_za32_vg1x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_bf16_vg1x4)))
void svdot_lane_za32_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_f16_vg1x4)))
void svdot_lane_za32_vg1x4(uint32_t, svfloat16x4_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_s8_vg1x4)))
void svdot_lane_za32_vg1x4(uint32_t, svint8x4_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_s16_vg1x4)))
void svdot_lane_za32_vg1x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_u8_vg1x4)))
void svdot_lane_za32_vg1x4(uint32_t, svuint8x4_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za32_u16_vg1x4)))
void svdot_lane_za32_vg1x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_bf16_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_f16_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svfloat16x2_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_s8_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svint8x2_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_s16_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_u8_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svuint8x2_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_u16_vg1x2)))
void svdot_za32_vg1x2(uint32_t, svuint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_bf16_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svbfloat16x4_t, svbfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_f16_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svfloat16x4_t, svfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_s8_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svint8x4_t, svint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_s16_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svint16x4_t, svint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_u8_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svuint8x4_t, svuint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za32_u16_vg1x4)))
void svdot_za32_vg1x4(uint32_t, svuint16x4_t, svuint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_f32_vg1x2)))
void svmla_za32_vg1x2(uint32_t, svfloat32x2_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_f32_vg1x4)))
void svmla_za32_vg1x4(uint32_t, svfloat32x4_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_bf16_vg2x2)))
void svmla_za32_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_f16_vg2x2)))
void svmla_za32_vg2x2(uint32_t, svfloat16x2_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_s16_vg2x2)))
void svmla_za32_vg2x2(uint32_t, svint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_u16_vg2x2)))
void svmla_za32_vg2x2(uint32_t, svuint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_bf16_vg2x4)))
void svmla_za32_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_f16_vg2x4)))
void svmla_za32_vg2x4(uint32_t, svfloat16x4_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_s16_vg2x4)))
void svmla_za32_vg2x4(uint32_t, svint16x4_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_u16_vg2x4)))
void svmla_za32_vg2x4(uint32_t, svuint16x4_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_s8_vg4x2)))
void svmla_za32_vg4x2(uint32_t, svint8x2_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_u8_vg4x2)))
void svmla_za32_vg4x2(uint32_t, svuint8x2_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_s8_vg4x4)))
void svmla_za32_vg4x4(uint32_t, svint8x4_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za32_u8_vg4x4)))
void svmla_za32_vg4x4(uint32_t, svuint8x4_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_f32_vg1x2)))
void svmla_lane_za32_vg1x2(uint32_t, svfloat32x2_t, svfloat32_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_f32_vg1x4)))
void svmla_lane_za32_vg1x4(uint32_t, svfloat32x4_t, svfloat32_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_bf16_vg2x1)))
void svmla_lane_za32_vg2x1(uint32_t, svbfloat16_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_f16_vg2x1)))
void svmla_lane_za32_vg2x1(uint32_t, svfloat16_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s16_vg2x1)))
void svmla_lane_za32_vg2x1(uint32_t, svint16_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u16_vg2x1)))
void svmla_lane_za32_vg2x1(uint32_t, svuint16_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_bf16_vg2x2)))
void svmla_lane_za32_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_f16_vg2x2)))
void svmla_lane_za32_vg2x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s16_vg2x2)))
void svmla_lane_za32_vg2x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u16_vg2x2)))
void svmla_lane_za32_vg2x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_bf16_vg2x4)))
void svmla_lane_za32_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_f16_vg2x4)))
void svmla_lane_za32_vg2x4(uint32_t, svfloat16x4_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s16_vg2x4)))
void svmla_lane_za32_vg2x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u16_vg2x4)))
void svmla_lane_za32_vg2x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s8_vg4x1)))
void svmla_lane_za32_vg4x1(uint32_t, svint8_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u8_vg4x1)))
void svmla_lane_za32_vg4x1(uint32_t, svuint8_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s8_vg4x2)))
void svmla_lane_za32_vg4x2(uint32_t, svint8x2_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u8_vg4x2)))
void svmla_lane_za32_vg4x2(uint32_t, svuint8x2_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_s8_vg4x4)))
void svmla_lane_za32_vg4x4(uint32_t, svint8x4_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za32_u8_vg4x4)))
void svmla_lane_za32_vg4x4(uint32_t, svuint8x4_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_f32_vg1x2)))
void svmla_za32_vg1x2(uint32_t, svfloat32x2_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_f32_vg1x4)))
void svmla_za32_vg1x4(uint32_t, svfloat32x4_t, svfloat32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_bf16_vg2x1)))
void svmla_za32_vg2x1(uint32_t, svbfloat16_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_f16_vg2x1)))
void svmla_za32_vg2x1(uint32_t, svfloat16_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s16_vg2x1)))
void svmla_za32_vg2x1(uint32_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u16_vg2x1)))
void svmla_za32_vg2x1(uint32_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_bf16_vg2x2)))
void svmla_za32_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_f16_vg2x2)))
void svmla_za32_vg2x2(uint32_t, svfloat16x2_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s16_vg2x2)))
void svmla_za32_vg2x2(uint32_t, svint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u16_vg2x2)))
void svmla_za32_vg2x2(uint32_t, svuint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_bf16_vg2x4)))
void svmla_za32_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_f16_vg2x4)))
void svmla_za32_vg2x4(uint32_t, svfloat16x4_t, svfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s16_vg2x4)))
void svmla_za32_vg2x4(uint32_t, svint16x4_t, svint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u16_vg2x4)))
void svmla_za32_vg2x4(uint32_t, svuint16x4_t, svuint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s8_vg4x1)))
void svmla_za32_vg4x1(uint32_t, svint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u8_vg4x1)))
void svmla_za32_vg4x1(uint32_t, svuint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s8_vg4x2)))
void svmla_za32_vg4x2(uint32_t, svint8x2_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u8_vg4x2)))
void svmla_za32_vg4x2(uint32_t, svuint8x2_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_s8_vg4x4)))
void svmla_za32_vg4x4(uint32_t, svint8x4_t, svint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za32_u8_vg4x4)))
void svmla_za32_vg4x4(uint32_t, svuint8x4_t, svuint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_f32_vg1x2)))
void svmls_za32_vg1x2(uint32_t, svfloat32x2_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_f32_vg1x4)))
void svmls_za32_vg1x4(uint32_t, svfloat32x4_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_bf16_vg2x2)))
void svmls_za32_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_f16_vg2x2)))
void svmls_za32_vg2x2(uint32_t, svfloat16x2_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_s16_vg2x2)))
void svmls_za32_vg2x2(uint32_t, svint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_u16_vg2x2)))
void svmls_za32_vg2x2(uint32_t, svuint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_bf16_vg2x4)))
void svmls_za32_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_f16_vg2x4)))
void svmls_za32_vg2x4(uint32_t, svfloat16x4_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_s16_vg2x4)))
void svmls_za32_vg2x4(uint32_t, svint16x4_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_u16_vg2x4)))
void svmls_za32_vg2x4(uint32_t, svuint16x4_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_s8_vg4x2)))
void svmls_za32_vg4x2(uint32_t, svint8x2_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_u8_vg4x2)))
void svmls_za32_vg4x2(uint32_t, svuint8x2_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_s8_vg4x4)))
void svmls_za32_vg4x4(uint32_t, svint8x4_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za32_u8_vg4x4)))
void svmls_za32_vg4x4(uint32_t, svuint8x4_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_f32_vg1x2)))
void svmls_lane_za32_vg1x2(uint32_t, svfloat32x2_t, svfloat32_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_f32_vg1x4)))
void svmls_lane_za32_vg1x4(uint32_t, svfloat32x4_t, svfloat32_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_bf16_vg2x1)))
void svmls_lane_za32_vg2x1(uint32_t, svbfloat16_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_f16_vg2x1)))
void svmls_lane_za32_vg2x1(uint32_t, svfloat16_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s16_vg2x1)))
void svmls_lane_za32_vg2x1(uint32_t, svint16_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u16_vg2x1)))
void svmls_lane_za32_vg2x1(uint32_t, svuint16_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_bf16_vg2x2)))
void svmls_lane_za32_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_f16_vg2x2)))
void svmls_lane_za32_vg2x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s16_vg2x2)))
void svmls_lane_za32_vg2x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u16_vg2x2)))
void svmls_lane_za32_vg2x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_bf16_vg2x4)))
void svmls_lane_za32_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_f16_vg2x4)))
void svmls_lane_za32_vg2x4(uint32_t, svfloat16x4_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s16_vg2x4)))
void svmls_lane_za32_vg2x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u16_vg2x4)))
void svmls_lane_za32_vg2x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s8_vg4x1)))
void svmls_lane_za32_vg4x1(uint32_t, svint8_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u8_vg4x1)))
void svmls_lane_za32_vg4x1(uint32_t, svuint8_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s8_vg4x2)))
void svmls_lane_za32_vg4x2(uint32_t, svint8x2_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u8_vg4x2)))
void svmls_lane_za32_vg4x2(uint32_t, svuint8x2_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_s8_vg4x4)))
void svmls_lane_za32_vg4x4(uint32_t, svint8x4_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za32_u8_vg4x4)))
void svmls_lane_za32_vg4x4(uint32_t, svuint8x4_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_f32_vg1x2)))
void svmls_za32_vg1x2(uint32_t, svfloat32x2_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_f32_vg1x4)))
void svmls_za32_vg1x4(uint32_t, svfloat32x4_t, svfloat32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_bf16_vg2x1)))
void svmls_za32_vg2x1(uint32_t, svbfloat16_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_f16_vg2x1)))
void svmls_za32_vg2x1(uint32_t, svfloat16_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s16_vg2x1)))
void svmls_za32_vg2x1(uint32_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u16_vg2x1)))
void svmls_za32_vg2x1(uint32_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_bf16_vg2x2)))
void svmls_za32_vg2x2(uint32_t, svbfloat16x2_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_f16_vg2x2)))
void svmls_za32_vg2x2(uint32_t, svfloat16x2_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s16_vg2x2)))
void svmls_za32_vg2x2(uint32_t, svint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u16_vg2x2)))
void svmls_za32_vg2x2(uint32_t, svuint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_bf16_vg2x4)))
void svmls_za32_vg2x4(uint32_t, svbfloat16x4_t, svbfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_f16_vg2x4)))
void svmls_za32_vg2x4(uint32_t, svfloat16x4_t, svfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s16_vg2x4)))
void svmls_za32_vg2x4(uint32_t, svint16x4_t, svint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u16_vg2x4)))
void svmls_za32_vg2x4(uint32_t, svuint16x4_t, svuint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s8_vg4x1)))
void svmls_za32_vg4x1(uint32_t, svint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u8_vg4x1)))
void svmls_za32_vg4x1(uint32_t, svuint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s8_vg4x2)))
void svmls_za32_vg4x2(uint32_t, svint8x2_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u8_vg4x2)))
void svmls_za32_vg4x2(uint32_t, svuint8x2_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_s8_vg4x4)))
void svmls_za32_vg4x4(uint32_t, svint8x4_t, svint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za32_u8_vg4x4)))
void svmls_za32_vg4x4(uint32_t, svuint8x4_t, svuint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_s16_m)))
void svmopa_za32_m(uint64_t, svbool_t, svbool_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmopa_za32_u16_m)))
void svmopa_za32_m(uint64_t, svbool_t, svbool_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_s16_m)))
void svmops_za32_m(uint64_t, svbool_t, svbool_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmops_za32_u16_m)))
void svmops_za32_m(uint64_t, svbool_t, svbool_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za32_u32_vg1x2)))
void svsub_write_za32_vg1x2(uint32_t, svuint32x2_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za32_s32_vg1x2)))
void svsub_write_za32_vg1x2(uint32_t, svint32x2_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za32_u32_vg1x4)))
void svsub_write_za32_vg1x4(uint32_t, svuint32x4_t, svuint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za32_s32_vg1x4)))
void svsub_write_za32_vg1x4(uint32_t, svint32x4_t, svint32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za32_u32_vg1x2)))
void svsub_write_za32_vg1x2(uint32_t, svuint32x2_t, svuint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za32_s32_vg1x2)))
void svsub_write_za32_vg1x2(uint32_t, svint32x2_t, svint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za32_u32_vg1x4)))
void svsub_write_za32_vg1x4(uint32_t, svuint32x4_t, svuint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za32_s32_vg1x4)))
void svsub_write_za32_vg1x4(uint32_t, svint32x4_t, svint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_u32_vg1x2)))
void svsub_za32_vg1x2(uint32_t, svuint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_f32_vg1x2)))
void svsub_za32_vg1x2(uint32_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_s32_vg1x2)))
void svsub_za32_vg1x2(uint32_t, svint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_u32_vg1x4)))
void svsub_za32_vg1x4(uint32_t, svuint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_f32_vg1x4)))
void svsub_za32_vg1x4(uint32_t, svfloat32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za32_s32_vg1x4)))
void svsub_za32_vg1x4(uint32_t, svint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_single_za32_s8_vg1x2)))
void svsudot_za32_vg1x2(uint32_t, svint8x2_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_single_za32_s8_vg1x4)))
void svsudot_za32_vg1x4(uint32_t, svint8x4_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_lane_za32_s8_vg1x2)))
void svsudot_lane_za32_vg1x2(uint32_t, svint8x2_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_lane_za32_s8_vg1x4)))
void svsudot_lane_za32_vg1x4(uint32_t, svint8x4_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_za32_s8_vg1x2)))
void svsudot_za32_vg1x2(uint32_t, svint8x2_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsudot_za32_s8_vg1x4)))
void svsudot_za32_vg1x4(uint32_t, svint8x4_t, svuint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_single_za32_s8_vg4x2)))
void svsumla_za32_vg4x2(uint32_t, svint8x2_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_single_za32_s8_vg4x4)))
void svsumla_za32_vg4x4(uint32_t, svint8x4_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_lane_za32_s8_vg4x1)))
void svsumla_lane_za32_vg4x1(uint32_t, svint8_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_lane_za32_s8_vg4x2)))
void svsumla_lane_za32_vg4x2(uint32_t, svint8x2_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_lane_za32_s8_vg4x4)))
void svsumla_lane_za32_vg4x4(uint32_t, svint8x4_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_za32_s8_vg4x1)))
void svsumla_za32_vg4x1(uint32_t, svint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_za32_s8_vg4x2)))
void svsumla_za32_vg4x2(uint32_t, svint8x2_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsumla_za32_s8_vg4x4)))
void svsumla_za32_vg4x4(uint32_t, svint8x4_t, svuint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsuvdot_lane_za32_s8_vg1x4)))
void svsuvdot_lane_za32_vg1x4(uint32_t, svint8x4_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_single_za32_u8_vg1x2)))
void svusdot_za32_vg1x2(uint32_t, svuint8x2_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_single_za32_u8_vg1x4)))
void svusdot_za32_vg1x4(uint32_t, svuint8x4_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_lane_za32_u8_vg1x2)))
void svusdot_lane_za32_vg1x2(uint32_t, svuint8x2_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_lane_za32_u8_vg1x4)))
void svusdot_lane_za32_vg1x4(uint32_t, svuint8x4_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_za32_u8_vg1x2)))
void svusdot_za32_vg1x2(uint32_t, svuint8x2_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusdot_za32_u8_vg1x4)))
void svusdot_za32_vg1x4(uint32_t, svuint8x4_t, svint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_single_za32_u8_vg4x2)))
void svusmla_za32_vg4x2(uint32_t, svuint8x2_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_single_za32_u8_vg4x4)))
void svusmla_za32_vg4x4(uint32_t, svuint8x4_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_lane_za32_u8_vg4x1)))
void svusmla_lane_za32_vg4x1(uint32_t, svuint8_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_lane_za32_u8_vg4x2)))
void svusmla_lane_za32_vg4x2(uint32_t, svuint8x2_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_lane_za32_u8_vg4x4)))
void svusmla_lane_za32_vg4x4(uint32_t, svuint8x4_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_za32_u8_vg4x1)))
void svusmla_za32_vg4x1(uint32_t, svuint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_za32_u8_vg4x2)))
void svusmla_za32_vg4x2(uint32_t, svuint8x2_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusmla_za32_u8_vg4x4)))
void svusmla_za32_vg4x4(uint32_t, svuint8x4_t, svint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svusvdot_lane_za32_u8_vg1x4)))
void svusvdot_lane_za32_vg1x4(uint32_t, svuint8x4_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_bf16_vg1x2)))
void svvdot_lane_za32_vg1x2(uint32_t, svbfloat16x2_t, svbfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_f16_vg1x2)))
void svvdot_lane_za32_vg1x2(uint32_t, svfloat16x2_t, svfloat16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_s16_vg1x2)))
void svvdot_lane_za32_vg1x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_u16_vg1x2)))
void svvdot_lane_za32_vg1x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_s8_vg1x4)))
void svvdot_lane_za32_vg1x4(uint32_t, svint8x4_t, svint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za32_u8_vg1x4)))
void svvdot_lane_za32_vg1x4(uint32_t, svuint8x4_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_u16_vg2)))
void svwrite_hor_za16_vg2(uint64_t, uint32_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_bf16_vg2)))
void svwrite_hor_za16_vg2(uint64_t, uint32_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_f16_vg2)))
void svwrite_hor_za16_vg2(uint64_t, uint32_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_s16_vg2)))
void svwrite_hor_za16_vg2(uint64_t, uint32_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_u16_vg4)))
void svwrite_hor_za16_vg4(uint64_t, uint32_t, svuint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_bf16_vg4)))
void svwrite_hor_za16_vg4(uint64_t, uint32_t, svbfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_f16_vg4)))
void svwrite_hor_za16_vg4(uint64_t, uint32_t, svfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za16_s16_vg4)))
void svwrite_hor_za16_vg4(uint64_t, uint32_t, svint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_u32_vg2)))
void svwrite_hor_za32_vg2(uint64_t, uint32_t, svuint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_f32_vg2)))
void svwrite_hor_za32_vg2(uint64_t, uint32_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_s32_vg2)))
void svwrite_hor_za32_vg2(uint64_t, uint32_t, svint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_u32_vg4)))
void svwrite_hor_za32_vg4(uint64_t, uint32_t, svuint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_f32_vg4)))
void svwrite_hor_za32_vg4(uint64_t, uint32_t, svfloat32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za32_s32_vg4)))
void svwrite_hor_za32_vg4(uint64_t, uint32_t, svint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_u64_vg2)))
void svwrite_hor_za64_vg2(uint64_t, uint32_t, svuint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_f64_vg2)))
void svwrite_hor_za64_vg2(uint64_t, uint32_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_s64_vg2)))
void svwrite_hor_za64_vg2(uint64_t, uint32_t, svint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_u64_vg4)))
void svwrite_hor_za64_vg4(uint64_t, uint32_t, svuint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_f64_vg4)))
void svwrite_hor_za64_vg4(uint64_t, uint32_t, svfloat64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za64_s64_vg4)))
void svwrite_hor_za64_vg4(uint64_t, uint32_t, svint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_u8_vg2)))
void svwrite_hor_za8_vg2(uint64_t, uint32_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_s8_vg2)))
void svwrite_hor_za8_vg2(uint64_t, uint32_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_mf8_vg2)))
void svwrite_hor_za8_vg2(uint64_t, uint32_t, svmfloat8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_u8_vg4)))
void svwrite_hor_za8_vg4(uint64_t, uint32_t, svuint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_s8_vg4)))
void svwrite_hor_za8_vg4(uint64_t, uint32_t, svint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_hor_za8_mf8_vg4)))
void svwrite_hor_za8_vg4(uint64_t, uint32_t, svmfloat8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_u16_vg2)))
void svwrite_ver_za16_vg2(uint64_t, uint32_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_bf16_vg2)))
void svwrite_ver_za16_vg2(uint64_t, uint32_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_f16_vg2)))
void svwrite_ver_za16_vg2(uint64_t, uint32_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_s16_vg2)))
void svwrite_ver_za16_vg2(uint64_t, uint32_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_u16_vg4)))
void svwrite_ver_za16_vg4(uint64_t, uint32_t, svuint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_bf16_vg4)))
void svwrite_ver_za16_vg4(uint64_t, uint32_t, svbfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_f16_vg4)))
void svwrite_ver_za16_vg4(uint64_t, uint32_t, svfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za16_s16_vg4)))
void svwrite_ver_za16_vg4(uint64_t, uint32_t, svint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_u32_vg2)))
void svwrite_ver_za32_vg2(uint64_t, uint32_t, svuint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_f32_vg2)))
void svwrite_ver_za32_vg2(uint64_t, uint32_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_s32_vg2)))
void svwrite_ver_za32_vg2(uint64_t, uint32_t, svint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_u32_vg4)))
void svwrite_ver_za32_vg4(uint64_t, uint32_t, svuint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_f32_vg4)))
void svwrite_ver_za32_vg4(uint64_t, uint32_t, svfloat32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za32_s32_vg4)))
void svwrite_ver_za32_vg4(uint64_t, uint32_t, svint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_u64_vg2)))
void svwrite_ver_za64_vg2(uint64_t, uint32_t, svuint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_f64_vg2)))
void svwrite_ver_za64_vg2(uint64_t, uint32_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_s64_vg2)))
void svwrite_ver_za64_vg2(uint64_t, uint32_t, svint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_u64_vg4)))
void svwrite_ver_za64_vg4(uint64_t, uint32_t, svuint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_f64_vg4)))
void svwrite_ver_za64_vg4(uint64_t, uint32_t, svfloat64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za64_s64_vg4)))
void svwrite_ver_za64_vg4(uint64_t, uint32_t, svint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_u8_vg2)))
void svwrite_ver_za8_vg2(uint64_t, uint32_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_s8_vg2)))
void svwrite_ver_za8_vg2(uint64_t, uint32_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_mf8_vg2)))
void svwrite_ver_za8_vg2(uint64_t, uint32_t, svmfloat8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_u8_vg4)))
void svwrite_ver_za8_vg4(uint64_t, uint32_t, svuint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_s8_vg4)))
void svwrite_ver_za8_vg4(uint64_t, uint32_t, svint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_ver_za8_mf8_vg4)))
void svwrite_ver_za8_vg4(uint64_t, uint32_t, svmfloat8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_u16_vg1x2)))
void svwrite_za16_vg1x2(uint32_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_bf16_vg1x2)))
void svwrite_za16_vg1x2(uint32_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_f16_vg1x2)))
void svwrite_za16_vg1x2(uint32_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_s16_vg1x2)))
void svwrite_za16_vg1x2(uint32_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_u16_vg1x4)))
void svwrite_za16_vg1x4(uint32_t, svuint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_bf16_vg1x4)))
void svwrite_za16_vg1x4(uint32_t, svbfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_f16_vg1x4)))
void svwrite_za16_vg1x4(uint32_t, svfloat16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za16_s16_vg1x4)))
void svwrite_za16_vg1x4(uint32_t, svint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_u32_vg1x2)))
void svwrite_za32_vg1x2(uint32_t, svuint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_f32_vg1x2)))
void svwrite_za32_vg1x2(uint32_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_s32_vg1x2)))
void svwrite_za32_vg1x2(uint32_t, svint32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_u32_vg1x4)))
void svwrite_za32_vg1x4(uint32_t, svuint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_f32_vg1x4)))
void svwrite_za32_vg1x4(uint32_t, svfloat32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za32_s32_vg1x4)))
void svwrite_za32_vg1x4(uint32_t, svint32x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_u64_vg1x2)))
void svwrite_za64_vg1x2(uint32_t, svuint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_f64_vg1x2)))
void svwrite_za64_vg1x2(uint32_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_s64_vg1x2)))
void svwrite_za64_vg1x2(uint32_t, svint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_u64_vg1x4)))
void svwrite_za64_vg1x4(uint32_t, svuint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_f64_vg1x4)))
void svwrite_za64_vg1x4(uint32_t, svfloat64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za64_s64_vg1x4)))
void svwrite_za64_vg1x4(uint32_t, svint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_u8_vg1x2)))
void svwrite_za8_vg1x2(uint32_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_s8_vg1x2)))
void svwrite_za8_vg1x2(uint32_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_mf8_vg1x2)))
void svwrite_za8_vg1x2(uint32_t, svmfloat8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_u8_vg1x4)))
void svwrite_za8_vg1x4(uint32_t, svuint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_s8_vg1x4)))
void svwrite_za8_vg1x4(uint32_t, svint8x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svwrite_za8_mf8_vg1x4)))
void svwrite_za8_vg1x4(uint32_t, svmfloat8x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_f64_vg1x2)))
void svadd_za64_f64_vg1x2(uint32_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_f64_vg1x4)))
void svadd_za64_f64_vg1x4(uint32_t, svfloat64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_f64_vg1x2)))
void svmla_single_za64_f64_vg1x2(uint32_t, svfloat64x2_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_f64_vg1x4)))
void svmla_single_za64_f64_vg1x4(uint32_t, svfloat64x4_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_f64_vg1x2)))
void svmla_lane_za64_f64_vg1x2(uint32_t, svfloat64x2_t, svfloat64_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_f64_vg1x4)))
void svmla_lane_za64_f64_vg1x4(uint32_t, svfloat64x4_t, svfloat64_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_f64_vg1x2)))
void svmla_za64_f64_vg1x2(uint32_t, svfloat64x2_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_f64_vg1x4)))
void svmla_za64_f64_vg1x4(uint32_t, svfloat64x4_t, svfloat64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_f64_vg1x2)))
void svmls_single_za64_f64_vg1x2(uint32_t, svfloat64x2_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_f64_vg1x4)))
void svmls_single_za64_f64_vg1x4(uint32_t, svfloat64x4_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_f64_vg1x2)))
void svmls_lane_za64_f64_vg1x2(uint32_t, svfloat64x2_t, svfloat64_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_f64_vg1x4)))
void svmls_lane_za64_f64_vg1x4(uint32_t, svfloat64x4_t, svfloat64_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_f64_vg1x2)))
void svmls_za64_f64_vg1x2(uint32_t, svfloat64x2_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_f64_vg1x4)))
void svmls_za64_f64_vg1x4(uint32_t, svfloat64x4_t, svfloat64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_f64_vg1x2)))
void svsub_za64_f64_vg1x2(uint32_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_f64_vg1x4)))
void svsub_za64_f64_vg1x4(uint32_t, svfloat64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_f64_vg1x2)))
void svadd_za64_vg1x2(uint32_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_f64_vg1x4)))
void svadd_za64_vg1x4(uint32_t, svfloat64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_f64_vg1x2)))
void svmla_za64_vg1x2(uint32_t, svfloat64x2_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_f64_vg1x4)))
void svmla_za64_vg1x4(uint32_t, svfloat64x4_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_f64_vg1x2)))
void svmla_lane_za64_vg1x2(uint32_t, svfloat64x2_t, svfloat64_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_f64_vg1x4)))
void svmla_lane_za64_vg1x4(uint32_t, svfloat64x4_t, svfloat64_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_f64_vg1x2)))
void svmla_za64_vg1x2(uint32_t, svfloat64x2_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_f64_vg1x4)))
void svmla_za64_vg1x4(uint32_t, svfloat64x4_t, svfloat64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_f64_vg1x2)))
void svmls_za64_vg1x2(uint32_t, svfloat64x2_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_f64_vg1x4)))
void svmls_za64_vg1x4(uint32_t, svfloat64x4_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_f64_vg1x2)))
void svmls_lane_za64_vg1x2(uint32_t, svfloat64x2_t, svfloat64_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_f64_vg1x4)))
void svmls_lane_za64_vg1x4(uint32_t, svfloat64x4_t, svfloat64_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_f64_vg1x2)))
void svmls_za64_vg1x2(uint32_t, svfloat64x2_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_f64_vg1x4)))
void svmls_za64_vg1x4(uint32_t, svfloat64x4_t, svfloat64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_f64_vg1x2)))
void svsub_za64_vg1x2(uint32_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_f64_vg1x4)))
void svsub_za64_vg1x4(uint32_t, svfloat64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za64_u64_vg1x2)))
void svadd_write_single_za64_u64_vg1x2(uint32_t, svuint64x2_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za64_s64_vg1x2)))
void svadd_write_single_za64_s64_vg1x2(uint32_t, svint64x2_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za64_u64_vg1x4)))
void svadd_write_single_za64_u64_vg1x4(uint32_t, svuint64x4_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za64_s64_vg1x4)))
void svadd_write_single_za64_s64_vg1x4(uint32_t, svint64x4_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za64_u64_vg1x2)))
void svadd_write_za64_u64_vg1x2(uint32_t, svuint64x2_t, svuint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za64_s64_vg1x2)))
void svadd_write_za64_s64_vg1x2(uint32_t, svint64x2_t, svint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za64_u64_vg1x4)))
void svadd_write_za64_u64_vg1x4(uint32_t, svuint64x4_t, svuint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za64_s64_vg1x4)))
void svadd_write_za64_s64_vg1x4(uint32_t, svint64x4_t, svint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_u64_vg1x2)))
void svadd_za64_u64_vg1x2(uint32_t, svuint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_s64_vg1x2)))
void svadd_za64_s64_vg1x2(uint32_t, svint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_u64_vg1x4)))
void svadd_za64_u64_vg1x4(uint32_t, svuint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_s64_vg1x4)))
void svadd_za64_s64_vg1x4(uint32_t, svint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za64_s16_vg1x2)))
void svdot_single_za64_s16_vg1x2(uint32_t, svint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za64_u16_vg1x2)))
void svdot_single_za64_u16_vg1x2(uint32_t, svuint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za64_s16_vg1x4)))
void svdot_single_za64_s16_vg1x4(uint32_t, svint16x4_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za64_u16_vg1x4)))
void svdot_single_za64_u16_vg1x4(uint32_t, svuint16x4_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za64_s16_vg1x2)))
void svdot_lane_za64_s16_vg1x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za64_u16_vg1x2)))
void svdot_lane_za64_u16_vg1x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za64_s16_vg1x4)))
void svdot_lane_za64_s16_vg1x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za64_u16_vg1x4)))
void svdot_lane_za64_u16_vg1x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za64_s16_vg1x2)))
void svdot_za64_s16_vg1x2(uint32_t, svint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za64_u16_vg1x2)))
void svdot_za64_u16_vg1x2(uint32_t, svuint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za64_s16_vg1x4)))
void svdot_za64_s16_vg1x4(uint32_t, svint16x4_t, svint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za64_u16_vg1x4)))
void svdot_za64_u16_vg1x4(uint32_t, svuint16x4_t, svuint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_s16_vg4x2)))
void svmla_single_za64_s16_vg4x2(uint32_t, svint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_u16_vg4x2)))
void svmla_single_za64_u16_vg4x2(uint32_t, svuint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_s16_vg4x4)))
void svmla_single_za64_s16_vg4x4(uint32_t, svint16x4_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_u16_vg4x4)))
void svmla_single_za64_u16_vg4x4(uint32_t, svuint16x4_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_s16_vg4x1)))
void svmla_lane_za64_s16_vg4x1(uint32_t, svint16_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_u16_vg4x1)))
void svmla_lane_za64_u16_vg4x1(uint32_t, svuint16_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_s16_vg4x2)))
void svmla_lane_za64_s16_vg4x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_u16_vg4x2)))
void svmla_lane_za64_u16_vg4x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_s16_vg4x4)))
void svmla_lane_za64_s16_vg4x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_u16_vg4x4)))
void svmla_lane_za64_u16_vg4x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_s16_vg4x1)))
void svmla_za64_s16_vg4x1(uint32_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_u16_vg4x1)))
void svmla_za64_u16_vg4x1(uint32_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_s16_vg4x2)))
void svmla_za64_s16_vg4x2(uint32_t, svint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_u16_vg4x2)))
void svmla_za64_u16_vg4x2(uint32_t, svuint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_s16_vg4x4)))
void svmla_za64_s16_vg4x4(uint32_t, svint16x4_t, svint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_u16_vg4x4)))
void svmla_za64_u16_vg4x4(uint32_t, svuint16x4_t, svuint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_s16_vg4x2)))
void svmls_single_za64_s16_vg4x2(uint32_t, svint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_u16_vg4x2)))
void svmls_single_za64_u16_vg4x2(uint32_t, svuint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_s16_vg4x4)))
void svmls_single_za64_s16_vg4x4(uint32_t, svint16x4_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_u16_vg4x4)))
void svmls_single_za64_u16_vg4x4(uint32_t, svuint16x4_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_s16_vg4x1)))
void svmls_lane_za64_s16_vg4x1(uint32_t, svint16_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_u16_vg4x1)))
void svmls_lane_za64_u16_vg4x1(uint32_t, svuint16_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_s16_vg4x2)))
void svmls_lane_za64_s16_vg4x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_u16_vg4x2)))
void svmls_lane_za64_u16_vg4x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_s16_vg4x4)))
void svmls_lane_za64_s16_vg4x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_u16_vg4x4)))
void svmls_lane_za64_u16_vg4x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_s16_vg4x1)))
void svmls_za64_s16_vg4x1(uint32_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_u16_vg4x1)))
void svmls_za64_u16_vg4x1(uint32_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_s16_vg4x2)))
void svmls_za64_s16_vg4x2(uint32_t, svint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_u16_vg4x2)))
void svmls_za64_u16_vg4x2(uint32_t, svuint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_s16_vg4x4)))
void svmls_za64_s16_vg4x4(uint32_t, svint16x4_t, svint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_u16_vg4x4)))
void svmls_za64_u16_vg4x4(uint32_t, svuint16x4_t, svuint16x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za64_u64_vg1x2)))
void svsub_write_single_za64_u64_vg1x2(uint32_t, svuint64x2_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za64_s64_vg1x2)))
void svsub_write_single_za64_s64_vg1x2(uint32_t, svint64x2_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za64_u64_vg1x4)))
void svsub_write_single_za64_u64_vg1x4(uint32_t, svuint64x4_t, svuint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za64_s64_vg1x4)))
void svsub_write_single_za64_s64_vg1x4(uint32_t, svint64x4_t, svint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za64_u64_vg1x2)))
void svsub_write_za64_u64_vg1x2(uint32_t, svuint64x2_t, svuint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za64_s64_vg1x2)))
void svsub_write_za64_s64_vg1x2(uint32_t, svint64x2_t, svint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za64_u64_vg1x4)))
void svsub_write_za64_u64_vg1x4(uint32_t, svuint64x4_t, svuint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za64_s64_vg1x4)))
void svsub_write_za64_s64_vg1x4(uint32_t, svint64x4_t, svint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_u64_vg1x2)))
void svsub_za64_u64_vg1x2(uint32_t, svuint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_s64_vg1x2)))
void svsub_za64_s64_vg1x2(uint32_t, svint64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_u64_vg1x4)))
void svsub_za64_u64_vg1x4(uint32_t, svuint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_s64_vg1x4)))
void svsub_za64_s64_vg1x4(uint32_t, svint64x4_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za64_s16_vg1x4)))
void svvdot_lane_za64_s16_vg1x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za64_u16_vg1x4)))
void svvdot_lane_za64_u16_vg1x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za64_u64_vg1x2)))
void svadd_write_za64_vg1x2(uint32_t, svuint64x2_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za64_s64_vg1x2)))
void svadd_write_za64_vg1x2(uint32_t, svint64x2_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za64_u64_vg1x4)))
void svadd_write_za64_vg1x4(uint32_t, svuint64x4_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_single_za64_s64_vg1x4)))
void svadd_write_za64_vg1x4(uint32_t, svint64x4_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za64_u64_vg1x2)))
void svadd_write_za64_vg1x2(uint32_t, svuint64x2_t, svuint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za64_s64_vg1x2)))
void svadd_write_za64_vg1x2(uint32_t, svint64x2_t, svint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za64_u64_vg1x4)))
void svadd_write_za64_vg1x4(uint32_t, svuint64x4_t, svuint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_write_za64_s64_vg1x4)))
void svadd_write_za64_vg1x4(uint32_t, svint64x4_t, svint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_u64_vg1x2)))
void svadd_za64_vg1x2(uint32_t, svuint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_s64_vg1x2)))
void svadd_za64_vg1x2(uint32_t, svint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_u64_vg1x4)))
void svadd_za64_vg1x4(uint32_t, svuint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svadd_za64_s64_vg1x4)))
void svadd_za64_vg1x4(uint32_t, svint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za64_s16_vg1x2)))
void svdot_za64_vg1x2(uint32_t, svint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za64_u16_vg1x2)))
void svdot_za64_vg1x2(uint32_t, svuint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za64_s16_vg1x4)))
void svdot_za64_vg1x4(uint32_t, svint16x4_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_single_za64_u16_vg1x4)))
void svdot_za64_vg1x4(uint32_t, svuint16x4_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za64_s16_vg1x2)))
void svdot_lane_za64_vg1x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za64_u16_vg1x2)))
void svdot_lane_za64_vg1x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za64_s16_vg1x4)))
void svdot_lane_za64_vg1x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_lane_za64_u16_vg1x4)))
void svdot_lane_za64_vg1x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za64_s16_vg1x2)))
void svdot_za64_vg1x2(uint32_t, svint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za64_u16_vg1x2)))
void svdot_za64_vg1x2(uint32_t, svuint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za64_s16_vg1x4)))
void svdot_za64_vg1x4(uint32_t, svint16x4_t, svint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svdot_za64_u16_vg1x4)))
void svdot_za64_vg1x4(uint32_t, svuint16x4_t, svuint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_s16_vg4x2)))
void svmla_za64_vg4x2(uint32_t, svint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_u16_vg4x2)))
void svmla_za64_vg4x2(uint32_t, svuint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_s16_vg4x4)))
void svmla_za64_vg4x4(uint32_t, svint16x4_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_single_za64_u16_vg4x4)))
void svmla_za64_vg4x4(uint32_t, svuint16x4_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_s16_vg4x1)))
void svmla_lane_za64_vg4x1(uint32_t, svint16_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_u16_vg4x1)))
void svmla_lane_za64_vg4x1(uint32_t, svuint16_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_s16_vg4x2)))
void svmla_lane_za64_vg4x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_u16_vg4x2)))
void svmla_lane_za64_vg4x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_s16_vg4x4)))
void svmla_lane_za64_vg4x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_lane_za64_u16_vg4x4)))
void svmla_lane_za64_vg4x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_s16_vg4x1)))
void svmla_za64_vg4x1(uint32_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_u16_vg4x1)))
void svmla_za64_vg4x1(uint32_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_s16_vg4x2)))
void svmla_za64_vg4x2(uint32_t, svint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_u16_vg4x2)))
void svmla_za64_vg4x2(uint32_t, svuint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_s16_vg4x4)))
void svmla_za64_vg4x4(uint32_t, svint16x4_t, svint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmla_za64_u16_vg4x4)))
void svmla_za64_vg4x4(uint32_t, svuint16x4_t, svuint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_s16_vg4x2)))
void svmls_za64_vg4x2(uint32_t, svint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_u16_vg4x2)))
void svmls_za64_vg4x2(uint32_t, svuint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_s16_vg4x4)))
void svmls_za64_vg4x4(uint32_t, svint16x4_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_single_za64_u16_vg4x4)))
void svmls_za64_vg4x4(uint32_t, svuint16x4_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_s16_vg4x1)))
void svmls_lane_za64_vg4x1(uint32_t, svint16_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_u16_vg4x1)))
void svmls_lane_za64_vg4x1(uint32_t, svuint16_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_s16_vg4x2)))
void svmls_lane_za64_vg4x2(uint32_t, svint16x2_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_u16_vg4x2)))
void svmls_lane_za64_vg4x2(uint32_t, svuint16x2_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_s16_vg4x4)))
void svmls_lane_za64_vg4x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_lane_za64_u16_vg4x4)))
void svmls_lane_za64_vg4x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_s16_vg4x1)))
void svmls_za64_vg4x1(uint32_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_u16_vg4x1)))
void svmls_za64_vg4x1(uint32_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_s16_vg4x2)))
void svmls_za64_vg4x2(uint32_t, svint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_u16_vg4x2)))
void svmls_za64_vg4x2(uint32_t, svuint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_s16_vg4x4)))
void svmls_za64_vg4x4(uint32_t, svint16x4_t, svint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmls_za64_u16_vg4x4)))
void svmls_za64_vg4x4(uint32_t, svuint16x4_t, svuint16x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za64_u64_vg1x2)))
void svsub_write_za64_vg1x2(uint32_t, svuint64x2_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za64_s64_vg1x2)))
void svsub_write_za64_vg1x2(uint32_t, svint64x2_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za64_u64_vg1x4)))
void svsub_write_za64_vg1x4(uint32_t, svuint64x4_t, svuint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_single_za64_s64_vg1x4)))
void svsub_write_za64_vg1x4(uint32_t, svint64x4_t, svint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za64_u64_vg1x2)))
void svsub_write_za64_vg1x2(uint32_t, svuint64x2_t, svuint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za64_s64_vg1x2)))
void svsub_write_za64_vg1x2(uint32_t, svint64x2_t, svint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za64_u64_vg1x4)))
void svsub_write_za64_vg1x4(uint32_t, svuint64x4_t, svuint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_write_za64_s64_vg1x4)))
void svsub_write_za64_vg1x4(uint32_t, svint64x4_t, svint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_u64_vg1x2)))
void svsub_za64_vg1x2(uint32_t, svuint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_s64_vg1x2)))
void svsub_za64_vg1x2(uint32_t, svint64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_u64_vg1x4)))
void svsub_za64_vg1x4(uint32_t, svuint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svsub_za64_s64_vg1x4)))
void svsub_za64_vg1x4(uint32_t, svint64x4_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za64_s16_vg1x4)))
void svvdot_lane_za64_vg1x4(uint32_t, svint16x4_t, svint16_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svvdot_lane_za64_u16_vg1x4)))
void svvdot_lane_za64_vg1x4(uint32_t, svuint16x4_t, svuint16_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_s8_u8)))
void svmop4a_1x1_za32_s8_u8(uint64_t, svint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_u8_s8)))
void svmop4a_1x1_za32_u8_s8(uint64_t, svuint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_bf16_bf16)))
void svmop4a_1x1_za32_bf16_bf16(uint64_t, svbfloat16_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_f16_f16)))
void svmop4a_1x1_za32_f16_f16(uint64_t, svfloat16_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_f32_f32)))
void svmop4a_1x1_za32_f32_f32(uint64_t, svfloat32_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_s8_s8)))
void svmop4a_1x1_za32_s8_s8(uint64_t, svint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_s16_s16)))
void svmop4a_1x1_za32_s16_s16(uint64_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_u8_u8)))
void svmop4a_1x1_za32_u8_u8(uint64_t, svuint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_u16_u16)))
void svmop4a_1x1_za32_u16_u16(uint64_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_s8_u8)))
void svmop4a_1x2_za32_s8_u8(uint64_t, svint8_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_u8_s8)))
void svmop4a_1x2_za32_u8_s8(uint64_t, svuint8_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_bf16_bf16)))
void svmop4a_1x2_za32_bf16_bf16(uint64_t, svbfloat16_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_f16_f16)))
void svmop4a_1x2_za32_f16_f16(uint64_t, svfloat16_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_f32_f32)))
void svmop4a_1x2_za32_f32_f32(uint64_t, svfloat32_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_s8_s8)))
void svmop4a_1x2_za32_s8_s8(uint64_t, svint8_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_s16_s16)))
void svmop4a_1x2_za32_s16_s16(uint64_t, svint16_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_u8_u8)))
void svmop4a_1x2_za32_u8_u8(uint64_t, svuint8_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_u16_u16)))
void svmop4a_1x2_za32_u16_u16(uint64_t, svuint16_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_s8_u8)))
void svmop4a_2x1_za32_s8_u8(uint64_t, svint8x2_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_u8_s8)))
void svmop4a_2x1_za32_u8_s8(uint64_t, svuint8x2_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_bf16_bf16)))
void svmop4a_2x1_za32_bf16_bf16(uint64_t, svbfloat16x2_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_f16_f16)))
void svmop4a_2x1_za32_f16_f16(uint64_t, svfloat16x2_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_f32_f32)))
void svmop4a_2x1_za32_f32_f32(uint64_t, svfloat32x2_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_s8_s8)))
void svmop4a_2x1_za32_s8_s8(uint64_t, svint8x2_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_s16_s16)))
void svmop4a_2x1_za32_s16_s16(uint64_t, svint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_u8_u8)))
void svmop4a_2x1_za32_u8_u8(uint64_t, svuint8x2_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_u16_u16)))
void svmop4a_2x1_za32_u16_u16(uint64_t, svuint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_s8_u8)))
void svmop4a_2x2_za32_s8_u8(uint64_t, svint8x2_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_u8_s8)))
void svmop4a_2x2_za32_u8_s8(uint64_t, svuint8x2_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_bf16_bf16)))
void svmop4a_2x2_za32_bf16_bf16(uint64_t, svbfloat16x2_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_f16_f16)))
void svmop4a_2x2_za32_f16_f16(uint64_t, svfloat16x2_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_f32_f32)))
void svmop4a_2x2_za32_f32_f32(uint64_t, svfloat32x2_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_s8_s8)))
void svmop4a_2x2_za32_s8_s8(uint64_t, svint8x2_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_s16_s16)))
void svmop4a_2x2_za32_s16_s16(uint64_t, svint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_u8_u8)))
void svmop4a_2x2_za32_u8_u8(uint64_t, svuint8x2_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_u16_u16)))
void svmop4a_2x2_za32_u16_u16(uint64_t, svuint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_s8_u8)))
void svmop4s_1x1_za32_s8_u8(uint64_t, svint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_u8_s8)))
void svmop4s_1x1_za32_u8_s8(uint64_t, svuint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_bf16_bf16)))
void svmop4s_1x1_za32_bf16_bf16(uint64_t, svbfloat16_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_f16_f16)))
void svmop4s_1x1_za32_f16_f16(uint64_t, svfloat16_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_f32_f32)))
void svmop4s_1x1_za32_f32_f32(uint64_t, svfloat32_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_s8_s8)))
void svmop4s_1x1_za32_s8_s8(uint64_t, svint8_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_s16_s16)))
void svmop4s_1x1_za32_s16_s16(uint64_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_u8_u8)))
void svmop4s_1x1_za32_u8_u8(uint64_t, svuint8_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_u16_u16)))
void svmop4s_1x1_za32_u16_u16(uint64_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_s8_u8)))
void svmop4s_1x2_za32_s8_u8(uint64_t, svint8_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_u8_s8)))
void svmop4s_1x2_za32_u8_s8(uint64_t, svuint8_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_bf16_bf16)))
void svmop4s_1x2_za32_bf16_bf16(uint64_t, svbfloat16_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_f16_f16)))
void svmop4s_1x2_za32_f16_f16(uint64_t, svfloat16_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_f32_f32)))
void svmop4s_1x2_za32_f32_f32(uint64_t, svfloat32_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_s8_s8)))
void svmop4s_1x2_za32_s8_s8(uint64_t, svint8_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_s16_s16)))
void svmop4s_1x2_za32_s16_s16(uint64_t, svint16_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_u8_u8)))
void svmop4s_1x2_za32_u8_u8(uint64_t, svuint8_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_u16_u16)))
void svmop4s_1x2_za32_u16_u16(uint64_t, svuint16_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_s8_u8)))
void svmop4s_2x1_za32_s8_u8(uint64_t, svint8x2_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_u8_s8)))
void svmop4s_2x1_za32_u8_s8(uint64_t, svuint8x2_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_bf16_bf16)))
void svmop4s_2x1_za32_bf16_bf16(uint64_t, svbfloat16x2_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_f16_f16)))
void svmop4s_2x1_za32_f16_f16(uint64_t, svfloat16x2_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_f32_f32)))
void svmop4s_2x1_za32_f32_f32(uint64_t, svfloat32x2_t, svfloat32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_s8_s8)))
void svmop4s_2x1_za32_s8_s8(uint64_t, svint8x2_t, svint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_s16_s16)))
void svmop4s_2x1_za32_s16_s16(uint64_t, svint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_u8_u8)))
void svmop4s_2x1_za32_u8_u8(uint64_t, svuint8x2_t, svuint8_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_u16_u16)))
void svmop4s_2x1_za32_u16_u16(uint64_t, svuint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_s8_u8)))
void svmop4s_2x2_za32_s8_u8(uint64_t, svint8x2_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_u8_s8)))
void svmop4s_2x2_za32_u8_s8(uint64_t, svuint8x2_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_bf16_bf16)))
void svmop4s_2x2_za32_bf16_bf16(uint64_t, svbfloat16x2_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_f16_f16)))
void svmop4s_2x2_za32_f16_f16(uint64_t, svfloat16x2_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_f32_f32)))
void svmop4s_2x2_za32_f32_f32(uint64_t, svfloat32x2_t, svfloat32x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_s8_s8)))
void svmop4s_2x2_za32_s8_s8(uint64_t, svint8x2_t, svint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_s16_s16)))
void svmop4s_2x2_za32_s16_s16(uint64_t, svint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_u8_u8)))
void svmop4s_2x2_za32_u8_u8(uint64_t, svuint8x2_t, svuint8x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_u16_u16)))
void svmop4s_2x2_za32_u16_u16(uint64_t, svuint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_s8_u8)))
void svmop4a_za32(uint64_t, svint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_u8_s8)))
void svmop4a_za32(uint64_t, svuint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_bf16_bf16)))
void svmop4a_za32(uint64_t, svbfloat16_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_f16_f16)))
void svmop4a_za32(uint64_t, svfloat16_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_f32_f32)))
void svmop4a_za32(uint64_t, svfloat32_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_s8_s8)))
void svmop4a_za32(uint64_t, svint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_s16_s16)))
void svmop4a_za32(uint64_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_u8_u8)))
void svmop4a_za32(uint64_t, svuint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za32_u16_u16)))
void svmop4a_za32(uint64_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_s8_u8)))
void svmop4a_za32(uint64_t, svint8_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_u8_s8)))
void svmop4a_za32(uint64_t, svuint8_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_bf16_bf16)))
void svmop4a_za32(uint64_t, svbfloat16_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_f16_f16)))
void svmop4a_za32(uint64_t, svfloat16_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_f32_f32)))
void svmop4a_za32(uint64_t, svfloat32_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_s8_s8)))
void svmop4a_za32(uint64_t, svint8_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_s16_s16)))
void svmop4a_za32(uint64_t, svint16_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_u8_u8)))
void svmop4a_za32(uint64_t, svuint8_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za32_u16_u16)))
void svmop4a_za32(uint64_t, svuint16_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_s8_u8)))
void svmop4a_za32(uint64_t, svint8x2_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_u8_s8)))
void svmop4a_za32(uint64_t, svuint8x2_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_bf16_bf16)))
void svmop4a_za32(uint64_t, svbfloat16x2_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_f16_f16)))
void svmop4a_za32(uint64_t, svfloat16x2_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_f32_f32)))
void svmop4a_za32(uint64_t, svfloat32x2_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_s8_s8)))
void svmop4a_za32(uint64_t, svint8x2_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_s16_s16)))
void svmop4a_za32(uint64_t, svint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_u8_u8)))
void svmop4a_za32(uint64_t, svuint8x2_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za32_u16_u16)))
void svmop4a_za32(uint64_t, svuint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_s8_u8)))
void svmop4a_za32(uint64_t, svint8x2_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_u8_s8)))
void svmop4a_za32(uint64_t, svuint8x2_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_bf16_bf16)))
void svmop4a_za32(uint64_t, svbfloat16x2_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_f16_f16)))
void svmop4a_za32(uint64_t, svfloat16x2_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_f32_f32)))
void svmop4a_za32(uint64_t, svfloat32x2_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_s8_s8)))
void svmop4a_za32(uint64_t, svint8x2_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_s16_s16)))
void svmop4a_za32(uint64_t, svint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_u8_u8)))
void svmop4a_za32(uint64_t, svuint8x2_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za32_u16_u16)))
void svmop4a_za32(uint64_t, svuint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_s8_u8)))
void svmop4s_za32(uint64_t, svint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_u8_s8)))
void svmop4s_za32(uint64_t, svuint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_bf16_bf16)))
void svmop4s_za32(uint64_t, svbfloat16_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_f16_f16)))
void svmop4s_za32(uint64_t, svfloat16_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_f32_f32)))
void svmop4s_za32(uint64_t, svfloat32_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_s8_s8)))
void svmop4s_za32(uint64_t, svint8_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_s16_s16)))
void svmop4s_za32(uint64_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_u8_u8)))
void svmop4s_za32(uint64_t, svuint8_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za32_u16_u16)))
void svmop4s_za32(uint64_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_s8_u8)))
void svmop4s_za32(uint64_t, svint8_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_u8_s8)))
void svmop4s_za32(uint64_t, svuint8_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_bf16_bf16)))
void svmop4s_za32(uint64_t, svbfloat16_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_f16_f16)))
void svmop4s_za32(uint64_t, svfloat16_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_f32_f32)))
void svmop4s_za32(uint64_t, svfloat32_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_s8_s8)))
void svmop4s_za32(uint64_t, svint8_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_s16_s16)))
void svmop4s_za32(uint64_t, svint16_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_u8_u8)))
void svmop4s_za32(uint64_t, svuint8_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za32_u16_u16)))
void svmop4s_za32(uint64_t, svuint16_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_s8_u8)))
void svmop4s_za32(uint64_t, svint8x2_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_u8_s8)))
void svmop4s_za32(uint64_t, svuint8x2_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_bf16_bf16)))
void svmop4s_za32(uint64_t, svbfloat16x2_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_f16_f16)))
void svmop4s_za32(uint64_t, svfloat16x2_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_f32_f32)))
void svmop4s_za32(uint64_t, svfloat32x2_t, svfloat32_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_s8_s8)))
void svmop4s_za32(uint64_t, svint8x2_t, svint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_s16_s16)))
void svmop4s_za32(uint64_t, svint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_u8_u8)))
void svmop4s_za32(uint64_t, svuint8x2_t, svuint8_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za32_u16_u16)))
void svmop4s_za32(uint64_t, svuint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_s8_u8)))
void svmop4s_za32(uint64_t, svint8x2_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_u8_s8)))
void svmop4s_za32(uint64_t, svuint8x2_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_bf16_bf16)))
void svmop4s_za32(uint64_t, svbfloat16x2_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_f16_f16)))
void svmop4s_za32(uint64_t, svfloat16x2_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_f32_f32)))
void svmop4s_za32(uint64_t, svfloat32x2_t, svfloat32x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_s8_s8)))
void svmop4s_za32(uint64_t, svint8x2_t, svint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_s16_s16)))
void svmop4s_za32(uint64_t, svint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_u8_u8)))
void svmop4s_za32(uint64_t, svuint8x2_t, svuint8x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za32_u16_u16)))
void svmop4s_za32(uint64_t, svuint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za16_bf16_bf16)))
void svmop4a_1x1_za16_bf16_bf16(uint64_t, svbfloat16_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za16_bf16_bf16)))
void svmop4a_1x2_za16_bf16_bf16(uint64_t, svbfloat16_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za16_bf16_bf16)))
void svmop4a_2x1_za16_bf16_bf16(uint64_t, svbfloat16x2_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za16_bf16_bf16)))
void svmop4a_2x2_za16_bf16_bf16(uint64_t, svbfloat16x2_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za16_bf16_bf16)))
void svmop4s_1x1_za16_bf16_bf16(uint64_t, svbfloat16_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za16_bf16_bf16)))
void svmop4s_1x2_za16_bf16_bf16(uint64_t, svbfloat16_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za16_bf16_bf16)))
void svmop4s_2x1_za16_bf16_bf16(uint64_t, svbfloat16x2_t, svbfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za16_bf16_bf16)))
void svmop4s_2x2_za16_bf16_bf16(uint64_t, svbfloat16x2_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za16_bf16_bf16)))
void svmop4a_za16(uint64_t, svbfloat16_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za16_bf16_bf16)))
void svmop4a_za16(uint64_t, svbfloat16_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za16_bf16_bf16)))
void svmop4a_za16(uint64_t, svbfloat16x2_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za16_bf16_bf16)))
void svmop4a_za16(uint64_t, svbfloat16x2_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za16_bf16_bf16)))
void svmop4s_za16(uint64_t, svbfloat16_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za16_bf16_bf16)))
void svmop4s_za16(uint64_t, svbfloat16_t, svbfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za16_bf16_bf16)))
void svmop4s_za16(uint64_t, svbfloat16x2_t, svbfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za16_bf16_bf16)))
void svmop4s_za16(uint64_t, svbfloat16x2_t, svbfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za16_f16_f16)))
void svmop4a_1x1_za16_f16_f16(uint64_t, svfloat16_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za16_f16_f16)))
void svmop4a_1x2_za16_f16_f16(uint64_t, svfloat16_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za16_f16_f16)))
void svmop4a_2x1_za16_f16_f16(uint64_t, svfloat16x2_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za16_f16_f16)))
void svmop4a_2x2_za16_f16_f16(uint64_t, svfloat16x2_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za16_f16_f16)))
void svmop4s_1x1_za16_f16_f16(uint64_t, svfloat16_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za16_f16_f16)))
void svmop4s_1x2_za16_f16_f16(uint64_t, svfloat16_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za16_f16_f16)))
void svmop4s_2x1_za16_f16_f16(uint64_t, svfloat16x2_t, svfloat16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za16_f16_f16)))
void svmop4s_2x2_za16_f16_f16(uint64_t, svfloat16x2_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za16_f16_f16)))
void svmop4a_za16(uint64_t, svfloat16_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za16_f16_f16)))
void svmop4a_za16(uint64_t, svfloat16_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za16_f16_f16)))
void svmop4a_za16(uint64_t, svfloat16x2_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za16_f16_f16)))
void svmop4a_za16(uint64_t, svfloat16x2_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za16_f16_f16)))
void svmop4s_za16(uint64_t, svfloat16_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za16_f16_f16)))
void svmop4s_za16(uint64_t, svfloat16_t, svfloat16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za16_f16_f16)))
void svmop4s_za16(uint64_t, svfloat16x2_t, svfloat16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za16_f16_f16)))
void svmop4s_za16(uint64_t, svfloat16x2_t, svfloat16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za64_f64_f64)))
void svmop4a_1x1_za64_f64_f64(uint64_t, svfloat64_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za64_f64_f64)))
void svmop4a_1x2_za64_f64_f64(uint64_t, svfloat64_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za64_f64_f64)))
void svmop4a_2x1_za64_f64_f64(uint64_t, svfloat64x2_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za64_f64_f64)))
void svmop4a_2x2_za64_f64_f64(uint64_t, svfloat64x2_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za64_f64_f64)))
void svmop4s_1x1_za64_f64_f64(uint64_t, svfloat64_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za64_f64_f64)))
void svmop4s_1x2_za64_f64_f64(uint64_t, svfloat64_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za64_f64_f64)))
void svmop4s_2x1_za64_f64_f64(uint64_t, svfloat64x2_t, svfloat64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za64_f64_f64)))
void svmop4s_2x2_za64_f64_f64(uint64_t, svfloat64x2_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za64_f64_f64)))
void svmop4a_za64(uint64_t, svfloat64_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za64_f64_f64)))
void svmop4a_za64(uint64_t, svfloat64_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za64_f64_f64)))
void svmop4a_za64(uint64_t, svfloat64x2_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za64_f64_f64)))
void svmop4a_za64(uint64_t, svfloat64x2_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za64_f64_f64)))
void svmop4s_za64(uint64_t, svfloat64_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za64_f64_f64)))
void svmop4s_za64(uint64_t, svfloat64_t, svfloat64x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za64_f64_f64)))
void svmop4s_za64(uint64_t, svfloat64x2_t, svfloat64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za64_f64_f64)))
void svmop4s_za64(uint64_t, svfloat64x2_t, svfloat64x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za64_s16_u16)))
void svmop4a_1x1_za64_s16_u16(uint64_t, svint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za64_u16_s16)))
void svmop4a_1x1_za64_u16_s16(uint64_t, svuint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za64_s16_s16)))
void svmop4a_1x1_za64_s16_s16(uint64_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za64_u16_u16)))
void svmop4a_1x1_za64_u16_u16(uint64_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za64_s16_u16)))
void svmop4a_1x2_za64_s16_u16(uint64_t, svint16_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za64_u16_s16)))
void svmop4a_1x2_za64_u16_s16(uint64_t, svuint16_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za64_s16_s16)))
void svmop4a_1x2_za64_s16_s16(uint64_t, svint16_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za64_u16_u16)))
void svmop4a_1x2_za64_u16_u16(uint64_t, svuint16_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za64_s16_u16)))
void svmop4a_2x1_za64_s16_u16(uint64_t, svint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za64_u16_s16)))
void svmop4a_2x1_za64_u16_s16(uint64_t, svuint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za64_s16_s16)))
void svmop4a_2x1_za64_s16_s16(uint64_t, svint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za64_u16_u16)))
void svmop4a_2x1_za64_u16_u16(uint64_t, svuint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za64_s16_u16)))
void svmop4a_2x2_za64_s16_u16(uint64_t, svint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za64_u16_s16)))
void svmop4a_2x2_za64_u16_s16(uint64_t, svuint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za64_s16_s16)))
void svmop4a_2x2_za64_s16_s16(uint64_t, svint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za64_u16_u16)))
void svmop4a_2x2_za64_u16_u16(uint64_t, svuint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za64_s16_u16)))
void svmop4s_1x1_za64_s16_u16(uint64_t, svint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za64_u16_s16)))
void svmop4s_1x1_za64_u16_s16(uint64_t, svuint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za64_s16_s16)))
void svmop4s_1x1_za64_s16_s16(uint64_t, svint16_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za64_u16_u16)))
void svmop4s_1x1_za64_u16_u16(uint64_t, svuint16_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za64_s16_u16)))
void svmop4s_1x2_za64_s16_u16(uint64_t, svint16_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za64_u16_s16)))
void svmop4s_1x2_za64_u16_s16(uint64_t, svuint16_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za64_s16_s16)))
void svmop4s_1x2_za64_s16_s16(uint64_t, svint16_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za64_u16_u16)))
void svmop4s_1x2_za64_u16_u16(uint64_t, svuint16_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za64_s16_u16)))
void svmop4s_2x1_za64_s16_u16(uint64_t, svint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za64_u16_s16)))
void svmop4s_2x1_za64_u16_s16(uint64_t, svuint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za64_s16_s16)))
void svmop4s_2x1_za64_s16_s16(uint64_t, svint16x2_t, svint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za64_u16_u16)))
void svmop4s_2x1_za64_u16_u16(uint64_t, svuint16x2_t, svuint16_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za64_s16_u16)))
void svmop4s_2x2_za64_s16_u16(uint64_t, svint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za64_u16_s16)))
void svmop4s_2x2_za64_u16_s16(uint64_t, svuint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za64_s16_s16)))
void svmop4s_2x2_za64_s16_s16(uint64_t, svint16x2_t, svint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za64_u16_u16)))
void svmop4s_2x2_za64_u16_u16(uint64_t, svuint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za64_s16_u16)))
void svmop4a_za64(uint64_t, svint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za64_u16_s16)))
void svmop4a_za64(uint64_t, svuint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za64_s16_s16)))
void svmop4a_za64(uint64_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x1_za64_u16_u16)))
void svmop4a_za64(uint64_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za64_s16_u16)))
void svmop4a_za64(uint64_t, svint16_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za64_u16_s16)))
void svmop4a_za64(uint64_t, svuint16_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za64_s16_s16)))
void svmop4a_za64(uint64_t, svint16_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_1x2_za64_u16_u16)))
void svmop4a_za64(uint64_t, svuint16_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za64_s16_u16)))
void svmop4a_za64(uint64_t, svint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za64_u16_s16)))
void svmop4a_za64(uint64_t, svuint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za64_s16_s16)))
void svmop4a_za64(uint64_t, svint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x1_za64_u16_u16)))
void svmop4a_za64(uint64_t, svuint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za64_s16_u16)))
void svmop4a_za64(uint64_t, svint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za64_u16_s16)))
void svmop4a_za64(uint64_t, svuint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za64_s16_s16)))
void svmop4a_za64(uint64_t, svint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4a_2x2_za64_u16_u16)))
void svmop4a_za64(uint64_t, svuint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za64_s16_u16)))
void svmop4s_za64(uint64_t, svint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za64_u16_s16)))
void svmop4s_za64(uint64_t, svuint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za64_s16_s16)))
void svmop4s_za64(uint64_t, svint16_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x1_za64_u16_u16)))
void svmop4s_za64(uint64_t, svuint16_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za64_s16_u16)))
void svmop4s_za64(uint64_t, svint16_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za64_u16_s16)))
void svmop4s_za64(uint64_t, svuint16_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za64_s16_s16)))
void svmop4s_za64(uint64_t, svint16_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_1x2_za64_u16_u16)))
void svmop4s_za64(uint64_t, svuint16_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za64_s16_u16)))
void svmop4s_za64(uint64_t, svint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za64_u16_s16)))
void svmop4s_za64(uint64_t, svuint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za64_s16_s16)))
void svmop4s_za64(uint64_t, svint16x2_t, svint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x1_za64_u16_u16)))
void svmop4s_za64(uint64_t, svuint16x2_t, svuint16_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za64_s16_u16)))
void svmop4s_za64(uint64_t, svint16x2_t, svuint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za64_u16_s16)))
void svmop4s_za64(uint64_t, svuint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za64_s16_s16)))
void svmop4s_za64(uint64_t, svint16x2_t, svint16x2_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svmop4s_2x2_za64_u16_u16)))
void svmop4s_za64(uint64_t, svuint16x2_t, svuint16x2_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_s8_u8)))
void svtmopa_lane_za32_s8_u8(uint64_t, svint8x2_t, svuint8_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_u8_s8)))
void svtmopa_lane_za32_u8_s8(uint64_t, svuint8x2_t, svint8_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_s8_s8)))
void svtmopa_lane_za32_s8_s8(uint64_t, svint8x2_t, svint8_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_s16_s16)))
void svtmopa_lane_za32_s16_s16(uint64_t, svint16x2_t, svint16_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_bf16_bf16)))
void svtmopa_lane_za32_bf16_bf16(uint64_t, svbfloat16x2_t, svbfloat16_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_f32_f32)))
void svtmopa_lane_za32_f32_f32(uint64_t, svfloat32x2_t, svfloat32_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_f16_f16)))
void svtmopa_lane_za32_f16_f16(uint64_t, svfloat16x2_t, svfloat16_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_u8_u8)))
void svtmopa_lane_za32_u8_u8(uint64_t, svuint8x2_t, svuint8_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_u16_u16)))
void svtmopa_lane_za32_u16_u16(uint64_t, svuint16x2_t, svuint16_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_s8_u8)))
void svtmopa_lane_za32(uint64_t, svint8x2_t, svuint8_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_u8_s8)))
void svtmopa_lane_za32(uint64_t, svuint8x2_t, svint8_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_s8_s8)))
void svtmopa_lane_za32(uint64_t, svint8x2_t, svint8_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_s16_s16)))
void svtmopa_lane_za32(uint64_t, svint16x2_t, svint16_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_bf16_bf16)))
void svtmopa_lane_za32(uint64_t, svbfloat16x2_t, svbfloat16_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_f32_f32)))
void svtmopa_lane_za32(uint64_t, svfloat32x2_t, svfloat32_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_f16_f16)))
void svtmopa_lane_za32(uint64_t, svfloat16x2_t, svfloat16_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_u8_u8)))
void svtmopa_lane_za32(uint64_t, svuint8x2_t, svuint8_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_u16_u16)))
void svtmopa_lane_za32(uint64_t, svuint16x2_t, svuint16_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za16_bf16_bf16)))
void svtmopa_lane_za16_bf16_bf16(uint64_t, svbfloat16x2_t, svbfloat16_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za16_bf16_bf16)))
void svtmopa_lane_za16(uint64_t, svbfloat16x2_t, svbfloat16_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za16_f16_f16)))
void svtmopa_lane_za16_f16_f16(uint64_t, svfloat16x2_t, svfloat16_t, svuint8_t, uint64_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za16_f16_f16)))
void svtmopa_lane_za16(uint64_t, svfloat16x2_t, svfloat16_t, svuint8_t, uint64_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za16_mf8_mf8_fpm)))
void svtmopa_lane_za16_mf8_mf8_fpm(uint64_t, svmfloat8x2_t, svmfloat8_t, svuint8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za16_mf8_mf8_fpm)))
void svtmopa_lane_za16_fpm(uint64_t, svmfloat8x2_t, svmfloat8_t, svuint8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_mf8_mf8_fpm)))
void svtmopa_lane_za32_mf8_mf8_fpm(uint64_t, svmfloat8x2_t, svmfloat8_t, svuint8_t, uint64_t, fpm_t);
__aio __attribute__((__clang_arm_builtin_alias(__builtin_sme_svtmopa_lane_za32_mf8_mf8_fpm)))
void svtmopa_lane_za32_fpm(uint64_t, svmfloat8x2_t, svmfloat8_t, svuint8_t, uint64_t, fpm_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_u8)))
svuint8_t svreadz_hor_za128_u8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_u32)))
svuint32_t svreadz_hor_za128_u32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_u64)))
svuint64_t svreadz_hor_za128_u64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_u16)))
svuint16_t svreadz_hor_za128_u16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_bf16)))
svbfloat16_t svreadz_hor_za128_bf16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_s8)))
svint8_t svreadz_hor_za128_s8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_f64)))
svfloat64_t svreadz_hor_za128_f64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_f32)))
svfloat32_t svreadz_hor_za128_f32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_f16)))
svfloat16_t svreadz_hor_za128_f16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_s32)))
svint32_t svreadz_hor_za128_s32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_s64)))
svint64_t svreadz_hor_za128_s64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_mf8)))
svmfloat8_t svreadz_hor_za128_mf8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za128_s16)))
svint16_t svreadz_hor_za128_s16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_u16)))
svuint16_t svreadz_hor_za16_u16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_bf16)))
svbfloat16_t svreadz_hor_za16_bf16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_f16)))
svfloat16_t svreadz_hor_za16_f16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_s16)))
svint16_t svreadz_hor_za16_s16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_u16_vg2)))
svuint16x2_t svreadz_hor_za16_u16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_bf16_vg2)))
svbfloat16x2_t svreadz_hor_za16_bf16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_f16_vg2)))
svfloat16x2_t svreadz_hor_za16_f16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_s16_vg2)))
svint16x2_t svreadz_hor_za16_s16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_u16_vg4)))
svuint16x4_t svreadz_hor_za16_u16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_bf16_vg4)))
svbfloat16x4_t svreadz_hor_za16_bf16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_f16_vg4)))
svfloat16x4_t svreadz_hor_za16_f16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za16_s16_vg4)))
svint16x4_t svreadz_hor_za16_s16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za32_u32)))
svuint32_t svreadz_hor_za32_u32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za32_f32)))
svfloat32_t svreadz_hor_za32_f32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za32_s32)))
svint32_t svreadz_hor_za32_s32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za32_u32_vg2)))
svuint32x2_t svreadz_hor_za32_u32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za32_f32_vg2)))
svfloat32x2_t svreadz_hor_za32_f32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za32_s32_vg2)))
svint32x2_t svreadz_hor_za32_s32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za32_u32_vg4)))
svuint32x4_t svreadz_hor_za32_u32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za32_f32_vg4)))
svfloat32x4_t svreadz_hor_za32_f32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za32_s32_vg4)))
svint32x4_t svreadz_hor_za32_s32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za64_u64)))
svuint64_t svreadz_hor_za64_u64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za64_f64)))
svfloat64_t svreadz_hor_za64_f64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za64_s64)))
svint64_t svreadz_hor_za64_s64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za64_u64_vg2)))
svuint64x2_t svreadz_hor_za64_u64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za64_f64_vg2)))
svfloat64x2_t svreadz_hor_za64_f64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za64_s64_vg2)))
svint64x2_t svreadz_hor_za64_s64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za64_u64_vg4)))
svuint64x4_t svreadz_hor_za64_u64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za64_f64_vg4)))
svfloat64x4_t svreadz_hor_za64_f64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za64_s64_vg4)))
svint64x4_t svreadz_hor_za64_s64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za8_u8)))
svuint8_t svreadz_hor_za8_u8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za8_s8)))
svint8_t svreadz_hor_za8_s8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za8_mf8)))
svmfloat8_t svreadz_hor_za8_mf8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za8_u8_vg2)))
svuint8x2_t svreadz_hor_za8_u8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za8_s8_vg2)))
svint8x2_t svreadz_hor_za8_s8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za8_mf8_vg2)))
svmfloat8x2_t svreadz_hor_za8_mf8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za8_u8_vg4)))
svuint8x4_t svreadz_hor_za8_u8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za8_s8_vg4)))
svint8x4_t svreadz_hor_za8_s8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_hor_za8_mf8_vg4)))
svmfloat8x4_t svreadz_hor_za8_mf8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_u8)))
svuint8_t svreadz_ver_za128_u8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_u32)))
svuint32_t svreadz_ver_za128_u32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_u64)))
svuint64_t svreadz_ver_za128_u64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_u16)))
svuint16_t svreadz_ver_za128_u16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_bf16)))
svbfloat16_t svreadz_ver_za128_bf16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_s8)))
svint8_t svreadz_ver_za128_s8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_f64)))
svfloat64_t svreadz_ver_za128_f64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_f32)))
svfloat32_t svreadz_ver_za128_f32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_f16)))
svfloat16_t svreadz_ver_za128_f16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_s32)))
svint32_t svreadz_ver_za128_s32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_s64)))
svint64_t svreadz_ver_za128_s64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_mf8)))
svmfloat8_t svreadz_ver_za128_mf8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za128_s16)))
svint16_t svreadz_ver_za128_s16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_u16)))
svuint16_t svreadz_ver_za16_u16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_bf16)))
svbfloat16_t svreadz_ver_za16_bf16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_f16)))
svfloat16_t svreadz_ver_za16_f16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_s16)))
svint16_t svreadz_ver_za16_s16(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_u16_vg2)))
svuint16x2_t svreadz_ver_za16_u16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_bf16_vg2)))
svbfloat16x2_t svreadz_ver_za16_bf16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_f16_vg2)))
svfloat16x2_t svreadz_ver_za16_f16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_s16_vg2)))
svint16x2_t svreadz_ver_za16_s16_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_u16_vg4)))
svuint16x4_t svreadz_ver_za16_u16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_bf16_vg4)))
svbfloat16x4_t svreadz_ver_za16_bf16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_f16_vg4)))
svfloat16x4_t svreadz_ver_za16_f16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za16_s16_vg4)))
svint16x4_t svreadz_ver_za16_s16_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za32_u32)))
svuint32_t svreadz_ver_za32_u32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za32_f32)))
svfloat32_t svreadz_ver_za32_f32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za32_s32)))
svint32_t svreadz_ver_za32_s32(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za32_u32_vg2)))
svuint32x2_t svreadz_ver_za32_u32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za32_f32_vg2)))
svfloat32x2_t svreadz_ver_za32_f32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za32_s32_vg2)))
svint32x2_t svreadz_ver_za32_s32_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za32_u32_vg4)))
svuint32x4_t svreadz_ver_za32_u32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za32_f32_vg4)))
svfloat32x4_t svreadz_ver_za32_f32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za32_s32_vg4)))
svint32x4_t svreadz_ver_za32_s32_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za64_u64)))
svuint64_t svreadz_ver_za64_u64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za64_f64)))
svfloat64_t svreadz_ver_za64_f64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za64_s64)))
svint64_t svreadz_ver_za64_s64(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za64_u64_vg2)))
svuint64x2_t svreadz_ver_za64_u64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za64_f64_vg2)))
svfloat64x2_t svreadz_ver_za64_f64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za64_s64_vg2)))
svint64x2_t svreadz_ver_za64_s64_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za64_u64_vg4)))
svuint64x4_t svreadz_ver_za64_u64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za64_f64_vg4)))
svfloat64x4_t svreadz_ver_za64_f64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za64_s64_vg4)))
svint64x4_t svreadz_ver_za64_s64_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za8_u8)))
svuint8_t svreadz_ver_za8_u8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za8_s8)))
svint8_t svreadz_ver_za8_s8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za8_mf8)))
svmfloat8_t svreadz_ver_za8_mf8(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za8_u8_vg2)))
svuint8x2_t svreadz_ver_za8_u8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za8_s8_vg2)))
svint8x2_t svreadz_ver_za8_s8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za8_mf8_vg2)))
svmfloat8x2_t svreadz_ver_za8_mf8_vg2(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za8_u8_vg4)))
svuint8x4_t svreadz_ver_za8_u8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za8_s8_vg4)))
svint8x4_t svreadz_ver_za8_s8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_ver_za8_mf8_vg4)))
svmfloat8x4_t svreadz_ver_za8_mf8_vg4(uint64_t, uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za16_u16_vg1x2)))
svuint16x2_t svreadz_za16_u16_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za16_bf16_vg1x2)))
svbfloat16x2_t svreadz_za16_bf16_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za16_f16_vg1x2)))
svfloat16x2_t svreadz_za16_f16_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za16_s16_vg1x2)))
svint16x2_t svreadz_za16_s16_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za16_u16_vg1x4)))
svuint16x4_t svreadz_za16_u16_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za16_bf16_vg1x4)))
svbfloat16x4_t svreadz_za16_bf16_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za16_f16_vg1x4)))
svfloat16x4_t svreadz_za16_f16_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za16_s16_vg1x4)))
svint16x4_t svreadz_za16_s16_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za32_u32_vg1x2)))
svuint32x2_t svreadz_za32_u32_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za32_f32_vg1x2)))
svfloat32x2_t svreadz_za32_f32_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za32_s32_vg1x2)))
svint32x2_t svreadz_za32_s32_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za32_u32_vg1x4)))
svuint32x4_t svreadz_za32_u32_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za32_f32_vg1x4)))
svfloat32x4_t svreadz_za32_f32_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za32_s32_vg1x4)))
svint32x4_t svreadz_za32_s32_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za64_u64_vg1x2)))
svuint64x2_t svreadz_za64_u64_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za64_f64_vg1x2)))
svfloat64x2_t svreadz_za64_f64_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za64_s64_vg1x2)))
svint64x2_t svreadz_za64_s64_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za64_u64_vg1x4)))
svuint64x4_t svreadz_za64_u64_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za64_f64_vg1x4)))
svfloat64x4_t svreadz_za64_f64_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za64_s64_vg1x4)))
svint64x4_t svreadz_za64_s64_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za8_u8_vg1x2)))
svuint8x2_t svreadz_za8_u8_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za8_s8_vg1x2)))
svint8x2_t svreadz_za8_s8_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za8_mf8_vg1x2)))
svmfloat8x2_t svreadz_za8_mf8_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za8_u8_vg1x4)))
svuint8x4_t svreadz_za8_u8_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za8_s8_vg1x4)))
svint8x4_t svreadz_za8_s8_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svreadz_za8_mf8_vg1x4)))
svmfloat8x4_t svreadz_za8_mf8_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_za64_vg1x2)))
void svzero_za64_vg1x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_za64_vg1x4)))
void svzero_za64_vg1x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_za64_vg2x1)))
void svzero_za64_vg2x1(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_za64_vg2x2)))
void svzero_za64_vg2x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_za64_vg2x4)))
void svzero_za64_vg2x4(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_za64_vg4x1)))
void svzero_za64_vg4x1(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_za64_vg4x2)))
void svzero_za64_vg4x2(uint32_t);
__ai __attribute__((__clang_arm_builtin_alias(__builtin_sme_svzero_za64_vg4x4)))
void svzero_za64_vg4x4(uint32_t);
#ifdef __cplusplus
} // extern "C"
#endif

#undef __ai

#endif /* __ARM_SME_H */
