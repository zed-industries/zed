// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2002, Oracle and/or its affiliates. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_BN_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_BN_INTERNAL_H

#include <openssl/bn.h>

#if defined(OPENSSL_X86_64) && defined(_MSC_VER)
#include <intrin.h>
#pragma intrinsic(__umulh, _umul128)
#endif

#include "../../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif

#if defined(OPENSSL_64_BIT)

#if defined(BORINGSSL_HAS_UINT128)
// MSVC doesn't support two-word integers on 64-bit.
#define BN_ULLONG uint128_t
#if defined(BORINGSSL_CAN_DIVIDE_UINT128)
#define BN_CAN_DIVIDE_ULLONG
#endif
#endif

#define BN_BITS2 64
#define BN_BITS2_LG 6
#define BN_BYTES 8
#define BN_BITS4 32
#define BN_MASK2 (0xffffffffffffffffUL)
#define BN_MASK2l (0xffffffffUL)
#define BN_MASK2h (0xffffffff00000000UL)
#define BN_MASK2h1 (0xffffffff80000000UL)
#define BN_MONT_CTX_N0_LIMBS 1
#define BN_DEC_CONV (10000000000000000000UL)
#define BN_DEC_NUM 19
#define TOBN(hi, lo) ((BN_ULONG)(hi) << 32 | (lo))

#elif defined(OPENSSL_32_BIT)

#define BN_ULLONG uint64_t
#define BN_CAN_DIVIDE_ULLONG
#define BN_BITS2 32
#define BN_BITS2_LG 5
#define BN_BYTES 4
#define BN_BITS4 16
#define BN_MASK2 (0xffffffffUL)
#define BN_MASK2l (0xffffUL)
#define BN_MASK2h1 (0xffff8000UL)
#define BN_MASK2h (0xffff0000UL)
// On some 32-bit platforms, Montgomery multiplication is done using 64-bit
// arithmetic with SIMD instructions. On such platforms, |BN_MONT_CTX::n0|
// needs to be two words long. Only certain 32-bit platforms actually make use
// of n0[1] and shorter R value would suffice for the others. However,
// currently only the assembly files know which is which.
#define BN_MONT_CTX_N0_LIMBS 2
#define BN_DEC_CONV (1000000000UL)
#define BN_DEC_NUM 9
#define TOBN(hi, lo) (lo), (hi)

#else
#error "Must define either OPENSSL_32_BIT or OPENSSL_64_BIT"
#endif

#if !defined(OPENSSL_NO_ASM) && (defined(__GNUC__) || defined(__clang__))
#define BN_CAN_USE_INLINE_ASM
#endif

// MOD_EXP_CTIME_ALIGN is the alignment needed for |BN_mod_exp_mont_consttime|'s
// tables.
//
// TODO(davidben): Historically, this alignment came from cache line
// assumptions, which we've since removed. Is 64-byte alignment still necessary
// or ideal? The true alignment requirement seems to now be 32 bytes, coming
// from RSAZ's use of VMOVDQA to a YMM register. Non-x86_64 has even fewer
// requirements.
#define MOD_EXP_CTIME_ALIGN 64

// MOD_EXP_CTIME_STORAGE_LEN is the number of |BN_ULONG|s needed for the
// |BN_mod_exp_mont_consttime| stack-allocated storage buffer. The buffer is
// just the right size for the RSAZ and is about ~1KB larger than what's
// necessary (4480 bytes) for 1024-bit inputs.
#define MOD_EXP_CTIME_STORAGE_LEN \
  (((320u * 3u) + (32u * 9u * 16u)) / sizeof(BN_ULONG))

#define STATIC_BIGNUM(x)                                    \
  {                                                         \
    (BN_ULONG *)(x), sizeof(x) / sizeof(BN_ULONG),          \
        sizeof(x) / sizeof(BN_ULONG), 0, BN_FLG_STATIC_DATA \
  }

#if defined(BN_ULLONG)
#define Lw(t) ((BN_ULONG)(t))
#define Hw(t) ((BN_ULONG)((t) >> BN_BITS2))
#endif

// bn_minimal_width returns the minimal number of words needed to represent
// |bn|.
int bn_minimal_width(const BIGNUM *bn);

// bn_set_minimal_width sets |bn->width| to |bn_minimal_width(bn)|. If |bn| is
// zero, |bn->neg| is set to zero.
void bn_set_minimal_width(BIGNUM *bn);

// bn_wexpand ensures that |bn| has at least |words| works of space without
// altering its value. It returns one on success or zero on allocation
// failure.
int bn_wexpand(BIGNUM *bn, size_t words);

