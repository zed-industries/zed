// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2000.
// Copyright (c) 2000 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/x509.h>

#include <openssl/asn1.h>
#include <openssl/asn1t.h>
#include <openssl/digest.h>
#include <openssl/obj.h>

#include "../asn1/internal.h"


ASN1_SEQUENCE(X509_ALGOR) = {
    ASN1_SIMPLE(X509_ALGOR, algorithm, ASN1_OBJECT),
    ASN1_OPT(X509_ALGOR, parameter, ASN1_ANY),
} ASN1_SEQUENCE_END(X509_ALGOR)

IMPLEMENT_ASN1_FUNCTIONS_const(X509_ALGOR)
IMPLEMENT_ASN1_DUP_FUNCTION_const(X509_ALGOR)

int X509_ALGOR_set0(X509_ALGOR *alg, ASN1_OBJECT *aobj, int ptype, void *pval) {
  if (!alg) {
    return 0;
  }
  if (ptype != V_ASN1_UNDEF) {
    if (alg->parameter == NULL) {
      alg->parameter = ASN1_TYPE_new();
    }
    if (alg->parameter == NULL) {
      return 0;
    }
  }
  if (alg) {
    ASN1_OBJECT_free(alg->algorithm);
    alg->algorithm = aobj;
  }
  if (ptype == 0) {
    return 1;
  }
  if (ptype == V_ASN1_UNDEF) {
    if (alg->parameter) {
      ASN1_TYPE_free(alg->parameter);
      alg->parameter = NULL;
    }
  } else {
    ASN1_TYPE_set(alg->parameter, ptype, pval);
  }
  return 1;
}

void X509_ALGOR_get0(const ASN1_OBJECT **out_obj, int *out_param_type,
                     const void **out_param_value, const X509_ALGOR *alg) {
  if (out_obj != NULL) {
    *out_obj = alg->algorithm;
  }
  if (out_param_type != NULL) {
    int type = V_ASN1_UNDEF;
    const void *value = NULL;
    if (alg->parameter != NULL) {
      type = alg->parameter->type;
      value = asn1_type_value_as_pointer(alg->parameter);
    }
    *out_param_type = type;
    if (out_param_value != NULL) {
      *out_param_value = value;
    }
  }
}

// Set up an X509_ALGOR DigestAlgorithmIdentifier from an EVP_MD

int X509_ALGOR_set_md(X509_ALGOR *alg, const EVP_MD *md) {
  int param_type;

  if (EVP_MD_flags(md) & EVP_MD_FLAG_DIGALGID_ABSENT) {
    param_type = V_ASN1_UNDEF;
  } else {
    param_type = V_ASN1_NULL;
  }

  return X509_ALGOR_set0(alg, OBJ_nid2obj(EVP_MD_type(md)), param_type, NULL);
}

// X509_ALGOR_cmp returns 0 if |a| and |b| are equal and non-zero otherwise.
int X509_ALGOR_cmp(const X509_ALGOR *a, const X509_ALGOR *b) {
  int rv;
  rv = OBJ_cmp(a->algorithm, b->algorithm);
  if (rv) {
    return rv;
  }
  if (!a->parameter && !b->parameter) {
    return 0;
  }
  return ASN1_TYPE_cmp(a->parameter, b->parameter);
}
