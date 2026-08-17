/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/* References
 * ==========
 *
 * - [FIPS204]
 *   FIPS 204 Module-Lattice-Based Digital Signature Standard
 *   National Institute of Standards and Technology
 *   https://csrc.nist.gov/pubs/fips/204/final
 */

/*
 * Eager and lazy variants of polynomial vector types.
 *
 * In eager mode, full vectors are precomputed and stored in memory.
 * In lazy mode, data is stored in packed form and expanded on demand,
 * trading computation for reduced memory usage.
 *
 * MLD_CONFIG_REDUCE_RAM selects which variant is used.
 */

#ifndef MLD_POLYVEC_LAZY_H
#define MLD_POLYVEC_LAZY_H

#if !defined(MLD_CONFIG_NO_KEYPAIR_API) || !defined(MLD_CONFIG_NO_SIGN_API) || \
    !defined(MLD_CONFIG_NO_VERIFY_API)

#include "poly.h"
#include "poly_kl.h"
#include "polyvec.h"

/* Parameter set namespacing */
#define mld_sk_s1hat_eager MLD_ADD_PARAM_SET(mld_sk_s1hat_eager)
#define mld_sk_s1hat_lazy MLD_ADD_PARAM_SET(mld_sk_s1hat_lazy)
#define mld_sk_s1hat MLD_ADD_PARAM_SET(mld_sk_s1hat)
#define mld_unpack_sk_s1hat_eager MLD_ADD_PARAM_SET(mld_unpack_sk_s1hat_eager)
#define mld_unpack_sk_s1hat_lazy MLD_ADD_PARAM_SET(mld_unpack_sk_s1hat_lazy)
#define mld_sk_s1hat_get_poly_eager \
  MLD_ADD_PARAM_SET(mld_sk_s1hat_get_poly_eager)
#define mld_sk_s1hat_get_poly_lazy MLD_ADD_PARAM_SET(mld_sk_s1hat_get_poly_lazy)
#define mld_sk_s2hat_eager MLD_ADD_PARAM_SET(mld_sk_s2hat_eager)
#define mld_sk_s2hat_lazy MLD_ADD_PARAM_SET(mld_sk_s2hat_lazy)
#define mld_sk_s2hat MLD_ADD_PARAM_SET(mld_sk_s2hat)
#define mld_unpack_sk_s2hat_eager MLD_ADD_PARAM_SET(mld_unpack_sk_s2hat_eager)
#define mld_unpack_sk_s2hat_lazy MLD_ADD_PARAM_SET(mld_unpack_sk_s2hat_lazy)
#define mld_sk_s2hat_get_poly_eager \
  MLD_ADD_PARAM_SET(mld_sk_s2hat_get_poly_eager)
#define mld_sk_s2hat_get_poly_lazy MLD_ADD_PARAM_SET(mld_sk_s2hat_get_poly_lazy)
#define mld_sk_t0hat_eager MLD_ADD_PARAM_SET(mld_sk_t0hat_eager)
#define mld_sk_t0hat_lazy MLD_ADD_PARAM_SET(mld_sk_t0hat_lazy)
#define mld_sk_t0hat MLD_ADD_PARAM_SET(mld_sk_t0hat)
#define mld_unpack_sk_t0hat_eager MLD_ADD_PARAM_SET(mld_unpack_sk_t0hat_eager)
#define mld_unpack_sk_t0hat_lazy MLD_ADD_PARAM_SET(mld_unpack_sk_t0hat_lazy)
#define mld_sk_t0hat_get_poly_eager \
  MLD_ADD_PARAM_SET(mld_sk_t0hat_get_poly_eager)
#define mld_sk_t0hat_get_poly_lazy MLD_ADD_PARAM_SET(mld_sk_t0hat_get_poly_lazy)
#define mld_polymat MLD_ADD_PARAM_SET(mld_polymat)
#define mld_polymat_eager MLD_ADD_PARAM_SET(mld_polymat_eager)
#define mld_polymat_lazy MLD_ADD_PARAM_SET(mld_polymat_lazy)
#define mld_poly_permute_bitrev_to_custom_optional \
  MLD_ADD_PARAM_SET(mld_poly_permute_bitrev_to_custom_optional)
#define mld_polyvec_matrix_expand_eager \
  MLD_NAMESPACE_KL(polyvec_matrix_expand_eager)
