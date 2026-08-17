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

#include "common.h"
#if !defined(MLK_CONFIG_MULTILEVEL_NO_SHARED)

#include "debug.h"
#include "sampling.h"
#include "symmetric.h"

/* Reference: `rej_uniform()` in the reference implementation @[REF].
 *            - Our signature differs from the reference implementation
 *              in that it adds the offset and always expects the base of the
 *              target buffer. This avoids shifting the buffer base in the
 *              caller, which appears tricky to reason about. */
MLK_STATIC_TESTABLE unsigned mlk_rej_uniform_c(int16_t *r, unsigned target,
                                               unsigned offset,
                                               const uint8_t *buf,
                                               unsigned buflen)
__contract__(
  requires(offset <= target && target <= 4096 && buflen <= 4096 && buflen % 3 == 0)
  requires(memory_no_alias(r, sizeof(int16_t) * target))
  requires(memory_no_alias(buf, buflen))
  requires(array_bound(r, 0, offset, 0, MLKEM_Q))
  assigns(memory_slice(r, sizeof(int16_t) * target))
  ensures(offset <= return_value && return_value <= target)
  ensures(array_bound(r, 0, return_value, 0, MLKEM_Q)))
{
  unsigned ctr, pos;
  int16_t val0, val1;

  mlk_assert_bound(r, offset, 0, MLKEM_Q);

  ctr = offset;
  pos = 0;
  /* pos + 3 cannot overflow due to the assumption buflen <= 4096 */
  while (ctr < target && pos + 3 <= buflen)
  __loop__(
    invariant(offset <= ctr && ctr <= target && pos <= buflen)
    invariant(array_bound(r, 0, ctr, 0, MLKEM_Q)))
  {
    val0 = ((buf[pos + 0] >> 0) | (buf[pos + 1] << 8)) & 0xFFF;
    val1 = ((buf[pos + 1] >> 4) | (buf[pos + 2] << 4)) & 0xFFF;
    pos += 3;

    if (val0 < MLKEM_Q)
    {
      r[ctr++] = val0;
    }
    if (ctr < target && val1 < MLKEM_Q)
    {
      r[ctr++] = val1;
    }
  }

  mlk_assert_bound(r, ctr, 0, MLKEM_Q);
  return ctr;
}

/*************************************************
 * Name:        mlk_rej_uniform
 *
 * Description: Run rejection sampling on uniform random bytes to generate
 *              uniform random integers mod q
 *
 * Arguments:   - int16_t *r:          pointer to output buffer
 *              - unsigned target:     requested number of 16-bit integers
 *                                     (uniform mod q).
 *                                     Must be <= 4096.
 *              - unsigned offset:     number of 16-bit integers that have
 *                                     already been sampled.
 *                                     Must be <= target.
 *              - const uint8_t *buf:  pointer to input buffer
 *                                     (assumed to be uniform random bytes)
 *              - unsigned buflen:     length of input buffer in bytes
 *                                     Must be <= 4096.
 *                                     Must be a multiple of 3.
 *
 * Note: Strictly speaking, only a few values of buflen near UINT_MAX need
 * excluding. The limit of 4096 is somewhat arbitrary but sufficient for all
 * uses of this function. Similarly, the actual limit for target is UINT_MAX/2.
 *
 * Returns the new offset of sampled 16-bit integers, at most target,
 * and at least the initial offset.
 * If the new offset is strictly less than len, all of the input buffers
 * is guaranteed to have been consumed. If it is equal to len, no information
 * is provided on how many bytes of the input buffer have been consumed.
 **************************************************/

/* Reference: `rej_uniform()` in the reference implementation @[REF].
 *            - Our signature differs from the reference implementation
 *              in that it adds the offset and always expects the base of the
 *              target buffer. This avoids shifting the buffer base in the
 *              caller, which appears tricky to reason about.
 *            - Optional fallback to native implementation. */
