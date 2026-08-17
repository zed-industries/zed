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
 *
 * - [libmceliece]
 *   libmceliece implementation of Classic McEliece
 *   Bernstein, Chou
 *   https://lib.mceliece.org/
 *
 * - [optblocker]
 *   PQC forum post on opt-blockers using volatile globals
 *   Daniel J. Bernstein
 *   https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/hqbtIGFKIpU/m/H14H0wOlBgAJ
 */

#ifndef MLK_VERIFY_H
#define MLK_VERIFY_H


#include "cbmc.h"
#include "common.h"

/* Constant-time comparisons and conditional operations

   We reduce the risk for compilation into variable-time code
   through the use of 'value barriers'.

   Functionally, a value barrier is a no-op. To the compiler, however,
   it constitutes an arbitrary modification of its input, and therefore
   harden's value propagation and range analysis.

   We consider two approaches to implement a value barrier:
   - An empty inline asm block which marks the target value as clobbered.
   - XOR'ing with the value of a volatile global that's set to 0;
     see @[optblocker] for a discussion of this idea, and
     @[libmceliece, inttypes/crypto_intN.h] for an implementation.

   The first approach is cheap because it only prevents the compiler
   from reasoning about the value of the variable past the barrier,
   but does not directly generate additional instructions.

   The second approach generates redundant loads and XOR operations
   and therefore comes at a higher runtime cost. However, it appears
   more robust towards optimization, as compilers should never drop
   a volatile load.

   We use the empty-ASM value barrier for GCC and clang, and fall
   back to the global volatile barrier otherwise.

   The global value barrier can be forced by setting
   MLK_CONFIG_NO_ASM_VALUE_BARRIER.

*/

#if defined(MLK_HAVE_INLINE_ASM) && !defined(MLK_CONFIG_NO_ASM_VALUE_BARRIER)
#define MLK_USE_ASM_VALUE_BARRIER
#endif

#if !defined(MLK_USE_ASM_VALUE_BARRIER)

/*
 * Declaration of global volatile that the global value barrier
 * is loading from and masking with.
 */
#define mlk_ct_opt_blocker_u64 MLK_NAMESPACE(ct_opt_blocker_u64)
extern volatile uint64_t mlk_ct_opt_blocker_u64;

/* Helper functions for obtaining global masks of various sizes */

/* This contract is not proved but treated as an axiom.
 *
 * Its validity relies on the assumption that the global opt-blocker
 * constant mlk_ct_opt_blocker_u64 is not modified.
 */
static MLK_INLINE uint64_t mlk_ct_get_optblocker_u64(void)
__contract__(ensures(return_value == 0)) { return mlk_ct_opt_blocker_u64; }

static MLK_INLINE uint8_t mlk_ct_get_optblocker_u8(void)
__contract__(ensures(return_value == 0)) { return (uint8_t)mlk_ct_get_optblocker_u64(); }

static MLK_INLINE uint32_t mlk_ct_get_optblocker_u32(void)
__contract__(ensures(return_value == 0)) { return (uint32_t)mlk_ct_get_optblocker_u64(); }

static MLK_INLINE int32_t mlk_ct_get_optblocker_i32(void)
__contract__(ensures(return_value == 0)) { return (int32_t)mlk_ct_get_optblocker_u64(); }

/* Opt-blocker based implementation of value barriers */
static MLK_INLINE uint32_t mlk_value_barrier_u32(uint32_t b)
__contract__(ensures(return_value == b)) { return (b ^ mlk_ct_get_optblocker_u32()); }

static MLK_INLINE int32_t mlk_value_barrier_i32(int32_t b)
__contract__(ensures(return_value == b)) { return (b ^ mlk_ct_get_optblocker_i32()); }

static MLK_INLINE uint8_t mlk_value_barrier_u8(uint8_t b)
__contract__(ensures(return_value == b)) { return (b ^ mlk_ct_get_optblocker_u8()); }

#else /* !MLK_USE_ASM_VALUE_BARRIER */

static MLK_INLINE uint32_t mlk_value_barrier_u32(uint32_t b)
__contract__(ensures(return_value == b))
{
  __asm__ volatile("" : "+r"(b));
  return b;
}

static MLK_INLINE int32_t mlk_value_barrier_i32(int32_t b)
__contract__(ensures(return_value == b))
{
  __asm__ volatile("" : "+r"(b));
  return b;
}

static MLK_INLINE uint8_t mlk_value_barrier_u8(uint8_t b)
__contract__(ensures(return_value == b))
{
  __asm__ volatile("" : "+r"(b));
  return b;
}

