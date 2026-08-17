// Copyright 2006-2019 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include <string.h>
#include <openssl/evp.h>

#include <assert.h>

#include <openssl/dh.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "internal.h"


typedef struct dh_pkey_ctx_st {
  int pad;
  /* Parameter gen parameters */
  int prime_len;
  int generator;
} DH_PKEY_CTX;

static int pkey_dh_init(EVP_PKEY_CTX *ctx) {
  DH_PKEY_CTX *dctx = OPENSSL_zalloc(sizeof(DH_PKEY_CTX));
  if (dctx == NULL) {
    return 0;
  }
  // Default parameters
  dctx->prime_len = 2048;
  dctx->generator = DH_GENERATOR_2;

  ctx->data = dctx;
  return 1;
}

static int pkey_dh_copy(EVP_PKEY_CTX *dst, EVP_PKEY_CTX *src) {
  if (!pkey_dh_init(dst)) {
    return 0;
  }

  const DH_PKEY_CTX *sctx = src->data;
  DH_PKEY_CTX *dctx = dst->data;
  dctx->pad = sctx->pad;
  return 1;
}

static void pkey_dh_cleanup(EVP_PKEY_CTX *ctx) {
  OPENSSL_free(ctx->data);
  ctx->data = NULL;
}

static int pkey_dh_keygen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  DH *dh = DH_new();
  if (dh == NULL || !EVP_PKEY_assign_DH(pkey, dh)) {
    DH_free(dh);
    return 0;
  }

  if (ctx->pkey != NULL && !EVP_PKEY_copy_parameters(pkey, ctx->pkey)) {
    return 0;
  }

  return DH_generate_key(dh);
}

static int pkey_dh_derive(EVP_PKEY_CTX *ctx, uint8_t *out, size_t *out_len) {
  DH_PKEY_CTX *dctx = ctx->data;
  if (ctx->pkey == NULL || ctx->peerkey == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_KEYS_NOT_SET);
    return 0;
  }

  DH *our_key = ctx->pkey->pkey.dh;
  DH *peer_key = ctx->peerkey->pkey.dh;
  if (our_key == NULL || peer_key == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_KEYS_NOT_SET);
    return 0;
  }

  const BIGNUM *pub_key = DH_get0_pub_key(peer_key);
  if (pub_key == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_KEYS_NOT_SET);
    return 0;
  }

  if (out == NULL) {
    *out_len = DH_size(our_key);
    return 1;
  }

  if (*out_len < (size_t)DH_size(our_key)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  int ret = dctx->pad ? DH_compute_key_padded(out, pub_key, our_key)
                      : DH_compute_key(out, pub_key, our_key);
  if (ret < 0) {
    return 0;
  }

  assert(ret <= DH_size(our_key));
  *out_len = (size_t)ret;
  return 1;
}

static int pkey_dh_ctrl(EVP_PKEY_CTX *ctx, int type, int p1, void *_p2) {
  DH_PKEY_CTX *dctx = ctx->data;
  switch (type) {
    case EVP_PKEY_CTRL_PEER_KEY:
      // |EVP_PKEY_derive_set_peer| requires the key implement this command,
      // even if it is a no-op.
      return 1;

    case EVP_PKEY_CTRL_DH_PAD:
      dctx->pad = p1;
      return 1;

    case EVP_PKEY_CTRL_DH_PARAMGEN_PRIME_LEN:
      if(p1 < 256) {
        return -2;
      }
      dctx->prime_len = p1;
      return 1;

    case EVP_PKEY_CTRL_DH_PARAMGEN_GENERATOR:
      if(p1 < 2) {
        return -2;
      }
      dctx->generator = p1;
      return 1;

    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
      return 0;
  }
}

static int pkey_dh_paramgen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  int ret = 0;
  DH_PKEY_CTX *dctx = ctx->data;
  DH *dh = DH_new();
  if (dh == NULL) {
    return 0;
  }

  BN_GENCB *pkey_ctx_cb = NULL;
  if (ctx->pkey_gencb) {
    pkey_ctx_cb = BN_GENCB_new();
    if (pkey_ctx_cb == NULL) {
      goto end;
    }
    evp_pkey_set_cb_translate(pkey_ctx_cb, ctx);
  }

  ret = DH_generate_parameters_ex(dh, dctx->prime_len, dctx->generator, pkey_ctx_cb);
end:
  if (ret == 1) {
    EVP_PKEY_assign_DH(pkey, dh);
  } else {
    ret = 0;
    DH_free(dh);
  }
  BN_GENCB_free(pkey_ctx_cb);
  return ret;
}


static int pkey_dh_ctrl_str(EVP_PKEY_CTX *ctx, const char *type,
                            const char *value) {
  // We don't support:
  // * dh_rfc5114
  // * dh_param
  // * dh_paramgen_subprime_len
  // * dh_paramgen_type
  if (strcmp(type, "dh_paramgen_prime_len") == 0) {
    char* str_end = NULL;
    long prime_len = strtol(value, &str_end, 10);
    if(str_end == value || prime_len < 0 || prime_len > INT_MAX) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
      return 0;
    }
    return EVP_PKEY_CTX_set_dh_paramgen_prime_len(ctx, (int)prime_len);
  }

  if (strcmp(type, "dh_paramgen_generator") == 0) {
    char* str_end = NULL;
    long generator = strtol(value, &str_end, 10);
    if(str_end == value || generator < 0 || generator > INT_MAX) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
      return 0;
    }
    return EVP_PKEY_CTX_set_dh_paramgen_generator(ctx, (int)generator);
  }


  if (strcmp(type, "dh_pad") == 0) {
    char* str_end = NULL;
    long pad = strtol(value, &str_end, 10);
    if(str_end == value || pad < 0 || pad > INT_MAX) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
      return 0;
    }
    return EVP_PKEY_CTX_set_dh_pad(ctx, (int)pad);
  }
  return -2;
}


const EVP_PKEY_METHOD dh_pkey_meth = {
    .pkey_id = EVP_PKEY_DH,
    .init = pkey_dh_init,
    .copy = pkey_dh_copy,
    .cleanup = pkey_dh_cleanup,
    .keygen = pkey_dh_keygen,
    .derive = pkey_dh_derive,
    .paramgen = pkey_dh_paramgen,
    .ctrl = pkey_dh_ctrl,
    .ctrl_str = pkey_dh_ctrl_str
};

int EVP_PKEY_CTX_set_dh_pad(EVP_PKEY_CTX *ctx, int pad) {
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_DH, EVP_PKEY_OP_DERIVE,
                           EVP_PKEY_CTRL_DH_PAD, pad, NULL);
}

int EVP_PKEY_CTX_set_dh_paramgen_prime_len(EVP_PKEY_CTX *ctx, int pbits) {
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_DH, EVP_PKEY_OP_PARAMGEN,
                    EVP_PKEY_CTRL_DH_PARAMGEN_PRIME_LEN, pbits, NULL);
}
int EVP_PKEY_CTX_set_dh_paramgen_generator(EVP_PKEY_CTX *ctx, int gen) {
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_DH, EVP_PKEY_OP_PARAMGEN,
                   EVP_PKEY_CTRL_DH_PARAMGEN_GENERATOR, gen, NULL);
}
