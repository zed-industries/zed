// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/evp.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../crypto/evp_extra/internal.h"
#include "../delocate.h"
#include "../ml_dsa/ml_dsa.h"
#include "../crypto/internal.h"
#include "../pqdsa/internal.h"

// PQDSA PKEY functions

typedef struct {
  const PQDSA *pqdsa;
  uint8_t context[255];
  size_t context_len;
} PQDSA_PKEY_CTX;

static int pkey_pqdsa_init(EVP_PKEY_CTX *ctx) {
  PQDSA_PKEY_CTX *dctx;
  dctx = OPENSSL_zalloc(sizeof(PQDSA_PKEY_CTX));
  if (dctx == NULL) {
    return 0;
  }

  ctx->data = dctx;

  return 1;
}

static void pkey_pqdsa_cleanup(EVP_PKEY_CTX *ctx) {
  OPENSSL_free(ctx->data);
}

static int pkey_pqdsa_copy(EVP_PKEY_CTX *dst, EVP_PKEY_CTX *src) {
  if (!pkey_pqdsa_init(dst)) {
    return 0;
  }

  PQDSA_PKEY_CTX *dctx = dst->data;
  PQDSA_PKEY_CTX *sctx = src->data;
  GUARD_PTR(dctx);
  GUARD_PTR(sctx);

  // Shallow copy is safe here because |pqdsa| points to a static-storage
  // object returned by |PQDSA_find_dsa_by_nid|.
  dctx->pqdsa = sctx->pqdsa;
  OPENSSL_memcpy(dctx->context, sctx->context, sizeof(sctx->context));
  dctx->context_len = sctx->context_len;

  return 1;
}

static int pkey_pqdsa_ctrl(EVP_PKEY_CTX *ctx, int type, int p1, void *p2) {
  GUARD_PTR(ctx);
  PQDSA_PKEY_CTX *dctx = (PQDSA_PKEY_CTX *)ctx->data;
  switch (type) {
    case EVP_PKEY_CTRL_SIGNING_CONTEXT: {
      EVP_PKEY_CTX_SIGNATURE_CONTEXT_PARAMS *params = p2;
      if (!params || !dctx ||
          params->context_len > sizeof(dctx->context) ||
          (params->context_len > 0 && !params->context)) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
        return 0;
      }
      OPENSSL_cleanse(dctx->context, sizeof(dctx->context));
      if (params->context_len > 0) {
        OPENSSL_memcpy(dctx->context, params->context, params->context_len);
      }
      dctx->context_len = params->context_len;
      break;
    }
    case EVP_PKEY_CTRL_GET_SIGNING_CONTEXT: {
      EVP_PKEY_CTX_SIGNATURE_CONTEXT_PARAMS *params = p2;
      if (!params || !dctx) {
        return 0;
      }
      if (dctx->context_len == 0) {
        params->context = NULL;
        params->context_len = 0;
      } else {
        params->context = dctx->context;
        params->context_len = dctx->context_len;
      }
      return 1;
    }
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
      return 0;
  }
  return 1;
}

static int pkey_pqdsa_keygen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  GUARD_PTR(ctx);
  PQDSA_PKEY_CTX *dctx = ctx->data;
  GUARD_PTR(dctx);
  const PQDSA *pqdsa = dctx->pqdsa;
  if (pqdsa == NULL) {
    if (ctx->pkey == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
      return 0;
    }
    pqdsa = PQDSA_KEY_get0_dsa(ctx->pkey->pkey.pqdsa_key);
  }

  PQDSA_KEY *key = PQDSA_KEY_new();
  if (key == NULL ||
      !PQDSA_KEY_init(key, pqdsa) ||
      !pqdsa->method->pqdsa_keygen(key->public_key, key->private_key, key->seed) ||
      !EVP_PKEY_assign(pkey, EVP_PKEY_PQDSA, key)) {
    PQDSA_KEY_free(key);
    return 0;
      }
  return 1;
}