// bn_expand acts the same as |bn_wexpand|, but takes a number of bits rather
// than a number of words.
int bn_expand(BIGNUM *bn, size_t bits);

// bn_resize_words adjusts |bn->width| to be |words|. It returns one on success
// and zero on allocation error or if |bn|'s value is too large.
OPENSSL_EXPORT int bn_resize_words(BIGNUM *bn, size_t words);

// bn_select_words sets |r| to |a| if |mask| is all ones or |b| if |mask| is
// all zeros.
void bn_select_words(BN_ULONG *r, BN_ULONG mask, const BN_ULONG *a,
                     const BN_ULONG *b, size_t num);

// bn_set_words sets |bn| to the value encoded in the |num| words in |words|,
// least significant word first.
int bn_set_words(BIGNUM *bn, const BN_ULONG *words, size_t num);

// bn_set_static_words acts like |bn_set_words|, but doesn't copy the data. A
// flag is set on |bn| so that |BN_free| won't attempt to free the data.
//
// The |STATIC_BIGNUM| macro is probably a better solution for this outside of
// the FIPS module. Inside of the FIPS module that macro generates rel.ro data,
// which doesn't work with FIPS requirements.
void bn_set_static_words(BIGNUM *bn, const BN_ULONG *words, size_t num);

// bn_fits_in_words returns one if |bn| may be represented in |num| words, plus
// a sign bit, and zero otherwise.
int bn_fits_in_words(const BIGNUM *bn, size_t num);

// bn_copy_words copies the value of |bn| to |out| and returns one if the value
// is representable in |num| words. Otherwise, it returns zero.
int bn_copy_words(BN_ULONG *out, size_t num, const BIGNUM *bn);

// bn_assert_fits_in_bytes asserts that |bn| fits in |num| bytes. This is a
// no-op in release builds, but triggers an assert in debug builds, and
// declassifies all bytes which are therefore known to be zero in constant-time
// validation.
void bn_assert_fits_in_bytes(const BIGNUM *bn, size_t num);

// bn_secret marks |bn|'s contents, but not its width or sign, as secret. See
// |CONSTTIME_SECRET| for details.
inline void bn_secret(BIGNUM *bn) {
  CONSTTIME_SECRET(bn->d, bn->width * sizeof(BN_ULONG));
}

// bn_declassify marks |bn|'s value as public. See |CONSTTIME_DECLASSIFY| for
// details.
inline void bn_declassify(BIGNUM *bn) {
  CONSTTIME_DECLASSIFY(bn->d, bn->width * sizeof(BN_ULONG));
}

// bn_mul_add_words multiples |ap| by |w|, adds the result to |rp|, and places
// the result in |rp|. |ap| and |rp| must both be |num| words long. It returns
// the carry word of the operation. |ap| and |rp| may be equal but otherwise may
// not alias.
BN_ULONG bn_mul_add_words(BN_ULONG *rp, const BN_ULONG *ap, size_t num,
                          BN_ULONG w);

// bn_mul_words multiples |ap| by |w| and places the result in |rp|. |ap| and
// |rp| must both be |num| words long. It returns the carry word of the
// operation. |ap| and |rp| may be equal but otherwise may not alias.
BN_ULONG bn_mul_words(BN_ULONG *rp, const BN_ULONG *ap, size_t num, BN_ULONG w);

// bn_sqr_words sets |rp[2*i]| and |rp[2*i+1]| to |ap[i]|'s square, for all |i|
// up to |num|. |ap| is an array of |num| words and |rp| an array of |2*num|
// words. |ap| and |rp| may not alias.
//
// This gives the contribution of the |ap[i]*ap[i]| terms when squaring |ap|.
void bn_sqr_words(BN_ULONG *rp, const BN_ULONG *ap, size_t num);

// bn_add_words adds |ap| to |bp| and places the result in |rp|, each of which
// are |num| words long. It returns the carry bit, which is one if the operation
// overflowed and zero otherwise. Any pair of |ap|, |bp|, and |rp| may be equal
// to each other but otherwise may not alias.
BN_ULONG bn_add_words(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                      size_t num);

// bn_sub_words subtracts |bp| from |ap| and places the result in |rp|. It
// returns the borrow bit, which is one if the computation underflowed and zero
// otherwise. Any pair of |ap|, |bp|, and |rp| may be equal to each other but
// otherwise may not alias.
BN_ULONG bn_sub_words(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                      size_t num);

