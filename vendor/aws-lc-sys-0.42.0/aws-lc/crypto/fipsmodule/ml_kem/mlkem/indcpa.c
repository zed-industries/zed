/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/* References
 * ==========
 *
 * - [FIPS203]
 *   FIPS 203 Module-Lattice-Based Key-Encapsulation Mechanism Standard
 *   National Institute of Standards and Technology
 *   https://csrc.nist.gov/pubs/fips/203/final
 *
 * - [REF]
 *   CRYSTALS-Kyber C reference implementation
 *   Bos, Ducas, Kiltz, Lepoint, Lyubashevsky, Schanck, Schwabe, Seiler, Stehlé
 *   https://github.com/pq-crystals/kyber/tree/main/ref
 */

#include "indcpa.h"

#include "debug.h"
#include "randombytes.h"
#include "sampling.h"
#include "symmetric.h"

/* Parameter set namespacing
 * This is to facilitate building multiple instances
 * of mlkem-native (e.g. with varying parameter sets)
 * within a single compilation unit. */
#define mlk_pack_pk MLK_ADD_PARAM_SET(mlk_pack_pk)
#define mlk_unpack_pk MLK_ADD_PARAM_SET(mlk_unpack_pk)
#define mlk_pack_sk MLK_ADD_PARAM_SET(mlk_pack_sk)
#define mlk_unpack_sk MLK_ADD_PARAM_SET(mlk_unpack_sk)
#define mlk_pack_ciphertext MLK_ADD_PARAM_SET(mlk_pack_ciphertext)
#define mlk_unpack_ciphertext MLK_ADD_PARAM_SET(mlk_unpack_ciphertext)
#define mlk_matvec_mul MLK_ADD_PARAM_SET(mlk_matvec_mul)
#define mlk_polyvec_permute_bitrev_to_custom \
  MLK_ADD_PARAM_SET(mlk_polyvec_permute_bitrev_to_custom)
#define mlk_polymat_permute_bitrev_to_custom \
  MLK_ADD_PARAM_SET(mlk_polymat_permute_bitrev_to_custom)
/* End of parameter set namespacing */

/*************************************************
 * Name:        mlk_pack_pk
 *
 * Description: Serialize the public key as concatenation of the
 *              serialized vector of polynomials pk
 *              and the public seed used to generate the matrix A.
 *
 * Arguments:   uint8_t *r: pointer to the output serialized public key
 *              mlk_polyvec pk: pointer to the input public-key mlk_polyvec.
 *                Must have coefficients within [0,..,q-1].
 *              const uint8_t *seed: pointer to the input public seed
 *
 * Specification:
 * Implements @[FIPS203, Algorithm 13 (K-PKE.KeyGen), L19]
 *
 **************************************************/
static void mlk_pack_pk(uint8_t r[MLKEM_INDCPA_PUBLICKEYBYTES],
                        const mlk_polyvec *pk,
                        const uint8_t seed[MLKEM_SYMBYTES])
{
  mlk_assert_bound_2d(pk->vec, MLKEM_K, MLKEM_N, 0, MLKEM_Q);
  mlk_polyvec_tobytes(r, pk);
  mlk_memcpy(r + MLKEM_POLYVECBYTES, seed, MLKEM_SYMBYTES);
}

/*************************************************
 * Name:        mlk_unpack_pk
 *
 * Description: De-serialize public key from a byte array;
 *              approximate inverse of mlk_pack_pk
 *
 * Arguments:   - mlk_polyvec pk: pointer to output public-key polynomial
 *                vector Coefficients will be normalized to [0,..,q-1].
 *              - uint8_t *seed: pointer to output seed to generate matrix A
 *              - const uint8_t *packedpk: pointer to input serialized public
 *                  key.
 *
 * Specification:
 * Implements @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L2-3]
 *
 **************************************************/
static void mlk_unpack_pk(mlk_polyvec *pk, uint8_t seed[MLKEM_SYMBYTES],
                          const uint8_t packedpk[MLKEM_INDCPA_PUBLICKEYBYTES])
{
  mlk_polyvec_frombytes(pk, packedpk);
  mlk_memcpy(seed, packedpk + MLKEM_POLYVECBYTES, MLKEM_SYMBYTES);

  /* NOTE: If a modulus check was conducted on the PK, we know at this
   * point that the coefficients of `pk` are unsigned canonical. The
   * specifications and proofs, however, do _not_ assume this, and instead
   * work with the easily provable bound by MLKEM_UINT12_LIMIT. */
}

