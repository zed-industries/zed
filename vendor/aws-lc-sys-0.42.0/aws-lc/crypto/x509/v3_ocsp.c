// Copyright 2000-2016 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include <string.h>

#include <openssl/x509.h>

#include <openssl/asn1.h>
#include <openssl/bio.h>
#include <openssl/nid.h>
#include "../internal.h"

// OCSP extensions and a couple of CRL entry extensions

static int i2r_ocsp_acutoff(const X509V3_EXT_METHOD *method, void *cutoff,
                            BIO *bp, int indent);

static void *ocsp_nonce_new(void);
static int i2d_ocsp_nonce(void *a, unsigned char **pp);
static void *d2i_ocsp_nonce(void *a, const unsigned char **pp, long length);
static void ocsp_nonce_free(void *a);
static int i2r_ocsp_nonce(const X509V3_EXT_METHOD *method, void *nonce,
                          BIO *out, int indent);

static int i2r_ocsp_nocheck(const X509V3_EXT_METHOD *method, void *nocheck,
                            BIO *out, int indent);
static void *s2i_ocsp_nocheck(const X509V3_EXT_METHOD *method,
                              const X509V3_CTX *ctx, const char *str);

const X509V3_EXT_METHOD v3_crl_invdate = {
    NID_invalidity_date,
    0,
    ASN1_ITEM_ref(ASN1_GENERALIZEDTIME),
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    i2r_ocsp_acutoff,
    0,
    NULL,
};

const X509V3_EXT_METHOD v3_ocsp_nonce = {
    NID_id_pkix_OCSP_Nonce,
    0,
    NULL,
    ocsp_nonce_new,
    ocsp_nonce_free,
    d2i_ocsp_nonce,
    i2d_ocsp_nonce,
    0,
    0,
    0,
    0,
    i2r_ocsp_nonce,
    0,
    NULL
};

const X509V3_EXT_METHOD v3_ocsp_nocheck = {
    NID_id_pkix_OCSP_noCheck,
    0,
    ASN1_ITEM_ref(ASN1_NULL),
    0,
    0,
    0,
    0,
    0,
    s2i_ocsp_nocheck,
    0,
    0,
    i2r_ocsp_nocheck,
    0,
    NULL,
};

static int i2r_ocsp_acutoff(const X509V3_EXT_METHOD *method, void *cutoff,
                            BIO *bp, int ind) {
  if (BIO_printf(bp, "%*s", ind, "") <= 0) {
    return 0;
  }
  if (!ASN1_GENERALIZEDTIME_print(bp, cutoff)) {
    return 0;
  }
  return 1;
}

// OCSP nonce. This is needs special treatment because it doesn't have an
// ASN1 encoding at all: it just contains arbitrary data.

static void *ocsp_nonce_new(void) { return ASN1_OCTET_STRING_new(); }

static int i2d_ocsp_nonce(void *a, unsigned char **pp) {
  ASN1_OCTET_STRING *os = a;
  if (pp != NULL) {
    OPENSSL_memcpy(*pp, os->data, os->length);
    *pp += os->length;
  }
  return os->length;
}

static void *d2i_ocsp_nonce(void *a, const unsigned char **pp, long length) {
  ASN1_OCTET_STRING *os, **pos;
  pos = a;
  if (pos == NULL || *pos == NULL) {
    os = ASN1_OCTET_STRING_new();
    if (os == NULL) {
      goto err;
    }
  } else {
    os = *pos;
  }
  if (!ASN1_OCTET_STRING_set(os, *pp, (int)length)) {
    goto err;
  }

  *pp += length;

  if (pos != NULL) {
    *pos = os;
  }
  return os;

err:
  if ((pos == NULL) || (*pos != os)) {
    ASN1_OCTET_STRING_free(os);
  }
  OPENSSL_PUT_ERROR(OCSP, ERR_R_MALLOC_FAILURE);
  return NULL;
}

static void ocsp_nonce_free(void *a) { ASN1_OCTET_STRING_free(a); }

static int i2r_ocsp_nonce(const X509V3_EXT_METHOD *method, void *nonce,
                          BIO *out, int indent) {
  if (BIO_printf(out, "%*s", indent, "") <= 0) {
    return 0;
  }
  if (i2a_ASN1_STRING(out, nonce, V_ASN1_OCTET_STRING) <= 0) {
    return 0;
  }
  return 1;
}

// Nocheck is just a single NULL. Don't print anything and always set it

static int i2r_ocsp_nocheck(const X509V3_EXT_METHOD *method, void *nocheck,
                            BIO *out, int indent) {
  return 1;
}

static void *s2i_ocsp_nocheck(const X509V3_EXT_METHOD *method,
                              const X509V3_CTX *ctx, const char *str) {
  return ASN1_NULL_new();
}
