/*
 * Copyright (c) The mldsa-native project authors
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/* References
 * ==========
 *
 * - [FIPS204]
 *   FIPS 204 Module-Lattice-Based Digital Signature Standard
 *   National Institute of Standards and Technology
 *   https://csrc.nist.gov/pubs/fips/204/final
 *
 * - [REF]
 *   CRYSTALS-Dilithium reference implementation
 *   Bai, Ducas, Kiltz, Lepoint, Lyubashevsky, Schwabe, Seiler, Stehlé
 *   https://github.com/pq-crystals/dilithium/tree/master/ref
 */

#include "poly_kl.h"

#include "ct.h"
#include "debug.h"
#include "rounding.h"
#include "symmetric.h"

/* Parameter set namespacing
 * This is to facilitate building multiple instances
 * of mldsa-native (e.g. with varying parameter sets)
 * within a single compilation unit. */
#define mld_rej_eta MLD_ADD_PARAM_SET(mld_rej_eta)
#define mld_rej_eta_c MLD_ADD_PARAM_SET(mld_rej_eta_c)
#define mld_poly_decompose_c MLD_ADD_PARAM_SET(mld_poly_decompose_c)
#define mld_poly_use_hint_c MLD_ADD_PARAM_SET(mld_poly_use_hint_c)
#define mld_polyz_unpack_c MLD_ADD_PARAM_SET(mld_polyz_unpack_c)
/* End of parameter set namespacing */


#if !defined(MLD_CONFIG_NO_SIGN_API)
MLD_STATIC_TESTABLE
void mld_poly_decompose_c(mld_poly *a1, mld_poly *a0)
__contract__(
  requires(memory_no_alias(a1,  sizeof(mld_poly)))
  requires(memory_no_alias(a0, sizeof(mld_poly)))
  requires(array_bound(a0->coeffs, 0, MLDSA_N, 0, MLDSA_Q))
  assigns(memory_slice(a1, sizeof(mld_poly)))
  assigns(memory_slice(a0, sizeof(mld_poly)))
  ensures(array_bound(a1->coeffs, 0, MLDSA_N, 0, (MLDSA_Q-1)/(2*MLDSA_GAMMA2)))
  ensures(array_abs_bound(a0->coeffs, 0, MLDSA_N, MLDSA_GAMMA2+1))
)
{
  unsigned int i;
  mld_assert_bound(a0->coeffs, MLDSA_N, 0, MLDSA_Q);
  for (i = 0; i < MLDSA_N; ++i)
  __loop__(
    assigns(i, memory_slice(a0, sizeof(mld_poly)), memory_slice(a1, sizeof(mld_poly)))
    invariant(i <= MLDSA_N)
    invariant(array_bound(a0->coeffs, i, MLDSA_N, 0, MLDSA_Q))
    invariant(array_bound(a1->coeffs, 0, i, 0, (MLDSA_Q-1)/(2*MLDSA_GAMMA2)))
    invariant(array_abs_bound(a0->coeffs, 0, i, MLDSA_GAMMA2+1))
    decreases(MLDSA_N - i)
  )
  {
    mld_decompose(&a0->coeffs[i], &a1->coeffs[i], a0->coeffs[i]);
  }

  mld_assert_abs_bound(a0->coeffs, MLDSA_N, MLDSA_GAMMA2 + 1);
  mld_assert_bound(a1->coeffs, MLDSA_N, 0, (MLDSA_Q - 1) / (2 * MLDSA_GAMMA2));
}

MLD_INTERNAL_API
void mld_poly_decompose(mld_poly *a1, mld_poly *a0)
{
#if defined(MLD_USE_NATIVE_POLY_DECOMPOSE_88) && MLD_CONFIG_PARAMETER_SET == 44
  int ret;
  mld_assert_bound(a0->coeffs, MLDSA_N, 0, MLDSA_Q);
  ret = mld_poly_decompose_88_native(a1->coeffs, a0->coeffs);
  if (ret == MLD_NATIVE_FUNC_SUCCESS)
  {
    mld_assert_abs_bound(a0->coeffs, MLDSA_N, MLDSA_GAMMA2 + 1);
    mld_assert_bound(a1->coeffs, MLDSA_N, 0,
                     (MLDSA_Q - 1) / (2 * MLDSA_GAMMA2));
    return;
  }
#elif defined(MLD_USE_NATIVE_POLY_DECOMPOSE_32) && \
    (MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_PARAMETER_SET == 87)
  int ret;
  mld_assert_bound(a0->coeffs, MLDSA_N, 0, MLDSA_Q);
  ret = mld_poly_decompose_32_native(a1->coeffs, a0->coeffs);
  if (ret == MLD_NATIVE_FUNC_SUCCESS)
  {
    mld_assert_abs_bound(a0->coeffs, MLDSA_N, MLDSA_GAMMA2 + 1);
    mld_assert_bound(a1->coeffs, MLDSA_N, 0,
                     (MLDSA_Q - 1) / (2 * MLDSA_GAMMA2));
    return;
  }
#endif /* !(MLD_USE_NATIVE_POLY_DECOMPOSE_88 && MLD_CONFIG_PARAMETER_SET ==    \
          44) && MLD_USE_NATIVE_POLY_DECOMPOSE_32 && (MLD_CONFIG_PARAMETER_SET \
          == 65 || MLD_CONFIG_PARAMETER_SET == 87) */
  mld_poly_decompose_c(a1, a0);
}

