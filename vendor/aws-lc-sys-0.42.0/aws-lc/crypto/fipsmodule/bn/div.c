// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/bn.h>

#include <assert.h>
#include <limits.h>

#include <openssl/err.h>

#include "internal.h"


// bn_div_words divides a double-width |h|,|l| by |d| and returns the result,
// which must fit in a |BN_ULONG|.
OPENSSL_UNUSED static BN_ULONG bn_div_words(BN_ULONG h, BN_ULONG l,
                                            BN_ULONG d) {
  BN_ULONG dh, dl, q, ret = 0, th, tl, t;
  int i, count = 2;

  if (d == 0) {
    return BN_MASK2;
  }

  i = BN_num_bits_word(d);
  assert((i == BN_BITS2) || (h <= (BN_ULONG)1 << i));

  i = BN_BITS2 - i;
  if (h >= d) {
    h -= d;
  }

  if (i) {
    d <<= i;
    h = (h << i) | (l >> (BN_BITS2 - i));
    l <<= i;
  }
  dh = (d & BN_MASK2h) >> BN_BITS4;
  dl = (d & BN_MASK2l);
  for (;;) {
    if ((h >> BN_BITS4) == dh) {
      q = BN_MASK2l;
    } else {
      q = h / dh;
    }

    th = q * dh;
    tl = dl * q;
    for (;;) {
      t = h - th;
      if ((t & BN_MASK2h) ||
          ((tl) <= ((t << BN_BITS4) | ((l & BN_MASK2h) >> BN_BITS4)))) {
        break;
      }
      q--;
      th -= dh;
      tl -= dl;
    }
    t = (tl >> BN_BITS4);
    tl = (tl << BN_BITS4) & BN_MASK2h;
    th += t;

    if (l < tl) {
      th++;
    }
    l -= tl;
    if (h < th) {
      h += d;
      q--;
    }
    h -= th;

    if (--count == 0) {
      break;
    }

    ret = q << BN_BITS4;
    h = (h << BN_BITS4) | (l >> BN_BITS4);
    l = (l & BN_MASK2l) << BN_BITS4;
  }

  ret |= q;
  return ret;
}

static inline void bn_div_rem_words(BN_ULONG *quotient_out, BN_ULONG *rem_out,
                                    BN_ULONG n0, BN_ULONG n1, BN_ULONG d0) {
  // GCC and Clang generate function calls to |__udivdi3| and |__umoddi3| when
  // the |BN_ULLONG|-based C code is used.
  //
  // GCC bugs:
  //   * https://gcc.gnu.org/bugzilla/show_bug.cgi?id=14224
  //   * https://gcc.gnu.org/bugzilla/show_bug.cgi?id=43721
  //   * https://gcc.gnu.org/bugzilla/show_bug.cgi?id=54183
  //   * https://gcc.gnu.org/bugzilla/show_bug.cgi?id=58897
  //   * https://gcc.gnu.org/bugzilla/show_bug.cgi?id=65668
  //
  // Clang bugs:
  //   * https://llvm.org/bugs/show_bug.cgi?id=6397
  //   * https://llvm.org/bugs/show_bug.cgi?id=12418
  //
  // These issues aren't specific to x86 and x86_64, so it might be worthwhile
  // to add more assembly language implementations.
#if defined(BN_CAN_USE_INLINE_ASM) && defined(OPENSSL_X86)
  __asm__ volatile("divl %4"
                   : "=a"(*quotient_out), "=d"(*rem_out)
                   : "a"(n1), "d"(n0), "rm"(d0)
                   : "cc");
#elif defined(BN_CAN_USE_INLINE_ASM) && defined(OPENSSL_X86_64)
  __asm__ volatile("divq %4"
                   : "=a"(*quotient_out), "=d"(*rem_out)
                   : "a"(n1), "d"(n0), "rm"(d0)
                   : "cc");
#else
#if defined(BN_CAN_DIVIDE_ULLONG)
  BN_ULLONG n = (((BN_ULLONG)n0) << BN_BITS2) | n1;
  *quotient_out = (BN_ULONG)(n / d0);
#else
  *quotient_out = bn_div_words(n0, n1, d0);
#endif
  *rem_out = n1 - (*quotient_out * d0);
#endif
}

