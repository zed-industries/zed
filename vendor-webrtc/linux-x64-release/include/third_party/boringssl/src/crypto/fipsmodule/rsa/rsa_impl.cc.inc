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

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/bn.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/thread.h>

#include "../../bcm_support.h"
#include "../../internal.h"
#include "../bn/internal.h"
#include "../delocate.h"
#include "../service_indicator/internal.h"
#include "internal.h"


static_assert(OPENSSL_RSA_MAX_MODULUS_BITS <=
                  BN_MONTGOMERY_MAX_WORDS * BN_BITS2,
              "Max RSA size too big for Montgomery arithmetic");

int rsa_check_public_key(const RSA *rsa) {
  if (rsa->n == NULL) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_VALUE_MISSING);
    return 0;
  }

  unsigned n_bits = BN_num_bits(rsa->n);
  if (n_bits > OPENSSL_RSA_MAX_MODULUS_BITS) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_MODULUS_TOO_LARGE);
    return 0;
  }

  // TODO(crbug.com/boringssl/607): Raise this limit. 512-bit RSA was factored
  // in 1999.
  if (n_bits < 512) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_KEY_SIZE_TOO_SMALL);
    return 0;
  }

  // RSA moduli must be positive and odd. In addition to being necessary for RSA
  // in general, we cannot setup Montgomery reduction with even moduli.
  if (!BN_is_odd(rsa->n) || BN_is_negative(rsa->n)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_RSA_PARAMETERS);
    return 0;
  }

  static const unsigned kMaxExponentBits = 33;
  if (rsa->e != NULL) {
    // Reject e = 1, negative e, and even e. e must be odd to be relatively
    // prime with phi(n).
    unsigned e_bits = BN_num_bits(rsa->e);
    if (e_bits < 2 || BN_is_negative(rsa->e) || !BN_is_odd(rsa->e)) {
      OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_E_VALUE);
      return 0;
    }
    if (rsa->flags & RSA_FLAG_LARGE_PUBLIC_EXPONENT) {
      // The caller has requested disabling DoS protections. Still, e must be
      // less than n.
      if (BN_ucmp(rsa->n, rsa->e) <= 0) {
        OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_E_VALUE);
        return 0;
      }
    } else {
      // Mitigate DoS attacks by limiting the exponent size. 33 bits was chosen
      // as the limit based on the recommendations in [1] and [2]. Windows
      // CryptoAPI doesn't support values larger than 32 bits [3], so it is
      // unlikely that exponents larger than 32 bits are being used for anything
      // Windows commonly does.
      //
      // [1] https://www.imperialviolet.org/2012/03/16/rsae.html
      // [2] https://www.imperialviolet.org/2012/03/17/rsados.html
      // [3] https://msdn.microsoft.com/en-us/library/aa387685(VS.85).aspx
      if (e_bits > kMaxExponentBits) {
        OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_E_VALUE);
        return 0;
      }

      // The upper bound on |e_bits| and lower bound on |n_bits| imply e is
      // bounded by n.
      assert(BN_ucmp(rsa->n, rsa->e) > 0);
    }
  } else if (!(rsa->flags & RSA_FLAG_NO_PUBLIC_EXPONENT)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_VALUE_MISSING);
    return 0;
  }

  return 1;
}

static int ensure_fixed_copy(BIGNUM **out, const BIGNUM *in, int width) {
  if (*out != NULL) {
    return 1;
  }
  BIGNUM *copy = BN_dup(in);
  if (copy == NULL || !bn_resize_words(copy, width)) {
    BN_free(copy);
    return 0;
  }
  *out = copy;
  bn_secret(copy);

  return 1;
}

// freeze_private_key finishes initializing |rsa|'s private key components.
// After this function has returned, |rsa| may not be changed. This is needed
// because |RSA| is a public struct and, additionally, OpenSSL 1.1.0 opaquified
// it wrong (see https://github.com/openssl/openssl/issues/5158).
static int freeze_private_key(RSA *rsa, BN_CTX *ctx) {
  CRYPTO_MUTEX_lock_read(&rsa->lock);
  int frozen = rsa->private_key_frozen;
  CRYPTO_MUTEX_unlock_read(&rsa->lock);
  if (frozen) {
    return 1;
  }

  int ret = 0;
  const BIGNUM *n_fixed;
  CRYPTO_MUTEX_lock_write(&rsa->lock);
  if (rsa->private_key_frozen) {
    ret = 1;
    goto err;
  }

  // Check the public components are within DoS bounds.
  if (!rsa_check_public_key(rsa)) {
    goto err;
  }

  // Pre-compute various intermediate values, as well as copies of private
  // exponents with correct widths. Note that other threads may concurrently
  // read from |rsa->n|, |rsa->e|, etc., so any fixes must be in separate
  // copies. We use |mont_n->N|, |mont_p->N|, and |mont_q->N| as copies of |n|,
  // |p|, and |q| with the correct minimal widths.

  if (rsa->mont_n == NULL) {
    rsa->mont_n = BN_MONT_CTX_new_for_modulus(rsa->n, ctx);
    if (rsa->mont_n == NULL) {
      goto err;
    }
  }
  n_fixed = &rsa->mont_n->N;

  // The only public upper-bound of |rsa->d| is the bit length of |rsa->n|. The
  // ASN.1 serialization of RSA private keys unfortunately leaks the byte length
  // of |rsa->d|, but normalize it so we only leak it once, rather than per
  // operation.
  if (rsa->d != NULL &&
      !ensure_fixed_copy(&rsa->d_fixed, rsa->d, n_fixed->width)) {
    goto err;
  }

  if (rsa->e != NULL && rsa->p != NULL && rsa->q != NULL) {
    // TODO: p and q are also CONSTTIME_SECRET but not yet marked as such
    // because the Montgomery code does things like test whether or not values
    // are zero. So the secret marking probably needs to happen inside that
    // code.

    if (rsa->mont_p == NULL) {
      rsa->mont_p = BN_MONT_CTX_new_consttime(rsa->p, ctx);
      if (rsa->mont_p == NULL) {
        goto err;
      }
    }

    if (rsa->mont_q == NULL) {
      rsa->mont_q = BN_MONT_CTX_new_consttime(rsa->q, ctx);
      if (rsa->mont_q == NULL) {
        goto err;
      }
    }

    if (rsa->dmp1 != NULL && rsa->dmq1 != NULL && rsa->iqmp != NULL) {
      // CRT components are only publicly bounded by their corresponding
      // moduli's bit lengths.
      const BIGNUM *p_fixed = &rsa->mont_p->N;
      const BIGNUM *q_fixed = &rsa->mont_q->N;
      if (!ensure_fixed_copy(&rsa->dmp1_fixed, rsa->dmp1, p_fixed->width) ||
          !ensure_fixed_copy(&rsa->dmq1_fixed, rsa->dmq1, q_fixed->width)) {
        goto err;
      }

      // Compute |iqmp_mont|, which is |iqmp| in Montgomery form and with the
      // correct bit width.
      if (rsa->iqmp_mont == NULL) {
        BIGNUM *iqmp_mont = BN_new();
        if (iqmp_mont == NULL ||
            !BN_to_montgomery(iqmp_mont, rsa->iqmp, rsa->mont_p, ctx)) {
          BN_free(iqmp_mont);
          goto err;
        }
        rsa->iqmp_mont = iqmp_mont;
        bn_secret(rsa->iqmp_mont);
      }
    }
  }

  rsa->private_key_frozen = 1;
  ret = 1;

err:
  CRYPTO_MUTEX_unlock_write(&rsa->lock);
  return ret;
}

