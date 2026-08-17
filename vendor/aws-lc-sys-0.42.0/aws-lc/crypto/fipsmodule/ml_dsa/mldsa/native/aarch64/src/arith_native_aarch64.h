/*
 * Copyright (c) The mlkem-native project authors
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

#ifndef MLD_NATIVE_AARCH64_SRC_ARITH_NATIVE_AARCH64_H
#define MLD_NATIVE_AARCH64_SRC_ARITH_NATIVE_AARCH64_H

#include "../../../cbmc.h"
#include "../../../common.h"

#define mld_aarch64_ntt_zetas_layer123456 \
  MLD_NAMESPACE(aarch64_ntt_zetas_layer123456)
#define mld_aarch64_ntt_zetas_layer78 MLD_NAMESPACE(aarch64_ntt_zetas_layer78)

#define mld_aarch64_intt_zetas_layer78 MLD_NAMESPACE(aarch64_intt_zetas_layer78)
#define mld_aarch64_intt_zetas_layer123456 \
  MLD_NAMESPACE(aarch64_intt_zetas_layer123456)

MLD_INTERNAL_DATA_DECLARATION const int32_t
    mld_aarch64_ntt_zetas_layer123456[144];
MLD_INTERNAL_DATA_DECLARATION const int32_t mld_aarch64_ntt_zetas_layer78[384];

MLD_INTERNAL_DATA_DECLARATION const int32_t mld_aarch64_intt_zetas_layer78[384];
MLD_INTERNAL_DATA_DECLARATION const int32_t
    mld_aarch64_intt_zetas_layer123456[160];

#define mld_rej_uniform_table MLD_NAMESPACE(rej_uniform_table)
MLD_INTERNAL_DATA_DECLARATION const uint8_t mld_rej_uniform_table[256];
#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
#define mld_rej_uniform_eta_table MLD_NAMESPACE(rej_uniform_eta_table)
MLD_INTERNAL_DATA_DECLARATION const uint8_t mld_rej_uniform_eta_table[4096];
#endif

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLD_CONFIG_PARAMETER_SET == 44
#define mld_polyz_unpack_17_indices MLD_NAMESPACE(polyz_unpack_17_indices)
MLD_INTERNAL_DATA_DECLARATION const uint8_t mld_polyz_unpack_17_indices[64];
#endif
#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || \
    (MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_PARAMETER_SET == 87)
#define mld_polyz_unpack_19_indices MLD_NAMESPACE(polyz_unpack_19_indices)
MLD_INTERNAL_DATA_DECLARATION const uint8_t mld_polyz_unpack_19_indices[64];
#endif
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */


/*
 * Sampling 256 coefficients mod 15 using rejection sampling from 4 bits.
 * Expected number of required bytes: (256 * (16/15))/2 = 136.5 bytes.
 * We sample 1 block (=136 bytes) of SHAKE256_RATE output initially.
 * Sampling 2 blocks initially results in slightly worse performance.
 */
#define MLD_AARCH64_REJ_UNIFORM_ETA2_BUFLEN (1 * 136)
/*
 * Sampling 256 coefficients mod 9 using rejection sampling from 4 bits.
 * Expected number of required bytes: (256 * (16/9))/2 = 227.5 bytes.
 * We sample 2 blocks (=272 bytes) of SHAKE256_RATE output initially.
 */
#define MLD_AARCH64_REJ_UNIFORM_ETA4_BUFLEN (2 * 136)

#define mld_ntt_aarch64_asm MLD_NAMESPACE(ntt_aarch64_asm)
void mld_ntt_aarch64_asm(int32_t *r, const int32_t *zetas_l123456,
                         const int32_t *zetas_l78)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/ntt_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  /* check-magic: 8380417 == MLDSA_Q */
  requires(array_abs_bound(r, 0, MLDSA_N, 8380417))
  requires(zetas_l123456 == mld_aarch64_ntt_zetas_layer123456)
  requires(zetas_l78 == mld_aarch64_ntt_zetas_layer78)
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  /* check-magic: off */
  ensures(array_abs_bound(r, 0, MLDSA_N, 75423753))
  /* check-magic: on */
);

