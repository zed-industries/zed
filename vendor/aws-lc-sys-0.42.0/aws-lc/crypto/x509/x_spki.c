// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

// This module was send to me my Pat Richards <patr@x509.com> who wrote it.
// It is under my Copyright with his permission.

#include <openssl/asn1t.h>
#include <openssl/x509.h>


ASN1_SEQUENCE(NETSCAPE_SPKAC) = {
    ASN1_SIMPLE(NETSCAPE_SPKAC, pubkey, X509_PUBKEY),
    ASN1_SIMPLE(NETSCAPE_SPKAC, challenge, ASN1_IA5STRING),
} ASN1_SEQUENCE_END(NETSCAPE_SPKAC)

IMPLEMENT_ASN1_FUNCTIONS_const(NETSCAPE_SPKAC)

ASN1_SEQUENCE(NETSCAPE_SPKI) = {
    ASN1_SIMPLE(NETSCAPE_SPKI, spkac, NETSCAPE_SPKAC),
    ASN1_SIMPLE(NETSCAPE_SPKI, sig_algor, X509_ALGOR),
    ASN1_SIMPLE(NETSCAPE_SPKI, signature, ASN1_BIT_STRING),
} ASN1_SEQUENCE_END(NETSCAPE_SPKI)

IMPLEMENT_ASN1_FUNCTIONS_const(NETSCAPE_SPKI)