// BN_div computes "quotient := numerator / divisor", rounding towards zero,
// and sets up |rem| such that "quotient * divisor + rem = numerator" holds.
//
// Thus:
//
//     quotient->neg == numerator->neg ^ divisor->neg
//        (unless the result is zero)
//     rem->neg == numerator->neg
//        (unless the remainder is zero)
//
// If |quotient| or |rem| is NULL, the respective value is not returned.
//
// This was specifically designed to contain fewer branches that may leak
// sensitive information; see "New Branch Prediction Vulnerabilities in OpenSSL
// and Necessary Software Countermeasures" by Onur Acıçmez, Shay Gueron, and
// Jean-Pierre Seifert.
int BN_div(BIGNUM *quotient, BIGNUM *rem, const BIGNUM *numerator,
           const BIGNUM *divisor, BN_CTX *ctx) {
  int norm_shift, loop;
  BIGNUM wnum;
  BN_ULONG *resp, *wnump;
  BN_ULONG d0, d1;
  int num_n, div_n;

  // This function relies on the historical minimal-width |BIGNUM| invariant.
  // It is already not constant-time (constant-time reductions should use
  // Montgomery logic), so we shrink all inputs and intermediate values to
  // retain the previous behavior.

  // Invalid zero-padding would have particularly bad consequences.
  int numerator_width = bn_minimal_width(numerator);
  int divisor_width = bn_minimal_width(divisor);
  if ((numerator_width > 0 && numerator->d[numerator_width - 1] == 0) ||
      (divisor_width > 0 && divisor->d[divisor_width - 1] == 0)) {
    OPENSSL_PUT_ERROR(BN, BN_R_NOT_INITIALIZED);
    return 0;
  }

  if (BN_is_zero(divisor)) {
    OPENSSL_PUT_ERROR(BN, BN_R_DIV_BY_ZERO);
    return 0;
  }

  BN_CTX_start(ctx);
  BIGNUM *tmp = BN_CTX_get(ctx);
  BIGNUM *snum = BN_CTX_get(ctx);
  BIGNUM *sdiv = BN_CTX_get(ctx);
  BIGNUM *res = NULL;
  if (quotient == NULL) {
    res = BN_CTX_get(ctx);
  } else {
    res = quotient;
  }
  if (sdiv == NULL || res == NULL) {
    goto err;
  }

  // First we normalise the numbers
  norm_shift = BN_BITS2 - (BN_num_bits(divisor) % BN_BITS2);
  if (!BN_lshift(sdiv, divisor, norm_shift)) {
    goto err;
  }
  bn_set_minimal_width(sdiv);
  sdiv->neg = 0;
  norm_shift += BN_BITS2;
  if (!BN_lshift(snum, numerator, norm_shift)) {
    goto err;
  }
  bn_set_minimal_width(snum);
  snum->neg = 0;

  // Since we don't want to have special-case logic for the case where snum is
  // larger than sdiv, we pad snum with enough zeroes without changing its
  // value.
  if (snum->width <= sdiv->width + 1) {
    if (!bn_wexpand(snum, sdiv->width + 2)) {
      goto err;
    }
    for (int i = snum->width; i < sdiv->width + 2; i++) {
      snum->d[i] = 0;
    }
    snum->width = sdiv->width + 2;
  } else {
    if (!bn_wexpand(snum, snum->width + 1)) {
      goto err;
    }
    snum->d[snum->width] = 0;
    snum->width++;
  }

  div_n = sdiv->width;
  num_n = snum->width;
  loop = num_n - div_n;
  // Lets setup a 'window' into snum
  // This is the part that corresponds to the current
  // 'area' being divided
  wnum.neg = 0;
  wnum.d = &(snum->d[loop]);
  wnum.width = div_n;
  // only needed when BN_ucmp messes up the values between width and max
  wnum.dmax = snum->dmax - loop;  // so we don't step out of bounds

  // Get the top 2 words of sdiv
  // div_n=sdiv->width;
  d0 = sdiv->d[div_n - 1];
  d1 = (div_n == 1) ? 0 : sdiv->d[div_n - 2];

  // pointer to the 'top' of snum
  wnump = &(snum->d[num_n - 1]);

  // Setup |res|. |numerator| and |res| may alias, so we save |numerator->neg|
  // for later.
  const int numerator_neg = numerator->neg;
  res->neg = (numerator_neg ^ divisor->neg);
  if (!bn_wexpand(res, loop + 1)) {
    goto err;
  }
  res->width = loop - 1;
  resp = &(res->d[loop - 1]);

  // space for temp
  if (!bn_wexpand(tmp, div_n + 1)) {
    goto err;
  }

  // if res->width == 0 then clear the neg value otherwise decrease
  // the resp pointer
  if (res->width == 0) {
    res->neg = 0;
  } else {
    resp--;
  }

  for (int i = 0; i < loop - 1; i++, wnump--, resp--) {
    BN_ULONG q, l0;
    // the first part of the loop uses the top two words of snum and sdiv to
    // calculate a BN_ULONG q such that | wnum - sdiv * q | < sdiv
    BN_ULONG n0, n1, rm = 0;

    n0 = wnump[0];
    n1 = wnump[-1];
    if (n0 == d0) {
      q = BN_MASK2;
    } else {
      // n0 < d0
      bn_div_rem_words(&q, &rm, n0, n1, d0);

#ifdef BN_ULLONG
      BN_ULLONG t2 = (BN_ULLONG)d1 * q;
      for (;;) {
        if (t2 <= ((((BN_ULLONG)rm) << BN_BITS2) | wnump[-2])) {
          break;
        }
        q--;
        rm += d0;
        if (rm < d0) {
          break;  // don't let rm overflow
        }
        t2 -= d1;
      }
#else  // !BN_ULLONG
      BN_ULONG t2l, t2h;
      BN_UMULT_LOHI(t2l, t2h, d1, q);
      for (;;) {
        if (t2h < rm ||
            (t2h == rm && t2l <= wnump[-2])) {
          break;
        }
        q--;
        rm += d0;
        if (rm < d0) {
          break;  // don't let rm overflow
        }
        if (t2l < d1) {
          t2h--;
        }
        t2l -= d1;
      }
#endif  // !BN_ULLONG
    }

    l0 = bn_mul_words(tmp->d, sdiv->d, div_n, q);
    tmp->d[div_n] = l0;
    wnum.d--;
    // ingore top values of the bignums just sub the two
    // BN_ULONG arrays with bn_sub_words
    if (bn_sub_words(wnum.d, wnum.d, tmp->d, div_n + 1)) {
      // Note: As we have considered only the leading
      // two BN_ULONGs in the calculation of q, sdiv * q
      // might be greater than wnum (but then (q-1) * sdiv
      // is less or equal than wnum)
      q--;
      if (bn_add_words(wnum.d, wnum.d, sdiv->d, div_n)) {
        // we can't have an overflow here (assuming
        // that q != 0, but if q == 0 then tmp is
        // zero anyway)
        (*wnump)++;
      }
    }
    // store part of the result
    *resp = q;
  }

  bn_set_minimal_width(snum);

  if (rem != NULL) {
    if (!BN_rshift(rem, snum, norm_shift)) {
      goto err;
    }
    if (!BN_is_zero(rem)) {
      rem->neg = numerator_neg;
    }
  }

  bn_set_minimal_width(res);
  BN_CTX_end(ctx);
  return 1;

err:
  BN_CTX_end(ctx);
  return 0;
}