void rsa_invalidate_key(RSA *rsa) {
  rsa->private_key_frozen = 0;

  BN_MONT_CTX_free(rsa->mont_n);
  rsa->mont_n = NULL;
  BN_MONT_CTX_free(rsa->mont_p);
  rsa->mont_p = NULL;
  BN_MONT_CTX_free(rsa->mont_q);
  rsa->mont_q = NULL;

  BN_free(rsa->d_fixed);
  rsa->d_fixed = NULL;
  BN_free(rsa->dmp1_fixed);
  rsa->dmp1_fixed = NULL;
  BN_free(rsa->dmq1_fixed);
  rsa->dmq1_fixed = NULL;
  BN_free(rsa->iqmp_mont);
  rsa->iqmp_mont = NULL;

  for (size_t i = 0; i < rsa->num_blindings; i++) {
    BN_BLINDING_free(rsa->blindings[i]);
  }
  OPENSSL_free(rsa->blindings);
  rsa->blindings = NULL;
  rsa->num_blindings = 0;
  OPENSSL_free(rsa->blindings_inuse);
  rsa->blindings_inuse = NULL;
  rsa->blinding_fork_generation = 0;
}

// MAX_BLINDINGS_PER_RSA defines the maximum number of cached BN_BLINDINGs per
// RSA*. Then this limit is exceeded, BN_BLINDING objects will be created and
// destroyed as needed.
#if defined(OPENSSL_TSAN)
// Smaller under TSAN so that the edge case can be hit with fewer threads.
#define MAX_BLINDINGS_PER_RSA 2
#else
#define MAX_BLINDINGS_PER_RSA 1024
#endif

// rsa_blinding_get returns a BN_BLINDING to use with |rsa|. It does this by
// allocating one of the cached BN_BLINDING objects in |rsa->blindings|. If
// none are free, the cache will be extended by a extra element and the new
// BN_BLINDING is returned.
//
// On success, the index of the assigned BN_BLINDING is written to
// |*index_used| and must be passed to |rsa_blinding_release| when finished.
static BN_BLINDING *rsa_blinding_get(RSA *rsa, size_t *index_used,
                                     BN_CTX *ctx) {
  assert(ctx != NULL);
  assert(rsa->mont_n != NULL);

  BN_BLINDING *ret = NULL;
  const uint64_t fork_generation = CRYPTO_get_fork_generation();
  CRYPTO_MUTEX_lock_write(&rsa->lock);

  // Wipe the blinding cache on |fork|.
  if (rsa->blinding_fork_generation != fork_generation) {
    for (size_t i = 0; i < rsa->num_blindings; i++) {
      // The inuse flag must be zero unless we were forked from a
      // multi-threaded process, in which case calling back into BoringSSL is
      // forbidden.
      assert(rsa->blindings_inuse[i] == 0);
      BN_BLINDING_invalidate(rsa->blindings[i]);
    }
    rsa->blinding_fork_generation = fork_generation;
  }

  uint8_t *const free_inuse_flag = reinterpret_cast<uint8_t *>(
      OPENSSL_memchr(rsa->blindings_inuse, 0, rsa->num_blindings));
  size_t new_num_blindings;
  BN_BLINDING **new_blindings;
  uint8_t *new_blindings_inuse;
  if (free_inuse_flag != NULL) {
    *free_inuse_flag = 1;
    *index_used = free_inuse_flag - rsa->blindings_inuse;
    ret = rsa->blindings[*index_used];
    goto out;
  }

  if (rsa->num_blindings >= MAX_BLINDINGS_PER_RSA) {
    // No |BN_BLINDING| is free and nor can the cache be extended. This index
    // value is magic and indicates to |rsa_blinding_release| that a
    // |BN_BLINDING| was not inserted into the array.
    *index_used = MAX_BLINDINGS_PER_RSA;
    ret = BN_BLINDING_new();
    goto out;
  }

  // Double the length of the cache.
  static_assert(MAX_BLINDINGS_PER_RSA < UINT_MAX / 2,
                "MAX_BLINDINGS_PER_RSA too large");
  new_num_blindings = rsa->num_blindings * 2;
  if (new_num_blindings == 0) {
    new_num_blindings = 1;
  }
  if (new_num_blindings > MAX_BLINDINGS_PER_RSA) {
    new_num_blindings = MAX_BLINDINGS_PER_RSA;
  }
  assert(new_num_blindings > rsa->num_blindings);

  new_blindings = reinterpret_cast<BN_BLINDING **>(
      OPENSSL_calloc(new_num_blindings, sizeof(BN_BLINDING *)));
  new_blindings_inuse =
      reinterpret_cast<uint8_t *>(OPENSSL_malloc(new_num_blindings));
  if (new_blindings == NULL || new_blindings_inuse == NULL) {
    goto err;
  }

  OPENSSL_memcpy(new_blindings, rsa->blindings,
                 sizeof(BN_BLINDING *) * rsa->num_blindings);
  OPENSSL_memcpy(new_blindings_inuse, rsa->blindings_inuse, rsa->num_blindings);

  for (size_t i = rsa->num_blindings; i < new_num_blindings; i++) {
    new_blindings[i] = BN_BLINDING_new();
    if (new_blindings[i] == NULL) {
      for (size_t j = rsa->num_blindings; j < i; j++) {
        BN_BLINDING_free(new_blindings[j]);
      }
      goto err;
    }
  }
  memset(&new_blindings_inuse[rsa->num_blindings], 0,
         new_num_blindings - rsa->num_blindings);

  new_blindings_inuse[rsa->num_blindings] = 1;
  *index_used = rsa->num_blindings;
  assert(*index_used != MAX_BLINDINGS_PER_RSA);
  ret = new_blindings[rsa->num_blindings];

  OPENSSL_free(rsa->blindings);
  rsa->blindings = new_blindings;
  OPENSSL_free(rsa->blindings_inuse);
  rsa->blindings_inuse = new_blindings_inuse;
  rsa->num_blindings = new_num_blindings;

  goto out;

err:
  OPENSSL_free(new_blindings_inuse);
  OPENSSL_free(new_blindings);

out:
  CRYPTO_MUTEX_unlock_write(&rsa->lock);
  return ret;
}

