// Copyright (c) 2010 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/cmac.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/aes.h>
#include <openssl/cipher.h>
#include <openssl/mem.h>

#include "../../internal.h"


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
    case 16:
      cipher = EVP_aes_128_cbc();
      break;
    case 32:
      cipher = EVP_aes_256_cbc();
      break;
    default:
      return 0;
  }

  // We have to verify that all the CMAC services actually succeed before
  // updating the indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  
  size_t scratch_out_len;
  CMAC_CTX ctx;
  CMAC_CTX_init(&ctx);

  const int ok = CMAC_Init(&ctx, key, key_len, cipher, NULL /* engine */) &&
                 CMAC_Update(&ctx, in, in_len) &&
                 CMAC_Final(&ctx, out, &scratch_out_len);

  FIPS_service_indicator_unlock_state();
  if(ok) {
    AES_CMAC_verify_service_indicator(&ctx);
  }
  CMAC_CTX_cleanup(&ctx);
  return ok;
}

CMAC_CTX *CMAC_CTX_new(void) {
  CMAC_CTX *ctx = OPENSSL_zalloc(sizeof(*ctx));
  if (ctx != NULL) {
    // NO-OP: struct already zeroed
    //CMAC_CTX_init(ctx);
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
    out[i] = (in[i] << 1) | (in[i+1] >> 7);
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
    out[i] = (in[i] << 1) | (in[i+1] >> 7);
  }

  // If MSB set fixup with R.
  const uint8_t carry = in[0] >> 7;
  out[i] = (in[i] << 1) ^ ((0 - carry) & 0x1b);
}

static const uint8_t kZeroIV[AES_BLOCK_SIZE] = {0};

int CMAC_Init(CMAC_CTX *ctx, const void *key, size_t key_len,
              const EVP_CIPHER *cipher, ENGINE *engine) {
  // We have to avoid the underlying AES-CBC |EVP_CIPHER| services updating the
  // indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  int ret = 0;
  uint8_t scratch[AES_BLOCK_SIZE];

  size_t block_size = EVP_CIPHER_block_size(cipher);
  if ((block_size != AES_BLOCK_SIZE && block_size != 8 /* 3-DES */) ||
      EVP_CIPHER_key_length(cipher) != key_len ||
      !EVP_EncryptInit_ex(&ctx->cipher_ctx, cipher, NULL, key, kZeroIV) ||
      !EVP_Cipher(&ctx->cipher_ctx, scratch, kZeroIV, block_size) ||
      // Reset context again ready for first data.
      !EVP_EncryptInit_ex(&ctx->cipher_ctx, NULL, NULL, NULL, kZeroIV)) {
    goto end;
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

end:
  FIPS_service_indicator_unlock_state();
  return ret;
}

int CMAC_Reset(CMAC_CTX *ctx) {
  ctx->block_used = 0;
  return EVP_EncryptInit_ex(&ctx->cipher_ctx, NULL, NULL, NULL, kZeroIV);
}

int CMAC_Update(CMAC_CTX *ctx, const uint8_t *in, size_t in_len) {
  // We have to avoid the underlying AES-CBC |EVP_Cipher| services updating the
  // indicator state, so we lock the state here.
  int ret = 0;
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
      goto end;
    }

    assert(ctx->block_used == block_size);

    if (!EVP_Cipher(&ctx->cipher_ctx, scratch, ctx->block, block_size)) {
      goto end;
    }
  }

  // Encrypt all but one of the remaining blocks.
  while (in_len > block_size) {
    if (!EVP_Cipher(&ctx->cipher_ctx, scratch, in, block_size)) {
      goto end;
    }
    in += block_size;
    in_len -= block_size;
  }

  OPENSSL_memcpy(ctx->block, in, in_len);
  // |in_len| is bounded by |block_size|, which fits in |unsigned|.
  OPENSSL_STATIC_ASSERT(EVP_MAX_BLOCK_LENGTH < UINT_MAX,
                EVP_MAX_BLOCK_LENGTH_is_too_large);
  ctx->block_used = (unsigned)in_len;
  ret = 1;

end:
  FIPS_service_indicator_unlock_state();
  return ret;
}

int CMAC_Final(CMAC_CTX *ctx, uint8_t *out, size_t *out_len) {
  // We have to avoid the underlying AES-CBC |EVP_Cipher| services updating the
  // indicator state, so we lock the state here.
  FIPS_service_indicator_lock_state();
  int ret = 0;
  size_t block_size = EVP_CIPHER_CTX_block_size(&ctx->cipher_ctx);
  assert(block_size <= AES_BLOCK_SIZE);

  *out_len = block_size;
  if (out == NULL) {
    ret = 1;
    goto end;
  }

  const uint8_t *mask = ctx->k1;

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

end:
  FIPS_service_indicator_unlock_state();
  if(ret) {
    AES_CMAC_verify_service_indicator(ctx);
  }
  return ret;
}

EVP_CIPHER_CTX *CMAC_CTX_get0_cipher_ctx(CMAC_CTX *ctx) {
  return &ctx->cipher_ctx;
}