// bn_mul_comba4 sets |r| to the product of |a| and |b|.
void bn_mul_comba4(BN_ULONG r[8], const BN_ULONG a[4], const BN_ULONG b[4]);

// bn_mul_comba8 sets |r| to the product of |a| and |b|.
void bn_mul_comba8(BN_ULONG r[16], const BN_ULONG a[8], const BN_ULONG b[8]);

// bn_sqr_comba8 sets |r| to |a|^2.
void bn_sqr_comba8(BN_ULONG r[16], const BN_ULONG a[8]);

// bn_sqr_comba4 sets |r| to |a|^2.
void bn_sqr_comba4(BN_ULONG r[8], const BN_ULONG a[4]);

// bn_less_than_words returns one if |a| < |b| and zero otherwise, where |a|
// and |b| both are |len| words long. It runs in constant time.
int bn_less_than_words(const BN_ULONG *a, const BN_ULONG *b, size_t len);

// bn_in_range_words returns one if |min_inclusive| <= |a| < |max_exclusive|,
// where |a| and |max_exclusive| both are |len| words long. |a| and
// |max_exclusive| are treated as secret.
int bn_in_range_words(const BN_ULONG *a, BN_ULONG min_inclusive,
                      const BN_ULONG *max_exclusive, size_t len);

// bn_rand_range_words sets |out| to a uniformly distributed random number from
// |min_inclusive| to |max_exclusive|. Both |out| and |max_exclusive| are |len|
// words long.
//
// This function runs in time independent of the result, but |min_inclusive| and
// |max_exclusive| are public data. (Information about the range is unavoidably
// leaked by how many iterations it took to select a number.)
int bn_rand_range_words(BN_ULONG *out, BN_ULONG min_inclusive,
                        const BN_ULONG *max_exclusive, size_t len,
                        const uint8_t additional_data[32]);

// bn_range_secret_range behaves like |BN_rand_range_ex|, but treats
// |max_exclusive| as secret. Because of this constraint, the distribution of
// values returned is more complex.
//
// Rather than repeatedly generating values until one is in range, which would
// leak information, it generates one value. If the value is in range, it sets
// |*out_is_uniform| to one. Otherwise, it sets |*out_is_uniform| to zero,
// fixing up the value to force it in range.
//
// The subset of calls to |bn_rand_secret_range| which set |*out_is_uniform| to
// one are uniformly distributed in the target range. Calls overall are not.
// This function is intended for use in situations where the extra values are
// still usable and where the number of iterations needed to reach the target
// number of uniform outputs may be blinded for negligible probabilities of
// timing leaks.
//
// Although this function treats |max_exclusive| as secret, it treats the number
// of bits in |max_exclusive| as public.
int bn_rand_secret_range(BIGNUM *r, int *out_is_uniform, BN_ULONG min_inclusive,
                         const BIGNUM *max_exclusive);

// BN_MONTGOMERY_MAX_WORDS is the maximum number of words allowed in a |BIGNUM|
// used with Montgomery reduction. Ideally this limit would be applied to all
// |BIGNUM|s, in |bn_wexpand|, but the exactfloat library needs to create 8 MiB
// values for other operations.
//
// TODO(crbug.com/402677800): This is not quite tight enough to limit the
// |bn_mul_mont| allocation to under a page. Lower the maximum RSA key and then
// lower this to match.
#define BN_MONTGOMERY_MAX_WORDS (16384 / BN_BITS2)

#if !defined(OPENSSL_NO_ASM) &&                         \
    (defined(OPENSSL_X86) || defined(OPENSSL_X86_64) || \
     defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64))
#define OPENSSL_BN_ASM_MONT
// bn_mul_mont writes |ap| * |bp| mod |np| to |rp|, each |num| words
// long. Inputs and outputs are in Montgomery form. |n0| is a pointer to the
// corresponding field in |BN_MONT_CTX|.
//
// If at least one of |ap| or |bp| is fully reduced, |rp| will be fully reduced.
// If neither is fully-reduced, the output may not be either.
//
// This function allocates up to 2 * |num| words (plus a constant allocation) on
// the stack, so |num| should be at most |BN_MONTGOMERY_MAX_WORDS|.
// Additionally, |num| must be at least 128 / |BN_BITS2|.
//
// TODO(davidben): The x86_64 implementation expects a 32-bit input and masks
// off upper bits. The aarch64 implementation expects a 64-bit input and does
// not. |size_t| is the safer option but not strictly correct for x86_64. But
// the |BN_MONTGOMERY_MAX_WORDS| bound makes this moot.
//
// See also discussion in |ToWord| in abi_test.h for notes on smaller-than-word
// inputs.
void bn_mul_mont(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                 const BN_ULONG *np, const BN_ULONG *n0, size_t num);