/*************************************************
 * Name:        mlk_pack_sk
 *
 * Description: Serialize the secret key
 *
 * Arguments:   - uint8_t *r: pointer to output serialized secret key
 *              - mlk_polyvec sk: pointer to input vector of polynomials
 *                (secret key)
 *
 * Specification:
 * Implements @[FIPS203, Algorithm 13 (K-PKE.KeyGen), L20]
 *
 **************************************************/
static void mlk_pack_sk(uint8_t r[MLKEM_INDCPA_SECRETKEYBYTES],
                        const mlk_polyvec *sk)
{
  mlk_assert_bound_2d(sk->vec, MLKEM_K, MLKEM_N, 0, MLKEM_Q);
  mlk_polyvec_tobytes(r, sk);
}

/*************************************************
 * Name:        mlk_unpack_sk
 *
 * Description: De-serialize the secret key; inverse of mlk_pack_sk
 *
 * Arguments:   - mlk_polyvec sk: pointer to output vector of polynomials
 *                (secret key)
 *              - const uint8_t *packedsk: pointer to input serialized secret
 *                key
 *
 * Specification:
 * Implements @[FIPS203, Algorithm 15 (K-PKE.Decrypt), L5]
 *
 **************************************************/
static void mlk_unpack_sk(mlk_polyvec *sk,
                          const uint8_t packedsk[MLKEM_INDCPA_SECRETKEYBYTES])
{
  mlk_polyvec_frombytes(sk, packedsk);
}

/*************************************************
 * Name:        mlk_pack_ciphertext
 *
 * Description: Serialize the ciphertext as concatenation of the
 *              compressed and serialized vector of polynomials b
 *              and the compressed and serialized polynomial v
 *
 * Arguments:   uint8_t *r: pointer to the output serialized ciphertext
 *              mlk_poly *pk: pointer to the input vector of polynomials b
 *              mlk_poly *v: pointer to the input polynomial v
 *
 * Specification:
 * Implements @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L22-23]
 *
 **************************************************/
static void mlk_pack_ciphertext(uint8_t r[MLKEM_INDCPA_BYTES],
                                const mlk_polyvec *b, mlk_poly *v)
{
  mlk_polyvec_compress_du(r, b);
  mlk_poly_compress_dv(r + MLKEM_POLYVECCOMPRESSEDBYTES_DU, v);
}

/*************************************************
 * Name:        mlk_unpack_ciphertext
 *
 * Description: De-serialize and decompress ciphertext from a byte array;
 *              approximate inverse of mlk_pack_ciphertext
 *
 * Arguments:   - mlk_polyvec b: pointer to the output vector of polynomials b
 *              - mlk_poly *v: pointer to the output polynomial v
 *              - const uint8_t *c: pointer to the input serialized ciphertext
 *
 * Specification:
 * Implements @[FIPS203, Algorithm 15 (K-PKE.Decrypt), L1-4]
 *
 **************************************************/
static void mlk_unpack_ciphertext(mlk_polyvec *b, mlk_poly *v,
                                  const uint8_t c[MLKEM_INDCPA_BYTES])
{
  mlk_polyvec_decompress_du(b, c);
  mlk_poly_decompress_dv(v, c + MLKEM_POLYVECCOMPRESSEDBYTES_DU);
}

/* Helper function to ensure that the polynomial entries in the output
 * of gen_matrix use the standard (bitreversed) ordering of coefficients.
 * No-op unless a native backend with a custom ordering is used.
 *
 * We don't inline this into gen_matrix to avoid having to split the CBMC
 * proof for gen_matrix based on MLK_USE_NATIVE_NTT_CUSTOM_ORDER. */
