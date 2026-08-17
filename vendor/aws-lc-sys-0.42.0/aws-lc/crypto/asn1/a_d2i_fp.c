// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <limits.h>

#include <openssl/bio.h>
#include <openssl/err.h>
#include <openssl/mem.h>


void *ASN1_item_d2i_bio(const ASN1_ITEM *it, BIO *in, void *x) {
  uint8_t *data;
  size_t len;
  // Historically, this function did not impose a limit in OpenSSL and is used
  // to read CRLs, so we leave this without an external bound.
  if (!BIO_read_asn1(in, &data, &len, INT_MAX)) {
    return NULL;
  }
  const uint8_t *ptr = data;
  void *ret = ASN1_item_d2i(x, &ptr, len, it);
  OPENSSL_free(data);
  return ret;
}

void *ASN1_item_d2i_fp(const ASN1_ITEM *it, FILE *in, void *x) {
  BIO *b = BIO_new_fp(in, BIO_NOCLOSE);
  if (b == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_BUF_LIB);
    return NULL;
  }
  void *ret = ASN1_item_d2i_bio(it, b, x);
  BIO_free(b);
  return ret;
}
