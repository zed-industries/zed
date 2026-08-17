// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2000.
// Copyright (c) 2000-2005 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/rsa.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../fipsmodule/rsa/internal.h"
#include "../bytestring/internal.h"
#include "../internal.h"


static int parse_integer(CBS *cbs, BIGNUM **out) {
  assert(*out == NULL);
  *out = BN_new();
  if (*out == NULL) {
    return 0;
  }
  return BN_parse_asn1_unsigned(cbs, *out);
}

static int marshal_integer(CBB *cbb, BIGNUM *bn) {
  if (bn == NULL) {
    // An RSA object may be missing some components.
    OPENSSL_PUT_ERROR(RSA, RSA_R_VALUE_MISSING);
    return 0;
  }
  return BN_marshal_asn1(cbb, bn);
}

RSA *RSA_parse_public_key(CBS *cbs) {
  RSA *ret = RSA_new();
  if (ret == NULL) {
    return NULL;
  }
  CBS child;
  if (!CBS_get_asn1(cbs, &child, CBS_ASN1_SEQUENCE) ||
      !parse_integer(&child, &ret->n) ||
      !parse_integer(&child, &ret->e) ||
      CBS_len(&child) != 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_ENCODING);
    RSA_free(ret);
    return NULL;
  }

  if (!RSA_check_key(ret)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_RSA_PARAMETERS);
    RSA_free(ret);
    return NULL;
  }

  return ret;
}

RSA *RSA_public_key_from_bytes(const uint8_t *in, size_t in_len) {
  CBS cbs;
  CBS_init(&cbs, in, in_len);
  RSA *ret = RSA_parse_public_key(&cbs);
  if (ret == NULL || CBS_len(&cbs) != 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_ENCODING);
    RSA_free(ret);
    return NULL;
  }
  return ret;
}

int RSA_marshal_public_key(CBB *cbb, const RSA *rsa) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, CBS_ASN1_SEQUENCE) ||
      !marshal_integer(&child, rsa->n) ||
      !marshal_integer(&child, rsa->e) ||
      !CBB_flush(cbb)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_ENCODE_ERROR);
    return 0;
  }
  return 1;
}

int RSA_public_key_to_bytes(uint8_t **out_bytes, size_t *out_len,
                            const RSA *rsa) {
  CBB cbb;
  CBB_zero(&cbb);
  if (!CBB_init(&cbb, 0) ||
      !RSA_marshal_public_key(&cbb, rsa) ||
      !CBB_finish(&cbb, out_bytes, out_len)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_ENCODE_ERROR);
    CBB_cleanup(&cbb);
    return 0;
  }
  return 1;
}

// kVersionTwoPrime is the value of the version field for a two-prime
// RSAPrivateKey structure (RFC 8017).
static const uint64_t kVersionTwoPrime = 0;

// Distinguisher for stripped JCA RSA private keys, sets zeroed values to NULL
// because ASN.1 treats absent values as 0, but post-parsing validation logic
// expects absent values to be NULL. Returns 1 if JCA stripped private key, 0
// otherwise.
static void detect_stripped_jca_private_key(RSA *key) {
  if (!BN_is_zero(key->d) && !BN_is_zero(key->n) &&
       BN_is_zero(key->e) && BN_is_zero(key->iqmp) &&
       BN_is_zero(key->p) && BN_is_zero(key->q) &&
       BN_is_zero(key->dmp1) && BN_is_zero(key->dmq1)) {
    BN_free(key->e);
    BN_free(key->p);
    BN_free(key->q);
    BN_free(key->dmp1);
    BN_free(key->dmq1);
    BN_free(key->iqmp);
    key->e = NULL;
    key->p = NULL;
    key->q = NULL;
    key->dmp1 = NULL;
    key->dmq1 = NULL;
    key->iqmp = NULL;
    key->flags |= RSA_FLAG_NO_PUBLIC_EXPONENT;
  }
}