static void mlk_polyvec_permute_bitrev_to_custom(mlk_polyvec *v)
__contract__(
  /* We don't specify that this should be a permutation, but only
   * that it does not change the bound established at the end of mlk_gen_matrix. */
  requires(memory_no_alias(v, sizeof(mlk_polyvec)))
  requires(forall(x, 0, MLKEM_K,
    array_bound(v->vec[x].coeffs, 0, MLKEM_N, 0, MLKEM_Q)))
  assigns(memory_slice(v, sizeof(mlk_polyvec)))
  ensures(forall(x, 0, MLKEM_K,
    array_bound(v->vec[x].coeffs, 0, MLKEM_N, 0, MLKEM_Q))))
{
#if defined(MLK_USE_NATIVE_NTT_CUSTOM_ORDER)
  unsigned i;
  for (i = 0; i < MLKEM_K; i++)
  __loop__(
     assigns(i, memory_slice(v, sizeof(mlk_polyvec)))
     invariant(i <= MLKEM_K)
     invariant(forall(x, 0, MLKEM_K,
       array_bound(v->vec[x].coeffs, 0, MLKEM_N, 0, MLKEM_Q))))
  {
    mlk_poly_permute_bitrev_to_custom(v->vec[i].coeffs);
  }
#else  /* MLK_USE_NATIVE_NTT_CUSTOM_ORDER */
  /* Nothing to do */
  (void)v;
#endif /* !MLK_USE_NATIVE_NTT_CUSTOM_ORDER */
}

static void mlk_polymat_permute_bitrev_to_custom(mlk_polymat *a)
__contract__(
  /* We don't specify that this should be a permutation, but only
   * that it does not change the bound established at the end of mlk_gen_matrix. */
  requires(memory_no_alias(a, sizeof(mlk_polymat)))
  requires(forall(x, 0, MLKEM_K, forall(y, 0, MLKEM_K,
    array_bound(a->vec[x].vec[y].coeffs, 0, MLKEM_N, 0, MLKEM_Q))))
  assigns(memory_slice(a, sizeof(mlk_polymat)))
  ensures(forall(x, 0, MLKEM_K, forall(y, 0, MLKEM_K,
    array_bound(a->vec[x].vec[y].coeffs, 0, MLKEM_N, 0, MLKEM_Q)))))
{
  unsigned i;
  for (i = 0; i < MLKEM_K; i++)
  __loop__(
     assigns(i, memory_slice(a, sizeof(mlk_polymat)))
     invariant(i <= MLKEM_K)
     invariant(forall(x, 0, MLKEM_K, forall(y, 0, MLKEM_K,
       array_bound(a->vec[x].vec[y].coeffs, 0, MLKEM_N, 0, MLKEM_Q)))))
  {
    mlk_polyvec_permute_bitrev_to_custom(&a->vec[i]);
  }
}

/* Reference: `gen_matrix()` in the reference implementation @[REF].
 *            - We use a special subroutine to generate 4 polynomials
 *              at a time, to be able to leverage batched Keccak-f1600
 *              implementations. The reference implementation generates
 *              one matrix entry a time.
 *
 * Not static for benchmarking */
