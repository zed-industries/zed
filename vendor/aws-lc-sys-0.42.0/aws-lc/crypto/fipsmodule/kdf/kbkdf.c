// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/kdf.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "../service_indicator/internal.h"
#include "internal.h"

int KBKDF_ctr_hmac(uint8_t *out_key, size_t out_len, const EVP_MD *digest,
                   const uint8_t *secret, size_t secret_len,
                   const uint8_t *info, size_t info_len) {
  SET_DIT_AUTO_RESET;

  // We have to avoid the underlying |HMAC_Final| services updating
  // the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  HMAC_CTX hmac_ctx;
  int ret = 0;

  if (!out_key || out_len == 0 || !secret || secret_len == 0) {
    goto err;
  }

  HMAC_CTX_init(&hmac_ctx);
  if (!HMAC_Init_ex(&hmac_ctx, secret, secret_len, digest, NULL)) {
    goto err;
  }

  // Determine the length of the output in bytes of a single invocation of the
  // HMAC function.
  const size_t h_output_bytes = HMAC_size(&hmac_ctx);
  if (h_output_bytes == 0 || h_output_bytes > EVP_MAX_MD_SIZE) {
    goto err;
  }

  if (out_len > SIZE_MAX - h_output_bytes) {
    goto err;
  }

  // NIST.SP.800-108r1-upd1: Step 1:
  // Determine how many output chunks are required to produce the requested
  // output length |out_len|. This determines how many times the variant compute
  // function will be called to output key material.
  uint64_t n = ((uint64_t)out_len + (uint64_t)h_output_bytes - 1) /
               (uint64_t)h_output_bytes;

  // NIST.SP.800-108r1-upd1: Step 2:
  // Verify that the number of output chunks does not exceed an unsigned 32-bit
  // integer.
  if (n > UINT32_MAX) {
    goto err;
  }

  uint8_t out_key_i[EVP_MAX_MD_SIZE];
  uint8_t counter[KBKDF_COUNTER_SIZE];
  size_t done = 0;
  uint32_t written = 0;

  for (uint32_t i = 0; i < n; i++) {
    // Increment the counter
    CRYPTO_store_u32_be(&counter[0], i + 1);

    written = 0;
    // NIST.SP.800-108r1-upd1: Step 4a:
    // K(i) := PRF(K_IN, [i] || FixedInfo)
    // Note |hmac_ctx| has already been configured with the secret key
    if (!HMAC_Init_ex(&hmac_ctx, NULL, 0, NULL, NULL) ||
        !HMAC_Update(&hmac_ctx, &counter[0], KBKDF_COUNTER_SIZE) ||
        !HMAC_Update(&hmac_ctx, info, info_len) ||
        !HMAC_Final(&hmac_ctx, out_key_i, &written) ||
        written != h_output_bytes) {
      goto err;
    }

    // NIST.SP.800-108r1-upd1: Step 4b, Step 5
    // result := result || K(i)
    // Ensure that we only copy |out_len| bytes in total from all chunks.
    if (written > out_len - done) {
      written = out_len - done;
    }
    OPENSSL_memcpy(out_key + done, out_key_i, written);
    done += written;
  }

  ret = 1;

err:
  OPENSSL_cleanse(&out_key_i[0], EVP_MAX_MD_SIZE);
  if (ret <= 0 && out_key && out_len > 0) {
    OPENSSL_cleanse(out_key, out_len);
  }
  HMAC_CTX_cleanup(&hmac_ctx);
  FIPS_service_indicator_unlock_state();
  if (ret) {
    KBKDF_ctr_hmac_verify_service_indicator(digest, secret_len);
  }
  return ret;
}
