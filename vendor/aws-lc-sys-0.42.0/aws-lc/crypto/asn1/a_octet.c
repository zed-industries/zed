// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>

#include <openssl/err.h>
#include <openssl/mem.h>

ASN1_OCTET_STRING *ASN1_OCTET_STRING_dup(const ASN1_OCTET_STRING *x) {
  return ASN1_STRING_dup(x);
}

int ASN1_OCTET_STRING_cmp(const ASN1_OCTET_STRING *a,
                          const ASN1_OCTET_STRING *b) {
  return ASN1_STRING_cmp(a, b);
}

int ASN1_OCTET_STRING_set(ASN1_OCTET_STRING *x, const unsigned char *d,
                          int len) {
  return ASN1_STRING_set(x, d, len);
}