int BN_nnmod(BIGNUM *r, const BIGNUM *m, const BIGNUM *d, BN_CTX *ctx) {
  if (!(BN_mod(r, m, d, ctx))) {
    return 0;
  }
  if (!r->neg) {
    return 1;
  }

  // now -|d| < r < 0, so we have to set r := r + |d|.
  return (d->neg ? BN_sub : BN_add)(r, r, d);
}

BN_ULONG bn_reduce_once(BN_ULONG *r, const BN_ULONG *a, BN_ULONG carry,
                        const BN_ULONG *m, size_t num) {
  assert(r != a);
  // |r| = |a| - |m|. |bn_sub_words| performs the bulk of the subtraction, and
  // then we apply the borrow to |carry|.
  carry -= bn_sub_words(r, a, m, num);
  // We know 0 <= |a| < 2*|m|, so -|m| <= |r| < |m|.
  //
  // If 0 <= |r| < |m|, |r| fits in |num| words and |carry| is zero. We then
  // wish to select |r| as the answer. Otherwise -m <= r < 0 and we wish to
  // return |r| + |m|, or |a|. |carry| must then be -1 or all ones. In both
  // cases, |carry| is a suitable input to |bn_select_words|.
  //
  // Although |carry| may be one if it was one on input and |bn_sub_words|
  // returns zero, this would give |r| > |m|, violating our input assumptions.
  declassify_assert(carry + 1 <= 1);
  bn_select_words(r, carry, a /* r < 0 */, r /* r >= 0 */, num);
  return carry;
}

