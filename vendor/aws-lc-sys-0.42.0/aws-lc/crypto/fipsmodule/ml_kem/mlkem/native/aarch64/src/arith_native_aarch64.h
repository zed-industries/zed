/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLK_NATIVE_AARCH64_SRC_ARITH_NATIVE_AARCH64_H
#define MLK_NATIVE_AARCH64_SRC_ARITH_NATIVE_AARCH64_H

#include "../../../cbmc.h"
#include "../../../common.h"

#define mlk_aarch64_ntt_zetas_layer12345 \
  MLK_NAMESPACE(aarch64_ntt_zetas_layer12345)
#define mlk_aarch64_ntt_zetas_layer67 MLK_NAMESPACE(aarch64_ntt_zetas_layer67)
#define mlk_aarch64_invntt_zetas_layer12345 \
  MLK_NAMESPACE(aarch64_invntt_zetas_layer12345)
#define mlk_aarch64_invntt_zetas_layer67 \
  MLK_NAMESPACE(aarch64_invntt_zetas_layer67)
#define mlk_aarch64_zetas_mulcache_native \
  MLK_NAMESPACE(aarch64_zetas_mulcache_native)
#define mlk_aarch64_zetas_mulcache_twisted_native \
  MLK_NAMESPACE(aarch64_zetas_mulcache_twisted_native)
#define mlk_rej_uniform_table MLK_NAMESPACE(rej_uniform_table)

extern const int16_t mlk_aarch64_ntt_zetas_layer12345[];
extern const int16_t mlk_aarch64_ntt_zetas_layer67[];
extern const int16_t mlk_aarch64_invntt_zetas_layer12345[];
extern const int16_t mlk_aarch64_invntt_zetas_layer67[];
extern const int16_t mlk_aarch64_zetas_mulcache_native[];
extern const int16_t mlk_aarch64_zetas_mulcache_twisted_native[];
extern const uint8_t mlk_rej_uniform_table[];

#define mlk_ntt_asm MLK_NAMESPACE(ntt_asm)
void mlk_ntt_asm(int16_t p[256], const int16_t twiddles12345[80],
                 const int16_t twiddles56[384])
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/mlkem_ntt.ml */
__contract__(
  requires(memory_no_alias(p, sizeof(int16_t) * MLKEM_N))
  requires(array_abs_bound(p, 0, MLKEM_N, 8192))
  requires(twiddles12345 == mlk_aarch64_ntt_zetas_layer12345)
  requires(twiddles56 == mlk_aarch64_ntt_zetas_layer67)
  assigns(memory_slice(p, sizeof(int16_t) * MLKEM_N))
  /* check-magic: off */
  ensures(array_abs_bound(p, 0, MLKEM_N, 23595))
  /* check-magic: on */
);

#define mlk_intt_asm MLK_NAMESPACE(intt_asm)
void mlk_intt_asm(int16_t p[256], const int16_t twiddles12345[80],
                  const int16_t twiddles56[384])
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/mlkem_intt.ml */
__contract__(
  requires(memory_no_alias(p, sizeof(int16_t) * MLKEM_N))
  requires(twiddles12345 == mlk_aarch64_invntt_zetas_layer12345)
  requires(twiddles56 == mlk_aarch64_invntt_zetas_layer67)
  assigns(memory_slice(p, sizeof(int16_t) * MLKEM_N))
  /* check-magic: off */
  ensures(array_abs_bound(p, 0, MLKEM_N, 26625))
  /* check-magic: on */
);

#define mlk_poly_reduce_asm MLK_NAMESPACE(poly_reduce_asm)
void mlk_poly_reduce_asm(int16_t p[256])
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/mlkem_poly_reduce.ml */
__contract__(
  requires(memory_no_alias(p, sizeof(int16_t) * MLKEM_N))
  assigns(memory_slice(p, sizeof(int16_t) * MLKEM_N))
  ensures(array_bound(p, 0, MLKEM_N, 0, MLKEM_Q))
);

#define mlk_poly_tomont_asm MLK_NAMESPACE(poly_tomont_asm)
void mlk_poly_tomont_asm(int16_t p[256])
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/mlkem_poly_tomont.ml */
__contract__(
  requires(memory_no_alias(p, sizeof(int16_t) * MLKEM_N))
  assigns(memory_slice(p, sizeof(int16_t) * MLKEM_N))
  ensures(array_abs_bound(p, 0, MLKEM_N, MLKEM_Q))
);

