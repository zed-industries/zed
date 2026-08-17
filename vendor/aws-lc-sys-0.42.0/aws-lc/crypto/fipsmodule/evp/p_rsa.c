// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2006.
// Copyright (c) 2006 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <limits.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>
#include <openssl/rsa.h>

#include "internal.h"
#include "../rsa/internal.h"
#include "../../rsa_extra/internal.h"

#define NO_PSS_SALT_LEN_RESTRICTION -1

typedef struct {
  // Key gen parameters
  int nbits;
  BIGNUM *pub_exp;
  // RSA padding mode
  int pad_mode;
  // message digest
  const EVP_MD *md;
  // message digest for MGF1
  const EVP_MD *mgf1md;
  // PSS salt length
  int saltlen;
  // Minimum salt length or NO_PSS_SALT_LEN_RESTRICTION.
  int min_saltlen;
  // tbuf is a buffer which is either NULL, or is the size of the RSA modulus.
  // It's used to store the output of RSA operations.
  uint8_t *tbuf;
  // OAEP label
  uint8_t *oaep_label;
  size_t oaep_labellen;
} RSA_PKEY_CTX;

typedef struct {
  uint8_t *data;
  size_t len;
} RSA_OAEP_LABEL_PARAMS;

static int pkey_ctx_is_pss(EVP_PKEY_CTX *ctx) {
  return ctx->pmeth->pkey_id == EVP_PKEY_RSA_PSS;
}

// This method checks if the NID of |s_md| is the same as the NID of |k_md| when
// |pkey_ctx_is_pss(ctx)| is true and there is PSS restriction, which means
// |min_saltlen| != |NO_PSS_SALT_LEN_RESTRICTION|.
static int pss_hash_algorithm_match(EVP_PKEY_CTX *ctx, int min_saltlen,
                                    const EVP_MD *k_md, const EVP_MD *s_md) {
  if (pkey_ctx_is_pss(ctx) && min_saltlen != NO_PSS_SALT_LEN_RESTRICTION) {
    if (k_md != NULL && s_md != NULL) {
      return EVP_MD_type(k_md) == EVP_MD_type(s_md);
    } else {
      return 0;
    }
  }
  return 1;
}

// Set PSS parameters when generating a key, if necessary.
static int rsa_set_pss_param(RSA *rsa, EVP_PKEY_CTX *ctx) {
  if (!pkey_ctx_is_pss(ctx)) {
    return 1;
  }
  RSA_PKEY_CTX *rctx = ctx->data;
  return RSASSA_PSS_PARAMS_create(rctx->md, rctx->mgf1md, rctx->saltlen, &(rsa->pss));
}

// Called for PSS sign or verify initialisation: checks PSS parameter
// sanity and sets any restrictions on key usage.
static int pkey_pss_init(EVP_PKEY_CTX *ctx) {
  RSA *rsa;
  RSA_PKEY_CTX *rctx = ctx->data;
  const EVP_MD *md = NULL;
  const EVP_MD *mgf1md = NULL;
  int min_saltlen, max_saltlen;
  // Should never happen.
  if (!pkey_ctx_is_pss(ctx)) {
    return 0;
  }
  if (ctx->pkey == NULL) {
    return 0;
  }
  rsa = ctx->pkey->pkey.rsa;
  // If no restrictions just return.
  if (rsa->pss == NULL) {
    return 1;
  }
  // Get and check parameters.
  if (!RSASSA_PSS_PARAMS_get(rsa->pss, &md, &mgf1md, &min_saltlen)) {
    return 0;
  }

  // See if minimum salt length exceeds maximum possible.
  // 8.1.1. Step1 https://tools.ietf.org/html/rfc8017#section-8.1.1
  // 9.1.1. Step3 https://tools.ietf.org/html/rfc8017#section-9.1.1
  max_saltlen = RSA_size(rsa) - EVP_MD_size(md) - 2;
  if ((RSA_bits(rsa) & 0x7) == 1) {
    max_saltlen--;
  }
  if (min_saltlen > max_saltlen) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PSS_SALT_LEN);
    return 0;
  }

  // Set PSS restrictions as defaults: we can then block any attempt to
  // use invalid values in pkey_rsa_ctrl
  rctx->md = md;
  rctx->mgf1md = mgf1md;
  rctx->saltlen = min_saltlen;
  rctx->min_saltlen = min_saltlen;

  return 1;
}

