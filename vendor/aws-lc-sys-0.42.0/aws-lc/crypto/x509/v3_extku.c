// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include <openssl/asn1t.h>
#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"


static void *v2i_EXTENDED_KEY_USAGE(const X509V3_EXT_METHOD *method,
                                    const X509V3_CTX *ctx,
                                    const STACK_OF(CONF_VALUE) *nval);
static STACK_OF(CONF_VALUE) *i2v_EXTENDED_KEY_USAGE(
    const X509V3_EXT_METHOD *method, void *eku, STACK_OF(CONF_VALUE) *extlist);

const X509V3_EXT_METHOD v3_ext_ku = {
    NID_ext_key_usage,
    0,
    ASN1_ITEM_ref(EXTENDED_KEY_USAGE),
    0,
    0,
    0,
    0,
    0,
    0,
    i2v_EXTENDED_KEY_USAGE,
    v2i_EXTENDED_KEY_USAGE,
    0,
    0,
    NULL,
};

// NB OCSP acceptable responses also is a SEQUENCE OF OBJECT
const X509V3_EXT_METHOD v3_ocsp_accresp = {
    NID_id_pkix_OCSP_acceptableResponses,
    0,
    ASN1_ITEM_ref(EXTENDED_KEY_USAGE),
    0,
    0,
    0,
    0,
    0,
    0,
    i2v_EXTENDED_KEY_USAGE,
    v2i_EXTENDED_KEY_USAGE,
    0,
    0,
    NULL,
};

ASN1_ITEM_TEMPLATE(EXTENDED_KEY_USAGE) = ASN1_EX_TEMPLATE_TYPE(
    ASN1_TFLG_SEQUENCE_OF, 0, EXTENDED_KEY_USAGE, ASN1_OBJECT)
ASN1_ITEM_TEMPLATE_END(EXTENDED_KEY_USAGE)

IMPLEMENT_ASN1_FUNCTIONS_const(EXTENDED_KEY_USAGE)

static STACK_OF(CONF_VALUE) *i2v_EXTENDED_KEY_USAGE(
    const X509V3_EXT_METHOD *method, void *a, STACK_OF(CONF_VALUE) *ext_list) {
  const EXTENDED_KEY_USAGE *eku = a;
  for (size_t i = 0; i < sk_ASN1_OBJECT_num(eku); i++) {
    const ASN1_OBJECT *obj = sk_ASN1_OBJECT_value(eku, i);
    char obj_tmp[80];
    i2t_ASN1_OBJECT(obj_tmp, 80, obj);
    X509V3_add_value(NULL, obj_tmp, &ext_list);
  }
  return ext_list;
}

static void *v2i_EXTENDED_KEY_USAGE(const X509V3_EXT_METHOD *method,
                                    const X509V3_CTX *ctx,
                                    const STACK_OF(CONF_VALUE) *nval) {
  EXTENDED_KEY_USAGE *extku = sk_ASN1_OBJECT_new_null();
  if (extku == NULL) {
    return NULL;
  }

  for (size_t i = 0; i < sk_CONF_VALUE_num(nval); i++) {
    const CONF_VALUE *val = sk_CONF_VALUE_value(nval, i);
    const char *extval;
    if (val->value) {
      extval = val->value;
    } else {
      extval = val->name;
    }
    ASN1_OBJECT *obj = OBJ_txt2obj(extval, 0);
    if (obj == NULL || !sk_ASN1_OBJECT_push(extku, obj)) {
      ASN1_OBJECT_free(obj);
      sk_ASN1_OBJECT_pop_free(extku, ASN1_OBJECT_free);
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_OBJECT_IDENTIFIER);
      X509V3_conf_err(val);
      return NULL;
    }
  }

  return extku;
}
