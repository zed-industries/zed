// Copyright (c) 2022, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/evp.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/hkdf.h>
#include <openssl/kdf.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "internal.h"


typedef struct {
  int mode;
  const EVP_MD *md;
  uint8_t *key;
  size_t key_len;
  uint8_t *salt;
  size_t salt_len;
  CBB info;
} HKDF_PKEY_CTX;

static int pkey_hkdf_init(EVP_PKEY_CTX *ctx) {
  HKDF_PKEY_CTX *hctx = OPENSSL_zalloc(sizeof(HKDF_PKEY_CTX));
  if (hctx == NULL) {
    return 0;
  }

  if (!CBB_init(&hctx->info, 0)) {
    OPENSSL_free(hctx);
    return 0;
  }

  ctx->data = hctx;
  return 1;
}

static int pkey_hkdf_copy(EVP_PKEY_CTX *dst, EVP_PKEY_CTX *src) {
  if (!pkey_hkdf_init(dst)) {
    return 0;
  }

  HKDF_PKEY_CTX *hctx_dst = dst->data;
  const HKDF_PKEY_CTX *hctx_src = src->data;
  hctx_dst->mode = hctx_src->mode;
  hctx_dst->md = hctx_src->md;

  if (hctx_src->key_len != 0) {
    hctx_dst->key = OPENSSL_memdup(hctx_src->key, hctx_src->key_len);
    if (hctx_dst->key == NULL) {
      return 0;
    }
    hctx_dst->key_len = hctx_src->key_len;
  }

  if (hctx_src->salt_len != 0) {
    hctx_dst->salt = OPENSSL_memdup(hctx_src->salt, hctx_src->salt_len);
    if (hctx_dst->salt == NULL) {
      return 0;
    }
    hctx_dst->salt_len = hctx_src->salt_len;
  }

  if (!CBB_add_bytes(&hctx_dst->info, CBB_data(&hctx_src->info),
                     CBB_len(&hctx_src->info))) {
    return 0;
  }

  return 1;
}

static void pkey_hkdf_cleanup(EVP_PKEY_CTX *ctx) {
  HKDF_PKEY_CTX *hctx = ctx->data;
  if (hctx != NULL) {
    OPENSSL_free(hctx->key);
    OPENSSL_free(hctx->salt);
    CBB_cleanup(&hctx->info);
    OPENSSL_free(hctx);
    ctx->data = NULL;
  }
}

static int pkey_hkdf_derive(EVP_PKEY_CTX *ctx, uint8_t *out, size_t *out_len) {
  HKDF_PKEY_CTX *hctx = ctx->data;
  if (hctx->md == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_MISSING_PARAMETERS);
    return 0;
  }
  if (EVP_MD_size(hctx->md) <= 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
    return 0;
  }
  if (hctx->key_len == 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }

  if (out == NULL) {
    if (hctx->mode == EVP_PKEY_HKDEF_MODE_EXTRACT_ONLY) {
      *out_len = EVP_MD_size(hctx->md);
    }
    // HKDF-Expand is variable-length and returns |*out_len| bytes. "Output" the
    // input length by leaving it alone.
    return 1;
  }

  switch (hctx->mode) {
    case EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND:
      return HKDF(out, *out_len, hctx->md, hctx->key, hctx->key_len, hctx->salt,
                  hctx->salt_len, CBB_data(&hctx->info), CBB_len(&hctx->info));

    case EVP_PKEY_HKDEF_MODE_EXTRACT_ONLY:
      if (*out_len < EVP_MD_size(hctx->md)) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
        return 0;
      }
      return HKDF_extract(out, out_len, hctx->md, hctx->key, hctx->key_len,
                          hctx->salt, hctx->salt_len);

    case EVP_PKEY_HKDEF_MODE_EXPAND_ONLY:
      return HKDF_expand(out, *out_len, hctx->md, hctx->key, hctx->key_len,
                         CBB_data(&hctx->info), CBB_len(&hctx->info));
  }
  OPENSSL_PUT_ERROR(EVP, ERR_R_INTERNAL_ERROR);
  return 0;
}

