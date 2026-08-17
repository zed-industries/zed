// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 2006.
// Copyright (c) 2006 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/ec.h>
#include <openssl/ec_key.h>
#include <openssl/ecdsa.h>
#include <openssl/err.h>

#include "../fipsmodule/evp/internal.h"
#include "../ec_extra/internal.h"
#include "internal.h"


static int eckey_pub_encode(CBB *out, const EVP_PKEY *key) {
  const EC_KEY *ec_key = key->pkey.ec;
  const EC_GROUP *group = EC_KEY_get0_group(ec_key);
  const EC_POINT *public_key = EC_KEY_get0_public_key(ec_key);

  // See RFC 5480, section 2.
  CBB spki, algorithm, oid, key_bitstring;
  if (!CBB_add_asn1(out, &spki, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&spki, &algorithm, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&algorithm, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, ec_asn1_meth.oid, ec_asn1_meth.oid_len) ||
      !EC_KEY_marshal_curve_name(&algorithm, group) ||
      !CBB_add_asn1(&spki, &key_bitstring, CBS_ASN1_BITSTRING) ||
      !CBB_add_u8(&key_bitstring, 0 /* padding */) ||
      !EC_POINT_point2cbb(&key_bitstring, group, public_key,
                          POINT_CONVERSION_UNCOMPRESSED, NULL) ||
      !CBB_flush(out)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_ENCODE_ERROR);
    return 0;
  }

  return 1;
}

static int eckey_pub_decode(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key) {
  // See RFC 5480, section 2.

  // The parameters are a named curve.
  EC_POINT *point = NULL;
  EC_KEY *eckey = NULL;

  enum ECParametersType paramType = UNKNOWN_EC_PARAMETERS;

  const EC_GROUP *group = EC_KEY_parse_parameters_and_type(params, &paramType);
  if (group == NULL || CBS_len(params) != 0 ||
      paramType == UNKNOWN_EC_PARAMETERS) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    goto err;
  }

  eckey = EC_KEY_new();
  if (eckey == NULL || !EC_KEY_set_group(eckey, group)) {
    goto err;
  }

  point = EC_POINT_new(group);
  if (point == NULL ||
      !EC_POINT_oct2point(group, point, CBS_data(key), CBS_len(key), NULL) ||
      !EC_KEY_set_public_key(eckey, point)) {
    goto err;
  }

  if(paramType == SPECIFIED_CURVE_EC_PARAMETERS) {
    eckey->group_decoded_from_explicit_params = 1;
  } else {
    eckey->group_decoded_from_explicit_params = 0;
  }

  EC_POINT_free(point);
  EVP_PKEY_assign_EC_KEY(out, eckey);
  return 1;

err:
  EC_POINT_free(point);
  EC_KEY_free(eckey);
  return 0;
}

static int eckey_pub_cmp(const EVP_PKEY *a, const EVP_PKEY *b) {
  int r;
  const EC_GROUP *group = EC_KEY_get0_group(b->pkey.ec);
  const EC_POINT *pa = EC_KEY_get0_public_key(a->pkey.ec),
                 *pb = EC_KEY_get0_public_key(b->pkey.ec);
  r = EC_POINT_cmp(group, pa, pb, NULL);
  if (r == 0) {
    return 1;
  } else if (r == 1) {
    return 0;
  } else {
    return -2;
  }
}

static int eckey_priv_decode(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key, CBS *pubkey) {
  // See RFC 5915.
  if(pubkey) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }

  enum ECParametersType paramType = UNKNOWN_EC_PARAMETERS;

  const EC_GROUP *group = EC_KEY_parse_parameters_and_type(params, &paramType);
  if (group == NULL || CBS_len(params) != 0 || paramType == UNKNOWN_EC_PARAMETERS) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }

  EC_KEY *ec_key = EC_KEY_parse_private_key(key, group);
  if (ec_key == NULL || CBS_len(key) != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    EC_KEY_free(ec_key);
    return 0;
  }

  if (paramType == SPECIFIED_CURVE_EC_PARAMETERS) {
    ec_key->group_decoded_from_explicit_params = 1;
  } else {
    ec_key->group_decoded_from_explicit_params = 0;
  }

  EVP_PKEY_assign_EC_KEY(out, ec_key);
  return 1;
}

