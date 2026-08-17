/*
 * Copyright (c) The mlkem-native project authors
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLD_NATIVE_X86_64_SRC_ARITH_NATIVE_X86_64_H
#define MLD_NATIVE_X86_64_SRC_ARITH_NATIVE_X86_64_H
#include "../../../common.h"

#include "consts.h"

#define MLD_AVX2_REJ_UNIFORM_BUFLEN \
  (5 * 168) /* REJ_UNIFORM_NBLOCKS * SHAKE128_RATE */


/*
 * Sampling 256 coefficients mod 15 using rejection sampling from 4 bits.
 * Expected number of required bytes: (256 * (16/15))/2 = 136.5 bytes.
 * We sample 1 block (=136 bytes) of SHAKE256_RATE output initially.
 * Sampling 2 blocks initially results in slightly worse performance.
 */
#define MLD_AVX2_REJ_UNIFORM_ETA2_BUFLEN (1 * 136)

/*
 * Sampling 256 coefficients mod 9 using rejection sampling from 4 bits.
 * Expected number of required bytes: (256 * (16/9))/2 = 227.5 bytes.
 * We sample 2 blocks (=272 bytes) of SHAKE256_RATE output initially.
 */
#define MLD_AVX2_REJ_UNIFORM_ETA4_BUFLEN (2 * 136)

#define mld_rej_uniform_table MLD_NAMESPACE(mld_rej_uniform_table)
MLD_INTERNAL_DATA_DECLARATION const uint8_t mld_rej_uniform_table[256][8];

#define mld_ntt_avx2_asm MLD_NAMESPACE(ntt_avx2_asm)
void mld_ntt_avx2_asm(int32_t *r, const int32_t *qdata)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/ntt_avx2_asm.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  /* check-magic: 8380417 == MLDSA_Q */
  requires(array_abs_bound(r, 0, MLDSA_N, 8380417))
  requires(qdata == mld_qdata)
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  /* check-magic: off */
  ensures(array_abs_bound(r, 0, MLDSA_N, 42035262))
  /* check-magic: on */
);

#define mld_invntt_avx2_asm MLD_NAMESPACE(invntt_avx2_asm)
void mld_invntt_avx2_asm(int32_t *r, const int32_t *qdata)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/intt_avx2_asm.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(array_abs_bound(r, 0, MLDSA_N, 8380417))
  requires(qdata == mld_qdata)
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  /* check-magic: off */
  ensures(array_abs_bound(r, 0, MLDSA_N, 6285313))
  /* check-magic: on */
);

#define mld_nttunpack_avx2_asm MLD_NAMESPACE(nttunpack_avx2_asm)
void mld_nttunpack_avx2_asm(int32_t *r)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/nttunpack_avx2_asm.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(array_abs_bound(r, 0, MLDSA_N, 8380417))
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  /* Output is a permutation of input: every output coefficient
   * is some input coefficient */
  ensures(forall(i, 0, MLDSA_N, exists(j, 0, MLDSA_N,
    r[i] == old(*(int32_t (*)[MLDSA_N])r)[j])))
);

#define mld_rej_uniform_avx2 MLD_NAMESPACE(mld_rej_uniform_avx2)
MLD_MUST_CHECK_RETURN_VALUE
unsigned mld_rej_uniform_avx2(int32_t *r,
                              const uint8_t buf[MLD_AVX2_REJ_UNIFORM_BUFLEN]);

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
#define mld_rej_uniform_eta2_avx2 MLD_NAMESPACE(mld_rej_uniform_eta2_avx2)
MLD_MUST_CHECK_RETURN_VALUE
unsigned mld_rej_uniform_eta2_avx2(
    int32_t *r, const uint8_t buf[MLD_AVX2_REJ_UNIFORM_ETA2_BUFLEN]);

#define mld_rej_uniform_eta4_avx2 MLD_NAMESPACE(mld_rej_uniform_eta4_avx2)
MLD_MUST_CHECK_RETURN_VALUE
unsigned mld_rej_uniform_eta4_avx2(
    int32_t *r, const uint8_t buf[MLD_AVX2_REJ_UNIFORM_ETA4_BUFLEN]);
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
#define mld_poly_decompose_32_avx2 MLD_NAMESPACE(mld_poly_decompose_32_avx2)
void mld_poly_decompose_32_avx2(int32_t *a1, int32_t *a0);

#define mld_poly_decompose_88_avx2 MLD_NAMESPACE(mld_poly_decompose_88_avx2)
void mld_poly_decompose_88_avx2(int32_t *a1, int32_t *a0);
#endif /* !MLD_CONFIG_NO_SIGN_API */

#define mld_poly_caddq_avx2_asm MLD_NAMESPACE(poly_caddq_avx2_asm)
void mld_poly_caddq_avx2_asm(int32_t *r)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/poly_caddq_avx2_asm.ml */
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(array_abs_bound(r, 0, MLDSA_N, MLDSA_Q))
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(array_bound(r, 0, MLDSA_N, 0, MLDSA_Q))
);

#if !defined(MLD_CONFIG_NO_VERIFY_API)
#define mld_poly_use_hint_32_avx2 MLD_NAMESPACE(mld_poly_use_hint_32_avx2)
void mld_poly_use_hint_32_avx2(int32_t *a, const int32_t *h);

