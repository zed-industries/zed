// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/evp.h>

#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/rand.h>

#include "../crypto/evp_extra/internal.h"
#include "internal.h"
#include "../delocate.h"
#include "../kem/internal.h"
#include "../../internal.h"
#include "internal.h"

typedef struct {
  const KEM *kem;
} KEM_PKEY_CTX;

static int pkey_kem_init(EVP_PKEY_CTX *ctx) {
  KEM_PKEY_CTX *dctx;
  dctx = OPENSSL_zalloc(sizeof(KEM_PKEY_CTX));
  if (dctx == NULL) {
    return 0;
  }

  ctx->data = dctx;

  return 1;
}

static void pkey_kem_cleanup(EVP_PKEY_CTX *ctx) {
  OPENSSL_free(ctx->data);
}

static int pkey_kem_keygen_deterministic(EVP_PKEY_CTX *ctx,
                                         EVP_PKEY *pkey,
                                         const uint8_t *seed,
                                         size_t *seed_len) {
  GUARD_PTR(ctx);
  KEM_PKEY_CTX *dctx = ctx->data;
  GUARD_PTR(dctx);
  const KEM *kem = dctx->kem;
  if (kem == NULL) {
    if (ctx->pkey == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
      return 0;
    }
    kem = KEM_KEY_get0_kem(ctx->pkey->pkey.kem_key);
  }

  // Check that size buffers can be written to.
  if (seed_len == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  // Caller is getting parameter values.
  if (seed == NULL) {
    *seed_len = kem->keygen_seed_len;
    return 1;
  }

  // The seed should be of the correct length.
  if (*seed_len != kem->keygen_seed_len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
    return 0;
  }

  KEM_KEY *key = KEM_KEY_new();
  size_t pubkey_len = kem->public_key_len;
  size_t secret_len = kem->secret_key_len;
  if (key == NULL ||
      !KEM_KEY_init(key, kem) ||
      !kem->method->keygen_deterministic(key->public_key, &pubkey_len, key->secret_key, &secret_len, seed)) {
    KEM_KEY_free(key);
    return 0;
  }

  key->seed = OPENSSL_memdup(seed, kem->keygen_seed_len);
  if (key->seed == NULL ||
      !EVP_PKEY_assign(pkey, EVP_PKEY_KEM, key)) {
    KEM_KEY_free(key);
    return 0;
  }

  return 1;
}

static int pkey_kem_keygen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  GUARD_PTR(ctx);
  KEM_PKEY_CTX *dctx = ctx->data;
  GUARD_PTR(dctx);
  const KEM *kem = dctx->kem;
  if (kem == NULL) {
    if (ctx->pkey == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
      return 0;
    }
    kem = KEM_KEY_get0_kem(ctx->pkey->pkey.kem_key);
  }

  KEM_KEY *key = KEM_KEY_new();
  size_t pubkey_len = kem->public_key_len;
  size_t secret_len = kem->secret_key_len;

  // Generate random seed, use deterministic keygen, and save the seed.
  // All ML-KEM variants use 64-byte seeds (d || z).
  uint8_t seed[64];
  BSSL_CHECK(kem->keygen_seed_len == 64);
  RAND_bytes(seed, kem->keygen_seed_len);

  if (key == NULL ||
      !KEM_KEY_init(key, kem) ||
      !kem->method->keygen_deterministic(key->public_key, &pubkey_len,
                                         key->secret_key, &secret_len, seed)) {
    OPENSSL_cleanse(seed, sizeof(seed));
    KEM_KEY_free(key);
    return 0;
  }

  key->seed = OPENSSL_memdup(seed, kem->keygen_seed_len);
  OPENSSL_cleanse(seed, sizeof(seed));
  if (key->seed == NULL ||
      !EVP_PKEY_set_type(pkey, EVP_PKEY_KEM)) {
    KEM_KEY_free(key);
    return 0;
  }
  pkey->pkey.kem_key = key;

  return 1;
}

