// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/x509.h>
#include <openssl/x509v3.h>

#include "internal.h"


typedef BIT_STRING_BITNAME ENUMERATED_NAMES;

static const ENUMERATED_NAMES crl_reasons[] = {
    {CRL_REASON_UNSPECIFIED, "Unspecified", "unspecified"},
    {CRL_REASON_KEY_COMPROMISE, "Key Compromise", "keyCompromise"},
    {CRL_REASON_CA_COMPROMISE, "CA Compromise", "CACompromise"},
    {CRL_REASON_AFFILIATION_CHANGED, "Affiliation Changed",
     "affiliationChanged"},
    {CRL_REASON_SUPERSEDED, "Superseded", "superseded"},
    {CRL_REASON_CESSATION_OF_OPERATION, "Cessation Of Operation",
     "cessationOfOperation"},
    {CRL_REASON_CERTIFICATE_HOLD, "Certificate Hold", "certificateHold"},
    {CRL_REASON_REMOVE_FROM_CRL, "Remove From CRL", "removeFromCRL"},
    {CRL_REASON_PRIVILEGE_WITHDRAWN, "Privilege Withdrawn",
     "privilegeWithdrawn"},
    {CRL_REASON_AA_COMPROMISE, "AA Compromise", "AACompromise"},
    {-1, NULL, NULL}};

static char *i2s_ASN1_ENUMERATED_TABLE(const X509V3_EXT_METHOD *method,
                                       void *ext) {
  const ASN1_ENUMERATED *e = ext;
  long strval = ASN1_ENUMERATED_get(e);
  for (const ENUMERATED_NAMES *enam = method->usr_data; enam->lname; enam++) {
    if (strval == enam->bitnum) {
      return OPENSSL_strdup(enam->lname);
    }
  }
  return i2s_ASN1_ENUMERATED(method, e);
}

const X509V3_EXT_METHOD v3_crl_reason = {
    NID_crl_reason,
    0,
    ASN1_ITEM_ref(ASN1_ENUMERATED),
    0,
    0,
    0,
    0,
    i2s_ASN1_ENUMERATED_TABLE,
    0,
    0,
    0,
    0,
    0,
    (void *)crl_reasons,
};