static int eckey_priv_encode(CBB *out, const EVP_PKEY *key) {
  const EC_KEY *ec_key = key->pkey.ec;

  // Omit the redundant copy of the curve name. This contradicts RFC 5915 but
  // aligns with PKCS #11. SEC 1 only says they may be omitted if known by other
  // means. Both OpenSSL and NSS omit the redundant parameters, so we omit them
  // as well.
  unsigned enc_flags = EC_KEY_get_enc_flags(ec_key) | EC_PKEY_NO_PARAMETERS;

  // See RFC 5915.
  CBB pkcs8, algorithm, oid, private_key;
  if (!CBB_add_asn1(out, &pkcs8, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&pkcs8, PKCS8_VERSION_ONE /* version */) ||
      !CBB_add_asn1(&pkcs8, &algorithm, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&algorithm, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, ec_asn1_meth.oid, ec_asn1_meth.oid_len) ||
      !EC_KEY_marshal_curve_name(&algorithm, EC_KEY_get0_group(ec_key)) ||
      !CBB_add_asn1(&pkcs8, &private_key, CBS_ASN1_OCTETSTRING) ||
      !EC_KEY_marshal_private_key(&private_key, ec_key, enc_flags) ||
      !CBB_flush(out)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_ENCODE_ERROR);
    return 0;
  }

  return 1;
}

static int int_ec_size(const EVP_PKEY *pkey) {
  return ECDSA_size(pkey->pkey.ec);
}

static int ec_bits(const EVP_PKEY *pkey) {
  const EC_GROUP *group = EC_KEY_get0_group(pkey->pkey.ec);
  if (group == NULL) {
    ERR_clear_error();
    return 0;
  }
  return EC_GROUP_order_bits(group);
}

static int ec_missing_parameters(const EVP_PKEY *pkey) {
  return pkey->pkey.ec == NULL || EC_KEY_get0_group(pkey->pkey.ec) == NULL;
}

static int ec_copy_parameters(EVP_PKEY *to, const EVP_PKEY *from) {
  if (from->pkey.ec == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }
  const EC_GROUP *group = EC_KEY_get0_group(from->pkey.ec);
  if (group == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_MISSING_PARAMETERS);
    return 0;
  }
  if (to->pkey.ec == NULL) {
    to->pkey.ec = EC_KEY_new();
    if (to->pkey.ec == NULL) {
      return 0;
    }
  }
  return EC_KEY_set_group(to->pkey.ec, group);
}

static int ec_cmp_parameters(const EVP_PKEY *a, const EVP_PKEY *b) {
  if (a->pkey.ec == NULL || b->pkey.ec == NULL) {
    return -2;
  }
  const EC_GROUP *group_a = EC_KEY_get0_group(a->pkey.ec),
                 *group_b = EC_KEY_get0_group(b->pkey.ec);
  if (group_a == NULL || group_b == NULL) {
    return -2;
  }
  if (EC_GROUP_cmp(group_a, group_b, NULL) != 0) {
    // mismatch
    return 0;
  }
  return 1;
}

static void int_ec_free(EVP_PKEY *pkey) { EC_KEY_free(pkey->pkey.ec); }

static int eckey_opaque(const EVP_PKEY *pkey) {
  return EC_KEY_is_opaque(pkey->pkey.ec);
}

const EVP_PKEY_ASN1_METHOD ec_asn1_meth = {
  EVP_PKEY_EC,
  // 1.2.840.10045.2.1
  {0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01}, 7,

  "EC",
  "OpenSSL EC algorithm",

  eckey_pub_decode,
  eckey_pub_encode,
  eckey_pub_cmp,

  eckey_priv_decode,
  eckey_priv_encode,
  NULL /* priv_encode_v2 */,

  NULL /* set_priv_raw */,
  NULL /* set_pub_raw */,
  NULL /* get_priv_raw */,
  NULL /* get_pub_raw */,
  NULL /* get_priv_seed */,

  eckey_opaque,

  int_ec_size,
  ec_bits,

  ec_missing_parameters,
  ec_copy_parameters,
  ec_cmp_parameters,

  int_ec_free,
};
