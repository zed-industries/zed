// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/evp.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"


int X509_REQ_set_version(X509_REQ *x, long version) {
  if (x == NULL) {
    return 0;
  }
  if (version != X509_REQ_VERSION_1) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_VERSION);
    return 0;
  }
  return ASN1_INTEGER_set_int64(x->req_info->version, version);
}

int X509_REQ_set_subject_name(X509_REQ *x, X509_NAME *name) {
  if ((x == NULL) || (x->req_info == NULL)) {
    return 0;
  }
  return (X509_NAME_set(&x->req_info->subject, name));
}

int X509_REQ_set_pubkey(X509_REQ *x, EVP_PKEY *pkey) {
  if ((x == NULL) || (x->req_info == NULL)) {
    return 0;
  }
  return (X509_PUBKEY_set(&x->req_info->pubkey, pkey));
}

int X509_REQ_set1_signature_algo(X509_REQ *req, const X509_ALGOR *algo) {
  X509_ALGOR *copy = X509_ALGOR_dup(algo);
  if (copy == NULL) {
    return 0;
  }

  X509_ALGOR_free(req->sig_alg);
  req->sig_alg = copy;
  return 1;
}

int X509_REQ_set1_signature_value(X509_REQ *req, const uint8_t *sig,
                                  size_t sig_len) {
  if (!ASN1_STRING_set(req->signature, sig, sig_len)) {
    return 0;
  }
  req->signature->flags &= ~(ASN1_STRING_FLAG_BITS_LEFT | 0x07);
  req->signature->flags |= ASN1_STRING_FLAG_BITS_LEFT;
  return 1;
}
