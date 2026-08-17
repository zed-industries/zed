// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef OPENSSL_HEADER_KDF_INTERNAL_H
#define OPENSSL_HEADER_KDF_INTERNAL_H

#include <openssl/digest.h>
#include <openssl/hmac.h>

#define SSKDF_MAX_INPUT_LEN (1 << 30)
#define SSKDF_COUNTER_SIZE 4

#define KBKDF_COUNTER_SIZE 4

typedef struct {
  void *data;
} sskdf_variant_ctx;

typedef struct {
  const EVP_MD *digest;
  EVP_MD_CTX *md_ctx;
} sskdf_variant_digest_ctx;

typedef struct {
  HMAC_CTX *hmac_ctx;
} sskdf_variant_hmac_ctx;

typedef struct {
  size_t (*h_output_bytes)(sskdf_variant_ctx *ctx);
  int (*compute)(sskdf_variant_ctx *ctx, uint8_t *out, size_t out_len,
                 const uint8_t counter[SSKDF_COUNTER_SIZE],
                 const uint8_t *secret, size_t secret_len, const uint8_t *info,
                 size_t info_len);
} sskdf_variant;

const sskdf_variant *sskdf_variant_digest(void);

const sskdf_variant *sskdf_variant_hmac(void);

#endif  // OPENSSL_HEADER_KDF_INTERNAL_H
