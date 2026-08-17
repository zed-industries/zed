// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>
#include <string.h>

#include <openssl/asn1.h>
#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "../internal.h"


static char *i2s_ASN1_IA5STRING(const X509V3_EXT_METHOD *method, void *ext) {
  const ASN1_IA5STRING *ia5 = ext;
  char *tmp;
  if (!ia5 || !ia5->length) {
    return NULL;
  }
  if (!(tmp = OPENSSL_malloc(ia5->length + 1))) {
    return NULL;
  }
  OPENSSL_memcpy(tmp, ia5->data, ia5->length);
  tmp[ia5->length] = 0;
  return tmp;
}

static void *s2i_ASN1_IA5STRING(const X509V3_EXT_METHOD *method,
                                const X509V3_CTX *ctx, const char *str) {
  ASN1_IA5STRING *ia5;
  if (!str) {
    OPENSSL_PUT_ERROR(X509V3, X509V3_R_INVALID_NULL_ARGUMENT);
    return NULL;
  }
  if (!(ia5 = ASN1_IA5STRING_new())) {
    goto err;
  }
  if (!ASN1_STRING_set(ia5, str, strlen(str))) {
    ASN1_IA5STRING_free(ia5);
    goto err;
  }
  return ia5;
err:
  return NULL;
}

#define EXT_IA5STRING(nid)                                                 \
  {                                                                        \
    nid, 0, ASN1_ITEM_ref(ASN1_IA5STRING), 0, 0, 0, 0, i2s_ASN1_IA5STRING, \
        s2i_ASN1_IA5STRING, 0, 0, 0, 0, NULL                               \
  }

#define EXT_END \
  { -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 }

const X509V3_EXT_METHOD v3_ns_ia5_list[] = {
    EXT_IA5STRING(NID_netscape_base_url),
    EXT_IA5STRING(NID_netscape_revocation_url),
    EXT_IA5STRING(NID_netscape_ca_revocation_url),
    EXT_IA5STRING(NID_netscape_renewal_url),
    EXT_IA5STRING(NID_netscape_ca_policy_url),
    EXT_IA5STRING(NID_netscape_ssl_server_name),
    EXT_IA5STRING(NID_netscape_comment),
    EXT_END};
