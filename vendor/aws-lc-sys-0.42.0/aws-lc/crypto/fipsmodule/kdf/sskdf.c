// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <assert.h>
#include <limits.h>
#include <openssl/base.h>
#include <openssl/digest.h>
#include <openssl/hmac.h>
#include <openssl/kdf.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "../delocate.h"
#include "../service_indicator/internal.h"
#include "internal.h"

static int sskdf_variant_digest_ctx_init(sskdf_variant_ctx *ctx,
                                         const EVP_MD *digest) {
  sskdf_variant_digest_ctx *variant_ctx = NULL;
  EVP_MD_CTX *md_ctx = NULL;

  int ret = 0;

  if (!ctx || ctx->data || !digest) {
    goto err;
  }

  variant_ctx = OPENSSL_malloc(sizeof(sskdf_variant_digest_ctx));
  if (!variant_ctx) {
    goto err;
  }

  md_ctx = EVP_MD_CTX_new();
  if (!md_ctx) {
    goto err;
  }

  ret = 1;
  variant_ctx->digest = digest;
  variant_ctx->md_ctx = md_ctx;
  ctx->data = variant_ctx;

  return ret;

err:
  EVP_MD_CTX_free(md_ctx);
  OPENSSL_free(variant_ctx);

  return ret;
}

static void sskdf_variant_digest_ctx_cleanup(sskdf_variant_ctx *ctx) {
  if (!ctx || !ctx->data) {
    return;
  }
  sskdf_variant_digest_ctx *variant_ctx = (sskdf_variant_digest_ctx *)ctx->data;
  EVP_MD_CTX_free(variant_ctx->md_ctx);
  OPENSSL_free(variant_ctx);
  ctx->data = NULL;
}

static size_t sskdf_variant_digest_output_size(sskdf_variant_ctx *ctx) {
  if (!ctx || !ctx->data) {
    return 0;
  }
  sskdf_variant_digest_ctx *variant_ctx = (sskdf_variant_digest_ctx *)ctx->data;
  return EVP_MD_size(variant_ctx->digest);
}

static int sskdf_variant_digest_compute(
    sskdf_variant_ctx *ctx, uint8_t *out, size_t out_len,
    const uint8_t counter[SSKDF_COUNTER_SIZE], const uint8_t *secret,
    size_t secret_len, const uint8_t *info, size_t info_len) {
  if (!ctx || !ctx->data || !out || !counter || !secret) {
    return 0;
  }

  sskdf_variant_digest_ctx *variant_ctx = (sskdf_variant_digest_ctx *)ctx->data;

  if (!variant_ctx->md_ctx || !variant_ctx->digest) {
    return 0;
  }

  uint32_t written;

  // NIST.SP.800-56Cr2: Step 6.2 hash(counter || secret || info)
  if (!EVP_MD_CTX_reset(variant_ctx->md_ctx) ||
      !EVP_DigestInit_ex(variant_ctx->md_ctx, variant_ctx->digest, NULL) ||
      !EVP_DigestUpdate(variant_ctx->md_ctx, &counter[0], SSKDF_COUNTER_SIZE) ||
      !EVP_DigestUpdate(variant_ctx->md_ctx, secret, secret_len) ||
      !EVP_DigestUpdate(variant_ctx->md_ctx, info, info_len) ||
      !EVP_DigestFinal(variant_ctx->md_ctx, out, &written) ||
      written != out_len) {
    return 0;
  }

  return 1;
}

DEFINE_METHOD_FUNCTION(sskdf_variant, sskdf_variant_digest) {
  out->h_output_bytes = sskdf_variant_digest_output_size;
  out->compute = sskdf_variant_digest_compute;
}

static int sskdf_variant_hmac_ctx_init(sskdf_variant_ctx *ctx,
                                       const EVP_MD *digest,
                                       const uint8_t *salt, size_t salt_len) {
  sskdf_variant_hmac_ctx *variant_ctx = NULL;
  HMAC_CTX *hmac_ctx = NULL;

  int ret = 0;

  if (!ctx || ctx->data || !digest) {
    goto err;
  }

  variant_ctx = OPENSSL_malloc(sizeof(sskdf_variant_hmac_ctx));
  if (!variant_ctx) {
    goto err;
  }

  hmac_ctx = HMAC_CTX_new();
  if (!hmac_ctx) {
    goto err;
  }

  if (!HMAC_Init_ex(hmac_ctx, salt, salt_len, digest, NULL)) {
    goto err;
  }

  ret = 1;
  variant_ctx->hmac_ctx = hmac_ctx;
  ctx->data = variant_ctx;

  return ret;

err:
  HMAC_CTX_free(hmac_ctx);
  OPENSSL_free(variant_ctx);

  return ret;
}

