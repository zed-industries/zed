// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project.
// Copyright (c) 2003 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include <openssl/asn1t.h>
#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"


static void *v2i_POLICY_MAPPINGS(const X509V3_EXT_METHOD *method,
                                 const X509V3_CTX *ctx,
                                 const STACK_OF(CONF_VALUE) *nval);
static STACK_OF(CONF_VALUE) *i2v_POLICY_MAPPINGS(
    const X509V3_EXT_METHOD *method, void *pmps, STACK_OF(CONF_VALUE) *extlist);

const X509V3_EXT_METHOD v3_policy_mappings = {
    NID_policy_mappings,
    0,
    ASN1_ITEM_ref(POLICY_MAPPINGS),
    0,
    0,
    0,
    0,
    0,
    0,
    i2v_POLICY_MAPPINGS,
    v2i_POLICY_MAPPINGS,
    0,
    0,
    NULL,
};

ASN1_SEQUENCE(POLICY_MAPPING) = {
    ASN1_SIMPLE(POLICY_MAPPING, issuerDomainPolicy, ASN1_OBJECT),
    ASN1_SIMPLE(POLICY_MAPPING, subjectDomainPolicy, ASN1_OBJECT),
} ASN1_SEQUENCE_END(POLICY_MAPPING)

ASN1_ITEM_TEMPLATE(POLICY_MAPPINGS) = ASN1_EX_TEMPLATE_TYPE(
    ASN1_TFLG_SEQUENCE_OF, 0, POLICY_MAPPINGS, POLICY_MAPPING)
ASN1_ITEM_TEMPLATE_END(POLICY_MAPPINGS)

IMPLEMENT_ASN1_ALLOC_FUNCTIONS(POLICY_MAPPING)

static STACK_OF(CONF_VALUE) *i2v_POLICY_MAPPINGS(
    const X509V3_EXT_METHOD *method, void *a, STACK_OF(CONF_VALUE) *ext_list) {
  const POLICY_MAPPINGS *pmaps = a;
  for (size_t i = 0; i < sk_POLICY_MAPPING_num(pmaps); i++) {
    const POLICY_MAPPING *pmap = sk_POLICY_MAPPING_value(pmaps, i);
    char obj_tmp1[80], obj_tmp2[80];
    i2t_ASN1_OBJECT(obj_tmp1, 80, pmap->issuerDomainPolicy);
    i2t_ASN1_OBJECT(obj_tmp2, 80, pmap->subjectDomainPolicy);
    X509V3_add_value(obj_tmp1, obj_tmp2, &ext_list);
  }
  return ext_list;
}

static void *v2i_POLICY_MAPPINGS(const X509V3_EXT_METHOD *method,
                                 const X509V3_CTX *ctx,
                                 const STACK_OF(CONF_VALUE) *nval) {
  POLICY_MAPPINGS *pmaps = sk_POLICY_MAPPING_new_null();
  if (pmaps == NULL) {
    return NULL;
  }

  for (size_t i = 0; i < sk_CONF_VALUE_num(nval); i++) {
    const CONF_VALUE *val = sk_CONF_VALUE_value(nval, i);
    if (!val->value || !val->name) {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_OBJECT_IDENTIFIER);
      X509V3_conf_err(val);
      goto err;
    }

    POLICY_MAPPING *pmap = POLICY_MAPPING_new();
    if (pmap == NULL || !sk_POLICY_MAPPING_push(pmaps, pmap)) {
      POLICY_MAPPING_free(pmap);
      goto err;
    }

    pmap->issuerDomainPolicy = OBJ_txt2obj(val->name, 0);
    pmap->subjectDomainPolicy = OBJ_txt2obj(val->value, 0);
    if (!pmap->issuerDomainPolicy || !pmap->subjectDomainPolicy) {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_OBJECT_IDENTIFIER);
      X509V3_conf_err(val);
      goto err;
    }
  }
  return pmaps;

err:
  sk_POLICY_MAPPING_pop_free(pmaps, POLICY_MAPPING_free);
  return NULL;
}
