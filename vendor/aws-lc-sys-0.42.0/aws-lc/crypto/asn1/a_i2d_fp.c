// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>

#include <openssl/asn1.h>

#include <openssl/bio.h>
#include <openssl/err.h>
#include <openssl/mem.h>


int ASN1_i2d_bio(i2d_of_void *i2d, BIO *out, void *in) {
  if (i2d == NULL || out == NULL || in == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  int size = i2d(in, NULL);
  if (size <= 0) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  unsigned char *buffer = (unsigned char *)OPENSSL_malloc(size);
  if (buffer == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  unsigned char *outp = buffer;
  int ret = i2d(in, &outp);
  if (ret < 0 || ret > size) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_BUFFER_TOO_SMALL);
    OPENSSL_free(buffer);
    return 0;
  }

  ret = BIO_write_all(out, buffer, size);
  OPENSSL_free(buffer);
  return ret;
}

int ASN1_item_i2d_fp(const ASN1_ITEM *it, FILE *out, void *x) {
  BIO *b = BIO_new_fp(out, BIO_NOCLOSE);
  if (b == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_BUF_LIB);
    return 0;
  }
  int ret = ASN1_item_i2d_bio(it, b, x);
  BIO_free(b);
  return ret;
}

int ASN1_item_i2d_bio(const ASN1_ITEM *it, BIO *out, void *x) {
  unsigned char *b = NULL;
  int n = ASN1_item_i2d(x, &b, it);
  if (b == NULL) {
    return 0;
  }

  int ret = BIO_write_all(out, b, n);
  OPENSSL_free(b);
  return ret;
}