static void sskdf_variant_hmac_ctx_cleanup(sskdf_variant_ctx *ctx) {
  if (!ctx || !ctx->data) {
    return;
  }
  sskdf_variant_hmac_ctx *variant_ctx = (sskdf_variant_hmac_ctx *)ctx->data;
  HMAC_CTX_free(variant_ctx->hmac_ctx);
  OPENSSL_free(variant_ctx);
  ctx->data = NULL;
}


static size_t sskdf_variant_hmac_output_size(sskdf_variant_ctx *ctx) {
  if (!ctx || !ctx->data) {
    return 0;
  }
  sskdf_variant_hmac_ctx *variant_ctx = (sskdf_variant_hmac_ctx *)ctx->data;
  if (!variant_ctx) {
    return 0;
  }
  return HMAC_size(variant_ctx->hmac_ctx);
}

static int sskdf_variant_hmac_compute(sskdf_variant_ctx *ctx, uint8_t *out,
                                      size_t out_len,
                                      const uint8_t counter[SSKDF_COUNTER_SIZE],
                                      const uint8_t *secret, size_t secret_len,
                                      const uint8_t *info, size_t info_len) {
  if (!ctx || !ctx->data || !out || !counter || !secret) {
    return 0;
  }

  sskdf_variant_hmac_ctx *variant_ctx = (sskdf_variant_hmac_ctx *)ctx->data;

  if (!variant_ctx->hmac_ctx) {
    return 0;
  }

  uint32_t written;

  // NIST.SP.800-56Cr2: Step 6.2 HMAC-hash(salt, counter || secret || info)
  // Note: |variant_ctx->hmac_ctx| is already initalized with the salt during
  // its initial construction.
  if (!HMAC_Init_ex(variant_ctx->hmac_ctx, NULL, 0, NULL, NULL) ||
      !HMAC_Update(variant_ctx->hmac_ctx, &counter[0], SSKDF_COUNTER_SIZE) ||
      !HMAC_Update(variant_ctx->hmac_ctx, secret, secret_len) ||
      !HMAC_Update(variant_ctx->hmac_ctx, info, info_len) ||
      !HMAC_Final(variant_ctx->hmac_ctx, out, &written) || out_len != written) {
    return 0;
  }

  return 1;
}

DEFINE_METHOD_FUNCTION(sskdf_variant, sskdf_variant_hmac) {
  out->h_output_bytes = sskdf_variant_hmac_output_size;
  out->compute = sskdf_variant_hmac_compute;
}

