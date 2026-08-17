// Copyright (c) 2019, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/sha.h>
#include <openssl/trust_token.h>

#include "internal.h"


// The Trust Token API is described in
// https://github.com/WICG/trust-token-api/blob/master/README.md and provides a
// protocol for issuing and redeeming tokens built on top of the PMBTokens
// construction.

const TRUST_TOKEN_METHOD *TRUST_TOKEN_experiment_v1(void) {
  static const TRUST_TOKEN_METHOD kMethod = {
      pmbtoken_exp1_generate_key,
      pmbtoken_exp1_derive_key_from_secret,
      pmbtoken_exp1_client_key_from_bytes,
      pmbtoken_exp1_issuer_key_from_bytes,
      pmbtoken_exp1_blind,
      pmbtoken_exp1_sign,
      pmbtoken_exp1_unblind,
      pmbtoken_exp1_read,
      1, /* has_private_metadata */
      3, /* max_keys */
      1, /* has_srr */
  };
  return &kMethod;
}

const TRUST_TOKEN_METHOD *TRUST_TOKEN_experiment_v2_voprf(void) {
  static const TRUST_TOKEN_METHOD kMethod = {
      voprf_exp2_generate_key,
      voprf_exp2_derive_key_from_secret,
      voprf_exp2_client_key_from_bytes,
      voprf_exp2_issuer_key_from_bytes,
      voprf_exp2_blind,
      voprf_exp2_sign,
      voprf_exp2_unblind,
      voprf_exp2_read,
      0, /* has_private_metadata */
      6, /* max_keys */
      0, /* has_srr */
  };
  return &kMethod;
}

const TRUST_TOKEN_METHOD *TRUST_TOKEN_experiment_v2_pmb(void) {
  static const TRUST_TOKEN_METHOD kMethod = {
      pmbtoken_exp2_generate_key,
      pmbtoken_exp2_derive_key_from_secret,
      pmbtoken_exp2_client_key_from_bytes,
      pmbtoken_exp2_issuer_key_from_bytes,
      pmbtoken_exp2_blind,
      pmbtoken_exp2_sign,
      pmbtoken_exp2_unblind,
      pmbtoken_exp2_read,
      1, /* has_private_metadata */
      3, /* max_keys */
      0, /* has_srr */
  };
  return &kMethod;
}

const TRUST_TOKEN_METHOD *TRUST_TOKEN_pst_v1_voprf(void) {
  static const TRUST_TOKEN_METHOD kMethod = {
      voprf_pst1_generate_key,
      voprf_pst1_derive_key_from_secret,
      voprf_pst1_client_key_from_bytes,
      voprf_pst1_issuer_key_from_bytes,
      voprf_pst1_blind,
      voprf_pst1_sign,
      voprf_pst1_unblind,
      voprf_pst1_read,
      0, /* has_private_metadata */
      6, /* max_keys */
      0, /* has_srr */
  };
  return &kMethod;
}

const TRUST_TOKEN_METHOD *TRUST_TOKEN_pst_v1_pmb(void) {
  static const TRUST_TOKEN_METHOD kMethod = {
      pmbtoken_pst1_generate_key,
      pmbtoken_pst1_derive_key_from_secret,
      pmbtoken_pst1_client_key_from_bytes,
      pmbtoken_pst1_issuer_key_from_bytes,
      pmbtoken_pst1_blind,
      pmbtoken_pst1_sign,
      pmbtoken_pst1_unblind,
      pmbtoken_pst1_read,
      1, /* has_private_metadata */
      3, /* max_keys */
      0, /* has_srr */
  };
  return &kMethod;
}


void TRUST_TOKEN_PRETOKEN_free(TRUST_TOKEN_PRETOKEN *pretoken) {
  OPENSSL_free(pretoken);
}

TRUST_TOKEN *TRUST_TOKEN_new(const uint8_t *data, size_t len) {
  TRUST_TOKEN *ret = OPENSSL_zalloc(sizeof(TRUST_TOKEN));
  if (ret == NULL) {
    return NULL;
  }
  ret->data = OPENSSL_memdup(data, len);
  if (len != 0 && ret->data == NULL) {
    OPENSSL_free(ret);
    return NULL;
  }
  ret->len = len;
  return ret;
}