// |pkey_pss_init| was assigned to both the sign and verify operations
// of the |EVP_PKEY_RSA_PSS| methods. This created an unwanted assembler
// optimization for the gcc-8 FIPS static release build on Ubuntu x86_64.
// The gcc-8 assembler will attempt to optimize function pointers used in
// multiple places under a ".data.rel.ro.local" section, but "delocate.go"
// does not have the ability to handle ".data" sections. Splitting
// |pkey_pss_init| into two functions: |pkey_pss_init_sign| and
// |pkey_pss_init_verify|, gets around this undesired behaviour.
static int pkey_pss_init_sign(EVP_PKEY_CTX *ctx) {
  return pkey_pss_init(ctx);
}

static int pkey_pss_init_verify(EVP_PKEY_CTX *ctx) {
  return pkey_pss_init(ctx);
}

static int pkey_rsa_init(EVP_PKEY_CTX *ctx) {
  RSA_PKEY_CTX *rctx;
  rctx = OPENSSL_zalloc(sizeof(RSA_PKEY_CTX));
  if (!rctx) {
    return 0;
  }

  rctx->nbits = 2048;
  if (pkey_ctx_is_pss(ctx)) {
    rctx->pad_mode = RSA_PKCS1_PSS_PADDING;
  } else {
    rctx->pad_mode = RSA_PKCS1_PADDING;
  }
  rctx->saltlen = -2;
  rctx->min_saltlen = NO_PSS_SALT_LEN_RESTRICTION;

  ctx->data = rctx;

  return 1;
}

static int pkey_rsa_copy(EVP_PKEY_CTX *dst, EVP_PKEY_CTX *src) {
  RSA_PKEY_CTX *dctx, *sctx;
  if (!pkey_rsa_init(dst)) {
    return 0;
  }
  sctx = src->data;
  dctx = dst->data;
  dctx->nbits = sctx->nbits;
  if (sctx->pub_exp) {
    dctx->pub_exp = BN_dup(sctx->pub_exp);
    if (!dctx->pub_exp) {
      return 0;
    }
  }

  dctx->pad_mode = sctx->pad_mode;
  dctx->md = sctx->md;
  dctx->mgf1md = sctx->mgf1md;
  dctx->saltlen = sctx->saltlen;
  if (sctx->oaep_label) {
    OPENSSL_free(dctx->oaep_label);
    dctx->oaep_label = OPENSSL_memdup(sctx->oaep_label, sctx->oaep_labellen);
    if (!dctx->oaep_label) {
      return 0;
    }
    dctx->oaep_labellen = sctx->oaep_labellen;
  }

  return 1;
}

static void pkey_rsa_cleanup(EVP_PKEY_CTX *ctx) {
  RSA_PKEY_CTX *rctx = ctx->data;

  if (rctx == NULL) {
    return;
  }

  BN_free(rctx->pub_exp);
  OPENSSL_free(rctx->tbuf);
  OPENSSL_free(rctx->oaep_label);
  OPENSSL_free(rctx);
}

static int setup_tbuf(RSA_PKEY_CTX *ctx, EVP_PKEY_CTX *pk) {
  if (ctx->tbuf) {
    return 1;
  }
  ctx->tbuf = OPENSSL_malloc(EVP_PKEY_size(pk->pkey));
  if (!ctx->tbuf) {
    return 0;
  }
  return 1;
}

