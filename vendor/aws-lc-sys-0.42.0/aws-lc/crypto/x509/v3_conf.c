// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999-2002 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

// extension creation utilities

#include <ctype.h>
#include <limits.h>
#include <stdio.h>
#include <string.h>

#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "../internal.h"
#include "internal.h"

static int v3_check_critical(const char **value);
static int v3_check_generic(const char **value);
static X509_EXTENSION *do_ext_nconf(const CONF *conf, const X509V3_CTX *ctx,
                                    int ext_nid, int crit, const char *value);
static X509_EXTENSION *v3_generic_extension(const char *ext, const char *value,
                                            int crit, int type,
                                            const X509V3_CTX *ctx);
static X509_EXTENSION *do_ext_i2d(const X509V3_EXT_METHOD *method, int ext_nid,
                                  int crit, void *ext_struc);
static unsigned char *generic_asn1(const char *value, const X509V3_CTX *ctx,
                                   size_t *ext_len);

static void delete_ext(STACK_OF(X509_EXTENSION) *sk, X509_EXTENSION *dext);

X509_EXTENSION *X509V3_EXT_nconf(const CONF *conf, const X509V3_CTX *ctx,
                                 const char *name, const char *value) {
  // If omitted, fill in an empty |X509V3_CTX|.
  X509V3_CTX ctx_tmp;
  if (ctx == NULL) {
    X509V3_set_ctx(&ctx_tmp, NULL, NULL, NULL, NULL, 0);
    X509V3_set_nconf(&ctx_tmp, conf);
    ctx = &ctx_tmp;
  }

  int crit = v3_check_critical(&value);
  int ext_type = v3_check_generic(&value);
  if (ext_type != 0) {
    return v3_generic_extension(name, value, crit, ext_type, ctx);
  }
  X509_EXTENSION *ret = do_ext_nconf(conf, ctx, OBJ_sn2nid(name), crit, value);
  if (!ret) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_ERROR_IN_EXTENSION);
    ERR_add_error_data(4, "name=", name, ", value=", value);
  }
  return ret;
}

X509_EXTENSION *X509V3_EXT_nconf_nid(const CONF *conf, const X509V3_CTX *ctx,
                                     int ext_nid, const char *value) {
  // If omitted, fill in an empty |X509V3_CTX|.
  X509V3_CTX ctx_tmp;
  if (ctx == NULL) {
    X509V3_set_ctx(&ctx_tmp, NULL, NULL, NULL, NULL, 0);
    X509V3_set_nconf(&ctx_tmp, conf);
    ctx = &ctx_tmp;
  }

  int crit = v3_check_critical(&value);
  int ext_type = v3_check_generic(&value);
  if (ext_type != 0) {
    return v3_generic_extension(OBJ_nid2sn(ext_nid), value, crit, ext_type,
                                ctx);
  }
  return do_ext_nconf(conf, ctx, ext_nid, crit, value);
}

// CONF *conf:  Config file
// char *value:  Value
static X509_EXTENSION *do_ext_nconf(const CONF *conf, const X509V3_CTX *ctx,
                                    int ext_nid, int crit, const char *value) {
  const X509V3_EXT_METHOD *method;
  X509_EXTENSION *ext;
  const STACK_OF(CONF_VALUE) *nval;
  STACK_OF(CONF_VALUE) *nval_owned = NULL;
  void *ext_struc;
  if (ext_nid == NID_undef) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_UNKNOWN_EXTENSION_NAME);
    return NULL;
  }
  if (!(method = X509V3_EXT_get_nid(ext_nid))) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_UNKNOWN_EXTENSION);
    return NULL;
  }
  // Now get internal extension representation based on type
  if (method->v2i) {
    if (*value == '@') {
      // TODO(davidben): This is the only place where |X509V3_EXT_nconf|'s
      // |conf| parameter is used. All other codepaths use the copy inside
      // |ctx|. Should this be switched and then the parameter ignored?
      if (conf == NULL) {
        OPENSSL_PUT_ERROR(X509V3, X509V3_R_NO_CONFIG_DATABASE);
        return NULL;
      }
      nval = NCONF_get_section(conf, value + 1);
    } else {
      nval_owned = X509V3_parse_list(value);
      nval = nval_owned;
    }
    if (nval == NULL || sk_CONF_VALUE_num(nval) <= 0) {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_EXTENSION_STRING);
      ERR_add_error_data(4, "name=", OBJ_nid2sn(ext_nid), ",section=", value);
      sk_CONF_VALUE_pop_free(nval_owned, X509V3_conf_free);
      return NULL;
    }
    ext_struc = method->v2i(method, ctx, nval);
    sk_CONF_VALUE_pop_free(nval_owned, X509V3_conf_free);
    if (!ext_struc) {
      return NULL;
    }
  } else if (method->s2i) {
    if (!(ext_struc = method->s2i(method, ctx, value))) {
      return NULL;
    }
  } else if (method->r2i) {
    // TODO(davidben): Should this check be removed? This matches OpenSSL, but
    // r2i-based extensions do not necessarily require a config database. The
    // two built-in extensions only use it some of the time, and already handle
    // |X509V3_get_section| returning NULL.
    if (!ctx->db) {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_NO_CONFIG_DATABASE);
      return NULL;
    }
    if (!(ext_struc = method->r2i(method, ctx, value))) {
      return NULL;
    }
  } else {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_EXTENSION_SETTING_NOT_SUPPORTED);
    ERR_add_error_data(2, "name=", OBJ_nid2sn(ext_nid));
    return NULL;
  }

  ext = do_ext_i2d(method, ext_nid, crit, ext_struc);
  ASN1_item_free(ext_struc, ASN1_ITEM_ptr(method->it));
  return ext;
}