MLK_INTERNAL_API
void mlk_gen_matrix(mlk_polymat *a, const uint8_t seed[MLKEM_SYMBYTES],
                    int transposed)
{
  unsigned i, j;
  MLK_ALIGN uint8_t seed_ext[4][MLK_ALIGN_UP(MLKEM_SYMBYTES + 2)];

  for (j = 0; j < 4; j++)
  {
    mlk_memcpy(seed_ext[j], seed, MLKEM_SYMBYTES);
  }

#if !defined(MLK_CONFIG_SERIAL_FIPS202_ONLY)
  /* Sample 4 matrix entries a time. */
  for (i = 0; i < (MLKEM_K * MLKEM_K / 4) * 4; i += 4)
  {
    for (j = 0; j < 4; j++)
    {
      uint8_t x, y;
      /* MLKEM_K <= 4, so the values fit in uint8_t. */
      x = (uint8_t)((i + j) / MLKEM_K);
      y = (uint8_t)((i + j) % MLKEM_K);
      if (transposed)
      {
        seed_ext[j][MLKEM_SYMBYTES + 0] = x;
        seed_ext[j][MLKEM_SYMBYTES + 1] = y;
      }
      else
      {
        seed_ext[j][MLKEM_SYMBYTES + 0] = y;
        seed_ext[j][MLKEM_SYMBYTES + 1] = x;
      }
    }

    mlk_poly_rej_uniform_x4(&a->vec[i / MLKEM_K].vec[i % MLKEM_K],
                            &a->vec[(i + 1) / MLKEM_K].vec[(i + 1) % MLKEM_K],
                            &a->vec[(i + 2) / MLKEM_K].vec[(i + 2) % MLKEM_K],
                            &a->vec[(i + 3) / MLKEM_K].vec[(i + 3) % MLKEM_K],
                            seed_ext);
  }
#else  /* !MLK_CONFIG_SERIAL_FIPS202_ONLY */
  /* When using serial FIPS202, sample all entries individually. */
  i = 0;
#endif /* MLK_CONFIG_SERIAL_FIPS202_ONLY */

  /* For MLKEM_K == 3, sample the last entry individually.
   * When MLK_CONFIG_SERIAL_FIPS202_ONLY is set, sample all entries
   * individually. */
  for (; i < MLKEM_K * MLKEM_K; i++)
  {
    uint8_t x, y;
    /* MLKEM_K <= 4, so the values fit in uint8_t. */
    x = (uint8_t)(i / MLKEM_K);
    y = (uint8_t)(i % MLKEM_K);

    if (transposed)
    {
      seed_ext[0][MLKEM_SYMBYTES + 0] = x;
      seed_ext[0][MLKEM_SYMBYTES + 1] = y;
    }
    else
    {
      seed_ext[0][MLKEM_SYMBYTES + 0] = y;
      seed_ext[0][MLKEM_SYMBYTES + 1] = x;
    }

    mlk_poly_rej_uniform(&a->vec[i / MLKEM_K].vec[i % MLKEM_K], seed_ext[0]);
  }

  mlk_assert(i == MLKEM_K * MLKEM_K);

  /*
   * The public matrix is generated in NTT domain. If the native backend
   * uses a custom order in NTT domain, permute A accordingly.
   */
  mlk_polymat_permute_bitrev_to_custom(a);

  /* Specification: Partially implements
   * @[FIPS203, Section 3.3, Destruction of intermediate values] */
  mlk_zeroize(seed_ext, sizeof(seed_ext));
}

/*************************************************
 * Name:        mlk_matvec_mul
 *
 * Description: Computes matrix-vector product in NTT domain,
 *              via Montgomery multiplication.
 *
 * Arguments:   - mlk_polyvec out: Pointer to output polynomial vector
 *              - mlk_polymat a: Input matrix. Must be in NTT domain
 *                  and have coefficients of absolute value < 4096.
 *              - mlk_polyvec v: Input polynomial vector. Must be in NTT
 *                  domain.
 *              - mlk_polyvec vc: Mulcache for v, computed via
 *                  mlk_polyvec_mulcache_compute().
 *
 * Specification: Implements @[FIPS203, Section 2.4.7, Eq (2.12), (2.13)]
 *
 **************************************************/
static void mlk_matvec_mul(mlk_polyvec *out, const mlk_polymat *a,
                           const mlk_polyvec *v, const mlk_polyvec_mulcache *vc)
__contract__(
  requires(memory_no_alias(out, sizeof(mlk_polyvec)))
  requires(memory_no_alias(a, sizeof(mlk_polymat)))
  requires(memory_no_alias(v, sizeof(mlk_polyvec)))
  requires(memory_no_alias(vc, sizeof(mlk_polyvec_mulcache)))
  requires(forall(k0, 0, MLKEM_K,
    forall(k1, 0, MLKEM_K,
      array_bound(a->vec[k0].vec[k1].coeffs, 0, MLKEM_N, 0, MLKEM_UINT12_LIMIT))))
  assigns(memory_slice(out, sizeof(mlk_polyvec))))
{
  unsigned i;
  for (i = 0; i < MLKEM_K; i++)
  __loop__(
    assigns(i, memory_slice(out, sizeof(mlk_polyvec)))
    invariant(i <= MLKEM_K))
  {
    mlk_polyvec_basemul_acc_montgomery_cached(&out->vec[i], &a->vec[i], v, vc);
  }
}

/* Reference: `indcpa_keypair_derand()` in the reference implementation @[REF].
 *            - We use x4-batched versions of `poly_getnoise` to leverage
 *              batched x4-batched Keccak-f1600.
 *            - We use a different implementation of `gen_matrix()` which
 *              uses x4-batched Keccak-f1600 (see `mlk_gen_matrix()` above).
 *            - We use a mulcache to speed up matrix-vector multiplication.
 *            - We include buffer zeroization.
 */