static unsigned mlk_rej_uniform(int16_t *r, unsigned target, unsigned offset,
                                const uint8_t *buf, unsigned buflen)
__contract__(
  requires(offset <= target && target <= 4096 && buflen <= 4096 && buflen % 3 == 0)
  requires(memory_no_alias(r, sizeof(int16_t) * target))
  requires(memory_no_alias(buf, buflen))
  requires(array_bound(r, 0, offset, 0, MLKEM_Q))
  assigns(memory_slice(r, sizeof(int16_t) * target))
  ensures(offset <= return_value && return_value <= target)
  ensures(array_bound(r, 0, return_value, 0, MLKEM_Q))
)
{
#if defined(MLK_USE_NATIVE_REJ_UNIFORM)
  if (offset == 0)
  {
    int ret;
    ret = mlk_rej_uniform_native(r, target, buf, buflen);
    if (ret != MLK_NATIVE_FUNC_FALLBACK)
    {
      unsigned res = (unsigned)ret;
      mlk_assert_bound(r, res, 0, MLKEM_Q);
      return res;
    }
  }
#endif /* MLK_USE_NATIVE_REJ_UNIFORM */

  return mlk_rej_uniform_c(r, target, offset, buf, buflen);
}

#ifndef MLKEM_GEN_MATRIX_NBLOCKS
#define MLKEM_GEN_MATRIX_NBLOCKS                                       \
  ((12 * MLKEM_N / 8 * ((uint32_t)1 << 12) / MLKEM_Q + MLK_XOF_RATE) / \
   MLK_XOF_RATE)
#endif

#if !defined(MLK_CONFIG_SERIAL_FIPS202_ONLY)
/* Reference: Does not exist in the reference implementation @[REF].
 *            - x4-batched version of `rej_uniform()` from the
 *              reference implementation, leveraging x4-batched Keccak-f1600. */
MLK_INTERNAL_API
void mlk_poly_rej_uniform_x4(mlk_poly *vec0, mlk_poly *vec1, mlk_poly *vec2,
                             mlk_poly *vec3,
                             uint8_t seed[4][MLK_ALIGN_UP(MLKEM_SYMBYTES + 2)])
{
  /* Temporary buffers for XOF output before rejection sampling */
  MLK_ALIGN uint8_t
      buf[4][MLK_ALIGN_UP(MLKEM_GEN_MATRIX_NBLOCKS * MLK_XOF_RATE)];

  /* Tracks the number of coefficients we have already sampled */
  unsigned ctr[4];
  mlk_xof_x4_ctx statex;
  unsigned buflen;

  mlk_xof_x4_init(&statex);
  mlk_xof_x4_absorb(&statex, seed, MLKEM_SYMBYTES + 2);

  /*
   * Initially, squeeze heuristic number of MLKEM_GEN_MATRIX_NBLOCKS.
   * This should generate the matrix entries with high probability.
   */
  mlk_xof_x4_squeezeblocks(buf, MLKEM_GEN_MATRIX_NBLOCKS, &statex);
  buflen = MLKEM_GEN_MATRIX_NBLOCKS * MLK_XOF_RATE;
  ctr[0] = mlk_rej_uniform(vec0->coeffs, MLKEM_N, 0, buf[0], buflen);
  ctr[1] = mlk_rej_uniform(vec1->coeffs, MLKEM_N, 0, buf[1], buflen);
  ctr[2] = mlk_rej_uniform(vec2->coeffs, MLKEM_N, 0, buf[2], buflen);
  ctr[3] = mlk_rej_uniform(vec3->coeffs, MLKEM_N, 0, buf[3], buflen);

  /*
   * So long as not all matrix entries have been generated, squeeze
   * one more block a time until we're done.
   */
  buflen = MLK_XOF_RATE;
  while (ctr[0] < MLKEM_N || ctr[1] < MLKEM_N || ctr[2] < MLKEM_N ||
         ctr[3] < MLKEM_N)
  __loop__(
    assigns(ctr, statex,
            memory_slice(vec0, sizeof(mlk_poly)),
            memory_slice(vec1, sizeof(mlk_poly)),
            memory_slice(vec2, sizeof(mlk_poly)),
            memory_slice(vec3, sizeof(mlk_poly)),
            object_whole(buf))
    invariant(ctr[0] <= MLKEM_N && ctr[1] <= MLKEM_N)
    invariant(ctr[2] <= MLKEM_N && ctr[3] <= MLKEM_N)
    invariant(array_bound(vec0->coeffs, 0, ctr[0], 0, MLKEM_Q))
    invariant(array_bound(vec1->coeffs, 0, ctr[1], 0, MLKEM_Q))
    invariant(array_bound(vec2->coeffs, 0, ctr[2], 0, MLKEM_Q))
    invariant(array_bound(vec3->coeffs, 0, ctr[3], 0, MLKEM_Q)))
  {
    mlk_xof_x4_squeezeblocks(buf, 1, &statex);
    ctr[0] = mlk_rej_uniform(vec0->coeffs, MLKEM_N, ctr[0], buf[0], buflen);
    ctr[1] = mlk_rej_uniform(vec1->coeffs, MLKEM_N, ctr[1], buf[1], buflen);
    ctr[2] = mlk_rej_uniform(vec2->coeffs, MLKEM_N, ctr[2], buf[2], buflen);
    ctr[3] = mlk_rej_uniform(vec3->coeffs, MLKEM_N, ctr[3], buf[3], buflen);
  }

  mlk_xof_x4_release(&statex);

  /* Specification: Partially implements
   * @[FIPS203, Section 3.3, Destruction of intermediate values] */
  mlk_zeroize(buf, sizeof(buf));
}
#endif /* !MLK_CONFIG_SERIAL_FIPS202_ONLY */

