/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLK_NATIVE_X86_64_SRC_ARITH_NATIVE_X86_64_H
#define MLK_NATIVE_X86_64_SRC_ARITH_NATIVE_X86_64_H

#include "../../../common.h"

#include <stdint.h>
#include "compress_consts.h"
#include "consts.h"

#define MLK_AVX2_REJ_UNIFORM_BUFLEN \
  (3 * 168) /* REJ_UNIFORM_NBLOCKS * SHAKE128_RATE */

#define mlk_rej_uniform_table MLK_NAMESPACE(rej_uniform_table)
extern const uint8_t mlk_rej_uniform_table[];

#define mlk_rej_uniform_asm MLK_NAMESPACE(rej_uniform_asm)
MLK_MUST_CHECK_RETURN_VALUE
uint64_t mlk_rej_uniform_asm(int16_t *r, const uint8_t *buf, unsigned buflen,
                             const uint8_t *table)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/mlkem_rej_uniform.ml. */
__contract__(
    requires(buflen % 12 == 0)
    requires(memory_no_alias(buf, buflen))
    requires(table == mlk_rej_uniform_table)
    requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
    assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
    ensures(return_value <= MLKEM_N)
    ensures(array_bound(r, 0, (unsigned) return_value, 0, MLKEM_Q))
);

#define mlk_ntt_avx2 MLK_NAMESPACE(ntt_avx2)
void mlk_ntt_avx2(int16_t *r, const int16_t *qdata)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/mlkem_ntt.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(array_abs_bound(r, 0, MLKEM_N, 8192))
  requires(qdata == mlk_qdata)
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  /* check-magic: off */
  ensures(array_abs_bound(r, 0, MLKEM_N, 23595))
  /* check-magic: on */
);

#define mlk_invntt_avx2 MLK_NAMESPACE(invntt_avx2)
void mlk_invntt_avx2(int16_t *r, const int16_t *qdata)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/mlkem_intt.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(qdata == mlk_qdata)
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  /* check-magic: off */
  ensures(array_abs_bound(r, 0, MLKEM_N, 26632))
  /* check-magic: on */
);

#define mlk_nttunpack_avx2 MLK_NAMESPACE(nttunpack_avx2)
void mlk_nttunpack_avx2(int16_t *r)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/mlkem_unpack.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(r, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  /* Output is a permutation of input: every output coefficient
   * is some input coefficient */
  ensures(forall(i, 0, MLKEM_N, exists(j, 0, MLKEM_N,
    r[i] == old(*(int16_t (*)[MLKEM_N])r)[j])))
);

#define mlk_reduce_avx2 MLK_NAMESPACE(reduce_avx2)
void mlk_reduce_avx2(int16_t *r)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/mlkem_reduce.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(array_bound(r, 0, MLKEM_N, 0, MLKEM_Q))
);

#define mlk_poly_mulcache_compute_avx2 MLK_NAMESPACE(poly_mulcache_compute_avx2)
void mlk_poly_mulcache_compute_avx2(int16_t *out, const int16_t *in,
                                    const int16_t *qdata)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/mlkem_mulcache_compute.ml */
__contract__(
  requires(memory_no_alias(out, sizeof(int16_t) * (MLKEM_N / 2)))
  requires(memory_no_alias(in, sizeof(int16_t) * MLKEM_N))
  requires(qdata == mlk_qdata)
  assigns(memory_slice(out, sizeof(int16_t) * (MLKEM_N / 2)))
  ensures(array_abs_bound(out, 0, MLKEM_N/2, MLKEM_Q))
);

#define mlk_polyvec_basemul_acc_montgomery_cached_asm_k2 \
  MLK_NAMESPACE(polyvec_basemul_acc_montgomery_cached_asm_k2)
void mlk_polyvec_basemul_acc_montgomery_cached_asm_k2(int16_t *r,
                                                      const int16_t *a,
                                                      const int16_t *b,
                                                      const int16_t *b_cache)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_basemul_acc_montgomery_cached_k2.ml.
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
void mlk_polyvec_basemul_acc_montgomery_cached_asm_k3(int16_t *r,
                                                      const int16_t *a,
                                                      const int16_t *b,
                                                      const int16_t *b_cache)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_basemul_acc_montgomery_cached_k3.ml.
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
void mlk_polyvec_basemul_acc_montgomery_cached_asm_k4(int16_t *r,
                                                      const int16_t *a,
                                                      const int16_t *b,
                                                      const int16_t *b_cache)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_basemul_acc_montgomery_cached_k4.ml.
 */
