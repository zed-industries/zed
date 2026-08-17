// Copyright (c) 1998-2005 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ecdsa.h>

#include <assert.h>
#include <string.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/sha.h>
#include <openssl/type_check.h>

#include "../../internal.h"
#include "../bn/internal.h"
#include "../ec/internal.h"
#include "internal.h"


// digest_to_scalar interprets |digest_len| bytes from |digest| as a scalar for
// ECDSA.
static void digest_to_scalar(const EC_GROUP *group, EC_SCALAR *out,
                             const uint8_t *digest, size_t digest_len) {
  const BIGNUM *order = EC_GROUP_get0_order(group);
  size_t num_bits = BN_num_bits(order);
  // Need to truncate digest if it is too long: first truncate whole bytes.
  size_t num_bytes = (num_bits + 7) / 8;
  if (digest_len > num_bytes) {
    digest_len = num_bytes;
  }
  bn_big_endian_to_words(out->words, order->width, digest, digest_len);

  // If it is still too long, truncate remaining bits with a shift.
  if (8 * digest_len > num_bits) {
    bn_rshift_words(out->words, out->words, 8 - (num_bits & 0x7), order->width);
  }

  // |out| now has the same bit width as |order|, but this only bounds by
  // 2*|order|. Subtract the order if out of range.
  //
  // Montgomery multiplication accepts the looser bounds, so this isn't strictly
  // necessary, but it is a cleaner abstraction and has no performance impact.
  BN_ULONG tmp[EC_MAX_WORDS];
  bn_reduce_once_in_place(out->words, 0 /* no carry */, order->d, tmp,
                          order->width);
}

ECDSA_SIG *ECDSA_SIG_new(void) {
  ECDSA_SIG *sig = OPENSSL_malloc(sizeof(ECDSA_SIG));
  if (sig == NULL) {
    return NULL;
  }
  sig->r = BN_new();
  sig->s = BN_new();
  if (sig->r == NULL || sig->s == NULL) {
    ECDSA_SIG_free(sig);
    return NULL;
  }
  return sig;
}

void ECDSA_SIG_free(ECDSA_SIG *sig) {
  if (sig == NULL) {
    return;
  }

  BN_free(sig->r);
  BN_free(sig->s);
  OPENSSL_free(sig);
}

const BIGNUM *ECDSA_SIG_get0_r(const ECDSA_SIG *sig) {
  return sig->r;
}

const BIGNUM *ECDSA_SIG_get0_s(const ECDSA_SIG *sig) {
  return sig->s;
}

void ECDSA_SIG_get0(const ECDSA_SIG *sig, const BIGNUM **out_r,
                    const BIGNUM **out_s) {
  if (out_r != NULL) {
    *out_r = sig->r;
  }
  if (out_s != NULL) {
    *out_s = sig->s;
  }
}

int ECDSA_SIG_set0(ECDSA_SIG *sig, BIGNUM *r, BIGNUM *s) {
  if (r == NULL || s == NULL) {
    return 0;
  }
  BN_free(sig->r);
  BN_free(sig->s);
  sig->r = r;
  sig->s = s;
  return 1;
}

int ecdsa_do_verify_no_self_test(const uint8_t *digest, size_t digest_len,
                                 const ECDSA_SIG *sig, const EC_KEY *eckey) {
  const EC_GROUP *group = EC_KEY_get0_group(eckey);
  const EC_POINT *pub_key = EC_KEY_get0_public_key(eckey);
  if (group == NULL || pub_key == NULL || sig == NULL) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_MISSING_PARAMETERS);
    return 0;
  }

  EC_SCALAR r, s, u1, u2, s_inv_mont, m;
  if (BN_is_zero(sig->r) ||
      !ec_bignum_to_scalar(group, &r, sig->r) ||
      BN_is_zero(sig->s) ||
      !ec_bignum_to_scalar(group, &s, sig->s)) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_BAD_SIGNATURE);
    return 0;
  }

  // s_inv_mont = s^-1 in the Montgomery domain.
  if (!ec_scalar_to_montgomery_inv_vartime(group, &s_inv_mont, &s)) {
    OPENSSL_PUT_ERROR(ECDSA, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  // u1 = m * s^-1 mod order
  // u2 = r * s^-1 mod order
  //
  // |s_inv_mont| is in Montgomery form while |m| and |r| are not, so |u1| and
  // |u2| will be taken out of Montgomery form, as desired.
  digest_to_scalar(group, &m, digest, digest_len);
  ec_scalar_mul_montgomery(group, &u1, &m, &s_inv_mont);
  ec_scalar_mul_montgomery(group, &u2, &r, &s_inv_mont);

  EC_JACOBIAN point;
  if (!ec_point_mul_scalar_public(group, &point, &u1, &pub_key->raw, &u2)) {
    OPENSSL_PUT_ERROR(ECDSA, ERR_R_EC_LIB);
    return 0;
  }

  if (!ec_cmp_x_coordinate(group, &point, &r)) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_MISMATCHED_SIGNATURE);
    return 0;
  }

  return 1;
}