MLK_INTERNAL_API
void mlk_poly_rej_uniform(mlk_poly *entry, uint8_t seed[MLKEM_SYMBYTES + 2])
{
  mlk_xof_ctx state;
  MLK_ALIGN uint8_t buf[MLKEM_GEN_MATRIX_NBLOCKS * MLK_XOF_RATE];
  unsigned ctr, buflen;

  mlk_xof_init(&state);
  mlk_xof_absorb(&state, seed, MLKEM_SYMBYTES + 2);

  /* Initially, squeeze + sample heuristic number of MLKEM_GEN_MATRIX_NBLOCKS.
   */
  /* This should generate the matrix entry with high probability. */
  mlk_xof_squeezeblocks(buf, MLKEM_GEN_MATRIX_NBLOCKS, &state);
  buflen = MLKEM_GEN_MATRIX_NBLOCKS * MLK_XOF_RATE;
  ctr = mlk_rej_uniform(entry->coeffs, MLKEM_N, 0, buf, buflen);

  /* Squeeze + sample one more block a time until we're done */
  buflen = MLK_XOF_RATE;
  while (ctr < MLKEM_N)
  __loop__(
    assigns(ctr, state, memory_slice(entry, sizeof(mlk_poly)), object_whole(buf))
    invariant(ctr <= MLKEM_N)
    invariant(array_bound(entry->coeffs, 0, ctr, 0, MLKEM_Q)))
  {
    mlk_xof_squeezeblocks(buf, 1, &state);
    ctr = mlk_rej_uniform(entry->coeffs, MLKEM_N, ctr, buf, buflen);
  }

  mlk_xof_release(&state);

  /* Specification: Partially implements
   * @[FIPS203, Section 3.3, Destruction of intermediate values] */
  mlk_zeroize(buf, sizeof(buf));
}

/*************************************************
 * Name:        mlk_load32_littleendian
 *
 * Description: load 4 bytes into a 32-bit integer
 *              in little-endian order
 *
 * Arguments:   - const uint8_t *x: pointer to input byte array
 *
 * Returns 32-bit unsigned integer loaded from x
 *
 **************************************************/

