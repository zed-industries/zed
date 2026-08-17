// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include <openssl/asn1t.h>
#include <openssl/x509.h>

#include "internal.h"


ASN1_SEQUENCE(X509_VAL) = {
    ASN1_SIMPLE(X509_VAL, notBefore, ASN1_TIME),
    ASN1_SIMPLE(X509_VAL, notAfter, ASN1_TIME),
} ASN1_SEQUENCE_END(X509_VAL)

IMPLEMENT_ASN1_FUNCTIONS_const(X509_VAL)