// rsa_blinding_release marks the cached BN_BLINDING at the given index as free
// for other threads to use.
static void rsa_blinding_release(RSA *rsa, BN_BLINDING *blinding,
                                 size_t blinding_index) {
  if (blinding_index == MAX_BLINDINGS_PER_RSA) {
    // This blinding wasn't cached.
    BN_BLINDING_free(blinding);
    return;
  }

  CRYPTO_MUTEX_lock_write(&rsa->lock);
  rsa->blindings_inuse[blinding_index] = 0;
  CRYPTO_MUTEX_unlock_write(&rsa->lock);
}

// signing
int rsa_default_sign_raw(RSA *rsa, size_t *out_len, uint8_t *out,
                         size_t max_out, const uint8_t *in, size_t in_len,
                         int padding) {
  const unsigned rsa_size = RSA_size(rsa);
  uint8_t *buf = NULL;
  int i, ret = 0;

  if (max_out < rsa_size) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_OUTPUT_BUFFER_TOO_SMALL);
    return 0;
  }

  buf = reinterpret_cast<uint8_t *>(OPENSSL_malloc(rsa_size));
  if (buf == NULL) {
    goto err;
  }

  switch (padding) {
    case RSA_PKCS1_PADDING:
      i = RSA_padding_add_PKCS1_type_1(buf, rsa_size, in, in_len);
      break;
    case RSA_NO_PADDING:
      i = RSA_padding_add_none(buf, rsa_size, in, in_len);
      break;
    default:
      OPENSSL_PUT_ERROR(RSA, RSA_R_UNKNOWN_PADDING_TYPE);
      goto err;
  }

  if (i <= 0) {
    goto err;
  }

  if (!rsa_private_transform_no_self_test(rsa, out, buf, rsa_size)) {
    goto err;
  }

  CONSTTIME_DECLASSIFY(out, rsa_size);
  *out_len = rsa_size;
  ret = 1;

err:
  OPENSSL_free(buf);

  return ret;
}


static int rsa_mod_exp_crt(BIGNUM *r0, const BIGNUM *I, RSA *rsa, BN_CTX *ctx);

int rsa_verify_raw_no_self_test(RSA *rsa, size_t *out_len, uint8_t *out,
                                size_t max_out, const uint8_t *in,
                                size_t in_len, int padding) {
  if (rsa->n == NULL || rsa->e == NULL) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_VALUE_MISSING);
    return 0;
  }

  if (!rsa_check_public_key(rsa)) {
    return 0;
  }

  const unsigned rsa_size = RSA_size(rsa);
  if (max_out < rsa_size) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_OUTPUT_BUFFER_TOO_SMALL);
    return 0;
  }

  if (in_len != rsa_size) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_LEN_NOT_EQUAL_TO_MOD_LEN);
    return 0;
  }

  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  if (ctx == nullptr) {
    return 0;
  }

  int ret = 0;
  uint8_t *buf = nullptr;
  bssl::BN_CTXScope scope(ctx.get());
  BIGNUM *f = BN_CTX_get(ctx.get());
  BIGNUM *result = BN_CTX_get(ctx.get());
  if (f == nullptr || result == nullptr) {
    goto err;
  }

  if (padding == RSA_NO_PADDING) {
    buf = out;
  } else {
    // Allocate a temporary buffer to hold the padded plaintext.
    buf = reinterpret_cast<uint8_t *>(OPENSSL_malloc(rsa_size));
    if (buf == nullptr) {
      goto err;
    }
  }

  if (BN_bin2bn(in, in_len, f) == nullptr) {
    goto err;
  }

  if (BN_ucmp(f, rsa->n) >= 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE_FOR_MODULUS);
    goto err;
  }

  if (!BN_MONT_CTX_set_locked(&rsa->mont_n, &rsa->lock, rsa->n, ctx.get()) ||
      !BN_mod_exp_mont(result, f, rsa->e, &rsa->mont_n->N, ctx.get(),
                       rsa->mont_n)) {
    goto err;
  }

  if (!BN_bn2bin_padded(buf, rsa_size, result)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    goto err;
  }

  switch (padding) {
    case RSA_PKCS1_PADDING:
      ret =
          RSA_padding_check_PKCS1_type_1(out, out_len, rsa_size, buf, rsa_size);
      break;
    case RSA_NO_PADDING:
      ret = 1;
      *out_len = rsa_size;
      break;
    default:
      OPENSSL_PUT_ERROR(RSA, RSA_R_UNKNOWN_PADDING_TYPE);
      goto err;
  }

  if (!ret) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_PADDING_CHECK_FAILED);
    goto err;
  }

err:
  if (buf != out) {
    OPENSSL_free(buf);
  }
  return ret;
}

int RSA_verify_raw(RSA *rsa, size_t *out_len, uint8_t *out, size_t max_out,
                   const uint8_t *in, size_t in_len, int padding) {
  boringssl_ensure_rsa_self_test();
  return rsa_verify_raw_no_self_test(rsa, out_len, out, max_out, in, in_len,
                                     padding);
}