static int pkey_rsa_sign(EVP_PKEY_CTX *ctx, uint8_t *sig, size_t *siglen,
                         const uint8_t *tbs, size_t tbslen) {
  RSA_PKEY_CTX *rctx = ctx->data;
  RSA *rsa = ctx->pkey->pkey.rsa;
  const size_t key_len = EVP_PKEY_size(ctx->pkey);

  if (!sig) {
    *siglen = key_len;
    return 1;
  }

  if (*siglen < key_len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (rctx->md) {
    unsigned out_len;
    switch (rctx->pad_mode) {
      case RSA_PKCS1_PADDING:
        if (!RSA_sign(EVP_MD_type(rctx->md), tbs, tbslen, sig, &out_len, rsa)) {
          return 0;
        }
        *siglen = out_len;
        return 1;

      case RSA_PKCS1_PSS_PADDING:
        return RSA_sign_pss_mgf1(rsa, siglen, sig, *siglen, tbs, tbslen,
                                 rctx->md, rctx->mgf1md, rctx->saltlen);

      default:
        return 0;
    }
  }

  return RSA_sign_raw(rsa, siglen, sig, *siglen, tbs, tbslen, rctx->pad_mode);
}

static int pkey_rsa_verify(EVP_PKEY_CTX *ctx, const uint8_t *sig,
                           size_t siglen, const uint8_t *tbs,
                           size_t tbslen) {
  RSA_PKEY_CTX *rctx = ctx->data;
  RSA *rsa = ctx->pkey->pkey.rsa;

  if (rctx->md) {
    switch (rctx->pad_mode) {
      case RSA_PKCS1_PADDING:
        return RSA_verify(EVP_MD_type(rctx->md), tbs, tbslen, sig, siglen, rsa);

      case RSA_PKCS1_PSS_PADDING:
        return RSA_verify_pss_mgf1(rsa, tbs, tbslen, rctx->md, rctx->mgf1md,
                                   rctx->saltlen, sig, siglen);

      default:
        return 0;
    }
  }

  size_t rslen;
  const size_t key_len = EVP_PKEY_size(ctx->pkey);
  if (!setup_tbuf(rctx, ctx) ||
      !RSA_verify_raw(rsa, &rslen, rctx->tbuf, key_len, sig, siglen,
                      rctx->pad_mode) ||
      rslen != tbslen ||
      CRYPTO_memcmp(tbs, rctx->tbuf, rslen) != 0) {
    return 0;
  }

  return 1;
}

static int pkey_rsa_verify_recover(EVP_PKEY_CTX *ctx, uint8_t *out,
                                   size_t *out_len, const uint8_t *sig,
                                   size_t sig_len) {
  RSA_PKEY_CTX *rctx = ctx->data;
  RSA *rsa = ctx->pkey->pkey.rsa;
  const size_t key_len = EVP_PKEY_size(ctx->pkey);

  if (out == NULL) {
    *out_len = key_len;
    return 1;
  }

  if (*out_len < key_len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (rctx->md == NULL) {
    return RSA_verify_raw(rsa, out_len, out, *out_len, sig, sig_len,
                          rctx->pad_mode);
  }

  if (rctx->pad_mode != RSA_PKCS1_PADDING) {
    return 0;
  }

  // Assemble the encoded hash, using a placeholder hash value.
  static const uint8_t kDummyHash[EVP_MAX_MD_SIZE] = {0};
  const size_t hash_len = EVP_MD_size(rctx->md);
  uint8_t *asn1_prefix;
  size_t asn1_prefix_len;
  int asn1_prefix_allocated;
  if (!setup_tbuf(rctx, ctx) ||
      !RSA_add_pkcs1_prefix(&asn1_prefix, &asn1_prefix_len,
                            &asn1_prefix_allocated, EVP_MD_type(rctx->md),
                            kDummyHash, hash_len)) {
    return 0;
  }

  size_t rslen;
  int ok = 1;
  if (!RSA_verify_raw(rsa, &rslen, rctx->tbuf, key_len, sig, sig_len,
                      RSA_PKCS1_PADDING) ||
      rslen != asn1_prefix_len ||
      // Compare all but the hash suffix.
      CRYPTO_memcmp(rctx->tbuf, asn1_prefix, asn1_prefix_len - hash_len) != 0) {
    ok = 0;
  }

  if (asn1_prefix_allocated) {
    OPENSSL_free(asn1_prefix);
  }

  if (!ok) {
    return 0;
  }

  if (out != NULL) {
    OPENSSL_memcpy(out, rctx->tbuf + rslen - hash_len, hash_len);
  }
  *out_len = hash_len;

  return 1;
}

static int pkey_rsa_encrypt(EVP_PKEY_CTX *ctx, uint8_t *out, size_t *outlen,
                            const uint8_t *in, size_t inlen) {
  RSA_PKEY_CTX *rctx = ctx->data;
  RSA *rsa = ctx->pkey->pkey.rsa;
  const size_t key_len = EVP_PKEY_size(ctx->pkey);

  if (!out) {
    *outlen = key_len;
    return 1;
  }

  if (*outlen < key_len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (rctx->pad_mode == RSA_PKCS1_OAEP_PADDING) {
    if (!setup_tbuf(rctx, ctx) ||
        !RSA_padding_add_PKCS1_OAEP_mgf1(rctx->tbuf, key_len, in, inlen,
                                         rctx->oaep_label, rctx->oaep_labellen,
                                         rctx->md, rctx->mgf1md) ||
        !RSA_encrypt(rsa, outlen, out, *outlen, rctx->tbuf, key_len,
                     RSA_NO_PADDING)) {
      return 0;
    }
    return 1;
  }

  return RSA_encrypt(rsa, outlen, out, *outlen, in, inlen, rctx->pad_mode);
}

static int pkey_rsa_decrypt(EVP_PKEY_CTX *ctx, uint8_t *out,
                            size_t *outlen, const uint8_t *in,
                            size_t inlen) {
  RSA_PKEY_CTX *rctx = ctx->data;
  RSA *rsa = ctx->pkey->pkey.rsa;
  const size_t key_len = EVP_PKEY_size(ctx->pkey);

  if (!out) {
    *outlen = key_len;
    return 1;
  }

  if (*outlen < key_len) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  if (rctx->pad_mode == RSA_PKCS1_OAEP_PADDING) {
    size_t padded_len;
    if (!setup_tbuf(rctx, ctx) ||
        !RSA_decrypt(rsa, &padded_len, rctx->tbuf, key_len, in, inlen,
                     RSA_NO_PADDING) ||
        !RSA_padding_check_PKCS1_OAEP_mgf1(
            out, outlen, key_len, rctx->tbuf, padded_len, rctx->oaep_label,
            rctx->oaep_labellen, rctx->md, rctx->mgf1md)) {
      return 0;
    }
    return 1;
  }

  return RSA_decrypt(rsa, outlen, out, key_len, in, inlen, rctx->pad_mode);
}

static int check_padding_md(const EVP_MD *md, int padding) {
  if (!md) {
    return 1;
  }

  if (padding == RSA_NO_PADDING) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PADDING_MODE);
    return 0;
  }

  return 1;
}

static int is_known_padding(int padding_mode) {
  switch (padding_mode) {
    case RSA_PKCS1_PADDING:
    case RSA_NO_PADDING:
    case RSA_PKCS1_OAEP_PADDING:
    case RSA_PKCS1_PSS_PADDING:
      return 1;
    default:
      return 0;
  }
}

static int pkey_rsa_ctrl(EVP_PKEY_CTX *ctx, int type, int p1, void *p2) {
  RSA_PKEY_CTX *rctx = ctx->data;
  switch (type) {
    case EVP_PKEY_CTRL_RSA_PADDING:
      if (!is_known_padding(p1) || !check_padding_md(rctx->md, p1) ||
          (p1 == RSA_PKCS1_PSS_PADDING &&
           0 == (ctx->operation & (EVP_PKEY_OP_SIGN | EVP_PKEY_OP_VERIFY))) ||
          (p1 == RSA_PKCS1_OAEP_PADDING &&
           0 == (ctx->operation & EVP_PKEY_OP_TYPE_CRYPT))) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_ILLEGAL_OR_UNSUPPORTED_PADDING_MODE);
        return 0;
      }
      if (p1 != RSA_PKCS1_PSS_PADDING && pkey_ctx_is_pss(ctx)) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_ILLEGAL_OR_UNSUPPORTED_PADDING_MODE);
        return 0;
      }
      if ((p1 == RSA_PKCS1_PSS_PADDING || p1 == RSA_PKCS1_OAEP_PADDING) &&
          rctx->md == NULL) {
        rctx->md = EVP_sha1();
      }
      rctx->pad_mode = p1;
      return 1;

    case EVP_PKEY_CTRL_GET_RSA_PADDING:
      *(int *)p2 = rctx->pad_mode;
      return 1;

    case EVP_PKEY_CTRL_RSA_PSS_SALTLEN:
    case EVP_PKEY_CTRL_GET_RSA_PSS_SALTLEN:
      if (rctx->pad_mode != RSA_PKCS1_PSS_PADDING) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PSS_SALTLEN);
        return 0;
      }
      if (type == EVP_PKEY_CTRL_GET_RSA_PSS_SALTLEN) {
        *(int *)p2 = rctx->saltlen;
      } else {
        // |p1| can be |-2|, |-1| and non-negative.
        // The functions of these values are mentioned in the API doc of
        // |EVP_PKEY_CTX_set_rsa_pss_saltlen| in |evp.h|.
        // Accordingly, |-2| is the smallest value that |p1| can be.
        if (p1 < -2) {
          return 0;
        }
        int min_saltlen = rctx->min_saltlen;
        if (min_saltlen != NO_PSS_SALT_LEN_RESTRICTION) {
          // Check |min_saltlen| when |p1| is -1.
          if ((p1 == RSA_PSS_SALTLEN_DIGEST &&
               (size_t)min_saltlen > EVP_MD_size(rctx->md)) ||
              // Check |min_saltlen| when |p1| is the value gives the size of
              // the salt in bytes.
              (p1 >= 0 && p1 < min_saltlen)) {
            OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PSS_SALTLEN);
            return 0;
          }
        }
        rctx->saltlen = p1;
      }
      return 1;

    case EVP_PKEY_CTRL_RSA_KEYGEN_BITS:
      if (p1 < 256) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_KEYBITS);
        return 0;
      }
      rctx->nbits = p1;
      return 1;

    case EVP_PKEY_CTRL_RSA_KEYGEN_PUBEXP:
      if (!p2) {
        return 0;
      }
