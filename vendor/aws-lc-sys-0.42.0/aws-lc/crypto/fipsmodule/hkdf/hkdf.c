// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <assert.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/hkdf.h>
#include <openssl/hmac.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "../cpucap/internal.h"
#include "../service_indicator/internal.h"


int HKDF(uint8_t *out_key, size_t out_len, const EVP_MD *digest,
         const uint8_t *secret, size_t secret_len, const uint8_t *salt,
         size_t salt_len, const uint8_t *info, size_t info_len) {

  // HKDF is built on HMAC
  // HMAC does not support SHAKE (XOF) algorithms
  if (EVP_MD_flags(digest) & EVP_MD_FLAG_XOF) {
    OPENSSL_PUT_ERROR(HKDF, HKDF_R_UNSUPPORTED_DIGEST);
    return 0;
  }

  // https://tools.ietf.org/html/rfc5869#section-2
  uint8_t prk[EVP_MAX_MD_SIZE];
  size_t prk_len = 0;
  int ret = 0;

  // We have to avoid the underlying HKDF services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  if (!HKDF_extract(prk, &prk_len, digest, secret, secret_len, salt,
                    salt_len) ||
      !HKDF_expand(out_key, out_len, digest, prk, prk_len, info, info_len)) {
    // |HKDF_expand| zeroizes |out_key| on failure.
    goto out;
  }

  ret = 1;

out:
  OPENSSL_cleanse(prk, EVP_MAX_MD_SIZE);
  FIPS_service_indicator_unlock_state();
  if (ret == 1) {
    HKDF_verify_service_indicator(digest, salt, salt_len, info_len);
  }
  return ret;
}

int HKDF_extract(uint8_t *out_key, size_t *out_len, const EVP_MD *digest,
                 const uint8_t *secret, size_t secret_len, const uint8_t *salt,
                 size_t salt_len) {
  SET_DIT_AUTO_RESET;

  // HKDF is built on HMAC
  // HMAC does not support SHAKE (XOF) algorithms
  if (EVP_MD_flags(digest) & EVP_MD_FLAG_XOF) {
    OPENSSL_PUT_ERROR(HKDF, HKDF_R_UNSUPPORTED_DIGEST);
    return 0;
  }

  // https://tools.ietf.org/html/rfc5869#section-2.2
  int ret = 0;

  // We have to avoid the underlying HMAC services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  // If salt is not given, HashLength zeros are used. However, HMAC does that
  // internally already so we can ignore it.
  unsigned len;
  if (HMAC(digest, salt, salt_len, secret, secret_len, out_key, &len) == NULL) {
    OPENSSL_PUT_ERROR(HKDF, ERR_R_HMAC_LIB);
    goto out;
  }

  *out_len = len;
  assert(*out_len == EVP_MD_size(digest));
  ret = 1;

out:
  FIPS_service_indicator_unlock_state();
  return ret;
}

int HKDF_expand(uint8_t *out_key, size_t out_len, const EVP_MD *digest,
                const uint8_t *prk, size_t prk_len, const uint8_t *info,
                size_t info_len) {

  // HKDF is built on HMAC
  // HMAC does not support SHAKE (XOF) algorithms
  if (EVP_MD_flags(digest) & EVP_MD_FLAG_XOF) {
    OPENSSL_PUT_ERROR(HKDF, HKDF_R_UNSUPPORTED_DIGEST);
    return 0;
  }

  // https://tools.ietf.org/html/rfc5869#section-2.3
  SET_DIT_AUTO_RESET;
  const size_t digest_len = EVP_MD_size(digest);
  uint8_t previous[EVP_MAX_MD_SIZE];
  int ret = 0;
  HMAC_CTX hmac;

  // Expand key material to desired length.
  size_t n = (out_len + digest_len - 1) / digest_len;
  if (out_len + digest_len < out_len || n > 255) {
    OPENSSL_PUT_ERROR(HKDF, HKDF_R_OUTPUT_TOO_LARGE);
    return 0;
  }

  // We have to avoid the underlying HMAC services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  HMAC_CTX_init(&hmac);
  if (!HMAC_Init_ex(&hmac, prk, prk_len, digest, NULL)) {
    goto out;
  }

  size_t done = 0;
  uint32_t written = 0;

  for (size_t i = 0; i < n; i++) {
    // n is verified above to be <= 255. Hence 8-bit type is sufficient.
    uint8_t ctr = i + 1;

    if (i != 0 && (!HMAC_Init_ex(&hmac, NULL, 0, NULL, NULL) ||
                   !HMAC_Update(&hmac, previous, digest_len))) {
      goto out;
    }

    written = 0;
    if (!HMAC_Update(&hmac, info, info_len) ||
        !HMAC_Update(&hmac, &ctr, 1) ||
        !HMAC_Final(&hmac, previous, &written) ||
        written != digest_len) {
      goto out;
    }

    if (written > out_len - done) {
      written = out_len - done;
    }
    OPENSSL_memcpy(out_key + done, previous, written);
    done += written;
  }

  ret = 1;

out:
  OPENSSL_cleanse(previous, EVP_MAX_MD_SIZE);
  HMAC_CTX_cleanup(&hmac);
  if (ret != 1) {
    OPENSSL_PUT_ERROR(HKDF, ERR_R_HMAC_LIB);
    OPENSSL_cleanse(out_key, out_len);
  }

  FIPS_service_indicator_unlock_state();
  if (ret == 1) {
    HKDFExpand_verify_service_indicator(digest);
  }

  return ret;
}
