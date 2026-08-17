// Copyright 2016 Brian Smith.
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

#include "internal.h"
#include "../../internal.h"


static uint64_t bn_neg_inv_mod_r_u64(uint64_t n);

static_assert(BN_MONT_CTX_N0_LIMBS == 1 || BN_MONT_CTX_N0_LIMBS == 2,
              "BN_MONT_CTX_N0_LIMBS value is invalid");
static_assert(sizeof(BN_ULONG) * BN_MONT_CTX_N0_LIMBS == sizeof(uint64_t),
              "uint64_t is insufficient precision for n0");

// LG_LITTLE_R is log_2(r).
#define LG_LITTLE_R (BN_MONT_CTX_N0_LIMBS * BN_BITS2)

uint64_t bn_mont_n0(const BIGNUM *n) {
  // These conditions are checked by the caller, |BN_MONT_CTX_set| or
  // |BN_MONT_CTX_new_consttime|.
  assert(!BN_is_zero(n));
  assert(!BN_is_negative(n));
  assert(BN_is_odd(n));

  // r == 2**(BN_MONT_CTX_N0_LIMBS * BN_BITS2) and LG_LITTLE_R == lg(r). This
  // ensures that we can do integer division by |r| by simply ignoring
  // |BN_MONT_CTX_N0_LIMBS| limbs. Similarly, we can calculate values modulo
  // |r| by just looking at the lowest |BN_MONT_CTX_N0_LIMBS| limbs. This is
  // what makes Montgomery multiplication efficient.
  //
  // As shown in Algorithm 1 of "Fast Prime Field Elliptic Curve Cryptography
  // with 256 Bit Primes" by Shay Gueron and Vlad Krasnov, in the loop of a
  // multi-limb Montgomery multiplication of |a * b (mod n)|, given the
  // unreduced product |t == a * b|, we repeatedly calculate:
  //
  //    t1 := t % r         |t1| is |t|'s lowest limb (see previous paragraph).
  //    t2 := t1*n0*n
  //    t3 := t + t2
  //    t := t3 / r         copy all limbs of |t3| except the lowest to |t|.
  //
  // In the last step, it would only make sense to ignore the lowest limb of
  // |t3| if it were zero. The middle steps ensure that this is the case:
  //
  //                            t3 ==  0 (mod r)
  //                        t + t2 ==  0 (mod r)
  //                   t + t1*n0*n ==  0 (mod r)
  //                       t1*n0*n == -t (mod r)
  //                        t*n0*n == -t (mod r)
  //                          n0*n == -1 (mod r)
  //                            n0 == -1/n (mod r)
  //
  // Thus, in each iteration of the loop, we multiply by the constant factor
  // |n0|, the negative inverse of n (mod r).

  // n_mod_r = n % r. As explained above, this is done by taking the lowest
  // |BN_MONT_CTX_N0_LIMBS| limbs of |n|.
  uint64_t n_mod_r = n->d[0];
#if BN_MONT_CTX_N0_LIMBS == 2
  if (n->width > 1) {
    n_mod_r |= (uint64_t)n->d[1] << BN_BITS2;
  }
#endif

  return bn_neg_inv_mod_r_u64(n_mod_r);
}