#endif /* MLK_USE_ASM_VALUE_BARRIER */

#ifdef CBMC
#pragma CPROVER check push
#pragma CPROVER check disable "conversion"
#endif
/*************************************************
 * Name:        mlk_cast_uint16_to_int16
 *
 * Description: Cast uint16 value to int16
 *
 * Returns:     For uint16_t x, the unique y in int16_t
 *              so that x == y mod 2^16.
 *
 *              Concretely:
 *              - x <  32768: returns x
 *              - x >= 32768: returns x - 65536
 *
 **************************************************/
static MLK_ALWAYS_INLINE int16_t mlk_cast_uint16_to_int16(uint16_t x)
{
  /*
   * PORTABILITY: This relies on uint16_t -> int16_t
   * being implemented as the inverse of int16_t -> uint16_t,
   * which is implementation-defined (C99 6.3.1.3 (3))
   * CBMC (correctly) fails to prove this conversion is OK,
   * so we have to suppress that check here
   */
  return (int16_t)x;
}
#ifdef CBMC
#pragma CPROVER check pop
#endif

/*************************************************
 * Name:        mlk_cast_int32_to_uint16
 *
 * Description: Cast int32 value to uint16 as per C standard.
 *
 * Returns:     For int32_t x, the unique y in uint16_t
 *              so that x == y mod 2^16.
 **************************************************/
static MLK_ALWAYS_INLINE uint16_t mlk_cast_int32_to_uint16(int32_t x)
{
  return (uint16_t)(x & (int32_t)UINT16_MAX);
}

/*************************************************
 * Name:        mlk_cast_int16_to_uint16
 *
 * Description: Cast int16 value to uint16 as per C standard.
 *
 * Returns:     For int16_t x, the unique y in uint16_t
 *              so that x == y mod 2^16.
 **************************************************/
static MLK_ALWAYS_INLINE uint16_t mlk_cast_int16_to_uint16(int32_t x)
{
  return mlk_cast_int32_to_uint16((int32_t)x);
}

/*************************************************
 * Name:        mlk_ct_cmask_neg_i16
 *
 * Description: Return 0 if input is non-negative, and -1 otherwise.
 *
 * Arguments:   uint16_t x: Value to be converted into a mask
 *
 **************************************************/

/* Reference: Embedded in polynomial compression function in the
 *            reference implementation @[REF].
 *            - Used as part of signed->unsigned conversion for modular
 *              representatives to detect whether the input is negative.
 *              This happen in `mlk_poly_reduce()` here, and as part of
 *              polynomial compression functions in the reference
 *              implementation. See `mlk_poly_reduce()`.
 *            - We use value barriers to reduce the risk of
 *              compiler-introduced branches. */
static MLK_INLINE uint16_t mlk_ct_cmask_neg_i16(int16_t x)
__contract__(ensures(return_value == ((x < 0) ? 0xFFFF : 0)))
{
  int32_t tmp = mlk_value_barrier_i32((int32_t)x);
  tmp >>= 16;
  return mlk_cast_int32_to_uint16(tmp);
}

/*************************************************
 * Name:        mlk_ct_cmask_nonzero_u16
 *
 * Description: Return 0 if input is zero, and -1 otherwise.
 *
 * Arguments:   uint16_t x: Value to be converted into a mask
 *
 **************************************************/

/* Reference: Embedded in `cmov_int16()` in the reference implementation @[REF].
 *            - Use value barrier and shift instead of `b = -b` to
 *              convert condition into mask. */
static MLK_INLINE uint16_t mlk_ct_cmask_nonzero_u16(uint16_t x)
__contract__(ensures(return_value == ((x == 0) ? 0 : 0xFFFF)))
{
  int32_t tmp = mlk_value_barrier_i32(-((int32_t)x));
  tmp >>= 16;
  return mlk_cast_int32_to_uint16(tmp);
}

/*************************************************
 * Name:        mlk_ct_cmask_nonzero_u8
 *
 * Description: Return 0 if input is zero, and -1 otherwise.
 *
 * Arguments:   uint8_t x: Value to be converted into a mask
 *
 **************************************************/

/* Reference: Embedded in `verify()` and `cmov()` in the
 *            reference implementation @[REF].
 *            - We include a value barrier not present in the
 *              reference implementation, to prevent the compiler
 *              from realizing that this function returns a mask. */
