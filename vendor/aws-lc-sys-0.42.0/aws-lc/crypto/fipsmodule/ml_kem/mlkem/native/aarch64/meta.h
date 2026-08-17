/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

#ifndef MLK_NATIVE_AARCH64_META_H
#define MLK_NATIVE_AARCH64_META_H

/* Set of primitives that this backend replaces */
#define MLK_USE_NATIVE_NTT
#define MLK_USE_NATIVE_INTT
#define MLK_USE_NATIVE_POLY_REDUCE
#define MLK_USE_NATIVE_POLY_TOMONT
#define MLK_USE_NATIVE_POLY_MULCACHE_COMPUTE
#define MLK_USE_NATIVE_POLYVEC_BASEMUL_ACC_MONTGOMERY_CACHED
#define MLK_USE_NATIVE_POLY_TOBYTES
#define MLK_USE_NATIVE_REJ_UNIFORM

/* Identifier for this backend so that source and assembly files
 * in the build can be appropriately guarded. */
#define MLK_ARITH_BACKEND_AARCH64


#if !defined(__ASSEMBLER__)
#include "../api.h"
#include "src/arith_native_aarch64.h"

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_ntt_native(int16_t data[MLKEM_N])
{
  mlk_ntt_asm(data, mlk_aarch64_ntt_zetas_layer12345,
              mlk_aarch64_ntt_zetas_layer67);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_intt_native(int16_t data[MLKEM_N])
{
  mlk_intt_asm(data, mlk_aarch64_invntt_zetas_layer12345,
               mlk_aarch64_invntt_zetas_layer67);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_reduce_native(int16_t data[MLKEM_N])
{
  mlk_poly_reduce_asm(data);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_tomont_native(int16_t data[MLKEM_N])
{
  mlk_poly_tomont_asm(data);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_mulcache_compute_native(int16_t x[MLKEM_N / 2],
                                                       const int16_t y[MLKEM_N])
{
  mlk_poly_mulcache_compute_asm(x, y, mlk_aarch64_zetas_mulcache_native,
                                mlk_aarch64_zetas_mulcache_twisted_native);
  return MLK_NATIVE_FUNC_SUCCESS;
}

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 2
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_polyvec_basemul_acc_montgomery_cached_k2_native(
    int16_t r[MLKEM_N], const int16_t a[2 * MLKEM_N],
    const int16_t b[2 * MLKEM_N], const int16_t b_cache[2 * (MLKEM_N / 2)])
{
  mlk_polyvec_basemul_acc_montgomery_cached_asm_k2(r, a, b, b_cache);
  return MLK_NATIVE_FUNC_SUCCESS;
}
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 2 */

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 3
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_polyvec_basemul_acc_montgomery_cached_k3_native(
    int16_t r[MLKEM_N], const int16_t a[3 * MLKEM_N],
    const int16_t b[3 * MLKEM_N], const int16_t b_cache[3 * (MLKEM_N / 2)])
{
  mlk_polyvec_basemul_acc_montgomery_cached_asm_k3(r, a, b, b_cache);
  return MLK_NATIVE_FUNC_SUCCESS;
}
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 3 */

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 4
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_polyvec_basemul_acc_montgomery_cached_k4_native(
    int16_t r[MLKEM_N], const int16_t a[4 * MLKEM_N],
    const int16_t b[4 * MLKEM_N], const int16_t b_cache[4 * (MLKEM_N / 2)])
{
  mlk_polyvec_basemul_acc_montgomery_cached_asm_k4(r, a, b, b_cache);
  return MLK_NATIVE_FUNC_SUCCESS;
}
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 4 */

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_tobytes_native(uint8_t r[MLKEM_POLYBYTES],
                                              const int16_t a[MLKEM_N])
{
  mlk_poly_tobytes_asm(r, a);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_rej_uniform_native(int16_t *r, unsigned len,
                                             const uint8_t *buf,
                                             unsigned buflen)
{
  if (len != MLKEM_N ||
      buflen % 24 != 0) /* NEON support is mandatory for AArch64 */
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }
  return (int)mlk_rej_uniform_asm(r, buf, buflen, mlk_rej_uniform_table);
}
#endif /* !__ASSEMBLER__ */

#endif /* !MLK_NATIVE_AARCH64_META_H */
