/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLD_PACKING_H
#define MLD_PACKING_H

#include "polyvec.h"
#include "polyvec_lazy.h"

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
#define mld_pack_sk_s1 MLD_NAMESPACE_KL(pack_sk_s1)
/**
 * Bit-pack the s1 component into the secret key.
 *
 * @param[out] sk Output byte array.
 * @param[in]  s1 Pointer to vector s1.
 */
MLD_INTERNAL_API
void mld_pack_sk_s1(uint8_t sk[MLDSA_CRYPTO_SECRETKEYBYTES],
                    const mld_polyvecl *s1)
__contract__(
  requires(memory_no_alias(sk, MLDSA_CRYPTO_SECRETKEYBYTES))
  requires(memory_no_alias(s1, sizeof(mld_polyvecl)))
  requires(forall(k1, 0, MLDSA_L,
    array_abs_bound(s1->vec[k1].coeffs, 0, MLDSA_N, MLDSA_ETA + 1)))
  assigns(memory_slice(sk, MLDSA_CRYPTO_SECRETKEYBYTES))
);

#define mld_pack_sk_rho_key_tr_s2 MLD_NAMESPACE_KL(pack_sk_rho_key_tr_s2)
/**
 * Bit-pack rho, key, tr, s2 into the secret key.
 *
 * s1 must already be packed via mld_pack_sk_s1, and t0 via
 * mld_compute_pack_t0_t1.
 *
 * @param[out] sk  Output byte array.
 * @param[in]  rho Byte array containing rho.
 * @param[in]  tr  Byte array containing tr.
 * @param[in]  key Byte array containing key.
 * @param[in]  s2  Pointer to vector s2.
 */
MLD_INTERNAL_API
void mld_pack_sk_rho_key_tr_s2(uint8_t sk[MLDSA_CRYPTO_SECRETKEYBYTES],
                               const uint8_t rho[MLDSA_SEEDBYTES],
                               const uint8_t tr[MLDSA_TRBYTES],
                               const uint8_t key[MLDSA_SEEDBYTES],
                               const mld_polyveck *s2)
__contract__(
  requires(memory_no_alias(sk, MLDSA_CRYPTO_SECRETKEYBYTES))
  requires(memory_no_alias(rho, MLDSA_SEEDBYTES))
  requires(memory_no_alias(tr, MLDSA_TRBYTES))
  requires(memory_no_alias(key, MLDSA_SEEDBYTES))
  requires(memory_no_alias(s2, sizeof(mld_polyveck)))
  requires(forall(k2, 0, MLDSA_K,
    array_abs_bound(s2->vec[k2].coeffs, 0, MLDSA_N, MLDSA_ETA + 1)))
  assigns(memory_slice(sk, MLDSA_CRYPTO_SECRETKEYBYTES))
);
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */


#if !defined(MLD_CONFIG_NO_SIGN_API)
#define mld_pack_sig_c MLD_NAMESPACE_KL(pack_sig_c)
/**
 * Bit-pack challenge c into sig = (c, z, h).
 *
 * @param[out] sig Output byte array.
 * @param[in]  c   Pointer to challenge hash.
 */
MLD_INTERNAL_API
void mld_pack_sig_c(uint8_t sig[MLDSA_CRYPTO_BYTES],
                    const uint8_t c[MLDSA_CTILDEBYTES])
__contract__(
  requires(memory_no_alias(sig, MLDSA_CRYPTO_BYTES))
  requires(memory_no_alias(c, MLDSA_CTILDEBYTES))
  assigns(memory_slice(sig, MLDSA_CRYPTO_BYTES))
);

#define mld_pack_sig_h MLD_NAMESPACE_KL(pack_sig_h)
/**
 * Compute hints from (w0, w1) and pack them into the hint section of sig.
 *
 * @param[in,out] sig Byte array containing signature.
 * @param[in]     w0  Pointer to low part of input vector.
 * @param[in]     w1  Pointer to high part of input vector.
 *
 * @retval 0            Success.
 * @retval MLD_ERR_FAIL The total number of hints exceeds MLDSA_OMEGA. In this
 *                      case the hint section of sig is left in a
 *                      partially-written state and the caller must reject the
 *                      signature.
 */
