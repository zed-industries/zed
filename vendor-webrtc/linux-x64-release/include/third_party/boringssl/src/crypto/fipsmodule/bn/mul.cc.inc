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
#include <stdlib.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "internal.h"


#define BN_MUL_RECURSIVE_SIZE_NORMAL 16
#define BN_SQR_RECURSIVE_SIZE_NORMAL BN_MUL_RECURSIVE_SIZE_NORMAL


static void bn_abs_sub_words(BN_ULONG *r, const BN_ULONG *a, const BN_ULONG *b,
                             size_t num, BN_ULONG *tmp) {
  BN_ULONG borrow = bn_sub_words(tmp, a, b, num);
  bn_sub_words(r, b, a, num);
  bn_select_words(r, 0 - borrow, r /* tmp < 0 */, tmp /* tmp >= 0 */, num);
}

static void bn_mul_normal(BN_ULONG *r, const BN_ULONG *a, size_t na,
                          const BN_ULONG *b, size_t nb) {
  if (na < nb) {
    size_t itmp = na;
    na = nb;
    nb = itmp;
    const BN_ULONG *ltmp = a;
    a = b;
    b = ltmp;
  }
  BN_ULONG *rr = &(r[na]);
  if (nb == 0) {
    OPENSSL_memset(r, 0, na * sizeof(BN_ULONG));
    return;
  }
  rr[0] = bn_mul_words(r, a, na, b[0]);

  for (;;) {
    if (--nb == 0) {
      return;
    }
    rr[1] = bn_mul_add_words(&(r[1]), a, na, b[1]);
    if (--nb == 0) {
      return;
    }
    rr[2] = bn_mul_add_words(&(r[2]), a, na, b[2]);
    if (--nb == 0) {
      return;
    }
    rr[3] = bn_mul_add_words(&(r[3]), a, na, b[3]);
    if (--nb == 0) {
      return;
    }
    rr[4] = bn_mul_add_words(&(r[4]), a, na, b[4]);
    rr += 4;
    r += 4;
    b += 4;
  }
}

// bn_sub_part_words sets |r| to |a| - |b|. It returns the borrow bit, which is
// one if the operation underflowed and zero otherwise. |cl| is the common
// length, that is, the shorter of len(a) or len(b). |dl| is the delta length,
// that is, len(a) - len(b). |r|'s length matches the larger of |a| and |b|, or
// cl + abs(dl).
//
// TODO(davidben): Make this take |size_t|. The |cl| + |dl| calling convention
// is confusing.
static BN_ULONG bn_sub_part_words(BN_ULONG *r, const BN_ULONG *a,
                                  const BN_ULONG *b, int cl, int dl) {
  assert(cl >= 0);
  BN_ULONG borrow = bn_sub_words(r, a, b, cl);
  if (dl == 0) {
    return borrow;
  }

  r += cl;
  a += cl;
  b += cl;

  if (dl < 0) {
    // |a| is shorter than |b|. Complete the subtraction as if the excess words
    // in |a| were zeros.
    dl = -dl;
    for (int i = 0; i < dl; i++) {
      r[i] = CRYPTO_subc_w(0, b[i], borrow, &borrow);
    }
  } else {
    // |b| is shorter than |a|. Complete the subtraction as if the excess words
    // in |b| were zeros.
    for (int i = 0; i < dl; i++) {
      r[i] = CRYPTO_subc_w(a[i], 0, borrow, &borrow);
    }
  }

  return borrow;
}

// bn_abs_sub_part_words computes |r| = |a| - |b|, storing the absolute value
// and returning a mask of all ones if the result was negative and all zeros if
// the result was positive. |cl| and |dl| follow the |bn_sub_part_words| calling
// convention.
//
// TODO(davidben): Make this take |size_t|. The |cl| + |dl| calling convention
// is confusing.
static BN_ULONG bn_abs_sub_part_words(BN_ULONG *r, const BN_ULONG *a,
                                      const BN_ULONG *b, int cl, int dl,
                                      BN_ULONG *tmp) {
  BN_ULONG borrow = bn_sub_part_words(tmp, a, b, cl, dl);
  bn_sub_part_words(r, b, a, cl, -dl);
  int r_len = cl + (dl < 0 ? -dl : dl);
  borrow = 0 - borrow;
  bn_select_words(r, borrow, r /* tmp < 0 */, tmp /* tmp >= 0 */, r_len);
  return borrow;
}