MLK_INTERNAL_API
int mlk_indcpa_keypair_derand(uint8_t pk[MLKEM_INDCPA_PUBLICKEYBYTES],
                              uint8_t sk[MLKEM_INDCPA_SECRETKEYBYTES],
                              const uint8_t coins[MLKEM_SYMBYTES],
                              MLK_CONFIG_CONTEXT_PARAMETER_TYPE context)
{
  int ret = 0;
  const uint8_t *publicseed;
  const uint8_t *noiseseed;
  MLK_ALLOC(buf, uint8_t, 2 * MLKEM_SYMBYTES, context);
  MLK_ALLOC(coins_with_domain_separator, uint8_t, MLKEM_SYMBYTES + 1, context);
  MLK_ALLOC(a, mlk_polymat, 1, context);
  MLK_ALLOC(e, mlk_polyvec, 1, context);
  MLK_ALLOC(pkpv, mlk_polyvec, 1, context);
  MLK_ALLOC(skpv, mlk_polyvec, 1, context);
  MLK_ALLOC(skpv_cache, mlk_polyvec_mulcache, 1, context);

  if (buf == NULL || coins_with_domain_separator == NULL || a == NULL ||
      e == NULL || pkpv == NULL || skpv == NULL || skpv_cache == NULL)
  {
    ret = MLK_ERR_OUT_OF_MEMORY;
    goto cleanup;
  }

  publicseed = buf;
  noiseseed = buf + MLKEM_SYMBYTES;

  /* Concatenate coins with MLKEM_K for domain separation of security levels */
  mlk_memcpy(coins_with_domain_separator, coins, MLKEM_SYMBYTES);
  coins_with_domain_separator[MLKEM_SYMBYTES] = MLKEM_K;

  mlk_hash_g(buf, coins_with_domain_separator, MLKEM_SYMBYTES + 1);

  /*
   * Declassify the public seed.
   * Required to use it in conditional-branches in rejection sampling.
   * This is needed because all output of randombytes is marked as secret
   * (=undefined)
   */
  MLK_CT_TESTING_DECLASSIFY(publicseed, MLKEM_SYMBYTES);

  mlk_gen_matrix(a, publicseed, 0 /* no transpose */);

#if MLKEM_K == 2
  mlk_poly_getnoise_eta1_4x(&skpv->vec[0], &skpv->vec[1], &e->vec[0],
                            &e->vec[1], noiseseed, 0, 1, 2, 3);
#elif MLKEM_K == 3
  /*
   * Only the first three output buffers are needed.
   * The laster parameter is a dummy that's overwritten later.
   */
  mlk_poly_getnoise_eta1_4x(&skpv->vec[0], &skpv->vec[1], &skpv->vec[2], NULL,
                            noiseseed, 0, 1, 2, 0xFF /* irrelevant */);
  /* Same here */
  mlk_poly_getnoise_eta1_4x(&e->vec[0], &e->vec[1], &e->vec[2], NULL, noiseseed,
                            3, 4, 5, 0xFF /* irrelevant */);
#elif MLKEM_K == 4
  mlk_poly_getnoise_eta1_4x(&skpv->vec[0], &skpv->vec[1], &skpv->vec[2],
                            &skpv->vec[3], noiseseed, 0, 1, 2, 3);
  mlk_poly_getnoise_eta1_4x(&e->vec[0], &e->vec[1], &e->vec[2], &e->vec[3],
                            noiseseed, 4, 5, 6, 7);
#endif /* MLKEM_K == 4 */

  mlk_polyvec_ntt(skpv);
  mlk_polyvec_ntt(e);

  mlk_polyvec_mulcache_compute(skpv_cache, skpv);
  mlk_matvec_mul(pkpv, a, skpv, skpv_cache);
  mlk_polyvec_tomont(pkpv);

  mlk_polyvec_add(pkpv, e);
  mlk_polyvec_reduce(pkpv);
  mlk_polyvec_reduce(skpv);

  mlk_pack_sk(sk, skpv);
  mlk_pack_pk(pk, pkpv, publicseed);

cleanup:
  /* Specification: Partially implements
   * @[FIPS203, Section 3.3, Destruction of intermediate values] */
  MLK_FREE(skpv_cache, mlk_polyvec_mulcache, 1, context);
  MLK_FREE(skpv, mlk_polyvec, 1, context);
  MLK_FREE(pkpv, mlk_polyvec, 1, context);
  MLK_FREE(e, mlk_polyvec, 1, context);
  MLK_FREE(a, mlk_polymat, 1, context);
  MLK_FREE(coins_with_domain_separator, uint8_t, MLKEM_SYMBYTES + 1, context);
  MLK_FREE(buf, uint8_t, 2 * MLKEM_SYMBYTES, context);
  return ret;
}