void TRUST_TOKEN_free(TRUST_TOKEN *token) {
  if (token == NULL) {
    return;
  }
  OPENSSL_free(token->data);
  OPENSSL_free(token);
}

int TRUST_TOKEN_generate_key(const TRUST_TOKEN_METHOD *method,
                             uint8_t *out_priv_key, size_t *out_priv_key_len,
                             size_t max_priv_key_len, uint8_t *out_pub_key,
                             size_t *out_pub_key_len, size_t max_pub_key_len,
                             uint32_t id) {
  // Prepend the key ID in front of the PMBTokens format.
  CBB priv_cbb, pub_cbb;
  CBB_init_fixed(&priv_cbb, out_priv_key, max_priv_key_len);
  CBB_init_fixed(&pub_cbb, out_pub_key, max_pub_key_len);
  if (!CBB_add_u32(&priv_cbb, id) ||  //
      !CBB_add_u32(&pub_cbb, id)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (!method->generate_key(&priv_cbb, &pub_cbb)) {
    return 0;
  }

  if (!CBB_finish(&priv_cbb, NULL, out_priv_key_len) ||
      !CBB_finish(&pub_cbb, NULL, out_pub_key_len)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_BUFFER_TOO_SMALL);
    return 0;
  }

  return 1;
}

int TRUST_TOKEN_derive_key_from_secret(
    const TRUST_TOKEN_METHOD *method, uint8_t *out_priv_key,
    size_t *out_priv_key_len, size_t max_priv_key_len, uint8_t *out_pub_key,
    size_t *out_pub_key_len, size_t max_pub_key_len, uint32_t id,
    const uint8_t *secret, size_t secret_len) {
  // Prepend the key ID in front of the PMBTokens format.
  CBB priv_cbb, pub_cbb;
  CBB_init_fixed(&priv_cbb, out_priv_key, max_priv_key_len);
  CBB_init_fixed(&pub_cbb, out_pub_key, max_pub_key_len);
  if (!CBB_add_u32(&priv_cbb, id) ||  //
      !CBB_add_u32(&pub_cbb, id)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (!method->derive_key_from_secret(&priv_cbb, &pub_cbb, secret,
                                        secret_len)) {
    return 0;
  }

  if (!CBB_finish(&priv_cbb, NULL, out_priv_key_len) ||
      !CBB_finish(&pub_cbb, NULL, out_pub_key_len)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_BUFFER_TOO_SMALL);
    return 0;
  }

  return 1;
}

TRUST_TOKEN_CLIENT *TRUST_TOKEN_CLIENT_new(const TRUST_TOKEN_METHOD *method,
                                           size_t max_batchsize) {
  if (max_batchsize > 0xffff) {
    // The protocol supports only two-byte token counts.
    OPENSSL_PUT_ERROR(TRUST_TOKEN, ERR_R_OVERFLOW);
    return NULL;
  }

  TRUST_TOKEN_CLIENT *ret = OPENSSL_zalloc(sizeof(TRUST_TOKEN_CLIENT));
  if (ret == NULL) {
    return NULL;
  }
  ret->method = method;
  ret->max_batchsize = (uint16_t)max_batchsize;
  return ret;
}

void TRUST_TOKEN_CLIENT_free(TRUST_TOKEN_CLIENT *ctx) {
  if (ctx == NULL) {
    return;
  }
  EVP_PKEY_free(ctx->srr_key);
  sk_TRUST_TOKEN_PRETOKEN_pop_free(ctx->pretokens, TRUST_TOKEN_PRETOKEN_free);
  OPENSSL_free(ctx);
}

int TRUST_TOKEN_CLIENT_add_key(TRUST_TOKEN_CLIENT *ctx, size_t *out_key_index,
                               const uint8_t *key, size_t key_len) {
  if (ctx->num_keys == OPENSSL_ARRAY_SIZE(ctx->keys) ||
      ctx->num_keys >= ctx->method->max_keys) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_TOO_MANY_KEYS);
    return 0;
  }

  struct trust_token_client_key_st *key_s = &ctx->keys[ctx->num_keys];
  CBS cbs;
  CBS_init(&cbs, key, key_len);
  uint32_t key_id;
  if (!CBS_get_u32(&cbs, &key_id) ||
      !ctx->method->client_key_from_bytes(&key_s->key, CBS_data(&cbs),
                                          CBS_len(&cbs))) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }
  key_s->id = key_id;
  *out_key_index = ctx->num_keys;
  ctx->num_keys += 1;
  return 1;
}

