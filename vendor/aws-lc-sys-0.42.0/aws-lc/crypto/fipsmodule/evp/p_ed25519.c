// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/evp.h>

#include <openssl/curve25519.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "internal.h"
#include "../../evp_extra/internal.h"
#include "../curve25519/internal.h"


// Ed25519 has no parameters to copy.
static int pkey_ed25519_copy(EVP_PKEY_CTX *dst, EVP_PKEY_CTX *src) { return 1; }

static int pkey_ed25519_keygen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  ED25519_KEY *key = OPENSSL_malloc(sizeof(ED25519_KEY));
  if (key == NULL) {
    return 0;
  }

  uint8_t pubkey_unused[32];
  int result = ED25519_keypair_internal(pubkey_unused, key->key);
  if (result) {
    key->has_private = 1;
    evp_pkey_set0(pkey, &ed25519_asn1_meth, key);
  } else {
    OPENSSL_free(key);
  }

  return result;
}

static int pkey_ed25519_sign_message(EVP_PKEY_CTX *ctx, uint8_t *sig,
                                     size_t *siglen, const uint8_t *tbs,
                                     size_t tbslen) {
  ED25519_KEY *key = ctx->pkey->pkey.ptr;
  if (!key->has_private) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NOT_A_PRIVATE_KEY);
    return 0;
  }

  if (sig == NULL) {
    *siglen = 64;
    return 1;
  }

  if (*siglen < 64) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (!ED25519_sign(sig, tbs, tbslen, key->key)) {
    return 0;
  }

  *siglen = 64;
  return 1;
}

static int pkey_ed25519_verify_message(EVP_PKEY_CTX *ctx, const uint8_t *sig,
                                       size_t siglen, const uint8_t *tbs,
                                       size_t tbslen) {
  ED25519_KEY *key = ctx->pkey->pkey.ptr;
  if (siglen != 64 ||
      !ED25519_verify(tbs, tbslen, sig, key->key + ED25519_PUBLIC_KEY_OFFSET)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_SIGNATURE);
    return 0;
  }

  return 1;
}

DEFINE_METHOD_FUNCTION(EVP_PKEY_METHOD, EVP_PKEY_ed25519_pkey_meth) {
  out->pkey_id = EVP_PKEY_ED25519;
  out->init = NULL;
  out->copy = pkey_ed25519_copy;
  out->cleanup = NULL;
  out->keygen = pkey_ed25519_keygen;
  out->sign_init = NULL;
  out->sign = NULL;
  out->sign_message = pkey_ed25519_sign_message;
  out->verify_init = NULL;
  out->verify = NULL;
  out->verify_message = pkey_ed25519_verify_message;
  out->verify_recover = NULL;
  out->encrypt = NULL;
  out->decrypt = NULL;
  out->derive = NULL;
  out->paramgen = NULL;
  out->ctrl = NULL;
  out->ctrl_str = NULL;
  out->keygen_deterministic = NULL;
  out->encapsulate_deterministic = NULL;
  out->encapsulate = NULL;
  out->decapsulate = NULL;
}
