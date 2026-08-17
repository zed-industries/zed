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

#include <openssl/rsa.h>

#include <string.h>

#include <openssl/bn.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "internal.h"


#define BN_BLINDING_COUNTER 32

struct bn_blinding_st {
  BIGNUM *A;   // The base blinding factor, Montgomery-encoded.
  BIGNUM *Ai;  // The inverse of the blinding factor, Montgomery-encoded.
  unsigned counter;
};

static int bn_blinding_create_param(BN_BLINDING *b, const BIGNUM *e,
                                    const BN_MONT_CTX *mont, BN_CTX *ctx);

BN_BLINDING *BN_BLINDING_new(void) {
  BN_BLINDING *ret =
      reinterpret_cast<BN_BLINDING *>(OPENSSL_zalloc(sizeof(BN_BLINDING)));
  if (ret == NULL) {
    return NULL;
  }

  ret->A = BN_new();
  if (ret->A == NULL) {
    goto err;
  }

  ret->Ai = BN_new();
  if (ret->Ai == NULL) {
    goto err;
  }

  // The blinding values need to be created before this blinding can be used.
  ret->counter = BN_BLINDING_COUNTER - 1;

  return ret;

err:
  BN_BLINDING_free(ret);
  return NULL;
}

void BN_BLINDING_free(BN_BLINDING *r) {
  if (r == nullptr) {
    return;
  }
  BN_free(r->A);
  BN_free(r->Ai);
  OPENSSL_free(r);
}

void BN_BLINDING_invalidate(BN_BLINDING *b) {
  b->counter = BN_BLINDING_COUNTER - 1;
}

static int bn_blinding_update(BN_BLINDING *b, const BIGNUM *e,
                              const BN_MONT_CTX *mont, BN_CTX *ctx) {
  if (++b->counter == BN_BLINDING_COUNTER) {
    // re-create blinding parameters
    if (!bn_blinding_create_param(b, e, mont, ctx)) {
      goto err;
    }
    b->counter = 0;
  } else {
    if (!BN_mod_mul_montgomery(b->A, b->A, b->A, mont, ctx) ||
        !BN_mod_mul_montgomery(b->Ai, b->Ai, b->Ai, mont, ctx)) {
      goto err;
    }
  }

  return 1;

err:
  // |A| and |Ai| may be in an inconsistent state so they both need to be
  // replaced the next time this blinding is used. Note that this is only
  // sufficient because support for |BN_BLINDING_NO_UPDATE| and
  // |BN_BLINDING_NO_RECREATE| was previously dropped.
  b->counter = BN_BLINDING_COUNTER - 1;

  return 0;
}

int BN_BLINDING_convert(BIGNUM *n, BN_BLINDING *b, const BIGNUM *e,
                        const BN_MONT_CTX *mont, BN_CTX *ctx) {
  // |n| is not Montgomery-encoded and |b->A| is. |BN_mod_mul_montgomery|
  // cancels one Montgomery factor, so the resulting value of |n| is unencoded.
  if (!bn_blinding_update(b, e, mont, ctx) ||
      !BN_mod_mul_montgomery(n, n, b->A, mont, ctx)) {
    return 0;
  }

  return 1;
}

int BN_BLINDING_invert(BIGNUM *n, const BN_BLINDING *b, BN_MONT_CTX *mont,
                       BN_CTX *ctx) {
  // |n| is not Montgomery-encoded and |b->A| is. |BN_mod_mul_montgomery|
  // cancels one Montgomery factor, so the resulting value of |n| is unencoded.
  return BN_mod_mul_montgomery(n, n, b->Ai, mont, ctx);
}

static int bn_blinding_create_param(BN_BLINDING *b, const BIGNUM *e,
                                    const BN_MONT_CTX *mont, BN_CTX *ctx) {
  int no_inverse;
  if (!BN_rand_range_ex(b->A, 1, &mont->N) ||
      // Compute |b->A|^-1 in Montgomery form. Note |BN_from_montgomery| +
      // |BN_mod_inverse_blinded| is equivalent to, but more efficient than,
      // |BN_mod_inverse_blinded| + |BN_to_montgomery|.
      //
      // We do not retry if |b->A| has no inverse. Finding a non-invertible
      // value of |b->A| is equivalent to factoring |mont->N|. There is
      // negligible probability of stumbling on one at random.
      !BN_from_montgomery(b->Ai, b->A, mont, ctx) ||
      !BN_mod_inverse_blinded(b->Ai, &no_inverse, b->Ai, mont, ctx) ||
      // TODO(davidben): |BN_mod_exp_mont| internally computes the result in
      // Montgomery form. Save a pair of Montgomery reductions and a
      // multiplication by returning that value directly.
      !BN_mod_exp_mont(b->A, b->A, e, &mont->N, ctx, mont) ||
      !BN_to_montgomery(b->A, b->A, mont, ctx)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  return 1;
}