static int pkey_kem_encapsulate_deterministic(EVP_PKEY_CTX *ctx,
                                              uint8_t *ciphertext,
                                              size_t  *ciphertext_len,
                                              uint8_t *shared_secret,
                                              size_t  *shared_secret_len,
                                              const uint8_t *seed,
                                              size_t *seed_len) {
  GUARD_PTR(ctx);
  KEM_PKEY_CTX *dctx = ctx->data;
  GUARD_PTR(dctx);
  const KEM *kem = dctx->kem;
  if (kem == NULL) {
    if (ctx->pkey == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
      return 0;
    }
    kem = KEM_KEY_get0_kem(ctx->pkey->pkey.kem_key);
  }

  // Check that size buffers can be written to.
  if (ciphertext_len == NULL || shared_secret_len == NULL || seed_len == NULL ) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  // Caller is getting parameter values.
  if (ciphertext == NULL && shared_secret == NULL && seed == NULL) {
    *ciphertext_len = kem->ciphertext_len;
    *shared_secret_len = kem->shared_secret_len;
    *seed_len = kem->encaps_seed_len;
    return 1;
  }

  // If not getting parameter values, then all three
  // output buffers need to be valid (non-NULL)
  if (ciphertext == NULL || shared_secret == NULL || seed == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_MISSING_PARAMETERS);
    return 0;
  }

  // The output buffers need to be large enough.
  if (*ciphertext_len < kem->ciphertext_len ||
      *shared_secret_len < kem->shared_secret_len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  // The seed should be of the correct length.
  if (*seed_len != kem->encaps_seed_len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
    return 0;
  }

  // Check that the context is properly configured.
  if (ctx->pkey == NULL ||
      ctx->pkey->pkey.kem_key == NULL ||
      ctx->pkey->type != EVP_PKEY_KEM) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }

  // Check that the key has a public key set.
  KEM_KEY *key = ctx->pkey->pkey.kem_key;
  if (key->public_key == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }

  if (!kem->method->encaps_deterministic(ciphertext, ciphertext_len, shared_secret, shared_secret_len, key->public_key, seed)) {
    return 0;
  }

  // The size of the ciphertext and the shared secret
  // that has been writen to the output buffers.
  *ciphertext_len = kem->ciphertext_len;
  *shared_secret_len = kem->shared_secret_len;

  return 1;
}

static int pkey_kem_encapsulate(EVP_PKEY_CTX *ctx,
                                uint8_t *ciphertext,
                                size_t  *ciphertext_len,
                                uint8_t *shared_secret,
                                size_t  *shared_secret_len) {
  GUARD_PTR(ctx);
  KEM_PKEY_CTX *dctx = ctx->data;
  GUARD_PTR(dctx);
  const KEM *kem = dctx->kem;
  if (kem == NULL) {
    if (ctx->pkey == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
      return 0;
    }
    kem = KEM_KEY_get0_kem(ctx->pkey->pkey.kem_key);
  }

  // Check that length pointers can be written to.
  if (ciphertext_len == NULL || shared_secret_len == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  // Caller is getting parameter values.
  if (ciphertext == NULL && shared_secret == NULL) {
    *ciphertext_len = kem->ciphertext_len;
    *shared_secret_len = kem->shared_secret_len;
    return 1;
  }

  // If not getting parameter values, then both
  // output buffers need to be valid (non-NULL)
  if (ciphertext == NULL || shared_secret == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_MISSING_PARAMETERS);
    return 0;
  }

  // The output buffers need to be large enough.
  if (*ciphertext_len < kem->ciphertext_len ||
      *shared_secret_len < kem->shared_secret_len) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
      return 0;
  }

  // Check that the context is properly configured.
  if (ctx->pkey == NULL ||
      ctx->pkey->pkey.kem_key == NULL ||
      ctx->pkey->type != EVP_PKEY_KEM) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
      return 0;
  }

  // Check that the key has a public key set.
  KEM_KEY *key = ctx->pkey->pkey.kem_key;
  if (key->public_key == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }

  if (!kem->method->encaps(ciphertext, ciphertext_len, shared_secret, shared_secret_len, key->public_key)) {
    return 0;
  }

  // The size of the ciphertext and the shared secret
  // that has been writen to the output buffers.
  *ciphertext_len = kem->ciphertext_len;
  *shared_secret_len = kem->shared_secret_len;

  return 1;
}