#define mld_poly_use_hint_88_avx2 MLD_NAMESPACE(mld_poly_use_hint_88_avx2)
void mld_poly_use_hint_88_avx2(int32_t *a, const int32_t *h);
#endif /* !MLD_CONFIG_NO_VERIFY_API */

#define mld_poly_chknorm_avx2 MLD_NAMESPACE(mld_poly_chknorm_avx2)
MLD_MUST_CHECK_RETURN_VALUE
int mld_poly_chknorm_avx2(const int32_t *a, int32_t B);

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
#define mld_polyz_unpack_17_avx2 MLD_NAMESPACE(mld_polyz_unpack_17_avx2)
void mld_polyz_unpack_17_avx2(int32_t *r, const uint8_t *a);

#define mld_polyz_unpack_19_avx2 MLD_NAMESPACE(mld_polyz_unpack_19_avx2)
void mld_polyz_unpack_19_avx2(int32_t *r, const uint8_t *a);
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */

#define mld_pointwise_avx2_asm MLD_NAMESPACE(pointwise_avx2_asm)
void mld_pointwise_avx2_asm(int32_t *a, const int32_t *b, const int32_t *qdata)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/pointwise_avx2_asm.ml */
__contract__(
  requires(memory_no_alias(a, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(b, sizeof(int32_t) * MLDSA_N))
  /* Input bound MLD_NTT_BOUND = 9 * MLD_FQMUL_BOUND, the guaranteed bound of
   * any forward NTT implementation. Hardcoded here to keep this header free
   * of poly.h. */
  /* check-magic: 94279698 == 9 * ((5 * MLDSA_Q + 3) / 4) */
  requires(array_abs_bound(a, 0, MLDSA_N, 94279698))
  requires(array_abs_bound(b, 0, MLDSA_N, 94279698))
  requires(qdata == mld_qdata)
  assigns(memory_slice(a, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(a, 0, MLDSA_N, 8380417))
);

#define mld_pointwise_acc_l4_avx2_asm MLD_NAMESPACE(pointwise_acc_l4_avx2_asm)
void mld_pointwise_acc_l4_avx2_asm(int32_t c[MLDSA_N],
                                   const int32_t a[4][MLDSA_N],
                                   const int32_t b[4][MLDSA_N],
                                   const int32_t *qdata)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/pointwise_acc_l4_avx2_asm.ml */
__contract__(
  requires(memory_no_alias(c, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(a, sizeof(int32_t) * 4 * MLDSA_N))
  requires(memory_no_alias(b, sizeof(int32_t) * 4 * MLDSA_N))
  requires(forall(l0, 0, 4, array_abs_bound(a[l0], 0, MLDSA_N, 8380417)))
  /* check-magic: 94279698 == 9 * ((5 * MLDSA_Q + 3) / 4) */
  requires(forall(l1, 0, 4, array_abs_bound(b[l1], 0, MLDSA_N, 94279698)))
  requires(qdata == mld_qdata)
  assigns(memory_slice(c, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(c, 0, MLDSA_N, 8380417))
);

#define mld_pointwise_acc_l5_avx2_asm MLD_NAMESPACE(pointwise_acc_l5_avx2_asm)
void mld_pointwise_acc_l5_avx2_asm(int32_t c[MLDSA_N],
                                   const int32_t a[5][MLDSA_N],
                                   const int32_t b[5][MLDSA_N],
                                   const int32_t *qdata)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/pointwise_acc_l5_avx2_asm.ml */
__contract__(
  requires(memory_no_alias(c, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(a, sizeof(int32_t) * 5 * MLDSA_N))
  requires(memory_no_alias(b, sizeof(int32_t) * 5 * MLDSA_N))
  requires(forall(l0, 0, 5, array_abs_bound(a[l0], 0, MLDSA_N, 8380417)))
  /* check-magic: 94279698 == 9 * ((5 * MLDSA_Q + 3) / 4) */
  requires(forall(l1, 0, 5, array_abs_bound(b[l1], 0, MLDSA_N, 94279698)))
  requires(qdata == mld_qdata)
  assigns(memory_slice(c, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(c, 0, MLDSA_N, 8380417))
);

#define mld_pointwise_acc_l7_avx2_asm MLD_NAMESPACE(pointwise_acc_l7_avx2_asm)
void mld_pointwise_acc_l7_avx2_asm(int32_t c[MLDSA_N],
                                   const int32_t a[7][MLDSA_N],
                                   const int32_t b[7][MLDSA_N],
                                   const int32_t *qdata)
/* This must be kept in sync with the HOL-Light specification
 * in proofs/hol_light/x86_64/proofs/pointwise_acc_l7_avx2_asm.ml */
__contract__(
  requires(memory_no_alias(c, sizeof(int32_t) * MLDSA_N))
  requires(memory_no_alias(a, sizeof(int32_t) * 7 * MLDSA_N))
  requires(memory_no_alias(b, sizeof(int32_t) * 7 * MLDSA_N))
  requires(forall(l0, 0, 7, array_abs_bound(a[l0], 0, MLDSA_N, 8380417)))
  /* check-magic: 94279698 == 9 * ((5 * MLDSA_Q + 3) / 4) */
  requires(forall(l1, 0, 7, array_abs_bound(b[l1], 0, MLDSA_N, 94279698)))
  requires(qdata == mld_qdata)
  assigns(memory_slice(c, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(c, 0, MLDSA_N, 8380417))
);

#endif /* !MLD_NATIVE_X86_64_SRC_ARITH_NATIVE_X86_64_H */