BN_ULONG bn_reduce_once_in_place(BN_ULONG *r, BN_ULONG carry, const BN_ULONG *m,
                                 BN_ULONG *tmp, size_t num) {
  // See |bn_reduce_once| for why this logic works.
  carry -= bn_sub_words(tmp, r, m, num);
  declassify_assert(carry + 1 <= 1);
  bn_select_words(r, carry, r /* tmp < 0 */, tmp /* tmp >= 0 */, num);
  return carry;
}

void bn_mod_sub_words(BN_ULONG *r, const BN_ULONG *a, const BN_ULONG *b,
                      const BN_ULONG *m, BN_ULONG *tmp, size_t num) {
  // r = a - b
  BN_ULONG borrow = bn_sub_words(r, a, b, num);
  // tmp = a - b + m
  bn_add_words(tmp, r, m, num);
  bn_select_words(r, 0 - borrow, tmp /* r < 0 */, r /* r >= 0 */, num);
}

void bn_mod_add_words(BN_ULONG *r, const BN_ULONG *a, const BN_ULONG *b,
                      const BN_ULONG *m, BN_ULONG *tmp, size_t num) {
  BN_ULONG carry = bn_add_words(r, a, b, num);
  bn_reduce_once_in_place(r, carry, m, tmp, num);
}

int bn_div_consttime(BIGNUM *quotient, BIGNUM *remainder,
                     const BIGNUM *numerator, const BIGNUM *divisor,
                     unsigned divisor_min_bits, BN_CTX *ctx) {
  if (BN_is_negative(numerator) || BN_is_negative(divisor)) {
    OPENSSL_PUT_ERROR(BN, BN_R_NEGATIVE_NUMBER);
    return 0;
  }
  if (BN_is_zero(divisor)) {
    OPENSSL_PUT_ERROR(BN, BN_R_DIV_BY_ZERO);
    return 0;
  }

  // This function implements long division in binary. It is not very efficient,
  // but it is simple, easy to make constant-time, and performant enough for RSA
  // key generation.

  int ret = 0;
  BN_CTX_start(ctx);
  BIGNUM *q = quotient, *r = remainder;
  if (quotient == NULL || quotient == numerator || quotient == divisor) {
    q = BN_CTX_get(ctx);
  }
  if (remainder == NULL || remainder == numerator || remainder == divisor) {
    r = BN_CTX_get(ctx);
  }
  BIGNUM *tmp = BN_CTX_get(ctx);
  if (q == NULL || r == NULL || tmp == NULL ||
      !bn_wexpand(q, numerator->width) ||
      !bn_wexpand(r, divisor->width) ||
      !bn_wexpand(tmp, divisor->width)) {
    goto err;
  }

  OPENSSL_memset(q->d, 0, numerator->width * sizeof(BN_ULONG));
  q->width = numerator->width;
  q->neg = 0;

  OPENSSL_memset(r->d, 0, divisor->width * sizeof(BN_ULONG));
  r->width = divisor->width;
  r->neg = 0;

  // Incorporate |numerator| into |r|, one bit at a time, reducing after each
  // step. We maintain the invariant that |0 <= r < divisor| and
  // |q * divisor + r = n| where |n| is the portion of |numerator| incorporated
  // so far.
  //
  // First, we short-circuit the loop: if we know |divisor| has at least
  // |divisor_min_bits| bits, the top |divisor_min_bits - 1| can be incorporated
  // without reductions. This significantly speeds up |RSA_check_key|. For
  // simplicity, we round down to a whole number of words.
  declassify_assert(divisor_min_bits <= BN_num_bits(divisor));
  int initial_words = 0;
  if (divisor_min_bits > 0) {
    initial_words = (divisor_min_bits - 1) / BN_BITS2;
    if (initial_words > numerator->width) {
      initial_words = numerator->width;
    }
    OPENSSL_memcpy(r->d, numerator->d + numerator->width - initial_words,
                   initial_words * sizeof(BN_ULONG));
  }

  for (int i = numerator->width - initial_words - 1; i >= 0; i--) {
    for (int bit = BN_BITS2 - 1; bit >= 0; bit--) {
      // Incorporate the next bit of the numerator, by computing
      // r = 2*r or 2*r + 1. Note the result fits in one more word. We store the
      // extra word in |carry|.
      BN_ULONG carry = bn_add_words(r->d, r->d, r->d, divisor->width);
      r->d[0] |= (numerator->d[i] >> bit) & 1;
      // |r| was previously fully-reduced, so we know:
      //      2*0 <= r <= 2*(divisor-1) + 1
      //        0 <= r <= 2*divisor - 1 < 2*divisor.
      // Thus |r| satisfies the preconditions for |bn_reduce_once_in_place|.
      BN_ULONG subtracted = bn_reduce_once_in_place(r->d, carry, divisor->d,
                                                    tmp->d, divisor->width);
      // The corresponding bit of the quotient is set iff we needed to subtract.
      q->d[i] |= (~subtracted & 1) << bit;
    }
  }

  if ((quotient != NULL && !BN_copy(quotient, q)) ||
      (remainder != NULL && !BN_copy(remainder, r))) {
    goto err;
  }

  ret = 1;

err:
  BN_CTX_end(ctx);
  return ret;
}

