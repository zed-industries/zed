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

#include <openssl/err.h>

#include "internal.h"


int BN_mod_inverse_odd(BIGNUM *out, int *out_no_inverse, const BIGNUM *a,
                       const BIGNUM *n, BN_CTX *ctx) {
  *out_no_inverse = 0;

  if (!BN_is_odd(n)) {
    OPENSSL_PUT_ERROR(BN, BN_R_CALLED_WITH_EVEN_MODULUS);
    return 0;
  }

  if (BN_is_negative(a) || BN_cmp(a, n) >= 0) {
    OPENSSL_PUT_ERROR(BN, BN_R_INPUT_NOT_REDUCED);
    return 0;
  }

  int sign;
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *A = BN_CTX_get(ctx);
  BIGNUM *B = BN_CTX_get(ctx);
  BIGNUM *X = BN_CTX_get(ctx);
  BIGNUM *Y = BN_CTX_get(ctx);
  BIGNUM *R = out;
  if (Y == NULL) {
    return 0;
  }

  BN_zero(Y);
  if (!BN_one(X) || BN_copy(B, a) == NULL || BN_copy(A, n) == NULL) {
    return 0;
  }
  A->neg = 0;
  sign = -1;
  // From  B = a mod |n|,  A = |n|  it follows that
  //
  //      0 <= B < A,
  //     -sign*X*a  ==  B   (mod |n|),
  //      sign*Y*a  ==  A   (mod |n|).

  // Binary inversion algorithm; requires odd modulus. This is faster than the
  // general algorithm if the modulus is sufficiently small (about 400 .. 500
  // bits on 32-bit systems, but much more on 64-bit systems)
  int shift;

  while (!BN_is_zero(B)) {
    //      0 < B < |n|,
    //      0 < A <= |n|,
    // (1) -sign*X*a  ==  B   (mod |n|),
    // (2)  sign*Y*a  ==  A   (mod |n|)

    // Now divide  B  by the maximum possible power of two in the integers,
    // and divide  X  by the same value mod |n|.
    // When we're done, (1) still holds.
    shift = 0;
    while (!BN_is_bit_set(B, shift)) {
      // note that 0 < B
      shift++;

      if (BN_is_odd(X)) {
        if (!BN_uadd(X, X, n)) {
          return 0;
        }
      }
      // now X is even, so we can easily divide it by two
      if (!BN_rshift1(X, X)) {
        return 0;
      }
    }
    if (shift > 0) {
      if (!BN_rshift(B, B, shift)) {
        return 0;
      }
    }

    // Same for A and Y. Afterwards, (2) still holds.
    shift = 0;
    while (!BN_is_bit_set(A, shift)) {
      // note that 0 < A
      shift++;

      if (BN_is_odd(Y)) {
        if (!BN_uadd(Y, Y, n)) {
          return 0;
        }
      }
      // now Y is even
      if (!BN_rshift1(Y, Y)) {
        return 0;
      }
    }
    if (shift > 0) {
      if (!BN_rshift(A, A, shift)) {
        return 0;
      }
    }

    // We still have (1) and (2).
    // Both  A  and  B  are odd.
    // The following computations ensure that
    //
    //     0 <= B < |n|,
    //      0 < A < |n|,
    // (1) -sign*X*a  ==  B   (mod |n|),
    // (2)  sign*Y*a  ==  A   (mod |n|),
    //
    // and that either  A  or  B  is even in the next iteration.
    if (BN_ucmp(B, A) >= 0) {
      // -sign*(X + Y)*a == B - A  (mod |n|)
      if (!BN_uadd(X, X, Y)) {
        return 0;
      }
      // NB: we could use BN_mod_add_quick(X, X, Y, n), but that
      // actually makes the algorithm slower
      if (!BN_usub(B, B, A)) {
        return 0;
      }
    } else {
      //  sign*(X + Y)*a == A - B  (mod |n|)
      if (!BN_uadd(Y, Y, X)) {
        return 0;
      }
      // as above, BN_mod_add_quick(Y, Y, X, n) would slow things down
      if (!BN_usub(A, A, B)) {
        return 0;
      }
    }
  }

  if (!BN_is_one(A)) {
    *out_no_inverse = 1;
    OPENSSL_PUT_ERROR(BN, BN_R_NO_INVERSE);
    return 0;
  }

  // The while loop (Euclid's algorithm) ends when
  //      A == gcd(a,n);
  // we have
  //       sign*Y*a  ==  A  (mod |n|),
  // where  Y  is non-negative.

  if (sign < 0) {
    if (!BN_sub(Y, n, Y)) {
      return 0;
    }
  }
  // Now  Y*a  ==  A  (mod |n|).

  // Y*a == 1  (mod |n|)
  if (Y->neg || BN_ucmp(Y, n) >= 0) {
    if (!BN_nnmod(Y, Y, n, ctx)) {
      return 0;
    }
  }
  if (!BN_copy(R, Y)) {
    return 0;
  }

  return 1;
}