int bn_abs_sub_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                         BN_CTX *ctx) {
  int cl = a->width < b->width ? a->width : b->width;
  int dl = a->width - b->width;
  int r_len = a->width < b->width ? b->width : a->width;
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *tmp = BN_CTX_get(ctx);
  if (tmp == nullptr || !bn_wexpand(r, r_len) || !bn_wexpand(tmp, r_len)) {
    return 0;
  }
  bn_abs_sub_part_words(r->d, a->d, b->d, cl, dl, tmp->d);
  r->width = r_len;
  return 1;
}

// Karatsuba recursive multiplication algorithm
// (cf. Knuth, The Art of Computer Programming, Vol. 2)

// bn_mul_recursive sets |r| to |a| * |b|, using |t| as scratch space. |r| has
// length 2*|n2|, |a| has length |n2| + |dna|, |b| has length |n2| + |dnb|, and
// |t| has length 4*|n2|. |n2| must be a power of two. Finally, we must have
// -|BN_MUL_RECURSIVE_SIZE_NORMAL|/2 <= |dna| <= 0 and
// -|BN_MUL_RECURSIVE_SIZE_NORMAL|/2 <= |dnb| <= 0.
//
// TODO(davidben): Simplify and |size_t| the calling convention around lengths
// here.
static void bn_mul_recursive(BN_ULONG *r, const BN_ULONG *a, const BN_ULONG *b,
                             int n2, int dna, int dnb, BN_ULONG *t) {
  // |n2| is a power of two.
  assert(n2 != 0 && (n2 & (n2 - 1)) == 0);
  // Check |dna| and |dnb| are in range.
  assert(-BN_MUL_RECURSIVE_SIZE_NORMAL / 2 <= dna && dna <= 0);
  assert(-BN_MUL_RECURSIVE_SIZE_NORMAL / 2 <= dnb && dnb <= 0);

  // Only call bn_mul_comba 8 if n2 == 8 and the
  // two arrays are complete [steve]
  if (n2 == 8 && dna == 0 && dnb == 0) {
    bn_mul_comba8(r, a, b);
    return;
  }

  // Else do normal multiply
  if (n2 < BN_MUL_RECURSIVE_SIZE_NORMAL) {
    bn_mul_normal(r, a, n2 + dna, b, n2 + dnb);
    if (dna + dnb < 0) {
      OPENSSL_memset(&r[2 * n2 + dna + dnb], 0,
                     sizeof(BN_ULONG) * -(dna + dnb));
    }
    return;
  }

  // Split |a| and |b| into a0,a1 and b0,b1, where a0 and b0 have size |n|.
  // Split |t| into t0,t1,t2,t3, each of size |n|, with the remaining 4*|n| used
  // for recursive calls.
  // Split |r| into r0,r1,r2,r3. We must contribute a0*b0 to r0,r1, a0*a1+b0*b1
  // to r1,r2, and a1*b1 to r2,r3. The middle term we will compute as:
  //
  //   a0*a1 + b0*b1 = (a0 - a1)*(b1 - b0) + a1*b1 + a0*b0
  //
  // Note that we know |n| >= |BN_MUL_RECURSIVE_SIZE_NORMAL|/2 above, so
  // |tna| and |tnb| are non-negative.
  int n = n2 / 2, tna = n + dna, tnb = n + dnb;

  // t0 = a0 - a1 and t1 = b1 - b0. The result will be multiplied, so we XOR
  // their sign masks, giving the sign of (a0 - a1)*(b1 - b0). t0 and t1
  // themselves store the absolute value.
  BN_ULONG neg = bn_abs_sub_part_words(t, a, &a[n], tna, n - tna, &t[n2]);
  neg ^= bn_abs_sub_part_words(&t[n], &b[n], b, tnb, tnb - n, &t[n2]);

  // Compute:
  // t2,t3 = t0 * t1 = |(a0 - a1)*(b1 - b0)|
  // r0,r1 = a0 * b0
  // r2,r3 = a1 * b1
  if (n == 4 && dna == 0 && dnb == 0) {
    bn_mul_comba4(&t[n2], t, &t[n]);

    bn_mul_comba4(r, a, b);
    bn_mul_comba4(&r[n2], &a[n], &b[n]);
  } else if (n == 8 && dna == 0 && dnb == 0) {
    bn_mul_comba8(&t[n2], t, &t[n]);

    bn_mul_comba8(r, a, b);
    bn_mul_comba8(&r[n2], &a[n], &b[n]);
  } else {
    BN_ULONG *p = &t[n2 * 2];
    bn_mul_recursive(&t[n2], t, &t[n], n, 0, 0, p);
    bn_mul_recursive(r, a, b, n, 0, 0, p);
    bn_mul_recursive(&r[n2], &a[n], &b[n], n, dna, dnb, p);
  }

  // t0,t1,c = r0,r1 + r2,r3 = a0*b0 + a1*b1
  BN_ULONG c = bn_add_words(t, r, &r[n2], n2);

  // t2,t3,c = t0,t1,c + neg*t2,t3 = (a0 - a1)*(b1 - b0) + a1*b1 + a0*b0.
  // The second term is stored as the absolute value, so we do this with a
  // constant-time select.
  BN_ULONG c_neg = c - bn_sub_words(&t[n2 * 2], t, &t[n2], n2);
  BN_ULONG c_pos = c + bn_add_words(&t[n2], t, &t[n2], n2);
  bn_select_words(&t[n2], neg, &t[n2 * 2], &t[n2], n2);
  static_assert(sizeof(BN_ULONG) <= sizeof(crypto_word_t),
                "crypto_word_t is too small");
  c = constant_time_select_w(neg, c_neg, c_pos);

  // We now have our three components. Add them together.
  // r1,r2,c = r1,r2 + t2,t3,c
  c += bn_add_words(&r[n], &r[n], &t[n2], n2);

  // Propagate the carry bit to the end.
  for (int i = n + n2; i < n2 + n2; i++) {
    BN_ULONG old = r[i];
    r[i] = old + c;
    c = r[i] < old;
  }

  // The product should fit without carries.
  declassify_assert(c == 0);
}