#if defined(OPENSSL_X86_64)
inline int bn_mulx_adx_capable(void) {
  // MULX is in BMI2.
  return CRYPTO_is_BMI2_capable() && CRYPTO_is_ADX_capable();
}
void bn_mul_mont_nohw(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                      const BN_ULONG *np, const BN_ULONG *n0, size_t num);
inline int bn_mul4x_mont_capable(size_t num) {
  return num >= 8 && (num & 3) == 0;
}
void bn_mul4x_mont(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                   const BN_ULONG *np, const BN_ULONG *n0, size_t num);
inline int bn_mulx4x_mont_capable(size_t num) {
  return bn_mul4x_mont_capable(num) && bn_mulx_adx_capable();
}
void bn_mulx4x_mont(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                    const BN_ULONG *np, const BN_ULONG *n0, size_t num);
inline int bn_sqr8x_mont_capable(size_t num) {
  return num >= 8 && (num & 7) == 0;
}
void bn_sqr8x_mont(BN_ULONG *rp, const BN_ULONG *ap, BN_ULONG mulx_adx_capable,
                   const BN_ULONG *np, const BN_ULONG *n0, size_t num);
#elif defined(OPENSSL_ARM)
inline int bn_mul8x_mont_neon_capable(size_t num) {
  return (num & 7) == 0 && CRYPTO_is_NEON_capable();
}
void bn_mul8x_mont_neon(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                        const BN_ULONG *np, const BN_ULONG *n0, size_t num);
void bn_mul_mont_nohw(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                      const BN_ULONG *np, const BN_ULONG *n0, size_t num);
#endif

#endif  // OPENSSL_BN_ASM_MONT

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64)
#define OPENSSL_BN_ASM_MONT5

// The following functions implement |bn_mul_mont_gather5|. See
// |bn_mul_mont_gather5| for details.
inline int bn_mul4x_mont_gather5_capable(int num) { return (num & 7) == 0; }
void bn_mul4x_mont_gather5(BN_ULONG *rp, const BN_ULONG *ap,
                           const BN_ULONG *table, const BN_ULONG *np,
                           const BN_ULONG *n0, int num, int power);

inline int bn_mulx4x_mont_gather5_capable(int num) {
  return bn_mul4x_mont_gather5_capable(num) && CRYPTO_is_ADX_capable() &&
         CRYPTO_is_BMI1_capable() && CRYPTO_is_BMI2_capable();
}
void bn_mulx4x_mont_gather5(BN_ULONG *rp, const BN_ULONG *ap,
                            const BN_ULONG *table, const BN_ULONG *np,
                            const BN_ULONG *n0, int num, int power);

void bn_mul_mont_gather5_nohw(BN_ULONG *rp, const BN_ULONG *ap,
                              const BN_ULONG *table, const BN_ULONG *np,
                              const BN_ULONG *n0, int num, int power);

// bn_scatter5 stores |inp| to index |power| of |table|. |inp| and each entry of
// |table| are |num| words long. |power| must be less than 32 and is treated as
// public. |table| must be 32*|num| words long. |table| must be aligned to at
// least 16 bytes.
void bn_scatter5(const BN_ULONG *inp, size_t num, BN_ULONG *table,
                 size_t power);

// bn_gather5 loads index |power| of |table| and stores it in |out|. |out| and
// each entry of |table| are |num| words long. |power| must be less than 32 and
// is treated as secret. |table| must be aligned to at least 16 bytes.
void bn_gather5(BN_ULONG *out, size_t num, const BN_ULONG *table, size_t power);

// The following functions implement |bn_power5|. See |bn_power5| for details.
void bn_power5_nohw(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *table,
                    const BN_ULONG *np, const BN_ULONG *n0, int num, int power);

inline int bn_power5_capable(int num) { return (num & 7) == 0; }

inline int bn_powerx5_capable(int num) {
  return bn_power5_capable(num) && CRYPTO_is_ADX_capable() &&
         CRYPTO_is_BMI1_capable() && CRYPTO_is_BMI2_capable();
}
void bn_powerx5(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *table,
                const BN_ULONG *np, const BN_ULONG *n0, int num, int power);

#endif  // !OPENSSL_NO_ASM && OPENSSL_X86_64

uint64_t bn_mont_n0(const BIGNUM *n);