int ECDSA_do_verify(const uint8_t *digest, size_t digest_len,
                    const ECDSA_SIG *sig, const EC_KEY *eckey) {
  boringssl_ensure_ecc_self_test();

  return ecdsa_do_verify_no_self_test(digest, digest_len, sig, eckey);
}

static ECDSA_SIG *ecdsa_sign_impl(const EC_GROUP *group, int *out_retry,
                                  const EC_SCALAR *priv_key, const EC_SCALAR *k,
                                  const uint8_t *digest, size_t digest_len) {
  *out_retry = 0;

  // Check that the size of the group order is FIPS compliant (FIPS 186-4
  // B.5.2).
  const BIGNUM *order = EC_GROUP_get0_order(group);
  if (BN_num_bits(order) < 160) {
    OPENSSL_PUT_ERROR(ECDSA, EC_R_INVALID_GROUP_ORDER);
    return NULL;
  }

  // Compute r, the x-coordinate of k * generator.
  EC_JACOBIAN tmp_point;
  EC_SCALAR r;
  if (!ec_point_mul_scalar_base(group, &tmp_point, k) ||
      !ec_get_x_coordinate_as_scalar(group, &r, &tmp_point)) {
    return NULL;
  }

  if (constant_time_declassify_int(ec_scalar_is_zero(group, &r))) {
    *out_retry = 1;
    return NULL;
  }

  // s = priv_key * r. Note if only one parameter is in the Montgomery domain,
  // |ec_scalar_mod_mul_montgomery| will compute the answer in the normal
  // domain.
  EC_SCALAR s;
  ec_scalar_to_montgomery(group, &s, &r);
  ec_scalar_mul_montgomery(group, &s, priv_key, &s);

  // s = m + priv_key * r.
  EC_SCALAR tmp;
  digest_to_scalar(group, &tmp, digest, digest_len);
  ec_scalar_add(group, &s, &s, &tmp);

  // s = k^-1 * (m + priv_key * r). First, we compute k^-1 in the Montgomery
  // domain. This is |ec_scalar_to_montgomery| followed by
  // |ec_scalar_inv0_montgomery|, but |ec_scalar_inv0_montgomery| followed by
  // |ec_scalar_from_montgomery| is equivalent and slightly more efficient.
  // Then, as above, only one parameter is in the Montgomery domain, so the
  // result is in the normal domain. Finally, note k is non-zero (or computing r
  // would fail), so the inverse must exist.
  ec_scalar_inv0_montgomery(group, &tmp, k);     // tmp = k^-1 R^2
  ec_scalar_from_montgomery(group, &tmp, &tmp);  // tmp = k^-1 R
  ec_scalar_mul_montgomery(group, &s, &s, &tmp);
  OPENSSL_cleanse(&tmp, sizeof(tmp));
  if (constant_time_declassify_int(ec_scalar_is_zero(group, &s))) {
    *out_retry = 1;
    return NULL;
  }

  CONSTTIME_DECLASSIFY(r.words, sizeof(r.words));
  CONSTTIME_DECLASSIFY(s.words, sizeof(r.words));
  ECDSA_SIG *ret = ECDSA_SIG_new();
  if (ret == NULL ||  //
      !bn_set_words(ret->r, r.words, order->width) ||
      !bn_set_words(ret->s, s.words, order->width)) {
    ECDSA_SIG_free(ret);
    return NULL;
  }
  return ret;
}