#if defined(AWSLC_FIPS)
      if (BN_get_word(p2) != RSA_F4) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
        return 0;
      }
#endif
      BN_free(rctx->pub_exp);
      rctx->pub_exp = p2;
      return 1;
    case EVP_PKEY_CTRL_RSA_OAEP_MD:
    case EVP_PKEY_CTRL_GET_RSA_OAEP_MD:
      if (rctx->pad_mode != RSA_PKCS1_OAEP_PADDING) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PADDING_MODE);
        return 0;
      }
      if (type == EVP_PKEY_CTRL_GET_RSA_OAEP_MD) {
        *(const EVP_MD **)p2 = rctx->md;
      } else {
        rctx->md = p2;
      }
      return 1;

    case EVP_PKEY_CTRL_MD:
      if (!check_padding_md(p2, rctx->pad_mode)) {
        return 0;
      }
      // Check if the hashAlgorithm is matched.
      // Sec 3.3 https://tools.ietf.org/html/rfc4055#section-3.3
      if (!pss_hash_algorithm_match(ctx, rctx->min_saltlen, rctx->md, p2)) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PSS_MD);
        return 0;
      }
      rctx->md = p2;
      return 1;

    case EVP_PKEY_CTRL_GET_MD:
      *(const EVP_MD **)p2 = rctx->md;
      return 1;

    case EVP_PKEY_CTRL_RSA_MGF1_MD:
    case EVP_PKEY_CTRL_GET_RSA_MGF1_MD:
      if (rctx->pad_mode != RSA_PKCS1_PSS_PADDING &&
          rctx->pad_mode != RSA_PKCS1_OAEP_PADDING) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_MGF1_MD);
        return 0;
      }
      if (type == EVP_PKEY_CTRL_GET_RSA_MGF1_MD) {
        if (rctx->mgf1md) {
          *(const EVP_MD **)p2 = rctx->mgf1md;
        } else {
          *(const EVP_MD **)p2 = rctx->md;
        }
      } else {
        // Check if the hashAlgorithm is matched.
        // Sec 3.3 https://tools.ietf.org/html/rfc4055#section-3.3
        if (!pss_hash_algorithm_match(ctx, rctx->min_saltlen, rctx->mgf1md, p2)) {
          OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_MGF1_MD);
          return 0;
        }
        rctx->mgf1md = p2;
      }
      return 1;

    case EVP_PKEY_CTRL_RSA_OAEP_LABEL: {
      if (rctx->pad_mode != RSA_PKCS1_OAEP_PADDING) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PADDING_MODE);
        return 0;
      }
      OPENSSL_free(rctx->oaep_label);
      RSA_OAEP_LABEL_PARAMS *params = p2;
      rctx->oaep_label = params->data;
      rctx->oaep_labellen = params->len;
      return 1;
    }

    case EVP_PKEY_CTRL_GET_RSA_OAEP_LABEL:
      if (rctx->pad_mode != RSA_PKCS1_OAEP_PADDING) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PADDING_MODE);
        return 0;
      }
      CBS_init((CBS *)p2, rctx->oaep_label, rctx->oaep_labellen);
      return 1;

    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
      return 0;
  }
}