int TRUST_TOKEN_CLIENT_set_srr_key(TRUST_TOKEN_CLIENT *ctx, EVP_PKEY *key) {
  if (!ctx->method->has_srr) {
    return 1;
  }
  EVP_PKEY_free(ctx->srr_key);
  EVP_PKEY_up_ref(key);
  ctx->srr_key = key;
  return 1;
}

static int trust_token_client_begin_issuance_impl(
    TRUST_TOKEN_CLIENT *ctx, uint8_t **out, size_t *out_len, size_t count,
    int include_message, const uint8_t *msg, size_t msg_len) {
  if (count > ctx->max_batchsize) {
    count = ctx->max_batchsize;
  }

  int ret = 0;
  CBB request;
  STACK_OF(TRUST_TOKEN_PRETOKEN) *pretokens = NULL;
  if (!CBB_init(&request, 0) ||
      !CBB_add_u16(&request, count)) {
    goto err;
  }

  pretokens =
      ctx->method->blind(&request, count, include_message, msg, msg_len);
  if (pretokens == NULL) {
    goto err;
  }

  if (!CBB_finish(&request, out, out_len)) {
    goto err;
  }

  sk_TRUST_TOKEN_PRETOKEN_pop_free(ctx->pretokens, TRUST_TOKEN_PRETOKEN_free);
  ctx->pretokens = pretokens;
  pretokens = NULL;
  ret = 1;

err:
  CBB_cleanup(&request);
  sk_TRUST_TOKEN_PRETOKEN_pop_free(pretokens, TRUST_TOKEN_PRETOKEN_free);
  return ret;
}

int TRUST_TOKEN_CLIENT_begin_issuance(TRUST_TOKEN_CLIENT *ctx, uint8_t **out,
                                      size_t *out_len, size_t count) {
  return trust_token_client_begin_issuance_impl(ctx, out, out_len, count,
                                                /*include_message=*/0, NULL, 0);
}

int TRUST_TOKEN_CLIENT_begin_issuance_over_message(
    TRUST_TOKEN_CLIENT *ctx, uint8_t **out, size_t *out_len, size_t count,
    const uint8_t *msg, size_t msg_len) {
  return trust_token_client_begin_issuance_impl(
      ctx, out, out_len, count, /*include_message=*/1, msg, msg_len);
}


STACK_OF(TRUST_TOKEN) *
    TRUST_TOKEN_CLIENT_finish_issuance(TRUST_TOKEN_CLIENT *ctx,
                                       size_t *out_key_index,
                                       const uint8_t *response,
                                       size_t response_len) {
  CBS in;
  CBS_init(&in, response, response_len);
  uint16_t count;
  uint32_t key_id;
  if (!CBS_get_u16(&in, &count) ||
      !CBS_get_u32(&in, &key_id)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return NULL;
  }

  size_t key_index = 0;
  const struct trust_token_client_key_st *key = NULL;
  for (size_t i = 0; i < ctx->num_keys; i++) {
    if (ctx->keys[i].id == key_id) {
      key_index = i;
      key = &ctx->keys[i];
      break;
    }
  }

  if (key == NULL) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_INVALID_KEY_ID);
    return NULL;
  }

  if (count > sk_TRUST_TOKEN_PRETOKEN_num(ctx->pretokens)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return NULL;
  }

  STACK_OF(TRUST_TOKEN) *tokens =
      ctx->method->unblind(&key->key, ctx->pretokens, &in, count, key_id);
  if (tokens == NULL) {
    return NULL;
  }

  if (CBS_len(&in) != 0) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    sk_TRUST_TOKEN_pop_free(tokens, TRUST_TOKEN_free);
    return NULL;
  }

  sk_TRUST_TOKEN_PRETOKEN_pop_free(ctx->pretokens, TRUST_TOKEN_PRETOKEN_free);
  ctx->pretokens = NULL;

  *out_key_index = key_index;
  return tokens;
}