// bn_mul_part_recursive sets |r| to |a| * |b|, using |t| as scratch space. |r|
// has length 4*|n|, |a| has length |n| + |tna|, |b| has length |n| + |tnb|, and
// |t| has length 8*|n|. |n| must be a power of two. Additionally, we must have
// 0 <= tna < n and 0 <= tnb < n, and |tna| and |tnb| must differ by at most
// one.
//
// TODO(davidben): Make this take |size_t| and perhaps the actual lengths of |a|
// and |b|.
static void bn_mul_part_recursive(BN_ULONG *r, const BN_ULONG *a,
                                  const BN_ULONG *b, int n, int tna, int tnb,
                                  BN_ULONG *t) {
  // |n| is a power of two.
  assert(n != 0 && (n & (n - 1)) == 0);
  // Check |tna| and |tnb| are in range.
  assert(0 <= tna && tna < n);
  assert(0 <= tnb && tnb < n);
  assert(-1 <= tna - tnb && tna - tnb <= 1);

  int n2 = n * 2;
  if (n < 8) {
    bn_mul_normal(r, a, n + tna, b, n + tnb);
    OPENSSL_memset(r + n2 + tna + tnb, 0, n2 - tna - tnb);
    return;
  }

  // Split |a| and |b| into a0,a1 and b0,b1, where a0 and b0 have size |n|. |a1|
  // and |b1| have size |tna| and |tnb|, respectively.
  // Split |t| into t0,t1,t2,t3, each of size |n|, with the remaining 4*|n| used
  // for recursive calls.
  // Split |r| into r0,r1,r2,r3. We must contribute a0*b0 to r0,r1, a0*a1+b0*b1
  // to r1,r2, and a1*b1 to r2,r3. The middle term we will compute as:
  //
  //   a0*a1 + b0*b1 = (a0 - a1)*(b1 - b0) + a1*b1 + a0*b0

  // t0 = a0 - a1 and t1 = b1 - b0. The result will be multiplied, so we XOR
  // their sign masks, giving the sign of (a0 - a1)*(b1 - b0). t0 and t1
  // themselves store the absolute value.
  BN_ULONG neg = bn_abs_sub_part_words(t, a, &a[n], tna, n - tna, &t[n2]);
  neg ^= bn_abs_sub_part_words(&t[n], &b[n], b, tnb, tnb - n, &t[n2]);

  // Compute:
  // t2,t3 = t0 * t1 = |(a0 - a1)*(b1 - b0)|
  // r0,r1 = a0 * b0
  // r2,r3 = a1 * b1
  if (n == 8) {
    bn_mul_comba8(&t[n2], t, &t[n]);
    bn_mul_comba8(r, a, b);

    bn_mul_normal(&r[n2], &a[n], tna, &b[n], tnb);
    // |bn_mul_normal| only writes |tna| + |tna| words. Zero the rest.
    OPENSSL_memset(&r[n2 + tna + tnb], 0, sizeof(BN_ULONG) * (n2 - tna - tnb));
  } else {
    BN_ULONG *p = &t[n2 * 2];
    bn_mul_recursive(&t[n2], t, &t[n], n, 0, 0, p);
    bn_mul_recursive(r, a, b, n, 0, 0, p);

    OPENSSL_memset(&r[n2], 0, sizeof(BN_ULONG) * n2);
    if (tna < BN_MUL_RECURSIVE_SIZE_NORMAL &&
        tnb < BN_MUL_RECURSIVE_SIZE_NORMAL) {
      bn_mul_normal(&r[n2], &a[n], tna, &b[n], tnb);
    } else {
      int i = n;
      for (;;) {
        i /= 2;
        if (i < tna || i < tnb) {
          // E.g., n == 16, i == 8 and tna == 11. |tna| and |tnb| are within one
          // of each other, so if |tna| is larger and tna > i, then we know
          // tnb >= i, and this call is valid.
          bn_mul_part_recursive(&r[n2], &a[n], &b[n], i, tna - i, tnb - i, p);
          break;
        }
        if (i == tna || i == tnb) {
          // If there is only a bottom half to the number, just do it. We know
          // the larger of |tna - i| and |tnb - i| is zero. The other is zero or
          // -1 by because of |tna| and |tnb| differ by at most one.
          bn_mul_recursive(&r[n2], &a[n], &b[n], i, tna - i, tnb - i, p);
          break;
        }

        // This loop will eventually terminate when |i| falls below
        // |BN_MUL_RECURSIVE_SIZE_NORMAL| because we know one of |tna| and |tnb|
        // exceeds that.
      }
    }
  }

  // t0,t1,c = r0,r1 + r2,r3 = a0*b0 + a1*b1
  BN_ULONG c = bn_add_words(t, r, &r[n2], n2);

  // t2,t3,c = t0,t1,c + neg*t2,t3 = (a0 - a1)*(b1 - b0) + a1*b1 + a0*b0.
  // The second term is stored as the absolute value, so we do this with a
  // constant-time select.
  BN_ULONG c_neg = c - bn_sub_words(&t[n2 * 2], t, &t[n2], n2);
  BN_ULONG c_pos = c + bn_add_words(&t[n2], t, &t[n2], n2);
  bn_select_words(&t[n2], neg, &t[n2 * 2], &t[n2], n2);
  static_assert(sizeof(BN_ULONG) <= sizeof(crypto_word_t),
                "crypto_word_t is too small");
  c = constant_time_select_w(neg, c_neg, c_pos);

  // We now have our three components. Add them together.
  // r1,r2,c = r1,r2 + t2,t3,c
  c += bn_add_words(&r[n], &r[n], &t[n2], n2);

  // Propagate the carry bit to the end.
  for (int i = n + n2; i < n2 + n2; i++) {
    BN_ULONG old = r[i];
    r[i] = old + c;
    c = r[i] < old;
  }

  // The product should fit without carries.
  declassify_assert(c == 0);
}