#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_VERIFY_API)
MLD_STATIC_TESTABLE void mld_poly_use_hint_c(mld_poly *a, const mld_poly *h)
__contract__(
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(memory_no_alias(h, sizeof(mld_poly)))
  requires(array_bound(a->coeffs, 0, MLDSA_N, 0, MLDSA_Q))
  requires(array_bound(h->coeffs, 0, MLDSA_N, 0, 2))
  assigns(memory_slice(a, sizeof(mld_poly)))
  ensures(array_bound(a->coeffs, 0, MLDSA_N, 0, (MLDSA_Q-1)/(2*MLDSA_GAMMA2)))
)
{
  unsigned int i;
  mld_assert_bound(a->coeffs, MLDSA_N, 0, MLDSA_Q);
  mld_assert_bound(h->coeffs, MLDSA_N, 0, 2);

  for (i = 0; i < MLDSA_N; ++i)
  __loop__(
    invariant(i <= MLDSA_N)
    invariant(array_bound(a->coeffs, 0, i, 0, (MLDSA_Q-1)/(2*MLDSA_GAMMA2)))
    invariant(array_bound(a->coeffs, i, MLDSA_N, 0, MLDSA_Q))
    decreases(MLDSA_N - i)
  )
  {
    a->coeffs[i] = mld_use_hint(a->coeffs[i], h->coeffs[i]);
  }
  mld_assert_bound(a->coeffs, MLDSA_N, 0, (MLDSA_Q - 1) / (2 * MLDSA_GAMMA2));
}

MLD_INTERNAL_API
void mld_poly_use_hint(mld_poly *a, const mld_poly *h)
{
#if defined(MLD_USE_NATIVE_POLY_USE_HINT_88) && MLD_CONFIG_PARAMETER_SET == 44
  int ret;
  mld_assert_bound(a->coeffs, MLDSA_N, 0, MLDSA_Q);
  mld_assert_bound(h->coeffs, MLDSA_N, 0, 2);
  ret = mld_poly_use_hint_88_native(a->coeffs, h->coeffs);
  if (ret == MLD_NATIVE_FUNC_SUCCESS)
  {
    mld_assert_bound(a->coeffs, MLDSA_N, 0, (MLDSA_Q - 1) / (2 * MLDSA_GAMMA2));
    return;
  }
#elif defined(MLD_USE_NATIVE_POLY_USE_HINT_32) && \
    (MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_PARAMETER_SET == 87)
  int ret;
  mld_assert_bound(a->coeffs, MLDSA_N, 0, MLDSA_Q);
  mld_assert_bound(h->coeffs, MLDSA_N, 0, 2);
  ret = mld_poly_use_hint_32_native(a->coeffs, h->coeffs);
  if (ret == MLD_NATIVE_FUNC_SUCCESS)
  {
    mld_assert_bound(a->coeffs, MLDSA_N, 0, (MLDSA_Q - 1) / (2 * MLDSA_GAMMA2));
    return;
  }
#endif /* !(MLD_USE_NATIVE_POLY_USE_HINT_88 && MLD_CONFIG_PARAMETER_SET == 44) \
          && MLD_USE_NATIVE_POLY_USE_HINT_32 && (MLD_CONFIG_PARAMETER_SET ==   \
          65 || MLD_CONFIG_PARAMETER_SET == 87) */
  mld_poly_use_hint_c(a, h);
}
#endif /* !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
/**
 * Sample uniformly random coefficients in [-MLDSA_ETA, MLDSA_ETA] by
 * performing rejection sampling on an array of random bytes.
 *
 * @param[out] a      Pointer to output array (allocated).
 * @param      target Requested number of coefficients to sample.
 * @param      offset Number of coefficients already sampled.
 * @param[in]  buf    Array of random bytes to sample from.
 * @param      buflen Length of array of random bytes.
 *
 * @return Number of sampled coefficients. Can be smaller than target if not
 *         enough random bytes were given.
 */

/* Reference: `mld_rej_eta()` in the reference implementation @[REF].
 *            - Our signature differs from the reference implementation
 *              in that it adds the offset and always expects the base of the
 *              target buffer. This avoids shifting the buffer base in the
 *              caller, which appears tricky to reason about. */
#if MLDSA_ETA == 2
/*
 * Sampling 256 coefficients mod 15 using rejection sampling from 4 bits.
 * Expected number of required bytes: (256 * (16/15))/2 = 136.5 bytes.
 * We sample 1 block (=136 bytes) of SHAKE256_RATE output initially.
 * Sampling 2 blocks initially results in slightly worse performance.
 */
#define MLD_POLY_UNIFORM_ETA_NBLOCKS 1
#elif MLDSA_ETA == 4
/*
 * Sampling 256 coefficients mod 9 using rejection sampling from 4 bits.
 * Expected number of required bytes: (256 * (16/9))/2 = 227.5 bytes.
 * We sample 2 blocks (=272 bytes) of SHAKE256_RATE output initially.
 */
#define MLD_POLY_UNIFORM_ETA_NBLOCKS 2
#else /* MLDSA_ETA == 4 */
#error "Invalid value of MLDSA_ETA"
#endif /* MLDSA_ETA != 2 && MLDSA_ETA != 4 */

MLD_STATIC_TESTABLE unsigned int mld_rej_eta_c(int32_t *a, unsigned int target,
                                               unsigned int offset,
                                               const uint8_t *buf,
                                               unsigned int buflen)