ECDSA_SIG *ecdsa_sign_with_nonce_for_known_answer_test(const uint8_t *digest,
                                                       size_t digest_len,
                                                       const EC_KEY *eckey,
                                                       const uint8_t *nonce,
                                                       size_t nonce_len) {
  if (eckey->eckey_method && eckey->eckey_method->sign) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_NOT_IMPLEMENTED);
    return NULL;
  }

  const EC_GROUP *group = EC_KEY_get0_group(eckey);
  if (group == NULL || eckey->priv_key == NULL) {
    OPENSSL_PUT_ERROR(ECDSA, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }
  const EC_SCALAR *priv_key = &eckey->priv_key->scalar;

  EC_SCALAR k;
  if (!ec_scalar_from_bytes(group, &k, nonce, nonce_len)) {
    return NULL;
  }
  int retry_ignored;
  return ecdsa_sign_impl(group, &retry_ignored, priv_key, &k, digest,
                         digest_len);
}

// This function is only exported for testing and is not called in production
// code.
ECDSA_SIG *ECDSA_sign_with_nonce_and_leak_private_key_for_testing(
    const uint8_t *digest, size_t digest_len, const EC_KEY *eckey,
    const uint8_t *nonce, size_t nonce_len) {
  boringssl_ensure_ecc_self_test();

  return ecdsa_sign_with_nonce_for_known_answer_test(digest, digest_len, eckey,
                                                     nonce, nonce_len);
}

ECDSA_SIG *ECDSA_do_sign(const uint8_t *digest, size_t digest_len,
                         const EC_KEY *eckey) {
  boringssl_ensure_ecc_self_test();

  if (eckey->eckey_method && eckey->eckey_method->sign_sig) {
    return eckey->eckey_method->sign_sig(digest, (int)digest_len, NULL, NULL,
                                         (EC_KEY *)eckey);
  }

  const EC_GROUP *group = EC_KEY_get0_group(eckey);
  if (group == NULL || eckey->priv_key == NULL) {
    OPENSSL_PUT_ERROR(ECDSA, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }
  // We have to avoid the underlying |SHA512_Final| services updating the
  // indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  const BIGNUM *order = EC_GROUP_get0_order(group);
  const EC_SCALAR *priv_key = &eckey->priv_key->scalar;

  // Pass a SHA512 hash of the private key and digest as additional data
  // into the RBG. This is a hardening measure against entropy failure.
  OPENSSL_STATIC_ASSERT(SHA512_DIGEST_LENGTH >= 32,
                        additional_data_is_too_large_for_SHA_512)
  SHA512_CTX sha;
  uint8_t additional_data[SHA512_DIGEST_LENGTH];
  SHA512_Init(&sha);
  SHA512_Update(&sha, priv_key->words, order->width * sizeof(BN_ULONG));
  SHA512_Update(&sha, digest, digest_len);
  SHA512_Final(additional_data, &sha);

  FIPS_service_indicator_unlock_state();
  // Cap iterations so callers who supply invalid values as custom groups do not
  // infinite loop. This does not impact valid parameters (e.g. those covered by
  // FIPS) because the probability of requiring even one retry is negligible,
  // let alone 32.
  static const int kMaxIterations = 32;
  int iters = 0;
  for (;;) {
    EC_SCALAR k;
    if (!ec_random_nonzero_scalar(group, &k, additional_data)) {
      OPENSSL_cleanse(&k, sizeof(EC_SCALAR));
      return NULL;
    }

    // TODO(davidben): Move this inside |ec_random_nonzero_scalar| or lower, so
    // that all scalars we generate are, by default, secret.
    CONSTTIME_SECRET(k.words, sizeof(k.words));

    int retry;
    ECDSA_SIG *sig =
        ecdsa_sign_impl(group, &retry, priv_key, &k, digest, digest_len);
    if (sig != NULL || !retry) {
      OPENSSL_cleanse(&k, sizeof(EC_SCALAR));
      return sig;
    }

    iters++;
    if (iters > kMaxIterations) {
      OPENSSL_cleanse(&k, sizeof(EC_SCALAR));
      OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_TOO_MANY_ITERATIONS);
      return NULL;
    }
  }
}