/* Reference: `load32_littleendian()` in the reference implementation @[REF]. */
static uint32_t mlk_load32_littleendian(const uint8_t x[4])
{
  uint32_t r;
  r = (uint32_t)x[0];
  r |= (uint32_t)x[1] << 8;
  r |= (uint32_t)x[2] << 16;
  r |= (uint32_t)x[3] << 24;
  return r;
}

/* Reference: `cbd2()` in the reference implementation @[REF]. */
MLK_INTERNAL_API
void mlk_poly_cbd2(mlk_poly *r, const uint8_t buf[2 * MLKEM_N / 4])
{
  unsigned i;
  for (i = 0; i < MLKEM_N / 8; i++)
  __loop__(
    invariant(i <= MLKEM_N / 8)
    invariant(array_abs_bound(r->coeffs, 0, 8 * i, 3)))
  {
    unsigned j;
    uint32_t t = mlk_load32_littleendian(buf + 4 * i);
    uint32_t d = t & 0x55555555;
    d += (t >> 1) & 0x55555555;

    for (j = 0; j < 8; j++)
    __loop__(
      invariant(i <= MLKEM_N / 8 && j <= 8)
      invariant(array_abs_bound(r->coeffs, 0, 8 * i + j, 3)))
    {
      const int16_t a = (d >> (4 * j + 0)) & 0x3;
      const int16_t b = (d >> (4 * j + 2)) & 0x3;
      r->coeffs[8 * i + j] = (int16_t)(a - b);
    }
  }
}

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_ETA1 == 3
/*************************************************
 * Name:        mlk_load24_littleendian
 *
 * Description: load 3 bytes into a 32-bit integer
 *              in little-endian order.
 *              This function is only needed for ML-KEM-512
 *
 * Arguments:   - const uint8_t *x: pointer to input byte array
 *
 * Returns 32-bit unsigned integer loaded from x (most significant byte is zero)
 *
 **************************************************/

/* Reference: `load24_littleendian()` in the reference implementation @[REF]. */
static uint32_t mlk_load24_littleendian(const uint8_t x[3])
{
  uint32_t r;
  r = (uint32_t)x[0];
  r |= (uint32_t)x[1] << 8;
  r |= (uint32_t)x[2] << 16;
  return r;
}

/* Reference: `cbd3()` in the reference implementation @[REF]. */
MLK_INTERNAL_API
void mlk_poly_cbd3(mlk_poly *r, const uint8_t buf[3 * MLKEM_N / 4])
{
  unsigned i;
  for (i = 0; i < MLKEM_N / 4; i++)
  __loop__(
    invariant(i <= MLKEM_N / 4)
    invariant(array_abs_bound(r->coeffs, 0, 4 * i, 4)))
  {
    unsigned j;
    const uint32_t t = mlk_load24_littleendian(buf + 3 * i);
    uint32_t d = t & 0x00249249;
    d += (t >> 1) & 0x00249249;
    d += (t >> 2) & 0x00249249;

    for (j = 0; j < 4; j++)
    __loop__(
      invariant(i <= MLKEM_N / 4 && j <= 4)
      invariant(array_abs_bound(r->coeffs, 0, 4 * i + j, 4)))
    {
      const int16_t a = (d >> (6 * j + 0)) & 0x7;
      const int16_t b = (d >> (6 * j + 3)) & 0x7;
      r->coeffs[4 * i + j] = (int16_t)(a - b);
    }
  }
}
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_ETA1 == 3 */

#else /* !MLK_CONFIG_MULTILEVEL_NO_SHARED */

MLK_EMPTY_CU(sampling)

#endif /* MLK_CONFIG_MULTILEVEL_NO_SHARED */

/* To facilitate single-compilation-unit (SCU) builds, undefine all macros.
 * Don't modify by hand -- this is auto-generated by scripts/autogen. */
#undef MLKEM_GEN_MATRIX_NBLOCKS