__contract__(
  requires(offset <= target && target <= MLDSA_N)
  requires(buflen <= (MLD_POLY_UNIFORM_ETA_NBLOCKS * MLD_STREAM256_BLOCKBYTES))
  requires(memory_no_alias(a, sizeof(int32_t) * target))
  requires(memory_no_alias(buf, buflen))
  requires(array_abs_bound(a, 0, offset, MLDSA_ETA + 1))
  assigns(memory_slice(a, sizeof(int32_t) * target))
  ensures(offset <= return_value && return_value <= target)
  ensures(array_abs_bound(a, 0, return_value, MLDSA_ETA + 1))
)
{
  unsigned int ctr, pos;
  int t_valid;
  uint32_t t0, t1;
  mld_assert_abs_bound(a, offset, MLDSA_ETA + 1);
  ctr = offset;
  pos = 0;
  while (ctr < target && pos < buflen)
  __loop__(
    invariant(offset <= ctr && ctr <= target && pos <= buflen)
    invariant(array_abs_bound(a, 0, ctr, MLDSA_ETA + 1))
    decreases(buflen - pos)
  )
  {
    t0 = buf[pos] & 0x0F;
    t1 = buf[pos++] >> 4;

    /* Constant time: The inputs and outputs to the rejection sampling are
     * secret. However, it is fine to leak which coefficients have been
     * rejected. For constant-time testing, we declassify the result of
     * the comparison.
     */
#if MLDSA_ETA == 2
    t_valid = t0 < 15;
    MLD_CT_TESTING_DECLASSIFY(&t_valid, sizeof(int));
    if (t_valid) /* t0 < 15 */
    {
      t0 = t0 - (205 * t0 >> 10) * 5;
      a[ctr++] = 2 - (int32_t)t0;
    }
    t_valid = t1 < 15;
    MLD_CT_TESTING_DECLASSIFY(&t_valid, sizeof(int));
    if (t_valid && ctr < target) /* t1 < 15 */
    {
      t1 = t1 - (205 * t1 >> 10) * 5;
      a[ctr++] = 2 - (int32_t)t1;
    }
#elif MLDSA_ETA == 4
    t_valid = t0 < 9;
    MLD_CT_TESTING_DECLASSIFY(&t_valid, sizeof(int));
    if (t_valid) /* t0 < 9 */
    {
      a[ctr++] = 4 - (int32_t)t0;
    }
    t_valid = t1 < 9; /* t1 < 9 */
    MLD_CT_TESTING_DECLASSIFY(&t_valid, sizeof(int));
    if (t_valid && ctr < target)
    {
      a[ctr++] = 4 - (int32_t)t1;
    }
#else /* MLDSA_ETA == 4 */
#error "Invalid value of MLDSA_ETA"
#endif /* MLDSA_ETA != 2 && MLDSA_ETA != 4 */
  }

  mld_assert_abs_bound(a, ctr, MLDSA_ETA + 1);

  return ctr;
}

static unsigned int mld_rej_eta(int32_t *a, unsigned int target,
                                unsigned int offset, const uint8_t *buf,
                                unsigned int buflen)
__contract__(
  requires(offset <= target && target <= MLDSA_N)
  requires(buflen <= (MLD_POLY_UNIFORM_ETA_NBLOCKS * MLD_STREAM256_BLOCKBYTES))
  requires(memory_no_alias(a, sizeof(int32_t) * target))
  requires(memory_no_alias(buf, buflen))
  requires(array_abs_bound(a, 0, offset, MLDSA_ETA + 1))
  assigns(memory_slice(a, sizeof(int32_t) * target))
  ensures(offset <= return_value && return_value <= target)
  ensures(array_abs_bound(a, 0, return_value, MLDSA_ETA + 1))
)
{
#if MLDSA_ETA == 2 && defined(MLD_USE_NATIVE_REJ_UNIFORM_ETA2)
  int ret;
  mld_assert_abs_bound(a, offset, MLDSA_ETA + 1);
  if (offset == 0)
  {
    ret = mld_rej_uniform_eta2_native(a, target, buf, buflen);
    if (ret != MLD_NATIVE_FUNC_FALLBACK)
    {
      unsigned res = (unsigned)ret;
      mld_assert_abs_bound(a, res, MLDSA_ETA + 1);
      return res;
    }
  }
#elif MLDSA_ETA == 4 && defined(MLD_USE_NATIVE_REJ_UNIFORM_ETA4)
  int ret;
  mld_assert_abs_bound(a, offset, MLDSA_ETA + 1);
  if (offset == 0)
  {
    ret = mld_rej_uniform_eta4_native(a, target, buf, buflen);
    if (ret != MLD_NATIVE_FUNC_FALLBACK)
    {
      unsigned res = (unsigned)ret;
      mld_assert_abs_bound(a, res, MLDSA_ETA + 1);
      return res;
    }
  }
#endif /* !(MLDSA_ETA == 2 && MLD_USE_NATIVE_REJ_UNIFORM_ETA2) && MLDSA_ETA == \
          4 && MLD_USE_NATIVE_REJ_UNIFORM_ETA4 */

  return mld_rej_eta_c(a, target, offset, buf, buflen);
}

