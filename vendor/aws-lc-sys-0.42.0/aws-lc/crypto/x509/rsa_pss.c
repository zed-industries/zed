// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2006.
// Copyright (c) 2006 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/x509.h>

#include <assert.h>

#include <openssl/asn1.h>
#include <openssl/asn1t.h>
#include <openssl/bio.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/obj.h>

#include "internal.h"


static int rsa_pss_cb(int operation, ASN1_VALUE **pval, const ASN1_ITEM *it,
                      void *exarg) {
  if (operation == ASN1_OP_FREE_PRE) {
    RSA_PSS_PARAMS *pss = (RSA_PSS_PARAMS *)*pval;
    X509_ALGOR_free(pss->maskHash);
  }
  return 1;
}

ASN1_SEQUENCE_cb(RSA_PSS_PARAMS, rsa_pss_cb) = {
    ASN1_EXP_OPT(RSA_PSS_PARAMS, hashAlgorithm, X509_ALGOR, 0),
    ASN1_EXP_OPT(RSA_PSS_PARAMS, maskGenAlgorithm, X509_ALGOR, 1),
    ASN1_EXP_OPT(RSA_PSS_PARAMS, saltLength, ASN1_INTEGER, 2),
    ASN1_EXP_OPT(RSA_PSS_PARAMS, trailerField, ASN1_INTEGER, 3),
} ASN1_SEQUENCE_END_cb(RSA_PSS_PARAMS, RSA_PSS_PARAMS)

IMPLEMENT_ASN1_FUNCTIONS_const(RSA_PSS_PARAMS)


// Given an MGF1 Algorithm ID decode to an Algorithm Identifier
static X509_ALGOR *rsa_mgf1_decode(X509_ALGOR *alg) {
  if (alg == NULL || alg->parameter == NULL ||
      OBJ_obj2nid(alg->algorithm) != NID_mgf1 ||
      alg->parameter->type != V_ASN1_SEQUENCE) {
    return NULL;
  }

  const uint8_t *p = alg->parameter->value.sequence->data;
  int plen = alg->parameter->value.sequence->length;
  return d2i_X509_ALGOR(NULL, &p, plen);
}

static RSA_PSS_PARAMS *rsa_pss_decode(const X509_ALGOR *alg,
                                      X509_ALGOR **pmaskHash) {
  *pmaskHash = NULL;

  if (alg->parameter == NULL || alg->parameter->type != V_ASN1_SEQUENCE) {
    return NULL;
  }

  const uint8_t *p = alg->parameter->value.sequence->data;
  int plen = alg->parameter->value.sequence->length;
  RSA_PSS_PARAMS *pss = d2i_RSA_PSS_PARAMS(NULL, &p, plen);
  if (pss == NULL) {
    return NULL;
  }

  *pmaskHash = rsa_mgf1_decode(pss->maskGenAlgorithm);
  return pss;
}

// allocate and set algorithm ID from EVP_MD, default SHA1
static int rsa_md_to_algor(X509_ALGOR **palg, const EVP_MD *md) {
  if (EVP_MD_type(md) == NID_sha1) {
    return 1;
  }
  *palg = X509_ALGOR_new();
  if (*palg == NULL) {
    return 0;
  }
  if (!X509_ALGOR_set_md(*palg, md)) {
    X509_ALGOR_free(*palg);
    *palg = NULL;
    return 0;
  }
  return 1;
}

// Allocate and set MGF1 algorithm ID from EVP_MD
static int rsa_md_to_mgf1(X509_ALGOR **palg, const EVP_MD *mgf1md) {
  X509_ALGOR *algtmp = NULL;
  ASN1_STRING *stmp = NULL;
  *palg = NULL;

  if (EVP_MD_type(mgf1md) == NID_sha1) {
    return 1;
  }
  // need to embed algorithm ID inside another
  if (!rsa_md_to_algor(&algtmp, mgf1md) ||
      !ASN1_item_pack(algtmp, ASN1_ITEM_rptr(X509_ALGOR), &stmp)) {
    goto err;
  }
  *palg = X509_ALGOR_new();
  if (!*palg) {
    goto err;
  }
  if (!X509_ALGOR_set0(*palg, OBJ_nid2obj(NID_mgf1), V_ASN1_SEQUENCE, stmp)) {
    goto err;
  }
  stmp = NULL;

err:
  ASN1_STRING_free(stmp);
  X509_ALGOR_free(algtmp);
  if (*palg) {
    return 1;
  }

  return 0;
}