static int pkey_pqdsa_sign_generic(EVP_PKEY_CTX *ctx, uint8_t *sig,
                                   size_t *sig_len, const uint8_t *message,
                                   size_t message_len, int sign_digest) {
  GUARD_PTR(sig_len);

  PQDSA_PKEY_CTX *dctx = ctx->data;
  const PQDSA *pqdsa = dctx->pqdsa;
  if (pqdsa == NULL) {
    if (ctx->pkey == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
      return 0;
    }
    pqdsa = PQDSA_KEY_get0_dsa(ctx->pkey->pkey.pqdsa_key);
  }

  // Caller is getting parameter values.
  if (sig == NULL) {
    *sig_len = pqdsa->signature_len;
    return 1;
  }

  if (*sig_len < pqdsa->signature_len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  // Check that the context is properly configured.
  if (ctx->pkey == NULL ||
      ctx->pkey->pkey.pqdsa_key == NULL ||
      ctx->pkey->type != EVP_PKEY_PQDSA) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }

  PQDSA_KEY *key = ctx->pkey->pkey.pqdsa_key;
  if (!key || !key->private_key) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }

  // |sign_digest| is a flag we use to indicate that the message to be signed has
  // already been pre-processed and hashed into a message digest.
  // When the PQDSA algorithm is selected as ML-DSA (i.e., NID_MLDSA{44/65/87}),
  // |sign_digest| indicates that the input is |mu| which is the result of a SHAKE256
  // hash of the associated public key concatenated with a zero byte to indicate
  // pure-mode, the context string length, the contents of the context string,
  // and the input message in this order e.g.
  // mu = SHAKE256(SHAKE256(pk) || 0 || |ctx| || ctx || M).

  // RAW sign mode
  if (!sign_digest) {
    if (!pqdsa->method->pqdsa_sign_message(
            key->private_key, sig, sig_len, message, message_len,
            dctx->context_len > 0 ? dctx->context : NULL,
            dctx->context_len)) {
      OPENSSL_PUT_ERROR(EVP, ERR_R_INTERNAL_ERROR);
      return 0;
    }
  }
  // DIGEST sign mode
  else {
    // For ML-DSA, the digest-sign path (|EVP_PKEY_sign|) takes a pre-hashed
    // |mu| input which already encodes the context string per FIPS 204
    // section 5.3. Applying a separately-configured context here would be
    // silently ignored and produce a signature inconsistent with the
    // caller's intent, so reject the combination explicitly.
    if (dctx->context_len > 0) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
      return 0;
    }
    if (message_len != pqdsa->digest_len) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_BUFFER_SIZE);
      return 0;
    }
    if (!pqdsa->method->pqdsa_sign(key->private_key, sig, sig_len, message, message_len)) {
      OPENSSL_PUT_ERROR(EVP, ERR_R_INTERNAL_ERROR);
      return 0;
    }
  }

  return 1;
}

// DIGEST signing
static int pkey_pqdsa_sign(EVP_PKEY_CTX *ctx, uint8_t *sig,
                                     size_t *sig_len, const uint8_t *digest,
                                     size_t digest_len) {
  return pkey_pqdsa_sign_generic(ctx, sig, sig_len, digest, digest_len, 1);
}

// RAW message signing
static int pkey_pqdsa_sign_message(EVP_PKEY_CTX *ctx, uint8_t *sig,
                                     size_t *sig_len, const uint8_t *message,
                                     size_t message_len) {
  return pkey_pqdsa_sign_generic(ctx, sig, sig_len, message, message_len, 0);
}