#define mld_intt_aarch64_asm MLD_NAMESPACE(intt_aarch64_asm)
void mld_intt_aarch64_asm(int32_t *r, const int32_t *zetas_l78,
                          const int32_t *zetas_l123456)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/intt_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(array_abs_bound(r, 0, MLDSA_N, 8380417))
  requires(zetas_l78 == mld_aarch64_intt_zetas_layer78)
  requires(zetas_l123456 == mld_aarch64_intt_zetas_layer123456)
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  /* check-magic: off */
  ensures(array_abs_bound(r, 0, MLDSA_N, 8380417))
  /* check-magic: on */
);

#define mld_rej_uniform_aarch64_asm MLD_NAMESPACE(rej_uniform_aarch64_asm)
MLD_MUST_CHECK_RETURN_VALUE
uint64_t mld_rej_uniform_aarch64_asm(int32_t *r, const uint8_t *buf,
                                     unsigned buflen, const uint8_t *table)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/rej_uniform_aarch64_asm.ml. */
__contract__(
  requires(buflen % 24 == 0)
  requires(memory_no_alias(buf, buflen))
  requires(table == mld_rej_uniform_table)
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(return_value <= MLDSA_N)
  ensures(array_bound(r, 0, (unsigned) return_value, 0, MLDSA_Q))
);

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
#define mld_rej_uniform_eta2_aarch64_asm \
  MLD_NAMESPACE(rej_uniform_eta2_aarch64_asm)
MLD_MUST_CHECK_RETURN_VALUE
uint64_t mld_rej_uniform_eta2_aarch64_asm(int32_t *r, const uint8_t *buf,
                                          unsigned buflen, const uint8_t *table)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/rej_uniform_eta2_aarch64_asm.ml */
__contract__(
  requires(buflen % 8 == 0)
  requires(buflen >= 8)
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(buf, buflen))
  requires(table == mld_rej_uniform_eta_table)
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(return_value <= MLDSA_N)
  /* check-magic: 3 == 2 + 1 (asm is eta=2-specific) */
  ensures(array_abs_bound(r, 0, return_value, 3))
);

#define mld_rej_uniform_eta4_aarch64_asm \
  MLD_NAMESPACE(rej_uniform_eta4_aarch64_asm)
MLD_MUST_CHECK_RETURN_VALUE
uint64_t mld_rej_uniform_eta4_aarch64_asm(int32_t *r, const uint8_t *buf,
                                          unsigned buflen, const uint8_t *table)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/rej_uniform_eta4_aarch64_asm.ml */
__contract__(
  requires(buflen % 8 == 0)
  requires(buflen >= 8)
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(buf, buflen))
  requires(table == mld_rej_uniform_eta_table)
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(return_value <= MLDSA_N)
  /* check-magic: 5 == 4 + 1 (asm is eta=4-specific) */
  ensures(array_abs_bound(r, 0, return_value, 5))
);
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
#define mld_poly_decompose_32_aarch64_asm \
  MLD_NAMESPACE(poly_decompose_32_aarch64_asm)
void mld_poly_decompose_32_aarch64_asm(int32_t *a1, int32_t *a0)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/poly_decompose_32_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(a1, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(a0, sizeof(int32_t) * MLDSA_N))
  requires(array_bound(a0, 0, MLDSA_N, 0, MLDSA_Q))
  assigns(memory_slice(a1, sizeof(int32_t) * MLDSA_N))
  assigns(memory_slice(a0, sizeof(int32_t) * MLDSA_N))
  /* check-magic: 16 == (MLDSA_Q - 1) / (2 * ((MLDSA_Q - 1) / 32)) */
  ensures(array_bound(a1, 0, MLDSA_N, 0, 16))
  /* check-magic: 261889 == (MLDSA_Q - 1) / 32 + 1 */
  ensures(array_abs_bound(a0, 0, MLDSA_N, 261889))
);

#define mld_poly_decompose_88_aarch64_asm \
  MLD_NAMESPACE(poly_decompose_88_aarch64_asm)
void mld_poly_decompose_88_aarch64_asm(int32_t *a1, int32_t *a0)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/poly_decompose_88_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(a1, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(a0, sizeof(int32_t) * MLDSA_N))
  requires(array_bound(a0, 0, MLDSA_N, 0, MLDSA_Q))
  assigns(memory_slice(a1, sizeof(int32_t) * MLDSA_N))
  assigns(memory_slice(a0, sizeof(int32_t) * MLDSA_N))
  /* check-magic: 44 == (MLDSA_Q - 1) / (2 * ((MLDSA_Q - 1) / 88)) */
  ensures(array_bound(a1, 0, MLDSA_N, 0, 44))
  /* check-magic: 95233 == (MLDSA_Q - 1) / 88 + 1 */
  ensures(array_abs_bound(a0, 0, MLDSA_N, 95233))
);
#endif /* !MLD_CONFIG_NO_SIGN_API */

