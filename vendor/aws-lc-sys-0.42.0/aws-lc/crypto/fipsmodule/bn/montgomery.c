// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2006 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/bn.h>

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/thread.h>
#include <openssl/type_check.h>

#include "internal.h"
#include "../cpucap/internal.h"
#include "../../internal.h"

#if !defined(OPENSSL_NO_ASM) &&                          \
    (defined(OPENSSL_LINUX) || defined(OPENSSL_APPLE) || \
     defined(OPENSSL_OPENBSD) || defined(OPENSSL_FREEBSD) || \
     defined(OPENSSL_NETBSD) ) &&                        \
    defined(OPENSSL_AARCH64) && defined(OPENSSL_BN_ASM_MONT)

#include "../../../third_party/s2n-bignum/s2n-bignum_aws-lc.h"

#define BN_MONTGOMERY_S2N_BIGNUM_CAPABLE 1

OPENSSL_INLINE int montgomery_use_s2n_bignum(unsigned int num) {
  // Use s2n-bignum's functions only if
  // (1) The ARM architecture has slow multipliers, and
  // (2) num (which is the number of words) is multiplie of 8, because
  //     s2n-bignum's bignum_emontredc_8n requires it, and
  // (3) The word size is 64 bits.
  // (4) CPU has NEON.
  assert(S2NBIGNUM_KSQR_16_32_TEMP_NWORDS <= S2NBIGNUM_KMUL_32_64_TEMP_NWORDS &&
         S2NBIGNUM_KSQR_32_64_TEMP_NWORDS <= S2NBIGNUM_KMUL_32_64_TEMP_NWORDS &&
         S2NBIGNUM_KMUL_16_32_TEMP_NWORDS <= S2NBIGNUM_KMUL_32_64_TEMP_NWORDS);
  assert(BN_BITS2 == 64);
  return !CRYPTO_is_ARMv8_wide_multiplier_capable() &&
          (num % 8 == 0) &&
          CRYPTO_is_NEON_capable();
}

#else

OPENSSL_INLINE int montgomery_use_s2n_bignum(unsigned int num) {
  return 0;
}

#endif

void bn_mont_ctx_init(BN_MONT_CTX *mont) {
  OPENSSL_memset(mont, 0, sizeof(BN_MONT_CTX));
  BN_init(&mont->RR);
  BN_init(&mont->N);
}

void bn_mont_ctx_cleanup(BN_MONT_CTX *mont) {
  BN_free(&mont->RR);
  BN_free(&mont->N);
}

BN_MONT_CTX *BN_MONT_CTX_new(void) {
  BN_MONT_CTX *ret = OPENSSL_malloc(sizeof(BN_MONT_CTX));
  if (ret == NULL) {
    return NULL;
  }

  bn_mont_ctx_init(ret);
  return ret;
}

void BN_MONT_CTX_free(BN_MONT_CTX *mont) {
  if (mont == NULL) {
    return;
  }

  bn_mont_ctx_cleanup(mont);
  OPENSSL_free(mont);
}

BN_MONT_CTX *BN_MONT_CTX_copy(BN_MONT_CTX *to, const BN_MONT_CTX *from) {
  if (to == from) {
    return to;
  }

  if (!BN_copy(&to->RR, &from->RR) ||
      !BN_copy(&to->N, &from->N)) {
    return NULL;
  }
  to->n0[0] = from->n0[0];
  to->n0[1] = from->n0[1];
  return to;
}