// bn_mul_impl implements |BN_mul| and |bn_mul_consttime|. Note this function
// breaks |BIGNUM| invariants and may return a negative zero. This is handled by
// the callers.
static int bn_mul_impl(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
                       BN_CTX *ctx) {
  int al = a->width;
  int bl = b->width;
  if (al == 0 || bl == 0) {
    BN_zero(r);
    return 1;
  }

  int i, top;
  BIGNUM *rr;
  bssl::BN_CTXScope scope(ctx);
  if (r == a || r == b) {
    rr = BN_CTX_get(ctx);
    if (rr == NULL) {
      return 0;
    }
  } else {
    rr = r;
  }
  rr->neg = a->neg ^ b->neg;

  i = al - bl;
  if (i == 0) {
    if (al == 8) {
      if (!bn_wexpand(rr, 16)) {
        return 0;
      }
      rr->width = 16;
      bn_mul_comba8(rr->d, a->d, b->d);
      goto end;
    }
  }

  top = al + bl;
  // TODO(crbug.com/406497222): The recursive implementation is actually worse
  // for cryptographic use cases, but we need to retain it in |BN_mul| for the
  // projects misusing BIGNUM as a general-purpose calculator library with
  // giant integers. Disconnect this code from our cryptographic primitives.
  static const int kMulNormalSize = 16;
  if (al >= kMulNormalSize && bl >= kMulNormalSize) {
    if (-1 <= i && i <= 1) {
      // Find the largest power of two less than or equal to the larger length.
      int j;
      if (i >= 0) {
        j = BN_num_bits_word((BN_ULONG)al);
      } else {
        j = BN_num_bits_word((BN_ULONG)bl);
      }
      j = 1 << (j - 1);
      assert(j <= al || j <= bl);
      BIGNUM *t = BN_CTX_get(ctx);
      if (t == NULL) {
        return 0;
      }
      if (al > j || bl > j) {
        // We know |al| and |bl| are at most one from each other, so if al > j,
        // bl >= j, and vice versa. Thus we can use |bn_mul_part_recursive|.
        //
        // TODO(davidben): This codepath is almost unused in standard
        // algorithms. Is this optimization necessary? See notes in
        // https://boringssl-review.googlesource.com/q/I0bd604e2cd6a75c266f64476c23a730ca1721ea6
        assert(al >= j && bl >= j);
        if (!bn_wexpand(t, j * 8) || !bn_wexpand(rr, j * 4)) {
          return 0;
        }
        bn_mul_part_recursive(rr->d, a->d, b->d, j, al - j, bl - j, t->d);
      } else {
        // al <= j && bl <= j. Additionally, we know j <= al or j <= bl, so one
        // of al - j or bl - j is zero. The other, by the bound on |i| above, is
        // zero or -1. Thus, we can use |bn_mul_recursive|.
        if (!bn_wexpand(t, j * 4) || !bn_wexpand(rr, j * 2)) {
          return 0;
        }
        bn_mul_recursive(rr->d, a->d, b->d, j, al - j, bl - j, t->d);
      }
      rr->width = top;
      goto end;
    }
  }

  if (!bn_wexpand(rr, top)) {
    return 0;
  }
  rr->width = top;
  bn_mul_normal(rr->d, a->d, al, b->d, bl);

end:
  if (r != rr && !BN_copy(r, rr)) {
    return 0;
  }
  return 1;
}