MLD_INTERNAL_API
MLD_MUST_CHECK_RETURN_VALUE
int mld_pack_sig_h(uint8_t sig[MLDSA_CRYPTO_BYTES], const mld_polyveck *w0,
                   const mld_polyveck *w1)
__contract__(
  requires(memory_no_alias(sig, MLDSA_CRYPTO_BYTES))
  requires(memory_no_alias(w0, sizeof(mld_polyveck)))
  requires(memory_no_alias(w1, sizeof(mld_polyveck)))
  assigns(memory_slice(sig + MLDSA_SIG_H_OFFSET, MLDSA_POLYVECH_PACKEDBYTES))
  ensures(return_value == 0 || return_value == MLD_ERR_FAIL)
);

#define mld_pack_sig_z MLD_NAMESPACE_KL(pack_sig_z)
/**
 * Bit-pack single polynomial of z component of sig = (c, z, h).
 *
 * The c and h components are packed separately using mld_pack_sig_c and
 * mld_pack_sig_h.
 *
 * @param[in,out] sig Output byte array.
 * @param[in]     zi  Pointer to a single polynomial in z.
 * @param         i   Index of zi in vector z.
 */
MLD_INTERNAL_API
void mld_pack_sig_z(uint8_t sig[MLDSA_CRYPTO_BYTES], const mld_poly *zi,
                    unsigned i)
