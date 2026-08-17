/*
 * Copyright (c) The mlkem-native project authors
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

#ifndef MLD_CT_H
#define MLD_CT_H

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
   MLD_CONFIG_NO_ASM_VALUE_BARRIER.

*/

#if defined(MLD_HAVE_INLINE_ASM) && !defined(MLD_CONFIG_NO_ASM_VALUE_BARRIER)
#define MLD_USE_ASM_VALUE_BARRIER
#endif


#if !defined(MLD_USE_ASM_VALUE_BARRIER)
/*
 * Declaration of global volatile that the global value barrier
 * is loading from and masking with.
 */
#define mld_ct_opt_blocker_u64 MLD_NAMESPACE(ct_opt_blocker_u64)
extern volatile uint64_t mld_ct_opt_blocker_u64;


/* Helper functions for obtaining global masks of various sizes */

/* This contract is not proved but treated as an axiom.
 *
 * Its validity relies on the assumption that the global opt-blocker
 * constant mld_ct_opt_blocker_u64 is not modified.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint64_t mld_ct_get_optblocker_u64(void)
__contract__(ensures(return_value == 0)) { return mld_ct_opt_blocker_u64; }

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int64_t mld_ct_get_optblocker_i64(void)
__contract__(ensures(return_value == 0)) { return (int64_t)mld_ct_get_optblocker_u64(); }

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint32_t mld_ct_get_optblocker_u32(void)
__contract__(ensures(return_value == 0)) { return (uint32_t)mld_ct_get_optblocker_u64(); }

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint8_t mld_ct_get_optblocker_u8(void)
__contract__(ensures(return_value == 0)) { return (uint8_t)mld_ct_get_optblocker_u64(); }

/* Opt-blocker based implementation of value barriers */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int64_t mld_value_barrier_i64(int64_t b)
__contract__(ensures(return_value == b)) { return (b ^ mld_ct_get_optblocker_i64()); }

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint32_t mld_value_barrier_u32(uint32_t b)
__contract__(ensures(return_value == b)) { return (b ^ mld_ct_get_optblocker_u32()); }

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint8_t mld_value_barrier_u8(uint8_t b)
__contract__(ensures(return_value == b)) { return (b ^ mld_ct_get_optblocker_u8()); }


#else  /* !MLD_USE_ASM_VALUE_BARRIER */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int64_t mld_value_barrier_i64(int64_t b)
__contract__(ensures(return_value == b))
{
  __asm__ volatile("" : "+r"(b));
  return b;
}

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint32_t mld_value_barrier_u32(uint32_t b)
__contract__(ensures(return_value == b))
{
  __asm__ volatile("" : "+r"(b));
  return b;
}

MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint8_t mld_value_barrier_u8(uint8_t b)
__contract__(ensures(return_value == b))
{
  __asm__ volatile("" : "+r"(b));
  return b;
}
#endif /* MLD_USE_ASM_VALUE_BARRIER */

#ifdef CBMC
#pragma CPROVER check push
#pragma CPROVER check disable "conversion"
#endif

/**
 * Cast uint32 value to int32.
 *
 * @param x Input value.
 *
 * @return For uint32_t x, the unique y in int32_t so that x == y mod 2^32.
 *         Concretely:
 *         - x <  2^31: returns x
 *         - x >= 2^31: returns x - 2^31
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_ALWAYS_INLINE int32_t mld_cast_uint32_to_int32(uint32_t x)
{
  /*
   * PORTABILITY: This relies on uint32_t -> int32_t
   * being implemented as the inverse of int32_t -> uint32_t,
   * which is implementation-defined (C99 6.3.1.3 (3))
   * CBMC (correctly) fails to prove this conversion is OK,
   * so we have to suppress that check here
   */
  return (int32_t)x;
}

#ifdef CBMC
#pragma CPROVER check pop
#endif