int rsa_default_private_transform(RSA *rsa, uint8_t *out, const uint8_t *in,
                                  size_t len) {
  if (rsa->n == nullptr || rsa->d == nullptr) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_VALUE_MISSING);
    return 0;
  }

  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  if (ctx == nullptr) {
    return 0;
  }
  size_t blinding_index = 0;
  BN_BLINDING *blinding = nullptr;
  int ret = 0, do_blinding;
  bssl::BN_CTXScope scope(ctx.get());
  BIGNUM *f = BN_CTX_get(ctx.get());
  BIGNUM *result = BN_CTX_get(ctx.get());
  if (f == nullptr || result == nullptr) {
    goto err;
  }

  // The caller should have ensured this.
  assert(len == BN_num_bytes(rsa->n));
  if (BN_bin2bn(in, len, f) == nullptr) {
    goto err;
  }

  // The input to the RSA private transform may be secret, but padding is
  // expected to construct a value within range, so we can leak this comparison.
  if (constant_time_declassify_int(BN_ucmp(f, rsa->n) >= 0)) {
    // Usually the padding functions would catch this.
    OPENSSL_PUT_ERROR(RSA, RSA_R_DATA_TOO_LARGE_FOR_MODULUS);
    goto err;
  }

  if (!freeze_private_key(rsa, ctx.get())) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    goto err;
  }

  do_blinding =
      (rsa->flags & (RSA_FLAG_NO_BLINDING | RSA_FLAG_NO_PUBLIC_EXPONENT)) == 0;

  if (rsa->e == nullptr && do_blinding) {
    // We cannot do blinding or verification without |e|, and continuing without
    // those countermeasures is dangerous. However, the Java/Android RSA API
    // requires support for keys where only |d| and |n| (and not |e|) are known.
    // The callers that require that bad behavior must set
    // |RSA_FLAG_NO_BLINDING| or use |RSA_new_private_key_no_e|.
    //
    // TODO(davidben): Update this comment when Conscrypt is updated to use
    // |RSA_new_private_key_no_e|.
    OPENSSL_PUT_ERROR(RSA, RSA_R_NO_PUBLIC_EXPONENT);
    goto err;
  }

  if (do_blinding) {
    blinding = rsa_blinding_get(rsa, &blinding_index, ctx.get());
    if (blinding == nullptr) {
      OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
      goto err;
    }
    if (!BN_BLINDING_convert(f, blinding, rsa->e, rsa->mont_n, ctx.get())) {
      goto err;
    }
  }

  if (rsa->p != nullptr && rsa->q != nullptr && rsa->e != nullptr &&
      rsa->dmp1 != nullptr && rsa->dmq1 != nullptr && rsa->iqmp != nullptr &&
      // Require that we can reduce |f| by |rsa->p| and |rsa->q| in constant
      // time, which requires primes be the same size, rounded to the Montgomery
      // coefficient. (See |mod_montgomery|.) This is not required by RFC 8017,
      // but it is true for keys generated by us and all common implementations.
      bn_less_than_montgomery_R(rsa->q, rsa->mont_p) &&
      bn_less_than_montgomery_R(rsa->p, rsa->mont_q)) {
    if (!rsa_mod_exp_crt(result, f, rsa, ctx.get())) {
      goto err;
    }
  } else if (!BN_mod_exp_mont_consttime(result, f, rsa->d_fixed, rsa->n,
                                        ctx.get(), rsa->mont_n)) {
    goto err;
  }

  // Verify the result to protect against fault attacks as described in the
  // 1997 paper "On the Importance of Checking Cryptographic Protocols for
  // Faults" by Dan Boneh, Richard A. DeMillo, and Richard J. Lipton. Some
  // implementations do this only when the CRT is used, but we do it in all
  // cases. Section 6 of the aforementioned paper describes an attack that
  // works when the CRT isn't used. That attack is much less likely to succeed
  // than the CRT attack, but there have likely been improvements since 1997.
  //
  // This check is cheap assuming |e| is small, which we require in
  // |rsa_check_public_key|.
  if (rsa->e != nullptr) {
    BIGNUM *vrfy = BN_CTX_get(ctx.get());
    if (vrfy == nullptr ||
        !BN_mod_exp_mont(vrfy, result, rsa->e, rsa->n, ctx.get(),
                         rsa->mont_n) ||
        !constant_time_declassify_int(BN_equal_consttime(vrfy, f))) {
      OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
      goto err;
    }
  }

  if (do_blinding &&
      !BN_BLINDING_invert(result, blinding, rsa->mont_n, ctx.get())) {
    goto err;
  }

  // The computation should have left |result| as a maximally-wide number, so
  // that it and serializing does not leak information about the magnitude of
  // the result.
  //
  // See Falko Strenzke, "Manger's Attack revisited", ICICS 2010.
  assert(result->width == rsa->mont_n->N.width);
  bn_assert_fits_in_bytes(result, len);
  if (!BN_bn2bin_padded(out, len, result)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    goto err;
  }

  ret = 1;

err:
  if (blinding != nullptr) {
    rsa_blinding_release(rsa, blinding, blinding_index);
  }

  return ret;
}

// mod_montgomery sets |r| to |I| mod |p|. |I| must already be fully reduced
// modulo |p| times |q|. It returns one on success and zero on error.
static int mod_montgomery(BIGNUM *r, const BIGNUM *I, const BIGNUM *p,
                          const BN_MONT_CTX *mont_p, const BIGNUM *q,
                          BN_CTX *ctx) {
  // Reducing in constant-time with Montgomery reduction requires I <= p * R. We
  // have I < p * q, so this follows if q < R. The caller should have checked
  // this already.
  if (!bn_less_than_montgomery_R(q, mont_p)) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  if (  // Reduce mod p with Montgomery reduction. This computes I * R^-1 mod p.
      !BN_from_montgomery(r, I, mont_p, ctx) ||
      // Multiply by R^2 and do another Montgomery reduction to compute
      // I * R^-1 * R^2 * R^-1 = I mod p.
      !BN_to_montgomery(r, r, mont_p, ctx)) {
    return 0;
  }

  // By precomputing R^3 mod p (normally |BN_MONT_CTX| only uses R^2 mod p) and
  // adjusting the API for |BN_mod_exp_mont_consttime|, we could instead compute
  // I * R mod p here and save a reduction per prime. But this would require
  // changing the RSAZ code and may not be worth it. Note that the RSAZ code
  // uses a different radix, so it uses R' = 2^1044. There we'd actually want
  // R^2 * R', and would futher benefit from a precomputed R'^2. It currently
  // converts |mont_p->RR| to R'^2.
  return 1;
}

