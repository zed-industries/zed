// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/x509.h>

#include <stdio.h>
#include <sys/types.h>

#include <openssl/bn.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/obj.h>

#include "internal.h"

int ASN1_item_verify(const ASN1_ITEM *it, const X509_ALGOR *a,
                     const ASN1_BIT_STRING *signature, void *asn,
                     EVP_PKEY *pkey) {
  if (!pkey) {
    OPENSSL_PUT_ERROR(X509, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  size_t sig_len;
  if (signature->type == V_ASN1_BIT_STRING) {
    if (!ASN1_BIT_STRING_num_bytes(signature, &sig_len)) {
      OPENSSL_PUT_ERROR(X509, X509_R_INVALID_BIT_STRING_BITS_LEFT);
      return 0;
    }
  } else {
    sig_len = (size_t)ASN1_STRING_length(signature);
  }

  EVP_MD_CTX ctx;
  uint8_t *buf_in = NULL;
  int ret = 0, inl = 0;
  EVP_MD_CTX_init(&ctx);

  if (!x509_digest_verify_init(&ctx, a, pkey)) {
    goto err;
  }

  inl = ASN1_item_i2d(asn, &buf_in, it);

  if (buf_in == NULL) {
    goto err;
  }

  if (!EVP_DigestVerify(&ctx, ASN1_STRING_get0_data(signature), sig_len, buf_in,
                        inl)) {
    OPENSSL_PUT_ERROR(X509, ERR_R_EVP_LIB);
    goto err;
  }

  ret = 1;

err:
  OPENSSL_free(buf_in);
  EVP_MD_CTX_cleanup(&ctx);
  return ret;
}
