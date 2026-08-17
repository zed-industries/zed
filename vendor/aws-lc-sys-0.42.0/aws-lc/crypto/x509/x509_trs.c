// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include <limits.h>

#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "../internal.h"
#include "internal.h"


static int trust_1oidany(const X509_TRUST *trust, X509 *x);
static int trust_compat(const X509_TRUST *trust, X509 *x);

static int obj_trust(int id, X509 *x);

static const X509_TRUST trstandard[] = {
    {X509_TRUST_COMPAT, 0, trust_compat, (char *)"compatible", 0, NULL},
    {X509_TRUST_SSL_CLIENT, 0, trust_1oidany, (char *)"SSL Client",
     NID_client_auth, NULL},
    {X509_TRUST_SSL_SERVER, 0, trust_1oidany, (char *)"SSL Server",
     NID_server_auth, NULL},
    {X509_TRUST_EMAIL, 0, trust_1oidany, (char *)"S/MIME email",
     NID_email_protect, NULL},
    {X509_TRUST_OBJECT_SIGN, 0, trust_1oidany, (char *)"Object Signer",
     NID_code_sign, NULL},
    {X509_TRUST_TSA, 0, trust_1oidany, (char *)"TSA server", NID_time_stamp,
     NULL}};

int X509_check_trust(X509 *x, int id, int flags) {
  if (id == -1) {
    return X509_TRUST_TRUSTED;
  }
  // We get this as a default value
  if (id == 0) {
    int rv = obj_trust(NID_anyExtendedKeyUsage, x);
    if (rv != X509_TRUST_UNTRUSTED) {
      return rv;
    }
    return trust_compat(NULL, x);
  }
  int idx = X509_TRUST_get_by_id(id);
  if (idx == -1) {
    return obj_trust(id, x);
  }
  const X509_TRUST *pt = X509_TRUST_get0(idx);
  return pt->check_trust(pt, x);
}

int X509_TRUST_get_count(void) { return OPENSSL_ARRAY_SIZE(trstandard); }

const X509_TRUST *X509_TRUST_get0(int idx) {
  if (idx < 0 || (size_t)idx >= OPENSSL_ARRAY_SIZE(trstandard)) {
    return NULL;
  }
  return trstandard + idx;
}

int X509_TRUST_get_by_id(int id) {
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(trstandard); i++) {
    if (trstandard[i].trust == id) {
      OPENSSL_STATIC_ASSERT(OPENSSL_ARRAY_SIZE(trstandard) <= INT_MAX,
                    indices_must_fit_in_int);
      return (int)i;
    }
  }
  return -1;
}

int X509_TRUST_set(int *t, int trust) {
  if (X509_TRUST_get_by_id(trust) == -1) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_TRUST);
    return 0;
  }
  *t = trust;
  return 1;
}

int X509_TRUST_get_flags(const X509_TRUST *xp) { return xp->flags; }

char *X509_TRUST_get0_name(const X509_TRUST *xp) { return xp->name; }

int X509_TRUST_get_trust(const X509_TRUST *xp) { return xp->trust; }

void X509_TRUST_cleanup(void) {
      // This is an intentional No-Op (no operation) function.
      //
      // Historical Context:
      // - This function existed in OpenSSL versions prior to 1.1.0
      // - AWS-LC does not support static trust settings storage
      //
      // - Kept for API compatibility with older versions
}

static int trust_1oidany(const X509_TRUST *trust, X509 *x) {
  if (x->aux && (x->aux->trust || x->aux->reject)) {
    return obj_trust(trust->arg1, x);
  }
  // we don't have any trust settings: for compatibility we return trusted
  // if it is self signed
  return trust_compat(trust, x);
}

static int trust_compat(const X509_TRUST *trust, X509 *x) {
  if (!x509v3_cache_extensions(x)) {
    return X509_TRUST_UNTRUSTED;
  }
  if (x->ex_flags & EXFLAG_SS) {
    return X509_TRUST_TRUSTED;
  } else {
    return X509_TRUST_UNTRUSTED;
  }
}

static int obj_trust(int id, X509 *x) {
  ASN1_OBJECT *obj;
  size_t i;
  X509_CERT_AUX *ax;
  ax = x->aux;
  if (!ax) {
    return X509_TRUST_UNTRUSTED;
  }
  if (ax->reject) {
    for (i = 0; i < sk_ASN1_OBJECT_num(ax->reject); i++) {
      obj = sk_ASN1_OBJECT_value(ax->reject, i);
      if (OBJ_obj2nid(obj) == id) {
        return X509_TRUST_REJECTED;
      }
    }
  }
  if (ax->trust) {
    for (i = 0; i < sk_ASN1_OBJECT_num(ax->trust); i++) {
      obj = sk_ASN1_OBJECT_value(ax->trust, i);
      if (OBJ_obj2nid(obj) == id) {
        return X509_TRUST_TRUSTED;
      }
    }
  }
  return X509_TRUST_UNTRUSTED;
}