static int rsa_mod_exp_crt(BIGNUM *r0, const BIGNUM *I, RSA *rsa, BN_CTX *ctx) {
  assert(ctx != NULL);

  assert(rsa->n != NULL);
  assert(rsa->e != NULL);
  assert(rsa->d != NULL);
  assert(rsa->p != NULL);
  assert(rsa->q != NULL);
  assert(rsa->dmp1 != NULL);
  assert(rsa->dmq1 != NULL);
  assert(rsa->iqmp != NULL);

  bssl::BN_CTXScope scope(ctx);
  BIGNUM *r1 = BN_CTX_get(ctx);
  BIGNUM *m1 = BN_CTX_get(ctx);
  if (r1 == NULL || m1 == NULL) {
    return 0;
  }

  // Use the minimal-width versions of |n|, |p|, and |q|. Either works, but if
  // someone gives us non-minimal values, these will be slightly more efficient
  // on the non-Montgomery operations.
  BIGNUM *n = &rsa->mont_n->N;
  BIGNUM *p = &rsa->mont_p->N;
  BIGNUM *q = &rsa->mont_q->N;

  // This is a pre-condition for |mod_montgomery|. It was already checked by the
  // caller.
  declassify_assert(BN_ucmp(I, n) < 0);

  if (  // |m1| is the result modulo |q|.
      !mod_montgomery(r1, I, q, rsa->mont_q, p, ctx) ||
      !BN_mod_exp_mont_consttime(m1, r1, rsa->dmq1_fixed, q, ctx,
                                 rsa->mont_q) ||
      // |r0| is the result modulo |p|.
      !mod_montgomery(r1, I, p, rsa->mont_p, q, ctx) ||
      !BN_mod_exp_mont_consttime(r0, r1, rsa->dmp1_fixed, p, ctx,
                                 rsa->mont_p) ||
      // Compute r0 = r0 - m1 mod p. |m1| is reduced mod |q|, not |p|, so we
      // just run |mod_montgomery| again for simplicity. This could be more
      // efficient with more cases: if |p > q|, |m1| is already reduced. If
      // |p < q| but they have the same bit width, |bn_reduce_once| suffices.
      // However, compared to over 2048 Montgomery multiplications above, this
      // difference is not measurable.
      !mod_montgomery(r1, m1, p, rsa->mont_p, q, ctx) ||
      !bn_mod_sub_consttime(r0, r0, r1, p, ctx) ||
      // r0 = r0 * iqmp mod p. We use Montgomery multiplication to compute this
      // in constant time. |iqmp_mont| is in Montgomery form and r0 is not, so
      // the result is taken out of Montgomery form.
      !BN_mod_mul_montgomery(r0, r0, rsa->iqmp_mont, rsa->mont_p, ctx) ||
      // r0 = r0 * q + m1 gives the final result. Reducing modulo q gives m1, so
      // it is correct mod p. Reducing modulo p gives (r0-m1)*iqmp*q + m1 = r0,
      // so it is correct mod q. Finally, the result is bounded by [m1, n + m1),
      // and the result is at least |m1|, so this must be the unique answer in
      // [0, n).
      !bn_mul_consttime(r0, r0, q, ctx) ||  //
      !bn_uadd_consttime(r0, r0, m1)) {
    return 0;
  }

  // The result should be bounded by |n|, but fixed-width operations may
  // bound the width slightly higher, so fix it. This trips constant-time checks
  // because a naive data flow analysis does not realize the excess words are
  // publicly zero.
  declassify_assert(BN_cmp(r0, n) < 0);
  bn_assert_fits_in_bytes(r0, BN_num_bytes(n));
  if (!bn_resize_words(r0, n->width)) {
    return 0;
  }

  return 1;
}

static int ensure_bignum(BIGNUM **out) {
  if (*out == NULL) {
    *out = BN_new();
  }
  return *out != NULL;
}

// kBoringSSLRSASqrtTwo is the BIGNUM representation of ⌊2²⁰⁴⁷×√2⌋. This is
// chosen to give enough precision for 4096-bit RSA, the largest key size FIPS
// specifies. Key sizes beyond this will round up.
//
// To calculate, use the following Haskell code:
//
// import Text.Printf (printf)
// import Data.List (intercalate)
//
// pow2 = 4095
// target = 2^pow2
//
// f x = x*x - (toRational target)
//
// fprime x = 2*x
//
// newtonIteration x = x - (f x) / (fprime x)
//
// converge x =
//   let n = floor x in
//   if n*n - target < 0 && (n+1)*(n+1) - target > 0
//     then n
//     else converge (newtonIteration x)
//
// divrem bits x = (x `div` (2^bits), x `rem` (2^bits))
//
// bnWords :: Integer -> [Integer]
// bnWords x =
//   if x == 0
//     then []
//     else let (high, low) = divrem 64 x in low : bnWords high
//
// showWord x = let (high, low) = divrem 32 x in printf "TOBN(0x%08x, 0x%08x)"
// high low
//
// output :: String
// output = intercalate ", " $ map showWord $ bnWords $ converge (2 ^ (pow2
// `div` 2))
//
// To verify this number, check that n² < 2⁴⁰⁹⁵ < (n+1)², where n is value
// represented here. Note the components are listed in little-endian order. Here
// is some sample Python code to check:
//
//   >>> TOBN = lambda a, b: a << 32 | b
//   >>> l = [ <paste the contents of kSqrtTwo> ]
//   >>> n = sum(a * 2**(64*i) for i, a in enumerate(l))
//   >>> n**2 < 2**4095 < (n+1)**2
//   True
const BN_ULONG kBoringSSLRSASqrtTwo[] = {
    TOBN(0x4d7c60a5, 0xe633e3e1), TOBN(0x5fcf8f7b, 0xca3ea33b),
    TOBN(0xc246785e, 0x92957023), TOBN(0xf9acce41, 0x797f2805),
    TOBN(0xfdfe170f, 0xd3b1f780), TOBN(0xd24f4a76, 0x3facb882),
    TOBN(0x18838a2e, 0xaff5f3b2), TOBN(0xc1fcbdde, 0xa2f7dc33),
    TOBN(0xdea06241, 0xf7aa81c2), TOBN(0xf6a1be3f, 0xca221307),
    TOBN(0x332a5e9f, 0x7bda1ebf), TOBN(0x0104dc01, 0xfe32352f),
    TOBN(0xb8cf341b, 0x6f8236c7), TOBN(0x4264dabc, 0xd528b651),
    TOBN(0xf4d3a02c, 0xebc93e0c), TOBN(0x81394ab6, 0xd8fd0efd),
    TOBN(0xeaa4a089, 0x9040ca4a), TOBN(0xf52f120f, 0x836e582e),
    TOBN(0xcb2a6343, 0x31f3c84d), TOBN(0xc6d5a8a3, 0x8bb7e9dc),
    TOBN(0x460abc72, 0x2f7c4e33), TOBN(0xcab1bc91, 0x1688458a),
    TOBN(0x53059c60, 0x11bc337b), TOBN(0xd2202e87, 0x42af1f4e),
    TOBN(0x78048736, 0x3dfa2768), TOBN(0x0f74a85e, 0x439c7b4a),
    TOBN(0xa8b1fe6f, 0xdc83db39), TOBN(0x4afc8304, 0x3ab8a2c3),
    TOBN(0xed17ac85, 0x83339915), TOBN(0x1d6f60ba, 0x893ba84c),
    TOBN(0x597d89b3, 0x754abe9f), TOBN(0xb504f333, 0xf9de6484),
};
const size_t kBoringSSLRSASqrtTwoLen = OPENSSL_ARRAY_SIZE(kBoringSSLRSASqrtTwo);