#define mld_poly_caddq_aarch64_asm MLD_NAMESPACE(poly_caddq_aarch64_asm)
void mld_poly_caddq_aarch64_asm(int32_t *a)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/poly_caddq_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(a, sizeof(int32_t) * MLDSA_N))
  requires(array_abs_bound(a, 0, MLDSA_N, MLDSA_Q))
  assigns(memory_slice(a, sizeof(int32_t) * MLDSA_N))
  ensures(array_bound(a, 0, MLDSA_N, 0, MLDSA_Q))
);

#if !defined(MLD_CONFIG_NO_VERIFY_API)
#define mld_poly_use_hint_32_aarch64_asm \
  MLD_NAMESPACE(poly_use_hint_32_aarch64_asm)
void mld_poly_use_hint_32_aarch64_asm(int32_t *a, const int32_t *h)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/poly_use_hint_32_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(a, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(h, sizeof(int32_t) * MLDSA_N))
  requires(array_bound(a, 0, MLDSA_N, 0, MLDSA_Q))
  requires(array_bound(h, 0, MLDSA_N, 0, 2))
  assigns(memory_slice(a, sizeof(int32_t) * MLDSA_N))
  ensures(array_bound(a, 0, MLDSA_N, 0, 16))
);

#define mld_poly_use_hint_88_aarch64_asm \
  MLD_NAMESPACE(poly_use_hint_88_aarch64_asm)
void mld_poly_use_hint_88_aarch64_asm(int32_t *a, const int32_t *h)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/poly_use_hint_88_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(a, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(h, sizeof(int32_t) * MLDSA_N))
  requires(array_bound(a, 0, MLDSA_N, 0, MLDSA_Q))
  requires(array_bound(h, 0, MLDSA_N, 0, 2))
  assigns(memory_slice(a, sizeof(int32_t) * MLDSA_N))
  ensures(array_bound(a, 0, MLDSA_N, 0, 44))
);
#endif /* !MLD_CONFIG_NO_VERIFY_API */

#define mld_poly_chknorm_aarch64_asm MLD_NAMESPACE(poly_chknorm_aarch64_asm)
MLD_MUST_CHECK_RETURN_VALUE
int mld_poly_chknorm_aarch64_asm(const int32_t *a, int32_t B)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/poly_chknorm_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(a, sizeof(int32_t) * MLDSA_N))
  /* HOL Light precondition: abs(ival(x i)) < 2^31, i.e., a[i] != INT32_MIN */
  requires(forall(k0, 0, MLDSA_N, a[k0] > INT32_MIN))
  ensures(return_value == 0 || return_value == 1)
  ensures((return_value == 0) == array_abs_bound(a, 0, MLDSA_N, B))
);

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || MLD_CONFIG_PARAMETER_SET == 44
#define mld_polyz_unpack_17_aarch64_asm \
  MLD_NAMESPACE(polyz_unpack_17_aarch64_asm)
void mld_polyz_unpack_17_aarch64_asm(int32_t *r, const uint8_t *buf,
                                     const uint8_t *indices)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/polyz_unpack_17_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(buf, 576))
  requires(indices == mld_polyz_unpack_17_indices)
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(array_bound(r, 0, MLDSA_N, -((1 << 17) - 1), (1 << 17) + 1))
);
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 44 \
        */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || \
    (MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_PARAMETER_SET == 87)
#define mld_polyz_unpack_19_aarch64_asm \
  MLD_NAMESPACE(polyz_unpack_19_aarch64_asm)
void mld_polyz_unpack_19_aarch64_asm(int32_t *r, const uint8_t *buf,
                                     const uint8_t *indices)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/polyz_unpack_19_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(buf, 640))
  requires(indices == mld_polyz_unpack_19_indices)
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(array_bound(r, 0, MLDSA_N, -((1 << 19) - 1), (1 << 19) + 1))
);
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 65 \
          || MLD_CONFIG_PARAMETER_SET == 87 */
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API) || \
    defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
#define mld_poly_pointwise_montgomery_aarch64_asm \
  MLD_NAMESPACE(poly_pointwise_montgomery_aarch64_asm)