static int pkey_hkdf_ctrl(EVP_PKEY_CTX *ctx, int type, int p1, void *p2) {
  HKDF_PKEY_CTX *hctx = ctx->data;
  switch (type) {
    case EVP_PKEY_CTRL_HKDF_MODE:
      if (p1 != EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND &&
          p1 != EVP_PKEY_HKDEF_MODE_EXTRACT_ONLY &&
          p1 != EVP_PKEY_HKDEF_MODE_EXPAND_ONLY) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
        return 0;
      }
      hctx->mode = p1;
      return 1;
    case EVP_PKEY_CTRL_HKDF_MD:
      if (p2 == NULL || EVP_MD_size((const EVP_MD *)p2) <= 0) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
        return 0;
      }
      if (EVP_MD_flags((const EVP_MD *)p2) & EVP_MD_FLAG_XOF) {
        OPENSSL_PUT_ERROR(EVP, HKDF_R_UNSUPPORTED_DIGEST);
        return 0;
      }
      hctx->md = (const EVP_MD *)p2;
      return 1;
    case EVP_PKEY_CTRL_HKDF_KEY: {
      const CBS *key = p2;
      if (!CBS_stow(key, &hctx->key, &hctx->key_len)) {
        return 0;
      }
      return 1;
    }
    case EVP_PKEY_CTRL_HKDF_SALT: {
      const CBS *salt = p2;
      if (!CBS_stow(salt, &hctx->salt, &hctx->salt_len)) {
        return 0;
      }
      return 1;
    }
    case EVP_PKEY_CTRL_HKDF_INFO: {
      const CBS *info = p2;
      // |EVP_PKEY_CTX_add1_hkdf_info| appends to the info string, rather than
      // replacing it.
      if (!CBB_add_bytes(&hctx->info, CBS_data(info), CBS_len(info))) {
        return 0;
      }
      return 1;
    }
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
      return 0;
  }
}

static int pkey_hkdf_ctrl_str(EVP_PKEY_CTX *ctx, const char *type,
                              const char *value) {
  if (strcmp(type, "mode") == 0) {
    int mode;

    if (strcmp(value, "EXTRACT_AND_EXPAND") == 0) {
      mode = EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND;
    } else if (strcmp(value, "EXTRACT_ONLY") == 0) {
      mode = EVP_PKEY_HKDEF_MODE_EXTRACT_ONLY;
    } else if (strcmp(value, "EXPAND_ONLY") == 0) {
      mode = EVP_PKEY_HKDEF_MODE_EXPAND_ONLY;
    } else {
      return 0;
    }

    return EVP_PKEY_CTX_hkdf_mode(ctx, mode);
  }

  if (strcmp(type, "md") == 0) {
    OPENSSL_BEGIN_ALLOW_DEPRECATED
    return EVP_PKEY_CTX_md(ctx, EVP_PKEY_OP_DERIVE, EVP_PKEY_CTRL_HKDF_MD,
                           value);
    OPENSSL_END_ALLOW_DEPRECATED
  }

  if (strcmp(type, "salt") == 0) {
    // What if the salt contains a 0-byte?
    const size_t saltlen = OPENSSL_strnlen(value, INT16_MAX);
    return EVP_PKEY_CTX_set1_hkdf_salt(ctx, (const uint8_t *)value, saltlen);
  }

  if (strcmp(type, "hexsalt") == 0) {
    size_t hex_saltlen = 0;
    uint8_t *salt = OPENSSL_hexstr2buf(value, &hex_saltlen);
    if (salt == NULL) {
      return 0;
    }
    int result = EVP_PKEY_CTX_set1_hkdf_salt(ctx, salt, hex_saltlen);
    OPENSSL_free(salt);
    return result;
  }

  if (strcmp(type, "key") == 0) {
    // What if the key contains a 0-byte?
    const size_t keylen = OPENSSL_strnlen(value, INT16_MAX);
    return EVP_PKEY_CTX_set1_hkdf_key(ctx, (const uint8_t *)value, keylen);
  }

  if (strcmp(type, "hexkey") == 0) {
    size_t hex_keylen = 0;
    uint8_t *key = OPENSSL_hexstr2buf(value, &hex_keylen);
    if (key == NULL) {
      return 0;
    }
    int result = EVP_PKEY_CTX_set1_hkdf_key(ctx, key, hex_keylen);
    OPENSSL_free(key);
    return result;
  }

  if (strcmp(type, "info") == 0) {
    // What if info contains a 0-byte?
    const size_t infolen = OPENSSL_strnlen(value, INT16_MAX);
    return EVP_PKEY_CTX_add1_hkdf_info(ctx, (const uint8_t *)value, infolen);
  }

  if (strcmp(type, "hexinfo") == 0) {
    size_t hex_infolen = 0;
    uint8_t *info = OPENSSL_hexstr2buf(value, &hex_infolen);
    if (info == NULL) {
      return 0;
    }
    int result = EVP_PKEY_CTX_add1_hkdf_info(ctx, info, hex_infolen);
    OPENSSL_free(info);
    return result;
  }

  return -2;
}

