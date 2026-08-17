// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/dsa.h>
#include <openssl/err.h>
#include <openssl/evp.h>

#include "../dsa/internal.h"
#include "../internal.h"
#include "./internal.h"

typedef struct {
  int nbits;  // defaults to 2048
  int qbits;
  const EVP_MD *pmd;  // MD for paramgen
  const EVP_MD *md;   // MD for signing
} DSA_PKEY_CTX;

static int pkey_dsa_init(EVP_PKEY_CTX *ctx) {
  DSA_PKEY_CTX *dctx = NULL;
  if (!(dctx = (DSA_PKEY_CTX *)OPENSSL_zalloc(sizeof(DSA_PKEY_CTX)))) {
    return 0;
  }
  dctx->nbits = 2048;
  dctx->qbits = 256;
  dctx->pmd = NULL;
  dctx->md = NULL;

  ctx->data = dctx;

  return 1;
}

static int pkey_dsa_copy(EVP_PKEY_CTX *dst, EVP_PKEY_CTX *src) {
  DSA_PKEY_CTX *dctx = NULL;
  DSA_PKEY_CTX *sctx = NULL;
  if (!pkey_dsa_init(dst)) {
    return 0;
  }
  sctx = (DSA_PKEY_CTX *)src->data;
  dctx = (DSA_PKEY_CTX *)dst->data;
  if (sctx == NULL || dctx == NULL) {
    return 0;
  }

  dctx->nbits = sctx->nbits;
  dctx->qbits = sctx->qbits;
  dctx->pmd = sctx->pmd;
  dctx->md = sctx->md;
  return 1;
}

static void pkey_dsa_cleanup(EVP_PKEY_CTX *ctx) {
  OPENSSL_free(ctx->data);
  ctx->data = NULL;
}

static int pkey_dsa_keygen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  GUARD_PTR(ctx->pkey);

  DSA *dsa = DSA_new();
  if (dsa == NULL || !EVP_PKEY_assign_DSA(pkey, dsa)) {
    DSA_free(dsa);
    return 0;
  }

  // |pkey| now has ownership of DSA, and EVP_PKEY_free will handle from this
  // point on.

  if (!EVP_PKEY_copy_parameters(pkey, ctx->pkey)) {
    return 0;
  }

  return DSA_generate_key(pkey->pkey.dsa);
}

static int pkey_dsa_paramgen(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey) {
  BN_GENCB *pkey_ctx_cb = NULL;
  DSA *dsa = NULL;
  DSA_PKEY_CTX *dctx = (DSA_PKEY_CTX *)ctx->data;
  GUARD_PTR(dctx);

  int ret = 0;

  if (ctx->pkey_gencb) {
    pkey_ctx_cb = BN_GENCB_new();
    if (pkey_ctx_cb == NULL) {
      goto end;
    }
    evp_pkey_set_cb_translate(pkey_ctx_cb, ctx);
  }

  const EVP_MD *pmd = dctx->pmd;
  if (pmd == NULL) {
    switch (dctx->qbits) {
      case 160:
        pmd = EVP_sha1();
        break;
      case 224:
        pmd = EVP_sha224();
        break;
      case 256:
        pmd = EVP_sha256();
        break;
      default:
        // This should not be possible.
        OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
        goto end;
    }
  }

  if (!((dsa = DSA_new()))) {
    goto end;
  }

  if (!dsa_internal_paramgen(dsa, dctx->nbits, pmd, NULL, 0, NULL, NULL,
                             pkey_ctx_cb)) {
    goto end;
  }

  ret = EVP_PKEY_assign_DSA(pkey, dsa);
end:
  BN_GENCB_free(pkey_ctx_cb);
  if (ret != 1) {
    OPENSSL_free(dsa);
  }

  return ret;
}