static int pkey_rsa_keygen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  int ret = 0;
  RSA *rsa = NULL;
  RSA_PKEY_CTX *rctx = ctx->data;
  BN_GENCB *pkey_ctx_cb = NULL;

  // In FIPS mode, the public exponent is set within |RSA_generate_key_fips|
  if (!is_fips_build() && !rctx->pub_exp) {
    rctx->pub_exp = BN_new();
    if (!rctx->pub_exp || !BN_set_word(rctx->pub_exp, RSA_F4)) {
      goto end;
    }
  }
  rsa = RSA_new();
  if (!rsa) {
    goto end;
  }

  if (ctx->pkey_gencb) {
    pkey_ctx_cb = BN_GENCB_new();
    if (pkey_ctx_cb == NULL) {
      goto end;
    }
    evp_pkey_set_cb_translate(pkey_ctx_cb, ctx);
  }

  // In FIPS build, |RSA_generate_key_fips| updates the service indicator so lock it here
  FIPS_service_indicator_lock_state();
  if ((!is_fips_build() &&
       !RSA_generate_key_ex(rsa, rctx->nbits, rctx->pub_exp, pkey_ctx_cb)) ||
      (is_fips_build() &&
       !RSA_generate_key_fips(rsa, rctx->nbits, pkey_ctx_cb)) ||
      !rsa_set_pss_param(rsa, ctx)) {
    FIPS_service_indicator_unlock_state();
    goto end;
  }
  FIPS_service_indicator_unlock_state();

  if (pkey_ctx_is_pss(ctx)) {
    ret = EVP_PKEY_assign(pkey, EVP_PKEY_RSA_PSS, rsa);
  } else {
    ret = EVP_PKEY_assign_RSA(pkey, rsa);
  }