static int pkey_pqdsa_verify_generic(EVP_PKEY_CTX *ctx, const uint8_t *sig,
                                     size_t sig_len, const uint8_t *message,
                                     size_t message_len, int verify_digest) {
  PQDSA_PKEY_CTX *dctx = ctx->data;
  const PQDSA *pqdsa = dctx->pqdsa;

  if (sig == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_MISSING_PARAMETERS);
    return 0;
  }

  if (pqdsa == NULL) {
    if (ctx->pkey == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
      return 0;
    }

    pqdsa = PQDSA_KEY_get0_dsa(ctx->pkey->pkey.pqdsa_key);
  }
  // Check that the context is properly configured.
  if (ctx->pkey == NULL ||
    ctx->pkey->pkey.pqdsa_key == NULL ||
    ctx->pkey->type != EVP_PKEY_PQDSA) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }

  PQDSA_KEY *key = ctx->pkey->pkey.pqdsa_key;
  if (!key || !key->public_key) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }

  // |verify_digest| is a flag we use to indicate that the message to be verified has
  // already been pre-processed and hashed into a message digest.
  // When the PQDSA algorithm is selected as ML-DSA (i.e., NID_MLDSA{44/65/87}),
  // |verify_digest| indicates that the input is |mu| which is the result of a SHAKE256
  // hash of the associated public key concatenated with a zero byte to indicate
  // pure-mode, the context string length, the contents of the context string,
  // and the input message in this order e.g.
  // mu = SHAKE256(SHAKE256(pk) || 0 || |ctx| || ctx || M).

  // RAW verify mode
  if(!verify_digest) {
    if (sig_len != pqdsa->signature_len ||
        !pqdsa->method->pqdsa_verify_message(
            key->public_key, sig, sig_len, message, message_len,
            dctx->context_len > 0 ? dctx->context : NULL,
            dctx->context_len)) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_SIGNATURE);
      return 0;
    }
  }
  // DIGEST verify mode
  else {
    // For ML-DSA, the digest-verify path (|EVP_PKEY_verify|) takes a
    // pre-hashed |mu| input which already encodes the context string per
    // FIPS 204 section 5.3. Applying a separately-configured context here
    // would be silently ignored and produce a verification inconsistent
    // with the caller's intent, so reject the combination explicitly.
    if (dctx->context_len > 0) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
      return 0;
    }
    if (message_len != pqdsa->digest_len) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_BUFFER_SIZE);
      return 0;
    }
    if (sig_len != pqdsa->signature_len ||
    !pqdsa->method->pqdsa_verify(key->public_key, sig, sig_len, message, message_len)) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_SIGNATURE);
      return 0;
    }
  }

  return 1;
}

// DIGEST verification
static int pkey_pqdsa_verify(EVP_PKEY_CTX *ctx, const uint8_t *sig,
                             size_t sig_len, const uint8_t *message,
                             size_t message_len) {
  return pkey_pqdsa_verify_generic(ctx, sig, sig_len, message, message_len, 1);
}

// RAW message verification
static int pkey_pqdsa_verify_message(EVP_PKEY_CTX *ctx, const uint8_t *sig,
                                    size_t sig_len, const uint8_t *message,
                                    size_t message_len) {
  return pkey_pqdsa_verify_generic(ctx, sig, sig_len, message, message_len, 0);
}

// Additional PQDSA specific EVP functions.

// This function sets pqdsa parameters defined by |nid| in |pkey|.
int EVP_PKEY_pqdsa_set_params(EVP_PKEY *pkey, int nid) {
  const PQDSA *pqdsa = PQDSA_find_dsa_by_nid(nid);

  if (pqdsa == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    return 0;
  }

  PQDSA_KEY *key = PQDSA_KEY_new();
  if (key == NULL) {
    // PQDSA_KEY_new sets the appropriate error.
    return 0;
  }

  key->pqdsa = pqdsa;
  evp_pkey_set0(pkey, &pqdsa_asn1_meth, key);

  return 1;
}

// Takes an EVP_PKEY_CTX object |ctx| and sets pqdsa parameters defined
// by |nid|
int EVP_PKEY_CTX_pqdsa_set_params(EVP_PKEY_CTX *ctx, int nid) {
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

  const PQDSA *pqdsa = PQDSA_find_dsa_by_nid(nid);
  if (pqdsa == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    return 0;
  }

  PQDSA_PKEY_CTX *dctx = ctx->data;
  dctx->pqdsa = pqdsa;

  return 1;
}

