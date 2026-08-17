// Copyright 2010-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <openssl/cmac.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/aes.h>
#include <openssl/cipher.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "../service_indicator/internal.h"


struct cmac_ctx_st {
  EVP_CIPHER_CTX cipher_ctx;
  // k1 and k2 are the CMAC subkeys. See
  // https://tools.ietf.org/html/rfc4493#section-2.3
  uint8_t k1[AES_BLOCK_SIZE];
  uint8_t k2[AES_BLOCK_SIZE];
  // Last (possibly partial) scratch
  uint8_t block[AES_BLOCK_SIZE];
  // block_used contains the number of valid bytes in |block|.
  unsigned block_used;
};

static void CMAC_CTX_init(CMAC_CTX *ctx) {
  EVP_CIPHER_CTX_init(&ctx->cipher_ctx);
}

static void CMAC_CTX_cleanup(CMAC_CTX *ctx) {
  EVP_CIPHER_CTX_cleanup(&ctx->cipher_ctx);
  OPENSSL_cleanse(ctx->k1, sizeof(ctx->k1));
  OPENSSL_cleanse(ctx->k2, sizeof(ctx->k2));
  OPENSSL_cleanse(ctx->block, sizeof(ctx->block));
}

int AES_CMAC(uint8_t out[16], const uint8_t *key, size_t key_len,
             const uint8_t *in, size_t in_len) {
  const EVP_CIPHER *cipher;
  switch (key_len) {
    // WARNING: this code assumes that all supported key sizes are FIPS
    // Approved.
    case 16:
      cipher = EVP_aes_128_cbc();
      break;
    case 32:
      cipher = EVP_aes_256_cbc();
      break;
    default:
      return 0;
  }

  size_t scratch_out_len;
  CMAC_CTX ctx;
  CMAC_CTX_init(&ctx);

  // We have to verify that all the CMAC services actually succeed before
  // updating the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  const int ok = CMAC_Init(&ctx, key, key_len, cipher, NULL /* engine */) &&
                 CMAC_Update(&ctx, in, in_len) &&
                 CMAC_Final(&ctx, out, &scratch_out_len);
  FIPS_service_indicator_unlock_state();

  if (ok) {
    FIPS_service_indicator_update_state();
  }
  CMAC_CTX_cleanup(&ctx);
  return ok;
}

CMAC_CTX *CMAC_CTX_new(void) {
  CMAC_CTX *ctx = reinterpret_cast<CMAC_CTX *>(OPENSSL_malloc(sizeof(*ctx)));
  if (ctx != NULL) {
    CMAC_CTX_init(ctx);
  }
  return ctx;
}

void CMAC_CTX_free(CMAC_CTX *ctx) {
  if (ctx == NULL) {
    return;
  }

  CMAC_CTX_cleanup(ctx);
  OPENSSL_free(ctx);
}

int CMAC_CTX_copy(CMAC_CTX *out, const CMAC_CTX *in) {
  if (!EVP_CIPHER_CTX_copy(&out->cipher_ctx, &in->cipher_ctx)) {
    return 0;
  }
  OPENSSL_memcpy(out->k1, in->k1, AES_BLOCK_SIZE);
  OPENSSL_memcpy(out->k2, in->k2, AES_BLOCK_SIZE);
  OPENSSL_memcpy(out->block, in->block, AES_BLOCK_SIZE);
  out->block_used = in->block_used;
  return 1;
}

// binary_field_mul_x_128 treats the 128 bits at |in| as an element of GF(2¹²⁸)
// with a hard-coded reduction polynomial and sets |out| as x times the input.
//
// See https://tools.ietf.org/html/rfc4493#section-2.3
static void binary_field_mul_x_128(uint8_t out[16], const uint8_t in[16]) {
  unsigned i;

  // Shift |in| to left, including carry.
  for (i = 0; i < 15; i++) {
    out[i] = (in[i] << 1) | (in[i + 1] >> 7);
  }

  // If MSB set fixup with R.
  const uint8_t carry = in[0] >> 7;
  out[i] = (in[i] << 1) ^ ((0 - carry) & 0x87);
}

// binary_field_mul_x_64 behaves like |binary_field_mul_x_128| but acts on an
// element of GF(2⁶⁴).
//
// See https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-38b.pdf
static void binary_field_mul_x_64(uint8_t out[8], const uint8_t in[8]) {
  unsigned i;

  // Shift |in| to left, including carry.
  for (i = 0; i < 7; i++) {
    out[i] = (in[i] << 1) | (in[i + 1] >> 7);
  }

  // If MSB set fixup with R.
  const uint8_t carry = in[0] >> 7;
  out[i] = (in[i] << 1) ^ ((0 - carry) & 0x1b);
}

static const uint8_t kZeroIV[AES_BLOCK_SIZE] = {0};

