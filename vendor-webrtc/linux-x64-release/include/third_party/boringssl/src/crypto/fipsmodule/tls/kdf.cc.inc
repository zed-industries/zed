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

#include <assert.h>

#include <openssl/bytestring.h>
#include <openssl/digest.h>
#include <openssl/hkdf.h>
#include <openssl/hmac.h>
#include <openssl/mem.h>

#include "internal.h"
#include "../../internal.h"
#include "../service_indicator/internal.h"


// tls1_P_hash computes the TLS P_<hash> function as described in RFC 5246,
// section 5. It XORs |out_len| bytes to |out|, using |md| as the hash and
// |secret| as the secret. |label|, |seed1|, and |seed2| are concatenated to
// form the seed parameter. It returns true on success and false on failure.
static int tls1_P_hash(uint8_t *out, size_t out_len,
                       const EVP_MD *md,
                       const uint8_t *secret, size_t secret_len,
                       const char *label, size_t label_len,
                       const uint8_t *seed1, size_t seed1_len,
                       const uint8_t *seed2, size_t seed2_len) {
  HMAC_CTX ctx, ctx_tmp, ctx_init;
  uint8_t A1[EVP_MAX_MD_SIZE];
  unsigned A1_len;
  int ret = 0;

  const size_t chunk = EVP_MD_size(md);
  HMAC_CTX_init(&ctx);
  HMAC_CTX_init(&ctx_tmp);
  HMAC_CTX_init(&ctx_init);

  if (!HMAC_Init_ex(&ctx_init, secret, secret_len, md, NULL) ||
      !HMAC_CTX_copy_ex(&ctx, &ctx_init) ||
      !HMAC_Update(&ctx, (const uint8_t *) label, label_len) ||
      !HMAC_Update(&ctx, seed1, seed1_len) ||
      !HMAC_Update(&ctx, seed2, seed2_len) ||
      !HMAC_Final(&ctx, A1, &A1_len)) {
    goto err;
  }

  for (;;) {
    unsigned len_u;
    uint8_t hmac[EVP_MAX_MD_SIZE];
    if (!HMAC_CTX_copy_ex(&ctx, &ctx_init) ||
        !HMAC_Update(&ctx, A1, A1_len) ||
        // Save a copy of |ctx| to compute the next A1 value below.
        (out_len > chunk && !HMAC_CTX_copy_ex(&ctx_tmp, &ctx)) ||
        !HMAC_Update(&ctx, (const uint8_t *) label, label_len) ||
        !HMAC_Update(&ctx, seed1, seed1_len) ||
        !HMAC_Update(&ctx, seed2, seed2_len) ||
        !HMAC_Final(&ctx, hmac, &len_u)) {
      goto err;
    }
    size_t len = len_u;
    assert(len == chunk);

    // XOR the result into |out|.
    if (len > out_len) {
      len = out_len;
    }
    for (size_t i = 0; i < len; i++) {
      out[i] ^= hmac[i];
    }
    out += len;
    out_len -= len;

    if (out_len == 0) {
      break;
    }

    // Calculate the next A1 value.
    if (!HMAC_Final(&ctx_tmp, A1, &A1_len)) {
      goto err;
    }
  }

  ret = 1;

err:
  OPENSSL_cleanse(A1, sizeof(A1));
  HMAC_CTX_cleanup(&ctx);
  HMAC_CTX_cleanup(&ctx_tmp);
  HMAC_CTX_cleanup(&ctx_init);
  return ret;
}

int CRYPTO_tls1_prf(const EVP_MD *digest,
                    uint8_t *out, size_t out_len,
                    const uint8_t *secret, size_t secret_len,
                    const char *label, size_t label_len,
                    const uint8_t *seed1, size_t seed1_len,
                    const uint8_t *seed2, size_t seed2_len) {
  if (out_len == 0) {
    return 1;
  }

  OPENSSL_memset(out, 0, out_len);

  const EVP_MD *const original_digest = digest;
  FIPS_service_indicator_lock_state();
  int ret = 0;

  if (digest == EVP_md5_sha1()) {
    // If using the MD5/SHA1 PRF, |secret| is partitioned between MD5 and SHA-1.
    size_t secret_half = secret_len - (secret_len / 2);
    if (!tls1_P_hash(out, out_len, EVP_md5(), secret, secret_half, label,
                     label_len, seed1, seed1_len, seed2, seed2_len)) {
      goto end;
    }

    // Note that, if |secret_len| is odd, the two halves share a byte.
    secret += secret_len - secret_half;
    secret_len = secret_half;
    digest = EVP_sha1();
  }

  ret = tls1_P_hash(out, out_len, digest, secret, secret_len, label, label_len,
                    seed1, seed1_len, seed2, seed2_len);

end:
  FIPS_service_indicator_unlock_state();
  if (ret) {
    TLSKDF_verify_service_indicator(original_digest);
  }
  return ret;
}

int CRYPTO_tls13_hkdf_expand_label(uint8_t *out, size_t out_len,
                                   const EVP_MD *digest,  //
                                   const uint8_t *secret, size_t secret_len,
                                   const uint8_t *label, size_t label_len,
                                   const uint8_t *hash, size_t hash_len) {
  static const uint8_t kProtocolLabel[] = "tls13 ";
  CBB cbb, child;
  uint8_t *hkdf_label = NULL;
  size_t hkdf_label_len;

  FIPS_service_indicator_lock_state();
  CBB_zero(&cbb);
  if (!CBB_init(&cbb, 2 + 1 + sizeof(kProtocolLabel) - 1 + label_len + 1 +
                          hash_len) ||
      !CBB_add_u16(&cbb, out_len) ||
      !CBB_add_u8_length_prefixed(&cbb, &child) ||
      !CBB_add_bytes(&child, kProtocolLabel, sizeof(kProtocolLabel) - 1) ||
      !CBB_add_bytes(&child, label, label_len) ||
      !CBB_add_u8_length_prefixed(&cbb, &child) ||
      !CBB_add_bytes(&child, hash, hash_len) ||
      !CBB_finish(&cbb, &hkdf_label, &hkdf_label_len)) {
    CBB_cleanup(&cbb);
    FIPS_service_indicator_unlock_state();
    return 0;
  }

  const int ret = HKDF_expand(out, out_len, digest, secret, secret_len,
                              hkdf_label, hkdf_label_len);
  OPENSSL_free(hkdf_label);

  FIPS_service_indicator_unlock_state();
  if (ret) {
    TLSKDF_verify_service_indicator(digest);
  }
  return ret;
}

