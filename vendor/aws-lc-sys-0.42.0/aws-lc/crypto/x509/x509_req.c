// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/asn1t.h>
#include <openssl/bn.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/pem.h>
#include <openssl/x509.h>

#include "../asn1/internal.h"
#include "internal.h"


long X509_REQ_get_version(const X509_REQ *req) {
  return ASN1_INTEGER_get(req->req_info->version);
}

X509_NAME *X509_REQ_get_subject_name(const X509_REQ *req) {
  return req->req_info->subject;
}

EVP_PKEY *X509_REQ_get_pubkey(const X509_REQ *req) {
  if (req == NULL || req->req_info == NULL) {
    OPENSSL_PUT_ERROR(X509, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }
  return X509_PUBKEY_get(req->req_info->pubkey);
}

EVP_PKEY *X509_REQ_get0_pubkey(const X509_REQ *req) {
  if (req == NULL || req->req_info == NULL) {
    OPENSSL_PUT_ERROR(X509, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }
  return X509_PUBKEY_get0(req->req_info->pubkey);
}

int X509_REQ_check_private_key(const X509_REQ *x, const EVP_PKEY *k) {
  const EVP_PKEY *xk = X509_REQ_get0_pubkey(x);
  if (xk == NULL) {
    return 0;
  }

  int ret = EVP_PKEY_cmp(xk, k);
  if (ret > 0) {
    return 1;
  }

  switch (ret) {
    case 0:
      OPENSSL_PUT_ERROR(X509, X509_R_KEY_VALUES_MISMATCH);
      return 0;
    case -1:
      OPENSSL_PUT_ERROR(X509, X509_R_KEY_TYPE_MISMATCH);
      return 0;
    case -2:
      if (EVP_PKEY_id(k) == EVP_PKEY_EC) {
        OPENSSL_PUT_ERROR(X509, ERR_R_EC_LIB);
      } else {
        OPENSSL_PUT_ERROR(X509, X509_R_UNKNOWN_KEY_TYPE);
      }
      return 0;
  }

  return 0;
}

int X509_REQ_extension_nid(int req_nid) {
  return req_nid == NID_ext_req || req_nid == NID_ms_ext_req;
}

STACK_OF(X509_EXTENSION) *X509_REQ_get_extensions(const X509_REQ *req) {
  if (req == NULL || req->req_info == NULL) {
    return NULL;
  }

  int idx = X509_REQ_get_attr_by_NID(req, NID_ext_req, -1);
  if (idx == -1) {
    idx = X509_REQ_get_attr_by_NID(req, NID_ms_ext_req, -1);
  }
  if (idx == -1) {
    return NULL;
  }

  const X509_ATTRIBUTE *attr = X509_REQ_get_attr(req, idx);
  // TODO(davidben): |X509_ATTRIBUTE_get0_type| is not const-correct. It should
  // take and return a const pointer.
  const ASN1_TYPE *ext = X509_ATTRIBUTE_get0_type((X509_ATTRIBUTE *)attr, 0);
  if (!ext || ext->type != V_ASN1_SEQUENCE) {
    return NULL;
  }
  const unsigned char *p = ext->value.sequence->data;
  return (STACK_OF(X509_EXTENSION) *)ASN1_item_d2i(
      NULL, &p, ext->value.sequence->length, ASN1_ITEM_rptr(X509_EXTENSIONS));
}

// Add a STACK_OF extensions to a certificate request: allow alternative OIDs
// in case we want to create a non standard one.

int X509_REQ_add_extensions_nid(X509_REQ *req,
                                const STACK_OF(X509_EXTENSION) *exts, int nid) {
  // Generate encoding of extensions
  unsigned char *ext = NULL;
  int ext_len =
      ASN1_item_i2d((ASN1_VALUE *)exts, &ext, ASN1_ITEM_rptr(X509_EXTENSIONS));
  if (ext_len <= 0) {
    return 0;
  }
  int ret = X509_REQ_add1_attr_by_NID(req, nid, V_ASN1_SEQUENCE, ext, ext_len);
  OPENSSL_free(ext);
  return ret;
}

// This is the normal usage: use the "official" OID
int X509_REQ_add_extensions(X509_REQ *req,
                            const STACK_OF(X509_EXTENSION) *exts) {
  return X509_REQ_add_extensions_nid(req, exts, NID_ext_req);
}

int X509_REQ_get_attr_count(const X509_REQ *req) {
  return (int)sk_X509_ATTRIBUTE_num(req->req_info->attributes);
}

int X509_REQ_get_attr_by_NID(const X509_REQ *req, int nid, int lastpos) {
  const ASN1_OBJECT *obj = OBJ_nid2obj(nid);
  if (obj == NULL) {
    return -1;
  }
  return X509_REQ_get_attr_by_OBJ(req, obj, lastpos);
}

int X509_REQ_get_attr_by_OBJ(const X509_REQ *req, const ASN1_OBJECT *obj,
                             int lastpos) {
  if (req->req_info->attributes == NULL) {
    return -1;
  }
  lastpos++;
  if (lastpos < 0) {
    lastpos = 0;
  }
  int n = (int)sk_X509_ATTRIBUTE_num(req->req_info->attributes);
  for (; lastpos < n; lastpos++) {
    const X509_ATTRIBUTE *attr =
        sk_X509_ATTRIBUTE_value(req->req_info->attributes, lastpos);
    if (attr == NULL) {
      return -1;
    }
    if (OBJ_cmp(attr->object, obj) == 0) {
      return lastpos;
    }
  }
  return -1;
}

X509_ATTRIBUTE *X509_REQ_get_attr(const X509_REQ *req, int loc) {
  if (req->req_info->attributes == NULL || loc < 0 ||
      sk_X509_ATTRIBUTE_num(req->req_info->attributes) <= (size_t)loc) {
    return NULL;
  }
  return sk_X509_ATTRIBUTE_value(req->req_info->attributes, loc);
}

X509_ATTRIBUTE *X509_REQ_delete_attr(X509_REQ *req, int loc) {
  if (req->req_info->attributes == NULL || loc < 0 ||
      sk_X509_ATTRIBUTE_num(req->req_info->attributes) <= (size_t)loc) {
    return NULL;
  }
  return sk_X509_ATTRIBUTE_delete(req->req_info->attributes, loc);
}

static int X509_REQ_add0_attr(X509_REQ *req, X509_ATTRIBUTE *attr) {
  if (req->req_info->attributes == NULL) {
    req->req_info->attributes = sk_X509_ATTRIBUTE_new_null();
  }
  if (req->req_info->attributes == NULL ||
      !sk_X509_ATTRIBUTE_push(req->req_info->attributes, attr)) {
    return 0;
  }

  return 1;
}

int X509_REQ_add1_attr(X509_REQ *req, const X509_ATTRIBUTE *attr) {
  X509_ATTRIBUTE *new_attr = X509_ATTRIBUTE_dup(attr);
  if (new_attr == NULL || !X509_REQ_add0_attr(req, new_attr)) {
    X509_ATTRIBUTE_free(new_attr);
    return 0;
  }

  return 1;
}

int X509_REQ_add1_attr_by_OBJ(X509_REQ *req, const ASN1_OBJECT *obj,
                              int attrtype, const unsigned char *data,
                              int len) {
  X509_ATTRIBUTE *attr =
      X509_ATTRIBUTE_create_by_OBJ(NULL, obj, attrtype, data, len);
  if (attr == NULL || !X509_REQ_add0_attr(req, attr)) {
    X509_ATTRIBUTE_free(attr);
    return 0;
  }

  return 1;
}

int X509_REQ_add1_attr_by_NID(X509_REQ *req, int nid, int attrtype,
                              const unsigned char *data, int len) {
  X509_ATTRIBUTE *attr =
      X509_ATTRIBUTE_create_by_NID(NULL, nid, attrtype, data, len);
  if (attr == NULL || !X509_REQ_add0_attr(req, attr)) {
    X509_ATTRIBUTE_free(attr);
    return 0;
  }

  return 1;
}

int X509_REQ_add1_attr_by_txt(X509_REQ *req, const char *attrname, int attrtype,
                              const unsigned char *data, int len) {
  X509_ATTRIBUTE *attr =
      X509_ATTRIBUTE_create_by_txt(NULL, attrname, attrtype, data, len);
  if (attr == NULL || !X509_REQ_add0_attr(req, attr)) {
    X509_ATTRIBUTE_free(attr);
    return 0;
  }

  return 1;
}

void X509_REQ_get0_signature(const X509_REQ *req, const ASN1_BIT_STRING **psig,
                             const X509_ALGOR **palg) {
  if (psig != NULL) {
    *psig = req->signature;
  }
  if (palg != NULL) {
    *palg = req->sig_alg;
  }
}

int X509_REQ_get_signature_nid(const X509_REQ *req) {
  return OBJ_obj2nid(req->sig_alg->algorithm);
}

int i2d_re_X509_REQ_tbs(X509_REQ *req, unsigned char **pp) {
  asn1_encoding_clear(&req->req_info->enc);
  return i2d_X509_REQ_INFO(req->req_info, pp);
}