/* Reference: `indcpa_enc()` in the reference implementation @[REF].
 *            - We use x4-batched versions of `poly_getnoise` to leverage
 *              batched x4-batched Keccak-f1600.
 *            - We use a different implementation of `gen_matrix()` which
 *              uses x4-batched Keccak-f1600 (see `mlk_gen_matrix()` above).
 *            - We use a mulcache to speed up matrix-vector multiplication.
 *            - We include buffer zeroization.
 */
MLK_INTERNAL_API
int mlk_indcpa_enc(uint8_t c[MLKEM_INDCPA_BYTES],
                   const uint8_t m[MLKEM_INDCPA_MSGBYTES],
                   const uint8_t pk[MLKEM_INDCPA_PUBLICKEYBYTES],
                   const uint8_t coins[MLKEM_SYMBYTES],
                   MLK_CONFIG_CONTEXT_PARAMETER_TYPE context)
{
  int ret = 0;
  MLK_ALLOC(seed, uint8_t, MLKEM_SYMBYTES, context);
  MLK_ALLOC(at, mlk_polymat, 1, context);
  MLK_ALLOC(sp, mlk_polyvec, 1, context);
  MLK_ALLOC(pkpv, mlk_polyvec, 1, context);
  MLK_ALLOC(ep, mlk_polyvec, 1, context);
  MLK_ALLOC(b, mlk_polyvec, 1, context);
  MLK_ALLOC(v, mlk_poly, 1, context);
  MLK_ALLOC(k, mlk_poly, 1, context);
  MLK_ALLOC(epp, mlk_poly, 1, context);
  MLK_ALLOC(sp_cache, mlk_polyvec_mulcache, 1, context);

  if (seed == NULL || at == NULL || sp == NULL || pkpv == NULL || ep == NULL ||
      b == NULL || v == NULL || k == NULL || epp == NULL || sp_cache == NULL)
  {
    ret = MLK_ERR_OUT_OF_MEMORY;
    goto cleanup;
  }

  mlk_unpack_pk(pkpv, seed, pk);
  mlk_poly_frommsg(k, m);

  /*
   * Declassify the public seed.
   * Required to use it in conditional-branches in rejection sampling.
   * This is needed because in re-encryption the publicseed originated from sk
   * which is marked undefined.
   */
  MLK_CT_TESTING_DECLASSIFY(seed, MLKEM_SYMBYTES);

  mlk_gen_matrix(at, seed, 1 /* transpose */);

#if MLKEM_K == 2
  mlk_poly_getnoise_eta1122_4x(&sp->vec[0], &sp->vec[1], &ep->vec[0],
                               &ep->vec[1], coins, 0, 1, 2, 3);
  mlk_poly_getnoise_eta2(epp, coins, 4);
#elif MLKEM_K == 3
  /*
   * In this call, only the first three output buffers are needed.
   * The last parameter is a dummy that's overwritten later.
   */
  mlk_poly_getnoise_eta1_4x(&sp->vec[0], &sp->vec[1], &sp->vec[2], NULL, coins,
                            0, 1, 2, 0xFF /* irrelevant */);
  /* The fourth output buffer in this call _is_ used. */
  mlk_poly_getnoise_eta2_4x(&ep->vec[0], &ep->vec[1], &ep->vec[2], epp, coins,
                            3, 4, 5, 6);
#elif MLKEM_K == 4
  mlk_poly_getnoise_eta1_4x(&sp->vec[0], &sp->vec[1], &sp->vec[2], &sp->vec[3],
                            coins, 0, 1, 2, 3);
  mlk_poly_getnoise_eta2_4x(&ep->vec[0], &ep->vec[1], &ep->vec[2], &ep->vec[3],
                            coins, 4, 5, 6, 7);
  mlk_poly_getnoise_eta2(epp, coins, 8);
#endif /* MLKEM_K == 4 */

  mlk_polyvec_ntt(sp);

  mlk_polyvec_mulcache_compute(sp_cache, sp);
  mlk_matvec_mul(b, at, sp, sp_cache);
  mlk_polyvec_basemul_acc_montgomery_cached(v, pkpv, sp, sp_cache);

  mlk_polyvec_invntt_tomont(b);
  mlk_poly_invntt_tomont(v);

  mlk_polyvec_add(b, ep);
  mlk_poly_add(v, epp);
  mlk_poly_add(v, k);

  mlk_polyvec_reduce(b);
  mlk_poly_reduce(v);

  mlk_pack_ciphertext(c, b, v);

cleanup:
  /* Specification: Partially implements
   * @[FIPS203, Section 3.3, Destruction of intermediate values] */
  MLK_FREE(sp_cache, mlk_polyvec_mulcache, 1, context);
  MLK_FREE(epp, mlk_poly, 1, context);
  MLK_FREE(k, mlk_poly, 1, context);
  MLK_FREE(v, mlk_poly, 1, context);
  MLK_FREE(b, mlk_polyvec, 1, context);
  MLK_FREE(ep, mlk_polyvec, 1, context);
  MLK_FREE(pkpv, mlk_polyvec, 1, context);
  MLK_FREE(sp, mlk_polyvec, 1, context);
  MLK_FREE(at, mlk_polymat, 1, context);
  MLK_FREE(seed, uint8_t, MLKEM_SYMBYTES, context);
  return ret;
}

