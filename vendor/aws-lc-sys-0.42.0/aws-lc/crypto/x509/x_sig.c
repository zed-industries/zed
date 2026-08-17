// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include <openssl/asn1t.h>
#include <openssl/x509.h>


struct X509_sig_st {
  X509_ALGOR *algor;
  ASN1_OCTET_STRING *digest;
} /* X509_SIG */;

ASN1_SEQUENCE(X509_SIG) = {
    ASN1_SIMPLE(X509_SIG, algor, X509_ALGOR),
    ASN1_SIMPLE(X509_SIG, digest, ASN1_OCTET_STRING),
} ASN1_SEQUENCE_END(X509_SIG)

IMPLEMENT_ASN1_FUNCTIONS_const(X509_SIG)

void X509_SIG_get0(const X509_SIG *sig, const X509_ALGOR **out_alg,
                   const ASN1_OCTET_STRING **out_digest) {
  if (out_alg != NULL) {
    *out_alg = sig->algor;
  }
  if (out_digest != NULL) {
    *out_digest = sig->digest;
  }
}

void X509_SIG_getm(X509_SIG *sig, X509_ALGOR **out_alg,
                   ASN1_OCTET_STRING **out_digest) {
  if (out_alg != NULL) {
    *out_alg = sig->algor;
  }
  if (out_digest != NULL) {
    *out_digest = sig->digest;
  }
}