// bn_mont_ctx_set_RR_consttime initializes |mont->RR|. It returns one on
// success and zero on error. |mont->N| and |mont->n0| must have been
// initialized already. The bit width of |mont->N| is assumed public, but
// |mont->N| is otherwise treated as secret.
int bn_mont_ctx_set_RR_consttime(BN_MONT_CTX *mont, BN_CTX *ctx);

#if defined(_MSC_VER)
#if defined(OPENSSL_X86_64)
#define BN_UMULT_LOHI(low, high, a, b) ((low) = _umul128((a), (b), &(high)))
#elif defined(OPENSSL_AARCH64)
#define BN_UMULT_LOHI(low, high, a, b) \
  do {                                 \
    const BN_ULONG _a = (a);           \
    const BN_ULONG _b = (b);           \
    (low) = _a * _b;                   \
    (high) = __umulh(_a, _b);          \
  } while (0)
#endif
#endif  // _MSC_VER

#if !defined(BN_ULLONG) && !defined(BN_UMULT_LOHI)
#error "Either BN_ULLONG or BN_UMULT_LOHI must be defined on every platform."
#endif

// bn_jacobi returns the Jacobi symbol of |a| and |b| (which is -1, 0 or 1), or
// -2 on error.
int bn_jacobi(const BIGNUM *a, const BIGNUM *b, BN_CTX *ctx);

// bn_is_bit_set_words returns one if bit |bit| is set in |a| and zero
// otherwise.
int bn_is_bit_set_words(const BN_ULONG *a, size_t num, size_t bit);

// bn_one_to_montgomery sets |r| to one in Montgomery form. It returns one on
// success and zero on error. This function treats the bit width of the modulus
// as public.
int bn_one_to_montgomery(BIGNUM *r, const BN_MONT_CTX *mont, BN_CTX *ctx);

// bn_less_than_montgomery_R returns one if |bn| is less than the Montgomery R
// value for |mont| and zero otherwise.
int bn_less_than_montgomery_R(const BIGNUM *bn, const BN_MONT_CTX *mont);

// bn_mod_u16_consttime returns |bn| mod |d|, ignoring |bn|'s sign bit. It runs
// in time independent of the value of |bn|, but it treats |d| as public.
OPENSSL_EXPORT uint16_t bn_mod_u16_consttime(const BIGNUM *bn, uint16_t d);

// bn_odd_number_is_obviously_composite returns one if |bn| is divisible by one
// of the first several odd primes and zero otherwise.
int bn_odd_number_is_obviously_composite(const BIGNUM *bn);

// A BN_MILLER_RABIN stores state common to each Miller-Rabin iteration. It is
// initialized within an existing |BN_CTX| scope and may not be used after
// that scope is released with |BN_CTX_end|. Field names match those in FIPS
// 186-4, section C.3.1.
typedef struct {
  // w1 is w-1.
  BIGNUM *w1;
  // m is (w-1)/2^a.
  BIGNUM *m;
  // one_mont is 1 (mod w) in Montgomery form.
  BIGNUM *one_mont;
  // w1_mont is w-1 (mod w) in Montgomery form.
  BIGNUM *w1_mont;
  // w_bits is BN_num_bits(w).
  int w_bits;
  // a is the largest integer such that 2^a divides w-1.
  int a;
} BN_MILLER_RABIN;

// bn_miller_rabin_init initializes |miller_rabin| for testing if |mont->N| is
// prime. It returns one on success and zero on error.
OPENSSL_EXPORT int bn_miller_rabin_init(BN_MILLER_RABIN *miller_rabin,
                                        const BN_MONT_CTX *mont, BN_CTX *ctx);

// bn_miller_rabin_iteration performs one Miller-Rabin iteration, checking if
// |b| is a composite witness for |mont->N|. |miller_rabin| must have been
// initialized with |bn_miller_rabin_setup|. On success, it returns one and sets
// |*out_is_possibly_prime| to one if |mont->N| may still be prime or zero if
// |b| shows it is composite. On allocation or internal failure, it returns
// zero.
OPENSSL_EXPORT int bn_miller_rabin_iteration(
    const BN_MILLER_RABIN *miller_rabin, int *out_is_possibly_prime,
    const BIGNUM *b, const BN_MONT_CTX *mont, BN_CTX *ctx);

// bn_rshift1_words sets |r| to |a| >> 1, where both arrays are |num| bits wide.
void bn_rshift1_words(BN_ULONG *r, const BN_ULONG *a, size_t num);

// bn_rshift_words sets |r| to |a| >> |shift|, where both arrays are |num| bits
// wide.
void bn_rshift_words(BN_ULONG *r, const BN_ULONG *a, unsigned shift,
                     size_t num);