// generate_prime sets |out| to a prime with length |bits| such that |out|-1 is
// relatively prime to |e|. If |p| is non-NULL, |out| will also not be close to
// |p|. |sqrt2| must be ⌊2^(bits-1)×√2⌋ (or a slightly overestimate for large
// sizes), and |pow2_bits_100| must be 2^(bits-100).
//
// This function fails with probability around 2^-21.
static int generate_prime(BIGNUM *out, int bits, const BIGNUM *e,
                          const BIGNUM *p, const BIGNUM *sqrt2,
                          const BIGNUM *pow2_bits_100, BN_CTX *ctx,
                          BN_GENCB *cb) {
  if (bits < 128 || (bits % BN_BITS2) != 0) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    return 0;
  }
  assert(BN_is_pow2(pow2_bits_100));
  assert(BN_is_bit_set(pow2_bits_100, bits - 100));

  // See FIPS 186-4 appendix B.3.3, steps 4 and 5. Note |bits| here is nlen/2.

  // Use the limit from steps 4.7 and 5.8 for most values of |e|. When |e| is 3,
  // the 186-4 limit is too low, so we use a higher one. Note this case is not
  // reachable from |RSA_generate_key_fips|.
  //
  // |limit| determines the failure probability. We must find a prime that is
  // not 1 mod |e|. By the prime number theorem, we'll find one with probability
  // p = (e-1)/e * 2/(ln(2)*bits). Note the second term is doubled because we
  // discard even numbers.
  //
  // The failure probability is thus (1-p)^limit. To convert that to a power of
  // two, we take logs. -log_2((1-p)^limit) = -limit * ln(1-p) / ln(2).
  //
  // >>> def f(bits, e, limit):
  // ...   p = (e-1.0)/e * 2.0/(math.log(2)*bits)
  // ...   return -limit * math.log(1 - p) / math.log(2)
  // ...
  // >>> f(1024, 65537, 5*1024)
  // 20.842750558272634
  // >>> f(1536, 65537, 5*1536)
  // 20.83294549602474
  // >>> f(2048, 65537, 5*2048)
  // 20.828047576234948
  // >>> f(1024, 3, 8*1024)
  // 22.222147925962307
  // >>> f(1536, 3, 8*1536)
  // 22.21518251065506
  // >>> f(2048, 3, 8*2048)
  // 22.211701985875937
  if (bits >= INT_MAX / 32) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_MODULUS_TOO_LARGE);
    return 0;
  }
  int limit = BN_is_word(e, 3) ? bits * 8 : bits * 5;

  int tries = 0, rand_tries = 0;
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *tmp = BN_CTX_get(ctx);
  if (tmp == NULL) {
    return 0;
  }

  for (;;) {
    // Generate a random number of length |bits| where the bottom bit is set
    // (steps 4.2, 4.3, 5.2 and 5.3) and the top bit is set (implied by the
    // bound checked below in steps 4.4 and 5.5).
    if (!BN_rand(out, bits, BN_RAND_TOP_ONE, BN_RAND_BOTTOM_ODD) ||
        !BN_GENCB_call(cb, BN_GENCB_GENERATED, rand_tries++)) {
      return 0;
    }

    if (p != NULL) {
      // If |p| and |out| are too close, try again (step 5.4).
      if (!bn_abs_sub_consttime(tmp, out, p, ctx)) {
        return 0;
      }
      if (BN_cmp(tmp, pow2_bits_100) <= 0) {
        continue;
      }
    }

    // If out < 2^(bits-1)×√2, try again (steps 4.4 and 5.5). This is equivalent
    // to out <= ⌊2^(bits-1)×√2⌋, or out <= sqrt2 for FIPS key sizes.
    //
    // For larger keys, the comparison is approximate, leaning towards
    // retrying. That is, we reject a negligible fraction of primes that are
    // within the FIPS bound, but we will never accept a prime outside the
    // bound, ensuring the resulting RSA key is the right size.
    //
    // Values over the threshold are discarded, so it is safe to leak this
    // comparison.
    if (constant_time_declassify_int(BN_cmp(out, sqrt2) <= 0)) {
      continue;
    }

    // RSA key generation's bottleneck is discarding composites. If it fails
    // trial division, do not bother computing a GCD or performing Miller-Rabin.
    if (!bn_odd_number_is_obviously_composite(out)) {
      // Check gcd(out-1, e) is one (steps 4.5 and 5.6). Leaking the final
      // result of this comparison is safe because, if not relatively prime, the
      // value will be discarded.
      int relatively_prime;
      if (!bn_usub_consttime(tmp, out, BN_value_one()) ||
          !bn_is_relatively_prime(&relatively_prime, tmp, e, ctx)) {
        return 0;
      }
      if (constant_time_declassify_int(relatively_prime)) {
        // Test |out| for primality (steps 4.5.1 and 5.6.1).
        int is_probable_prime;
        if (!BN_primality_test(&is_probable_prime, out,
                               BN_prime_checks_for_generation, ctx, 0, cb)) {
          return 0;
        }
        if (is_probable_prime) {
          return 1;
        }
      }
    }

    // If we've tried too many times to find a prime, abort (steps 4.7 and
    // 5.8).
    tries++;
    if (tries >= limit) {
      OPENSSL_PUT_ERROR(RSA, RSA_R_TOO_MANY_ITERATIONS);
      return 0;
    }
    if (!BN_GENCB_call(cb, 2, tries)) {
      return 0;
    }
  }
}