#if !defined(MLD_CONFIG_SERIAL_FIPS202_ONLY)
MLD_INTERNAL_API
void mld_poly_uniform_eta_4x(mld_poly *r0, mld_poly *r1, mld_poly *r2,
                             mld_poly *r3, const uint8_t seed[MLDSA_CRHBYTES],
                             uint8_t nonce0, uint8_t nonce1, uint8_t nonce2,
                             uint8_t nonce3)
{
  /* Temporary buffers for XOF output before rejection sampling */
  MLD_ALIGN uint8_t buf[4][MLD_ALIGN_UP(MLD_POLY_UNIFORM_ETA_NBLOCKS *
                                        MLD_STREAM256_BLOCKBYTES)];

  MLD_ALIGN uint8_t extseed[4][MLD_ALIGN_UP(MLDSA_CRHBYTES + 2)];

  /* Tracks the number of coefficients we have already sampled */
  unsigned ctr[4];
  mld_xof256_x4_ctx state;
  unsigned buflen;

  mld_memcpy(extseed[0], seed, MLDSA_CRHBYTES);
  mld_memcpy(extseed[1], seed, MLDSA_CRHBYTES);
  mld_memcpy(extseed[2], seed, MLDSA_CRHBYTES);
  mld_memcpy(extseed[3], seed, MLDSA_CRHBYTES);
  extseed[0][MLDSA_CRHBYTES] = nonce0;
  extseed[1][MLDSA_CRHBYTES] = nonce1;
  extseed[2][MLDSA_CRHBYTES] = nonce2;
  extseed[3][MLDSA_CRHBYTES] = nonce3;
  extseed[0][MLDSA_CRHBYTES + 1] = 0;
  extseed[1][MLDSA_CRHBYTES + 1] = 0;
  extseed[2][MLDSA_CRHBYTES + 1] = 0;
  extseed[3][MLDSA_CRHBYTES + 1] = 0;

  mld_xof256_x4_init(&state);
  mld_xof256_x4_absorb(&state, extseed, MLDSA_CRHBYTES + 2);

  /*
   * Initially, squeeze heuristic number of MLD_POLY_UNIFORM_ETA_NBLOCKS.
   * This should generate the coefficients with high probability.
   */
  mld_xof256_x4_squeezeblocks(buf, MLD_POLY_UNIFORM_ETA_NBLOCKS, &state);
  buflen = MLD_POLY_UNIFORM_ETA_NBLOCKS * MLD_STREAM256_BLOCKBYTES;

  ctr[0] = mld_rej_eta(r0->coeffs, MLDSA_N, 0, buf[0], buflen);
  ctr[1] = mld_rej_eta(r1->coeffs, MLDSA_N, 0, buf[1], buflen);
  ctr[2] = mld_rej_eta(r2->coeffs, MLDSA_N, 0, buf[2], buflen);
  ctr[3] = mld_rej_eta(r3->coeffs, MLDSA_N, 0, buf[3], buflen);

  /*
   * So long as not all entries have been generated, squeeze
   * one more block at a time until we're done.
   */
  buflen = MLD_STREAM256_BLOCKBYTES;
  while (ctr[0] < MLDSA_N || ctr[1] < MLDSA_N || ctr[2] < MLDSA_N ||
         ctr[3] < MLDSA_N)
  __loop__(
    assigns(ctr, state, memory_slice(r0, sizeof(mld_poly)),
            memory_slice(r1, sizeof(mld_poly)), memory_slice(r2, sizeof(mld_poly)),
            memory_slice(r3, sizeof(mld_poly)), object_whole(buf[0]),
            object_whole(buf[1]), object_whole(buf[2]),
            object_whole(buf[3]))
    invariant(ctr[0] <= MLDSA_N && ctr[1] <= MLDSA_N)
    invariant(ctr[2] <= MLDSA_N && ctr[3] <= MLDSA_N)
    invariant(array_abs_bound(r0->coeffs, 0, ctr[0], MLDSA_ETA + 1))
    invariant(array_abs_bound(r1->coeffs, 0, ctr[1], MLDSA_ETA + 1))
    invariant(array_abs_bound(r2->coeffs, 0, ctr[2], MLDSA_ETA + 1))
    invariant(array_abs_bound(r3->coeffs, 0, ctr[3], MLDSA_ETA + 1)))
  {
    mld_xof256_x4_squeezeblocks(buf, 1, &state);
    ctr[0] = mld_rej_eta(r0->coeffs, MLDSA_N, ctr[0], buf[0], buflen);
    ctr[1] = mld_rej_eta(r1->coeffs, MLDSA_N, ctr[1], buf[1], buflen);
    ctr[2] = mld_rej_eta(r2->coeffs, MLDSA_N, ctr[2], buf[2], buflen);
    ctr[3] = mld_rej_eta(r3->coeffs, MLDSA_N, ctr[3], buf[3], buflen);
  }

  mld_xof256_x4_release(&state);

  mld_assert_abs_bound(r0->coeffs, MLDSA_N, MLDSA_ETA + 1);
  mld_assert_abs_bound(r1->coeffs, MLDSA_N, MLDSA_ETA + 1);
  mld_assert_abs_bound(r2->coeffs, MLDSA_N, MLDSA_ETA + 1);
  mld_assert_abs_bound(r3->coeffs, MLDSA_N, MLDSA_ETA + 1);

  /* @[FIPS204, Section 3.6.3] Destruction of intermediate values. */
  mld_zeroize(buf, sizeof(buf));
  mld_zeroize(extseed, sizeof(extseed));
}
#else  /* !MLD_CONFIG_SERIAL_FIPS202_ONLY */

MLD_INTERNAL_API
void mld_poly_uniform_eta(mld_poly *r, const uint8_t seed[MLDSA_CRHBYTES],
                          uint8_t nonce)
{
  /* Temporary buffer for XOF output before rejection sampling */
  MLD_ALIGN uint8_t
      buf[MLD_POLY_UNIFORM_ETA_NBLOCKS * MLD_STREAM256_BLOCKBYTES];
  MLD_ALIGN uint8_t extseed[MLDSA_CRHBYTES + 2];

  /* Tracks the number of coefficients we have already sampled */
  unsigned ctr;
  mld_xof256_ctx state;
  unsigned buflen;

  mld_memcpy(extseed, seed, MLDSA_CRHBYTES);
  extseed[MLDSA_CRHBYTES] = nonce;
  extseed[MLDSA_CRHBYTES + 1] = 0;

  mld_xof256_init(&state);
  mld_xof256_absorb_once(&state, extseed, MLDSA_CRHBYTES + 2);

  /*
   * Initially, squeeze heuristic number of MLD_POLY_UNIFORM_ETA_NBLOCKS.
   * This should generate the coefficients with high probability.
   */
  mld_xof256_squeezeblocks(buf, MLD_POLY_UNIFORM_ETA_NBLOCKS, &state);
  buflen = MLD_POLY_UNIFORM_ETA_NBLOCKS * MLD_STREAM256_BLOCKBYTES;

  ctr = mld_rej_eta(r->coeffs, MLDSA_N, 0, buf, buflen);

  /*
   * So long as not all entries have been generated, squeeze
   * one more block at a time until we're done.
   */
  buflen = MLD_STREAM256_BLOCKBYTES;
  while (ctr < MLDSA_N)
  __loop__(
    assigns(ctr, object_whole(&state),
      object_whole(buf), memory_slice(r, sizeof(mld_poly)))
    invariant(ctr <= MLDSA_N)
    invariant(state.pos <= SHAKE256_RATE)
    invariant(array_abs_bound(r->coeffs, 0, ctr, MLDSA_ETA + 1)))
  {
    mld_xof256_squeezeblocks(buf, 1, &state);
    ctr = mld_rej_eta(r->coeffs, MLDSA_N, ctr, buf, buflen);
  }

  mld_xof256_release(&state);

  mld_assert_abs_bound(r->coeffs, MLDSA_N, MLDSA_ETA + 1);

  /* @[FIPS204, Section 3.6.3] Destruction of intermediate values. */
  mld_zeroize(buf, sizeof(buf));
  mld_zeroize(extseed, sizeof(extseed));
}
#endif /* MLD_CONFIG_SERIAL_FIPS202_ONLY */
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
#define MLD_POLY_UNIFORM_GAMMA1_NBLOCKS                       \
  ((MLDSA_POLYZ_PACKEDBYTES + MLD_STREAM256_BLOCKBYTES - 1) / \
   MLD_STREAM256_BLOCKBYTES)