int BN_mul(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, BN_CTX *ctx) {
  if (!bn_mul_impl(r, a, b, ctx)) {
    return 0;
  }

  // This additionally fixes any negative zeros created by |bn_mul_impl|.
  bn_set_minimal_width(r);
  return 1;
}

int bn_mul_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, BN_CTX *ctx) {
  // Prevent negative zeros.
  if (a->neg || b->neg) {
    OPENSSL_PUT_ERROR(BN, BN_R_NEGATIVE_NUMBER);
    return 0;
  }

  return bn_mul_impl(r, a, b, ctx);
}

void bn_mul_small(BN_ULONG *r, size_t num_r, const BN_ULONG *a, size_t num_a,
                  const BN_ULONG *b, size_t num_b) {
  if (num_r != num_a + num_b) {
    abort();
  }
  // TODO(davidben): Should this call |bn_mul_comba4| too? |BN_mul| does not
  // hit that code.
  if (num_a == 8 && num_b == 8) {
    bn_mul_comba8(r, a, b);
  } else {
    bn_mul_normal(r, a, num_a, b, num_b);
  }
}

// tmp must have 2*n words
static void bn_sqr_normal(BN_ULONG *r, const BN_ULONG *a, size_t n,
                          BN_ULONG *tmp) {
  if (n == 0) {
    return;
  }

  size_t max = n * 2;
  const BN_ULONG *ap = a;
  BN_ULONG *rp = r;
  rp[0] = rp[max - 1] = 0;
  rp++;

  // Compute the contribution of a[i] * a[j] for all i < j.
  if (n > 1) {
    ap++;
    rp[n - 1] = bn_mul_words(rp, ap, n - 1, ap[-1]);
    rp += 2;
  }
  if (n > 2) {
    for (size_t i = n - 2; i > 0; i--) {
      ap++;
      rp[i] = bn_mul_add_words(rp, ap, i, ap[-1]);
      rp += 2;
    }
  }

  // The final result fits in |max| words, so none of the following operations
  // will overflow.

  // Double |r|, giving the contribution of a[i] * a[j] for all i != j.
  bn_add_words(r, r, r, max);

  // Add in the contribution of a[i] * a[i] for all i.
  bn_sqr_words(tmp, a, n);
  bn_add_words(r, r, tmp, max);
}