static int pkey_dsa_sign(EVP_PKEY_CTX *ctx, unsigned char *sig, size_t *siglen,
                         const unsigned char *tbs, size_t tbslen) {
  GUARD_PTR(ctx->pkey->pkey.ptr);
  GUARD_PTR(ctx->data);

  DSA_PKEY_CTX *dctx = ctx->data;
  DSA *dsa = ctx->pkey->pkey.dsa;

  if (sig == NULL) {
    // Passing NULL for sig indicates a query for the size of the signature
    *siglen = DSA_size(dsa);
    return 1;
  }

  if (*siglen < (size_t)DSA_size(dsa)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  DSA_SIG *result = NULL;
  uint8_t *sig_buffer = NULL;
  int retval = 0;

  if (dctx->md != NULL && tbslen != EVP_MD_size(dctx->md)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
    goto end;
  }

  result = DSA_do_sign(tbs, tbslen, dsa);
  if (result == NULL) {
    goto end;
  }
  CBB sig_bytes;
  if (1 != CBB_init(&sig_bytes, tbslen)) {
    goto end;
  }
  DSA_SIG_marshal(&sig_bytes, result);
  if (1 != CBB_finish(&sig_bytes, &sig_buffer, siglen)) {
    goto end;
  }
  OPENSSL_memcpy(sig, sig_buffer, *siglen);
  retval = 1;
end:
  OPENSSL_free(sig_buffer);
  DSA_SIG_free(result);
  return retval;
}

static int pkey_dsa_verify(EVP_PKEY_CTX *ctx, const unsigned char *sig,
                           size_t siglen, const unsigned char *tbs,
                           size_t tbslen) {
  GUARD_PTR(ctx->pkey);
  GUARD_PTR(ctx->pkey->pkey.ptr);
  GUARD_PTR(ctx->data);
  GUARD_PTR(tbs);

  DSA_PKEY_CTX *dctx = ctx->data;
  const DSA *dsa = ctx->pkey->pkey.dsa;

  if (dctx->md != NULL && tbslen != EVP_MD_size(dctx->md)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_PARAMETERS);
    return 0;
  }

  DSA_SIG *dsa_sig = NULL;
  int retval = 0;

  CBS sig_cbs;
  CBS_init(&sig_cbs, sig, siglen);
  dsa_sig = DSA_SIG_parse(&sig_cbs);
  // Allocation failure, invalid asn1 encoding, or trailing garbage
  if (dsa_sig == NULL || CBS_len(&sig_cbs) != 0) {
    goto end;
  }
  if (1 != DSA_do_verify(tbs, tbslen, dsa_sig, dsa)) {
    goto end;
  }
  retval = 1;
end:
  DSA_SIG_free(dsa_sig);
  return retval;
}

static int pkey_dsa_ctrl(EVP_PKEY_CTX *ctx, int type, int p1, void *p2) {
  DSA_PKEY_CTX *dctx = ctx->data;

  switch (type) {
    case EVP_PKEY_CTRL_DSA_PARAMGEN_BITS:
      if (p1 < 512) {
        return -2;
      }
      dctx->nbits = p1;
      return 1;

    case EVP_PKEY_CTRL_DSA_PARAMGEN_Q_BITS: {
      switch (p1) {
        case 160:
        case 224:
        case 256:
          dctx->qbits = p1;
          return 1;
        default:
          return -2;
      }
    }

    case EVP_PKEY_CTRL_DSA_PARAMGEN_MD: {
      const EVP_MD *pmd = (const EVP_MD *)p2;
      if (pmd == NULL) {
        return 0;
      }
      switch (EVP_MD_type(pmd)) {
        case NID_sha1:
        case NID_sha224:
        case NID_sha256:
          dctx->pmd = pmd;
          return 1;
        default:
          OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_DIGEST_TYPE);
          return 0;
      }
    }
    case EVP_PKEY_CTRL_MD: {
      const EVP_MD *md = (const EVP_MD *)p2;
      if (md == NULL) {
        return 0;
      }
      switch (EVP_MD_type(md)) {
        case NID_sha1:
        case NID_sha224:
        case NID_sha256:
        case NID_sha384:
        case NID_sha512:
        case NID_sha3_224:
        case NID_sha3_256:
        case NID_sha3_384:
        case NID_sha3_512:
          dctx->md = md;
          return 1;
        default:
          OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_DIGEST_TYPE);
          return 0;
      }
    }
    case EVP_PKEY_CTRL_GET_MD:
      if (p2 == NULL) {
        return 0;
      }
      *(const EVP_MD **)p2 = dctx->md;
      return 1;

    case EVP_PKEY_CTRL_PEER_KEY:
      OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
      return -2;
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_COMMAND_NOT_SUPPORTED);
      return 0;
  }
}

