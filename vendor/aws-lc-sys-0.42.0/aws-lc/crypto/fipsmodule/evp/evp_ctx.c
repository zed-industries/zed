// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>
#include <openssl/experimental/kem_deterministic_api.h>

#include <string.h>

#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "internal.h"
#include "../../evp_extra/internal.h"

DEFINE_LOCAL_DATA(struct fips_evp_pkey_methods, AWSLC_fips_evp_pkey_methods) {
  out->methods[0] = EVP_PKEY_rsa_pkey_meth();
  out->methods[1] = EVP_PKEY_rsa_pss_pkey_meth();
  out->methods[2] = EVP_PKEY_ec_pkey_meth();
  out->methods[3] = EVP_PKEY_hkdf_pkey_meth();
  out->methods[4] = EVP_PKEY_hmac_pkey_meth();
  out->methods[5] = EVP_PKEY_ed25519_pkey_meth();
  out->methods[6] = EVP_PKEY_kem_pkey_meth();
  out->methods[7] = EVP_PKEY_pqdsa_pkey_meth();
  out->methods[8] = EVP_PKEY_ed25519ph_pkey_meth();
}

static const EVP_PKEY_METHOD *evp_pkey_meth_find(int type) {

  // First we search through the FIPS public key methods. We assume these are
  // the most popular.
  const struct fips_evp_pkey_methods *const fips_methods = AWSLC_fips_evp_pkey_methods();
  for (size_t i = 0; i < FIPS_EVP_PKEY_METHODS; i++) {
    if (fips_methods->methods[i]->pkey_id == type) {
      return fips_methods->methods[i];
    }
  }

  // Can still seek non-fips validated algorithms in fips mode.
  const EVP_PKEY_METHOD *const *non_fips_methods = AWSLC_non_fips_pkey_evp_methods();
  for (size_t i = 0; i < NON_FIPS_EVP_PKEY_METHODS; i++) {
    if (non_fips_methods[i]->pkey_id == type) {
      return non_fips_methods[i];
    }
  }

  return NULL;
}

static EVP_PKEY_CTX *evp_pkey_ctx_new(EVP_PKEY *pkey, ENGINE *e, int id) {
  EVP_PKEY_CTX *ret;
  const EVP_PKEY_METHOD *pmeth;

  if (id == -1) {
    if (!pkey || !pkey->ameth) {
      return NULL;
    }
    id = pkey->ameth->pkey_id;
  }

  pmeth = evp_pkey_meth_find(id);

  if (pmeth == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    ERR_add_error_dataf("algorithm %d", id);
    return NULL;
  }

  ret = OPENSSL_zalloc(sizeof(EVP_PKEY_CTX));
  if (!ret) {
    return NULL;
  }

  ret->engine = e;
  ret->pmeth = pmeth;
  ret->operation = EVP_PKEY_OP_UNDEFINED;

  if (pkey) {
    EVP_PKEY_up_ref(pkey);
    ret->pkey = pkey;
  }

  if (pmeth->init) {
    if (pmeth->init(ret) <= 0) {
      EVP_PKEY_free(ret->pkey);
      OPENSSL_free(ret);
      return NULL;
    }
  }

  return ret;
}

EVP_PKEY_CTX *EVP_PKEY_CTX_new(EVP_PKEY *pkey, ENGINE *e) {
  SET_DIT_AUTO_RESET;
  return evp_pkey_ctx_new(pkey, e, -1);
}

EVP_PKEY_CTX *EVP_PKEY_CTX_new_id(int id, ENGINE *e) {
  return evp_pkey_ctx_new(NULL, e, id);
}

void EVP_PKEY_CTX_free(EVP_PKEY_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  if (ctx == NULL) {
    return;
  }
  if (ctx->pmeth && ctx->pmeth->cleanup) {
    ctx->pmeth->cleanup(ctx);
  }
  EVP_PKEY_free(ctx->pkey);
  EVP_PKEY_free(ctx->peerkey);
  OPENSSL_free(ctx);
}