// bn_sqr_recursive sets |r| to |a|^2, using |t| as scratch space. |r| has
// length 2*|n2|, |a| has length |n2|, and |t| has length 4*|n2|. |n2| must be
// a power of two.
static void bn_sqr_recursive(BN_ULONG *r, const BN_ULONG *a, size_t n2,
                             BN_ULONG *t) {
  // |n2| is a power of two.
  assert(n2 != 0 && (n2 & (n2 - 1)) == 0);

  if (n2 == 4) {
    bn_sqr_comba4(r, a);
    return;
  }
  if (n2 == 8) {
    bn_sqr_comba8(r, a);
    return;
  }
  if (n2 < BN_SQR_RECURSIVE_SIZE_NORMAL) {
    bn_sqr_normal(r, a, n2, t);
    return;
  }

  // Split |a| into a0,a1, each of size |n|.
  // Split |t| into t0,t1,t2,t3, each of size |n|, with the remaining 4*|n| used
  // for recursive calls.
  // Split |r| into r0,r1,r2,r3. We must contribute a0^2 to r0,r1, 2*a0*a1 to
  // r1,r2, and a1^2 to r2,r3.
  size_t n = n2 / 2;
  BN_ULONG *t_recursive = &t[n2 * 2];

  // t0 = |a0 - a1|.
  bn_abs_sub_words(t, a, &a[n], n, &t[n]);
  // t2,t3 = t0^2 = |a0 - a1|^2 = a0^2 - 2*a0*a1 + a1^2
  bn_sqr_recursive(&t[n2], t, n, t_recursive);

  // r0,r1 = a0^2
  bn_sqr_recursive(r, a, n, t_recursive);

  // r2,r3 = a1^2
  bn_sqr_recursive(&r[n2], &a[n], n, t_recursive);

  // t0,t1,c = r0,r1 + r2,r3 = a0^2 + a1^2
  BN_ULONG c = bn_add_words(t, r, &r[n2], n2);
  // t2,t3,c = t0,t1,c - t2,t3 = 2*a0*a1
  c -= bn_sub_words(&t[n2], t, &t[n2], n2);

  // We now have our three components. Add them together.
  // r1,r2,c = r1,r2 + t2,t3,c
  c += bn_add_words(&r[n], &r[n], &t[n2], n2);

  // Propagate the carry bit to the end.
  for (size_t i = n + n2; i < n2 + n2; i++) {
    BN_ULONG old = r[i];
    r[i] = old + c;
    c = r[i] < old;
  }

  // The square should fit without carries.
  assert(c == 0);
}

