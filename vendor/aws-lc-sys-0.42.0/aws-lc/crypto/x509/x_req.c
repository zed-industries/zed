// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include <openssl/asn1t.h>
#include <openssl/thread.h>
#include <openssl/x509.h>

#include "internal.h"


// X509_REQ_INFO is handled in an unusual way to get round invalid encodings.
// Some broken certificate requests don't encode the attributes field if it
// is empty. This is in violation of PKCS#10 but we need to tolerate it. We
// do this by making the attributes field OPTIONAL then using the callback to
// initialise it to an empty STACK. This means that the field will be
// correctly encoded unless we NULL out the field.

static int rinf_cb(int operation, ASN1_VALUE **pval, const ASN1_ITEM *it,
                   void *exarg) {
  X509_REQ_INFO *rinf = (X509_REQ_INFO *)*pval;

  if (operation == ASN1_OP_NEW_POST) {
    rinf->attributes = sk_X509_ATTRIBUTE_new_null();
    if (!rinf->attributes) {
      return 0;
    }
  }

  if (operation == ASN1_OP_D2I_POST) {
    // The only defined CSR version is v1(0). For compatibility, we also accept
    // a hypothetical v3(2). Although not defined, older versions of certbot
    // use it. See https://github.com/certbot/certbot/pull/9334.
    long version = ASN1_INTEGER_get(rinf->version);
    if (version != X509_REQ_VERSION_1 && version != 2) {
      OPENSSL_PUT_ERROR(X509, X509_R_INVALID_VERSION);
      return 0;
    }
  }

  return 1;
}

ASN1_SEQUENCE_enc(X509_REQ_INFO, enc, rinf_cb) = {
    ASN1_SIMPLE(X509_REQ_INFO, version, ASN1_INTEGER),
    ASN1_SIMPLE(X509_REQ_INFO, subject, X509_NAME),
    ASN1_SIMPLE(X509_REQ_INFO, pubkey, X509_PUBKEY),
    // This isn't really OPTIONAL but it gets around invalid encodings.
    ASN1_IMP_SET_OF_OPT(X509_REQ_INFO, attributes, X509_ATTRIBUTE, 0),
} ASN1_SEQUENCE_END_enc(X509_REQ_INFO, X509_REQ_INFO)

IMPLEMENT_ASN1_FUNCTIONS(X509_REQ_INFO)

ASN1_SEQUENCE(X509_REQ) = {
    ASN1_SIMPLE(X509_REQ, req_info, X509_REQ_INFO),
    ASN1_SIMPLE(X509_REQ, sig_alg, X509_ALGOR),
    ASN1_SIMPLE(X509_REQ, signature, ASN1_BIT_STRING),
} ASN1_SEQUENCE_END(X509_REQ)

IMPLEMENT_ASN1_FUNCTIONS(X509_REQ)

IMPLEMENT_ASN1_DUP_FUNCTION(X509_REQ)