// rsa_generate_key_impl generates an RSA key using a generalized version of
// FIPS 186-4 appendix B.3. |RSA_generate_key_fips| performs additional checks
// for FIPS-compliant key generation.
//
// This function returns one on success and zero on failure. It has a failure
// probability of about 2^-20.
static int rsa_generate_key_impl(RSA *rsa, int bits, const BIGNUM *e_value,
                                 BN_GENCB *cb) {
  // See FIPS 186-4 appendix B.3. This function implements a generalized version
  // of the FIPS algorithm. |RSA_generate_key_fips| performs additional checks
  // for FIPS-compliant key generation.

  // Always generate RSA keys which are a multiple of 128 bits. Round |bits|
  // down as needed.
  bits &= ~127;

  // Reject excessively small keys.
  if (bits < 256) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_KEY_SIZE_TOO_SMALL);
    return 0;
  }

  // Reject excessively large public exponents. Windows CryptoAPI and Go don't
  // support values larger than 32 bits, so match their limits for generating
  // keys. (|rsa_check_public_key| uses a slightly more conservative value, but
  // we don't need to support generating such keys.)
  // https://github.com/golang/go/issues/3161
  // https://msdn.microsoft.com/en-us/library/aa387685(VS.85).aspx
  if (BN_num_bits(e_value) > 32) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_E_VALUE);
    return 0;
  }

  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  int sqrt2_bits;
  if (ctx == nullptr) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    return 0;
  }

  int prime_bits = bits / 2;
  bssl::BN_CTXScope scope(ctx.get());
  BIGNUM *totient = BN_CTX_get(ctx.get());
  BIGNUM *pm1 = BN_CTX_get(ctx.get());
  BIGNUM *qm1 = BN_CTX_get(ctx.get());
  BIGNUM *sqrt2 = BN_CTX_get(ctx.get());
  BIGNUM *pow2_prime_bits_100 = BN_CTX_get(ctx.get());
  BIGNUM *pow2_prime_bits = BN_CTX_get(ctx.get());
  if (totient == nullptr || pm1 == nullptr || qm1 == nullptr ||
      sqrt2 == nullptr || pow2_prime_bits_100 == nullptr ||
      pow2_prime_bits == nullptr ||
      !BN_set_bit(pow2_prime_bits_100, prime_bits - 100) ||
      !BN_set_bit(pow2_prime_bits, prime_bits)) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    return 0;
  }

  // We need the RSA components non-null.
  if (!ensure_bignum(&rsa->n) ||     //
      !ensure_bignum(&rsa->d) ||     //
      !ensure_bignum(&rsa->e) ||     //
      !ensure_bignum(&rsa->p) ||     //
      !ensure_bignum(&rsa->q) ||     //
      !ensure_bignum(&rsa->dmp1) ||  //
      !ensure_bignum(&rsa->dmq1) ||  //
      !ensure_bignum(&rsa->iqmp)) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    return 0;
  }

  if (!BN_copy(rsa->e, e_value)) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    return 0;
  }

  // Compute sqrt2 >= ⌊2^(prime_bits-1)×√2⌋.
  if (!bn_set_words(sqrt2, kBoringSSLRSASqrtTwo, kBoringSSLRSASqrtTwoLen)) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    return 0;
  }
  sqrt2_bits = kBoringSSLRSASqrtTwoLen * BN_BITS2;
  assert(sqrt2_bits == (int)BN_num_bits(sqrt2));
  if (sqrt2_bits > prime_bits) {
    // For key sizes up to 4096 (prime_bits = 2048), this is exactly
    // ⌊2^(prime_bits-1)×√2⌋.
    if (!BN_rshift(sqrt2, sqrt2, sqrt2_bits - prime_bits)) {
      OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
      return 0;
    }
  } else if (prime_bits > sqrt2_bits) {
    // For key sizes beyond 4096, this is approximate. We err towards retrying
    // to ensure our key is the right size and round up.
    if (!BN_add_word(sqrt2, 1) ||
        !BN_lshift(sqrt2, sqrt2, prime_bits - sqrt2_bits)) {
      OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
      return 0;
    }
  }
  assert(prime_bits == (int)BN_num_bits(sqrt2));

  do {
    // Generate p and q, each of size |prime_bits|, using the steps outlined in
    // appendix FIPS 186-4 appendix B.3.3.
    //
    // Each call to |generate_prime| fails with probability p = 2^-21. The
    // probability that either call fails is 1 - (1-p)^2, which is around 2^-20.
    if (!generate_prime(rsa->p, prime_bits, rsa->e, nullptr, sqrt2,
                        pow2_prime_bits_100, ctx.get(), cb) ||
        !BN_GENCB_call(cb, 3, 0) ||
        !generate_prime(rsa->q, prime_bits, rsa->e, rsa->p, sqrt2,
                        pow2_prime_bits_100, ctx.get(), cb) ||
        !BN_GENCB_call(cb, 3, 1)) {
      OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
      return 0;
    }

    if (BN_cmp(rsa->p, rsa->q) < 0) {
      BIGNUM *tmp = rsa->p;
      rsa->p = rsa->q;
      rsa->q = tmp;
    }

    // Calculate d = e^(-1) (mod lcm(p-1, q-1)), per FIPS 186-4. This differs
    // from typical RSA implementations which use (p-1)*(q-1).
    //
    // Note this means the size of d might reveal information about p-1 and
    // q-1. However, we do operations with Chinese Remainder Theorem, so we only
    // use d (mod p-1) and d (mod q-1) as exponents. Using a minimal totient
    // does not affect those two values.
    int no_inverse;
    if (!bn_usub_consttime(pm1, rsa->p, BN_value_one()) ||
        !bn_usub_consttime(qm1, rsa->q, BN_value_one()) ||
        !bn_lcm_consttime(totient, pm1, qm1, ctx.get()) ||
        !bn_mod_inverse_consttime(rsa->d, &no_inverse, rsa->e, totient,
                                  ctx.get())) {
      OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
      return 0;
    }

    // Retry if |rsa->d| <= 2^|prime_bits|. See appendix B.3.1's guidance on
    // values for d. When we retry, p and q are discarded, so it is safe to leak
    // this comparison.
  } while (constant_time_declassify_int(BN_cmp(rsa->d, pow2_prime_bits) <= 0));

  assert(BN_num_bits(pm1) == (unsigned)prime_bits);
  assert(BN_num_bits(qm1) == (unsigned)prime_bits);
  if (  // Calculate n.
      !bn_mul_consttime(rsa->n, rsa->p, rsa->q, ctx.get()) ||
      // Calculate d mod (p-1).
      !bn_div_consttime(nullptr, rsa->dmp1, rsa->d, pm1, prime_bits,
                        ctx.get()) ||
      // Calculate d mod (q-1)
      !bn_div_consttime(nullptr, rsa->dmq1, rsa->d, qm1, prime_bits,
                        ctx.get())) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    return 0;
  }
  bn_set_minimal_width(rsa->n);

  // |rsa->n| is computed from the private key, but is public.
  bn_declassify(rsa->n);

  // Calculate q^-1 mod p.
  rsa->mont_p = BN_MONT_CTX_new_consttime(rsa->p, ctx.get());
  if (rsa->mont_p == nullptr ||  //
      !bn_mod_inverse_secret_prime(rsa->iqmp, rsa->q, rsa->p, ctx.get(),
                                   rsa->mont_p)) {
    OPENSSL_PUT_ERROR(RSA, ERR_LIB_BN);
    return 0;
  }

  // Sanity-check that |rsa->n| has the specified size. This is implied by
  // |generate_prime|'s bounds.
  if (BN_num_bits(rsa->n) != (unsigned)bits) {
    OPENSSL_PUT_ERROR(RSA, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  // The key generation process is complex and thus error-prone. It could be
  // disastrous to generate and then use a bad key so double-check that the key
  // makes sense. Also, while |rsa| is mutable, fill in the cached components.
  if (!RSA_check_key(rsa) || !freeze_private_key(rsa, ctx.get())) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_INTERNAL_ERROR);
    return 0;
  }

  return 1;
}