static int bn_mont_ctx_set_N_and_n0(BN_MONT_CTX *mont, const BIGNUM *mod) {
  if (BN_is_zero(mod)) {
    OPENSSL_PUT_ERROR(BN, BN_R_DIV_BY_ZERO);
    return 0;
  }
  if (!BN_is_odd(mod)) {
    OPENSSL_PUT_ERROR(BN, BN_R_CALLED_WITH_EVEN_MODULUS);
    return 0;
  }
  if (BN_is_negative(mod)) {
    OPENSSL_PUT_ERROR(BN, BN_R_NEGATIVE_NUMBER);
    return 0;
  }
  if (!bn_fits_in_words(mod, BN_MONTGOMERY_MAX_WORDS)) {
    OPENSSL_PUT_ERROR(BN, BN_R_BIGNUM_TOO_LONG);
    return 0;
  }

  // Save the modulus.
  if (!BN_copy(&mont->N, mod)) {
    OPENSSL_PUT_ERROR(BN, ERR_R_INTERNAL_ERROR);
    return 0;
  }
  // |mont->N| is always stored minimally. Computing RR efficiently leaks the
  // size of the modulus. While the modulus may be private in RSA (one of the
  // primes), their sizes are public, so this is fine.
  bn_set_minimal_width(&mont->N);

  // Find n0 such that n0 * N == -1 (mod r).
  //
  // Only certain BN_BITS2<=32 platforms actually make use of n0[1]. For the
  // others, we could use a shorter R value and use faster |BN_ULONG|-based
  // math instead of |uint64_t|-based math, which would be double-precision.
  // However, currently only the assembler files know which is which.
  OPENSSL_STATIC_ASSERT(BN_MONT_CTX_N0_LIMBS == 1 || BN_MONT_CTX_N0_LIMBS == 2,
                        BN_MONT_CTX_N0_LIMBS_value_is_invalid)
  OPENSSL_STATIC_ASSERT(
      sizeof(BN_ULONG) * BN_MONT_CTX_N0_LIMBS == sizeof(uint64_t),
      uint64_t_is_insufficient_precision_for_n0);
  uint64_t n0 = bn_mont_n0(&mont->N);
  mont->n0[0] = (BN_ULONG)n0;
#if BN_MONT_CTX_N0_LIMBS == 2
  mont->n0[1] = (BN_ULONG)(n0 >> BN_BITS2);
#else
  mont->n0[1] = 0;
#endif
  return 1;
}

int BN_MONT_CTX_set(BN_MONT_CTX *mont, const BIGNUM *mod, BN_CTX *ctx) {
  if (!bn_mont_ctx_set_N_and_n0(mont, mod)) {
    return 0;
  }

  BN_CTX *new_ctx = NULL;
  if (ctx == NULL) {
    new_ctx = BN_CTX_new();
    if (new_ctx == NULL) {
      return 0;
    }
    ctx = new_ctx;
  }

  // Save RR = R**2 (mod N). R is the smallest power of 2**BN_BITS2 such that R
  // > mod. Even though the assembly on some 32-bit platforms works with 64-bit
  // values, using |BN_BITS2| here, rather than |BN_MONT_CTX_N0_LIMBS *
  // BN_BITS2|, is correct because R**2 will still be a multiple of the latter
  // as |BN_MONT_CTX_N0_LIMBS| is either one or two.
  unsigned lgBigR = mont->N.width * BN_BITS2;
  BN_zero(&mont->RR);
  int ok = BN_set_bit(&mont->RR, lgBigR * 2) &&
           BN_mod(&mont->RR, &mont->RR, &mont->N, ctx) &&
           bn_resize_words(&mont->RR, mont->N.width);
  BN_CTX_free(new_ctx);
  return ok;
}

BN_MONT_CTX *BN_MONT_CTX_new_for_modulus(const BIGNUM *mod, BN_CTX *ctx) {
  BN_MONT_CTX *mont = BN_MONT_CTX_new();
  if (mont == NULL ||
      !BN_MONT_CTX_set(mont, mod, ctx)) {
    BN_MONT_CTX_free(mont);
    return NULL;
  }
  return mont;
}

BN_MONT_CTX *BN_MONT_CTX_new_consttime(const BIGNUM *mod, BN_CTX *ctx) {
  BN_MONT_CTX *mont = BN_MONT_CTX_new();
  if (mont == NULL ||
      !bn_mont_ctx_set_N_and_n0(mont, mod) ||
      !bn_mont_ctx_set_RR_consttime(mont, ctx)) {
    BN_MONT_CTX_free(mont);
    return NULL;
  }
  return mont;
}

int BN_MONT_CTX_set_locked(BN_MONT_CTX **pmont, CRYPTO_MUTEX *lock,
                           const BIGNUM *mod, BN_CTX *bn_ctx) {
  CRYPTO_MUTEX_lock_read(lock);
  BN_MONT_CTX *ctx = *pmont;
  CRYPTO_MUTEX_unlock_read(lock);

  if (ctx) {
    return 1;
  }

  CRYPTO_MUTEX_lock_write(lock);
  if (*pmont == NULL) {
    *pmont = BN_MONT_CTX_new_for_modulus(mod, bn_ctx);
  }
  const int ok = *pmont != NULL;
  CRYPTO_MUTEX_unlock_write(lock);
  return ok;
}