int BN_mul_word(BIGNUM *bn, BN_ULONG w) {
  if (!bn->width) {
    return 1;
  }

  if (w == 0) {
    BN_zero(bn);
    return 1;
  }

  BN_ULONG ll = bn_mul_words(bn->d, bn->d, bn->width, w);
  if (ll) {
    if (!bn_wexpand(bn, bn->width + 1)) {
      return 0;
    }
    bn->d[bn->width++] = ll;
  }

  return 1;
}

int bn_sqr_consttime(BIGNUM *r, const BIGNUM *a, BN_CTX *ctx) {
  int al = a->width;
  if (al <= 0) {
    r->width = 0;
    r->neg = 0;
    return 1;
  }

  bssl::BN_CTXScope scope(ctx);
  BIGNUM *rr = (a != r) ? r : BN_CTX_get(ctx);
  BIGNUM *tmp = BN_CTX_get(ctx);
  if (!rr || !tmp) {
    return 0;
  }

  int max = 2 * al;  // Non-zero (from above)
  if (!bn_wexpand(rr, max)) {
    return 0;
  }

  if (al == 4) {
    bn_sqr_comba4(rr->d, a->d);
  } else if (al == 8) {
    bn_sqr_comba8(rr->d, a->d);
  } else {
    if (al < BN_SQR_RECURSIVE_SIZE_NORMAL) {
      BN_ULONG t[BN_SQR_RECURSIVE_SIZE_NORMAL * 2];
      bn_sqr_normal(rr->d, a->d, al, t);
    } else {
      // If |al| is a power of two, we can use |bn_sqr_recursive|.
      //
      // TODO(crbug.com/406497222): The recursive implementation is actually
      // worse for cryptographic use cases, but we need to retain it in |BN_mul|
      // for the projects misusing BIGNUM as a general-purpose calculator
      // library with giant integers. Disconnect this code from our
      // cryptographic primitives.
      if (al != 0 && (al & (al - 1)) == 0) {
        if (!bn_wexpand(tmp, al * 4)) {
          return 0;
        }
        bn_sqr_recursive(rr->d, a->d, al, tmp->d);
      } else {
        if (!bn_wexpand(tmp, max)) {
          return 0;
        }
        bn_sqr_normal(rr->d, a->d, al, tmp->d);
      }
    }
  }

  rr->neg = 0;
  rr->width = max;

  if (rr != r && !BN_copy(r, rr)) {
    return 0;
  }
  return 1;
}

int BN_sqr(BIGNUM *r, const BIGNUM *a, BN_CTX *ctx) {
  if (!bn_sqr_consttime(r, a, ctx)) {
    return 0;
  }

  bn_set_minimal_width(r);
  return 1;
}

void bn_sqr_small(BN_ULONG *r, size_t num_r, const BN_ULONG *a, size_t num_a) {
  if (num_r != 2 * num_a || num_a > BN_SMALL_MAX_WORDS) {
    abort();
  }
  if (num_a == 4) {
    bn_sqr_comba4(r, a);
  } else if (num_a == 8) {
    bn_sqr_comba8(r, a);
  } else {
    BN_ULONG tmp[2 * BN_SMALL_MAX_WORDS];
    bn_sqr_normal(r, a, num_a, tmp);
    OPENSSL_cleanse(tmp, 2 * num_a * sizeof(BN_ULONG));
  }
}
