// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2006.
// Copyright (c) 2006 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <string.h>

#include <openssl/bn.h>
#include <openssl/digest.h>
#include <openssl/ec.h>
#include <openssl/ec_key.h>
#include <openssl/ecdh.h>
#include <openssl/ecdsa.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>

#include "internal.h"
#include "../ec/internal.h"
#include "../../internal.h"


typedef struct {
  // message digest
  const EVP_MD *md;
  const EC_GROUP *gen_group;
} EC_PKEY_CTX;


static int pkey_ec_init(EVP_PKEY_CTX *ctx) {
  EC_PKEY_CTX *dctx;
  dctx = OPENSSL_zalloc(sizeof(EC_PKEY_CTX));
  if (!dctx) {
    return 0;
  }

  ctx->data = dctx;

  return 1;
}

static int pkey_ec_copy(EVP_PKEY_CTX *dst, EVP_PKEY_CTX *src) {
  if (!pkey_ec_init(dst)) {
    return 0;
  }

  const EC_PKEY_CTX *sctx = src->data;
  EC_PKEY_CTX *dctx = dst->data;
  dctx->md = sctx->md;
  dctx->gen_group = sctx->gen_group;
  return 1;
}

static void pkey_ec_cleanup(EVP_PKEY_CTX *ctx) {
  EC_PKEY_CTX *dctx = ctx->data;
  if (!dctx) {
    return;
  }

  OPENSSL_free(dctx);
}

static int pkey_ec_sign(EVP_PKEY_CTX *ctx, uint8_t *sig, size_t *siglen,
                        const uint8_t *tbs, size_t tbslen) {
  unsigned int sltmp;
  EC_KEY *ec = ctx->pkey->pkey.ec;

  if (!sig) {
    *siglen = ECDSA_size(ec);
    return 1;
  } else if (*siglen < (size_t)ECDSA_size(ec)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (!ECDSA_sign(0, tbs, tbslen, sig, &sltmp, ec)) {
    return 0;
  }
  *siglen = (size_t)sltmp;
  return 1;
}

static int pkey_ec_verify(EVP_PKEY_CTX *ctx, const uint8_t *sig, size_t siglen,
                          const uint8_t *tbs, size_t tbslen) {
  return ECDSA_verify(0, tbs, tbslen, sig, siglen, ctx->pkey->pkey.ec);
}

static int pkey_ec_derive(EVP_PKEY_CTX *ctx, uint8_t *key,
                          size_t *keylen) {
  int ret = 0;
  const EC_POINT *pubkey = NULL;
  EC_KEY *eckey;
  uint8_t buf[EC_MAX_BYTES];
  size_t buflen = sizeof(buf);

  if (!ctx->pkey || !ctx->peerkey) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_KEYS_NOT_SET);
    return 0;
  }

  eckey = ctx->pkey->pkey.ec;

  if (!key) {
    // This only outputs the expected |keylen| and does no actual crypto.
    const EC_GROUP *group;
    group = EC_KEY_get0_group(eckey);
    *keylen = (EC_GROUP_get_degree(group) + 7) / 8;
    return 1;
  }
  pubkey = EC_KEY_get0_public_key(ctx->peerkey->pkey.ec);

  // NB: unlike PKCS#3 DH, if the returned buflen is less than
  // the requested size in *keylen, this is not an error;
  // the result is truncated.
  // Note: This is an internal function which will not update
  // the service indicator.
  if (!ECDH_compute_shared_secret(buf, &buflen, pubkey, eckey)) {
      goto end;
  }

  if (buflen < *keylen) {
      *keylen = buflen;
  }
  OPENSSL_memcpy(key, buf, *keylen);

  // Insert service indicator check here because the pointer address cannot be
  // referenced from the higher level function |EVP_PKEY_derive|. |EC_KEY| is
  // is the only possible key that can do derivations.
  ECDH_verify_service_indicator(eckey);
  ret = 1;

end:
  OPENSSL_cleanse(buf, sizeof(buf));
  return ret;
}

static int pkey_ec_ctrl(EVP_PKEY_CTX *ctx, int type, int p1, void *p2) {
  EC_PKEY_CTX *dctx = ctx->data;

  switch (type) {
    case EVP_PKEY_CTRL_MD: {
      const EVP_MD *md = p2;
      int md_type = EVP_MD_type(md);
      if (md_type != NID_sha1 && md_type != NID_sha224 &&
          md_type != NID_sha256 && md_type != NID_sha384 &&
          md_type != NID_sha512 && md_type != NID_sha512_224 &&
          md_type != NID_sha512_256 && md_type != NID_sha3_224 &&
          md_type != NID_sha3_256 && md_type != NID_sha3_384 &&
          md_type != NID_sha3_512
      ) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_DIGEST_TYPE);
        return 0;
      }
      dctx->md = md;
      return 1;
    }

    case EVP_PKEY_CTRL_GET_MD:
      *(const EVP_MD **)p2 = dctx->md;
      return 1;

    case EVP_PKEY_CTRL_PEER_KEY:
      // Default behaviour is OK
      return 1;

    case EVP_PKEY_CTRL_EC_PARAMGEN_CURVE_NID: {
      const EC_GROUP *group = EC_GROUP_new_by_curve_name(p1);
      if (group == NULL) {
        return 0;
      }
      dctx->gen_group = group;
      return 1;
    }

    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
      return 0;
  }
}