#define mld_polyvec_matrix_expand_lazy \
  MLD_NAMESPACE_KL(polyvec_matrix_expand_lazy)
#define mld_polyvec_matrix_pointwise_montgomery \
  MLD_NAMESPACE_KL(polyvec_matrix_pointwise_montgomery)
#define mld_polyvec_matrix_pointwise_montgomery_row_eager \
  MLD_NAMESPACE_KL(polyvec_matrix_pointwise_montgomery_row_eager)
#define mld_polyvec_matrix_pointwise_montgomery_row_lazy \
  MLD_NAMESPACE_KL(polyvec_matrix_pointwise_montgomery_row_lazy)
#define mld_polyvec_matrix_pointwise_montgomery_yvec_eager \
  MLD_NAMESPACE_KL(polyvec_matrix_pointwise_montgomery_yvec_eager)
#define mld_polyvec_matrix_pointwise_montgomery_yvec_lazy \
  MLD_NAMESPACE_KL(polyvec_matrix_pointwise_montgomery_yvec_lazy)
#define mld_yvec_eager MLD_ADD_PARAM_SET(mld_yvec_eager)
#define mld_yvec_lazy MLD_ADD_PARAM_SET(mld_yvec_lazy)
#define mld_yvec MLD_ADD_PARAM_SET(mld_yvec)
#define mld_yvec_init_eager MLD_ADD_PARAM_SET(mld_yvec_init_eager)
#define mld_yvec_init_lazy MLD_ADD_PARAM_SET(mld_yvec_init_lazy)
#define mld_yvec_get_poly_eager MLD_ADD_PARAM_SET(mld_yvec_get_poly_eager)
#define mld_yvec_get_poly_lazy MLD_ADD_PARAM_SET(mld_yvec_get_poly_lazy)
/* End of parameter set namespacing */

/** Eager s1hat: precomputed s1 vector in NTT domain. */
typedef struct
{
  mld_polyvecl vec; /**< s1 vector in NTT domain. */
} mld_sk_s1hat_eager;

/** Eager s2hat: precomputed s2 vector in NTT domain. */
typedef struct
{
  mld_polyveck vec; /**< s2 vector in NTT domain. */
} mld_sk_s2hat_eager;

/** Eager t0hat: precomputed t0 vector in NTT domain. */
typedef struct
{
  mld_polyveck vec; /**< t0 vector in NTT domain. */
} mld_sk_t0hat_eager;

/** Lazy s1hat: borrow packed s1, unpack and convert to NTT domain on demand. */
typedef struct
{
  const uint8_t *packed; /**< Pointer to packed s1 in the secret key. */
} mld_sk_s1hat_lazy;

/** Lazy s2hat: borrow packed s2, unpack and convert to NTT domain on demand. */
typedef struct
{
  const uint8_t *packed; /**< Pointer to packed s2 in the secret key. */
} mld_sk_s2hat_lazy;

/** Lazy t0hat: borrow packed t0, unpack and convert to NTT domain on demand. */
typedef struct
{
  const uint8_t *packed; /**< Pointer to packed t0 in the secret key. */
} mld_sk_t0hat_lazy;

/** Eager yvec: precomputed and stored full signing masking vector y. */
typedef struct
{
  mld_polyvecl vec; /**< Masking vector y. */
} mld_yvec_eager;

/** Lazy yvec: store seed and nonce, regenerate y[i] on demand. */
typedef struct
{
  const uint8_t *rhoprime; /**< Pointer to seed used to derive y. */
  uint16_t nonce; /**< Base nonce; component i uses MLDSA_L*nonce + i. */
} mld_yvec_lazy;

#if !defined(MLD_CONFIG_NO_KEYPAIR_API) || !defined(MLD_CONFIG_NO_SIGN_API)
/* s1vec */

