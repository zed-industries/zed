// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <openssl/err.h>
#include <openssl/mem.h>

void *ASN1_dup(i2d_of_void *i2d, d2i_of_void *d2i, void *input) {
  if (i2d == NULL || d2i == NULL || input == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  // Size and allocate |buf|.
  unsigned char *buf = NULL;
  int buf_len = i2d(input, &buf);
  if (buf == NULL || buf_len < 0) {
    return NULL;
  }

  // |buf| needs to be converted to |const| to be passed in.
  const unsigned char *temp_input = buf;
  char *ret = d2i(NULL, &temp_input, buf_len);
  OPENSSL_free(buf);
  return ret;
}

// ASN1_ITEM version of dup: this follows the model above except we don't
// need to allocate the buffer. At some point this could be rewritten to
// directly dup the underlying structure instead of doing and encode and
// decode.
void *ASN1_item_dup(const ASN1_ITEM *it, void *x) {
  unsigned char *b = NULL;
  const unsigned char *p;
  long i;
  void *ret;

  if (x == NULL) {
    return NULL;
  }

  i = ASN1_item_i2d(x, &b, it);
  if (b == NULL) {
    return NULL;
  }
  p = b;
  ret = ASN1_item_d2i(NULL, &p, i, it);
  OPENSSL_free(b);
  return ret;
}
