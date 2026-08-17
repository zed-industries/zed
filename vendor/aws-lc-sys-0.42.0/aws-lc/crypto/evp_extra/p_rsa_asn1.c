// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2006.
// Copyright (c) 2006 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/rsa.h>

#include "../rsa_extra/internal.h"
#include "../fipsmodule/evp/internal.h"
#include "../fipsmodule/rsa/internal.h"
#include "internal.h"

static int rsa_pub_encode(CBB *out, const EVP_PKEY *key) {
  // See RFC 3279, section 2.3.1.
  CBB spki, algorithm, oid, null, key_bitstring;
  if (!CBB_add_asn1(out, &spki, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&spki, &algorithm, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&algorithm, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, rsa_asn1_meth.oid, rsa_asn1_meth.oid_len) ||
      !CBB_add_asn1(&algorithm, &null, CBS_ASN1_NULL) ||
      !CBB_add_asn1(&spki, &key_bitstring, CBS_ASN1_BITSTRING) ||
      !CBB_add_u8(&key_bitstring, 0 /* padding */) ||
      !RSA_marshal_public_key(&key_bitstring, key->pkey.rsa) ||
      !CBB_flush(out)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_ENCODE_ERROR);
    return 0;
  }

  return 1;
}

static int rsa_pub_decode(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key) {
  // The IETF specification defines that the parameters must be
  // NULL. See RFC 3279, section 2.3.1.
  // There is also an ITU-T X.509 specification that is rarely seen,
  // which defines a KeySize parameter. See ITU-T X.509 2008-11
  // AlgorithmObjectIdentifiers.
  //
  // OpenSSL ignores both parameters when parsing, however.

  RSA *rsa = RSA_parse_public_key(key);
  if (rsa == NULL || CBS_len(key) != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    RSA_free(rsa);
    return 0;
  }

  EVP_PKEY_assign_RSA(out, rsa);
  return 1;
}

static int rsa_pss_pub_decode(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key) {
  RSASSA_PSS_PARAMS *pss = NULL;
  if (!RSASSA_PSS_parse_params(params, &pss)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }
  RSA *rsa = RSA_parse_public_key(key);
  if (rsa != NULL) {
    rsa->pss = pss;
  } else {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    RSASSA_PSS_PARAMS_free(pss);
    return 0;
  }
  if (rsa == NULL ||
      CBS_len(key) != 0 ||
      !EVP_PKEY_assign(out, EVP_PKEY_RSA_PSS, rsa)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    RSA_free(rsa);
    return 0;
  }
  return 1;
}

static int rsa_pub_cmp(const EVP_PKEY *a, const EVP_PKEY *b) {
  return BN_cmp(b->pkey.rsa->n, a->pkey.rsa->n) == 0 &&
         BN_cmp(b->pkey.rsa->e, a->pkey.rsa->e) == 0;
}

static int rsa_priv_encode(CBB *out, const EVP_PKEY *key) {
  CBB pkcs8, algorithm, oid, null, private_key;
  if (!CBB_add_asn1(out, &pkcs8, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&pkcs8, PKCS8_VERSION_ONE /* version */) ||
      !CBB_add_asn1(&pkcs8, &algorithm, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&algorithm, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, rsa_asn1_meth.oid, rsa_asn1_meth.oid_len) ||
      !CBB_add_asn1(&algorithm, &null, CBS_ASN1_NULL) ||
      !CBB_add_asn1(&pkcs8, &private_key, CBS_ASN1_OCTETSTRING) ||
      !RSA_marshal_private_key(&private_key, key->pkey.rsa) ||
      !CBB_flush(out)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_ENCODE_ERROR);
    return 0;
  }

  return 1;
}

static int rsa_priv_decode(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key, CBS *pubkey) {
  if(pubkey) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }

  // Per RFC 8017, A.1, the parameters have type NULL.
  CBS null;
  if (!CBS_get_asn1(params, &null, CBS_ASN1_NULL) ||
      CBS_len(&null) != 0 ||
      CBS_len(params) != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }

  RSA *rsa = RSA_parse_private_key(key);
  if (rsa == NULL || CBS_len(key) != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    RSA_free(rsa);
    return 0;
  }

  EVP_PKEY_assign_RSA(out, rsa);
  return 1;
}

static int rsa_pss_priv_decode(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key, CBS *pubkey) {
  RSASSA_PSS_PARAMS *pss = NULL;
  if (!RSASSA_PSS_parse_params(params, &pss)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }
  RSA *rsa = RSA_parse_private_key(key);
  if (rsa != NULL) {
    rsa->pss = pss;
  } else {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    RSASSA_PSS_PARAMS_free(pss);
    return 0;
  }
  if (rsa == NULL ||
      CBS_len(key) != 0 ||
      !EVP_PKEY_assign(out, EVP_PKEY_RSA_PSS, rsa)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    RSA_free(rsa);
    return 0;
  }
  return 1;
}

static int rsa_opaque(const EVP_PKEY *pkey) {
  return RSA_is_opaque(pkey->pkey.rsa);
}

static int int_rsa_size(const EVP_PKEY *pkey) {
  return RSA_size(pkey->pkey.rsa);
}

static int rsa_bits(const EVP_PKEY *pkey) {
  return RSA_bits(pkey->pkey.rsa);
}

static void int_rsa_free(EVP_PKEY *pkey) { RSA_free(pkey->pkey.rsa); }

const EVP_PKEY_ASN1_METHOD rsa_asn1_meth = {
  EVP_PKEY_RSA,
  // 1.2.840.113549.1.1.1
  {0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01}, 9,

  "RSA",
  "OpenSSL RSA method",

  rsa_pub_decode,
  rsa_pub_encode,
  rsa_pub_cmp,

  rsa_priv_decode,
  rsa_priv_encode,
  NULL /* priv_encode_v2 */,

  NULL /* set_priv_raw */,
  NULL /* set_pub_raw */,
  NULL /* get_priv_raw */,
  NULL /* get_pub_raw */,
  NULL /* get_priv_seed */,

  rsa_opaque,

  int_rsa_size,
  rsa_bits,

  0,0,0,

  int_rsa_free,
};

const EVP_PKEY_ASN1_METHOD rsa_pss_asn1_meth = {
  EVP_PKEY_RSA_PSS,
  // 1.2.840.113549.1.1.10
  {0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0a}, 9,

  "RSA-PSS",
  "OpenSSL RSA-PSS method",

  rsa_pss_pub_decode,
  NULL /* pub_encode */,
  rsa_pub_cmp,

  rsa_pss_priv_decode,
  NULL /* priv_encode */,
  NULL /* priv_encode_v2 */,

  NULL /* set_priv_raw */,
  NULL /* set_pub_raw */,
  NULL /* get_priv_raw */,
  NULL /* get_pub_raw */,
  NULL /* get_priv_seed */,

  rsa_opaque,

  int_rsa_size,
  rsa_bits,

  0,0,0,

  int_rsa_free,
};
