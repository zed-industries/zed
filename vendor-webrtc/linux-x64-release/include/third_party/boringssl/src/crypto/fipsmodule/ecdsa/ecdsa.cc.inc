// Copyright 2002-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <openssl/ecdsa.h>

#include <assert.h>
#include <string.h>

#include <openssl/bn.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "../bn/internal.h"
#include "../ec/internal.h"
#include "../service_indicator/internal.h"
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

int ecdsa_verify_fixed_no_self_test(const uint8_t *digest, size_t digest_len,
                                    const uint8_t *sig, size_t sig_len,
                                    const EC_KEY *eckey) {
  const EC_GROUP *group = EC_KEY_get0_group(eckey);
  const EC_POINT *pub_key = EC_KEY_get0_public_key(eckey);
  if (group == NULL || pub_key == NULL || sig == NULL) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_MISSING_PARAMETERS);
    return 0;
  }

  size_t scalar_len = BN_num_bytes(EC_GROUP_get0_order(group));
  EC_SCALAR r, s, u1, u2, s_inv_mont, m;
  if (sig_len != 2 * scalar_len ||
      !ec_scalar_from_bytes(group, &r, sig, scalar_len) ||
      ec_scalar_is_zero(group, &r) ||
      !ec_scalar_from_bytes(group, &s, sig + scalar_len, scalar_len) ||
      ec_scalar_is_zero(group, &s)) {
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
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_BAD_SIGNATURE);
    return 0;
  }

  return 1;
}

int ecdsa_verify_fixed(const uint8_t *digest, size_t digest_len,
                       const uint8_t *sig, size_t sig_len, const EC_KEY *key) {
  boringssl_ensure_ecc_self_test();

  return ecdsa_verify_fixed_no_self_test(digest, digest_len, sig, sig_len, key);
}

static int ecdsa_sign_impl(const EC_GROUP *group, int *out_retry, uint8_t *sig,
                           size_t *out_sig_len, size_t max_sig_len,
                           const EC_SCALAR *priv_key, const EC_SCALAR *k,
                           const uint8_t *digest, size_t digest_len) {
  *out_retry = 0;

  // Check that the size of the group order is FIPS compliant (FIPS 186-4
  // B.5.2).
  const BIGNUM *order = EC_GROUP_get0_order(group);
  if (BN_num_bits(order) < 160) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_GROUP_ORDER);
    return 0;
  }

  size_t sig_len = 2 * BN_num_bytes(order);
  if (sig_len > max_sig_len) {
    OPENSSL_PUT_ERROR(EC, EC_R_BUFFER_TOO_SMALL);
    return 0;
  }

  // Compute r, the x-coordinate of k * generator.
  EC_JACOBIAN tmp_point;
  EC_SCALAR r;
  if (!ec_point_mul_scalar_base(group, &tmp_point, k) ||
      !ec_get_x_coordinate_as_scalar(group, &r, &tmp_point)) {
    return 0;
  }

  if (constant_time_declassify_int(ec_scalar_is_zero(group, &r))) {
    *out_retry = 1;
    return 0;
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
  if (constant_time_declassify_int(ec_scalar_is_zero(group, &s))) {
    *out_retry = 1;
    return 0;
  }

  CONSTTIME_DECLASSIFY(r.words, sizeof(r.words));
  CONSTTIME_DECLASSIFY(s.words, sizeof(r.words));
  size_t len;
  ec_scalar_to_bytes(group, sig, &len, &r);
  assert(len == sig_len / 2);
  ec_scalar_to_bytes(group, sig + len, &len, &s);
  assert(len == sig_len / 2);
  *out_sig_len = sig_len;
  return 1;
}

int ecdsa_sign_fixed_with_nonce_for_known_answer_test(
    const uint8_t *digest, size_t digest_len, uint8_t *sig, size_t *out_sig_len,
    size_t max_sig_len, const EC_KEY *eckey, const uint8_t *nonce,
    size_t nonce_len) {
  if (eckey->ecdsa_meth && eckey->ecdsa_meth->sign) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_NOT_IMPLEMENTED);
    return 0;
  }

  const EC_GROUP *group = EC_KEY_get0_group(eckey);
  if (group == NULL || eckey->priv_key == NULL) {
    OPENSSL_PUT_ERROR(ECDSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  const EC_SCALAR *priv_key = &eckey->priv_key->scalar;

  EC_SCALAR k;
  if (!ec_scalar_from_bytes(group, &k, nonce, nonce_len)) {
    return 0;
  }
  int retry_ignored;
  return ecdsa_sign_impl(group, &retry_ignored, sig, out_sig_len, max_sig_len,
                         priv_key, &k, digest, digest_len);
}

int ecdsa_sign_fixed(const uint8_t *digest, size_t digest_len, uint8_t *sig,
                     size_t *out_sig_len, size_t max_sig_len,
                     const EC_KEY *eckey) {
  boringssl_ensure_ecc_self_test();

  if (eckey->ecdsa_meth && eckey->ecdsa_meth->sign) {
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_NOT_IMPLEMENTED);
    return 0;
  }

  const EC_GROUP *group = EC_KEY_get0_group(eckey);
  if (group == NULL || eckey->priv_key == NULL) {
    OPENSSL_PUT_ERROR(ECDSA, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  const BIGNUM *order = EC_GROUP_get0_order(group);
  const EC_SCALAR *priv_key = &eckey->priv_key->scalar;

  // Pass a SHA512 hash of the private key and digest as additional data
  // into the RBG. This is a hardening measure against entropy failure.
  static_assert(BCM_SHA512_DIGEST_LENGTH >= 32,
                "additional_data is too large for SHA-512");

  FIPS_service_indicator_lock_state();

  SHA512_CTX sha;
  uint8_t additional_data[BCM_SHA512_DIGEST_LENGTH];
  BCM_sha512_init(&sha);
  BCM_sha512_update(&sha, priv_key->words, order->width * sizeof(BN_ULONG));
  BCM_sha512_update(&sha, digest, digest_len);
  BCM_sha512_final(additional_data, &sha);

  // Cap iterations so callers who supply invalid values as custom groups do not
  // infinite loop. This does not impact valid parameters (e.g. those covered by
  // FIPS) because the probability of requiring even one retry is negligible,
  // let alone 32.
  static const int kMaxIterations = 32;
  int ret = 0;
  int iters = 0;
  for (;;) {
    EC_SCALAR k;
    if (!ec_random_nonzero_scalar(group, &k, additional_data)) {
      goto out;
    }

    // TODO(davidben): Move this inside |ec_random_nonzero_scalar| or lower, so
    // that all scalars we generate are, by default, secret.
    CONSTTIME_SECRET(k.words, sizeof(k.words));

    int retry;
    ret = ecdsa_sign_impl(group, &retry, sig, out_sig_len, max_sig_len,
                          priv_key, &k, digest, digest_len);
    if (ret || !retry) {
      goto out;
    }

    iters++;
    if (iters > kMaxIterations) {
      OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_TOO_MANY_ITERATIONS);
      goto out;
    }
  }

out:
  FIPS_service_indicator_unlock_state();
  return ret;
}
