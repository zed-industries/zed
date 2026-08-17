// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999-2004 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include <openssl/obj.h>
#include <openssl/x509.h>


static char *i2s_ASN1_INTEGER_cb(const X509V3_EXT_METHOD *method, void *ext) {
  return i2s_ASN1_INTEGER(method, ext);
}

static void *s2i_asn1_int(const X509V3_EXT_METHOD *meth, const X509V3_CTX *ctx,
                          const char *value) {
  return s2i_ASN1_INTEGER(meth, value);
}

const X509V3_EXT_METHOD v3_crl_num = {
    NID_crl_number,
    0,
    ASN1_ITEM_ref(ASN1_INTEGER),
    0,
    0,
    0,
    0,
    i2s_ASN1_INTEGER_cb,
    0,
    0,
    0,
    0,
    0,
    NULL,
};

const X509V3_EXT_METHOD v3_delta_crl = {
    NID_delta_crl,
    0,
    ASN1_ITEM_ref(ASN1_INTEGER),
    0,
    0,
    0,
    0,
    i2s_ASN1_INTEGER_cb,
    0,
    0,
    0,
    0,
    0,
    NULL,
};

const X509V3_EXT_METHOD v3_inhibit_anyp = {
    NID_inhibit_any_policy,
    0,
    ASN1_ITEM_ref(ASN1_INTEGER),
    0,
    0,
    0,
    0,
    i2s_ASN1_INTEGER_cb,
    s2i_asn1_int,
    0,
    0,
    0,
    0,
    NULL,
};