static X509_EXTENSION *do_ext_i2d(const X509V3_EXT_METHOD *method, int ext_nid,
                                  int crit, void *ext_struc) {
  // Convert the extension's internal representation to DER.
  unsigned char *ext_der;
  int ext_len;
  if (method->it) {
    ext_der = NULL;
    ext_len = ASN1_item_i2d(ext_struc, &ext_der, ASN1_ITEM_ptr(method->it));
    if (ext_len < 0) {
      return NULL;
    }
  } else if (method->ext_nid == NID_id_pkix_OCSP_Nonce && method->i2d != NULL) {
    // |NID_id_pkix_OCSP_Nonce| is the only extension using the "old-style"
    // ASN.1 callbacks for backwards compatibility reasons.
    // Note: See |v3_ext_method| under "include/openssl/x509.h".
    ext_len = method->i2d(ext_struc, NULL);
    if (!(ext_der = OPENSSL_malloc(ext_len))) {
      return NULL;
    }
    unsigned char *p = ext_der;
    method->i2d(ext_struc, &p);
  } else {
    OPENSSL_PUT_ERROR(X509, X509V3_R_OPERATION_NOT_DEFINED);
    return NULL;
  }

  ASN1_OCTET_STRING *ext_oct = ASN1_OCTET_STRING_new();
  if (ext_oct == NULL) {
    OPENSSL_free(ext_der);
    return NULL;
  }
  ASN1_STRING_set0(ext_oct, ext_der, ext_len);

  X509_EXTENSION *ext =
      X509_EXTENSION_create_by_NID(NULL, ext_nid, crit, ext_oct);
  ASN1_OCTET_STRING_free(ext_oct);
  return ext;
}

// Given an internal structure, nid and critical flag create an extension

X509_EXTENSION *X509V3_EXT_i2d(int ext_nid, int crit, void *ext_struc) {
  const X509V3_EXT_METHOD *method;
  if (!(method = X509V3_EXT_get_nid(ext_nid))) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_UNKNOWN_EXTENSION);
    return NULL;
  }
  return do_ext_i2d(method, ext_nid, crit, ext_struc);
}

// Check the extension string for critical flag
static int v3_check_critical(const char **value) {
  const char *p = *value;
  if ((strlen(p) < 9) || strncmp(p, "critical,", 9)) {
    return 0;
  }
  p += 9;
  while (OPENSSL_isspace((unsigned char)*p)) {
    p++;
  }
  *value = p;
  return 1;
}

// Check extension string for generic extension and return the type
static int v3_check_generic(const char **value) {
  int gen_type = 0;
  const char *p = *value;
  if ((strlen(p) >= 4) && !strncmp(p, "DER:", 4)) {
    p += 4;
    gen_type = 1;
  } else if ((strlen(p) >= 5) && !strncmp(p, "ASN1:", 5)) {
    p += 5;
    gen_type = 2;
  } else {
    return 0;
  }

  while (OPENSSL_isspace((unsigned char)*p)) {
    p++;
  }
  *value = p;
  return gen_type;
}

// Create a generic extension: for now just handle DER type
static X509_EXTENSION *v3_generic_extension(const char *ext, const char *value,
                                            int crit, int gen_type,
                                            const X509V3_CTX *ctx) {
  unsigned char *ext_der = NULL;
  size_t ext_len = 0;
  ASN1_OBJECT *obj = NULL;
  ASN1_OCTET_STRING *oct = NULL;
  X509_EXTENSION *extension = NULL;
  if (!(obj = OBJ_txt2obj(ext, 0))) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_EXTENSION_NAME_ERROR);
    ERR_add_error_data(2, "name=", ext);
    goto err;
  }

  if (gen_type == 1) {
    ext_der = x509v3_hex_to_bytes(value, &ext_len);
  } else if (gen_type == 2) {
    ext_der = generic_asn1(value, ctx, &ext_len);
  }

  if (ext_der == NULL) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_EXTENSION_VALUE_ERROR);
    ERR_add_error_data(2, "value=", value);
    goto err;
  }

  if (ext_len > INT_MAX) {
    OPENSSL_PUT_ERROR(X509V3, ERR_R_OVERFLOW);
    goto err;
  }

  oct = ASN1_OCTET_STRING_new();
  if (oct == NULL) {
    goto err;
  }

  ASN1_STRING_set0(oct, ext_der, (int)ext_len);
  ext_der = NULL;

  extension = X509_EXTENSION_create_by_OBJ(NULL, obj, crit, oct);