static void replace_bignum(BIGNUM **out, BIGNUM **in) {
  BN_free(*out);
  *out = *in;
  *in = NULL;
}

static void replace_bn_mont_ctx(BN_MONT_CTX **out, BN_MONT_CTX **in) {
  BN_MONT_CTX_free(*out);
  *out = *in;
  *in = NULL;
}

static int RSA_generate_key_ex_maybe_fips(RSA *rsa, int bits,
                                          const BIGNUM *e_value, BN_GENCB *cb,
                                          int check_fips) {
  boringssl_ensure_rsa_self_test();

  if (rsa == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  RSA *tmp = NULL;
  uint32_t err;
  int ret = 0;

  // |rsa_generate_key_impl|'s 2^-20 failure probability is too high at scale,
  // so we run the FIPS algorithm four times, bringing it down to 2^-80. We
  // should just adjust the retry limit, but FIPS 186-4 prescribes that value
  // and thus results in unnecessary complexity.
  int failures = 0;
  do {
    ERR_clear_error();
    // Generate into scratch space, to avoid leaving partial work on failure.
    tmp = RSA_new();
    if (tmp == NULL) {
      goto out;
    }

    if (rsa_generate_key_impl(tmp, bits, e_value, cb)) {
      break;
    }

    err = ERR_peek_error();
    RSA_free(tmp);
    tmp = NULL;
    failures++;

    // Only retry on |RSA_R_TOO_MANY_ITERATIONS|. This is so a caller-induced
    // failure in |BN_GENCB_call| is still fatal.
  } while (failures < 4 && ERR_GET_LIB(err) == ERR_LIB_RSA &&
           ERR_GET_REASON(err) == RSA_R_TOO_MANY_ITERATIONS);

  if (tmp == NULL || (check_fips && !RSA_check_fips(tmp))) {
    goto out;
  }

  rsa_invalidate_key(rsa);
  replace_bignum(&rsa->n, &tmp->n);
  replace_bignum(&rsa->e, &tmp->e);
  replace_bignum(&rsa->d, &tmp->d);
  replace_bignum(&rsa->p, &tmp->p);
  replace_bignum(&rsa->q, &tmp->q);
  replace_bignum(&rsa->dmp1, &tmp->dmp1);
  replace_bignum(&rsa->dmq1, &tmp->dmq1);
  replace_bignum(&rsa->iqmp, &tmp->iqmp);
  replace_bn_mont_ctx(&rsa->mont_n, &tmp->mont_n);
  replace_bn_mont_ctx(&rsa->mont_p, &tmp->mont_p);
  replace_bn_mont_ctx(&rsa->mont_q, &tmp->mont_q);
  replace_bignum(&rsa->d_fixed, &tmp->d_fixed);
  replace_bignum(&rsa->dmp1_fixed, &tmp->dmp1_fixed);
  replace_bignum(&rsa->dmq1_fixed, &tmp->dmq1_fixed);
  replace_bignum(&rsa->iqmp_mont, &tmp->iqmp_mont);
  rsa->private_key_frozen = tmp->private_key_frozen;
  ret = 1;

out:
  RSA_free(tmp);
  return ret;
}

int RSA_generate_key_ex(RSA *rsa, int bits, const BIGNUM *e_value,
                        BN_GENCB *cb) {
  return RSA_generate_key_ex_maybe_fips(rsa, bits, e_value, cb,
                                        /*check_fips=*/0);
}

int RSA_generate_key_fips(RSA *rsa, int bits, BN_GENCB *cb) {
  // FIPS 186-4 allows 2048-bit and 3072-bit RSA keys (1024-bit and 1536-bit
  // primes, respectively) with the prime generation method we use.
  // Subsequently, IG A.14 stated that larger modulus sizes can be used and ACVP
  // testing supports 4096 bits.
  if (bits != 2048 && bits != 3072 && bits != 4096) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_RSA_PARAMETERS);
    return 0;
  }

  BIGNUM *e = BN_new();
  int ret = e != NULL && BN_set_word(e, RSA_F4) &&
            RSA_generate_key_ex_maybe_fips(rsa, bits, e, cb, /*check_fips=*/1);
  BN_free(e);

  if (ret) {
    FIPS_service_indicator_update_state();
  }
  return ret;
}

DEFINE_METHOD_FUNCTION(RSA_METHOD, RSA_default_method) {
  // All of the methods are NULL to make it easier for the compiler/linker to
  // drop unused functions. The wrapper functions will select the appropriate
  // |rsa_default_*| implementation.
  OPENSSL_memset(out, 0, sizeof(RSA_METHOD));
  out->common.is_static = 1;
}
