/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

#ifndef MLK_NATIVE_X86_64_META_H
#define MLK_NATIVE_X86_64_META_H

/* Identifier for this backend so that source and assembly files
 * in the build can be appropriately guarded. */
#define MLK_ARITH_BACKEND_X86_64_DEFAULT

#define MLK_USE_NATIVE_NTT_CUSTOM_ORDER
#define MLK_USE_NATIVE_REJ_UNIFORM
#define MLK_USE_NATIVE_NTT
#define MLK_USE_NATIVE_INTT
#define MLK_USE_NATIVE_POLY_REDUCE
#define MLK_USE_NATIVE_POLY_TOMONT
#define MLK_USE_NATIVE_POLYVEC_BASEMUL_ACC_MONTGOMERY_CACHED
#define MLK_USE_NATIVE_POLY_MULCACHE_COMPUTE
#define MLK_USE_NATIVE_POLY_TOBYTES
#define MLK_USE_NATIVE_POLY_FROMBYTES
#define MLK_USE_NATIVE_POLY_COMPRESS_D4
#define MLK_USE_NATIVE_POLY_COMPRESS_D5
#define MLK_USE_NATIVE_POLY_COMPRESS_D10
#define MLK_USE_NATIVE_POLY_COMPRESS_D11
#define MLK_USE_NATIVE_POLY_DECOMPRESS_D4
#define MLK_USE_NATIVE_POLY_DECOMPRESS_D5
#define MLK_USE_NATIVE_POLY_DECOMPRESS_D10
#define MLK_USE_NATIVE_POLY_DECOMPRESS_D11

#if !defined(__ASSEMBLER__)
#include "../../common.h"
#include "../api.h"
#include "src/arith_native_x86_64.h"
#include "src/compress_consts.h"

static MLK_INLINE void mlk_poly_permute_bitrev_to_custom(int16_t data[MLKEM_N])
{
  if (mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    mlk_nttunpack_avx2(data);
  }
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_rej_uniform_native(int16_t *r, unsigned len,
                                             const uint8_t *buf,
                                             unsigned buflen)
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2) || len != MLKEM_N ||
      buflen % 12 != 0)
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }
  return (int)mlk_rej_uniform_asm(r, buf, buflen, mlk_rej_uniform_table);
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_ntt_native(int16_t data[MLKEM_N])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_ntt_avx2(data, mlk_qdata);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_intt_native(int16_t data[MLKEM_N])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_invntt_avx2(data, mlk_qdata);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_reduce_native(int16_t data[MLKEM_N])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_reduce_avx2(data);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_tomont_native(int16_t data[MLKEM_N])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_tomont_avx2(data);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_mulcache_compute_native(int16_t x[MLKEM_N / 2],
                                                       const int16_t y[MLKEM_N])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_poly_mulcache_compute_avx2(x, y, mlk_qdata);
  return MLK_NATIVE_FUNC_SUCCESS;
}

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 2
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_polyvec_basemul_acc_montgomery_cached_k2_native(
    int16_t r[MLKEM_N], const int16_t a[2 * MLKEM_N],
    const int16_t b[2 * MLKEM_N], const int16_t b_cache[2 * (MLKEM_N / 2)])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

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
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

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
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_polyvec_basemul_acc_montgomery_cached_asm_k4(r, a, b, b_cache);
  return MLK_NATIVE_FUNC_SUCCESS;
}
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 4 */

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_tobytes_native(uint8_t r[MLKEM_POLYBYTES],
                                              const int16_t a[MLKEM_N])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_ntttobytes_avx2(r, a);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_frombytes_native(
    int16_t r[MLKEM_N], const uint8_t a[MLKEM_POLYBYTES])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_nttfrombytes_avx2(r, a);
  return MLK_NATIVE_FUNC_SUCCESS;
}

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || (MLKEM_K == 2 || MLKEM_K == 3)
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_compress_d4_native(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D4], const int16_t a[MLKEM_N])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_poly_compress_d4_avx2(r, a, mlk_compress_d4_data);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_compress_d10_native(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D10], const int16_t a[MLKEM_N])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_poly_compress_d10_avx2(r, a, mlk_compress_d10_data);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_decompress_d4_native(
    int16_t r[MLKEM_N], const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D4])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_poly_decompress_d4_avx2(r, a, mlk_decompress_d4_data);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_decompress_d10_native(
    int16_t r[MLKEM_N], const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D10])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_poly_decompress_d10_avx2(r, a, mlk_decompress_d10_data);
  return MLK_NATIVE_FUNC_SUCCESS;
}
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 2 || MLKEM_K == 3 */

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 4
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_compress_d5_native(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D5], const int16_t a[MLKEM_N])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_poly_compress_d5_avx2(r, a, mlk_compress_d5_data);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_compress_d11_native(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D11], const int16_t a[MLKEM_N])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_poly_compress_d11_avx2(r, a, mlk_compress_d11_data);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_decompress_d5_native(
    int16_t r[MLKEM_N], const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D5])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_poly_decompress_d5_avx2(r, a, mlk_decompress_d5_data);
  return MLK_NATIVE_FUNC_SUCCESS;
}

MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_decompress_d11_native(
    int16_t r[MLKEM_N], const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D11])
{
  if (!mlk_sys_check_capability(MLK_SYS_CAP_AVX2))
  {
    return MLK_NATIVE_FUNC_FALLBACK;
  }

  mlk_poly_decompress_d11_avx2(r, a, mlk_decompress_d11_data);
  return MLK_NATIVE_FUNC_SUCCESS;
}
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 4 */

#endif /* !__ASSEMBLER__ */

#endif /* !MLK_NATIVE_X86_64_META_H */