static MLK_INLINE uint8_t mlk_ct_cmask_nonzero_u8(uint8_t x)
__contract__(ensures(return_value == ((x == 0) ? 0 : 0xFF)))
{
  uint16_t mask = mlk_ct_cmask_nonzero_u16((uint16_t)x);
  return (uint8_t)(mask & 0xFF);
}

/*************************************************
 * Name:        mlk_ct_sel_int16
 *
 * Description: Functionally equivalent to cond ? a : b,
 *              but implemented with guards against
 *              compiler-introduced branches.
 *
 * Arguments:   int16_t a:       First alternative
 *              int16_t b:       Second alternative
 *              uint16_t cond:   Condition variable.
 *
 * Specification:
 * - With `a = MLKEM_Q_HALF` and `b=0`, this essentially
 *   implements `Decompress_1` @[FIPS203, Eq (4.8)] in `mlk_poly_frommsg()`.
 * - With `a = x + MLKEM_Q`, `b = x`, and `cond` indicating whether `x`
 *   is negative, implements signed->unsigned conversion of modular
 *   representatives. Questions of representation are not considered
 *   in the specification @[FIPS203, Section 2.4.1, "The pseudocode is
 *   agnostic regarding how an integer modulo 𝑚 is represented in
 *   actual implementations"].
 *
 **************************************************/

/* Reference: Embedded in polynomial compression function in the
 *            reference implementation @[REF].
 *            - Used as part of signed->unsigned conversion for modular
 *              representatives. This happen in `mlk_poly_reduce()` here,
 *              and as part of polynomial compression functions in @[REF].
 *              See `mlk_poly_reduce()`.
 *            - Barrier to reduce the risk of compiler-introduced branches.
 *            For `a = MLKEM_Q_HALF` and `b=0`, also embedded in
 *            `poly_frommsg()` from the reference implementation, which uses
 *            `cmov_int16()` instead. */
static MLK_INLINE int16_t mlk_ct_sel_int16(int16_t a, int16_t b, uint16_t cond)
__contract__(ensures(return_value == (cond ? a : b)))
{
  uint16_t au = mlk_cast_int16_to_uint16(a);
  uint16_t bu = mlk_cast_int16_to_uint16(b);
  uint16_t res = bu ^ (mlk_ct_cmask_nonzero_u16(cond) & (au ^ bu));
  return mlk_cast_uint16_to_int16(res);
}

/*************************************************
 * Name:        mlk_ct_sel_uint8
 *
 * Description: Functionally equivalent to cond ? a : b,
 *              but implemented with guards against
 *              compiler-introduced branches.
 *
 * Arguments:   uint8_t a:       First alternative
 *              uint8_t b:       Second alternative
 *              uuint8_t cond:   Condition variable.
 *
 **************************************************/

/* Reference: Embedded into `cmov()` in the reference implementation @[REF].
 *            - Use value barrier to get mask from condition value. */
static MLK_INLINE uint8_t mlk_ct_sel_uint8(uint8_t a, uint8_t b, uint8_t cond)
__contract__(ensures(return_value == (cond ? a : b)))
{
  return b ^ (mlk_ct_cmask_nonzero_u8(cond) & (a ^ b));
}

/*************************************************
 * Name:        mlk_ct_memcmp
 *
 * Description: Compare two arrays for equality in constant time.
 *
 * Arguments:   const uint8_t *a: pointer to first byte array
 *              const uint8_t *b: pointer to second byte array
 *              size_t len:       length of the byte arrays, upper-bounded
 *                                to UINT16_MAX to control proof complexity
 *                                only.
 *
 * Returns 0 if the byte arrays are equal, 0xFF otherwise.
 *
 * Specification:
 * - Used to securely compute conditional move in
 *   @[FIPS203, Algorithm 18 (ML-KEM.Decaps_Internal, L9-11]
 *
 **************************************************/

/* Reference: `cmov()` in the reference implementation @[REF]
 *            - We return `uint8_t`, not `int`.
 *            - We use an additional XOR-accumulator in the comparison loop
 *              which prevents early abort if the OR-accumulator is 0xFF.
 *            - We use a value barrier to convert the OR-accumulator into
 *              a mask. The reference implementation uses a shift which the
 *              compiler can argue to result in either 0 of 0xFF..FF. */
static MLK_INLINE uint8_t mlk_ct_memcmp(const uint8_t *a, const uint8_t *b,
                                        const size_t len)