__contract__(
    requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
    requires(memory_no_alias(a, sizeof(int16_t) * 4 * MLKEM_N))
    requires(memory_no_alias(b, sizeof(int16_t) * 4 * MLKEM_N))
    requires(memory_no_alias(b_cache, sizeof(int16_t) * 4 * (MLKEM_N / 2)))
    requires(array_abs_bound(a, 0, 4 * MLKEM_N, MLKEM_UINT12_LIMIT + 1))
    assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
);

#define mlk_ntttobytes_avx2 MLK_NAMESPACE(ntttobytes_avx2)
void mlk_ntttobytes_avx2(uint8_t *r, const int16_t *a)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_tobytes.ml.
 */
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYBYTES))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYBYTES))
);

#define mlk_nttfrombytes_avx2 MLK_NAMESPACE(nttfrombytes_avx2)
void mlk_nttfrombytes_avx2(int16_t *r, const uint8_t *a)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_frombytes.ml.
 */
__contract__(
  requires(memory_no_alias(a, MLKEM_POLYBYTES))
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(array_bound(r, 0, MLKEM_N, 0, MLKEM_UINT12_LIMIT))
);

#define mlk_tomont_avx2 MLK_NAMESPACE(tomont_avx2)
void mlk_tomont_avx2(int16_t *r)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_tomont.ml.
 */
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(array_abs_bound(r, 0, MLKEM_N, MLKEM_Q))
);

#define mlk_poly_compress_d4_avx2 MLK_NAMESPACE(poly_compress_d4_avx2)
void mlk_poly_compress_d4_avx2(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D4],
                               const int16_t *MLK_RESTRICT a,
                               const uint8_t *data)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_compress_d4.ml.
 */
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D4))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_Q))
  requires(data == mlk_compress_d4_data)
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D4))
);

#define mlk_poly_decompress_d4_avx2 MLK_NAMESPACE(poly_decompress_d4_avx2)
void mlk_poly_decompress_d4_avx2(int16_t *MLK_RESTRICT r,
                                 const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D4],
                                 const uint8_t *data)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_decompress_d4.ml.
 */
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D4))
  requires(data == mlk_decompress_d4_data)
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(array_bound(r, 0, MLKEM_N, 0, MLKEM_Q))
);

#define mlk_poly_compress_d10_avx2 MLK_NAMESPACE(poly_compress_d10_avx2)
void mlk_poly_compress_d10_avx2(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D10],
                                const int16_t *MLK_RESTRICT a,
                                const uint8_t *data)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_compress_d10.ml.
 */
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D10))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_Q))
  requires(data == mlk_compress_d10_data)
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D10))
);

#define mlk_poly_decompress_d10_avx2 MLK_NAMESPACE(poly_decompress_d10_avx2)
void mlk_poly_decompress_d10_avx2(
    int16_t *MLK_RESTRICT r, const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D10],
    const uint8_t *data)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_decompress_d10.ml.
 */
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D10))
  requires(data == mlk_decompress_d10_data)
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(array_bound(r, 0, MLKEM_N, 0, MLKEM_Q))
);

#define mlk_poly_compress_d5_avx2 MLK_NAMESPACE(poly_compress_d5_avx2)
void mlk_poly_compress_d5_avx2(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D5],
                               const int16_t *MLK_RESTRICT a,
                               const uint8_t *data)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_compress_d5.ml.
 */
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D5))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_Q))
  requires(data == mlk_compress_d5_data)
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D5))
);

#define mlk_poly_decompress_d5_avx2 MLK_NAMESPACE(poly_decompress_d5_avx2)
void mlk_poly_decompress_d5_avx2(int16_t *MLK_RESTRICT r,
                                 const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D5],
                                 const uint8_t *data)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_decompress_d5.ml.
 */
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D5))
  requires(data == mlk_decompress_d5_data)
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(array_bound(r, 0, MLKEM_N, 0, MLKEM_Q))
);

#define mlk_poly_compress_d11_avx2 MLK_NAMESPACE(poly_compress_d11_avx2)
void mlk_poly_compress_d11_avx2(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D11],
                                const int16_t *MLK_RESTRICT a,
                                const uint8_t *data)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_compress_d11.ml.
 */
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D11))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_Q))
  requires(data == mlk_compress_d11_data)
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D11))
);

#define mlk_poly_decompress_d11_avx2 MLK_NAMESPACE(poly_decompress_d11_avx2)
void mlk_poly_decompress_d11_avx2(
    int16_t *MLK_RESTRICT r, const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D11],
    const uint8_t *data)
/* This must be kept in sync with the HOL-Light specification in
 * proofs/hol_light/x86_64/proofs/mlkem_poly_decompress_d11.ml.
 */
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D11))
  requires(data == mlk_decompress_d11_data)
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(array_bound(r, 0, MLKEM_N, 0, MLKEM_Q))
);

#endif /* !MLK_NATIVE_X86_64_SRC_ARITH_NATIVE_X86_64_H */