BIGNUM *BN_mod_inverse(BIGNUM *out, const BIGNUM *a, const BIGNUM *n,
                       BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> new_out;
  if (out == nullptr) {
    new_out.reset(BN_new());
    if (new_out == nullptr) {
      return nullptr;
    }
    out = new_out.get();
  }

  bssl::UniquePtr<BIGNUM> a_reduced;
  if (a->neg || BN_ucmp(a, n) >= 0) {
    a_reduced.reset(BN_dup(a));
    if (a_reduced == nullptr) {
      return nullptr;
    }
    if (!BN_nnmod(a_reduced.get(), a_reduced.get(), n, ctx)) {
      return nullptr;
    }
    a = a_reduced.get();
  }

  int no_inverse;
  if (!BN_is_odd(n)) {
    if (!bn_mod_inverse_consttime(out, &no_inverse, a, n, ctx)) {
      return nullptr;
    }
  } else if (!BN_mod_inverse_odd(out, &no_inverse, a, n, ctx)) {
    return nullptr;
  }

  new_out.release();  // Passed to the caller via |out|.
  return out;
}

int BN_mod_inverse_blinded(BIGNUM *out, int *out_no_inverse, const BIGNUM *a,
                           const BN_MONT_CTX *mont, BN_CTX *ctx) {
  *out_no_inverse = 0;

  // |a| is secret, but it is required to be in range, so these comparisons may
  // be leaked.
  if (BN_is_negative(a) ||
      constant_time_declassify_int(BN_cmp(a, &mont->N) >= 0)) {
    OPENSSL_PUT_ERROR(BN, BN_R_INPUT_NOT_REDUCED);
    return 0;
  }

  bssl::UniquePtr<BIGNUM> blinding_factor(BN_new());
  if (blinding_factor == nullptr) {
    return 0;
  }

  // |BN_mod_inverse_odd| is leaky, so generate a secret blinding factor and
  // blind |a|. This works because (ar)^-1 * r = a^-1, supposing r is
  // invertible. If r is not invertible, this function will fail. However, we
  // only use this in RSA, where stumbling on an uninvertible element means
  // stumbling on the key's factorization. That is, if this function fails, the
  // RSA key was not actually a product of two large primes.
  //
  // TODO(crbug.com/boringssl/677): When the PRNG output is marked secret by
  // default, the explicit |bn_secret| call can be removed.
  if (!BN_rand_range_ex(blinding_factor.get(), 1, &mont->N)) {
    return 0;
  }
  bn_secret(blinding_factor.get());
  if (!BN_mod_mul_montgomery(out, blinding_factor.get(), a, mont, ctx)) {
    return 0;
  }

  // Once blinded, |out| is no longer secret, so it may be passed to a leaky
  // mod inverse function. Note |blinding_factor| is secret, so |out| will be
  // secret again after multiplying.
  bn_declassify(out);
  if (!BN_mod_inverse_odd(out, out_no_inverse, out, &mont->N, ctx) ||
      !BN_mod_mul_montgomery(out, blinding_factor.get(), out, mont, ctx)) {
    return 0;
  }

  return 1;
}

int bn_mod_inverse_prime(BIGNUM *out, const BIGNUM *a, const BIGNUM *p,
                         BN_CTX *ctx, const BN_MONT_CTX *mont_p) {
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *p_minus_2 = BN_CTX_get(ctx);
  return p_minus_2 != nullptr && BN_copy(p_minus_2, p) &&
         BN_sub_word(p_minus_2, 2) &&
         BN_mod_exp_mont(out, a, p_minus_2, p, ctx, mont_p);
}

int bn_mod_inverse_secret_prime(BIGNUM *out, const BIGNUM *a, const BIGNUM *p,
                                BN_CTX *ctx, const BN_MONT_CTX *mont_p) {
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *p_minus_2 = BN_CTX_get(ctx);
  return p_minus_2 != nullptr && BN_copy(p_minus_2, p) &&
         BN_sub_word(p_minus_2, 2) &&
         BN_mod_exp_mont_consttime(out, a, p_minus_2, p, ctx, mont_p);
}