/**
 * Cast int64 value to uint32 as per C standard.
 *
 * @param x Input value.
 *
 * @return For int64_t x, the unique y in uint32_t so that x == y mod 2^32.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_ALWAYS_INLINE uint32_t mld_cast_int64_to_uint32(int64_t x)
{
  return (uint32_t)(x & (int64_t)UINT32_MAX);
}

/**
 * Cast int32 value to uint32 as per C standard.
 *
 * @param x Input value.
 *
 * @return For int32_t x, the unique y in uint32_t so that x == y mod 2^32.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_ALWAYS_INLINE uint32_t mld_cast_int32_to_uint32(int32_t x)
{
  return mld_cast_int64_to_uint32((int64_t)x);
}

/**
 * Functionally equivalent to cond ? a : b, but implemented with guards against
 * compiler-introduced branches.
 *
 * @param a    First alternative.
 * @param b    Second alternative.
 * @param cond Condition variable.
 *
 * @return a if cond is 0xFFFFFFFF, b if cond is 0.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int32_t mld_ct_sel_int32(int32_t a, int32_t b, uint32_t cond)
__contract__(
  requires(cond == 0x0 || cond == 0xFFFFFFFF)
  ensures(return_value == (cond ? a : b))
)
{
  uint32_t au = mld_cast_int32_to_uint32(a);
  uint32_t bu = mld_cast_int32_to_uint32(b);
  uint32_t res = bu ^ (mld_value_barrier_u32(cond) & (au ^ bu));
  return mld_cast_uint32_to_int32(res);
}

/**
 * Return 0 if input is zero, and -1 otherwise.
 *
 * @param x Value to be converted into a mask.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint32_t mld_ct_cmask_nonzero_u32(uint32_t x)
__contract__(ensures(return_value == ((x == 0) ? 0 : 0xFFFFFFFF)))
{
  int64_t tmp = mld_value_barrier_i64(-((int64_t)x));
  tmp >>= 32;
  return mld_cast_int64_to_uint32(tmp);
}

/**
 * Return 0 if input is zero, and -1 otherwise.
 *
 * @param x Value to be converted into a mask.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint8_t mld_ct_cmask_nonzero_u8(uint8_t x)
__contract__(ensures(return_value == ((x == 0) ? 0 : 0xFF)))
{
  uint32_t mask = mld_ct_cmask_nonzero_u32((uint32_t)x);
  return (uint8_t)(mask & 0xFF);
}

/**
 * Return 0 if input is non-negative, and -1 otherwise.
 *
 * @param x Value to be converted into a mask.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint32_t mld_ct_cmask_neg_i32(int32_t x)
__contract__(
  ensures(return_value == ((x < 0) ? 0xFFFFFFFF : 0))
)
{
  int64_t tmp = mld_value_barrier_i64((int64_t)x);
  tmp >>= 31;
  return mld_cast_int64_to_uint32(tmp);
}

/**
 * Return -x if x<0, x otherwise.
 *
 * @param x Input value.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int32_t mld_ct_abs_i32(int32_t x)
__contract__(
  requires(x >= -INT32_MAX)
  ensures(return_value == ((x < 0) ? -x : x))
)
{
  return mld_ct_sel_int32(-x, x, mld_ct_cmask_neg_i32(x));
}

/**
 * Compare two arrays for equality in constant time.
 *
 * @param[in] a   Pointer to first byte array.
 * @param[in] b   Pointer to second byte array.
 * @param     len Length of the byte arrays, upper-bounded to UINT16_MAX to
 *                control proof complexity only.
 *
 * @return 0 if the byte arrays are equal, 0xFF otherwise.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE uint8_t mld_ct_memcmp(const uint8_t *a, const uint8_t *b,
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
    invariant((r == 0) == (forall(k, 0, i, (a[k] == b[k]))))
    decreases(len - i))
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
  return (mld_value_barrier_u8(mld_ct_cmask_nonzero_u8(r) ^ s) ^ s);
}

/**
 * Force-zeroize a buffer.
 *
 * @[FIPS204, Section 3.6.3] Destruction of intermediate values.
 *
 * @param[out] ptr Pointer to buffer to be zeroed.
 * @param      len Amount of bytes to be zeroed.
 */
#if !defined(MLD_CONFIG_CUSTOM_ZEROIZE)
#if defined(MLD_SYS_WINDOWS)
#include <windows.h>
static MLD_INLINE void mld_zeroize(void *ptr, size_t len)
__contract__(
  requires(memory_no_alias(ptr, len))
  assigns(memory_slice(ptr, len))) { SecureZeroMemory(ptr, len); }
#elif defined(MLD_HAVE_INLINE_ASM)
#include <string.h>
static MLD_INLINE void mld_zeroize(void *ptr, size_t len)
__contract__(
  requires(memory_no_alias(ptr, len))
  assigns(memory_slice(ptr, len)))
{
  memset(ptr, 0, len);
  /* This follows OpenSSL and seems sufficient to prevent the compiler
   * from optimizing away the memset.
   *
   * If there was a reliable way to detect availability of memset_s(),
   * that would be preferred. */
  __asm__ __volatile__("" : : "r"(ptr) : "memory");
}
#else /* !MLD_SYS_WINDOWS && MLD_HAVE_INLINE_ASM */
#error No plausibly-secure implementation of mld_zeroize available. Please provide your own using MLD_CONFIG_CUSTOM_ZEROIZE.
#endif /* !MLD_SYS_WINDOWS && !MLD_HAVE_INLINE_ASM */
#endif /* !MLD_CONFIG_CUSTOM_ZEROIZE */

#endif /* !MLD_CT_H */