end:
  BN_GENCB_free(pkey_ctx_cb);
  if (!ret && rsa) {
    RSA_free(rsa);
  }
  return ret;
}

static int pkey_rsa_ctrl_str(EVP_PKEY_CTX *ctx, const char *type,
                             const char *value) {
  if (value == NULL) {
    OPENSSL_PUT_ERROR(EVP, RSA_R_VALUE_MISSING);
    return 0;
  }
  if (strcmp(type, "rsa_padding_mode") == 0) {
    // "sslv23" and "x931" are not supported
    int pm;

    if (strcmp(value, "pkcs1") == 0) {
      pm = RSA_PKCS1_PADDING;
    } else if (strcmp(value, "none") == 0) {
      pm = RSA_NO_PADDING;
    // OpenSSL also supports the typo.
    } else if (strcmp(value, "oeap") == 0) {
      pm = RSA_PKCS1_OAEP_PADDING;
    } else if (strcmp(value, "oaep") == 0) {
      pm = RSA_PKCS1_OAEP_PADDING;
    } else if (strcmp(value, "pss") == 0) {
      pm = RSA_PKCS1_PSS_PADDING;
    } else {
      OPENSSL_PUT_ERROR(EVP, RSA_R_UNKNOWN_PADDING_TYPE);
      return -2;
    }
    return EVP_PKEY_CTX_set_rsa_padding(ctx, pm);
  }

  if (strcmp(type, "rsa_pss_saltlen") == 0) {
    // "max" and "auto" are not supported
    long saltlen;

    // A value of "digest" or "-1" causes the salt to be the same length as the
    // digest in the signature
    if (!strcmp(value, "digest") || !strcmp(value, "-1")) {
      saltlen = RSA_PSS_SALTLEN_DIGEST;
    } else {
      char* str_end;
      saltlen = strtol(value, &str_end, 10);
      if(str_end == value || saltlen < 0 || saltlen > INT_MAX) {
        OPENSSL_PUT_ERROR(EVP, RSA_R_INTERNAL_ERROR);
        return -2;
      }
    }
    return EVP_PKEY_CTX_set_rsa_pss_saltlen(ctx, (int)saltlen);
  }

  if (strcmp(type, "rsa_keygen_bits") == 0) {
    char* str_end;
    long nbits = strtol(value, &str_end, 10);
    if (str_end == value || nbits <= 0 || nbits > INT_MAX) {
      OPENSSL_PUT_ERROR(EVP, RSA_R_INTERNAL_ERROR);
      return -2;
    }
    return EVP_PKEY_CTX_set_rsa_keygen_bits(ctx, (int)nbits);
  }

  if (strcmp(type, "rsa_keygen_pubexp") == 0) {
    int ret;

    BIGNUM *pubexp = NULL;
    if (!BN_asc2bn(&pubexp, value)) {
      return -2;
    }
    ret = EVP_PKEY_CTX_set_rsa_keygen_pubexp(ctx, pubexp);
    if (ret <= 0) {
      BN_free(pubexp);
    }
    return ret;
  }

  if (strcmp(type, "rsa_mgf1_md") == 0) {
    OPENSSL_BEGIN_ALLOW_DEPRECATED
    return EVP_PKEY_CTX_md(ctx, EVP_PKEY_OP_TYPE_SIG | EVP_PKEY_OP_TYPE_CRYPT,
                           EVP_PKEY_CTRL_RSA_MGF1_MD, value);
    OPENSSL_END_ALLOW_DEPRECATED
  }

  // rsa_pss_keygen_XXX options are not supported

  if (strcmp(type, "rsa_oaep_md") == 0) {
    OPENSSL_BEGIN_ALLOW_DEPRECATED
    return EVP_PKEY_CTX_md(ctx, EVP_PKEY_OP_TYPE_CRYPT,
                           EVP_PKEY_CTRL_RSA_OAEP_MD, value);
    OPENSSL_END_ALLOW_DEPRECATED
  }
  if (strcmp(type, "rsa_oaep_label") == 0) {
    size_t lablen = 0;

    uint8_t *lab = OPENSSL_hexstr2buf(value, &lablen);
    if (lab == NULL) {
      return 0;
    }

    int ret = EVP_PKEY_CTX_set0_rsa_oaep_label(ctx, lab, lablen);
    if (ret <= 0) {
      OPENSSL_free(lab);
    }
    return ret;
  }

  return -2;
}

