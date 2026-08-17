// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/asn1t.h>
#include <openssl/cipher.h>
#include <openssl/x509.h>

#include "internal.h"


ASN1_SEQUENCE(X509_EXTENSION) = {
    ASN1_SIMPLE(X509_EXTENSION, object, ASN1_OBJECT),
    ASN1_OPT(X509_EXTENSION, critical, ASN1_BOOLEAN),
    ASN1_SIMPLE(X509_EXTENSION, value, ASN1_OCTET_STRING),
} ASN1_SEQUENCE_END(X509_EXTENSION)

ASN1_ITEM_TEMPLATE(X509_EXTENSIONS) =
    ASN1_EX_TEMPLATE_TYPE(ASN1_TFLG_SEQUENCE_OF, 0, Extension, X509_EXTENSION)
ASN1_ITEM_TEMPLATE_END(X509_EXTENSIONS)

IMPLEMENT_ASN1_FUNCTIONS_const(X509_EXTENSION)
IMPLEMENT_ASN1_ENCODE_FUNCTIONS_const_fname(X509_EXTENSIONS, X509_EXTENSIONS,
                                            X509_EXTENSIONS)
IMPLEMENT_ASN1_DUP_FUNCTION_const(X509_EXTENSION)