#if !defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
static MLD_INLINE void mld_unpack_sk_s1hat_eager(
    mld_sk_s1hat_eager *s1,
    const uint8_t packed_s1[MLDSA_L * MLDSA_POLYETA_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(s1, sizeof(mld_sk_s1hat_eager)))
  requires(memory_no_alias(packed_s1, MLDSA_L * MLDSA_POLYETA_PACKEDBYTES))
  assigns(memory_slice(s1, sizeof(mld_sk_s1hat_eager)))
  ensures(forall(k1, 0, MLDSA_L,
    array_abs_bound(s1->vec.vec[k1].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
)
{
  mld_polyvecl_unpack_eta(&s1->vec, packed_s1);
  mld_polyvecl_ntt(&s1->vec);
}

#if !defined(MLD_CONFIG_NO_SIGN_API)
static MLD_INLINE void mld_sk_s1hat_get_poly_eager(mld_poly *buf,
                                                   const mld_sk_s1hat_eager *s1,
                                                   unsigned int i)
__contract__(
  requires(memory_no_alias(buf, sizeof(mld_poly)))
  requires(memory_no_alias(s1, sizeof(mld_sk_s1hat_eager)))
  requires(i < MLDSA_L)
  requires(array_abs_bound(s1->vec.vec[i].coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
  assigns(memory_slice(buf, sizeof(mld_poly)))
  ensures(array_abs_bound(buf->coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
) { *buf = s1->vec.vec[i]; }
#endif /* !MLD_CONFIG_NO_SIGN_API */
#endif /* !MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */
#if defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
static MLD_INLINE void mld_unpack_sk_s1hat_lazy(
    mld_sk_s1hat_lazy *s1,
    const uint8_t packed_s1[MLDSA_L * MLDSA_POLYETA_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(s1, sizeof(mld_sk_s1hat_lazy)))
  assigns(memory_slice(s1, sizeof(mld_sk_s1hat_lazy)))
  ensures(s1->packed == old(packed_s1))
) { s1->packed = packed_s1; }

#if !defined(MLD_CONFIG_NO_SIGN_API)
static MLD_INLINE void mld_sk_s1hat_get_poly_lazy(mld_poly *buf,
                                                  const mld_sk_s1hat_lazy *s1,
                                                  unsigned int i)
__contract__(
  requires(memory_no_alias(buf, sizeof(mld_poly)))
  requires(memory_no_alias(s1, sizeof(mld_sk_s1hat_lazy)))
  requires(i < MLDSA_L)
  requires(memory_no_alias(s1->packed, MLDSA_L * MLDSA_POLYETA_PACKEDBYTES))
  assigns(memory_slice(buf, sizeof(mld_poly)))
  ensures(array_abs_bound(buf->coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
)
{
  mld_polyeta_unpack(buf, s1->packed + i * MLDSA_POLYETA_PACKEDBYTES);
  mld_poly_ntt(buf);
}
#endif /* !MLD_CONFIG_NO_SIGN_API */
#endif /* MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

/* s2vec */

#if (!defined(MLD_CONFIG_NO_SIGN_API) || defined(MLD_UNIT_TEST)) && \
    (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST))
static MLD_INLINE void mld_unpack_sk_s2hat_eager(
    mld_sk_s2hat_eager *s2,
    const uint8_t packed_s2[MLDSA_K * MLDSA_POLYETA_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(s2, sizeof(mld_sk_s2hat_eager)))
  requires(memory_no_alias(packed_s2, MLDSA_K * MLDSA_POLYETA_PACKEDBYTES))
  assigns(memory_slice(s2, sizeof(mld_sk_s2hat_eager)))
  ensures(forall(k1, 0, MLDSA_K,
    array_abs_bound(s2->vec.vec[k1].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
)
{
  mld_polyveck_unpack_eta(&s2->vec, packed_s2);
  mld_polyveck_ntt(&s2->vec);
}

#if !defined(MLD_CONFIG_NO_SIGN_API)
static MLD_INLINE void mld_sk_s2hat_get_poly_eager(mld_poly *buf,
                                                   const mld_sk_s2hat_eager *s2,
                                                   unsigned int i)
__contract__(
  requires(memory_no_alias(buf, sizeof(mld_poly)))
  requires(memory_no_alias(s2, sizeof(mld_sk_s2hat_eager)))
  requires(i < MLDSA_K)
  requires(array_abs_bound(s2->vec.vec[i].coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
  assigns(memory_slice(buf, sizeof(mld_poly)))
  ensures(array_abs_bound(buf->coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
) { *buf = s2->vec.vec[i]; }
#endif /* !MLD_CONFIG_NO_SIGN_API */
#endif /* (!MLD_CONFIG_NO_SIGN_API || MLD_UNIT_TEST) && \
          (!MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST) */
#if defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
static MLD_INLINE void mld_unpack_sk_s2hat_lazy(
    mld_sk_s2hat_lazy *s2,
    const uint8_t packed_s2[MLDSA_K * MLDSA_POLYETA_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(s2, sizeof(mld_sk_s2hat_lazy)))
  assigns(memory_slice(s2, sizeof(mld_sk_s2hat_lazy)))
  ensures(s2->packed == old(packed_s2))
) { s2->packed = packed_s2; }

#if !defined(MLD_CONFIG_NO_SIGN_API)
static MLD_INLINE void mld_sk_s2hat_get_poly_lazy(mld_poly *buf,
                                                  const mld_sk_s2hat_lazy *s2,
                                                  unsigned int i)
__contract__(
  requires(memory_no_alias(buf, sizeof(mld_poly)))
  requires(memory_no_alias(s2, sizeof(mld_sk_s2hat_lazy)))
  requires(i < MLDSA_K)
  requires(memory_no_alias(s2->packed, MLDSA_K * MLDSA_POLYETA_PACKEDBYTES))
  assigns(memory_slice(buf, sizeof(mld_poly)))
  ensures(array_abs_bound(buf->coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
)
{
  mld_polyeta_unpack(buf, s2->packed + i * MLDSA_POLYETA_PACKEDBYTES);
  mld_poly_ntt(buf);
}
#endif /* !MLD_CONFIG_NO_SIGN_API */
#endif /* MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

/* t0vec */

#if (!defined(MLD_CONFIG_NO_SIGN_API) || defined(MLD_UNIT_TEST)) && \
    (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST))
static MLD_INLINE void mld_unpack_sk_t0hat_eager(
    mld_sk_t0hat_eager *t0,
    const uint8_t packed_t0[MLDSA_K * MLDSA_POLYT0_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(t0, sizeof(mld_sk_t0hat_eager)))
  requires(memory_no_alias(packed_t0, MLDSA_K * MLDSA_POLYT0_PACKEDBYTES))
  assigns(memory_slice(t0, sizeof(mld_sk_t0hat_eager)))
  ensures(forall(k1, 0, MLDSA_K,
    array_abs_bound(t0->vec.vec[k1].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
)
{
  unsigned int i;
  for (i = 0; i < MLDSA_K; ++i)
  __loop__(
    assigns(i, memory_slice(t0, sizeof(mld_sk_t0hat_eager)))
    invariant(i <= MLDSA_K)
    invariant(forall(k0, 0, i,
      array_bound(t0->vec.vec[k0].coeffs, 0, MLDSA_N,
                  -(1 << (MLDSA_D - 1)) + 1, (1 << (MLDSA_D - 1)) + 1)))
    decreases(MLDSA_K - i)
  )
  {
    mld_polyt0_unpack(&t0->vec.vec[i],
                      packed_t0 + i * MLDSA_POLYT0_PACKEDBYTES);
  }
  mld_polyveck_ntt(&t0->vec);
}

#if !defined(MLD_CONFIG_NO_SIGN_API)
static MLD_INLINE void mld_sk_t0hat_get_poly_eager(mld_poly *buf,
                                                   const mld_sk_t0hat_eager *t0,
                                                   unsigned int i)
__contract__(
  requires(memory_no_alias(buf, sizeof(mld_poly)))
  requires(memory_no_alias(t0, sizeof(mld_sk_t0hat_eager)))
  requires(i < MLDSA_K)
  requires(array_abs_bound(t0->vec.vec[i].coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
  assigns(memory_slice(buf, sizeof(mld_poly)))
  ensures(array_abs_bound(buf->coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
) { *buf = t0->vec.vec[i]; }
#endif /* !MLD_CONFIG_NO_SIGN_API */
#endif /* (!MLD_CONFIG_NO_SIGN_API || MLD_UNIT_TEST) && \
          (!MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST) */
#if defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
static MLD_INLINE void mld_unpack_sk_t0hat_lazy(
    mld_sk_t0hat_lazy *t0,
    const uint8_t packed_t0[MLDSA_K * MLDSA_POLYT0_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(t0, sizeof(mld_sk_t0hat_lazy)))
  assigns(memory_slice(t0, sizeof(mld_sk_t0hat_lazy)))
  ensures(t0->packed == old(packed_t0))
) { t0->packed = packed_t0; }

#if !defined(MLD_CONFIG_NO_SIGN_API)
static MLD_INLINE void mld_sk_t0hat_get_poly_lazy(mld_poly *buf,
                                                  const mld_sk_t0hat_lazy *t0,
                                                  unsigned int i)
__contract__(
  requires(memory_no_alias(buf, sizeof(mld_poly)))
  requires(memory_no_alias(t0, sizeof(mld_sk_t0hat_lazy)))
  requires(i < MLDSA_K)
  requires(memory_no_alias(t0->packed, MLDSA_K * MLDSA_POLYT0_PACKEDBYTES))
  assigns(memory_slice(buf, sizeof(mld_poly)))
  ensures(array_abs_bound(buf->coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
)
{
  mld_polyt0_unpack(buf, t0->packed + i * MLDSA_POLYT0_PACKEDBYTES);
  mld_poly_ntt(buf);
}
#endif /* !MLD_CONFIG_NO_SIGN_API */
#endif /* MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */
#endif /* !MLD_CONFIG_NO_KEYPAIR_API || !MLD_CONFIG_NO_SIGN_API */

/* yvec */

#if !defined(MLD_CONFIG_NO_SIGN_API) && \
    (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST))
static MLD_INLINE void mld_yvec_init_eager(
    mld_yvec_eager *y, const uint8_t rhoprime[MLDSA_CRHBYTES], uint16_t nonce)
__contract__(
  requires(memory_no_alias(y, sizeof(mld_yvec_eager)))
  requires(memory_no_alias(rhoprime, MLDSA_CRHBYTES))
  requires(nonce <= (UINT16_MAX - MLDSA_L) / MLDSA_L)
  assigns(memory_slice(y, sizeof(mld_yvec_eager)))
  ensures(forall(k1, 0, MLDSA_L,
    array_bound(y->vec.vec[k1].coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1)))
)
{
  mld_polyvecl_uniform_gamma1(&y->vec, rhoprime, nonce);
}

static MLD_INLINE void mld_yvec_get_poly_eager(mld_poly *buf,
                                               const mld_yvec_eager *y,
                                               unsigned int i)
__contract__(
  requires(memory_no_alias(buf, sizeof(mld_poly)))
  requires(memory_no_alias(y, sizeof(mld_yvec_eager)))
  requires(i < MLDSA_L)
  requires(array_bound(y->vec.vec[i].coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
  assigns(memory_slice(buf, sizeof(mld_poly)))
  ensures(array_bound(buf->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
) { *buf = y->vec.vec[i]; }
#endif /* !MLD_CONFIG_NO_SIGN_API && (!MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST) \
        */
#if !defined(MLD_CONFIG_NO_SIGN_API) && \
    (defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST))
static MLD_INLINE void mld_yvec_init_lazy(
    mld_yvec_lazy *y, const uint8_t rhoprime[MLDSA_CRHBYTES], uint16_t nonce)
__contract__(
  requires(memory_no_alias(y, sizeof(mld_yvec_lazy)))
  assigns(memory_slice(y, sizeof(mld_yvec_lazy)))
  ensures(y->rhoprime == old(rhoprime))
  ensures(y->nonce == old(nonce))
)
{
  y->rhoprime = rhoprime;
  y->nonce = nonce;
}

static MLD_INLINE void mld_yvec_get_poly_lazy(mld_poly *buf,
                                              const mld_yvec_lazy *y,
                                              unsigned int i)
__contract__(
  requires(memory_no_alias(buf, sizeof(mld_poly)))
  requires(memory_no_alias(y, sizeof(mld_yvec_lazy)))
  requires(i < MLDSA_L)
  requires(memory_no_alias(y->rhoprime, MLDSA_CRHBYTES))
  requires(y->nonce <= ((UINT16_MAX - MLDSA_L) / MLDSA_L))
  assigns(memory_slice(buf, sizeof(mld_poly)))
  ensures(array_bound(buf->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
)
{
  /* Safety: y->nonce is at most ((UINT16_MAX - MLDSA_L) / MLDSA_L) and
   * i < MLDSA_L, so MLDSA_L * y->nonce + i fits in uint16_t. See
   * MLD_NONCE_UB comment in sign.c. */
  mld_poly_uniform_gamma1(buf, y->rhoprime,
                          (uint16_t)(MLDSA_L * y->nonce + (int)i));
}
#endif /* !MLD_CONFIG_NO_SIGN_API && (MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST) \
        */

/* polymat */

#if !defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
/** Eager polymat: precomputed and stored full MLDSA_K x MLDSA_L matrix. */
typedef struct
{
  mld_polyvecl vec[MLDSA_K]; /**< Rows of the matrix. */
} mld_polymat_eager;
#endif /* !MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

/** Lazy polymat: store seed, sample elements A[k][l] on demand. */
typedef struct
{
  mld_poly cur; /**< On-demand sampled matrix element A[k][l]. */
  uint8_t rho[MLDSA_SEEDBYTES]; /**< Public seed used to expand A. */
} mld_polymat_lazy;

static MLD_INLINE void mld_poly_permute_bitrev_to_custom_optional(mld_poly *p)
__contract__(
  /* We don't specify that this is a permutation, only that it preserves
   * the bounds.
   * When the native NTT backend does not use the custom order, this is a no-op. */
  requires(memory_no_alias(p, sizeof(mld_poly)))
  requires(array_bound(p->coeffs, 0, MLDSA_N, 0, MLDSA_Q))
  assigns(memory_slice(p, sizeof(mld_poly)))
  ensures(array_bound(p->coeffs, 0, MLDSA_N, 0, MLDSA_Q))
)
{
#if defined(MLD_USE_NATIVE_NTT_CUSTOM_ORDER)
  mld_poly_permute_bitrev_to_custom(p->coeffs);
#else
  (void)p;
#endif
}

#if !defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
/**
 * Implementation of ExpandA. Generates matrix A with uniformly random
 * coefficients a_{i,j} by performing rejection sampling on the output stream
 * of SHAKE128(rho|j|i).
 *
 * @param[out] mat Pointer to output matrix.
 * @param[in]  rho Byte array containing seed rho.
 */
MLD_INTERNAL_API
void mld_polyvec_matrix_expand_eager(mld_polymat_eager *mat,
                                     const uint8_t rho[MLDSA_SEEDBYTES])
__contract__(
  requires(memory_no_alias(mat, sizeof(mld_polymat_eager)))
  requires(memory_no_alias(rho, MLDSA_SEEDBYTES))
  assigns(memory_slice(mat, sizeof(mld_polymat_eager)))
  ensures(forall(k1, 0, MLDSA_K, forall(l1, 0, MLDSA_L,
    array_bound(mat->vec[k1].vec[l1].coeffs, 0, MLDSA_N, 0, MLDSA_Q))))
);

/**
 * Compute row i of matrix-vector multiplication in NTT domain with pointwise
 * multiplication and multiplication by 2^{-32}.
 *
 * Input matrix and vector must be in NTT domain representation. Output
 * coefficients are bounded by MLDSA_Q in absolute value.
 *
 * @param[out] t_row Pointer to output row polynomial.
 * @param[in]  mat   Pointer to input matrix.
 * @param[in]  v     Pointer to input vector v.
 * @param      i     Row index, 0 <= i < MLDSA_K.
 */
MLD_INTERNAL_API
void mld_polyvec_matrix_pointwise_montgomery_row_eager(mld_poly *t_row,
                                                       mld_polymat_eager *mat,
                                                       const mld_polyvecl *v,
                                                       unsigned int i)
__contract__(
  requires(memory_no_alias(t_row, sizeof(mld_poly)))
  requires(memory_no_alias(mat, sizeof(mld_polymat_eager)))
  requires(memory_no_alias(v, sizeof(mld_polyvecl)))
  requires(i < MLDSA_K)
  requires(forall(l1, 0, MLDSA_L,
                  array_bound(mat->vec[i].vec[l1].coeffs, 0, MLDSA_N, 0, MLDSA_Q)))
  requires(forall(l2, 0, MLDSA_L,
                  array_abs_bound(v->vec[l2].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
  assigns(memory_slice(t_row, sizeof(mld_poly)))
  ensures(array_abs_bound(t_row->coeffs, 0, MLDSA_N, MLDSA_Q))
);

#if !defined(MLD_CONFIG_NO_SIGN_API)
/**
 * Compute w = invNTT(A * NTT(y)) for the signing y vector.
 *
 * The eager variant copies y into the scratch polyvecl, NTTs it in place,
 * calls the standard matrix-vector multiply, and finally inverse-NTTs the
 * result into w.
 *
 * @param[out] w       Pointer to output vector.
 * @param[in]  mat     Pointer to input matrix.
 * @param[in]  y       Pointer to (non-NTT) y vector.
 * @param[out] scratch Scratch polyvecl for NTT'd copy of y.
 */
MLD_INTERNAL_API
void mld_polyvec_matrix_pointwise_montgomery_yvec_eager(mld_polyveck *w,
                                                        mld_polymat_eager *mat,
                                                        const mld_yvec_eager *y,
                                                        mld_polyvecl *scratch)
__contract__(
  requires(memory_no_alias(w, sizeof(mld_polyveck)))
  requires(memory_no_alias(mat, sizeof(mld_polymat_eager)))
  requires(memory_no_alias(y, sizeof(mld_yvec_eager)))
  requires(memory_no_alias(scratch, sizeof(mld_polyvecl)))
  requires(forall(k1, 0, MLDSA_K, forall(l1, 0, MLDSA_L,
    array_bound(mat->vec[k1].vec[l1].coeffs, 0, MLDSA_N, 0, MLDSA_Q))))
  requires(forall(l2, 0, MLDSA_L,
    array_bound(y->vec.vec[l2].coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1)))
  assigns(memory_slice(w, sizeof(mld_polyveck)))
  assigns(memory_slice(scratch, sizeof(mld_polyvecl)))
  ensures(forall(k0, 0, MLDSA_K,
    array_abs_bound(w->vec[k0].coeffs, 0, MLDSA_N, MLD_INTT_BOUND)))
);
#endif /* !MLD_CONFIG_NO_SIGN_API */
#endif /* !MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

#if defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
MLD_INTERNAL_API
void mld_polyvec_matrix_expand_lazy(mld_polymat_lazy *mat,
                                    const uint8_t rho[MLDSA_SEEDBYTES])
__contract__(
  requires(memory_no_alias(mat, sizeof(mld_polymat_lazy)))
  requires(memory_no_alias(rho, MLDSA_SEEDBYTES))
  assigns(memory_slice(mat, sizeof(mld_polymat_lazy)))
);

#if !defined(MLD_CONFIG_NO_KEYPAIR_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
/**
 * Compute row i of matrix-vector multiplication in NTT domain with pointwise
 * multiplication and multiplication by 2^{-32}.
 *
 * Input vector must be in NTT domain representation; the matrix entries are
 * sampled on demand from the seed stored in mat->rho, using mat->cur as
 * scratch. Output coefficients are bounded by MLDSA_Q in absolute value.
 *
 * @param[out]    t_row Pointer to output row polynomial.
 * @param[in,out] mat   Pointer to input matrix (seed + scratch).
 * @param[in]     v     Pointer to input vector v.
 * @param         i     Row index, 0 <= i < MLDSA_K.
 */
MLD_INTERNAL_API
void mld_polyvec_matrix_pointwise_montgomery_row_lazy(mld_poly *t_row,
                                                      mld_polymat_lazy *mat,
                                                      const mld_polyvecl *v,
                                                      unsigned int i)
__contract__(
  requires(memory_no_alias(t_row, sizeof(mld_poly)))
  requires(memory_no_alias(mat, sizeof(mld_polymat_lazy)))
  requires(memory_no_alias(v, sizeof(mld_polyvecl)))
  requires(i < MLDSA_K)
  requires(forall(l1, 0, MLDSA_L,
                  array_abs_bound(v->vec[l1].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
  assigns(memory_slice(t_row, sizeof(mld_poly)))
  assigns(memory_slice(mat, sizeof(mld_polymat_lazy)))
  ensures(array_abs_bound(t_row->coeffs, 0, MLDSA_N, MLDSA_Q))
);
#endif /* !MLD_CONFIG_NO_KEYPAIR_API || !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
/**
 * Compute w = invNTT(A * NTT(y)) for the signing y vector.
 *
 * The lazy variant samples one column of y at a time, NTTs it into
 * &scratch->vec[0], and accumulates the matrix-vector product
 * column-by-column with on-demand sampling of A[k][l]. Only the first poly of
 * the polyvecl scratch is used; the polyvecl type is shared with the eager
 * variant for API uniformity (the storage is provided "for free" by the
 * caller's polyveck/polyvecl union in REDUCE_RAM mode).
 *
 * @param[out]    w       Pointer to output vector.
 * @param[in,out] mat     Pointer to input matrix.
 * @param[in]     y       Pointer to y seed/nonce.
 * @param[out]    scratch Scratch (only &scratch->vec[0] used).
 */
MLD_INTERNAL_API
void mld_polyvec_matrix_pointwise_montgomery_yvec_lazy(mld_polyveck *w,
                                                       mld_polymat_lazy *mat,
                                                       const mld_yvec_lazy *y,
                                                       mld_polyvecl *scratch)
__contract__(
  requires(memory_no_alias(w, sizeof(mld_polyveck)))
  requires(memory_no_alias(mat, sizeof(mld_polymat_lazy)))
  requires(memory_no_alias(y, sizeof(mld_yvec_lazy)))
  requires(memory_no_alias(scratch, sizeof(mld_polyvecl)))
  requires(memory_no_alias(y->rhoprime, MLDSA_CRHBYTES))
  requires(y->nonce <= ((UINT16_MAX - MLDSA_L) / MLDSA_L))
  assigns(memory_slice(w, sizeof(mld_polyveck)))
  assigns(memory_slice(mat, sizeof(mld_polymat_lazy)))
  assigns(memory_slice(scratch, sizeof(mld_polyvecl)))
  ensures(forall(k0, 0, MLDSA_K,
    array_abs_bound(w->vec[k0].coeffs, 0, MLDSA_N, MLD_INTT_BOUND)))
);
#endif /* !MLD_CONFIG_NO_SIGN_API */
#endif /* MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

/* Dispatch: typedef and define based on MLD_CONFIG_REDUCE_RAM */
#if defined(MLD_CONFIG_REDUCE_RAM)
typedef mld_sk_s1hat_lazy mld_sk_s1hat;
typedef mld_sk_s2hat_lazy mld_sk_s2hat;
typedef mld_sk_t0hat_lazy mld_sk_t0hat;
typedef mld_polymat_lazy mld_polymat;
typedef mld_yvec_lazy mld_yvec;
#define mld_unpack_sk_s1hat mld_unpack_sk_s1hat_lazy
#define mld_unpack_sk_s2hat mld_unpack_sk_s2hat_lazy
#define mld_unpack_sk_t0hat mld_unpack_sk_t0hat_lazy
#if !defined(MLD_CONFIG_NO_SIGN_API)
#define mld_sk_s1hat_get_poly mld_sk_s1hat_get_poly_lazy
#define mld_sk_s2hat_get_poly mld_sk_s2hat_get_poly_lazy
#define mld_sk_t0hat_get_poly mld_sk_t0hat_get_poly_lazy
#endif
#define mld_polyvec_matrix_expand mld_polyvec_matrix_expand_lazy
#define mld_polyvec_matrix_pointwise_montgomery_row \
  mld_polyvec_matrix_pointwise_montgomery_row_lazy
#define mld_yvec_init mld_yvec_init_lazy
#define mld_yvec_get_poly mld_yvec_get_poly_lazy
#define mld_polyvec_matrix_pointwise_montgomery_yvec \
  mld_polyvec_matrix_pointwise_montgomery_yvec_lazy
#else /* MLD_CONFIG_REDUCE_RAM */
typedef mld_sk_s1hat_eager mld_sk_s1hat;
typedef mld_sk_s2hat_eager mld_sk_s2hat;
typedef mld_sk_t0hat_eager mld_sk_t0hat;
typedef mld_polymat_eager mld_polymat;
typedef mld_yvec_eager mld_yvec;
#define mld_unpack_sk_s1hat mld_unpack_sk_s1hat_eager
#define mld_unpack_sk_s2hat mld_unpack_sk_s2hat_eager
#define mld_unpack_sk_t0hat mld_unpack_sk_t0hat_eager
#if !defined(MLD_CONFIG_NO_SIGN_API)
#define mld_sk_s2hat_get_poly mld_sk_s2hat_get_poly_eager
#define mld_sk_s1hat_get_poly mld_sk_s1hat_get_poly_eager
#define mld_sk_t0hat_get_poly mld_sk_t0hat_get_poly_eager
#endif
#define mld_polyvec_matrix_expand mld_polyvec_matrix_expand_eager
#define mld_polyvec_matrix_pointwise_montgomery_row \
  mld_polyvec_matrix_pointwise_montgomery_row_eager
#define mld_yvec_init mld_yvec_init_eager
#define mld_yvec_get_poly mld_yvec_get_poly_eager
#define mld_polyvec_matrix_pointwise_montgomery_yvec \
  mld_polyvec_matrix_pointwise_montgomery_yvec_eager
#endif /* !MLD_CONFIG_REDUCE_RAM */

#endif /* !MLD_CONFIG_NO_KEYPAIR_API || !MLD_CONFIG_NO_SIGN_API || \
          !MLD_CONFIG_NO_VERIFY_API */
#endif /* !MLD_POLYVEC_LAZY_H */