// bn_neg_inv_r_mod_n_u64 calculates the -1/n mod r; i.e. it calculates |v|
// such that u*r - v*n == 1. |r| is the constant defined in |bn_mont_n0|. |n|
// must be odd.
//
// This is derived from |xbinGCD| in Henry S. Warren, Jr.'s "Montgomery
// Multiplication" (http://www.hackersdelight.org/MontgomeryMultiplication.pdf).
// It is very similar to the MODULAR-INVERSE function in Stephen R. Dussé's and
// Burton S. Kaliski Jr.'s "A Cryptographic Library for the Motorola DSP56000"
// (http://link.springer.com/chapter/10.1007%2F3-540-46877-3_21).
//
// This is inspired by Joppe W. Bos's "Constant Time Modular Inversion"
// (http://www.joppebos.com/files/CTInversion.pdf) so that the inversion is
// constant-time with respect to |n|. We assume uint64_t additions,
// subtractions, shifts, and bitwise operations are all constant time, which
// may be a large leap of faith on 32-bit targets. We avoid division and
// multiplication, which tend to be the most problematic in terms of timing
// leaks.
//
// Most GCD implementations return values such that |u*r + v*n == 1|, so the
// caller would have to negate the resultant |v| for the purpose of Montgomery
// multiplication. This implementation does the negation implicitly by doing
// the computations as a difference instead of a sum.
static uint64_t bn_neg_inv_mod_r_u64(uint64_t n) {
  assert(n % 2 == 1);

  // alpha == 2**(lg r - 1) == r / 2.
  static const uint64_t alpha = UINT64_C(1) << (LG_LITTLE_R - 1);

  const uint64_t beta = n;

  uint64_t u = 1;
  uint64_t v = 0;

  // The invariant maintained from here on is:
  // 2**(lg r - i) == u*2*alpha - v*beta.
  for (size_t i = 0; i < LG_LITTLE_R; ++i) {
#if BN_BITS2 == 64 && defined(BN_ULLONG)
    assert((BN_ULLONG)(1) << (LG_LITTLE_R - i) ==
           ((BN_ULLONG)u * 2 * alpha) - ((BN_ULLONG)v * beta));
#endif

    // Delete a common factor of 2 in u and v if |u| is even. Otherwise, set
    // |u = (u + beta) / 2| and |v = (v / 2) + alpha|.

    uint64_t u_is_odd = UINT64_C(0) - (u & 1);  // Either 0xff..ff or 0.

    // The addition can overflow, so use Dietz's method for it.
    //
    // Dietz calculates (x+y)/2 by (x⊕y)>>1 + x&y. This is valid for all
    // (unsigned) x and y, even when x+y overflows. Evidence for 32-bit values
    // (embedded in 64 bits to so that overflow can be ignored):
    //
    // (declare-fun x () (_ BitVec 64))
    // (declare-fun y () (_ BitVec 64))
    // (assert (let (
    //    (one (_ bv1 64))
    //    (thirtyTwo (_ bv32 64)))
    //    (and
    //      (bvult x (bvshl one thirtyTwo))
    //      (bvult y (bvshl one thirtyTwo))
    //      (not (=
    //        (bvadd (bvlshr (bvxor x y) one) (bvand x y))
    //        (bvlshr (bvadd x y) one)))
    // )))
    // (check-sat)
    uint64_t beta_if_u_is_odd = beta & u_is_odd;  // Either |beta| or 0.
    u = ((u ^ beta_if_u_is_odd) >> 1) + (u & beta_if_u_is_odd);

    uint64_t alpha_if_u_is_odd = alpha & u_is_odd;  // Either |alpha| or 0.
    v = (v >> 1) + alpha_if_u_is_odd;
  }

  // The invariant now shows that u*r - v*n == 1 since r == 2 * alpha.
#if BN_BITS2 == 64 && defined(BN_ULLONG)
  declassify_assert(1 == ((BN_ULLONG)u * 2 * alpha) - ((BN_ULLONG)v * beta));
#endif

  return v;
}

int bn_mont_ctx_set_RR_consttime(BN_MONT_CTX *mont, BN_CTX *ctx) {
  assert(!BN_is_zero(&mont->N));
  assert(!BN_is_negative(&mont->N));
  assert(BN_is_odd(&mont->N));
  assert(bn_minimal_width(&mont->N) == mont->N.width);

  unsigned n_bits = BN_num_bits(&mont->N);
  assert(n_bits != 0);
  if (n_bits == 1) {
    BN_zero(&mont->RR);
    return bn_resize_words(&mont->RR, mont->N.width);
  }

  unsigned lgBigR = mont->N.width * BN_BITS2;
  assert(lgBigR >= n_bits);

  // RR is R, or 2^lgBigR, in the Montgomery domain. We can compute 2 in the
  // Montgomery domain, 2R or 2^(lgBigR+1), and then use Montgomery
  // square-and-multiply to exponentiate.
  //
  // The square steps take 2^n R to (2^n)*(2^n) R = 2^2n R. This is the same as
  // doubling 2^n R, n times (doubling any x, n times, computes 2^n * x). When n
  // is below some threshold, doubling is faster; when above, squaring is
  // faster. From benchmarking various 32-bit and 64-bit architectures, the word
  // count seems to work well as a threshold. (Doubling scales linearly and
  // Montgomery reduction scales quadratically, so the threshold should scale
  // roughly linearly.)
  //
  // The multiply steps take 2^n R to 2*2^n R = 2^(n+1) R. It is faster to
  // double the value instead, so the square-and-multiply exponentiation would
  // become square-and-double. However, when using the word count as the
  // threshold, it turns out that no multiply/double steps will be needed at
  // all, because squaring any x, i times, computes x^(2^i):
  //
  //   (2^threshold)^(2^BN_BITS2_LG) R
  //   (2^mont->N.width)^BN_BITS2 R
  // = 2^(mont->N.width*BN_BITS2) R
  // = 2^lgBigR R
  // = RR
  int threshold = mont->N.width;

  // Calculate 2^threshold R = 2^(threshold + lgBigR) by doubling. The
  // first n_bits - 1 doubles can be skipped because we don't need to reduce.
  if (!BN_set_bit(&mont->RR, n_bits - 1) ||
      !bn_mod_lshift_consttime(&mont->RR, &mont->RR,
                               threshold + (lgBigR - (n_bits - 1)),
                               &mont->N, ctx)) {
    return 0;
  }

  // The above steps are the same regardless of the threshold. The steps below
  // need to be modified if the threshold changes.
  assert(threshold == mont->N.width);
  for (unsigned i = 0; i < BN_BITS2_LG; i++) {
    if (!BN_mod_mul_montgomery(&mont->RR, &mont->RR, &mont->RR, mont, ctx)) {
      return 0;
    }
  }

  return bn_resize_words(&mont->RR, mont->N.width);
}
