// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include <openssl/asn1t.h>
#include <openssl/evp.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"


// X509_CERT_AUX routines. These are used to encode additional user
// modifiable data about a certificate. This data is appended to the X509
// encoding when the *_X509_AUX routines are used. This means that the
// "traditional" X509 routines will simply ignore the extra data.

static X509_CERT_AUX *aux_get(X509 *x);

ASN1_SEQUENCE(X509_CERT_AUX) = {
    ASN1_SEQUENCE_OF_OPT(X509_CERT_AUX, trust, ASN1_OBJECT),
    ASN1_IMP_SEQUENCE_OF_OPT(X509_CERT_AUX, reject, ASN1_OBJECT, 0),
    ASN1_OPT(X509_CERT_AUX, alias, ASN1_UTF8STRING),
    ASN1_OPT(X509_CERT_AUX, keyid, ASN1_OCTET_STRING),
} ASN1_SEQUENCE_END(X509_CERT_AUX)

IMPLEMENT_ASN1_FUNCTIONS_const(X509_CERT_AUX)

static X509_CERT_AUX *aux_get(X509 *x) {
  if (!x) {
    return NULL;
  }
  if (!x->aux && !(x->aux = X509_CERT_AUX_new())) {
    return NULL;
  }
  return x->aux;
}

int X509_alias_set1(X509 *x, const uint8_t *name, ossl_ssize_t len) {
  X509_CERT_AUX *aux;
  // TODO(davidben): Empty aliases are not meaningful in PKCS#12, and the
  // getters cannot quite represent them. Also erase the object if |len| is
  // zero.
  if (!name) {
    if (!x || !x->aux || !x->aux->alias) {
      return 1;
    }
    ASN1_UTF8STRING_free(x->aux->alias);
    x->aux->alias = NULL;
    return 1;
  }
  if (!(aux = aux_get(x))) {
    return 0;
  }
  if (!aux->alias && !(aux->alias = ASN1_UTF8STRING_new())) {
    return 0;
  }
  return ASN1_STRING_set(aux->alias, name, len);
}

int X509_keyid_set1(X509 *x, const uint8_t *id, ossl_ssize_t len) {
  X509_CERT_AUX *aux;
  // TODO(davidben): Empty key IDs are not meaningful in PKCS#12, and the
  // getters cannot quite represent them. Also erase the object if |len| is
  // zero.
  if (!id) {
    if (!x || !x->aux || !x->aux->keyid) {
      return 1;
    }
    ASN1_OCTET_STRING_free(x->aux->keyid);
    x->aux->keyid = NULL;
    return 1;
  }
  if (!(aux = aux_get(x))) {
    return 0;
  }
  if (!aux->keyid && !(aux->keyid = ASN1_OCTET_STRING_new())) {
    return 0;
  }
  return ASN1_STRING_set(aux->keyid, id, len);
}

const uint8_t *X509_alias_get0(const X509 *x, int *out_len) {
  const ASN1_UTF8STRING *alias = x->aux != NULL ? x->aux->alias : NULL;
  if (out_len != NULL) {
    *out_len = alias != NULL ? alias->length : 0;
  }
  return alias != NULL ? alias->data : NULL;
}

const uint8_t *X509_keyid_get0(const X509 *x, int *out_len) {
  const ASN1_OCTET_STRING *keyid = x->aux != NULL ? x->aux->keyid : NULL;
  if (out_len != NULL) {
    *out_len = keyid != NULL ? keyid->length : 0;
  }
  return keyid != NULL ? keyid->data : NULL;
}

int X509_add1_trust_object(X509 *x, const ASN1_OBJECT *obj) {
  ASN1_OBJECT *objtmp = OBJ_dup(obj);
  if (objtmp == NULL) {
    goto err;
  }
  X509_CERT_AUX *aux = aux_get(x);
  if (aux->trust == NULL) {
    aux->trust = sk_ASN1_OBJECT_new_null();
    if (aux->trust == NULL) {
      goto err;
    }
  }
  if (!sk_ASN1_OBJECT_push(aux->trust, objtmp)) {
    goto err;
  }
  return 1;

err:
  ASN1_OBJECT_free(objtmp);
  return 0;
}

int X509_add1_reject_object(X509 *x, const ASN1_OBJECT *obj) {
  ASN1_OBJECT *objtmp = OBJ_dup(obj);
  if (objtmp == NULL) {
    goto err;
  }
  X509_CERT_AUX *aux = aux_get(x);
  if (aux->reject == NULL) {
    aux->reject = sk_ASN1_OBJECT_new_null();
    if (aux->reject == NULL) {
      goto err;
    }
  }
  if (!sk_ASN1_OBJECT_push(aux->reject, objtmp)) {
    goto err;
  }
  return 1;

err:
  ASN1_OBJECT_free(objtmp);
  return 0;
}

void X509_trust_clear(X509 *x) {
  if (x->aux && x->aux->trust) {
    sk_ASN1_OBJECT_pop_free(x->aux->trust, ASN1_OBJECT_free);
    x->aux->trust = NULL;
  }
}

void X509_reject_clear(X509 *x) {
  if (x->aux && x->aux->reject) {
    sk_ASN1_OBJECT_pop_free(x->aux->reject, ASN1_OBJECT_free);
    x->aux->reject = NULL;
  }
}