DEFINE_METHOD_FUNCTION(EVP_PKEY_METHOD, EVP_PKEY_rsa_pkey_meth) {
    out->pkey_id = EVP_PKEY_RSA;
    out->init = pkey_rsa_init;
    out->copy = pkey_rsa_copy;
    out->cleanup = pkey_rsa_cleanup;
    out->keygen = pkey_rsa_keygen;
    out->sign_init = NULL; /* sign_init */
    out->sign = pkey_rsa_sign;
    out->sign_message = NULL; /* sign_message */
    out->verify_init = NULL; /* verify_init */
    out->verify = pkey_rsa_verify;
    out->verify_message = NULL; /* verify_message */
    out->verify_recover = pkey_rsa_verify_recover; /* verify_recover */
    out->encrypt = pkey_rsa_encrypt; /* encrypt */
    out->decrypt = pkey_rsa_decrypt; /* decrypt */
    out->derive = NULL;
    out->paramgen = NULL;
    out->ctrl = pkey_rsa_ctrl;
    out->ctrl_str = pkey_rsa_ctrl_str;
}

DEFINE_METHOD_FUNCTION(EVP_PKEY_METHOD, EVP_PKEY_rsa_pss_pkey_meth) {
    out->pkey_id = EVP_PKEY_RSA_PSS;
    out->init = pkey_rsa_init;
    out->copy = pkey_rsa_copy;
    out->cleanup = pkey_rsa_cleanup;
    out->keygen = pkey_rsa_keygen;
    out->sign_init = pkey_pss_init_sign; /* sign_init */
    out->sign = pkey_rsa_sign;
    out->sign_message = NULL; /* sign_message */
    out->verify_init = pkey_pss_init_verify; /* verify_init */
    out->verify = pkey_rsa_verify;
    out->verify_message = NULL; /* verify_message */
    out->verify_recover = NULL; /* verify_recover */
    out->encrypt = NULL; /* encrypt */
    out->decrypt = NULL; /* decrypt */
    out->derive = NULL;
    out->paramgen = NULL;
    out->ctrl = pkey_rsa_ctrl;
    out->ctrl_str = pkey_rsa_ctrl_str;
}