static int pkey_kem_decapsulate(EVP_PKEY_CTX *ctx,
                                uint8_t *shared_secret,
                                size_t  *shared_secret_len,
                                const uint8_t *ciphertext,
                                size_t ciphertext_len) {
  GUARD_PTR(ctx);
  KEM_PKEY_CTX *dctx = ctx->data;
  GUARD_PTR(dctx);
  const KEM *kem = dctx->kem;
  if (kem == NULL) {
    if (ctx->pkey == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
      return 0;
    }
    kem = KEM_KEY_get0_kem(ctx->pkey->pkey.kem_key);
  }

  // Check that the length pointer can be written to.
  if (shared_secret_len == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  // Caller is getting parameter values.
  if (shared_secret == NULL) {
    *shared_secret_len = kem->shared_secret_len;
    return 1;
  }

  // The ciphertext buffer must be non-NULL for actual decapsulation.
  if (ciphertext == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  // The input and output buffers need to be large enough.
  if (ciphertext_len != kem->ciphertext_len ||
      *shared_secret_len < kem->shared_secret_len) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_BUFFER_SIZE);
      return 0;
  }

  // Check that the context is properly configured.
  if (ctx->pkey == NULL ||
      ctx->pkey->pkey.kem_key == NULL ||
      ctx->pkey->type != EVP_PKEY_KEM) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
      return 0;
  }

  // Check that the key has a secret key set.
  KEM_KEY *key = ctx->pkey->pkey.kem_key;
  if (key->secret_key == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }

  if (!kem->method->decaps(shared_secret, shared_secret_len, ciphertext, key->secret_key)) {
    return 0;
  }

  // The size of the shared secret that has been written to the output buffer.
  *shared_secret_len = kem->shared_secret_len;

  return 1;
}

DEFINE_METHOD_FUNCTION(EVP_PKEY_METHOD, EVP_PKEY_kem_pkey_meth) {
  out->pkey_id = EVP_PKEY_KEM;
  out->init = pkey_kem_init;
  out->copy = NULL;
  out->cleanup = pkey_kem_cleanup;
  out->keygen = pkey_kem_keygen;
  out->sign_init = NULL;
  out->sign = NULL;
  out->sign_message = NULL;
  out->verify_init = NULL;
  out->verify = NULL;
  out->verify_message = NULL;
  out->verify_recover = NULL;
  out->encrypt = NULL;
  out->decrypt = NULL;
  out->derive = NULL;
  out->paramgen = NULL;
  out->ctrl = NULL;
  out->ctrl_str = NULL;
  out->keygen_deterministic = pkey_kem_keygen_deterministic;
  out->encapsulate_deterministic = pkey_kem_encapsulate_deterministic;
  out->encapsulate = pkey_kem_encapsulate;
  out->decapsulate = pkey_kem_decapsulate;
}

// Additional KEM specific EVP functions.

