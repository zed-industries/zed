// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <openssl/bn.h>

#include <assert.h>
#include <limits.h>

#include <openssl/err.h>

#include "internal.h"

#if (defined(OPENSSL_X86) || defined(OPENSSL_X86_64)) && defined(_MSC_VER) && \
    !defined(__clang__)
#define HAVE_MSVC_DIV_INTRINSICS
#include <immintrin.h>
#if defined(OPENSSL_X86)
#pragma intrinsic(_udiv64)
#else
#pragma intrinsic(_udiv128)
#endif
#endif


// bn_div_words divides a double-width |h|,|l| by |d| and returns the result,
// which must fit in a |BN_ULONG|, i.e. |h < d|.
static inline BN_ULONG bn_div_words(BN_ULONG h, BN_ULONG l, BN_ULONG d) {
  assert(h < d);
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

// bn_div_rem_words divides a double-width numerator (high half |nh| and low
// half |nl|) with a single-width divisor. It sets |*quotient_out| and
// |*rem_out| to be the quotient and numerator, respectively. The quotient must
// fit in a |BN_ULONG|, i.e. |nh < d|.
static inline void bn_div_rem_words(BN_ULONG *quotient_out, BN_ULONG *rem_out,
                                    BN_ULONG nh, BN_ULONG nl, BN_ULONG d) {
  assert(nh < d);
  // This operation is the x86 and x86_64 DIV instruction, but it is difficult
  // for the compiler to emit it. Dividing a |BN_ULLONG| by a |BN_ULONG| does
  // not work because, a priori, the quotient may not fit in |BN_ULONG| and DIV
  // will trap on overflow, not truncate. The compiler will instead emit a call
  // to a more expensive support function (e.g. |__udivdi3|). Thus we use inline
  // assembly or intrinsics to get the instruction.
  //
  // These is specific to x86 and x86_64; Arm and RISC-V do not have double-wide
  // division instructions.
#if defined(BN_CAN_USE_INLINE_ASM) && defined(OPENSSL_X86)
  __asm__ volatile("divl %4"
                   : "=a"(*quotient_out), "=d"(*rem_out)
                   : "a"(nl), "d"(nh), "rm"(d)
                   : "cc");
#elif defined(BN_CAN_USE_INLINE_ASM) && defined(OPENSSL_X86_64)
  __asm__ volatile("divq %4"
                   : "=a"(*quotient_out), "=d"(*rem_out)
                   : "a"(nl), "d"(nh), "rm"(d)
                   : "cc");
#elif defined(HAVE_MSVC_DIV_INTRINSICS) && defined(OPENSSL_X86)
  BN_ULLONG n = (((BN_ULLONG)nh) << BN_BITS2) | nl;
  unsigned rem;
  *quotient_out = _udiv64(n, d, &rem);
  *rem_out = rem;
#elif defined(HAVE_MSVC_DIV_INTRINSICS) && defined(OPENSSL_X86_64)
  unsigned __int64 rem;
  *quotient_out = _udiv128(nh, nl, d, &rem);
  *rem_out = rem;
#else
#if defined(BN_CAN_DIVIDE_ULLONG)
  BN_ULLONG n = (((BN_ULLONG)nh) << BN_BITS2) | nl;
  *quotient_out = (BN_ULONG)(n / d);
#else
  *quotient_out = bn_div_words(nh, nl, d);
#endif  // BN_CAN_DIVIDE_ULLONG
  *rem_out = nl - (*quotient_out * d);
#endif
}

int BN_div(BIGNUM *quotient, BIGNUM *rem, const BIGNUM *numerator,
           const BIGNUM *divisor, BN_CTX *ctx) {
  // This function implements long division, per Knuth, The Art of Computer
  // Programming, Volume 2, Chapter 4.3.1, Algorithm D. This algorithm only
  // divides non-negative integers, but we round towards zero, so we divide
  // absolute values and adjust the signs separately.
  //
  // Inputs to this function are assumed public and may be leaked by timing and
  // cache side channels. Division with secret inputs should use other
  // implementation strategies such as Montgomery reduction.
  if (BN_is_zero(divisor)) {
    OPENSSL_PUT_ERROR(BN, BN_R_DIV_BY_ZERO);
    return 0;
  }

  bssl::BN_CTXScope scope(ctx);
  BIGNUM *tmp = BN_CTX_get(ctx);
  BIGNUM *snum = BN_CTX_get(ctx);
  BIGNUM *sdiv = BN_CTX_get(ctx);
  BIGNUM *res = quotient == NULL ? BN_CTX_get(ctx) : quotient;
  int norm_shift, num_n, loop, div_n;
  BN_ULONG d0, d1;
  if (tmp == NULL || snum == NULL || sdiv == NULL || res == NULL) {
    return 0;
  }

  // Knuth step D1: Normalise the numbers such that the divisor's MSB is set.
  // This ensures, in Knuth's terminology, that v1 >= b/2, needed for the
  // quotient estimation step.
  norm_shift = BN_BITS2 - (BN_num_bits(divisor) % BN_BITS2);
  if (!BN_lshift(sdiv, divisor, norm_shift) ||
      !BN_lshift(snum, numerator, norm_shift)) {
    return 0;
  }

  // This algorithm relies on |sdiv| being minimal width. We do not use this
  // function on secret inputs, so leaking this is fine. Also minimize |snum| to
  // avoid looping on leading zeros, as we're not trying to be leak-free.
  bn_set_minimal_width(sdiv);
  bn_set_minimal_width(snum);
  div_n = sdiv->width;
  d0 = sdiv->d[div_n - 1];
  d1 = (div_n == 1) ? 0 : sdiv->d[div_n - 2];
  assert(d0 & (((BN_ULONG)1) << (BN_BITS2 - 1)));

  // Extend |snum| with zeros to satisfy the long division invariants:
  // - |snum| must have at least |div_n| + 1 words.
  // - |snum|'s most significant word must be zero to guarantee the first loop
  //   iteration works with a prefix greater than |sdiv|. (This is the extra u0
  //   digit in Knuth step D1.)
  num_n = snum->width <= div_n ? div_n + 1 : snum->width + 1;
  if (!bn_resize_words(snum, num_n)) {
    return 0;
  }

  // Knuth step D2: The quotient's width is the difference between numerator and
  // denominator. Also set up its sign and size a temporary for the loop.
  loop = num_n - div_n;
  res->neg = snum->neg ^ sdiv->neg;
  if (!bn_wexpand(res, loop) ||  //
      !bn_wexpand(tmp, div_n + 1)) {
    return 0;
  }
  res->width = loop;

  // Knuth steps D2 through D7: Compute the quotient with a word-by-word long
  // division. Note that Knuth indexes words from most to least significant, so
  // our index is reversed. Each loop iteration computes res->d[i] of the
  // quotient and updates snum with the running remainder. Before each loop
  // iteration, the div_n words beginning at snum->d[i+1] must be less than
  // snum.
  for (int i = loop - 1; i >= 0; i--) {
    // The next word of the quotient, q, is floor(wnum / sdiv), where wnum is
    // the div_n + 1 words beginning at snum->d[i]. i starts at
    // num_n - div_n - 1, so there are at least div_n + 1 words available.
    //
    // Knuth step D3: Compute q', an estimate of q by looking at the top words
    // of wnum and sdiv. We must estimate such that q' = q or q' = q + 1.
    BN_ULONG q, rm = 0;
    BN_ULONG *wnum = snum->d + i;
    BN_ULONG n0 = wnum[div_n];
    BN_ULONG n1 = wnum[div_n - 1];
    if (n0 == d0) {
      // Estimate q' = b - 1, where b is the base.
      q = BN_MASK2;
      // Knuth also runs the fixup routine in this case, but this would require
      // computing rm and is unnecessary. q' is already close enough. That is,
      // the true quotient, q is either b - 1 or b - 2.
      //
      // By the loop invariant, q <= b - 1, so we must show that q >= b - 2. We
      // do this by showing wnum / sdiv >= b - 2. Suppose wnum / sdiv < b - 2.
      // wnum and sdiv have the same most significant word, so:
      //
      //    wnum >= n0 * b^div_n
      //    sdiv <  (n0 + 1) * b^(d_div - 1)
      //
      // Thus:
      //
      //    b - 2 > wnum / sdiv
      //          > (n0 * b^div_n) / (n0 + 1) * b^(div_n - 1)
      //          = (n0 * b) / (n0 + 1)
      //
      //         (n0 + 1) * (b - 2) > n0 * b
      //    n0 * b + b - 2 * n0 - 2 > n0 * b
      //                      b - 2 > 2 * n0
      //                    b/2 - 1 > n0
      //
      // This contradicts the normalization condition, so q >= b - 2 and our
      // estimate is close enough.
    } else {
      // Estimate q' = floor(n0n1 / d0). Per Theorem B, q' - 2 <= q <= q', which
      // is slightly outside of our bounds.
      assert(n0 < d0);
      bn_div_rem_words(&q, &rm, n0, n1, d0);

      // Fix the estimate by examining one more word and adjusting q' as needed.
      // This is the second half of step D3 and is sufficient per exercises 19,
      // 20, and 21. Although only one iteration is needed to correct q + 2 to
      // q + 1, Knuth uses a loop. A loop will often also correct q + 1 to q,
      // saving the slightly more expensive underflow handling below.
      if (div_n > 1) {
        BN_ULONG n2 = wnum[div_n - 2];
#ifdef BN_ULLONG
        BN_ULLONG t2 = (BN_ULLONG)d1 * q;
        for (;;) {
          if (t2 <= ((((BN_ULLONG)rm) << BN_BITS2) | n2)) {
            break;
          }
          q--;
          rm += d0;
          if (rm < d0) {
            // If rm overflows, the true value exceeds BN_ULONG and the next
            // t2 comparison should exit the loop.
            break;
          }
          t2 -= d1;
        }
#else   // !BN_ULLONG
        BN_ULONG t2l, t2h;
        BN_UMULT_LOHI(t2l, t2h, d1, q);
        for (;;) {
          if (t2h < rm || (t2h == rm && t2l <= n2)) {
            break;
          }
          q--;
          rm += d0;
          if (rm < d0) {
            // If rm overflows, the true value exceeds BN_ULONG and the next
            // t2 comparison should exit the loop.
            break;
          }
          if (t2l < d1) {
            t2h--;
          }
          t2l -= d1;
        }
#endif  // !BN_ULLONG
      }
    }

    // Knuth step D4 through D6: Now q' = q or q' = q + 1, and
    // -sdiv < wnum - sdiv * q < sdiv. If q' = q + 1, the subtraction will
    // underflow, and we fix it up below.
    tmp->d[div_n] = bn_mul_words(tmp->d, sdiv->d, div_n, q);
    if (bn_sub_words(wnum, wnum, tmp->d, div_n + 1)) {
      q--;
      // The final addition is expected to overflow, canceling the underflow.
      wnum[div_n] += bn_add_words(wnum, wnum, sdiv->d, div_n);
    }

    // q is now correct, and wnum has been updated to the running remainder.
    res->d[i] = q;
  }

  // Trim leading zeros and correct any negative zeros.
  bn_set_minimal_width(snum);
  bn_set_minimal_width(res);

  // Knuth step D8: Unnormalize. snum now contains the remainder.
  if (rem != NULL && !BN_rshift(rem, snum, norm_shift)) {
    return 0;
  }

  return 1;
}

int BN_nnmod(BIGNUM *r, const BIGNUM *m, const BIGNUM *d, BN_CTX *ctx) {
  if (!(BN_mod(r, m, d, ctx))) {
    return 0;
  }
  if (!r->neg) {
    return 1;
  }

  // now -d < r < 0, so we have to set r := r + d. Ignoring the sign bits, this
  // is r = d - r.
  return BN_usub(r, d, r);
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

  bssl::BN_CTXScope scope(ctx);
  BIGNUM *q = quotient, *r = remainder;
  if (quotient == NULL || quotient == numerator || quotient == divisor) {
    q = BN_CTX_get(ctx);
  }
  if (remainder == NULL || remainder == numerator || remainder == divisor) {
    r = BN_CTX_get(ctx);
  }
  BIGNUM *tmp = BN_CTX_get(ctx);
  int initial_words;
  if (q == NULL || r == NULL || tmp == NULL ||
      !bn_wexpand(q, numerator->width) || !bn_wexpand(r, divisor->width) ||
      !bn_wexpand(tmp, divisor->width)) {
    return 0;
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
  initial_words = 0;
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
    return 0;
  }

  return 1;
}

static BIGNUM *bn_scratch_space_from_ctx(size_t width, BN_CTX *ctx) {
  BIGNUM *ret = BN_CTX_get(ctx);
  if (ret == NULL || !bn_wexpand(ret, width)) {
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
  if (ret == NULL || !BN_copy(ret, bn) || !bn_resize_words(ret, width)) {
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
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  return ctx != nullptr && bn_mod_add_consttime(r, a, b, m, ctx.get());
}

int bn_mod_add_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                         const BIGNUM *m, BN_CTX *ctx) {
  bssl::BN_CTXScope scope(ctx);
  a = bn_resized_from_ctx(a, m->width, ctx);
  b = bn_resized_from_ctx(b, m->width, ctx);
  BIGNUM *tmp = bn_scratch_space_from_ctx(m->width, ctx);
  if (a == nullptr || b == nullptr || tmp == nullptr ||
      !bn_wexpand(r, m->width)) {
    return 0;
  }
  bn_mod_add_words(r->d, a->d, b->d, m->d, tmp->d, m->width);
  r->width = m->width;
  r->neg = 0;
  return 1;
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
  bssl::BN_CTXScope scope(ctx);
  a = bn_resized_from_ctx(a, m->width, ctx);
  b = bn_resized_from_ctx(b, m->width, ctx);
  BIGNUM *tmp = bn_scratch_space_from_ctx(m->width, ctx);
  if (a == nullptr || b == nullptr || tmp == nullptr ||
      !bn_wexpand(r, m->width)) {
    return 0;
  }
  bn_mod_sub_words(r->d, a->d, b->d, m->d, tmp->d, m->width);
  r->width = m->width;
  r->neg = 0;
  return 1;
}

int BN_mod_sub_quick(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                     const BIGNUM *m) {
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  return ctx != nullptr && bn_mod_sub_consttime(r, a, b, m, ctx.get());
}

int BN_mod_mul(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, const BIGNUM *m,
               BN_CTX *ctx) {
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *t = BN_CTX_get(ctx);
  if (t == NULL) {
    return 0;
  }

  if (a == b) {
    if (!BN_sqr(t, a, ctx)) {
      return 0;
    }
  } else {
    if (!BN_mul(t, a, b, ctx)) {
      return 0;
    }
  }

  if (!BN_nnmod(r, t, m, ctx)) {
    return 0;
  }

  return 1;
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
  if (!BN_nnmod(r, a, m, ctx)) {
    return 0;
  }

  bssl::UniquePtr<BIGNUM> abs_m;
  if (m->neg) {
    abs_m.reset(BN_dup(m));
    if (abs_m == nullptr) {
      return 0;
    }
    abs_m->neg = 0;
  }

  return bn_mod_lshift_consttime(r, r, n, (abs_m ? abs_m.get() : m), ctx);
}

int bn_mod_lshift_consttime(BIGNUM *r, const BIGNUM *a, int n, const BIGNUM *m,
                            BN_CTX *ctx) {
  if (!BN_copy(r, a) || !bn_resize_words(r, m->width)) {
    return 0;
  }

  bssl::BN_CTXScope scope(ctx);
  BIGNUM *tmp = bn_scratch_space_from_ctx(m->width, ctx);
  if (tmp == nullptr) {
    return 0;
  }
  for (int i = 0; i < n; i++) {
    bn_mod_add_words(r->d, r->d, r->d, m->d, tmp->d, m->width);
  }
  r->neg = 0;
  return 1;
}

int BN_mod_lshift_quick(BIGNUM *r, const BIGNUM *a, int n, const BIGNUM *m) {
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  return ctx != nullptr && bn_mod_lshift_consttime(r, a, n, m, ctx.get());
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
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  return ctx != nullptr && bn_mod_lshift1_consttime(r, a, m, ctx.get());
}

BN_ULONG BN_div_word(BIGNUM *a, BN_ULONG w) {
  BN_ULONG ret = 0;
  int i, j;

  if (!w) {
    // actually this an error (division by zero)
    return (BN_ULONG)-1;
  }

  if (a->width == 0) {
    return 0;
  }

  // normalize input for |bn_div_rem_words|.
  j = BN_BITS2 - BN_num_bits_word(w);
  w <<= j;
  if (!BN_lshift(a, a, j)) {
    return (BN_ULONG)-1;
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
    return (BN_ULONG)-1;
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