static BIGNUM *bn_scratch_space_from_ctx(size_t width, BN_CTX *ctx) {
  BIGNUM *ret = BN_CTX_get(ctx);
  if (ret == NULL ||
      !bn_wexpand(ret, width)) {
    return NULL;
  }
  ret->neg = 0;
  ret->width = (int)width;
  return ret;
}

// bn_resized_from_ctx returns |bn| with width at least |width| or NULL on
// error. This is so it may be used with low-level "words" functions. If
// necessary, it allocates a new |BIGNUM| with a lifetime of the current scope
// in |ctx|, so the caller does not need to explicitly free it. |bn| must fit in
// |width| words.
static const BIGNUM *bn_resized_from_ctx(const BIGNUM *bn, size_t width,
                                         BN_CTX *ctx) {
  if ((size_t)bn->width >= width) {
    // Any excess words must be zero.
    assert(bn_fits_in_words(bn, width));
    return bn;
  }
  BIGNUM *ret = bn_scratch_space_from_ctx(width, ctx);
  if (ret == NULL ||
      !BN_copy(ret, bn) ||
      !bn_resize_words(ret, width)) {
    return NULL;
  }
  return ret;
}

int BN_mod_add(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, const BIGNUM *m,
               BN_CTX *ctx) {
  if (!BN_add(r, a, b)) {
    return 0;
  }
  return BN_nnmod(r, r, m, ctx);
}

int BN_mod_add_quick(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                     const BIGNUM *m) {
  BN_CTX *ctx = BN_CTX_new();
  int ok = ctx != NULL &&
           bn_mod_add_consttime(r, a, b, m, ctx);
  BN_CTX_free(ctx);
  return ok;
}

int bn_mod_add_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                         const BIGNUM *m, BN_CTX *ctx) {
  BN_CTX_start(ctx);
  a = bn_resized_from_ctx(a, m->width, ctx);
  b = bn_resized_from_ctx(b, m->width, ctx);
  BIGNUM *tmp = bn_scratch_space_from_ctx(m->width, ctx);
  int ok = a != NULL && b != NULL && tmp != NULL &&
           bn_wexpand(r, m->width);
  if (ok) {
    bn_mod_add_words(r->d, a->d, b->d, m->d, tmp->d, m->width);
    r->width = m->width;
    r->neg = 0;
  }
  BN_CTX_end(ctx);
  return ok;
}

int BN_mod_sub(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, const BIGNUM *m,
               BN_CTX *ctx) {
  if (!BN_sub(r, a, b)) {
    return 0;
  }
  return BN_nnmod(r, r, m, ctx);
}

int bn_mod_sub_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                         const BIGNUM *m, BN_CTX *ctx) {
  BN_CTX_start(ctx);
  a = bn_resized_from_ctx(a, m->width, ctx);
  b = bn_resized_from_ctx(b, m->width, ctx);
  BIGNUM *tmp = bn_scratch_space_from_ctx(m->width, ctx);
  int ok = a != NULL && b != NULL && tmp != NULL &&
           bn_wexpand(r, m->width);
  if (ok) {
    bn_mod_sub_words(r->d, a->d, b->d, m->d, tmp->d, m->width);
    r->width = m->width;
    r->neg = 0;
  }
  BN_CTX_end(ctx);
  return ok;
}