// bn_rshift_secret_shift behaves like |BN_rshift| but runs in time independent
// of both |a| and |n|.
OPENSSL_EXPORT int bn_rshift_secret_shift(BIGNUM *r, const BIGNUM *a,
                                          unsigned n, BN_CTX *ctx);

// bn_reduce_once sets |r| to |a| mod |m| where 0 <= |a| < 2*|m|. It returns
// zero if |a| < |m| and a mask of all ones if |a| >= |m|. Each array is |num|
// words long, but |a| has an additional word specified by |carry|. |carry| must
// be zero or one, as implied by the bounds on |a|.
//
// |r|, |a|, and |m| may not alias. Use |bn_reduce_once_in_place| if |r| and |a|
// must alias.
BN_ULONG bn_reduce_once(BN_ULONG *r, const BN_ULONG *a, BN_ULONG carry,
                        const BN_ULONG *m, size_t num);

// bn_reduce_once_in_place behaves like |bn_reduce_once| but acts in-place on
// |r|, using |tmp| as scratch space. |r|, |tmp|, and |m| may not alias.
BN_ULONG bn_reduce_once_in_place(BN_ULONG *r, BN_ULONG carry, const BN_ULONG *m,
                                 BN_ULONG *tmp, size_t num);


// Constant-time non-modular arithmetic.
//
// The following functions implement non-modular arithmetic in constant-time
// and pessimally set |r->width| to the largest possible word size.
//
// Note this means that, e.g., repeatedly multiplying by one will cause widths
// to increase without bound. The corresponding public API functions minimize
// their outputs to avoid regressing calculator consumers.

// bn_uadd_consttime behaves like |BN_uadd|, but it pessimally sets
// |r->width| = |a->width| + |b->width| + 1.
int bn_uadd_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b);

// bn_usub_consttime behaves like |BN_usub|, but it pessimally sets
// |r->width| = |a->width|.
int bn_usub_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b);

// bn_abs_sub_consttime sets |r| to the absolute value of |a| - |b|, treating
// both inputs as secret. It returns one on success and zero on error.
OPENSSL_EXPORT int bn_abs_sub_consttime(BIGNUM *r, const BIGNUM *a,
                                        const BIGNUM *b, BN_CTX *ctx);

// bn_mul_consttime behaves like |BN_mul|, but it rejects negative inputs and
// pessimally sets |r->width| to |a->width| + |b->width|, to avoid leaking
// information about |a| and |b|.
int bn_mul_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, BN_CTX *ctx);

// bn_sqrt_consttime behaves like |BN_sqrt|, but it pessimally sets |r->width|
// to 2*|a->width|, to avoid leaking information about |a| and |b|.
int bn_sqr_consttime(BIGNUM *r, const BIGNUM *a, BN_CTX *ctx);

// bn_div_consttime behaves like |BN_div|, but it rejects negative inputs and
// treats both inputs, including their magnitudes, as secret. It is, as a
// result, much slower than |BN_div| and should only be used for rare operations
// where Montgomery reduction is not available. |divisor_min_bits| is a
// public lower bound for |BN_num_bits(divisor)|. When |divisor|'s bit width is
// public, this can speed up the operation.
//
// Note that |quotient->width| will be set pessimally to |numerator->width|.
OPENSSL_EXPORT int bn_div_consttime(BIGNUM *quotient, BIGNUM *remainder,
                                    const BIGNUM *numerator,
                                    const BIGNUM *divisor,
                                    unsigned divisor_min_bits, BN_CTX *ctx);

// bn_is_relatively_prime checks whether GCD(|x|, |y|) is one. On success, it
// returns one and sets |*out_relatively_prime| to one if the GCD was one and
// zero otherwise. On error, it returns zero.
OPENSSL_EXPORT int bn_is_relatively_prime(int *out_relatively_prime,
                                          const BIGNUM *x, const BIGNUM *y,
                                          BN_CTX *ctx);

// bn_lcm_consttime sets |r| to LCM(|a|, |b|). It returns one and success and
// zero on error. |a| and |b| are both treated as secret.
OPENSSL_EXPORT int bn_lcm_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                                    BN_CTX *ctx);

// bn_mont_ctx_init zero-initialies |mont|.
void bn_mont_ctx_init(BN_MONT_CTX *mont);

// bn_mont_ctx_cleanup releases memory associated with |mont|, without freeing
// |mont| itself.
void bn_mont_ctx_cleanup(BN_MONT_CTX *mont);


// Constant-time modular arithmetic.
//
// The following functions implement basic constant-time modular arithmetic.