#if MLD_CONFIG_PARAMETER_SET == 65 ||              \
    defined(MLD_CONFIG_SERIAL_FIPS202_ONLY) ||     \
    defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
MLD_INTERNAL_API
void mld_poly_uniform_gamma1(mld_poly *a, const uint8_t seed[MLDSA_CRHBYTES],
                             uint16_t nonce)
{
  MLD_ALIGN uint8_t
      buf[MLD_POLY_UNIFORM_GAMMA1_NBLOCKS * MLD_STREAM256_BLOCKBYTES];
  MLD_ALIGN uint8_t extseed[MLDSA_CRHBYTES + 2];
  mld_xof256_ctx state;

  mld_memcpy(extseed, seed, MLDSA_CRHBYTES);
  extseed[MLDSA_CRHBYTES] = (uint8_t)(nonce & 0xFF);
  extseed[MLDSA_CRHBYTES + 1] = (uint8_t)(nonce >> 8);

  mld_xof256_init(&state);
  mld_xof256_absorb_once(&state, extseed, MLDSA_CRHBYTES + 2);

  mld_xof256_squeezeblocks(buf, MLD_POLY_UNIFORM_GAMMA1_NBLOCKS, &state);
  mld_polyz_unpack(a, buf);

  mld_xof256_release(&state);

  mld_assert_bound(a->coeffs, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1);

  /* @[FIPS204, Section 3.6.3] Destruction of intermediate values. */
  mld_zeroize(buf, sizeof(buf));
  mld_zeroize(extseed, sizeof(extseed));
}
#endif /* MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_SERIAL_FIPS202_ONLY || \
          MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */


#if !defined(MLD_CONFIG_SERIAL_FIPS202_ONLY) && \
    (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST))
MLD_INTERNAL_API
void mld_poly_uniform_gamma1_4x(mld_poly *r0, mld_poly *r1, mld_poly *r2,
                                mld_poly *r3,
                                const uint8_t seed[MLDSA_CRHBYTES],
                                uint16_t nonce0, uint16_t nonce1,
                                uint16_t nonce2, uint16_t nonce3)
{
  /* Temporary buffers for XOF output before rejection sampling */
  MLD_ALIGN uint8_t buf[4][MLD_ALIGN_UP(MLD_POLY_UNIFORM_GAMMA1_NBLOCKS *
                                        MLD_STREAM256_BLOCKBYTES)];

  MLD_ALIGN uint8_t extseed[4][MLD_ALIGN_UP(MLDSA_CRHBYTES + 2)];

  /* Tracks the number of coefficients we have already sampled */
  mld_xof256_x4_ctx state;

  mld_memcpy(extseed[0], seed, MLDSA_CRHBYTES);
  mld_memcpy(extseed[1], seed, MLDSA_CRHBYTES);
  mld_memcpy(extseed[2], seed, MLDSA_CRHBYTES);
  mld_memcpy(extseed[3], seed, MLDSA_CRHBYTES);
  extseed[0][MLDSA_CRHBYTES] = (uint8_t)(nonce0 & 0xFF);
  extseed[1][MLDSA_CRHBYTES] = (uint8_t)(nonce1 & 0xFF);
  extseed[2][MLDSA_CRHBYTES] = (uint8_t)(nonce2 & 0xFF);
  extseed[3][MLDSA_CRHBYTES] = (uint8_t)(nonce3 & 0xFF);
  extseed[0][MLDSA_CRHBYTES + 1] = (uint8_t)(nonce0 >> 8);
  extseed[1][MLDSA_CRHBYTES + 1] = (uint8_t)(nonce1 >> 8);
  extseed[2][MLDSA_CRHBYTES + 1] = (uint8_t)(nonce2 >> 8);
  extseed[3][MLDSA_CRHBYTES + 1] = (uint8_t)(nonce3 >> 8);

  mld_xof256_x4_init(&state);
  mld_xof256_x4_absorb(&state, extseed, MLDSA_CRHBYTES + 2);
  mld_xof256_x4_squeezeblocks(buf, MLD_POLY_UNIFORM_GAMMA1_NBLOCKS, &state);

  mld_polyz_unpack(r0, buf[0]);
  mld_polyz_unpack(r1, buf[1]);
  mld_polyz_unpack(r2, buf[2]);
  mld_polyz_unpack(r3, buf[3]);
  mld_xof256_x4_release(&state);

  mld_assert_bound(r0->coeffs, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1);
  mld_assert_bound(r1->coeffs, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1);
  mld_assert_bound(r2->coeffs, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1);
  mld_assert_bound(r3->coeffs, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1);

  /* @[FIPS204, Section 3.6.3] Destruction of intermediate values. */
  mld_zeroize(buf, sizeof(buf));
  mld_zeroize(extseed, sizeof(extseed));
}
#endif /* !MLD_CONFIG_SERIAL_FIPS202_ONLY && (!MLD_CONFIG_REDUCE_RAM || \
          MLD_UNIT_TEST) */