// convert algorithm ID to EVP_MD, default SHA1
static const EVP_MD *rsa_algor_to_md(X509_ALGOR *alg) {
  const EVP_MD *md;
  if (!alg) {
    return EVP_sha1();
  }
  md = EVP_get_digestbyobj(alg->algorithm);
  if (md == NULL) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_PSS_PARAMETERS);
  }
  return md;
}

// convert MGF1 algorithm ID to EVP_MD, default SHA1
static const EVP_MD *rsa_mgf1_to_md(const X509_ALGOR *alg,
                                    X509_ALGOR *maskHash) {
  const EVP_MD *md;
  if (!alg) {
    return EVP_sha1();
  }
  // Check mask and lookup mask hash algorithm
  if (OBJ_obj2nid(alg->algorithm) != NID_mgf1 || maskHash == NULL) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_PSS_PARAMETERS);
    return NULL;
  }
  md = EVP_get_digestbyobj(maskHash->algorithm);
  if (md == NULL) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_PSS_PARAMETERS);
    return NULL;
  }
  return md;
}

int x509_rsa_ctx_to_pss(EVP_MD_CTX *ctx, X509_ALGOR *algor) {
  const EVP_MD *sigmd, *mgf1md;
  int saltlen;
  if (!EVP_PKEY_CTX_get_signature_md(ctx->pctx, &sigmd) ||
      !EVP_PKEY_CTX_get_rsa_mgf1_md(ctx->pctx, &mgf1md) ||
      !EVP_PKEY_CTX_get_rsa_pss_saltlen(ctx->pctx, &saltlen)) {
    return 0;
  }

  EVP_PKEY *pk = EVP_PKEY_CTX_get0_pkey(ctx->pctx);
  if (saltlen == -1) {
    saltlen = EVP_MD_size(sigmd);
  } else if (saltlen == -2) {
    // TODO(davidben): Forbid this mode. The world has largely standardized on
    // salt length matching hash length.
    saltlen = EVP_PKEY_size(pk) - EVP_MD_size(sigmd) - 2;
    if (((EVP_PKEY_bits(pk) - 1) & 0x7) == 0) {
      saltlen--;
    }
  } else if (saltlen != (int)EVP_MD_size(sigmd)) {
    // We only allow salt length matching hash length and, for now, the -2 case.
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_PSS_PARAMETERS);
    return 0;
  }

  int ret = 0;
  ASN1_STRING *os = NULL;
  RSA_PSS_PARAMS *pss = RSA_PSS_PARAMS_new();
  if (!pss) {
    goto err;
  }

  if (saltlen != 20) {
    pss->saltLength = ASN1_INTEGER_new();
    if (!pss->saltLength || !ASN1_INTEGER_set_int64(pss->saltLength, saltlen)) {
      goto err;
    }
  }

  if (!rsa_md_to_algor(&pss->hashAlgorithm, sigmd) ||
      !rsa_md_to_mgf1(&pss->maskGenAlgorithm, mgf1md)) {
    goto err;
  }

  // Finally create string with pss parameter encoding.
  if (!ASN1_item_pack(pss, ASN1_ITEM_rptr(RSA_PSS_PARAMS), &os)) {
    goto err;
  }

  if (!X509_ALGOR_set0(algor, OBJ_nid2obj(NID_rsassaPss), V_ASN1_SEQUENCE, os)) {
    goto err;
  }
  os = NULL;
  ret = 1;

err:
  RSA_PSS_PARAMS_free(pss);
  ASN1_STRING_free(os);
  return ret;
}

