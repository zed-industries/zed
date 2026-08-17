// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>
#include <string.h>

#include <openssl/asn1.h>
#include <openssl/asn1t.h>
#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"


static STACK_OF(CONF_VALUE) *i2v_BASIC_CONSTRAINTS(
    const X509V3_EXT_METHOD *method, void *ext, STACK_OF(CONF_VALUE) *extlist);
static void *v2i_BASIC_CONSTRAINTS(const X509V3_EXT_METHOD *method,
                                   const X509V3_CTX *ctx,
                                   const STACK_OF(CONF_VALUE) *values);

const X509V3_EXT_METHOD v3_bcons = {
    NID_basic_constraints,
    0,
    ASN1_ITEM_ref(BASIC_CONSTRAINTS),
    0,
    0,
    0,
    0,
    0,
    0,
    i2v_BASIC_CONSTRAINTS,
    v2i_BASIC_CONSTRAINTS,
    NULL,
    NULL,
    NULL,
};

ASN1_SEQUENCE(BASIC_CONSTRAINTS) = {
    ASN1_OPT(BASIC_CONSTRAINTS, ca, ASN1_FBOOLEAN),
    ASN1_OPT(BASIC_CONSTRAINTS, pathlen, ASN1_INTEGER),
} ASN1_SEQUENCE_END(BASIC_CONSTRAINTS)

IMPLEMENT_ASN1_FUNCTIONS_const(BASIC_CONSTRAINTS)

static STACK_OF(CONF_VALUE) *i2v_BASIC_CONSTRAINTS(
    const X509V3_EXT_METHOD *method, void *ext, STACK_OF(CONF_VALUE) *extlist) {
  const BASIC_CONSTRAINTS *bcons = ext;
  X509V3_add_value_bool("CA", bcons->ca, &extlist);
  X509V3_add_value_int("pathlen", bcons->pathlen, &extlist);
  return extlist;
}

static void *v2i_BASIC_CONSTRAINTS(const X509V3_EXT_METHOD *method,
                                   const X509V3_CTX *ctx,
                                   const STACK_OF(CONF_VALUE) *values) {
  BASIC_CONSTRAINTS *bcons = NULL;
  if (!(bcons = BASIC_CONSTRAINTS_new())) {
    return NULL;
  }
  for (size_t i = 0; i < sk_CONF_VALUE_num(values); i++) {
    const CONF_VALUE *val = sk_CONF_VALUE_value(values, i);
    if (!strcmp(val->name, "CA")) {
      if (!X509V3_get_value_bool(val, &bcons->ca)) {
        goto err;
      }
    } else if (!strcmp(val->name, "pathlen")) {
      if (!X509V3_get_value_int(val, &bcons->pathlen)) {
        goto err;
      }
    } else {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_NAME);
      X509V3_conf_err(val);
      goto err;
    }
  }
  return bcons;
err:
  BASIC_CONSTRAINTS_free(bcons);
  return NULL;
}