#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
MLD_INTERNAL_API
void mld_poly_challenge(mld_poly *c, const uint8_t seed[MLDSA_CTILDEBYTES])
{
  unsigned int i, j, pos;
  uint64_t signs;
  uint64_t offset;
  MLD_ALIGN uint8_t buf[SHAKE256_RATE];
  mld_shake256ctx state;

  mld_shake256_init(&state);
  mld_shake256_absorb(&state, seed, MLDSA_CTILDEBYTES);
  mld_shake256_finalize(&state);
  mld_shake256_squeeze(buf, SHAKE256_RATE, &state);

  /* Convert the first 8 bytes of buf[] into an unsigned 64-bit value.   */
  /* Each bit of that dictates the sign of the resulting challenge value */
  signs = 0;
  for (i = 0; i < 8; ++i)
  __loop__(
    assigns(i, signs)
    invariant(i <= 8)
    decreases(8 - i)
  )
  {
    signs |= (uint64_t)buf[i] << 8 * i;
  }
  pos = 8;

  mld_memset(c, 0, sizeof(mld_poly));

  for (i = MLDSA_N - MLDSA_TAU; i < MLDSA_N; ++i)
  __loop__(
    assigns(i, j, object_whole(buf), state, pos, memory_slice(c, sizeof(mld_poly)), signs)
    invariant(i >= MLDSA_N - MLDSA_TAU)
    invariant(i <= MLDSA_N)
    invariant(pos >= 1)
    invariant(pos <= SHAKE256_RATE)
    invariant(array_bound(c->coeffs, 0, MLDSA_N, -1, 2))
    invariant(state.pos <= SHAKE256_RATE)
    decreases(MLDSA_N - i)
  )
  {
    /* This loop teminates only probabilistically, hence no decreases clause. */
    do
    __loop__(
      assigns(j, object_whole(buf), state, pos)
      invariant(state.pos <= SHAKE256_RATE)
    )
    {
      if (pos >= SHAKE256_RATE)
      {
        mld_shake256_squeeze(buf, SHAKE256_RATE, &state);
        pos = 0;
      }
      j = buf[pos++];
    } while (j > i);

    c->coeffs[i] = c->coeffs[j];

    /* Reference: Compute coefficent value here in two steps to */
    /* mixinf unsigned and signed arithmetic with implicit      */
    /* conversions, and so that CBMC can keep track of ranges   */
    /* to complete type-safety proof here.                      */

    /* The least-significant bit of signs tells us if we want -1 or +1 */
    offset = 2 * (signs & 1);

    /* offset has value 0 or 2 here, so (1 - (int32_t) offset) has
     * value -1 or +1 */
    c->coeffs[j] = 1 - (int32_t)offset;

    /* Move to the next bit of signs for next time */
    signs >>= 1;
  }

  mld_assert_bound(c->coeffs, MLDSA_N, -1, 2);
  mld_shake256_release(&state);

  /* @[FIPS204, Section 3.6.3] Destruction of intermediate values. */
  mld_zeroize(buf, sizeof(buf));
  mld_zeroize(&signs, sizeof(signs));
}
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
MLD_INTERNAL_API
void mld_polyeta_pack(uint8_t r[MLDSA_POLYETA_PACKEDBYTES], const mld_poly *a)
{
  unsigned int i;
  uint8_t t[8];

  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLDSA_ETA + 1);

#if MLDSA_ETA == 2
  for (i = 0; i < MLDSA_N / 8; ++i)
  __loop__(
    invariant(i <= MLDSA_N/8)
    decreases(MLDSA_N / 8 - i))
  {
    /* The casts are safe since we assume that the coefficients
     * of a are <= MLDSA_ETA in absolute value. */
    t[0] = (uint8_t)(MLDSA_ETA - a->coeffs[8 * i + 0]);
    t[1] = (uint8_t)(MLDSA_ETA - a->coeffs[8 * i + 1]);
    t[2] = (uint8_t)(MLDSA_ETA - a->coeffs[8 * i + 2]);
    t[3] = (uint8_t)(MLDSA_ETA - a->coeffs[8 * i + 3]);
    t[4] = (uint8_t)(MLDSA_ETA - a->coeffs[8 * i + 4]);
    t[5] = (uint8_t)(MLDSA_ETA - a->coeffs[8 * i + 5]);
    t[6] = (uint8_t)(MLDSA_ETA - a->coeffs[8 * i + 6]);
    t[7] = (uint8_t)(MLDSA_ETA - a->coeffs[8 * i + 7]);

    r[3 * i + 0] = (uint8_t)(((t[0] >> 0) | (t[1] << 3) | (t[2] << 6)) & 0xFF);
    r[3 * i + 1] =
        (uint8_t)(((t[2] >> 2) | (t[3] << 1) | (t[4] << 4) | (t[5] << 7)) &
                  0xFF);
    r[3 * i + 2] = (uint8_t)(((t[5] >> 1) | (t[6] << 2) | (t[7] << 5)) & 0xFF);
  }
#elif MLDSA_ETA == 4
  for (i = 0; i < MLDSA_N / 2; ++i)
  __loop__(
    invariant(i <= MLDSA_N/2)
    decreases(MLDSA_N / 2 - i))
  {
    /* The casts are safe since we assume that the coefficients
     * of a are <= MLDSA_ETA in absolute value. */
    t[0] = (uint8_t)(MLDSA_ETA - a->coeffs[2 * i + 0]);
    t[1] = (uint8_t)(MLDSA_ETA - a->coeffs[2 * i + 1]);
    r[i] = (uint8_t)(t[0] | (t[1] << 4));
  }