// |ECDSA_sign| uses ASN1/CBB functionality, so it was previously placed in
// crypto/ecdsa_extra/ecdsa_asn1.c. It's now moved within the FIPS boundary for
// FIPS compliance.
int ECDSA_sign(int type, const uint8_t *digest, size_t digest_len, uint8_t *sig,
               unsigned int *sig_len, const EC_KEY *eckey) {
  if (eckey->eckey_method && eckey->eckey_method->sign) {
    return eckey->eckey_method->sign(type, digest, (int)digest_len, sig, sig_len,
                                     NULL, NULL,
                                     (EC_KEY*) eckey /* cast away const */);
  }

  int ret = 0;
  ECDSA_SIG *s = ECDSA_do_sign(digest, digest_len, eckey);
  if (s == NULL) {
    *sig_len = 0;
    goto err;
  }

  CBB cbb;
  CBB_init_fixed(&cbb, sig, ECDSA_size(eckey));
  size_t len;
  if (!ECDSA_SIG_marshal(&cbb, s) ||
      !CBB_finish(&cbb, NULL, &len)) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_ENCODE_ERROR);
    *sig_len = 0;
    goto err;
  }
  *sig_len = (unsigned)len;
  ret = 1;

err:
  ECDSA_SIG_free(s);
  return ret;
}

// |ECDSA_verify| uses ASN1/CBB functionality, so it was previously placed in
// crypto/evp_extra/ecdsa_asn1.c. It's now moved within the FIPS boundary for
// FIPS compliance.
int ECDSA_verify(int type, const uint8_t *digest, size_t digest_len,
                 const uint8_t *sig, size_t sig_len, const EC_KEY *eckey) {
  ECDSA_SIG *s;
  int ret = 0;
  uint8_t *der = NULL;

  // Decode the ECDSA signature.
  s = ECDSA_SIG_from_bytes(sig, sig_len);
  if (s == NULL) {
    goto err;
  }

  // Defend against potential laxness in the DER parser.
  size_t der_len;
  if (!ECDSA_SIG_to_bytes(&der, &der_len, s) ||
      der_len != sig_len || OPENSSL_memcmp(sig, der, sig_len) != 0) {
    // This should never happen. crypto/bytestring is strictly DER.
    OPENSSL_PUT_ERROR(ECDSA, ERR_R_INTERNAL_ERROR);
    goto err;
  }

  ret = ECDSA_do_verify(digest, digest_len, s, eckey);

err:
  OPENSSL_free(der);
  ECDSA_SIG_free(s);
  return ret;
}

ECDSA_SIG *ecdsa_digestsign_no_self_test(const EVP_MD *md, const uint8_t *input,
                                         size_t in_len, const EC_KEY *eckey,
                                         const uint8_t *nonce,
                                         size_t nonce_len) {
  uint8_t digest[EVP_MAX_MD_SIZE];
  unsigned int digest_len = EVP_MAX_MD_SIZE;
  if (!EVP_Digest(input, in_len, digest, &digest_len, md, NULL)) {
    return 0;
  }

  return ecdsa_sign_with_nonce_for_known_answer_test(digest, digest_len, eckey,
                                                     nonce, nonce_len);
}

int ecdsa_digestverify_no_self_test(const EVP_MD *md, const uint8_t *input,
                                    size_t in_len, const ECDSA_SIG *sig,
                                    const EC_KEY *eckey){
  uint8_t digest[EVP_MAX_MD_SIZE];
  unsigned int digest_len = EVP_MAX_MD_SIZE;
  if (!EVP_Digest(input, in_len, digest, &digest_len, md, NULL)) {
    return 0;
  }

  return ecdsa_do_verify_no_self_test(digest, digest_len, sig, eckey);
}