DEFINE_METHOD_FUNCTION(EVP_PKEY_METHOD, EVP_PKEY_hkdf_pkey_meth) {
    out->pkey_id = EVP_PKEY_HKDF;
    out->init = pkey_hkdf_init;
    out->copy = pkey_hkdf_copy;
    out->cleanup = pkey_hkdf_cleanup;
    out->keygen = NULL; /* keygen */
    out->sign_init = NULL; /* sign_init */
    out->sign = NULL; /* sign */
    out->sign_message = NULL; /* sign_message */
    out->verify_init = NULL; /* verify_init */
    out->verify = NULL; /* verify */
    out->verify_message = NULL; /* verify_message */
    out->verify_recover = NULL; /* verify_recover */
    out->encrypt = NULL; /* encrypt */
    out->decrypt = NULL; /* decrypt */
    out->derive = pkey_hkdf_derive;
    out->paramgen = NULL; /* paramgen */
    out->ctrl = pkey_hkdf_ctrl;
    out->ctrl_str = pkey_hkdf_ctrl_str;
}

int EVP_PKEY_CTX_hkdf_mode(EVP_PKEY_CTX *ctx, int mode) {
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_HKDF, EVP_PKEY_OP_DERIVE,
                           EVP_PKEY_CTRL_HKDF_MODE, mode, NULL);
}

int EVP_PKEY_CTX_set_hkdf_md(EVP_PKEY_CTX *ctx, const EVP_MD *md) {
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_HKDF, EVP_PKEY_OP_DERIVE,
                           EVP_PKEY_CTRL_HKDF_MD, 0, (void *)md);
}

int EVP_PKEY_CTX_set1_hkdf_key(EVP_PKEY_CTX *ctx, const uint8_t *key,
                               size_t key_len) {
  SET_DIT_AUTO_RESET;
  CBS cbs;
  CBS_init(&cbs, key, key_len);
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_HKDF, EVP_PKEY_OP_DERIVE,
                           EVP_PKEY_CTRL_HKDF_KEY, 0, &cbs);
}

int EVP_PKEY_CTX_set1_hkdf_salt(EVP_PKEY_CTX *ctx, const uint8_t *salt,
                                size_t salt_len) {
  CBS cbs;
  CBS_init(&cbs, salt, salt_len);
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_HKDF, EVP_PKEY_OP_DERIVE,
                           EVP_PKEY_CTRL_HKDF_SALT, 0, &cbs);
}

int EVP_PKEY_CTX_add1_hkdf_info(EVP_PKEY_CTX *ctx, const uint8_t *info,
                                size_t info_len) {
  CBS cbs;
  CBS_init(&cbs, info, info_len);
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_HKDF, EVP_PKEY_OP_DERIVE,
                           EVP_PKEY_CTRL_HKDF_INFO, 0, &cbs);
}