int TRUST_TOKEN_CLIENT_begin_redemption(TRUST_TOKEN_CLIENT *ctx, uint8_t **out,
                                        size_t *out_len,
                                        const TRUST_TOKEN *token,
                                        const uint8_t *data, size_t data_len,
                                        uint64_t time) {
  CBB request, token_inner, inner;
  if (!CBB_init(&request, 0) ||
      !CBB_add_u16_length_prefixed(&request, &token_inner) ||
      !CBB_add_bytes(&token_inner, token->data, token->len) ||
      !CBB_add_u16_length_prefixed(&request, &inner) ||
      !CBB_add_bytes(&inner, data, data_len) ||
      (ctx->method->has_srr && !CBB_add_u64(&request, time)) ||
      !CBB_finish(&request, out, out_len)) {
    CBB_cleanup(&request);
    return 0;
  }
  return 1;
}

int TRUST_TOKEN_CLIENT_finish_redemption(TRUST_TOKEN_CLIENT *ctx,
                                         uint8_t **out_rr, size_t *out_rr_len,
                                         uint8_t **out_sig, size_t *out_sig_len,
                                         const uint8_t *response,
                                         size_t response_len) {
  CBS in, srr, sig;
  CBS_init(&in, response, response_len);
  if (!ctx->method->has_srr) {
    if (!CBS_stow(&in, out_rr, out_rr_len)) {
      return 0;
    }

    *out_sig = NULL;
    *out_sig_len = 0;
    return 1;
  }

  if (!CBS_get_u16_length_prefixed(&in, &srr) ||
      !CBS_get_u16_length_prefixed(&in, &sig) ||
      CBS_len(&in) != 0) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_ERROR);
    return 0;
  }

  if (ctx->srr_key == NULL) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_NO_SRR_KEY_CONFIGURED);
    return 0;
  }

  EVP_MD_CTX md_ctx;
  EVP_MD_CTX_init(&md_ctx);
  int sig_ok = EVP_DigestVerifyInit(&md_ctx, NULL, NULL, NULL, ctx->srr_key) &&
               EVP_DigestVerify(&md_ctx, CBS_data(&sig), CBS_len(&sig),
                                CBS_data(&srr), CBS_len(&srr));
  EVP_MD_CTX_cleanup(&md_ctx);

  if (!sig_ok) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_SRR_SIGNATURE_ERROR);
    return 0;
  }

  uint8_t *srr_buf = NULL, *sig_buf = NULL;
  size_t srr_len, sig_len;
  if (!CBS_stow(&srr, &srr_buf, &srr_len) ||
      !CBS_stow(&sig, &sig_buf, &sig_len)) {
    OPENSSL_free(srr_buf);
    OPENSSL_free(sig_buf);
    return 0;
  }

  *out_rr = srr_buf;
  *out_rr_len = srr_len;
  *out_sig = sig_buf;
  *out_sig_len = sig_len;
  return 1;
}

TRUST_TOKEN_ISSUER *TRUST_TOKEN_ISSUER_new(const TRUST_TOKEN_METHOD *method,
                                           size_t max_batchsize) {
  if (max_batchsize > 0xffff) {
    // The protocol supports only two-byte token counts.
    OPENSSL_PUT_ERROR(TRUST_TOKEN, ERR_R_OVERFLOW);
    return NULL;
  }

  TRUST_TOKEN_ISSUER *ret = OPENSSL_zalloc(sizeof(TRUST_TOKEN_ISSUER));
  if (ret == NULL) {
    return NULL;
  }
  ret->method = method;
  ret->max_batchsize = (uint16_t)max_batchsize;
  return ret;
}

void TRUST_TOKEN_ISSUER_free(TRUST_TOKEN_ISSUER *ctx) {
  if (ctx == NULL) {
    return;
  }
  EVP_PKEY_free(ctx->srr_key);
  OPENSSL_free(ctx->metadata_key);
  OPENSSL_free(ctx);
}

