// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2007.
// Copyright (c) 2007 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0


#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/hmac.h>
#include <openssl/mem.h>

#include "../../evp_extra/internal.h"
#include "internal.h"

static int hmac_init(EVP_PKEY_CTX *ctx) {
  HMAC_PKEY_CTX *hctx;
  hctx = OPENSSL_zalloc(sizeof(HMAC_PKEY_CTX));
  if (hctx == NULL) {
    return 0;
  }
  HMAC_CTX_init(&hctx->ctx);
  ctx->data = hctx;
  return 1;
}

static int hmac_copy(EVP_PKEY_CTX *dst, EVP_PKEY_CTX *src) {
  HMAC_PKEY_CTX *sctx, *dctx;
  if (!hmac_init(dst)) {
    return 0;
  }
  sctx = src->data;
  dctx = dst->data;
  dctx->md = sctx->md;
  if(sctx->ktmp.key != NULL && !HMAC_KEY_copy(&dctx->ktmp, &sctx->ktmp)) {
    HMAC_CTX_cleanup(&dctx->ctx);
    OPENSSL_free(dctx);
    dst->data = NULL;
    return 0;
  }
  if (!HMAC_CTX_copy_ex(&dctx->ctx, &sctx->ctx)) {
    HMAC_CTX_cleanup(&dctx->ctx);
    OPENSSL_free(dctx->ktmp.key);
    OPENSSL_free(dctx);
    dst->data = NULL;
    return 0;
  }
  return 1;
}

static void hmac_cleanup(EVP_PKEY_CTX *ctx) {
  HMAC_PKEY_CTX *hctx = ctx->data;
  if (hctx == NULL) {
    return;
  }
  HMAC_CTX_cleanup(&hctx->ctx);
  OPENSSL_free(hctx->ktmp.key);
  OPENSSL_free(hctx);
  ctx->data = NULL;
}

static int hmac_ctrl(EVP_PKEY_CTX *ctx, int cmd, int p1, void *p2) {
  int result = -2;

  HMAC_PKEY_CTX *hctx = ctx->data;
  switch (cmd) {
    case EVP_PKEY_CTRL_SET_MAC_KEY:
      if (p1 >= 0 && p1 <= INT16_MAX) {
        // p1 is the key length
        // p2 is the key
        if (HMAC_KEY_set(&hctx->ktmp, p2, p1)) {
          result = 1;
        } else {
          result = 0;
        }
      }
      break;
    case EVP_PKEY_CTRL_MD:
      hctx->md = p2;
      result = 1;
      break;
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
  }
  return result;
}

static int hmac_ctrl_str(EVP_PKEY_CTX *ctx, const char *type,
                         const char *value) {
  if (!value) {
    return 0;
  }
  if (strcmp(type, "key") == 0) {
    // What if the key contains a 0-byte?
    const size_t keylen = OPENSSL_strnlen(value, INT16_MAX);
    return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_HMAC, EVP_PKEY_OP_KEYGEN,
                             EVP_PKEY_CTRL_SET_MAC_KEY, keylen, (void*)value);
  }
  if (strcmp(type, "hexkey") == 0) {
    size_t hex_keylen = 0;
    uint8_t *key = OPENSSL_hexstr2buf(value, &hex_keylen);
    if (key == NULL) {
      return 0;
    }
    int result =
        EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_HMAC, EVP_PKEY_OP_KEYGEN,
                          EVP_PKEY_CTRL_SET_MAC_KEY, hex_keylen, key);
    OPENSSL_free(key);
    return result;
  }
  return -2;
}

static int hmac_keygen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  GUARD_PTR(pkey);
  HMAC_KEY *hmac = NULL;
  HMAC_PKEY_CTX *hctx = ctx->data;
  if(hctx == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }

  if (!(hmac = HMAC_KEY_new())) {
      return 0;
  }

  if (!HMAC_KEY_copy(hmac, &hctx->ktmp) ||
      !EVP_PKEY_assign(pkey, EVP_PKEY_HMAC, hmac)) {
    OPENSSL_free(hmac->key);
    OPENSSL_free(hmac);
    return 0;
  }

  return 1;
}

DEFINE_METHOD_FUNCTION(EVP_PKEY_METHOD, EVP_PKEY_hmac_pkey_meth) {
  out->pkey_id = EVP_PKEY_HMAC;
  out->init = hmac_init;
  out->copy = hmac_copy;
  out->cleanup = hmac_cleanup;
  out->keygen = hmac_keygen;
  out->ctrl = hmac_ctrl;
  out->ctrl_str = hmac_ctrl_str;
}

int used_for_hmac(EVP_MD_CTX *ctx) {
  return ctx->flags == EVP_MD_CTX_HMAC && ctx->pctx != NULL;
}

HMAC_KEY *HMAC_KEY_new(void) {
  HMAC_KEY *key = OPENSSL_zalloc(sizeof(HMAC_KEY));
  if (key == NULL) {
    return NULL;
  }
  return key;
}

int HMAC_KEY_set(HMAC_KEY* hmac_key, const uint8_t* key, const size_t key_len) {
  if(hmac_key == NULL ) {
    return 0;
  }
  if (key == NULL || key_len == 0) {
    hmac_key->key = NULL;
    hmac_key->key_len = 0;
    return 1;
  }

  uint8_t* new_key = OPENSSL_memdup(key, key_len);
  if(new_key == NULL) {
    return 0;
  }
  OPENSSL_free(hmac_key->key);
  hmac_key->key = new_key;
  hmac_key->key_len = key_len;
  return 1;
}

int HMAC_KEY_copy(HMAC_KEY* dest, HMAC_KEY* src) {
  GUARD_PTR(dest);
  GUARD_PTR(src);

  return HMAC_KEY_set(dest, src->key, src->key_len);
}
