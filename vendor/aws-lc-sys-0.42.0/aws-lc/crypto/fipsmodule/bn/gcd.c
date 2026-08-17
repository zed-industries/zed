// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2001 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

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

  BIGNUM *A, *B, *X, *Y;
  int ret = 0;
  int sign;

  BN_CTX_start(ctx);
  A = BN_CTX_get(ctx);
  B = BN_CTX_get(ctx);
  X = BN_CTX_get(ctx);
  Y = BN_CTX_get(ctx);
  if (Y == NULL) {
    goto err;
  }

  BIGNUM *R = out;

  BN_zero(Y);
  if (!BN_one(X) || BN_copy(B, a) == NULL || BN_copy(A, n) == NULL) {
    goto err;
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
          goto err;
        }
      }
      // now X is even, so we can easily divide it by two
      if (!BN_rshift1(X, X)) {
        goto err;
      }
    }
    if (shift > 0) {
      if (!BN_rshift(B, B, shift)) {
        goto err;
      }
    }

    // Same for A and Y. Afterwards, (2) still holds.
    shift = 0;
    while (!BN_is_bit_set(A, shift)) {
      // note that 0 < A
      shift++;

      if (BN_is_odd(Y)) {
        if (!BN_uadd(Y, Y, n)) {
          goto err;
        }
      }
      // now Y is even
      if (!BN_rshift1(Y, Y)) {
        goto err;
      }
    }
    if (shift > 0) {
      if (!BN_rshift(A, A, shift)) {
        goto err;
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
        goto err;
      }
      // NB: we could use BN_mod_add_quick(X, X, Y, n), but that
      // actually makes the algorithm slower
      if (!BN_usub(B, B, A)) {
        goto err;
      }
    } else {
      //  sign*(X + Y)*a == A - B  (mod |n|)
      if (!BN_uadd(Y, Y, X)) {
        goto err;
      }
      // as above, BN_mod_add_quick(Y, Y, X, n) would slow things down
      if (!BN_usub(A, A, B)) {
        goto err;
      }
    }
  }

  if (!BN_is_one(A)) {
    *out_no_inverse = 1;
    OPENSSL_PUT_ERROR(BN, BN_R_NO_INVERSE);
    goto err;
  }

  // The while loop (Euclid's algorithm) ends when
  //      A == gcd(a,n);
  // we have
  //       sign*Y*a  ==  A  (mod |n|),
  // where  Y  is non-negative.

  if (sign < 0) {
    if (!BN_sub(Y, n, Y)) {
      goto err;
    }
  }
  // Now  Y*a  ==  A  (mod |n|).

  // Y*a == 1  (mod |n|)
  if (Y->neg || BN_ucmp(Y, n) >= 0) {
    if (!BN_nnmod(Y, Y, n, ctx)) {
      goto err;
    }
  }
  if (!BN_copy(R, Y)) {
    goto err;
  }

  ret = 1;

err:
  BN_CTX_end(ctx);
  return ret;
}

BIGNUM *BN_mod_inverse(BIGNUM *out, const BIGNUM *a, const BIGNUM *n,
                       BN_CTX *ctx) {
  BIGNUM *new_out = NULL;
  if (out == NULL) {
    new_out = BN_new();
    if (new_out == NULL) {
      return NULL;
    }
    out = new_out;
  }

  int ok = 0;
  BIGNUM *a_reduced = NULL;
  if (a->neg || BN_ucmp(a, n) >= 0) {
    a_reduced = BN_dup(a);
    if (a_reduced == NULL) {
      goto err;
    }
    if (!BN_nnmod(a_reduced, a_reduced, n, ctx)) {
      goto err;
    }
    a = a_reduced;
  }

  int no_inverse;
  if (!BN_is_odd(n)) {
    if (!bn_mod_inverse_consttime(out, &no_inverse, a, n, ctx)) {
      goto err;
    }
  } else if (!BN_mod_inverse_odd(out, &no_inverse, a, n, ctx)) {
    goto err;
  }

  ok = 1;

err:
  if (!ok) {
    BN_free(new_out);
    out = NULL;
  }
  BN_free(a_reduced);
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

  int ret = 0;
  BIGNUM blinding_factor;
  BN_init(&blinding_factor);

  // |BN_mod_inverse_odd| is leaky, so generate a secret blinding factor and
  // blind |a|. This works because (ar)^-1 * r = a^-1, supposing r is
  // invertible. If r is not invertible, this function will fail. However, we
  // only use this in RSA, where stumbling on an uninvertible element means
  // stumbling on the key's factorization. That is, if this function fails, the
  // RSA key was not actually a product of two large primes.
  //
  // TODO(crbug.com/boringssl/677): When the PRNG output is marked secret by
  // default, the explicit |bn_secret| call can be removed.
  if (!BN_rand_range_ex(&blinding_factor, 1, &mont->N)) {
    goto err;
  }
  bn_secret(&blinding_factor);
  if (!BN_mod_mul_montgomery(out, &blinding_factor, a, mont, ctx)) {
    goto err;
  }

  // Once blinded, |out| is no longer secret, so it may be passed to a leaky
  // mod inverse function. Note |blinding_factor| is secret, so |out| will be
  // secret again after multiplying.
  bn_declassify(out);
  if (!BN_mod_inverse_odd(out, out_no_inverse, out, &mont->N, ctx) ||
      !BN_mod_mul_montgomery(out, &blinding_factor, out, mont, ctx)) {
    goto err;
  }

  ret = 1;

err:
  BN_free(&blinding_factor);
  return ret;
}

int bn_mod_inverse_prime(BIGNUM *out, const BIGNUM *a, const BIGNUM *p,
                         BN_CTX *ctx, const BN_MONT_CTX *mont_p) {
  BN_CTX_start(ctx);
  BIGNUM *p_minus_2 = BN_CTX_get(ctx);
  int ok = p_minus_2 != NULL &&
           BN_copy(p_minus_2, p) &&
           BN_sub_word(p_minus_2, 2) &&
           BN_mod_exp_mont(out, a, p_minus_2, p, ctx, mont_p);
  BN_CTX_end(ctx);
  return ok;
}

int bn_mod_inverse_secret_prime(BIGNUM *out, const BIGNUM *a, const BIGNUM *p,
                                BN_CTX *ctx, const BN_MONT_CTX *mont_p) {
  BN_CTX_start(ctx);
  BIGNUM *p_minus_2 = BN_CTX_get(ctx);
  int ok = p_minus_2 != NULL &&
           BN_copy(p_minus_2, p) &&
           BN_sub_word(p_minus_2, 2) &&
           BN_mod_exp_mont_consttime(out, a, p_minus_2, p, ctx, mont_p);
  BN_CTX_end(ctx);
  return ok;
}
