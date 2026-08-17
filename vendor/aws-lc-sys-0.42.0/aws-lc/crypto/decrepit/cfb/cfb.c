// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/cipher.h>

#include <string.h>

#include <openssl/aes.h>
#include <openssl/obj.h>

#include "../../internal.h"
#include "../../fipsmodule/cipher/internal.h"

// MAXBITCHUNK is used in |aes_cfb1_cipher| to avoid overflow because
// |AES_cfb1_encrypt| operates data on bit level.
#define MAXBITCHUNK ((size_t)1<<(sizeof(size_t)*8-4))

typedef struct {
  AES_KEY ks;
} EVP_CFB_CTX;

static int aes_cfb_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                            const uint8_t *iv, int enc) {
  if (key) {
    EVP_CFB_CTX *cfb_ctx = (EVP_CFB_CTX *)ctx->cipher_data;
    AES_set_encrypt_key(key, ctx->key_len * 8, &cfb_ctx->ks);
  }

  return 1;
}

static int aes_cfb1_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out,
                            const uint8_t *in, size_t len) {
  if (!out || !in) {
    return 0;
  }

  EVP_CFB_CTX *cfb_ctx = (EVP_CFB_CTX *)ctx->cipher_data;
  if (ctx->flags & EVP_CIPH_FLAG_LENGTH_BITS) {
    int num = ctx->num;
    AES_cfb1_encrypt(in, out, len, &cfb_ctx->ks, ctx->iv, &num,
                      ctx->encrypt ? AES_ENCRYPT : AES_DECRYPT);
    ctx->num = num;
    return 1;
  }

  while (len >= MAXBITCHUNK) {
    int num = ctx->num;
    AES_cfb1_encrypt(in, out, MAXBITCHUNK * 8,  &cfb_ctx->ks, ctx->iv, &num,
                      ctx->encrypt ? AES_ENCRYPT : AES_DECRYPT);
    ctx->num = num;
    len -= MAXBITCHUNK;
    out += MAXBITCHUNK;
    in  += MAXBITCHUNK;
  }
  if (len) {
    int num = ctx->num;
    AES_cfb1_encrypt(in, out, len * 8, &cfb_ctx->ks, ctx->iv, &num,
                      ctx->encrypt ? AES_ENCRYPT : AES_DECRYPT);
    ctx->num = num;
  }

  return 1;
}

static int aes_cfb8_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out,
                            const uint8_t *in, size_t len) {
  if (!out || !in) {
    return 0;
  }

  EVP_CFB_CTX *cfb_ctx = (EVP_CFB_CTX *)ctx->cipher_data;
  int num = ctx->num;
  AES_cfb8_encrypt(in, out, len, &cfb_ctx->ks, ctx->iv, &num,
                     ctx->encrypt ? AES_ENCRYPT : AES_DECRYPT);
  ctx->num = num;

  return 1;
}

static int aes_cfb128_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out,
                             const uint8_t *in, size_t len) {
  if (!out || !in) {
    return 0;
  }

  EVP_CFB_CTX *cfb_ctx = (EVP_CFB_CTX *)ctx->cipher_data;
  int num = ctx->num;
  AES_cfb128_encrypt(in, out, len, &cfb_ctx->ks, ctx->iv, &num,
                     ctx->encrypt ? AES_ENCRYPT : AES_DECRYPT);
  ctx->num = num;

  return 1;
}

static const EVP_CIPHER aes_128_cfb1 = {
    NID_aes_128_cfb1, 1 /* block_size */,  16 /* key_size */,
    16 /* iv_len */,  sizeof(EVP_CFB_CTX), EVP_CIPH_CFB_MODE,
    aes_cfb_init_key, aes_cfb1_cipher,     NULL /* cleanup */,
    NULL /* ctrl */,
};

static const EVP_CIPHER aes_128_cfb8 = {
    NID_aes_128_cfb8, 1 /* block_size */,  16 /* key_size */,
    16 /* iv_len */,  sizeof(EVP_CFB_CTX), EVP_CIPH_CFB_MODE,
    aes_cfb_init_key, aes_cfb8_cipher,     NULL /* cleanup */,
    NULL /* ctrl */,
};