RSA *RSA_parse_private_key(CBS *cbs) {
  RSA *ret = RSA_new();
  if (ret == NULL) {
    return NULL;
  }

  CBS child;
  uint64_t version;
  if (!CBS_get_asn1(cbs, &child, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&child, &version)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_ENCODING);
    goto err;
  }

  if (version != kVersionTwoPrime) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_VERSION);
    goto err;
  }

  if (!parse_integer(&child, &ret->n) ||
      !parse_integer(&child, &ret->e) ||
      !parse_integer(&child, &ret->d) ||
      !parse_integer(&child, &ret->p) ||
      !parse_integer(&child, &ret->q) ||
      !parse_integer(&child, &ret->dmp1) ||
      !parse_integer(&child, &ret->dmq1) ||
      !parse_integer(&child, &ret->iqmp)) {
    goto err;
  }

  if (CBS_len(&child) != 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_ENCODING);
    goto err;
  }

  detect_stripped_jca_private_key(ret);

  if (!RSA_check_key(ret)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_RSA_PARAMETERS);
    goto err;
  }

  return ret;

err:
  RSA_free(ret);
  return NULL;
}

RSA *RSA_private_key_from_bytes(const uint8_t *in, size_t in_len) {
  CBS cbs;
  CBS_init(&cbs, in, in_len);
  RSA *ret = RSA_parse_private_key(&cbs);
  if (ret == NULL || CBS_len(&cbs) != 0) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_BAD_ENCODING);
    RSA_free(ret);
    return NULL;
  }
  return ret;
}

int RSA_marshal_private_key(CBB *cbb, const RSA *rsa) {
  CBB child;
  if (!CBB_add_asn1(cbb, &child, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&child, kVersionTwoPrime) ||
      !marshal_integer(&child, rsa->n) ||
      !marshal_integer(&child, rsa->e) ||
      !marshal_integer(&child, rsa->d) ||
      !marshal_integer(&child, rsa->p) ||
      !marshal_integer(&child, rsa->q) ||
      !marshal_integer(&child, rsa->dmp1) ||
      !marshal_integer(&child, rsa->dmq1) ||
      !marshal_integer(&child, rsa->iqmp) ||
      !CBB_flush(cbb)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_ENCODE_ERROR);
    return 0;
  }
  return 1;
}

int RSA_private_key_to_bytes(uint8_t **out_bytes, size_t *out_len,
                             const RSA *rsa) {
  CBB cbb;
  CBB_zero(&cbb);
  if (!CBB_init(&cbb, 0) ||
      !RSA_marshal_private_key(&cbb, rsa) ||
      !CBB_finish(&cbb, out_bytes, out_len)) {
    OPENSSL_PUT_ERROR(RSA, RSA_R_ENCODE_ERROR);
    CBB_cleanup(&cbb);
    return 0;
  }
  return 1;
}

RSA *d2i_RSAPublicKey(RSA **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  RSA *ret = RSA_parse_public_key(&cbs);
  if (ret == NULL) {
    return NULL;
  }
  if (out != NULL) {
    RSA_free(*out);
    *out = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

int i2d_RSAPublicKey(const RSA *in, uint8_t **outp) {
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||
      !RSA_marshal_public_key(&cbb, in)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}

RSA *d2i_RSAPrivateKey(RSA **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  RSA *ret = RSA_parse_private_key(&cbs);
  if (ret == NULL) {
    return NULL;
  }
  if (out != NULL) {
    RSA_free(*out);
    *out = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

int i2d_RSAPrivateKey(const RSA *in, uint8_t **outp) {
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||
      !RSA_marshal_private_key(&cbb, in)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}

RSA *RSAPublicKey_dup(const RSA *rsa) {
  uint8_t *der;
  size_t der_len;
  if (!RSA_public_key_to_bytes(&der, &der_len, rsa)) {
    return NULL;
  }
  RSA *ret = RSA_public_key_from_bytes(der, der_len);
  OPENSSL_free(der);
  return ret;
}

RSA *RSAPrivateKey_dup(const RSA *rsa) {
  uint8_t *der;
  size_t der_len;
  if (!RSA_private_key_to_bytes(&der, &der_len, rsa)) {
    return NULL;
  }
  RSA *ret = RSA_private_key_from_bytes(der, der_len);
  OPENSSL_free(der);
  return ret;
}