err:
  ASN1_OBJECT_free(obj);
  ASN1_OCTET_STRING_free(oct);
  OPENSSL_free(ext_der);
  return extension;
}

static unsigned char *generic_asn1(const char *value, const X509V3_CTX *ctx,
                                   size_t *ext_len) {
  ASN1_TYPE *typ = ASN1_generate_v3(value, ctx);
  if (typ == NULL) {
    return NULL;
  }
  unsigned char *ext_der = NULL;
  int len = i2d_ASN1_TYPE(typ, &ext_der);
  ASN1_TYPE_free(typ);
  if (len < 0) {
    return NULL;
  }
  *ext_len = len;
  return ext_der;
}

static void delete_ext(STACK_OF(X509_EXTENSION) *sk, X509_EXTENSION *dext) {
  int idx = 0;
  ASN1_OBJECT *obj = NULL;

  obj = X509_EXTENSION_get_object(dext);
  while ((idx = X509v3_get_ext_by_OBJ(sk, obj, -1)) >= 0) {
    X509_EXTENSION_free(X509v3_delete_ext(sk, idx));
  }
}

// This is the main function: add a bunch of extensions based on a config
// file section to an extension STACK.

int X509V3_EXT_add_nconf_sk(const CONF *conf, const X509V3_CTX *ctx,
                            const char *section,
                            STACK_OF(X509_EXTENSION) **sk) {
  const STACK_OF(CONF_VALUE) *nval = NCONF_get_section(conf, section);
  if (nval == NULL) {
    return 0;
  }
  for (size_t i = 0; i < sk_CONF_VALUE_num(nval); i++) {
    const CONF_VALUE *val = sk_CONF_VALUE_value(nval, i);
    X509_EXTENSION *ext = X509V3_EXT_nconf(conf, ctx, val->name, val->value);

    int ok = 0;
    if (ext) {
      if (!sk) {
        ok = 1;
      } else {
        if (ctx && ctx->flags == X509V3_CTX_REPLACE) {
          delete_ext(*sk, ext);
        }
        ok = X509v3_add_ext(sk, ext, -1) != NULL;
      }
    }

    X509_EXTENSION_free(ext);
    if (!ok) {
      return 0;
    }
  }
  return 1;
}

// Convenience functions to add extensions to a certificate, CRL and request

int X509V3_EXT_add_nconf(const CONF *conf, const X509V3_CTX *ctx,
                         const char *section, X509 *cert) {
  STACK_OF(X509_EXTENSION) **sk = NULL;
  if (cert) {
    sk = &cert->cert_info->extensions;
  }
  return X509V3_EXT_add_nconf_sk(conf, ctx, section, sk);
}

// Same as above but for a CRL

int X509V3_EXT_CRL_add_nconf(const CONF *conf, const X509V3_CTX *ctx,
                             const char *section, X509_CRL *crl) {
  STACK_OF(X509_EXTENSION) **sk = NULL;
  if (crl) {
    sk = &crl->crl->extensions;
  }
  return X509V3_EXT_add_nconf_sk(conf, ctx, section, sk);
}

// Add extensions to certificate request

int X509V3_EXT_REQ_add_nconf(const CONF *conf, const X509V3_CTX *ctx,
                             const char *section, X509_REQ *req) {
  STACK_OF(X509_EXTENSION) *extlist = NULL, **sk = NULL;
  int i;
  if (req) {
    sk = &extlist;
  }
  i = X509V3_EXT_add_nconf_sk(conf, ctx, section, sk);
  if (!i || !sk) {
    return i;
  }
  i = X509_REQ_add_extensions(req, extlist);
  sk_X509_EXTENSION_pop_free(extlist, X509_EXTENSION_free);
  return i;
}

// Config database functions

const STACK_OF(CONF_VALUE) *X509V3_get_section(const X509V3_CTX *ctx,
                                               const char *section) {
  if (ctx->db == NULL) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_OPERATION_NOT_DEFINED);
    return NULL;
  }
  return NCONF_get_section(ctx->db, section);
}

void X509V3_set_nconf(X509V3_CTX *ctx, const CONF *conf) { ctx->db = conf; }

void X509V3_set_ctx(X509V3_CTX *ctx, const X509 *issuer, const X509 *subj,
                    const X509_REQ *req, const X509_CRL *crl, int flags) {
  OPENSSL_memset(ctx, 0, sizeof(*ctx));
  ctx->issuer_cert = issuer;
  ctx->subject_cert = subj;
  ctx->crl = crl;
  ctx->subject_req = req;
  ctx->flags = flags;
}