static const EVP_CIPHER aes_128_cfb128 = {
    NID_aes_128_cfb128, 1 /* block_size */,  16 /* key_size */,
    16 /* iv_len */,    sizeof(EVP_CFB_CTX), EVP_CIPH_CFB_MODE,
    aes_cfb_init_key,   aes_cfb128_cipher,   NULL /* cleanup */,
    NULL /* ctrl */,
};

static const EVP_CIPHER aes_192_cfb1 = {
    NID_aes_192_cfb1, 1 /* block_size */,  24 /* key_size */,
    16 /* iv_len */,  sizeof(EVP_CFB_CTX), EVP_CIPH_CFB_MODE,
    aes_cfb_init_key, aes_cfb1_cipher,     NULL /* cleanup */,
    NULL /* ctrl */,
};

static const EVP_CIPHER aes_192_cfb8 = {
    NID_aes_192_cfb8, 1 /* block_size */,  24 /* key_size */,
    16 /* iv_len */,  sizeof(EVP_CFB_CTX), EVP_CIPH_CFB_MODE,
    aes_cfb_init_key, aes_cfb8_cipher,     NULL /* cleanup */,
    NULL /* ctrl */,
};

static const EVP_CIPHER aes_192_cfb128 = {
    NID_aes_192_cfb128, 1 /* block_size */,  24 /* key_size */,
    16 /* iv_len */,    sizeof(EVP_CFB_CTX), EVP_CIPH_CFB_MODE,
    aes_cfb_init_key,   aes_cfb128_cipher,   NULL /* cleanup */,
    NULL /* ctrl */,
};

static const EVP_CIPHER aes_256_cfb1 = {
    NID_aes_256_cfb1, 1 /* block_size */,  32 /* key_size */,
    16 /* iv_len */,  sizeof(EVP_CFB_CTX), EVP_CIPH_CFB_MODE,
    aes_cfb_init_key, aes_cfb1_cipher,     NULL /* cleanup */,
    NULL /* ctrl */,
};

static const EVP_CIPHER aes_256_cfb8 = {
    NID_aes_256_cfb8,  1 /* block_size */,  32 /* key_size */,
    16 /* iv_len */,     sizeof(EVP_CFB_CTX), EVP_CIPH_CFB_MODE,
    aes_cfb_init_key,    aes_cfb8_cipher, NULL /* cleanup */,
    NULL /* ctrl */,
};

static const EVP_CIPHER aes_256_cfb128 = {
    NID_aes_256_cfb128,  1 /* block_size */,  32 /* key_size */,
    16 /* iv_len */,     sizeof(EVP_CFB_CTX), EVP_CIPH_CFB_MODE,
    aes_cfb_init_key,    aes_cfb128_cipher, NULL /* cleanup */,
    NULL /* ctrl */,
};

const EVP_CIPHER *EVP_aes_128_cfb1(void) { return &aes_128_cfb1; }
const EVP_CIPHER *EVP_aes_128_cfb8(void) { return &aes_128_cfb8; }
const EVP_CIPHER *EVP_aes_128_cfb128(void) { return &aes_128_cfb128; }
const EVP_CIPHER *EVP_aes_128_cfb(void) { return &aes_128_cfb128; }

const EVP_CIPHER *EVP_aes_192_cfb1(void) { return &aes_192_cfb1; }
const EVP_CIPHER *EVP_aes_192_cfb8(void) { return &aes_192_cfb8; }
const EVP_CIPHER *EVP_aes_192_cfb128(void) { return &aes_192_cfb128; }
const EVP_CIPHER *EVP_aes_192_cfb(void) { return &aes_192_cfb128; }

const EVP_CIPHER *EVP_aes_256_cfb1(void) { return &aes_256_cfb1; }
const EVP_CIPHER *EVP_aes_256_cfb8(void) { return &aes_256_cfb8; }
const EVP_CIPHER *EVP_aes_256_cfb128(void) { return &aes_256_cfb128; }
const EVP_CIPHER *EVP_aes_256_cfb(void) { return &aes_256_cfb128; }