// Returns a fresh EVP_PKEY object of type EVP_PKEY_PQDSA,
// and sets PQDSA parameters defined by |nid|.
static EVP_PKEY *EVP_PKEY_pqdsa_new(int nid) {
  EVP_PKEY *ret = EVP_PKEY_new();
  if (ret == NULL || !EVP_PKEY_pqdsa_set_params(ret, nid)) {
    EVP_PKEY_free(ret);
    return NULL;
  }

  return ret;
}

EVP_PKEY *EVP_PKEY_pqdsa_new_raw_public_key(int nid, const uint8_t *in, size_t len) {
  if (in == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  EVP_PKEY *ret = EVP_PKEY_pqdsa_new(nid);
  if (ret == NULL || ret->pkey.pqdsa_key == NULL) {
    // EVP_PKEY_pqdsa_new sets the appropriate error.
    goto err;
  }

  CBS cbs;
  CBS_init(&cbs, in, len);
  if (!PQDSA_KEY_set_raw_public_key(ret->pkey.pqdsa_key, &cbs)) {
    // PQDSA_KEY_set_raw_public_key sets the appropriate error.
    goto err;
  }

  return ret;

  err:
    EVP_PKEY_free(ret);
  return NULL;
}

EVP_PKEY *EVP_PKEY_pqdsa_new_raw_private_key(int nid, const uint8_t *in, size_t len) {
  if (in == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  EVP_PKEY *ret = EVP_PKEY_pqdsa_new(nid);
  if (ret == NULL || ret->pkey.pqdsa_key == NULL) {
    // EVP_PKEY_pqdsa_new sets the appropriate error.
    goto err;
  }

  // Get PQDSA instance and validate lengths
  const PQDSA *pqdsa = PQDSA_KEY_get0_dsa(ret->pkey.pqdsa_key);
  if (len != pqdsa->private_key_len && len != pqdsa->keygen_seed_len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_BUFFER_SIZE);
    goto err;
  }

  CBS cbs;
  CBS_init(&cbs, in, len);

  // Set key based on input length
  if (len == pqdsa->private_key_len) {
    if (!PQDSA_KEY_set_raw_private_key(ret->pkey.pqdsa_key, &cbs)) {
      // PQDSA_KEY_set_raw_private_key sets the appropriate error.
      goto err;
    }
  } else if (len == pqdsa->keygen_seed_len) {
    if (!PQDSA_KEY_set_raw_keypair_from_seed(ret->pkey.pqdsa_key, &cbs)) {
      // PQDSA_KEY_set_raw_keypair_from_seed sets the appropriate error.
      goto err;
    }
  }

  return ret;

  err:
    EVP_PKEY_free(ret);
  return NULL;
}

DEFINE_METHOD_FUNCTION(EVP_PKEY_METHOD, EVP_PKEY_pqdsa_pkey_meth) {
  out->pkey_id = EVP_PKEY_PQDSA;
  out->init = pkey_pqdsa_init;
  out->copy = pkey_pqdsa_copy;
  out->cleanup = pkey_pqdsa_cleanup;
  out->keygen = pkey_pqdsa_keygen;
  out->sign_init = NULL;
  out->sign = pkey_pqdsa_sign;
  out->sign_message = pkey_pqdsa_sign_message;
  out->verify_init = NULL;
  out->verify = pkey_pqdsa_verify;
  out->verify_message = pkey_pqdsa_verify_message;
  out->verify_recover = NULL;
  out->encrypt = NULL;
  out->decrypt = NULL;
  out->derive = NULL;
  out->paramgen = NULL;
  out->ctrl = pkey_pqdsa_ctrl;
  out->ctrl_str = NULL;
  out->keygen_deterministic = NULL;
  out->encapsulate_deterministic = NULL;
  out->encapsulate = NULL;
  out->decapsulate = NULL;
}