/* Reference: `indcpa_dec()` in the reference implementation @[REF].
 *            - We use a mulcache for the scalar product.
 *            - We include buffer zeroization. */
MLK_INTERNAL_API
int mlk_indcpa_dec(uint8_t m[MLKEM_INDCPA_MSGBYTES],
                   const uint8_t c[MLKEM_INDCPA_BYTES],
                   const uint8_t sk[MLKEM_INDCPA_SECRETKEYBYTES],
                   MLK_CONFIG_CONTEXT_PARAMETER_TYPE context)
{
  int ret = 0;
  MLK_ALLOC(b, mlk_polyvec, 1, context);
  MLK_ALLOC(skpv, mlk_polyvec, 1, context);
  MLK_ALLOC(v, mlk_poly, 1, context);
  MLK_ALLOC(sb, mlk_poly, 1, context);
  MLK_ALLOC(b_cache, mlk_polyvec_mulcache, 1, context);

  if (b == NULL || skpv == NULL || v == NULL || sb == NULL || b_cache == NULL)
  {
    ret = MLK_ERR_OUT_OF_MEMORY;
    goto cleanup;
  }

  mlk_unpack_ciphertext(b, v, c);
  mlk_unpack_sk(skpv, sk);

  mlk_polyvec_ntt(b);
  mlk_polyvec_mulcache_compute(b_cache, b);
  mlk_polyvec_basemul_acc_montgomery_cached(sb, skpv, b, b_cache);
  mlk_poly_invntt_tomont(sb);

  mlk_poly_sub(v, sb);
  mlk_poly_reduce(v);

  mlk_poly_tomsg(m, v);

cleanup:
  /* Specification: Partially implements
   * @[FIPS203, Section 3.3, Destruction of intermediate values] */
  MLK_FREE(b_cache, mlk_polyvec_mulcache, 1, context);
  MLK_FREE(sb, mlk_poly, 1, context);
  MLK_FREE(v, mlk_poly, 1, context);
  MLK_FREE(skpv, mlk_polyvec, 1, context);
  MLK_FREE(b, mlk_polyvec, 1, context);
  return ret;
}

/* To facilitate single-compilation-unit (SCU) builds, undefine all macros.
 * Don't modify by hand -- this is auto-generated by scripts/autogen. */
#undef mlk_pack_pk
#undef mlk_unpack_pk
#undef mlk_pack_sk
#undef mlk_unpack_sk
#undef mlk_pack_ciphertext
#undef mlk_unpack_ciphertext
#undef mlk_matvec_mul
#undef mlk_polyvec_permute_bitrev_to_custom
#undef mlk_polymat_permute_bitrev_to_custom