#define mlk_poly_mulcache_compute_asm MLK_NAMESPACE(poly_mulcache_compute_asm)
void mlk_poly_mulcache_compute_asm(int16_t cache[128],
                                   const int16_t mlk_poly[256],
                                   const int16_t zetas[128],
                                   const int16_t zetas_twisted[128])
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/mlkem_poly_mulcache_compute.ml */
__contract__(
  requires(memory_no_alias(cache, sizeof(int16_t) * (MLKEM_N / 2)))
  requires(memory_no_alias(mlk_poly, sizeof(int16_t) * MLKEM_N))
  requires(zetas == mlk_aarch64_zetas_mulcache_native)
  requires(zetas_twisted == mlk_aarch64_zetas_mulcache_twisted_native)
  assigns(memory_slice(cache, sizeof(int16_t) * (MLKEM_N / 2)))
  ensures(array_abs_bound(cache, 0, MLKEM_N/2, MLKEM_Q))
);

#define mlk_poly_tobytes_asm MLK_NAMESPACE(poly_tobytes_asm)
void mlk_poly_tobytes_asm(uint8_t r[384], const int16_t a[256])
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/mlkem_poly_tobytes.ml */
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYBYTES))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_UINT12_LIMIT))
  assigns(memory_slice(r, MLKEM_POLYBYTES))
);

#define mlk_polyvec_basemul_acc_montgomery_cached_asm_k2 \
  MLK_NAMESPACE(polyvec_basemul_acc_montgomery_cached_asm_k2)
void mlk_polyvec_basemul_acc_montgomery_cached_asm_k2(
    int16_t r[256], const int16_t a[512], const int16_t b[512],
    const int16_t b_cache[256])
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/aarch64/proofs/mlkem_poly_basemul_acc_montgomery_cached_k2.ml.
 */
__contract__(
    requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
    requires(memory_no_alias(a, sizeof(int16_t) * 2 * MLKEM_N))
    requires(memory_no_alias(b, sizeof(int16_t) * 2 * MLKEM_N))
    requires(memory_no_alias(b_cache, sizeof(int16_t) * 2 * (MLKEM_N / 2)))
    requires(array_abs_bound(a, 0, 2 * MLKEM_N, MLKEM_UINT12_LIMIT + 1))
    assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
);

#define mlk_polyvec_basemul_acc_montgomery_cached_asm_k3 \
  MLK_NAMESPACE(polyvec_basemul_acc_montgomery_cached_asm_k3)
void mlk_polyvec_basemul_acc_montgomery_cached_asm_k3(
    int16_t r[256], const int16_t a[768], const int16_t b[768],
    const int16_t b_cache[384])
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/aarch64/proofs/mlkem_poly_basemul_acc_montgomery_cached_k3.ml.
 */
__contract__(
    requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
    requires(memory_no_alias(a, sizeof(int16_t) * 3 * MLKEM_N))
    requires(memory_no_alias(b, sizeof(int16_t) * 3 * MLKEM_N))
    requires(memory_no_alias(b_cache, sizeof(int16_t) * 3 * (MLKEM_N / 2)))
    requires(array_abs_bound(a, 0, 3 * MLKEM_N, MLKEM_UINT12_LIMIT + 1))
    assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
);

#define mlk_polyvec_basemul_acc_montgomery_cached_asm_k4 \
  MLK_NAMESPACE(polyvec_basemul_acc_montgomery_cached_asm_k4)
void mlk_polyvec_basemul_acc_montgomery_cached_asm_k4(
    int16_t r[256], const int16_t a[1024], const int16_t b[1024],
    const int16_t b_cache[512])
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/aarch64/proofs/mlkem_poly_basemul_acc_montgomery_cached_k4.ml.
 */
__contract__(
    requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
    requires(memory_no_alias(a, sizeof(int16_t) * 4 * MLKEM_N))
    requires(memory_no_alias(b, sizeof(int16_t) * 4 * MLKEM_N))
    requires(memory_no_alias(b_cache, sizeof(int16_t) * 4 * (MLKEM_N / 2)))
    requires(array_abs_bound(a, 0, 4 * MLKEM_N, MLKEM_UINT12_LIMIT + 1))
    assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
);

#define mlk_rej_uniform_asm MLK_NAMESPACE(rej_uniform_asm)
MLK_MUST_CHECK_RETURN_VALUE
uint64_t mlk_rej_uniform_asm(int16_t r[256], const uint8_t *buf,
                             unsigned buflen, const uint8_t table[2048])
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/mlkem_rej_uniform.ml. */
__contract__(
    requires(buflen % 24 == 0)
    requires(memory_no_alias(buf, buflen))
    requires(table == mlk_rej_uniform_table)
    requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
    assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
    ensures(return_value <= MLKEM_N)
    ensures(array_bound(r, 0, (unsigned) return_value, 0, MLKEM_Q))
);

#endif /* !MLK_NATIVE_AARCH64_SRC_ARITH_NATIVE_AARCH64_H */