int EVP_RSA_PKEY_CTX_ctrl(EVP_PKEY_CTX *ctx, int optype, int cmd, int p1, void *p2) {
  /* If key type is not RSA or RSA-PSS return error */
  if ((ctx != NULL) && (ctx->pmeth != NULL)
      && (ctx->pmeth->pkey_id != EVP_PKEY_RSA)
      && (ctx->pmeth->pkey_id != EVP_PKEY_RSA_PSS)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
  return EVP_PKEY_CTX_ctrl(ctx, -1, optype, cmd, p1, p2);
}

int EVP_PKEY_CTX_set_rsa_padding(EVP_PKEY_CTX *ctx, int padding) {
  return EVP_RSA_PKEY_CTX_ctrl(ctx, -1, EVP_PKEY_CTRL_RSA_PADDING, padding, NULL);
}

int EVP_PKEY_CTX_get_rsa_padding(EVP_PKEY_CTX *ctx, int *out_padding) {
  return EVP_RSA_PKEY_CTX_ctrl(ctx, -1, EVP_PKEY_CTRL_GET_RSA_PADDING, 0, out_padding);
}

int EVP_PKEY_CTX_set_rsa_pss_keygen_md(EVP_PKEY_CTX *ctx, const EVP_MD *md) {
  return 0;
}

int EVP_PKEY_CTX_set_rsa_pss_keygen_saltlen(EVP_PKEY_CTX *ctx, int salt_len) {
  return 0;
}

int EVP_PKEY_CTX_set_rsa_pss_keygen_mgf1_md(EVP_PKEY_CTX *ctx,
                                            const EVP_MD *md) {
  return 0;
}

int EVP_PKEY_CTX_set_rsa_pss_saltlen(EVP_PKEY_CTX *ctx, int salt_len) {
  return EVP_RSA_PKEY_CTX_ctrl(ctx,
                           (EVP_PKEY_OP_SIGN | EVP_PKEY_OP_VERIFY),
                           EVP_PKEY_CTRL_RSA_PSS_SALTLEN, salt_len, NULL);
}

int EVP_PKEY_CTX_get_rsa_pss_saltlen(EVP_PKEY_CTX *ctx, int *out_salt_len) {
  return EVP_RSA_PKEY_CTX_ctrl(ctx,
                           (EVP_PKEY_OP_SIGN | EVP_PKEY_OP_VERIFY),
                           EVP_PKEY_CTRL_GET_RSA_PSS_SALTLEN, 0, out_salt_len);
}

int EVP_PKEY_CTX_set_rsa_keygen_bits(EVP_PKEY_CTX *ctx, int bits) {
  return EVP_RSA_PKEY_CTX_ctrl(ctx, EVP_PKEY_OP_KEYGEN,
                           EVP_PKEY_CTRL_RSA_KEYGEN_BITS, bits, NULL);
}

int EVP_PKEY_CTX_set_rsa_keygen_pubexp(EVP_PKEY_CTX *ctx, BIGNUM *e) {
  return EVP_RSA_PKEY_CTX_ctrl(ctx, EVP_PKEY_OP_KEYGEN,
                           EVP_PKEY_CTRL_RSA_KEYGEN_PUBEXP, 0, e);
}

int EVP_PKEY_CTX_set_rsa_oaep_md(EVP_PKEY_CTX *ctx, const EVP_MD *md) {
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_RSA, EVP_PKEY_OP_TYPE_CRYPT,
                           EVP_PKEY_CTRL_RSA_OAEP_MD, 0, (void *)md);
}

int EVP_PKEY_CTX_get_rsa_oaep_md(EVP_PKEY_CTX *ctx, const EVP_MD **out_md) {
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_RSA, EVP_PKEY_OP_TYPE_CRYPT,
                           EVP_PKEY_CTRL_GET_RSA_OAEP_MD, 0, (void*) out_md);
}

int EVP_PKEY_CTX_set_rsa_mgf1_md(EVP_PKEY_CTX *ctx, const EVP_MD *md) {
  return EVP_RSA_PKEY_CTX_ctrl(ctx,
                           EVP_PKEY_OP_TYPE_SIG | EVP_PKEY_OP_TYPE_CRYPT,
                           EVP_PKEY_CTRL_RSA_MGF1_MD, 0, (void*) md);
}

int EVP_PKEY_CTX_get_rsa_mgf1_md(EVP_PKEY_CTX *ctx, const EVP_MD **out_md) {
  return EVP_RSA_PKEY_CTX_ctrl(ctx,
                           EVP_PKEY_OP_TYPE_SIG | EVP_PKEY_OP_TYPE_CRYPT,
                           EVP_PKEY_CTRL_GET_RSA_MGF1_MD, 0, (void*) out_md);
}

int EVP_PKEY_CTX_set0_rsa_oaep_label(EVP_PKEY_CTX *ctx, uint8_t *label,
                                     size_t label_len) {
  RSA_OAEP_LABEL_PARAMS params = {label, label_len};
  return EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_RSA, EVP_PKEY_OP_TYPE_CRYPT,
                           EVP_PKEY_CTRL_RSA_OAEP_LABEL, 0, &params);
}

int EVP_PKEY_CTX_get0_rsa_oaep_label(EVP_PKEY_CTX *ctx,
                                     const uint8_t **out_label) {
  CBS label;
  if (!EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_RSA, EVP_PKEY_OP_TYPE_CRYPT,
                         EVP_PKEY_CTRL_GET_RSA_OAEP_LABEL, 0, &label)) {
    return -1;
  }
  if (CBS_len(&label) > INT_MAX) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_OVERFLOW);
    return -1;
  }
  *out_label = CBS_data(&label);
  return (int)CBS_len(&label);
}
