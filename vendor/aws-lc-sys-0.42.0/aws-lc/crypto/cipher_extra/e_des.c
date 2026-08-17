// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/cipher.h>
#include <openssl/des.h>
#include <openssl/nid.h>

#include "../des/internal.h"
#include "../fipsmodule/cipher/internal.h"
#include "internal.h"


typedef struct {
  union {
    double align;
    DES_key_schedule ks;
  } ks;
} EVP_DES_KEY;

static int des_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                        const uint8_t *iv, int enc) {
  EVP_DES_KEY *dat = (EVP_DES_KEY *)ctx->cipher_data;
  DES_set_key_ex(key, &dat->ks.ks);
  return 1;
}

static int des_cbc_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                          size_t in_len) {
  EVP_DES_KEY *dat = (EVP_DES_KEY *)ctx->cipher_data;
  DES_ncbc_encrypt_ex(in, out, in_len, &dat->ks.ks, ctx->iv, ctx->encrypt);
  return 1;
}

static const EVP_CIPHER evp_des_cbc = {
    .nid = NID_des_cbc,
    .block_size = 8,
    .key_len = 8,
    .iv_len = 8,
    .ctx_size = sizeof(EVP_DES_KEY),
    .flags = EVP_CIPH_CBC_MODE,
    .init = des_init_key,
    .cipher = des_cbc_cipher,
};

const EVP_CIPHER *EVP_des_cbc(void) { return &evp_des_cbc; }

static int des_ecb_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out, const uint8_t *in,
                          size_t in_len) {
  if (in_len < ctx->cipher->block_size) {
    return 1;
  }
  in_len -= ctx->cipher->block_size;

  EVP_DES_KEY *dat = (EVP_DES_KEY *)ctx->cipher_data;
  for (size_t i = 0; i <= in_len; i += ctx->cipher->block_size) {
    DES_ecb_encrypt_ex(in + i, out + i, &dat->ks.ks, ctx->encrypt);
  }
  return 1;
}

static const EVP_CIPHER evp_des_ecb = {
    .nid = NID_des_ecb,
    .block_size = 8,
    .key_len = 8,
    .iv_len = 0,
    .ctx_size = sizeof(EVP_DES_KEY),
    .flags = EVP_CIPH_ECB_MODE,
    .init = des_init_key,
    .cipher = des_ecb_cipher,
};

const EVP_CIPHER *EVP_des_ecb(void) { return &evp_des_ecb; }

typedef struct {
  union {
    double align;
    DES_key_schedule ks[3];
  } ks;
} DES_EDE_KEY;

static int des_ede3_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                             const uint8_t *iv, int enc) {
  DES_EDE_KEY *dat = (DES_EDE_KEY *)ctx->cipher_data;
  DES_set_key_ex(key, &dat->ks.ks[0]);
  DES_set_key_ex(key + 8, &dat->ks.ks[1]);
  DES_set_key_ex(key + 16, &dat->ks.ks[2]);
  return 1;
}

static int des_ede3_cbc_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out,
                               const uint8_t *in, size_t in_len) {
  DES_EDE_KEY *dat = (DES_EDE_KEY *)ctx->cipher_data;
  DES_ede3_cbc_encrypt_ex(in, out, in_len, &dat->ks.ks[0], &dat->ks.ks[1],
                          &dat->ks.ks[2], ctx->iv, ctx->encrypt);
  return 1;
}

static const EVP_CIPHER evp_des_ede3_cbc = {
    .nid = NID_des_ede3_cbc,
    .block_size = 8,
    .key_len = 24,
    .iv_len = 8,
    .ctx_size = sizeof(DES_EDE_KEY),
    .flags = EVP_CIPH_CBC_MODE,
    .init = des_ede3_init_key,
    .cipher = des_ede3_cbc_cipher,
};

const EVP_CIPHER *EVP_des_ede3_cbc(void) { return &evp_des_ede3_cbc; }

static int des_ede_init_key(EVP_CIPHER_CTX *ctx, const uint8_t *key,
                            const uint8_t *iv, int enc) {
  DES_EDE_KEY *dat = (DES_EDE_KEY *)ctx->cipher_data;
  // 2-DES is 3-DES with the first key used twice.
  DES_set_key_ex(key, &dat->ks.ks[0]);
  DES_set_key_ex(key + 8, &dat->ks.ks[1]);
  DES_set_key_ex(key, &dat->ks.ks[2]);
  return 1;
}

static const EVP_CIPHER evp_des_ede_cbc = {
    .nid = NID_des_ede_cbc,
    .block_size = 8,
    .key_len = 16,
    .iv_len = 8,
    .ctx_size = sizeof(DES_EDE_KEY),
    .flags = EVP_CIPH_CBC_MODE,
    .init = des_ede_init_key,
    .cipher = des_ede3_cbc_cipher,
};

const EVP_CIPHER *EVP_des_ede_cbc(void) { return &evp_des_ede_cbc; }

static int des_ede_ecb_cipher(EVP_CIPHER_CTX *ctx, uint8_t *out,
                              const uint8_t *in, size_t in_len) {
  if (in_len < ctx->cipher->block_size) {
    return 1;
  }
  in_len -= ctx->cipher->block_size;

  DES_EDE_KEY *dat = (DES_EDE_KEY *) ctx->cipher_data;
  for (size_t i = 0; i <= in_len; i += ctx->cipher->block_size) {
    DES_ecb3_encrypt_ex(in + i, out + i, &dat->ks.ks[0], &dat->ks.ks[1],
                        &dat->ks.ks[2], ctx->encrypt);
  }
  return 1;
}

static const EVP_CIPHER evp_des_ede = {
    .nid = NID_des_ede_ecb,
    .block_size = 8,
    .key_len = 16,
    .iv_len = 0,
    .ctx_size = sizeof(DES_EDE_KEY),
    .flags = EVP_CIPH_ECB_MODE,
    .init = des_ede_init_key,
    .cipher = des_ede_ecb_cipher,
};

const EVP_CIPHER *EVP_des_ede(void) { return &evp_des_ede; }

static const EVP_CIPHER evp_des_ede3 = {
    .nid = NID_des_ede3_ecb,
    .block_size = 8,
    .key_len = 24,
    .iv_len = 0,
    .ctx_size = sizeof(DES_EDE_KEY),
    .flags = EVP_CIPH_ECB_MODE,
    .init = des_ede3_init_key,
    .cipher = des_ede_ecb_cipher,
};

const EVP_CIPHER *EVP_des_ede3(void) { return &evp_des_ede3; }

const EVP_CIPHER *EVP_des_ede3_ecb(void) { return EVP_des_ede3(); }