__contract__(
  requires(len <= UINT16_MAX)
  requires(memory_no_alias(a, len))
  requires(memory_no_alias(b, len))
  ensures((return_value == 0) || (return_value == 0xFF))
  ensures((return_value == 0) == forall(i, 0, len, (a[i] == b[i]))))
{
  uint8_t r = 0, s = 0;
  unsigned i;

  for (i = 0; i < len; i++)
  __loop__(
    invariant(i <= len)
    invariant((r == 0) == (forall(k, 0, i, (a[k] == b[k])))))
  {
    r |= a[i] ^ b[i];
    /* s is useless, but prevents the loop from being aborted once r=0xff. */
    s ^= a[i] ^ b[i];
  }

  /*
   * - Convert r into a mask; this may not be necessary, but is an additional
   *   safeguard
   *   towards leaking information about a and b.
   * - XOR twice with s, separated by a value barrier, to prevent the compile
   *   from dropping the s computation in the loop.
   */
  return (mlk_value_barrier_u8(mlk_ct_cmask_nonzero_u8(r) ^ s) ^ s);
}

/*************************************************
 * Name:        mlk_ct_cmov_zero
 *
 * Description: Copy len bytes from x to r if b is zero;
 *              don't modify x if b is non-zero.
 *              assumes two's complement representation of negative integers.
 *              Runs in constant time.
 *
 * Arguments:   uint8_t *r:       pointer to output byte array
 *              const uint8_t *x: pointer to input byte array
 *              size_t len:       Amount of bytes to be copied
 *              uint8_t b:        Condition value.
 *
 * Specification:
 * - Used to securely compute conditional move in
 *   @[FIPS203, Algorithm 18 (ML-KEM.Decaps_Internal, L9-11]
 *
 **************************************************/

/* Reference: `cmov()` in the reference implementation @[REF].
 *            - We move if condition value is `0`, not `1`.
 *            - We use `mlk_ct_sel_uint8` for constant-time selection. */
static MLK_INLINE void mlk_ct_cmov_zero(uint8_t *r, const uint8_t *x,
                                        size_t len, uint8_t b)
__contract__(
  requires(len <= MLK_MAX_BUFFER_SIZE)
  requires(memory_no_alias(r, len))
  requires(memory_no_alias(x, len))
  assigns(memory_slice(r, len))
  ensures(forall(i, 0, len, (r[i] == (b == 0 ? x[i] : old(r)[i])))))
{
  size_t i;
  for (i = 0; i < len; i++)
  __loop__(
    invariant(i <= len)
    invariant(forall(k, 0, i, r[k] == (b == 0 ? x[k] : loop_entry(r)[k]))))
  {
    r[i] = mlk_ct_sel_uint8(r[i], x[i], b);
  }
}

/*************************************************
 * Name:        mlk_zeroize
 *
 * Description: Force-zeroize a buffer.
 *
 * Arguments:   uint8_t *r:       pointer to byte array to be zeroed
 *              size_t len:       Amount of bytes to be zeroed
 *
 * Specification: Used to implement
 * @[FIPS203, Section 3.3, Destruction of intermediate values]
 *
 **************************************************/

/* Reference: Not present in the reference implementation @[REF]. */
#if !defined(MLK_CONFIG_CUSTOM_ZEROIZE)
#if defined(MLK_SYS_WINDOWS)
#include <windows.h>
static MLK_INLINE void mlk_zeroize(void *ptr, size_t len)
__contract__(
  requires(memory_no_alias(ptr, len))
  assigns(memory_slice(ptr, len))) { SecureZeroMemory(ptr, len); }
#elif defined(MLK_HAVE_INLINE_ASM)
#include <string.h>
static MLK_INLINE void mlk_zeroize(void *ptr, size_t len)
__contract__(
  requires(memory_no_alias(ptr, len))
  assigns(memory_slice(ptr, len)))
{
  mlk_memset(ptr, 0, len);
  /* This follows OpenSSL and seems sufficient to prevent the compiler
   * from optimizing away the memset.
   *
   * If there was a reliable way to detect availability of memset_s(),
   * that would be preferred. */
  __asm__ volatile("" : : "r"(ptr) : "memory");
}
#else /* !MLK_SYS_WINDOWS && MLK_HAVE_INLINE_ASM */
#error No plausibly-secure implementation of mlk_zeroize available. Please provide your own using MLK_CONFIG_CUSTOM_ZEROIZE.
#endif /* !MLK_SYS_WINDOWS && !MLK_HAVE_INLINE_ASM */
#endif /* !MLK_CONFIG_CUSTOM_ZEROIZE */

#endif /* !MLK_VERIFY_H */