static int pkey_ec_ctrl_str(EVP_PKEY_CTX *ctx, const char *type,
                            const char *value) {
  if (strcmp(type, "ec_paramgen_curve") == 0) {
    int nid;
    nid = EC_curve_nist2nid(value);
    if (nid == NID_undef) {
      nid = OBJ_sn2nid(value);
    }
    if (nid == NID_undef) {
      nid = OBJ_ln2nid(value);
    }
    if (nid == NID_undef) {
      OPENSSL_PUT_ERROR(EVP, EC_R_INVALID_ENCODING);
      return 0;
    }
    return EVP_PKEY_CTX_set_ec_paramgen_curve_nid(ctx, nid);
  }
  if (strcmp(type, "ec_param_enc") == 0) {
    int param_enc;
    // We don't support "explicit"
    if (strcmp(value, "named_curve") == 0) {
      param_enc = OPENSSL_EC_NAMED_CURVE;
    } else {
      return -2;
    }
    return EVP_PKEY_CTX_set_ec_param_enc(ctx, param_enc);
  }

  // We don't support "ecdh_kdf_md" nor "ecdh_cofactor_mode"

  return -2;
}

static int pkey_ec_keygen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  int ret = 0;
  EC_PKEY_CTX *dctx = ctx->data;
  const EC_GROUP *group = dctx->gen_group;
  if (group == NULL) {
    if (ctx->pkey == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
      return 0;
    }
    group = EC_KEY_get0_group(ctx->pkey->pkey.ec);
  }
  EC_KEY *ec = EC_KEY_new();
  // In FIPS build, |EC_KEY_generate_key_fips| updates the service indicator so lock it here
  FIPS_service_indicator_lock_state();
  if (ec == NULL ||
      !EC_KEY_set_group(ec, group) ||
      (!is_fips_build() && !EC_KEY_generate_key(ec)) ||
      ( is_fips_build() && !EC_KEY_generate_key_fips(ec))) {
    EC_KEY_free(ec);
    goto end;
  }

  EVP_PKEY_assign_EC_KEY(pkey, ec);
  ret = 1;
end:
  FIPS_service_indicator_unlock_state();
  return ret;
}

static int pkey_ec_paramgen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  EC_PKEY_CTX *dctx = ctx->data;
  if (dctx->gen_group == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_PARAMETERS_SET);
    return 0;
  }
  EC_KEY *ec = EC_KEY_new();
  if (ec == NULL ||
      !EC_KEY_set_group(ec, dctx->gen_group)) {
    EC_KEY_free(ec);
    return 0;
  }
  EVP_PKEY_assign_EC_KEY(pkey, ec);
  return 1;
}

DEFINE_METHOD_FUNCTION(EVP_PKEY_METHOD, EVP_PKEY_ec_pkey_meth) {
    out->pkey_id = EVP_PKEY_EC;
    out->init = pkey_ec_init;
    out->copy = pkey_ec_copy;
    out->cleanup = pkey_ec_cleanup;
    out->keygen = pkey_ec_keygen;
    out->sign_init = NULL; /* sign_init */
    out->sign = pkey_ec_sign;
    out->sign_message = NULL; /* sign_message */
    out->verify_init = NULL; /* verify_init */
    out->verify = pkey_ec_verify;
    out->verify_message = NULL; /* verify_message */
    out->verify_recover = NULL; /* verify_recover */
    out->encrypt = NULL; /* encrypt */
    out->decrypt = NULL; /* decrypt */
    out->derive = pkey_ec_derive;
    out->paramgen = pkey_ec_paramgen;
    out->ctrl = pkey_ec_ctrl;
    out->ctrl_str = pkey_ec_ctrl_str;
}

int EVP_PKEY_CTX_set_ec_paramgen_curve_nid(EVP_PKEY_CTX *ctx, int nid) {
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_EC, EVP_PKEY_OP_TYPE_GEN,
                           EVP_PKEY_CTRL_EC_PARAMGEN_CURVE_NID, nid, NULL);
}

int EVP_PKEY_CTX_set_ec_param_enc(EVP_PKEY_CTX *ctx, int encoding) {
  // BoringSSL only supports named curve syntax.
  if (encoding != OPENSSL_EC_NAMED_CURVE) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
    return 0;
  }
  return 1;
}
