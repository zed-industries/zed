// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/pem.h>

#include <stdio.h>

#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/rand.h>
#include <openssl/x509.h>

// Handle 'other' PEMs: not private keys

void *PEM_ASN1_read_bio(d2i_of_void *d2i, const char *name, BIO *bp, void **x,
                        pem_password_cb *cb, void *u) {
  const unsigned char *p = NULL;
  unsigned char *data = NULL;
  long len;
  char *ret = NULL;

  if (!PEM_bytes_read_bio(&data, &len, NULL, name, bp, cb, u)) {
    return NULL;
  }
  p = data;
  ret = d2i(x, &p, len);
  if (ret == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_ASN1_LIB);
  }
  OPENSSL_free(data);
  return ret;
}
