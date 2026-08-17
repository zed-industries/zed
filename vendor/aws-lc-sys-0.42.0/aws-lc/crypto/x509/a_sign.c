// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include <limits.h>

#include "internal.h"

int ASN1_item_sign(const ASN1_ITEM *it, X509_ALGOR *algor1, X509_ALGOR *algor2,
                   ASN1_BIT_STRING *signature, void *asn, EVP_PKEY *pkey,
                   const EVP_MD *type) {
  if (signature->type != V_ASN1_BIT_STRING) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_WRONG_TYPE);
    return 0;
  }
  EVP_MD_CTX ctx;
  EVP_MD_CTX_init(&ctx);
  if (!EVP_DigestSignInit(&ctx, NULL, type, NULL, pkey)) {
    EVP_MD_CTX_cleanup(&ctx);
    return 0;
  }
  return ASN1_item_sign_ctx(it, algor1, algor2, signature, asn, &ctx);
}

int ASN1_item_sign_ctx(const ASN1_ITEM *it, X509_ALGOR *algor1,
                       X509_ALGOR *algor2, ASN1_BIT_STRING *signature,
                       void *asn, EVP_MD_CTX *ctx) {
  int ret = 0;
  uint8_t *in = NULL, *out = NULL;
  if (signature->type != V_ASN1_BIT_STRING) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_WRONG_TYPE);
    goto err;
  }

  // Write out the requested copies of the AlgorithmIdentifier.
  if (algor1 && !x509_digest_sign_algorithm(ctx, algor1)) {
    goto err;
  }
  if (algor2 && !x509_digest_sign_algorithm(ctx, algor2)) {
    goto err;
  }

  int in_len = ASN1_item_i2d(asn, &in, it);
  if (in_len < 0) {
    goto err;
  }

  EVP_PKEY *pkey = EVP_PKEY_CTX_get0_pkey(ctx->pctx);
  size_t out_len = EVP_PKEY_size(pkey);
  if (out_len > INT_MAX) {
    OPENSSL_PUT_ERROR(X509, ERR_R_OVERFLOW);
    goto err;
  }

  out = OPENSSL_malloc(out_len);
  if (out == NULL) {
    goto err;
  }

  if (!EVP_DigestSign(ctx, out, &out_len, in, in_len)) {
    OPENSSL_PUT_ERROR(X509, ERR_R_EVP_LIB);
    goto err;
  }

  ASN1_STRING_set0(signature, out, (int)out_len);
  out = NULL;
  signature->flags &= ~(ASN1_STRING_FLAG_BITS_LEFT | 0x07);
  signature->flags |= ASN1_STRING_FLAG_BITS_LEFT;
  ret = (int)out_len;

err:
  EVP_MD_CTX_cleanup(ctx);
  OPENSSL_free(in);
  OPENSSL_free(out);
  return ret;
}