#else /* MLDSA_ETA == 4 */
#error "Invalid value of MLDSA_ETA"
#endif /* MLDSA_ETA != 2 && MLDSA_ETA != 4 */
}
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API) || !defined(MLD_CONFIG_NO_SIGN_API)
void mld_polyeta_unpack(mld_poly *r, const uint8_t a[MLDSA_POLYETA_PACKEDBYTES])
{
  unsigned int i;

#if MLDSA_ETA == 2
  for (i = 0; i < MLDSA_N / 8; ++i)
  __loop__(
    invariant(i <= MLDSA_N/8)
    invariant(array_bound(r->coeffs, 0, i*8, -5, MLDSA_ETA + 1))
    decreases(MLDSA_N / 8 - i))
  {
    r->coeffs[8 * i + 0] = (a[3 * i + 0] >> 0) & 7;
    r->coeffs[8 * i + 1] = (a[3 * i + 0] >> 3) & 7;
    r->coeffs[8 * i + 2] = ((a[3 * i + 0] >> 6) | (a[3 * i + 1] << 2)) & 7;
    r->coeffs[8 * i + 3] = (a[3 * i + 1] >> 1) & 7;
    r->coeffs[8 * i + 4] = (a[3 * i + 1] >> 4) & 7;
    r->coeffs[8 * i + 5] = ((a[3 * i + 1] >> 7) | (a[3 * i + 2] << 1)) & 7;
    r->coeffs[8 * i + 6] = (a[3 * i + 2] >> 2) & 7;
    r->coeffs[8 * i + 7] = (a[3 * i + 2] >> 5) & 7;

    r->coeffs[8 * i + 0] = MLDSA_ETA - r->coeffs[8 * i + 0];
    r->coeffs[8 * i + 1] = MLDSA_ETA - r->coeffs[8 * i + 1];
    r->coeffs[8 * i + 2] = MLDSA_ETA - r->coeffs[8 * i + 2];
    r->coeffs[8 * i + 3] = MLDSA_ETA - r->coeffs[8 * i + 3];
    r->coeffs[8 * i + 4] = MLDSA_ETA - r->coeffs[8 * i + 4];
    r->coeffs[8 * i + 5] = MLDSA_ETA - r->coeffs[8 * i + 5];
    r->coeffs[8 * i + 6] = MLDSA_ETA - r->coeffs[8 * i + 6];
    r->coeffs[8 * i + 7] = MLDSA_ETA - r->coeffs[8 * i + 7];
  }
#elif MLDSA_ETA == 4
  for (i = 0; i < MLDSA_N / 2; ++i)
  __loop__(
    invariant(i <= MLDSA_N/2)
    invariant(array_bound(r->coeffs, 0, i*2, -11, MLDSA_ETA + 1))
    decreases(MLDSA_N / 2 - i))
  {
    r->coeffs[2 * i + 0] = a[i] & 0x0F;
    r->coeffs[2 * i + 1] = a[i] >> 4;
    r->coeffs[2 * i + 0] = MLDSA_ETA - r->coeffs[2 * i + 0];
    r->coeffs[2 * i + 1] = MLDSA_ETA - r->coeffs[2 * i + 1];
  }
#else /* MLDSA_ETA == 4 */
#error "Invalid value of MLDSA_ETA"
#endif /* MLDSA_ETA != 2 && MLDSA_ETA != 4 */

  mld_assert_bound(r->coeffs, MLDSA_N, MLD_POLYETA_UNPACK_LOWER_BOUND,
                   MLDSA_ETA + 1);
}
#endif /* !MLD_CONFIG_NO_KEYPAIR_API || !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
MLD_INTERNAL_API
void mld_polyz_pack(uint8_t r[MLDSA_POLYZ_PACKEDBYTES], const mld_poly *a)
{
  unsigned int i;
  uint32_t t[4];

  mld_assert_bound(a->coeffs, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1);

#if MLD_CONFIG_PARAMETER_SET == 44
  for (i = 0; i < MLDSA_N / 4; ++i)
  __loop__(
    invariant(i <= MLDSA_N/4)
    decreases(MLDSA_N / 4 - i))
  {
    /* Safety: a->coeffs[i] <= MLDSA_GAMMA1, hence, these casts are safe. */
    t[0] = (uint32_t)(MLDSA_GAMMA1 - a->coeffs[4 * i + 0]);
    t[1] = (uint32_t)(MLDSA_GAMMA1 - a->coeffs[4 * i + 1]);
    t[2] = (uint32_t)(MLDSA_GAMMA1 - a->coeffs[4 * i + 2]);
    t[3] = (uint32_t)(MLDSA_GAMMA1 - a->coeffs[4 * i + 3]);

    r[9 * i + 0] = (uint8_t)((t[0]) & 0xFF);
    r[9 * i + 1] = (uint8_t)((t[0] >> 8) & 0xFF);
    r[9 * i + 2] = (uint8_t)((t[0] >> 16) & 0xFF);
    r[9 * i + 2] |= (uint8_t)((t[1] << 2) & 0xFF);
    r[9 * i + 3] = (uint8_t)((t[1] >> 6) & 0xFF);
    r[9 * i + 4] = (uint8_t)((t[1] >> 14) & 0xFF);
    r[9 * i + 4] |= (uint8_t)((t[2] << 4) & 0xFF);
    r[9 * i + 5] = (uint8_t)((t[2] >> 4) & 0xFF);
    r[9 * i + 6] = (uint8_t)((t[2] >> 12) & 0xFF);
    r[9 * i + 6] |= (uint8_t)((t[3] << 6) & 0xFF);
    r[9 * i + 7] = (uint8_t)((t[3] >> 2) & 0xFF);
    r[9 * i + 8] = (uint8_t)((t[3] >> 10) & 0xFF);
  }
#else  /* MLD_CONFIG_PARAMETER_SET == 44 */
  for (i = 0; i < MLDSA_N / 2; ++i)
  __loop__(
    invariant(i <= MLDSA_N/2)
    decreases(MLDSA_N / 2 - i))
  {
    /* Safety: a->coeffs[i] <= MLDSA_GAMMA1, hence, these casts are safe. */
    t[0] = (uint32_t)(MLDSA_GAMMA1 - a->coeffs[2 * i + 0]);
    t[1] = (uint32_t)(MLDSA_GAMMA1 - a->coeffs[2 * i + 1]);

    r[5 * i + 0] = (uint8_t)((t[0]) & 0xFF);
    r[5 * i + 1] = (uint8_t)((t[0] >> 8) & 0xFF);
    r[5 * i + 2] = (uint8_t)((t[0] >> 16) & 0xFF);
    r[5 * i + 2] |= (uint8_t)((t[1] << 4) & 0xFF);
    r[5 * i + 3] = (uint8_t)((t[1] >> 4) & 0xFF);
    r[5 * i + 4] = (uint8_t)((t[1] >> 12) & 0xFF);
  }
#endif /* MLD_CONFIG_PARAMETER_SET != 44 */
}
#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
MLD_STATIC_TESTABLE void mld_polyz_unpack_c(
    mld_poly *r, const uint8_t a[MLDSA_POLYZ_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(r, sizeof(mld_poly)))
  requires(memory_no_alias(a, MLDSA_POLYZ_PACKEDBYTES))
  assigns(memory_slice(r, sizeof(mld_poly)))
  ensures(array_bound(r->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
)
{
  unsigned int i;
#if MLD_CONFIG_PARAMETER_SET == 44
  for (i = 0; i < MLDSA_N / 4; ++i)
  __loop__(
    invariant(i <= MLDSA_N/4)
    invariant(array_bound(r->coeffs, 0, i*4, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
    decreases(MLDSA_N / 4 - i))
  {
    r->coeffs[4 * i + 0] = a[9 * i + 0];
    r->coeffs[4 * i + 0] |= (int32_t)a[9 * i + 1] << 8;
    r->coeffs[4 * i + 0] |= (int32_t)a[9 * i + 2] << 16;
    r->coeffs[4 * i + 0] &= 0x3FFFF;

    r->coeffs[4 * i + 1] = a[9 * i + 2] >> 2;
    r->coeffs[4 * i + 1] |= (int32_t)a[9 * i + 3] << 6;
    r->coeffs[4 * i + 1] |= (int32_t)a[9 * i + 4] << 14;
    r->coeffs[4 * i + 1] &= 0x3FFFF;

    r->coeffs[4 * i + 2] = a[9 * i + 4] >> 4;
    r->coeffs[4 * i + 2] |= (int32_t)a[9 * i + 5] << 4;
    r->coeffs[4 * i + 2] |= (int32_t)a[9 * i + 6] << 12;
    r->coeffs[4 * i + 2] &= 0x3FFFF;

    r->coeffs[4 * i + 3] = a[9 * i + 6] >> 6;
    r->coeffs[4 * i + 3] |= (int32_t)a[9 * i + 7] << 2;
    r->coeffs[4 * i + 3] |= (int32_t)a[9 * i + 8] << 10;
    r->coeffs[4 * i + 3] &= 0x3FFFF;

    r->coeffs[4 * i + 0] = MLDSA_GAMMA1 - r->coeffs[4 * i + 0];
    r->coeffs[4 * i + 1] = MLDSA_GAMMA1 - r->coeffs[4 * i + 1];
    r->coeffs[4 * i + 2] = MLDSA_GAMMA1 - r->coeffs[4 * i + 2];
    r->coeffs[4 * i + 3] = MLDSA_GAMMA1 - r->coeffs[4 * i + 3];
  }
#else  /* MLD_CONFIG_PARAMETER_SET == 44 */
  for (i = 0; i < MLDSA_N / 2; ++i)
  __loop__(
    invariant(i <= MLDSA_N/2)
    invariant(array_bound(r->coeffs, 0, i*2, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
    decreases(MLDSA_N / 2 - i))
  {
    r->coeffs[2 * i + 0] = a[5 * i + 0];
    r->coeffs[2 * i + 0] |= (int32_t)a[5 * i + 1] << 8;
    r->coeffs[2 * i + 0] |= (int32_t)a[5 * i + 2] << 16;
    r->coeffs[2 * i + 0] &= 0xFFFFF;

    r->coeffs[2 * i + 1] = a[5 * i + 2] >> 4;
    r->coeffs[2 * i + 1] |= (int32_t)a[5 * i + 3] << 4;
    r->coeffs[2 * i + 1] |= (int32_t)a[5 * i + 4] << 12;
    /* r->coeffs[2*i+1] &= 0xFFFFF; */ /* No effect, since we're anyway at 20
                                          bits */

    r->coeffs[2 * i + 0] = MLDSA_GAMMA1 - r->coeffs[2 * i + 0];
    r->coeffs[2 * i + 1] = MLDSA_GAMMA1 - r->coeffs[2 * i + 1];
  }
#endif /* MLD_CONFIG_PARAMETER_SET != 44 */
  mld_assert_bound(r->coeffs, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1);
}

MLD_INTERNAL_API
void mld_polyz_unpack(mld_poly *r, const uint8_t a[MLDSA_POLYZ_PACKEDBYTES])
{
#if defined(MLD_USE_NATIVE_POLYZ_UNPACK_17) && MLD_CONFIG_PARAMETER_SET == 44
  int ret;
  ret = mld_polyz_unpack_17_native(r->coeffs, a);
  if (ret == MLD_NATIVE_FUNC_SUCCESS)
  {
    mld_assert_bound(r->coeffs, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1);
    return;
  }
#elif defined(MLD_USE_NATIVE_POLYZ_UNPACK_19) && \
    (MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_PARAMETER_SET == 87)
  int ret;
  ret = mld_polyz_unpack_19_native(r->coeffs, a);
  if (ret == MLD_NATIVE_FUNC_SUCCESS)
  {
    mld_assert_bound(r->coeffs, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1);
    return;
  }
#endif /* !(MLD_USE_NATIVE_POLYZ_UNPACK_17 && MLD_CONFIG_PARAMETER_SET == 44)  \
          && MLD_USE_NATIVE_POLYZ_UNPACK_19 && (MLD_CONFIG_PARAMETER_SET == 65 \
          || MLD_CONFIG_PARAMETER_SET == 87) */

  mld_polyz_unpack_c(r, a);
}
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */

/* To facilitate single-compilation-unit (SCU) builds, undefine all macros. */

/* To facilitate single-compilation-unit (SCU) builds, undefine all macros.
 * Don't modify by hand -- this is auto-generated by scripts/autogen. */
#undef mld_rej_eta
#undef mld_rej_eta_c
#undef mld_poly_decompose_c
#undef mld_poly_use_hint_c
#undef mld_polyz_unpack_c
#undef MLD_POLY_UNIFORM_ETA_NBLOCKS
#undef MLD_POLY_UNIFORM_ETA_NBLOCKS
#undef MLD_POLY_UNIFORM_GAMMA1_NBLOCKS