void mld_poly_pointwise_montgomery_aarch64_asm(int32_t *a, const int32_t *b)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/aarch64/proofs/pointwise_montgomery_aarch64_asm.ml */
__contract__(
  requires(memory_no_alias(a, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(b, sizeof(int32_t) * MLDSA_N))
  /* Input bound MLD_NTT_BOUND = 9 * MLD_FQMUL_BOUND, the guaranteed bound of
   * any forward NTT implementation. Hardcoded here to keep this header free
   * of poly.h. */
  /* check-magic: 94279698 == 9 * ((5 * MLDSA_Q + 3) / 4) */
  requires(array_abs_bound(a, 0, MLDSA_N, 94279698))
  requires(array_abs_bound(b, 0, MLDSA_N, 94279698))
  assigns(memory_slice(a, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(a, 0, MLDSA_N, 8380417))
);
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API || \
          MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

#define mld_polyvecl_pointwise_acc_montgomery_l4_aarch64_asm \
  MLD_NAMESPACE(polyvecl_pointwise_acc_montgomery_l4_aarch64_asm)
void mld_polyvecl_pointwise_acc_montgomery_l4_aarch64_asm(
    int32_t *r, const int32_t a[4][MLDSA_N], const int32_t b[4][MLDSA_N])
/* This must be kept in sync with the HOL-Light specification
 * in
 * proofs/hol_light/aarch64/proofs/mld_polyvecl_pointwise_acc_montgomery_l4_aarch64_asm.ml
 */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(a, sizeof(int32_t) * 4 * MLDSA_N))
  requires(memory_no_alias(b, sizeof(int32_t) * 4 * MLDSA_N))
  requires(forall(l0, 0, 4, array_abs_bound(a[l0], 0, MLDSA_N, 8380417)))
  /* check-magic: 94279698 == 9 * ((5 * MLDSA_Q + 3) / 4) */
  requires(forall(l1, 0, 4, array_abs_bound(b[l1], 0, MLDSA_N, 94279698)))
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(r, 0, MLDSA_N, 8380417))
);

#define mld_polyvecl_pointwise_acc_montgomery_l5_aarch64_asm \
  MLD_NAMESPACE(polyvecl_pointwise_acc_montgomery_l5_aarch64_asm)
void mld_polyvecl_pointwise_acc_montgomery_l5_aarch64_asm(
    int32_t *r, const int32_t a[5][MLDSA_N], const int32_t b[5][MLDSA_N])
/* This must be kept in sync with the HOL-Light specification
 * in
 * proofs/hol_light/aarch64/proofs/mld_polyvecl_pointwise_acc_montgomery_l5_aarch64_asm.ml
 */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(a, sizeof(int32_t) * 5 * MLDSA_N))
  requires(memory_no_alias(b, sizeof(int32_t) * 5 * MLDSA_N))
  requires(forall(l0, 0, 5, array_abs_bound(a[l0], 0, MLDSA_N, 8380417)))
  /* check-magic: 94279698 == 9 * ((5 * MLDSA_Q + 3) / 4) */
  requires(forall(l1, 0, 5, array_abs_bound(b[l1], 0, MLDSA_N, 94279698)))
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(r, 0, MLDSA_N, 8380417))
);

#define mld_polyvecl_pointwise_acc_montgomery_l7_aarch64_asm \
  MLD_NAMESPACE(polyvecl_pointwise_acc_montgomery_l7_aarch64_asm)
void mld_polyvecl_pointwise_acc_montgomery_l7_aarch64_asm(
    int32_t *r, const int32_t a[7][MLDSA_N], const int32_t b[7][MLDSA_N])
/* This must be kept in sync with the HOL-Light specification
 * in
 * proofs/hol_light/aarch64/proofs/mld_polyvecl_pointwise_acc_montgomery_l7_aarch64_asm.ml
 */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(a, sizeof(int32_t) * 7 * MLDSA_N))
  requires(memory_no_alias(b, sizeof(int32_t) * 7 * MLDSA_N))
  requires(forall(l0, 0, 7, array_abs_bound(a[l0], 0, MLDSA_N, 8380417)))
  /* check-magic: 94279698 == 9 * ((5 * MLDSA_Q + 3) / 4) */
  requires(forall(l1, 0, 7, array_abs_bound(b[l1], 0, MLDSA_N, 94279698)))
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(r, 0, MLDSA_N, 8380417))
);

#endif /* !MLD_NATIVE_AARCH64_SRC_ARITH_NATIVE_AARCH64_H */