int CMAC_Init(CMAC_CTX *ctx, const void *key, size_t key_len,
              const EVP_CIPHER *cipher, ENGINE *engine) {
  int ret = 0;
  uint8_t scratch[AES_BLOCK_SIZE];

  // We have to avoid the underlying AES-CBC |EVP_CIPHER| services updating the
  // indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  size_t block_size = EVP_CIPHER_block_size(cipher);
  if ((block_size != AES_BLOCK_SIZE && block_size != 8 /* 3-DES */) ||
      EVP_CIPHER_key_length(cipher) != key_len ||
      !EVP_EncryptInit_ex(&ctx->cipher_ctx, cipher, NULL,
                          reinterpret_cast<const uint8_t *>(key), kZeroIV) ||
      !EVP_Cipher(&ctx->cipher_ctx, scratch, kZeroIV, block_size) ||
      // Reset context again ready for first data.
      !EVP_EncryptInit_ex(&ctx->cipher_ctx, NULL, NULL, NULL, kZeroIV)) {
    goto out;
  }

  if (block_size == AES_BLOCK_SIZE) {
    binary_field_mul_x_128(ctx->k1, scratch);
    binary_field_mul_x_128(ctx->k2, ctx->k1);
  } else {
    binary_field_mul_x_64(ctx->k1, scratch);
    binary_field_mul_x_64(ctx->k2, ctx->k1);
  }
  ctx->block_used = 0;
  ret = 1;

out:
  FIPS_service_indicator_unlock_state();
  return ret;
}

int CMAC_Reset(CMAC_CTX *ctx) {
  ctx->block_used = 0;
  return EVP_EncryptInit_ex(&ctx->cipher_ctx, NULL, NULL, NULL, kZeroIV);
}

int CMAC_Update(CMAC_CTX *ctx, const uint8_t *in, size_t in_len) {
  int ret = 0;

  // We have to avoid the underlying AES-CBC |EVP_Cipher| services updating the
  // indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  size_t block_size = EVP_CIPHER_CTX_block_size(&ctx->cipher_ctx);
  assert(block_size <= AES_BLOCK_SIZE);
  uint8_t scratch[AES_BLOCK_SIZE];

  if (ctx->block_used > 0) {
    size_t todo = block_size - ctx->block_used;
    if (in_len < todo) {
      todo = in_len;
    }

    OPENSSL_memcpy(ctx->block + ctx->block_used, in, todo);
    in += todo;
    in_len -= todo;
    ctx->block_used += todo;

    // If |in_len| is zero then either |ctx->block_used| is less than
    // |block_size|, in which case we can stop here, or |ctx->block_used| is
    // exactly |block_size| but there's no more data to process. In the latter
    // case we don't want to process this block now because it might be the last
    // block and that block is treated specially.
    if (in_len == 0) {
      ret = 1;
      goto out;
    }

    assert(ctx->block_used == block_size);

    if (!EVP_Cipher(&ctx->cipher_ctx, scratch, ctx->block, block_size)) {
      goto out;
    }
  }

  // Encrypt all but one of the remaining blocks.
  while (in_len > block_size) {
    if (!EVP_Cipher(&ctx->cipher_ctx, scratch, in, block_size)) {
      goto out;
    }
    in += block_size;
    in_len -= block_size;
  }

  OPENSSL_memcpy(ctx->block, in, in_len);
  // |in_len| is bounded by |block_size|, which fits in |unsigned|.
  static_assert(EVP_MAX_BLOCK_LENGTH < UINT_MAX,
                "EVP_MAX_BLOCK_LENGTH is too large");
  ctx->block_used = (unsigned)in_len;
  ret = 1;

out:
  FIPS_service_indicator_unlock_state();
  return ret;
}

int CMAC_Final(CMAC_CTX *ctx, uint8_t *out, size_t *out_len) {
  int ret = 0;
  size_t block_size = EVP_CIPHER_CTX_block_size(&ctx->cipher_ctx);
  assert(block_size <= AES_BLOCK_SIZE);

  // We have to avoid the underlying AES-CBC |EVP_Cipher| services updating the
  // indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  *out_len = block_size;
  const uint8_t *mask = ctx->k1;
  if (out == NULL) {
    ret = 1;
    goto out;
  }

  if (ctx->block_used != block_size) {
    // If the last block is incomplete, terminate it with a single 'one' bit
    // followed by zeros.
    ctx->block[ctx->block_used] = 0x80;
    OPENSSL_memset(ctx->block + ctx->block_used + 1, 0,
                   block_size - (ctx->block_used + 1));

    mask = ctx->k2;
  }

  for (unsigned i = 0; i < block_size; i++) {
    out[i] = ctx->block[i] ^ mask[i];
  }
  ret = EVP_Cipher(&ctx->cipher_ctx, out, out, block_size);

out:
  FIPS_service_indicator_unlock_state();
  if (ret) {
    FIPS_service_indicator_update_state();
  }
  return ret;
}