EVP_PKEY_CTX *EVP_PKEY_CTX_dup(EVP_PKEY_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  if (!ctx->pmeth || !ctx->pmeth->copy) {
    return NULL;
  }

  EVP_PKEY_CTX *ret = OPENSSL_zalloc(sizeof(EVP_PKEY_CTX));
  if (!ret) {
    return NULL;
  }

  ret->pmeth = ctx->pmeth;
  ret->engine = ctx->engine;
  ret->operation = ctx->operation;

  if (ctx->pkey != NULL) {
    EVP_PKEY_up_ref(ctx->pkey);
    ret->pkey = ctx->pkey;
  }

  if (ctx->peerkey != NULL) {
    EVP_PKEY_up_ref(ctx->peerkey);
    ret->peerkey = ctx->peerkey;
  }

  if (ctx->pmeth->copy(ret, ctx) <= 0) {
    ret->pmeth = NULL;
    EVP_PKEY_CTX_free(ret);
    OPENSSL_PUT_ERROR(EVP, ERR_LIB_EVP);
    return NULL;
  }

  return ret;
}

EVP_PKEY *EVP_PKEY_CTX_get0_pkey(EVP_PKEY_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  return ctx->pkey;
}

int EVP_PKEY_CTX_ctrl(EVP_PKEY_CTX *ctx, int keytype, int optype, int cmd,
                      int p1, void *p2) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->ctrl) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
    return 0;
  }
  if (keytype != -1 && ctx->pmeth->pkey_id != keytype) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  if (ctx->operation == EVP_PKEY_OP_UNDEFINED) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_OPERATION_SET);
    return 0;
  }

  if (optype != -1 && !(ctx->operation & optype)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
    return 0;
  }

  return ctx->pmeth->ctrl(ctx, cmd, p1, p2);
}

int EVP_PKEY_sign_init(EVP_PKEY_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  if (ctx == NULL || ctx->pmeth == NULL ||
      (ctx->pmeth->sign == NULL && ctx->pmeth->sign_message == NULL)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  ctx->operation = EVP_PKEY_OP_SIGN;
  if ((ctx->pmeth->sign_init == NULL) || (ctx->pmeth->sign_init(ctx))) {
    return 1;
  }
  ctx->operation = EVP_PKEY_OP_UNDEFINED;
  return 0;
}

int EVP_PKEY_sign(EVP_PKEY_CTX *ctx, uint8_t *sig, size_t *sig_len,
                  const uint8_t *digest, size_t digest_len) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->sign) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  if (ctx->operation != EVP_PKEY_OP_SIGN) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }
  GUARD_PTR(sig_len);
  GUARD_PTR(ctx->pkey);
  return ctx->pmeth->sign(ctx, sig, sig_len, digest, digest_len);
}

int EVP_PKEY_verify_init(EVP_PKEY_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  if (ctx == NULL || ctx->pmeth == NULL ||
      (ctx->pmeth->verify == NULL && ctx->pmeth->verify_message == NULL)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  ctx->operation = EVP_PKEY_OP_VERIFY;
  if ((ctx->pmeth->verify_init == NULL) || (ctx->pmeth->verify_init(ctx))) {
    return 1;
  }
  ctx->operation = EVP_PKEY_OP_UNDEFINED;
  return 0;
}

int EVP_PKEY_verify(EVP_PKEY_CTX *ctx, const uint8_t *sig, size_t sig_len,
                    const uint8_t *digest, size_t digest_len) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->verify) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  if (ctx->operation != EVP_PKEY_OP_VERIFY) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }
  return ctx->pmeth->verify(ctx, sig, sig_len, digest, digest_len);
}

int EVP_PKEY_encrypt_init(EVP_PKEY_CTX *ctx) {
  if (!ctx || !ctx->pmeth || !ctx->pmeth->encrypt) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  ctx->operation = EVP_PKEY_OP_ENCRYPT;
  return 1;
}

int EVP_PKEY_encrypt(EVP_PKEY_CTX *ctx, uint8_t *out, size_t *outlen,
                     const uint8_t *in, size_t inlen) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->encrypt) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  if (ctx->operation != EVP_PKEY_OP_ENCRYPT) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }
  return ctx->pmeth->encrypt(ctx, out, outlen, in, inlen);
}