static int pkey_dsa_ctrl_str(EVP_PKEY_CTX *ctx, const char *type,
                             const char *value) {
  if (strcmp(type, "dsa_paramgen_bits") == 0) {
    char *str_end = NULL;
    long nbits = strtol(value, &str_end, 10);
    if (str_end == value || nbits < 0 || nbits > INT_MAX) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
      return 0;
    }
OPENSSL_BEGIN_ALLOW_DEPRECATED
    return EVP_PKEY_CTX_set_dsa_paramgen_bits(ctx, (int)nbits);
OPENSSL_END_ALLOW_DEPRECATED
  }
  if (strcmp(type, "dsa_paramgen_q_bits") == 0) {
    char *str_end = NULL;
    long qbits = strtol(value, &str_end, 10);
    if (str_end == value || qbits < 0 || qbits > INT_MAX) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_OPERATION);
      return 0;
    }
OPENSSL_BEGIN_ALLOW_DEPRECATED
    return EVP_PKEY_CTX_set_dsa_paramgen_q_bits(ctx, (int)qbits);
OPENSSL_END_ALLOW_DEPRECATED
  }
  if (strcmp(type, "dsa_paramgen_md") == 0) {
    const EVP_MD *md = EVP_get_digestbyname(value);

    if (md == NULL) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_INVALID_DIGEST_TYPE);
      return 0;
    }
OPENSSL_BEGIN_ALLOW_DEPRECATED
    return EVP_PKEY_CTX_set_dsa_paramgen_md(ctx, md);
OPENSSL_END_ALLOW_DEPRECATED
  }
  return -2;
}

const EVP_PKEY_METHOD dsa_pkey_meth = {.pkey_id = EVP_PKEY_DSA,
                                       .init = pkey_dsa_init,
                                       .copy = pkey_dsa_copy,
                                       .cleanup = pkey_dsa_cleanup,
                                       .keygen = pkey_dsa_keygen,
                                       .paramgen = pkey_dsa_paramgen,
                                       .sign = pkey_dsa_sign,
                                       .verify = pkey_dsa_verify,
                                       .ctrl = pkey_dsa_ctrl,
                                       .ctrl_str = pkey_dsa_ctrl_str};


int EVP_PKEY_CTX_set_dsa_paramgen_bits(EVP_PKEY_CTX *ctx, int nbits) {
  if (1 == EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_DSA, EVP_PKEY_OP_PARAMGEN,
                             EVP_PKEY_CTRL_DSA_PARAMGEN_BITS, nbits, NULL)) {
    return 1;
  }
  return 0;
}

int EVP_PKEY_CTX_set_dsa_paramgen_q_bits(EVP_PKEY_CTX *ctx, int qbits) {
  if (1 == EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_DSA, EVP_PKEY_OP_PARAMGEN,
                             EVP_PKEY_CTRL_DSA_PARAMGEN_Q_BITS, qbits, NULL)) {
    return 1;
  }
  return 0;
}

int EVP_PKEY_CTX_set_dsa_paramgen_md(EVP_PKEY_CTX *ctx, const EVP_MD *md) {
  if (1 == EVP_PKEY_CTX_ctrl(ctx, EVP_PKEY_DSA, EVP_PKEY_OP_PARAMGEN,
                             EVP_PKEY_CTRL_DSA_PARAMGEN_MD, 0, (void *)md)) {
    return 1;
  }
  return 0;
}