int TRUST_TOKEN_ISSUER_add_key(TRUST_TOKEN_ISSUER *ctx, const uint8_t *key,
                               size_t key_len) {
  if (ctx->num_keys == OPENSSL_ARRAY_SIZE(ctx->keys) ||
      ctx->num_keys >= ctx->method->max_keys) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_TOO_MANY_KEYS);
    return 0;
  }

  struct trust_token_issuer_key_st *key_s = &ctx->keys[ctx->num_keys];
  CBS cbs;
  CBS_init(&cbs, key, key_len);
  uint32_t key_id;
  if (!CBS_get_u32(&cbs, &key_id) ||
      !ctx->method->issuer_key_from_bytes(&key_s->key, CBS_data(&cbs),
                                          CBS_len(&cbs))) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }

  key_s->id = key_id;
  ctx->num_keys += 1;
  return 1;
}

int TRUST_TOKEN_ISSUER_set_srr_key(TRUST_TOKEN_ISSUER *ctx, EVP_PKEY *key) {
  EVP_PKEY_free(ctx->srr_key);
  EVP_PKEY_up_ref(key);
  ctx->srr_key = key;
  return 1;
}

int TRUST_TOKEN_ISSUER_set_metadata_key(TRUST_TOKEN_ISSUER *ctx,
                                        const uint8_t *key, size_t len) {
  if (len < 32) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_INVALID_METADATA_KEY);
    return 0;
  }
  OPENSSL_free(ctx->metadata_key);
  ctx->metadata_key_len = 0;
  ctx->metadata_key = OPENSSL_memdup(key, len);
  if (ctx->metadata_key == NULL) {
    return 0;
  }
  ctx->metadata_key_len = len;
  return 1;
}

static const struct trust_token_issuer_key_st *trust_token_issuer_get_key(
    const TRUST_TOKEN_ISSUER *ctx, uint32_t key_id) {
  for (size_t i = 0; i < ctx->num_keys; i++) {
    if (ctx->keys[i].id == key_id) {
      return &ctx->keys[i];
    }
  }
  return NULL;
}

int TRUST_TOKEN_ISSUER_issue(const TRUST_TOKEN_ISSUER *ctx, uint8_t **out,
                             size_t *out_len, size_t *out_tokens_issued,
                             const uint8_t *request, size_t request_len,
                             uint32_t public_metadata, uint8_t private_metadata,
                             size_t max_issuance) {
  if (max_issuance > ctx->max_batchsize) {
    max_issuance = ctx->max_batchsize;
  }

  const struct trust_token_issuer_key_st *key =
      trust_token_issuer_get_key(ctx, public_metadata);
  if (key == NULL || private_metadata > 1 ||
      (!ctx->method->has_private_metadata && private_metadata != 0)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_INVALID_METADATA);
    return 0;
  }

  CBS in;
  uint16_t num_requested;
  CBS_init(&in, request, request_len);
  if (!CBS_get_u16(&in, &num_requested)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    return 0;
  }

  size_t num_to_issue = num_requested;
  if (num_to_issue > max_issuance) {
    num_to_issue = max_issuance;
  }

  int ret = 0;
  CBB response;
  if (!CBB_init(&response, 0) ||
      !CBB_add_u16(&response, num_to_issue) ||
      !CBB_add_u32(&response, public_metadata)) {
    goto err;
  }

  if (!ctx->method->sign(&key->key, &response, &in, num_requested, num_to_issue,
                         private_metadata)) {
    goto err;
  }

  if (CBS_len(&in) != 0) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_FAILURE);
    goto err;
  }

  if (!CBB_finish(&response, out, out_len)) {
    goto err;
  }

  *out_tokens_issued = num_to_issue;
  ret = 1;

err:
  CBB_cleanup(&response);
  return ret;
}