static int SSKDF(const sskdf_variant *variant, sskdf_variant_ctx *ctx,
                 uint8_t *out_key, size_t out_len, const uint8_t *secret,
                 size_t secret_len, const uint8_t *info, size_t info_len) {
  int ret = 0;

  if (!ctx || !variant) {
    return 0;
  }

  // The SSKDF_MAX_INPUT_LEN is an upper bound chosen for improved
  // interoperability with OpenSSL's SSKDF implementation. Additionally this
  // upper bound satisfies the NIST.SP.800-56Cr2 requirements outlined in table
  // 2 and 3 for digest and HMAC variants with approved hashes. The smallest max
  // limit imposed is (2^64-512)/8 bytes, and the limits used here places the
  // maximum allowed input to be (2^31 + 4). This satisfies the requirement
  // outlined in Step 4 of the specification.
  if (!out_key || out_len == 0 || out_len > SSKDF_MAX_INPUT_LEN || !secret ||
      secret_len == 0 || secret_len > SSKDF_MAX_INPUT_LEN ||
      info_len > SSKDF_MAX_INPUT_LEN) {
    goto err;
  }

  // h_output_bytes is the length in bytes of output of the SSKDF variant
  // auxilary function (EVP_DigestFinal or HMAC_Final)
  const size_t h_output_bytes = variant->h_output_bytes(ctx);
  if (h_output_bytes == 0 || h_output_bytes > EVP_MAX_MD_SIZE) {
    goto err;
  }

  // NIST.SP.800-56Cr2 Step 1:
  // Determine how many output chunks are required to produce the requested
  // output length |out_len|. This determines how many times the variant compute
  // function will be called to output key material.
  uint64_t n = ((uint64_t)out_len + (uint64_t)h_output_bytes - 1) /
               (uint64_t)h_output_bytes;

  // NIST.SP.800-56Cr2 Step 2:
  // Verify that the number of output chunks does not exceed an unsigned 32-bit
  // integer.
  if (n > UINT32_MAX) {
    goto err;
  }

  // TODO(awslc): Abstract buffer size, if we ever need to support KMAC this
  // could be variable. Currently sufficient for HMAC and digest variants
  uint8_t out_key_i[EVP_MAX_MD_SIZE];
  uint8_t counter[SSKDF_COUNTER_SIZE];
  size_t done = 0;
  size_t todo = h_output_bytes;

  // NIST.SP.800-56Cr2 Step 6
  for (uint32_t i = 0; i < n; i++) {
    // NIST.SP.800-56Cr2: Step 6.1
    // Increment the big-endian counter by one.
    CRYPTO_store_u32_be(&counter[0], i + 1);

    // NIST.SP.800-56Cr2: Step 6.2
    // Compute out_key_i = H(counter || secret || info)
    if (!variant->compute(ctx, &out_key_i[0], h_output_bytes, counter, secret,
                          secret_len, info, info_len)) {
      goto err;
    }

    // NIST.SP.800-56Cr2: Step 6.3. Step 7, Step 8
    // Combine the output from |out_key_i| with the output written to |out_key|
    // so far. Ensure that we only copy |out_len| bytes in total from all
    // chunks.
    todo = h_output_bytes;
    if (todo > out_len - done) {
      todo = out_len - done;
    }
    OPENSSL_memcpy(out_key + done, out_key_i, todo);
    done += todo;
  }

  ret = 1;

err:
  OPENSSL_cleanse(&out_key_i[0], EVP_MAX_MD_SIZE);
  if (ret <= 0 && out_key && out_len > 0) {
    OPENSSL_cleanse(out_key, out_len);
  }
  return ret;
}

int SSKDF_digest(uint8_t *out_key, size_t out_len, const EVP_MD *digest,
                 const uint8_t *secret, size_t secret_len, const uint8_t *info,
                 size_t info_len) {
  SET_DIT_AUTO_RESET;

  // We have to avoid the underlying |EVP_DigestFinal| services updating
  // the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  sskdf_variant_ctx ctx = {0};
  int ret = 0;

  if (!sskdf_variant_digest_ctx_init(&ctx, digest)) {
    FIPS_service_indicator_unlock_state();
    return 0;
  }

  if (!SSKDF(sskdf_variant_digest(), &ctx, out_key, out_len, secret, secret_len,
             info, info_len)) {
    goto end;
  }

  ret = 1;

end:
  sskdf_variant_digest_ctx_cleanup(&ctx);
  FIPS_service_indicator_unlock_state();
  if (ret) {
    SSKDF_digest_verify_service_indicator(digest);
  }
  return ret;
}

int SSKDF_hmac(uint8_t *out_key, size_t out_len, const EVP_MD *digest,
               const uint8_t *secret, size_t secret_len, const uint8_t *info,
               size_t info_len, const uint8_t *salt, size_t salt_len) {
  SET_DIT_AUTO_RESET;

  // We have to avoid the underlying |HMAC_Final| services updating
  // the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  sskdf_variant_ctx ctx = {0};
  int ret = 0;

  if (salt_len > SHRT_MAX) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_OVERFLOW);
    goto end;
  }

  if (!sskdf_variant_hmac_ctx_init(&ctx, digest, salt, salt_len)) {
    FIPS_service_indicator_unlock_state();
    return 0;
  }

  if (!SSKDF(sskdf_variant_hmac(), &ctx, out_key, out_len, secret, secret_len,
             info, info_len)) {
    goto end;
  }

  ret = 1;

end:
  sskdf_variant_hmac_ctx_cleanup(&ctx);
  FIPS_service_indicator_unlock_state();
  if(ret) {
    SSKDF_hmac_verify_service_indicator(digest);
  }
  return ret;
}
