// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/asn1t.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"


ASN1_SEQUENCE(X509_ATTRIBUTE) = {
    ASN1_SIMPLE(X509_ATTRIBUTE, object, ASN1_OBJECT),
    ASN1_SET_OF(X509_ATTRIBUTE, set, ASN1_ANY),
} ASN1_SEQUENCE_END(X509_ATTRIBUTE)

IMPLEMENT_ASN1_FUNCTIONS_const(X509_ATTRIBUTE)
IMPLEMENT_ASN1_DUP_FUNCTION_const(X509_ATTRIBUTE)

X509_ATTRIBUTE *X509_ATTRIBUTE_create(int nid, int attrtype, void *value) {
  ASN1_OBJECT *obj = OBJ_nid2obj(nid);
  if (obj == NULL) {
    return NULL;
  }

  X509_ATTRIBUTE *ret = X509_ATTRIBUTE_new();
  ASN1_TYPE *val = ASN1_TYPE_new();
  if (ret == NULL || val == NULL) {
    goto err;
  }

  ret->object = obj;
  if (!sk_ASN1_TYPE_push(ret->set, val)) {
    goto err;
  }

  ASN1_TYPE_set(val, attrtype, value);
  return ret;

err:
  X509_ATTRIBUTE_free(ret);
  ASN1_TYPE_free(val);
  return NULL;
}