__contract__(
  requires(memory_no_alias(sig, MLDSA_CRYPTO_BYTES))
  requires(memory_no_alias(zi, sizeof(mld_poly)))
  requires(i < MLDSA_L)
  requires(array_bound(zi->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
  assigns(memory_slice(sig, MLDSA_CRYPTO_BYTES))
);
#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_VERIFY_API)
#define mld_unpack_pk_t1 MLD_NAMESPACE_KL(unpack_pk_t1)
/**
 * Unpack a single polynomial of the t1 component of a public key
 * pk = (rho, t1).
 *
 * @param[out] t1 Pointer to output polynomial t1[i].
 * @param[in]  pk Byte array containing bit-packed pk.
 * @param      i  Row index, must be < MLDSA_K.
 */
MLD_INTERNAL_API
void mld_unpack_pk_t1(mld_poly *t1,
                      const uint8_t pk[MLDSA_CRYPTO_PUBLICKEYBYTES],
                      unsigned int i)
__contract__(
  requires(memory_no_alias(pk, MLDSA_CRYPTO_PUBLICKEYBYTES))
  requires(memory_no_alias(t1, sizeof(mld_poly)))
  requires(i < MLDSA_K)
  assigns(memory_slice(t1, sizeof(mld_poly)))
  ensures(array_bound(t1->coeffs, 0, MLDSA_N, 0, 1 << 10))
);
#endif /* !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
#define mld_unpack_sk MLD_NAMESPACE_KL(unpack_sk)
/**
 * Unpack secret key sk = (rho, tr, key, t0, s1, s2).
 *
 * NOTE: In REDUCE_RAM mode, s1/s2/t0 borrow from sk rather than copying.
 *
 * @param[out] rho Output byte array for rho.
 * @param[out] tr  Output byte array for tr.
 * @param[out] key Output byte array for key.
 * @param[out] t0  Pointer to output vector t0.
 * @param[out] s1  Pointer to output vector s1.
 * @param[out] s2  Pointer to output vector s2.
 * @param[in]  sk  Byte array containing bit-packed sk.
 */
MLD_INTERNAL_API
void mld_unpack_sk(uint8_t rho[MLDSA_SEEDBYTES], uint8_t tr[MLDSA_TRBYTES],
                   uint8_t key[MLDSA_SEEDBYTES], mld_sk_t0hat *t0,
                   mld_sk_s1hat *s1, mld_sk_s2hat *s2,
                   const uint8_t sk[MLDSA_CRYPTO_SECRETKEYBYTES])
__contract__(
  requires(memory_no_alias(rho, MLDSA_SEEDBYTES))
  requires(memory_no_alias(tr, MLDSA_TRBYTES))
  requires(memory_no_alias(key, MLDSA_SEEDBYTES))
  requires(memory_no_alias(t0, sizeof(mld_sk_t0hat)))
  requires(memory_no_alias(s1, sizeof(mld_sk_s1hat)))
  requires(memory_no_alias(s2, sizeof(mld_sk_s2hat)))
  requires(memory_no_alias(sk, MLDSA_CRYPTO_SECRETKEYBYTES))
  assigns(memory_slice(rho, MLDSA_SEEDBYTES))
  assigns(memory_slice(tr, MLDSA_TRBYTES))
  assigns(memory_slice(key, MLDSA_SEEDBYTES))
  assigns(memory_slice(t0, sizeof(mld_sk_t0hat)))
  assigns(memory_slice(s1, sizeof(mld_sk_s1hat)))
  assigns(memory_slice(s2, sizeof(mld_sk_s2hat)))
  MLD_IF_NOT_REDUCE_RAM(
    ensures(forall(k0, 0, MLDSA_K,
      array_abs_bound(t0->vec.vec[k0].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
    ensures(forall(k1, 0, MLDSA_L,
      array_abs_bound(s1->vec.vec[k1].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
    ensures(forall(k2, 0, MLDSA_K,
      array_abs_bound(s2->vec.vec[k2].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
  )
  MLD_IF_REDUCE_RAM(
    ensures(s1->packed == old(sk) + MLDSA_SK_S1_OFFSET)
    ensures(s2->packed == old(sk) + MLDSA_SK_S2_OFFSET)
    ensures(t0->packed == old(sk) + MLDSA_SK_T0_OFFSET)
  )
);
#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_VERIFY_API)
#define mld_sig_unpack_hints MLD_NAMESPACE_KL(sig_unpack_hints)
/**
 * Decode and validate a single row of the hint vector h from a signature
 * buffer.
 *
 * The hint encoding is shared across all rows (a count array followed by a
 * single index list), so this function performs the validation relevant to
 * row i:
 *   - the i'th hint count is non-decreasing and bounded by MLDSA_OMEGA;
 *   - the indices for row i are strictly ascending;
 *   - on i == MLDSA_K - 1, the trailing index slots are zero.
 *
 * Callers must invoke this for every i in [0, 1, .., MLDSA_K - 1]; if any
 * call returns MLD_ERR_FAIL the encoding is malformed and the signature must
 * be rejected.
 *
 * @param[out] h   Pointer to output polynomial h[i].
 * @param[in]  sig Signature buffer.
 * @param      i   Row index, must be < MLDSA_K.
 *
 * @retval 0            Hints were decoded successfully.
 * @retval MLD_ERR_FAIL Hints are malformed.
 */
MLD_INTERNAL_API
MLD_MUST_CHECK_RETURN_VALUE
int mld_sig_unpack_hints(mld_poly *h, const uint8_t sig[MLDSA_CRYPTO_BYTES],
                         unsigned int i)
__contract__(
  requires(memory_no_alias(sig, MLDSA_CRYPTO_BYTES))
  requires(memory_no_alias(h, sizeof(mld_poly)))
  requires(i < MLDSA_K)
  assigns(memory_slice(h, sizeof(mld_poly)))
  ensures(return_value == 0 || return_value == MLD_ERR_FAIL)
  ensures(return_value == 0 ==> array_bound(h->coeffs, 0, MLDSA_N, 0, 2))
);
#endif /* !MLD_CONFIG_NO_VERIFY_API */

#endif /* !MLD_PACKING_H */