int BN_mod_sub_quick(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                     const BIGNUM *m) {
  BN_CTX *ctx = BN_CTX_new();
  int ok = ctx != NULL &&
           bn_mod_sub_consttime(r, a, b, m, ctx);
  BN_CTX_free(ctx);
  return ok;
}

int BN_mod_mul(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, const BIGNUM *m,
               BN_CTX *ctx) {
  BIGNUM *t;
  int ret = 0;

  BN_CTX_start(ctx);
  t = BN_CTX_get(ctx);
  if (t == NULL) {
    goto err;
  }

  if (a == b) {
    if (!BN_sqr(t, a, ctx)) {
      goto err;
    }
  } else {
    if (!BN_mul(t, a, b, ctx)) {
      goto err;
    }
  }

  if (!BN_nnmod(r, t, m, ctx)) {
    goto err;
  }

  ret = 1;

err:
  BN_CTX_end(ctx);
  return ret;
}

int BN_mod_sqr(BIGNUM *r, const BIGNUM *a, const BIGNUM *m, BN_CTX *ctx) {
  if (!BN_sqr(r, a, ctx)) {
    return 0;
  }

  // r->neg == 0,  thus we don't need BN_nnmod
  return BN_mod(r, r, m, ctx);
}

int BN_mod_lshift(BIGNUM *r, const BIGNUM *a, int n, const BIGNUM *m,
                  BN_CTX *ctx) {
  BIGNUM *abs_m = NULL;
  int ret;

  if (!BN_nnmod(r, a, m, ctx)) {
    return 0;
  }

  if (m->neg) {
    abs_m = BN_dup(m);
    if (abs_m == NULL) {
      return 0;
    }
    abs_m->neg = 0;
  }

  ret = bn_mod_lshift_consttime(r, r, n, (abs_m ? abs_m : m), ctx);

  BN_free(abs_m);
  return ret;
}

int bn_mod_lshift_consttime(BIGNUM *r, const BIGNUM *a, int n, const BIGNUM *m,
                            BN_CTX *ctx) {
  if (!BN_copy(r, a) ||
      !bn_resize_words(r, m->width)) {
    return 0;
  }

  BN_CTX_start(ctx);
  BIGNUM *tmp = bn_scratch_space_from_ctx(m->width, ctx);
  int ok = tmp != NULL;
  if (ok) {
    for (int i = 0; i < n; i++) {
      bn_mod_add_words(r->d, r->d, r->d, m->d, tmp->d, m->width);
    }
    r->neg = 0;
  }
  BN_CTX_end(ctx);
  return ok;
}

int BN_mod_lshift_quick(BIGNUM *r, const BIGNUM *a, int n, const BIGNUM *m) {
  BN_CTX *ctx = BN_CTX_new();
  int ok = ctx != NULL &&
           bn_mod_lshift_consttime(r, a, n, m, ctx);
  BN_CTX_free(ctx);
  return ok;
}

int BN_mod_lshift1(BIGNUM *r, const BIGNUM *a, const BIGNUM *m, BN_CTX *ctx) {
  if (!BN_lshift1(r, a)) {
    return 0;
  }

  return BN_nnmod(r, r, m, ctx);
}

int bn_mod_lshift1_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *m,
                             BN_CTX *ctx) {
  return bn_mod_add_consttime(r, a, a, m, ctx);
}

int BN_mod_lshift1_quick(BIGNUM *r, const BIGNUM *a, const BIGNUM *m) {
  BN_CTX *ctx = BN_CTX_new();
  int ok = ctx != NULL &&
           bn_mod_lshift1_consttime(r, a, m, ctx);
  BN_CTX_free(ctx);
  return ok;
}

BN_ULONG BN_div_word(BIGNUM *a, BN_ULONG w) {
  BN_ULONG ret = 0;
  int i, j;

  if (!w) {
    // actually this an error (division by zero)
    return (BN_ULONG) - 1;
  }

  if (a->width == 0) {
    return 0;
  }

  // normalize input for |bn_div_rem_words|.
  j = BN_BITS2 - BN_num_bits_word(w);
  w <<= j;
  if (!BN_lshift(a, a, j)) {
    return (BN_ULONG) - 1;
  }

  for (i = a->width - 1; i >= 0; i--) {
    BN_ULONG l = a->d[i];
    BN_ULONG d;
    BN_ULONG unused_rem;
    bn_div_rem_words(&d, &unused_rem, ret, l, w);
    ret = l - (d * w);
    a->d[i] = d;
  }

  bn_set_minimal_width(a);
  ret >>= j;
  return ret;
}