int EVP_PKEY_decrypt_init(EVP_PKEY_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->decrypt) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  ctx->operation = EVP_PKEY_OP_DECRYPT;
  return 1;
}

int EVP_PKEY_decrypt(EVP_PKEY_CTX *ctx, uint8_t *out, size_t *outlen,
                     const uint8_t *in, size_t inlen) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->decrypt) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  if (ctx->operation != EVP_PKEY_OP_DECRYPT) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }
  return ctx->pmeth->decrypt(ctx, out, outlen, in, inlen);
}

int EVP_PKEY_verify_recover_init(EVP_PKEY_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->verify_recover) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  ctx->operation = EVP_PKEY_OP_VERIFYRECOVER;
  return 1;
}

int EVP_PKEY_verify_recover(EVP_PKEY_CTX *ctx, uint8_t *out, size_t *out_len,
                            const uint8_t *sig, size_t sig_len) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->verify_recover) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  if (ctx->operation != EVP_PKEY_OP_VERIFYRECOVER) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }
  return ctx->pmeth->verify_recover(ctx, out, out_len, sig, sig_len);
}

int EVP_PKEY_derive_init(EVP_PKEY_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->derive) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  ctx->operation = EVP_PKEY_OP_DERIVE;
  return 1;
}

int EVP_PKEY_derive_set_peer(EVP_PKEY_CTX *ctx, EVP_PKEY *peer) {
  SET_DIT_AUTO_RESET;
  int ret;
  if (!ctx || !ctx->pmeth ||
      !(ctx->pmeth->derive || ctx->pmeth->encrypt || ctx->pmeth->decrypt) ||
      !ctx->pmeth->ctrl) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  if (ctx->operation != EVP_PKEY_OP_DERIVE &&
      ctx->operation != EVP_PKEY_OP_ENCRYPT &&
      ctx->operation != EVP_PKEY_OP_DECRYPT) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }

  ret = ctx->pmeth->ctrl(ctx, EVP_PKEY_CTRL_PEER_KEY, 0, peer);

  if (ret <= 0) {
    return 0;
  }

  if (ret == 2) {
    return 1;
  }

  if (!ctx->pkey) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }

  if (ctx->pkey->type != peer->type) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DIFFERENT_KEY_TYPES);
    return 0;
  }

  // ran@cryptocom.ru: For clarity.  The error is if parameters in peer are
  // present (!missing) but don't match.  EVP_PKEY_cmp_parameters may return
  // 1 (match), 0 (don't match) and -2 (comparison is not defined).  -1
  // (different key types) is impossible here because it is checked earlier.
  // -2 is OK for us here, as well as 1, so we can check for 0 only.
  if (!EVP_PKEY_missing_parameters(peer) &&
      !EVP_PKEY_cmp_parameters(ctx->pkey, peer)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DIFFERENT_PARAMETERS);
    return 0;
  }

  // Take a reference to the new peer before freeing the old one, in case
  // peer == ctx->peerkey and the caller holds the sole reference.
  EVP_PKEY_up_ref(peer);
  EVP_PKEY_free(ctx->peerkey);
  ctx->peerkey = peer;

  ret = ctx->pmeth->ctrl(ctx, EVP_PKEY_CTRL_PEER_KEY, 1, peer);

  if (ret <= 0) {
    EVP_PKEY_free(ctx->peerkey);
    ctx->peerkey = NULL;
    return 0;
  }

  return 1;
}

int EVP_PKEY_derive(EVP_PKEY_CTX *ctx, uint8_t *key, size_t *out_key_len) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->derive) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  if (ctx->operation != EVP_PKEY_OP_DERIVE) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }
  return ctx->pmeth->derive(ctx, key, out_key_len);
}

int EVP_PKEY_keygen_init(EVP_PKEY_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->keygen) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  ctx->operation = EVP_PKEY_OP_KEYGEN;
  return 1;
}

