// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>
#include <string.h>

#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"


static const BIT_STRING_BITNAME ns_cert_type_table[] = {
    {0, "SSL Client", "client"},
    {1, "SSL Server", "server"},
    {2, "S/MIME", "email"},
    {3, "Object Signing", "objsign"},
    {4, "Unused", "reserved"},
    {5, "SSL CA", "sslCA"},
    {6, "S/MIME CA", "emailCA"},
    {7, "Object Signing CA", "objCA"},
    {-1, NULL, NULL}};

static const BIT_STRING_BITNAME key_usage_type_table[] = {
    {0, "Digital Signature", "digitalSignature"},
    {1, "Non Repudiation", "nonRepudiation"},
    {2, "Key Encipherment", "keyEncipherment"},
    {3, "Data Encipherment", "dataEncipherment"},
    {4, "Key Agreement", "keyAgreement"},
    {5, "Certificate Sign", "keyCertSign"},
    {6, "CRL Sign", "cRLSign"},
    {7, "Encipher Only", "encipherOnly"},
    {8, "Decipher Only", "decipherOnly"},
    {-1, NULL, NULL}};

static STACK_OF(CONF_VALUE) *i2v_ASN1_BIT_STRING(
    const X509V3_EXT_METHOD *method, void *ext, STACK_OF(CONF_VALUE) *ret) {
  const ASN1_BIT_STRING *bits = ext;
  const BIT_STRING_BITNAME *bnam;
  for (bnam = method->usr_data; bnam->lname; bnam++) {
    if (ASN1_BIT_STRING_get_bit(bits, bnam->bitnum)) {
      X509V3_add_value(bnam->lname, NULL, &ret);
    }
  }
  return ret;
}

static void *v2i_ASN1_BIT_STRING(const X509V3_EXT_METHOD *method,
                                 const X509V3_CTX *ctx,
                                 const STACK_OF(CONF_VALUE) *nval) {
  ASN1_BIT_STRING *bs;
  if (!(bs = ASN1_BIT_STRING_new())) {
    return NULL;
  }
  for (size_t i = 0; i < sk_CONF_VALUE_num(nval); i++) {
    const CONF_VALUE *val = sk_CONF_VALUE_value(nval, i);
    const BIT_STRING_BITNAME *bnam;
    for (bnam = method->usr_data; bnam->lname; bnam++) {
      if (!strcmp(bnam->sname, val->name) || !strcmp(bnam->lname, val->name)) {
        if (!ASN1_BIT_STRING_set_bit(bs, bnam->bitnum, 1)) {
          ASN1_BIT_STRING_free(bs);
          return NULL;
        }
        break;
      }
    }
    if (!bnam->lname) {
      OPENSSL_PUT_ERROR(X509V3, X509V3_R_UNKNOWN_BIT_STRING_ARGUMENT);
      X509V3_conf_err(val);
      ASN1_BIT_STRING_free(bs);
      return NULL;
    }
  }
  return bs;
}

#define EXT_BITSTRING(nid, table)                                             \
  {                                                                           \
    nid, 0, ASN1_ITEM_ref(ASN1_BIT_STRING), 0, 0, 0, 0, 0, 0,                 \
        i2v_ASN1_BIT_STRING, v2i_ASN1_BIT_STRING, NULL, NULL, (void *)(table) \
  }

const X509V3_EXT_METHOD v3_nscert =
    EXT_BITSTRING(NID_netscape_cert_type, ns_cert_type_table);
const X509V3_EXT_METHOD v3_key_usage =
    EXT_BITSTRING(NID_key_usage, key_usage_type_table);