BN_ULONG BN_mod_word(const BIGNUM *a, BN_ULONG w) {
#ifndef BN_CAN_DIVIDE_ULLONG
  BN_ULONG ret = 0;
#else
  BN_ULLONG ret = 0;
#endif
  int i;

  if (w == 0) {
    return (BN_ULONG) -1;
  }

#ifndef BN_CAN_DIVIDE_ULLONG
  // If |w| is too long and we don't have |BN_ULLONG| division then we need to
  // fall back to using |BN_div_word|.
  if (w > ((BN_ULONG)1 << BN_BITS4)) {
    BIGNUM *tmp = BN_dup(a);
    if (tmp == NULL) {
      return (BN_ULONG)-1;
    }
    ret = BN_div_word(tmp, w);
    BN_free(tmp);
    return ret;
  }
#endif

  for (i = a->width - 1; i >= 0; i--) {
#ifndef BN_CAN_DIVIDE_ULLONG
    ret = ((ret << BN_BITS4) | ((a->d[i] >> BN_BITS4) & BN_MASK2l)) % w;
    ret = ((ret << BN_BITS4) | (a->d[i] & BN_MASK2l)) % w;
#else
    ret = (BN_ULLONG)(((ret << (BN_ULLONG)BN_BITS2) | a->d[i]) % (BN_ULLONG)w);
#endif
  }
  return (BN_ULONG)ret;
}

int BN_mod_pow2(BIGNUM *r, const BIGNUM *a, size_t e) {
  if (e == 0 || a->width == 0) {
    BN_zero(r);
    return 1;
  }

  size_t num_words = 1 + ((e - 1) / BN_BITS2);

  // If |a| definitely has less than |e| bits, just BN_copy.
  if ((size_t) a->width < num_words) {
    return BN_copy(r, a) != NULL;
  }

  // Otherwise, first make sure we have enough space in |r|.
  // Note that this will fail if num_words > INT_MAX.
  if (!bn_wexpand(r, num_words)) {
    return 0;
  }

  // Copy the content of |a| into |r|.
  OPENSSL_memcpy(r->d, a->d, num_words * sizeof(BN_ULONG));

  // If |e| isn't word-aligned, we have to mask off some of our bits.
  size_t top_word_exponent = e % (sizeof(BN_ULONG) * 8);
  if (top_word_exponent != 0) {
    r->d[num_words - 1] &= (((BN_ULONG) 1) << top_word_exponent) - 1;
  }

  // Fill in the remaining fields of |r|.
  r->neg = a->neg;
  r->width = (int) num_words;
  bn_set_minimal_width(r);
  return 1;
}

int BN_nnmod_pow2(BIGNUM *r, const BIGNUM *a, size_t e) {
  if (!BN_mod_pow2(r, a, e)) {
    return 0;
  }

  // If the returned value was non-negative, we're done.
  if (BN_is_zero(r) || !r->neg) {
    return 1;
  }

  size_t num_words = 1 + (e - 1) / BN_BITS2;

  // Expand |r| to the size of our modulus.
  if (!bn_wexpand(r, num_words)) {
    return 0;
  }

  // Clear the upper words of |r|.
  OPENSSL_memset(&r->d[r->width], 0, (num_words - r->width) * BN_BYTES);

  // Set parameters of |r|.
  r->neg = 0;
  r->width = (int) num_words;

  // Now, invert every word. The idea here is that we want to compute 2^e-|x|,
  // which is actually equivalent to the twos-complement representation of |x|
  // in |e| bits, which is -x = ~x + 1.
  for (int i = 0; i < r->width; i++) {
    r->d[i] = ~r->d[i];
  }

  // If our exponent doesn't span the top word, we have to mask the rest.
  size_t top_word_exponent = e % BN_BITS2;
  if (top_word_exponent != 0) {
    r->d[r->width - 1] &= (((BN_ULONG) 1) << top_word_exponent) - 1;
  }

  // Keep the minimal-width invariant for |BIGNUM|.
  bn_set_minimal_width(r);

  // Finally, add one, for the reason described above.
  return BN_add(r, r, BN_value_one());
}