static int trust_token_issuer_redeem_impl(
    const TRUST_TOKEN_ISSUER *ctx, uint32_t *out_public, uint8_t *out_private,
    TRUST_TOKEN **out_token, uint8_t **out_client_data,
    size_t *out_client_data_len, const uint8_t *request, size_t request_len,
    int include_message, const uint8_t *msg, size_t msg_len) {
  CBS request_cbs, token_cbs;
  CBS_init(&request_cbs, request, request_len);
  if (!CBS_get_u16_length_prefixed(&request_cbs, &token_cbs)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_ERROR);
    return 0;
  }

  uint32_t public_metadata = 0;
  uint8_t private_metadata = 0;

  // Parse the token. If there is an error, treat it as an invalid token.
  if (!CBS_get_u32(&token_cbs, &public_metadata)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_INVALID_TOKEN);
    return 0;
  }

  const struct trust_token_issuer_key_st *key =
      trust_token_issuer_get_key(ctx, public_metadata);
  uint8_t nonce[TRUST_TOKEN_NONCE_SIZE];
  if (key == NULL ||
      !ctx->method->read(&key->key, nonce, &private_metadata,
                         CBS_data(&token_cbs), CBS_len(&token_cbs),
                         include_message, msg, msg_len)) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_INVALID_TOKEN);
    return 0;
  }

  CBS client_data;
  if (!CBS_get_u16_length_prefixed(&request_cbs, &client_data) ||
      (ctx->method->has_srr && !CBS_skip(&request_cbs, 8)) ||
      CBS_len(&request_cbs) != 0) {
    OPENSSL_PUT_ERROR(TRUST_TOKEN, TRUST_TOKEN_R_DECODE_ERROR);
    return 0;
  }

  uint8_t *client_data_buf = NULL;
  size_t client_data_len = 0;
  if (!CBS_stow(&client_data, &client_data_buf, &client_data_len)) {
    goto err;
  }

  TRUST_TOKEN *token = TRUST_TOKEN_new(nonce, TRUST_TOKEN_NONCE_SIZE);
  if (token == NULL) {
    goto err;
  }
  *out_public = public_metadata;
  *out_private = private_metadata;
  *out_token = token;
  *out_client_data = client_data_buf;
  *out_client_data_len = client_data_len;

  return 1;

err:
  OPENSSL_free(client_data_buf);
  return 0;
}


int TRUST_TOKEN_ISSUER_redeem(const TRUST_TOKEN_ISSUER *ctx,
                              uint32_t *out_public, uint8_t *out_private,
                              TRUST_TOKEN **out_token,
                              uint8_t **out_client_data,
                              size_t *out_client_data_len,
                              const uint8_t *request, size_t request_len) {
  return trust_token_issuer_redeem_impl(ctx, out_public, out_private, out_token,
                                        out_client_data, out_client_data_len,
                                        request, request_len, 0, NULL, 0);
}

int TRUST_TOKEN_ISSUER_redeem_over_message(
    const TRUST_TOKEN_ISSUER *ctx, uint32_t *out_public, uint8_t *out_private,
    TRUST_TOKEN **out_token, uint8_t **out_client_data,
    size_t *out_client_data_len, const uint8_t *request, size_t request_len,
    const uint8_t *msg, size_t msg_len) {
  return trust_token_issuer_redeem_impl(ctx, out_public, out_private, out_token,
                                        out_client_data, out_client_data_len,
                                        request, request_len, 1, msg, msg_len);
}

static uint8_t get_metadata_obfuscator(const uint8_t *key, size_t key_len,
                                       const uint8_t *client_data,
                                       size_t client_data_len) {
  uint8_t metadata_obfuscator[SHA256_DIGEST_LENGTH];
  SHA256_CTX sha_ctx;
  SHA256_Init(&sha_ctx);
  SHA256_Update(&sha_ctx, key, key_len);
  SHA256_Update(&sha_ctx, client_data, client_data_len);
  SHA256_Final(metadata_obfuscator, &sha_ctx);
  return metadata_obfuscator[0] >> 7;
}

int TRUST_TOKEN_decode_private_metadata(const TRUST_TOKEN_METHOD *method,
                                        uint8_t *out_value, const uint8_t *key,
                                        size_t key_len, const uint8_t *nonce,
                                        size_t nonce_len,
                                        uint8_t encrypted_bit) {
  uint8_t metadata_obfuscator =
      get_metadata_obfuscator(key, key_len, nonce, nonce_len);
  *out_value = encrypted_bit ^ metadata_obfuscator;
  return 1;
}