int BN_to_montgomery(BIGNUM *ret, const BIGNUM *a, const BN_MONT_CTX *mont,
                     BN_CTX *ctx) {
  return BN_mod_mul_montgomery(ret, a, &mont->RR, mont, ctx);
}

static int bn_from_montgomery_in_place(BN_ULONG *r, size_t num_r, BN_ULONG *a,
                                       size_t num_a, const BN_MONT_CTX *mont) {
  const BN_ULONG *n = mont->N.d;
  size_t num_n = mont->N.width;
  if (num_r != num_n || num_a != 2 * num_n) {
    OPENSSL_PUT_ERROR(BN, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  // Add multiples of |n| to |r| until R = 2^(nl * BN_BITS2) divides it. On
  // input, we had |r| < |n| * R, so now |r| < 2 * |n| * R. Note that |r|
  // includes |carry| which is stored separately.
  BN_ULONG n0 = mont->n0[0];
  BN_ULONG carry = 0;
  for (size_t i = 0; i < num_n; i++) {
    BN_ULONG v = bn_mul_add_words(a + i, n, num_n, a[i] * n0);
    v += carry + a[i + num_n];
    carry |= (v != a[i + num_n]);
    carry &= (v <= a[i + num_n]);
    a[i + num_n] = v;
  }

  // Shift |num_n| words to divide by R. We have |a| < 2 * |n|. Note that |a|
  // includes |carry| which is stored separately.
  a += num_n;

  // |a| thus requires at most one additional subtraction |n| to be reduced.
  bn_reduce_once(r, a, carry, n, num_n);
  return 1;
}

static int BN_from_montgomery_word(BIGNUM *ret, BIGNUM *r,
                                   const BN_MONT_CTX *mont) {
  if (r->neg) {
    OPENSSL_PUT_ERROR(BN, BN_R_NEGATIVE_NUMBER);
    return 0;
  }

  const BIGNUM *n = &mont->N;
  if (n->width == 0) {
    ret->width = 0;
    return 1;
  }

  int max = 2 * n->width;  // carry is stored separately
  if (!bn_resize_words(r, max) ||
      !bn_wexpand(ret, n->width)) {
    return 0;
  }

  ret->width = n->width;
  ret->neg = 0;
  return bn_from_montgomery_in_place(ret->d, ret->width, r->d, r->width, mont);
}

int BN_from_montgomery(BIGNUM *r, const BIGNUM *a, const BN_MONT_CTX *mont,
                       BN_CTX *ctx) {
  int ret = 0;
  BIGNUM *t;

  BN_CTX_start(ctx);
  t = BN_CTX_get(ctx);
  if (t == NULL ||
      !BN_copy(t, a)) {
    goto err;
  }

  ret = BN_from_montgomery_word(r, t, mont);

err:
  BN_CTX_end(ctx);

  return ret;
}

int bn_one_to_montgomery(BIGNUM *r, const BN_MONT_CTX *mont, BN_CTX *ctx) {
  // If the high bit of |n| is set, R = 2^(width*BN_BITS2) < 2 * |n|, so we
  // compute R - |n| rather than perform Montgomery reduction.
  const BIGNUM *n = &mont->N;
  if (n->width > 0 && (n->d[n->width - 1] >> (BN_BITS2 - 1)) != 0) {
    if (!bn_wexpand(r, n->width)) {
      return 0;
    }
    r->d[0] = 0 - n->d[0];
    for (int i = 1; i < n->width; i++) {
      r->d[i] = ~n->d[i];
    }
    r->width = n->width;
    r->neg = 0;
    return 1;
  }

  return BN_from_montgomery(r, &mont->RR, mont, ctx);
}

static int bn_mod_mul_montgomery_fallback(BIGNUM *r, const BIGNUM *a,
                                          const BIGNUM *b,
                                          const BN_MONT_CTX *mont,
                                          BN_CTX *ctx) {
  int ret = 0;

  BN_CTX_start(ctx);
  BIGNUM *tmp = BN_CTX_get(ctx);
  if (tmp == NULL) {
    goto err;
  }

  if (a == b) {
    if (!bn_sqr_consttime(tmp, a, ctx)) {
      goto err;
    }
  } else {
    if (!bn_mul_consttime(tmp, a, b, ctx)) {
      goto err;
    }
  }

  // reduce from aRR to aR
  if (!BN_from_montgomery_word(r, tmp, mont)) {
    goto err;
  }

  ret = 1;

err:
  BN_CTX_end(ctx);
  return ret;
}


#if defined(OPENSSL_BN_ASM_MONT)

// Perform montgomery multiplication using s2n-bignum functions. The arguments
// are equivalent to the arguments of bn_mul_mont.
// montgomery_s2n_bignum_mul_mont works only if num is a multiple of 8.
// montgomery_use_s2n_bignum(num) must be called in advance to check this
// condition, as well as other s2n-bignum requirements.
// For num = 32 or num = 16, this uses faster primitives in s2n-bignum.
// montgomery_s2n_bignum_mul_mont allocates S2NBIGNUM_KMUL_32_64_TEMP_NWORDS +
// 2 * BN_MONTGOMERY_MAX_WORDS uint64_t words at the stack.
static void montgomery_s2n_bignum_mul_mont(BN_ULONG *rp, const BN_ULONG *ap,
                                           const BN_ULONG *bp,
                                           const BN_ULONG *np,
                                           const BN_ULONG *n0, size_t num) {

#if defined(BN_MONTGOMERY_S2N_BIGNUM_CAPABLE)

  // t is a temporary buffer used by Karatsuba multiplication.
  // bignum_kmul_32_64 requires S2NBIGNUM_KMUL_32_64_TEMP_NWORDS words.
  uint64_t t[S2NBIGNUM_KMUL_32_64_TEMP_NWORDS];
  // mulres is the output buffer of big-int multiplication which uses
  // 2 * num elements of mulres. Note that num <= BN_MONTGOMERY_MAX_WORDS
  // is guaranteed by the caller (BN_mod_mul_montgomery).
  uint64_t mulres[2 * BN_MONTGOMERY_MAX_WORDS];

  // Given m the prime number stored at np, m * w = -1 mod 2^64.
  uint64_t w = n0[0];

  if (num == 32) {
    if (ap == bp) {
      bignum_ksqr_32_64(mulres, ap, t);
    } else {
      bignum_kmul_32_64(mulres, ap, bp, t);
    }
  } else if (num == 16) {
    if (ap == bp) {
      bignum_ksqr_16_32(mulres, ap, t);
    } else {
      bignum_kmul_16_32(mulres, ap, bp, t);
    }
  } else {
    if (ap == bp) {
      bignum_sqr(num * 2, mulres, num, ap);
    } else {
      bignum_mul(num * 2, mulres, num, ap, num, bp);
    }
  }

  // Do montgomery reduction. We follow the definition of montgomery reduction
  // which is:
  // 1. Calculate (mulres + ((mulres mod R) * (-m^-1 mod R) mod R) * m) / R
  //    using bignum_emontredc_8n, where R is 2^(64*num).
  //    The calculated result is stored in [mulres+num ... mulres+2*num-1]. If
  //    the result >= 2^(64*num), bignum_emontredc_8n returns 1.
  // 2. Optionally subtract the result if the (result of step 1) >= m.
  //    The comparison is true if either A or B holds:
  //    A. The result of step 1 >= 2^(64*num), meaning that bignum_emontredc_8n
  //       returned 1. Since m is less than 2^(64*num), (result of step 1) >= m holds.
  //    B. The result of step 1 fits in 2^(64*num), and the result >= m.
  uint64_t c = bignum_emontredc_8n(num, mulres, np, w); // c: case A
  c |= bignum_ge(num, mulres + num, num, np);  // c: case B
  // Optionally subtract and store the result at rp
  bignum_optsub(num, rp, mulres + num, c, np);

#else

  // Should not call this function unless s2n-bignum is supported.
  abort();

#endif
}

#endif


int BN_mod_mul_montgomery(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                          const BN_MONT_CTX *mont, BN_CTX *ctx) {
  if (a->neg || b->neg) {
    OPENSSL_PUT_ERROR(BN, BN_R_NEGATIVE_NUMBER);
    return 0;
  }

#if defined(OPENSSL_BN_ASM_MONT)
  // |bn_mul_mont| requires at least 128 bits of limbs, at least for x86.
  int num = mont->N.width;
  if (num >= (128 / BN_BITS2) &&
      a->width == num &&
      b->width == num) {
    if (!bn_wexpand(r, num)) {
      return 0;
    }
    // This bound is implied by |bn_mont_ctx_set_N_and_n0|. |bn_mul_mont|
    // allocates |num| words on the stack, so |num| cannot be too large.
    assert((size_t)num <= BN_MONTGOMERY_MAX_WORDS);

    if (montgomery_use_s2n_bignum(num)) {
      // Do montgomery multiplication using s2n-bignum.
      montgomery_s2n_bignum_mul_mont(r->d, a->d, b->d, mont->N.d, mont->n0,
                                     num);
    } else {
      if (!bn_mul_mont(r->d, a->d, b->d, mont->N.d, mont->n0, num)) {
        // The check above ensures this won't happen.
        assert(0);
        OPENSSL_PUT_ERROR(BN, ERR_R_INTERNAL_ERROR);
        return 0;
      }
    }
    r->neg = 0;
    r->width = num;
    return 1;
  }
#endif

  return bn_mod_mul_montgomery_fallback(r, a, b, mont, ctx);
}

int bn_less_than_montgomery_R(const BIGNUM *bn, const BN_MONT_CTX *mont) {
  return !BN_is_negative(bn) &&
         bn_fits_in_words(bn, mont->N.width);
}

void bn_to_montgomery_small(BN_ULONG *r, const BN_ULONG *a, size_t num,
                            const BN_MONT_CTX *mont) {
  bn_mod_mul_montgomery_small(r, a, mont->RR.d, num, mont);
}

void bn_from_montgomery_small(BN_ULONG *r, size_t num_r, const BN_ULONG *a,
                              size_t num_a, const BN_MONT_CTX *mont) {
  if (num_r != (size_t)mont->N.width || num_r > BN_SMALL_MAX_WORDS ||
      num_a > 2 * num_r) {
    abort();
  }
  BN_ULONG tmp[BN_SMALL_MAX_WORDS * 2] = {0};
  OPENSSL_memcpy(tmp, a, num_a * sizeof(BN_ULONG));
  if (!bn_from_montgomery_in_place(r, num_r, tmp, 2 * num_r, mont)) {
    abort();
  }
  OPENSSL_cleanse(tmp, 2 * num_r * sizeof(BN_ULONG));
}

void bn_mod_mul_montgomery_small(BN_ULONG *r, const BN_ULONG *a,
                                 const BN_ULONG *b, size_t num,
                                 const BN_MONT_CTX *mont) {
  if (num != (size_t)mont->N.width || num > BN_SMALL_MAX_WORDS) {
    abort();
  }

#if defined(OPENSSL_BN_ASM_MONT)
  // |bn_mul_mont| requires at least 128 bits of limbs, at least for x86.
  if (num >= (128 / BN_BITS2)) {
    if (!bn_mul_mont(r, a, b, mont->N.d, mont->n0, num)) {
      abort();  // The check above ensures this won't happen.
    }
    return;
  }
#endif

  // Compute the product.
  BN_ULONG tmp[2 * BN_SMALL_MAX_WORDS];
  if (a == b) {
    bn_sqr_small(tmp, 2 * num, a, num);
  } else {
    bn_mul_small(tmp, 2 * num, a, num, b, num);
  }

  // Reduce.
  if (!bn_from_montgomery_in_place(r, num, tmp, 2 * num, mont)) {
    abort();
  }
  OPENSSL_cleanse(tmp, 2 * num * sizeof(BN_ULONG));
}

#if defined(OPENSSL_BN_ASM_MONT) && defined(OPENSSL_X86_64)
int bn_mul_mont(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                const BN_ULONG *np, const BN_ULONG *n0, size_t num)
{
#if !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
  if (ap == bp && bn_sqr8x_mont_capable(num)) {
    return bn_sqr8x_mont(rp, ap, bn_mulx_adx_capable(), np, n0, num);
  }
  if (bn_mulx4x_mont_capable(num)) {
    return bn_mulx4x_mont(rp, ap, bp, np, n0, num);
  }
#endif // !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
  if (bn_mul4x_mont_capable(num)) {
    return bn_mul4x_mont(rp, ap, bp, np, n0, num);
  }
  return bn_mul_mont_nohw(rp, ap, bp, np, n0, num);
}
#endif

#if defined(OPENSSL_BN_ASM_MONT) && defined(OPENSSL_ARM)
int bn_mul_mont(BN_ULONG *rp, const BN_ULONG *ap, const BN_ULONG *bp,
                const BN_ULONG *np, const BN_ULONG *n0, size_t num) {
  if (bn_mul8x_mont_neon_capable(num)) {
    return bn_mul8x_mont_neon(rp, ap, bp, np, n0, num);
  }
  return bn_mul_mont_nohw(rp, ap, bp, np, n0, num);
}
#endif