int EVP_PKEY_CTX_kem_set_params(EVP_PKEY_CTX *ctx, int nid) {
  if (ctx == NULL || ctx->data == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  // It's not allowed to change context parameters if
  // a PKEY is already associated with the context.
  if (ctx->pkey != NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
    return 0;
  }

  const KEM *kem = KEM_find_kem_by_nid(nid);
  if (kem == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    return 0;
  }

  KEM_PKEY_CTX *dctx = ctx->data;
  dctx->kem = kem;

  return 1;
}


// This function sets KEM parameters defined by |nid| in |pkey|.
int EVP_PKEY_kem_set_params(EVP_PKEY *pkey, int nid) {
  const KEM *kem = KEM_find_kem_by_nid(nid);
  if (kem == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    return 0;
  }

  KEM_KEY *key = KEM_KEY_new();
  if (key == NULL) {
    // KEM_KEY_new sets the appropriate error.
    return 0;
  }

  key->kem = kem;
  evp_pkey_set0(pkey, &kem_asn1_meth, key);

  return 1;
}

// Returns a fresh EVP_PKEY object of type EVP_PKEY_KEM,
// and sets KEM parameters defined by |nid|.
static EVP_PKEY *EVP_PKEY_kem_new(int nid) {
  EVP_PKEY *ret = EVP_PKEY_new();
  if (ret == NULL || !EVP_PKEY_kem_set_params(ret, nid)) {
    EVP_PKEY_free(ret);
    return NULL;
  }

  return ret;
}

EVP_PKEY *EVP_PKEY_kem_new_raw_public_key(int nid, const uint8_t *in, size_t len) {
  if (in == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  EVP_PKEY *ret = EVP_PKEY_kem_new(nid);
  if (ret == NULL || ret->pkey.kem_key == NULL) {
    // EVP_PKEY_kem_new sets the appropriate error.
    goto err;
  }

  const KEM *kem = KEM_KEY_get0_kem(ret->pkey.kem_key);
  if (kem->public_key_len != len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_BUFFER_SIZE);
    goto err;
  }

  if (!KEM_KEY_set_raw_public_key(ret->pkey.kem_key, in)) {
    // KEM_KEY_set_raw_public_key sets the appropriate error.
    goto err;
  }

  return ret;

err:
  EVP_PKEY_free(ret);
  return NULL;
}

EVP_PKEY *EVP_PKEY_kem_new_raw_secret_key(int nid, const uint8_t *in, size_t len) {
  if (in == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  EVP_PKEY *ret = EVP_PKEY_kem_new(nid);
  if (ret == NULL || ret->pkey.kem_key == NULL) {
    // EVP_PKEY_kem_new sets the appropriate error.
    goto err;
  }

  const KEM *kem = KEM_KEY_get0_kem(ret->pkey.kem_key);
  if (kem->secret_key_len != len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_BUFFER_SIZE);
    goto err;
  }

  if (!KEM_KEY_set_raw_secret_key(ret->pkey.kem_key, in)) {
    // KEM_KEY_set_raw_secret_key sets the appropriate error.
    goto err;
  }

  return ret;

err:
  EVP_PKEY_free(ret);
  return NULL;
}

EVP_PKEY *EVP_PKEY_kem_new_raw_key(int nid,
                                   const uint8_t *in_public, size_t len_public,
                                   const uint8_t *in_secret, size_t len_secret) {
  if (in_public == NULL || in_secret == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  EVP_PKEY *ret = EVP_PKEY_kem_new(nid);
  if (ret == NULL || ret->pkey.kem_key == NULL) {
    // EVP_PKEY_kem_new sets the appropriate error.
    goto err;
  }

  const KEM *kem = KEM_KEY_get0_kem(ret->pkey.kem_key);
  if (kem->public_key_len != len_public || kem->secret_key_len != len_secret) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_BUFFER_SIZE);
    goto err;
  }
  
  if (!KEM_KEY_set_raw_key(ret->pkey.kem_key, in_public, in_secret)) {
    // KEM_KEY_set_raw_key sets the appropriate error.
    goto err;
  }

  return ret;

err:
  EVP_PKEY_free(ret);
  return NULL;
}

// EVP_PKEY_kem_check_key validates that the public key in |key| corresponds
// to the secret key in |key| by performing encapsulation and decapsulation
// and checking that the generated shared secrets are equal.
int EVP_PKEY_kem_check_key(EVP_PKEY *key) {
  if (key == NULL || key->pkey.kem_key == NULL ||
      key->pkey.kem_key->public_key == NULL ||
      key->pkey.kem_key->secret_key == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new(key, NULL);
  if (ctx == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  int ret = 0;

  // Get the required buffer lengths and allocate the buffers.
  size_t ct_len, ss_len;
  uint8_t *ct = NULL, *ss_a = NULL, *ss_b = NULL;
  if (!EVP_PKEY_encapsulate(ctx, NULL, &ct_len, NULL, &ss_len)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_INTERNAL_ERROR);
    goto end;
  }
  ct = OPENSSL_malloc(ct_len);
  ss_a = OPENSSL_malloc(ss_len);
  ss_b = OPENSSL_malloc(ss_len);
  if (ct == NULL || ss_a == NULL || ss_b == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_INTERNAL_ERROR);
    goto end;
  }

  // Encapsulate and decapsulate.
  if (!EVP_PKEY_encapsulate(ctx, ct, &ct_len, ss_b, &ss_len) ||
      !EVP_PKEY_decapsulate(ctx, ss_a, &ss_len, ct, ct_len)) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_INTERNAL_ERROR);
    goto end;
  }

  // Compare the shared secrets.
  uint8_t res = 0;
  for (size_t i = 0; i < ss_len; i++) {
    res |= ss_a[i] ^ ss_b[i];
  }

  // If the shared secrets |ss_a| and |ss_b| are the same then |res| is equal
  // to zero, otherwise it's not. |constant_time_is_zero_8| returns 0xff when
  // |res| is equal to zero, and returns 0 otherwise. To be consistent with the
  // rest of the library, we extract only the first bit so that |ret| is either
  // 0 or 1.
  ret = constant_time_is_zero_8(res) & 1;

end:
  OPENSSL_free(ct);
  OPENSSL_free(ss_a);
  OPENSSL_free(ss_b);
  EVP_PKEY_CTX_free(ctx);
  return ret;
}