// bn_mod_add_words sets |r| to |a| + |b| (mod |m|), using |tmp| as scratch
// space. Each array is |num| words long. |a| and |b| must be < |m|. Any pair of
// |r|, |a|, and |b| may alias.
void bn_mod_add_words(BN_ULONG *r, const BN_ULONG *a, const BN_ULONG *b,
                      const BN_ULONG *m, BN_ULONG *tmp, size_t num);

// bn_mod_add_consttime acts like |BN_mod_add_quick| but takes a |BN_CTX|.
int bn_mod_add_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                         const BIGNUM *m, BN_CTX *ctx);

// bn_mod_sub_words sets |r| to |a| - |b| (mod |m|), using |tmp| as scratch
// space. Each array is |num| words long. |a| and |b| must be < |m|. Any pair of
// |r|, |a|, and |b| may alias.
void bn_mod_sub_words(BN_ULONG *r, const BN_ULONG *a, const BN_ULONG *b,
                      const BN_ULONG *m, BN_ULONG *tmp, size_t num);

// bn_mod_sub_consttime acts like |BN_mod_sub_quick| but takes a |BN_CTX|.
int bn_mod_sub_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                         const BIGNUM *m, BN_CTX *ctx);

// bn_mod_lshift1_consttime acts like |BN_mod_lshift1_quick| but takes a
// |BN_CTX|.
int bn_mod_lshift1_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *m,
                             BN_CTX *ctx);

// bn_mod_lshift_consttime acts like |BN_mod_lshift_quick| but takes a |BN_CTX|.
int bn_mod_lshift_consttime(BIGNUM *r, const BIGNUM *a, int n, const BIGNUM *m,
                            BN_CTX *ctx);

// bn_mod_inverse_consttime sets |r| to |a|^-1, mod |n|. |a| must be non-
// negative and less than |n|. It returns one on success and zero on error. On
// failure, if the failure was caused by |a| having no inverse mod |n| then
// |*out_no_inverse| will be set to one; otherwise it will be set to zero.
//
// This function treats both |a| and |n| as secret, provided they are both non-
// zero and the inverse exists. It should only be used for even moduli where
// none of the less general implementations are applicable.
OPENSSL_EXPORT int bn_mod_inverse_consttime(BIGNUM *r, int *out_no_inverse,
                                            const BIGNUM *a, const BIGNUM *n,
                                            BN_CTX *ctx);

// bn_mod_inverse_prime sets |out| to the modular inverse of |a| modulo |p|,
// computed with Fermat's Little Theorem. It returns one on success and zero on
// error. If |mont_p| is NULL, one will be computed temporarily.
int bn_mod_inverse_prime(BIGNUM *out, const BIGNUM *a, const BIGNUM *p,
                         BN_CTX *ctx, const BN_MONT_CTX *mont_p);

// bn_mod_inverse_secret_prime behaves like |bn_mod_inverse_prime| but uses
// |BN_mod_exp_mont_consttime| instead of |BN_mod_exp_mont| in hopes of
// protecting the exponent.
int bn_mod_inverse_secret_prime(BIGNUM *out, const BIGNUM *a, const BIGNUM *p,
                                BN_CTX *ctx, const BN_MONT_CTX *mont_p);

// BN_MONT_CTX_set_locked takes |lock| and checks whether |*pmont| is NULL. If
// so, it creates a new |BN_MONT_CTX| and sets the modulus for it to |mod|. It
// then stores it as |*pmont|. It returns one on success and zero on error. Note
// this function assumes |mod| is public.
//
// If |*pmont| is already non-NULL then it does nothing and returns one.
int BN_MONT_CTX_set_locked(BN_MONT_CTX **pmont, CRYPTO_MUTEX *lock,
                           const BIGNUM *mod, BN_CTX *bn_ctx);


// Low-level operations for small numbers.
//
// The following functions implement algorithms suitable for use with scalars
// and field elements in elliptic curves. They rely on the number being small
// both to stack-allocate various temporaries and because they do not implement
// optimizations useful for the larger values used in RSA.

// BN_SMALL_MAX_WORDS is the largest size input these functions handle. This
// limit allows temporaries to be more easily stack-allocated. This limit is set
// to accommodate P-521.
#if defined(OPENSSL_32_BIT)
#define BN_SMALL_MAX_WORDS 17
#else
#define BN_SMALL_MAX_WORDS 9
#endif