int EVP_PKEY_keygen_deterministic(EVP_PKEY_CTX *ctx,
                                  EVP_PKEY **out_pkey,
                                  const uint8_t *seed,
                                  size_t *seed_len) {
  int ret = 0;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->keygen_deterministic) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    goto end;
  }
  if (ctx->operation != EVP_PKEY_OP_KEYGEN) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    goto end;
  }

  if ((out_pkey == NULL) != (seed == NULL)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
    goto end;
  }

  // Caller is performing a size check.
  if (out_pkey == NULL && seed == NULL) {
    if (!ctx->pmeth->keygen_deterministic(ctx, NULL, NULL, seed_len)) {
      goto end;
    }
    ret = 1;
    goto end;
  }

  if (!*out_pkey) {
    *out_pkey = EVP_PKEY_new();
    if (!*out_pkey) {
      OPENSSL_PUT_ERROR(EVP, ERR_LIB_EVP);
      goto end;
    }
  }

  if (!ctx->pmeth->keygen_deterministic(ctx, *out_pkey, seed, seed_len)) {
    EVP_PKEY_free(*out_pkey);
    *out_pkey = NULL;
    goto end;
  }

  ret = 1;
end:
  return ret;
}

int EVP_PKEY_keygen(EVP_PKEY_CTX *ctx, EVP_PKEY **out_pkey) {
  // We have to avoid potential underlying services updating the indicator state,
  // so we lock the state here.
  FIPS_service_indicator_lock_state();
  SET_DIT_AUTO_RESET;
  int ret = 0;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->keygen) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    goto end;
  }
  if (ctx->operation != EVP_PKEY_OP_KEYGEN) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    goto end;
  }

  if (!out_pkey) {
    goto end;
  }

  if (!*out_pkey) {
    *out_pkey = EVP_PKEY_new();
    if (!*out_pkey) {
      OPENSSL_PUT_ERROR(EVP, ERR_LIB_EVP);
      goto end;
    }
  }

  if (!ctx->pmeth->keygen(ctx, *out_pkey)) {
    EVP_PKEY_free(*out_pkey);
    *out_pkey = NULL;
    goto end;
  }

  ret = 1;
end:
  FIPS_service_indicator_unlock_state();
  if(ret) {
    EVP_PKEY_keygen_verify_service_indicator(*out_pkey);
  }
  return ret;
}

int EVP_PKEY_paramgen_init(EVP_PKEY_CTX *ctx) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->paramgen) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  ctx->operation = EVP_PKEY_OP_PARAMGEN;
  return 1;
}

int EVP_PKEY_paramgen(EVP_PKEY_CTX *ctx, EVP_PKEY **out_pkey) {
  SET_DIT_AUTO_RESET;
  if (!ctx || !ctx->pmeth || !ctx->pmeth->paramgen) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  if (ctx->operation != EVP_PKEY_OP_PARAMGEN) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATON_NOT_INITIALIZED);
    return 0;
  }

  if (!out_pkey) {
    return 0;
  }

  if (!*out_pkey) {
    *out_pkey = EVP_PKEY_new();
    if (!*out_pkey) {
      OPENSSL_PUT_ERROR(EVP, ERR_LIB_EVP);
      return 0;
    }
  }

  if (!ctx->pmeth->paramgen(ctx, *out_pkey)) {
    EVP_PKEY_free(*out_pkey);
    *out_pkey = NULL;
    return 0;
  }
  return 1;
}