int x509_rsa_pss_to_ctx(EVP_MD_CTX *ctx, const X509_ALGOR *sigalg,
                        EVP_PKEY *pkey) {
  assert(OBJ_obj2nid(sigalg->algorithm) == NID_rsassaPss);

  // Decode PSS parameters
  int ret = 0;
  X509_ALGOR *maskHash;
  RSA_PSS_PARAMS *pss = rsa_pss_decode(sigalg, &maskHash);
  if (pss == NULL) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_PSS_PARAMETERS);
    goto err;
  }

  const EVP_MD *mgf1md = rsa_mgf1_to_md(pss->maskGenAlgorithm, maskHash);
  const EVP_MD *md = rsa_algor_to_md(pss->hashAlgorithm);
  if (mgf1md == NULL || md == NULL) {
    goto err;
  }

  // Check that both signature algorithms are not weak
  if(!x509_digest_nid_ok(EVP_MD_type(md)) || !x509_digest_nid_ok(EVP_MD_type(mgf1md))) {
    OPENSSL_PUT_ERROR(X509, ASN1_R_DIGEST_AND_KEY_TYPE_NOT_SUPPORTED);
    goto err;
  }

  int saltlen = 20;
  if (pss->saltLength != NULL) {
    saltlen = ASN1_INTEGER_get(pss->saltLength);

    // Could perform more salt length sanity checks but the main
    // RSA routines will trap other invalid values anyway.
    if (saltlen < 0) {
      OPENSSL_PUT_ERROR(X509, X509_R_INVALID_PSS_PARAMETERS);
      goto err;
    }
  }

  // low-level routines support only trailer field 0xbc (value 1)
  // and PKCS#1 says we should reject any other value anyway.
  if (pss->trailerField != NULL && ASN1_INTEGER_get(pss->trailerField) != 1) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_PSS_PARAMETERS);
    goto err;
  }

  EVP_PKEY_CTX *pctx;
  if (!EVP_DigestVerifyInit(ctx, &pctx, md, NULL, pkey) ||
      !EVP_PKEY_CTX_set_rsa_padding(pctx, RSA_PKCS1_PSS_PADDING) ||
      !EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx, saltlen) ||
      !EVP_PKEY_CTX_set_rsa_mgf1_md(pctx, mgf1md)) {
    goto err;
  }

  ret = 1;

err:
  RSA_PSS_PARAMS_free(pss);
  X509_ALGOR_free(maskHash);
  return ret;
}

int x509_print_rsa_pss_params(BIO *bp, const X509_ALGOR *sigalg, int indent,
                              ASN1_PCTX *pctx) {
  assert(OBJ_obj2nid(sigalg->algorithm) == NID_rsassaPss);

  int rv = 0;
  X509_ALGOR *maskHash;
  RSA_PSS_PARAMS *pss = rsa_pss_decode(sigalg, &maskHash);
  if (!pss) {
    if (BIO_puts(bp, " (INVALID PSS PARAMETERS)\n") <= 0) {
      goto err;
    }
    rv = 1;
    goto err;
  }

  if (BIO_puts(bp, "\n") <= 0 ||       //
      !BIO_indent(bp, indent, 128) ||  //
      BIO_puts(bp, "Hash Algorithm: ") <= 0) {
    goto err;
  }

  if (pss->hashAlgorithm) {
    if (i2a_ASN1_OBJECT(bp, pss->hashAlgorithm->algorithm) <= 0) {
      goto err;
    }
  } else if (BIO_puts(bp, "sha1 (default)") <= 0) {
    goto err;
  }

  if (BIO_puts(bp, "\n") <= 0 ||       //
      !BIO_indent(bp, indent, 128) ||  //
      BIO_puts(bp, "Mask Algorithm: ") <= 0) {
    goto err;
  }

  if (pss->maskGenAlgorithm) {
    if (i2a_ASN1_OBJECT(bp, pss->maskGenAlgorithm->algorithm) <= 0 ||
        BIO_puts(bp, " with ") <= 0) {
      goto err;
    }

    if (maskHash) {
      if (i2a_ASN1_OBJECT(bp, maskHash->algorithm) <= 0) {
        goto err;
      }
    } else if (BIO_puts(bp, "INVALID") <= 0) {
      goto err;
    }
  } else if (BIO_puts(bp, "mgf1 with sha1 (default)") <= 0) {
    goto err;
  }
  BIO_puts(bp, "\n");

  if (!BIO_indent(bp, indent, 128) ||  //
      BIO_puts(bp, "Salt Length: 0x") <= 0) {
    goto err;
  }

  if (pss->saltLength) {
    if (i2a_ASN1_INTEGER(bp, pss->saltLength) <= 0) {
      goto err;
    }
  } else if (BIO_puts(bp, "14 (default)") <= 0) {
    goto err;
  }
  BIO_puts(bp, "\n");

  if (!BIO_indent(bp, indent, 128) ||  //
      BIO_puts(bp, "Trailer Field: 0x") <= 0) {
    goto err;
  }

  if (pss->trailerField) {
    if (i2a_ASN1_INTEGER(bp, pss->trailerField) <= 0) {
      goto err;
    }
  } else if (BIO_puts(bp, "BC (default)") <= 0) {
    goto err;
  }
  BIO_puts(bp, "\n");

  rv = 1;

err:
  RSA_PSS_PARAMS_free(pss);
  X509_ALGOR_free(maskHash);
  return rv;
}