// bn_mul_small sets |r| to |a|*|b|. |num_r| must be |num_a| + |num_b|. |r| may
// not alias with |a| or |b|.
void bn_mul_small(BN_ULONG *r, size_t num_r, const BN_ULONG *a, size_t num_a,
                 const BN_ULONG *b, size_t num_b);

// bn_sqr_small sets |r| to |a|^2. |num_a| must be at most |BN_SMALL_MAX_WORDS|.
// |num_r| must be |num_a|*2. |r| and |a| may not alias.
void bn_sqr_small(BN_ULONG *r, size_t num_r, const BN_ULONG *a, size_t num_a);

// In the following functions, the modulus must be at most |BN_SMALL_MAX_WORDS|
// words long.

// bn_to_montgomery_small sets |r| to |a| translated to the Montgomery domain.
// |r| and |a| are |num| words long, which must be |mont->N.width|. |a| must be
// fully reduced and may alias |r|.
void bn_to_montgomery_small(BN_ULONG *r, const BN_ULONG *a, size_t num,
                            const BN_MONT_CTX *mont);

// bn_from_montgomery_small sets |r| to |a| translated out of the Montgomery
// domain. |r| and |a| are |num_r| and |num_a| words long, respectively. |num_r|
// must be |mont->N.width|. |a| must be at most |mont->N|^2 and may alias |r|.
//
// Unlike most of these functions, only |num_r| is bounded by
// |BN_SMALL_MAX_WORDS|. |num_a| may exceed it, but must be at most 2 * |num_r|.
void bn_from_montgomery_small(BN_ULONG *r, size_t num_r, const BN_ULONG *a,
                              size_t num_a, const BN_MONT_CTX *mont);

// bn_mod_mul_montgomery_small sets |r| to |a| * |b| mod |mont->N|. Both inputs
// and outputs are in the Montgomery domain. Each array is |num| words long,
// which must be |mont->N.width|. Any two of |r|, |a|, and |b| may alias. |a|
// and |b| must be reduced on input.
void bn_mod_mul_montgomery_small(BN_ULONG *r, const BN_ULONG *a,
                                 const BN_ULONG *b, size_t num,
                                 const BN_MONT_CTX *mont);

// bn_mod_exp_mont_small sets |r| to |a|^|p| mod |mont->N|. It returns one on
// success and zero on programmer or internal error. Both inputs and outputs are
// in the Montgomery domain. |r| and |a| are |num| words long, which must be
// |mont->N.width| and at most |BN_SMALL_MAX_WORDS|. |num_p|, measured in bits,
// must fit in |size_t|. |a| must be fully-reduced. This function runs in time
// independent of |a|, but |p| and |mont->N| are public values. |a| must be
// fully-reduced and may alias with |r|.
//
// Note this function differs from |BN_mod_exp_mont| which uses Montgomery
// reduction but takes input and output outside the Montgomery domain. Combine
// this function with |bn_from_montgomery_small| and |bn_to_montgomery_small|
// if necessary.
void bn_mod_exp_mont_small(BN_ULONG *r, const BN_ULONG *a, size_t num,
                           const BN_ULONG *p, size_t num_p,
                           const BN_MONT_CTX *mont);

// bn_mod_inverse0_prime_mont_small sets |r| to |a|^-1 mod |mont->N|. If |a| is
// zero, |r| is set to zero. |mont->N| must be a prime. |r| and |a| are |num|
// words long, which must be |mont->N.width| and at most |BN_SMALL_MAX_WORDS|.
// |a| must be fully-reduced and may alias |r|. This function runs in time
// independent of |a|, but |mont->N| is a public value.
void bn_mod_inverse0_prime_mont_small(BN_ULONG *r, const BN_ULONG *a,
                                      size_t num, const BN_MONT_CTX *mont);


// Word-based byte conversion functions.

// bn_big_endian_to_words interprets |in_len| bytes from |in| as a big-endian,
// unsigned integer and writes the result to |out_len| words in |out|. |out_len|
// must be large enough to represent any |in_len|-byte value. That is, |in_len|
// must be at most |BN_BYTES * out_len|.
void bn_big_endian_to_words(BN_ULONG *out, size_t out_len, const uint8_t *in,
                            size_t in_len);

// bn_words_to_big_endian represents |in_len| words from |in| as a big-endian,
// unsigned integer in |out_len| bytes. It writes the result to |out|. |out_len|
// must be large enough to represent |in| without truncation.
//
// Note |out_len| may be less than |BN_BYTES * in_len| if |in| is known to have
// leading zeros.
void bn_words_to_big_endian(uint8_t *out, size_t out_len, const BN_ULONG *in,
                            size_t in_len);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_BN_INTERNAL_H