int EVP_PKEY_encapsulate_deterministic(EVP_PKEY_CTX *ctx,
                                       uint8_t *ciphertext,
                                       size_t *ciphertext_len,
                                       uint8_t *shared_secret,
                                       size_t *shared_secret_len,
                                       const uint8_t *seed,
                                       size_t *seed_len) {
  if (ctx == NULL || ctx->pmeth == NULL || ctx->pmeth->encapsulate_deterministic == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  return ctx->pmeth->encapsulate_deterministic(ctx, ciphertext, ciphertext_len,
                                               shared_secret, shared_secret_len,
                                               seed, seed_len);
}

int EVP_PKEY_encapsulate(EVP_PKEY_CTX *ctx, uint8_t *ciphertext,
                         size_t *ciphertext_len, uint8_t *shared_secret,
                         size_t *shared_secret_len) {
  SET_DIT_AUTO_RESET;
  // We have to avoid potential underlying services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  int ret = 0;
  if (ctx == NULL || ctx->pmeth == NULL || ctx->pmeth->encapsulate == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    goto end;
  }

  if (!ctx->pmeth->encapsulate(ctx, ciphertext, ciphertext_len, shared_secret,
                               shared_secret_len)) {
    goto end;
  }
  ret = 1;
end:
  FIPS_service_indicator_unlock_state();
  if (ret && ciphertext != NULL && shared_secret != NULL) {
    EVP_PKEY_encapsulate_verify_service_indicator(ctx);
  }
  return ret;
}

int EVP_PKEY_decapsulate(EVP_PKEY_CTX *ctx, uint8_t *shared_secret,
                         size_t *shared_secret_len, const uint8_t *ciphertext,
                         size_t ciphertext_len) {
  SET_DIT_AUTO_RESET;
  // We have to avoid potential underlying services updating the indicator
  // state, so we lock the state here.
  FIPS_service_indicator_lock_state();

  int ret = 0;
  if (ctx == NULL || ctx->pmeth == NULL || ctx->pmeth->decapsulate == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    goto end;
  }

  if (!ctx->pmeth->decapsulate(ctx, shared_secret, shared_secret_len,
                               ciphertext, ciphertext_len)) {
    goto end;
  }
  ret = 1;
end:
  FIPS_service_indicator_unlock_state();
  if (ret && shared_secret != NULL) {
    EVP_PKEY_decapsulate_verify_service_indicator(ctx);
  }
  return ret;
}

int EVP_PKEY_CTX_md(EVP_PKEY_CTX *ctx, int optype, int cmd, const char *md) {
  const EVP_MD *m;

  if (md == NULL || (m = EVP_get_digestbyname(md)) == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_DIGEST_TYPE);
    return 0;
  }
  return EVP_PKEY_CTX_ctrl(ctx, -1, optype, cmd, 0, (void *)m);
}

int EVP_PKEY_CTX_ctrl_str(EVP_PKEY_CTX *ctx, const char *name,
                          const char *value) {
  if (!ctx || !ctx->pmeth || !ctx->pmeth->ctrl_str) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
    return -2;
  }
  if (strcmp(name, "digest") == 0) {
    OPENSSL_BEGIN_ALLOW_DEPRECATED
    return EVP_PKEY_CTX_md(ctx, EVP_PKEY_OP_TYPE_SIG, EVP_PKEY_CTRL_MD, value);
    OPENSSL_END_ALLOW_DEPRECATED
  }
  return ctx->pmeth->ctrl_str(ctx, name, value);
}


static int trans_cb(int a, int b, BN_GENCB *gcb) {
  EVP_PKEY_CTX *ctx = BN_GENCB_get_arg(gcb);
  ctx->keygen_info[0] = a;
  ctx->keygen_info[1] = b;
  return ctx->pkey_gencb(ctx);
}


void evp_pkey_set_cb_translate(BN_GENCB *cb, EVP_PKEY_CTX *ctx) {
  BN_GENCB_set(cb, trans_cb, ctx);
}

void EVP_PKEY_CTX_set_cb(EVP_PKEY_CTX *ctx, EVP_PKEY_gen_cb *cb) {
  if (ctx == NULL) {
    return;
  }
  ctx->pkey_gencb = cb;
}

void EVP_PKEY_CTX_set_app_data(EVP_PKEY_CTX *ctx, void *data) {
  if (ctx == NULL) {
    return;
  }
  ctx->app_data = data;
}

void *EVP_PKEY_CTX_get_app_data(EVP_PKEY_CTX *ctx) {
  if (ctx == NULL) {
    return NULL;
  }
  return ctx->app_data;
}

int EVP_PKEY_CTX_get_keygen_info(EVP_PKEY_CTX *ctx, int idx) {
  GUARD_PTR(ctx);
  if (idx == -1) {
    return EVP_PKEY_CTX_KEYGEN_INFO_COUNT;
  }
  if (idx < 0 || idx >= EVP_PKEY_CTX_KEYGEN_INFO_COUNT ||
      (ctx->operation != EVP_PKEY_OP_KEYGEN &&
       ctx->operation != EVP_PKEY_OP_PARAMGEN)) {
    return 0;
  }
  return ctx->keygen_info[idx];
}
