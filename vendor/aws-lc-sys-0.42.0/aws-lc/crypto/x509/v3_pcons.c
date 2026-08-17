// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project.
// Copyright (c) 2003 The OpenSSL Project.  All rights reserved.
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


static STACK_OF(CONF_VALUE) *i2v_POLICY_CONSTRAINTS(
    const X509V3_EXT_METHOD *method, void *bcons,
    STACK_OF(CONF_VALUE) *extlist);
static void *v2i_POLICY_CONSTRAINTS(const X509V3_EXT_METHOD *method,
                                    const X509V3_CTX *ctx,
                                    const STACK_OF(CONF_VALUE) *values);

const X509V3_EXT_METHOD v3_policy_constraints = {
    NID_policy_constraints,
    0,
    ASN1_ITEM_ref(POLICY_CONSTRAINTS),
    0,
    0,
    0,
    0,
    0,
    0,
    i2v_POLICY_CONSTRAINTS,
    v2i_POLICY_CONSTRAINTS,
    NULL,
    NULL,
    NULL};

ASN1_SEQUENCE(POLICY_CONSTRAINTS) = {
    ASN1_IMP_OPT(POLICY_CONSTRAINTS, requireExplicitPolicy, ASN1_INTEGER, 0),
    ASN1_IMP_OPT(POLICY_CONSTRAINTS, inhibitPolicyMapping, ASN1_INTEGER, 1),
} ASN1_SEQUENCE_END(POLICY_CONSTRAINTS)

IMPLEMENT_ASN1_ALLOC_FUNCTIONS(POLICY_CONSTRAINTS)

static STACK_OF(CONF_VALUE) *i2v_POLICY_CONSTRAINTS(
    const X509V3_EXT_METHOD *method, void *a, STACK_OF(CONF_VALUE) *extlist) {
  const POLICY_CONSTRAINTS *pcons = a;
  X509V3_add_value_int("Require Explicit Policy", pcons->requireExplicitPolicy,
                       &extlist);
  X509V3_add_value_int("Inhibit Policy Mapping", pcons->inhibitPolicyMapping,
                       &extlist);
  return extlist;
}

static void *v2i_POLICY_CONSTRAINTS(const X509V3_EXT_METHOD *method,
                                    const X509V3_CTX *ctx,
                                    const STACK_OF(CONF_VALUE) *values) {
  POLICY_CONSTRAINTS *pcons = NULL;
  if (!(pcons = POLICY_CONSTRAINTS_new())) {
    return NULL;
  }
  for (size_t i = 0; i < sk_CONF_VALUE_num(values); i++) {
    const CONF_VALUE *val = sk_CONF_VALUE_value(values, i);
    if (!strcmp(val->name, "requireExplicitPolicy")) {
      if (!X509V3_get_value_int(val, &pcons->requireExplicitPolicy)) {
        goto err;
      }
    } else if (!strcmp(val->name, "inhibitPolicyMapping")) {
      if (!X509V3_get_value_int(val, &pcons->inhibitPolicyMapping)) {
        goto err;
      }
    } else {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_NAME);
      X509V3_conf_err(val);
      goto err;
    }
  }
  if (!pcons->inhibitPolicyMapping && !pcons->requireExplicitPolicy) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_ILLEGAL_EMPTY_EXTENSION);
    goto err;
  }

  return pcons;
err:
  POLICY_CONSTRAINTS_free(pcons);
  return NULL;
}
