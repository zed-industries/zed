// Written by Nils Larsch for the OpenSSL project.
// Copyright (c) 2000-2003 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ec.h>

#include <limits.h>
#include <string.h>

#include <openssl/bio.h>
#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>

#include "../bytestring/internal.h"
#include "../fipsmodule/ec/internal.h"
#include "../internal.h"
#include "internal.h"


static const CBS_ASN1_TAG kParametersTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 0;
static const CBS_ASN1_TAG kPublicKeyTag =
    CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 1;

// TODO(https://crbug.com/boringssl/497): Allow parsers to specify a list of
// acceptable groups, so parsers don't have to pull in all four.
typedef const EC_GROUP *(*ec_group_func)(void);
static const ec_group_func kAllGroups[] = {
    &EC_group_p224, &EC_group_p256,      &EC_group_p384,
    &EC_group_p521, &EC_group_secp256k1,
};

EC_KEY *EC_KEY_parse_private_key(CBS *cbs, const EC_GROUP *group) {
  CBS ec_private_key, private_key;
  uint64_t version;
  if (!CBS_get_asn1(cbs, &ec_private_key, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&ec_private_key, &version) || version != 1 ||
      !CBS_get_asn1(&ec_private_key, &private_key, CBS_ASN1_OCTETSTRING)) {
    OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
    return NULL;
  }

  // Parse the optional parameters field.
  EC_KEY *ret = NULL;
  BIGNUM *priv_key = NULL;
  enum ECParametersType paramType = UNKNOWN_EC_PARAMETERS;

  if (CBS_peek_asn1_tag(&ec_private_key, kParametersTag)) {
    // Per SEC 1, as an alternative to omitting it, one is allowed to specify
    // this field and put in a NULL to mean inheriting this value. This was
    // omitted in a previous version of this logic without problems, so leave it
    // unimplemented.
    CBS child;
    if (!CBS_get_asn1(&ec_private_key, &child, kParametersTag)) {
      OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
      goto err;
    }
    const EC_GROUP *inner_group = EC_KEY_parse_parameters_and_type(&child, &paramType);
    if (inner_group == NULL || paramType == UNKNOWN_EC_PARAMETERS) {
      goto err;
    }
    if (group == NULL) {
      group = inner_group;
    } else if (EC_GROUP_cmp(group, inner_group, NULL) != 0) {
      // If a group was supplied externally, it must match.
      OPENSSL_PUT_ERROR(EC, EC_R_GROUP_MISMATCH);
      goto err;
    }
    if (CBS_len(&child) != 0) {
      OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
      goto err;
    }
  }

  if (group == NULL) {
    OPENSSL_PUT_ERROR(EC, EC_R_MISSING_PARAMETERS);
    goto err;
  }

  ret = EC_KEY_new();
  if (ret == NULL || !EC_KEY_set_group(ret, group)) {
    goto err;
  }

  if(paramType == SPECIFIED_CURVE_EC_PARAMETERS) {
    ret->group_decoded_from_explicit_params = 1;
  } else {
    ret->group_decoded_from_explicit_params = 0;
  }

  // Although RFC 5915 specifies the length of the key, OpenSSL historically
  // got this wrong, so accept any length. See upstream's
  // 30cd4ff294252c4b6a4b69cbef6a5b4117705d22.
  priv_key = BN_bin2bn(CBS_data(&private_key), CBS_len(&private_key), NULL);
  ret->pub_key = EC_POINT_new(group);
  if (priv_key == NULL || ret->pub_key == NULL ||
      !EC_KEY_set_private_key(ret, priv_key)) {
    goto err;
  }

  if (CBS_peek_asn1_tag(&ec_private_key, kPublicKeyTag)) {
    CBS child, public_key;
    uint8_t padding;
    if (!CBS_get_asn1(&ec_private_key, &child, kPublicKeyTag) ||
        !CBS_get_asn1(&child, &public_key, CBS_ASN1_BITSTRING) ||
        // As in a SubjectPublicKeyInfo, the byte-encoded public key is then
        // encoded as a BIT STRING with bits ordered as in the DER encoding.
        !CBS_get_u8(&public_key, &padding) || padding != 0 ||
        // Explicitly check |public_key| is non-empty to save the conversion
        // form later.
        CBS_len(&public_key) == 0 ||
        !EC_POINT_oct2point(group, ret->pub_key, CBS_data(&public_key),
                            CBS_len(&public_key), NULL) ||
        CBS_len(&child) != 0) {
      OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
      goto err;
    }

    // Save the point conversion form.
    // TODO(davidben): Consider removing this.
    ret->conv_form =
        (point_conversion_form_t)(CBS_data(&public_key)[0] & ~0x01);
  } else {
    // Compute the public key instead.
    if (!ec_point_mul_scalar_base(group, &ret->pub_key->raw,
                                  &ret->priv_key->scalar)) {
      goto err;
    }
    // Remember the original private-key-only encoding.
    // TODO(davidben): Consider removing this.
    ret->enc_flag |= EC_PKEY_NO_PUBKEY;
  }

  if (CBS_len(&ec_private_key) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
    goto err;
  }

  // Ensure the resulting key is valid.
  if (!EC_KEY_check_key(ret)) {
    goto err;
  }

  BN_free(priv_key);
  return ret;

err:
  EC_KEY_free(ret);
  BN_free(priv_key);
  return NULL;
}

int EC_KEY_marshal_private_key(CBB *cbb, const EC_KEY *key,
                               unsigned enc_flags) {
  if (key == NULL || key->group == NULL || key->priv_key == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  CBB ec_private_key, private_key;
  if (!CBB_add_asn1(cbb, &ec_private_key, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&ec_private_key, 1 /* version */) ||
      !CBB_add_asn1(&ec_private_key, &private_key, CBS_ASN1_OCTETSTRING) ||
      !BN_bn2cbb_padded(&private_key,
                        BN_num_bytes(EC_GROUP_get0_order(key->group)),
                        EC_KEY_get0_private_key(key))) {
    OPENSSL_PUT_ERROR(EC, EC_R_ENCODE_ERROR);
    return 0;
  }

  if (!(enc_flags & EC_PKEY_NO_PARAMETERS)) {
    CBB child;
    if (!CBB_add_asn1(&ec_private_key, &child, kParametersTag) ||
        !EC_KEY_marshal_curve_name(&child, key->group) ||
        !CBB_flush(&ec_private_key)) {
      OPENSSL_PUT_ERROR(EC, EC_R_ENCODE_ERROR);
      return 0;
    }
  }

  // TODO(fork): replace this flexibility with sensible default?
  if (!(enc_flags & EC_PKEY_NO_PUBKEY) && key->pub_key != NULL) {
    CBB child, public_key;
    if (!CBB_add_asn1(&ec_private_key, &child, kPublicKeyTag) ||
        !CBB_add_asn1(&child, &public_key, CBS_ASN1_BITSTRING) ||
        // As in a SubjectPublicKeyInfo, the byte-encoded public key is then
        // encoded as a BIT STRING with bits ordered as in the DER encoding.
        !CBB_add_u8(&public_key, 0 /* padding */) ||
        !EC_POINT_point2cbb(&public_key, key->group, key->pub_key,
                            key->conv_form, NULL) ||
        !CBB_flush(&ec_private_key)) {
      OPENSSL_PUT_ERROR(EC, EC_R_ENCODE_ERROR);
      return 0;
    }
  }

  if (!CBB_flush(cbb)) {
    OPENSSL_PUT_ERROR(EC, EC_R_ENCODE_ERROR);
    return 0;
  }

  return 1;
}

// kPrimeFieldOID is the encoding of 1.2.840.10045.1.1.
static const uint8_t kPrimeField[] = {0x2a, 0x86, 0x48, 0xce, 0x3d, 0x01, 0x01};

struct explicit_prime_curve {
  CBS prime, a, b, base_x, base_y, order;
};

static int parse_explicit_prime_curve(CBS *in,
                                      struct explicit_prime_curve *out) {
  // See RFC 3279, section 2.3.5. Note that RFC 3279 calls this structure an
  // ECParameters while RFC 5480 calls it a SpecifiedECDomain.
  CBS params, field_id, field_type, curve, base, cofactor;
  int has_cofactor;
  uint64_t version;
  if (!CBS_get_asn1(in, &params, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&params, &version) || version != 1 ||
      !CBS_get_asn1(&params, &field_id, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1(&field_id, &field_type, CBS_ASN1_OBJECT) ||
      CBS_len(&field_type) != sizeof(kPrimeField) ||
      OPENSSL_memcmp(CBS_data(&field_type), kPrimeField, sizeof(kPrimeField)) !=
          0 ||
      !CBS_get_asn1(&field_id, &out->prime, CBS_ASN1_INTEGER) ||
      !CBS_is_unsigned_asn1_integer(&out->prime) || CBS_len(&field_id) != 0 ||
      !CBS_get_asn1(&params, &curve, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1(&curve, &out->a, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1(&curve, &out->b, CBS_ASN1_OCTETSTRING) ||
      // |curve| has an optional BIT STRING seed which we ignore.
      !CBS_get_optional_asn1(&curve, NULL, NULL, CBS_ASN1_BITSTRING) ||
      CBS_len(&curve) != 0 ||
      !CBS_get_asn1(&params, &base, CBS_ASN1_OCTETSTRING) ||
      !CBS_get_asn1(&params, &out->order, CBS_ASN1_INTEGER) ||
      !CBS_is_unsigned_asn1_integer(&out->order) ||
      !CBS_get_optional_asn1(&params, &cofactor, &has_cofactor,
                             CBS_ASN1_INTEGER) ||
      CBS_len(&params) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
    return 0;
  }

  if (has_cofactor) {
    // We only support prime-order curves so the cofactor must be one.
    if (CBS_len(&cofactor) != 1 || CBS_data(&cofactor)[0] != 1) {
      OPENSSL_PUT_ERROR(EC, EC_R_UNKNOWN_GROUP);
      return 0;
    }
  }

  // Require that the base point use uncompressed form.
  uint8_t form;
  if (!CBS_get_u8(&base, &form) || form != POINT_CONVERSION_UNCOMPRESSED) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_FORM);
    return 0;
  }

  if (CBS_len(&base) % 2 != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
    return 0;
  }
  size_t field_len = CBS_len(&base) / 2;
  CBS_init(&out->base_x, CBS_data(&base), field_len);
  CBS_init(&out->base_y, CBS_data(&base) + field_len, field_len);

  return 1;
}

// integers_equal returns one if |bytes| is a big-endian encoding of |bn|, and
// zero otherwise.
static int integers_equal(const CBS *bytes, const BIGNUM *bn) {
  // Although, in SEC 1, Field-Element-to-Octet-String has a fixed width,
  // OpenSSL mis-encodes the |a| and |b|, so we tolerate any number of leading
  // zeros. (This matters for P-521 whose |b| has a leading 0.)
  CBS copy = *bytes;
  while (CBS_len(&copy) > 0 && CBS_data(&copy)[0] == 0) {
    CBS_skip(&copy, 1);
  }

  if (CBS_len(&copy) > EC_MAX_BYTES) {
    return 0;
  }
  uint8_t buf[EC_MAX_BYTES];
  if (!BN_bn2bin_padded(buf, CBS_len(&copy), bn)) {
    ERR_clear_error();
    return 0;
  }

  return CBS_mem_equal(&copy, buf, CBS_len(&copy));
}

EC_GROUP *EC_KEY_parse_curve_name(CBS *cbs) {
  CBS named_curve;
  if (!CBS_get_asn1(cbs, &named_curve, CBS_ASN1_OBJECT)) {
    OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
    return NULL;
  }

  // Look for a matching curve.
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kAllGroups); i++) {
    const EC_GROUP *group = kAllGroups[i]();
    if (CBS_mem_equal(&named_curve, group->oid, group->oid_len)) {
      return (EC_GROUP *)group;
    }
  }

  OPENSSL_PUT_ERROR(EC, EC_R_UNKNOWN_GROUP);
  return NULL;
}

int EC_KEY_marshal_curve_name(CBB *cbb, const EC_GROUP *group) {
  if (group->oid_len == 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_UNKNOWN_GROUP);
    return 0;
  }

  CBB child;
  return CBB_add_asn1(cbb, &child, CBS_ASN1_OBJECT) &&
         CBB_add_bytes(&child, group->oid, group->oid_len) &&  //
         CBB_flush(cbb);
}

EC_GROUP *EC_KEY_parse_parameters(CBS *cbs) {
  enum ECParametersType paramType = UNKNOWN_EC_PARAMETERS;
  return EC_KEY_parse_parameters_and_type(cbs, &paramType);
}

EC_GROUP *EC_KEY_parse_parameters_and_type(CBS *cbs, enum ECParametersType *paramType) {
  GUARD_PTR(paramType);

  const EC_GROUP *ret = NULL;
  *paramType = UNKNOWN_EC_PARAMETERS;

  if (!CBS_peek_asn1_tag(cbs, CBS_ASN1_SEQUENCE)) {
    ret = EC_KEY_parse_curve_name(cbs);
    if(ret) {
      *paramType = NAMED_CURVE_EC_PARAMETERS;
    }
    return (EC_GROUP *)ret;
  }

  // OpenSSL sometimes produces ECPrivateKeys or ECPublicKey with explicitly-encoded versions
  // of named curves.
  struct explicit_prime_curve curve;
  if (!parse_explicit_prime_curve(cbs, &curve)) {
    return NULL;
  }

  BIGNUM *p = BN_new(), *a = BN_new(), *b = BN_new(), *x = BN_new(),
         *y = BN_new();
  if (p == NULL || a == NULL || b == NULL || x == NULL || y == NULL) {
    goto err;
  }

  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kAllGroups); i++) {
    const EC_GROUP *group = kAllGroups[i]();
    if (!integers_equal(&curve.order, EC_GROUP_get0_order(group))) {
      continue;
    }

    // The order alone uniquely identifies the group, but we check the other
    // parameters to avoid misinterpreting the group.
    if (!EC_GROUP_get_curve_GFp(group, p, a, b, NULL)) {
      goto err;
    }
    if (!integers_equal(&curve.prime, p) || !integers_equal(&curve.a, a) ||
        !integers_equal(&curve.b, b)) {
      break;
    }
    if (!EC_POINT_get_affine_coordinates_GFp(
            group, EC_GROUP_get0_generator(group), x, y, NULL)) {
      goto err;
    }
    if (!integers_equal(&curve.base_x, x) ||
        !integers_equal(&curve.base_y, y)) {
      break;
    }
    ret = group;
    *paramType = SPECIFIED_CURVE_EC_PARAMETERS;
    break;
  }

  if (ret == NULL) {
    OPENSSL_PUT_ERROR(EC, EC_R_UNKNOWN_GROUP);
  }

err:
  BN_free(p);
  BN_free(a);
  BN_free(b);
  BN_free(x);
  BN_free(y);
  return (EC_GROUP *)ret;
}


int EC_POINT_point2cbb(CBB *out, const EC_GROUP *group, const EC_POINT *point,
                       point_conversion_form_t form, BN_CTX *ctx) {
  size_t len = EC_POINT_point2oct(group, point, form, NULL, 0, ctx);
  if (len == 0) {
    return 0;
  }
  uint8_t *p;
  return CBB_add_space(out, &p, len) &&
         EC_POINT_point2oct(group, point, form, p, len, ctx) == len;
}

EC_KEY *d2i_ECPrivateKey(EC_KEY **out, const uint8_t **inp, long len) {
  // This function treats its |out| parameter differently from other |d2i|
  // functions. If supplied, take the group from |*out|.
  const EC_GROUP *group = NULL;
  if (out != NULL && *out != NULL) {
    group = EC_KEY_get0_group(*out);
  }

  if (len < 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  EC_KEY *ret = EC_KEY_parse_private_key(&cbs, group);
  if (ret == NULL) {
    return NULL;
  }
  if (out != NULL) {
    EC_KEY_free(*out);
    *out = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

int i2d_ECPrivateKey(const EC_KEY *key, uint8_t **outp) {
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||
      !EC_KEY_marshal_private_key(&cbb, key, EC_KEY_get_enc_flags(key))) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}

EC_KEY *d2i_ECParameters(EC_KEY **out_key, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }

  EC_GROUP *group = d2i_ECPKParameters(NULL, inp, len);
  if (group == NULL) {
    return NULL;
  }

  EC_KEY *ret = EC_KEY_new();
  if (ret == NULL || !EC_KEY_set_group(ret, group)) {
    EC_KEY_free(ret);
    return NULL;
  }

  if (out_key != NULL) {
    EC_KEY_free(*out_key);
    *out_key = ret;
  }
  return ret;
}

EC_GROUP *d2i_ECPKParameters(EC_GROUP **out_group, const uint8_t **inp,
                             long len) {
  if (inp == NULL || len < 0) {
    return NULL;
  }

  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  EC_GROUP *group = EC_KEY_parse_parameters(&cbs);
  if (group == NULL) {
    return NULL;
  }

  if (out_group != NULL) {
    EC_GROUP_free(*out_group);
    *out_group = group;
  }
  *inp = CBS_data(&cbs);
  return group;
}

int i2d_ECParameters(const EC_KEY *key, uint8_t **outp) {
  if (key == NULL || key->group == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  return i2d_ECPKParameters(key->group, outp);
}

int i2d_ECPKParameters(const EC_GROUP *group, uint8_t **outp) {
  if (group == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  CBB cbb;
  if (!CBB_init(&cbb, 0) || !EC_KEY_marshal_curve_name(&cbb, group)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}

EC_GROUP *d2i_ECPKParameters_bio(BIO *bio, EC_GROUP **out_group) {
  if (bio == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  uint8_t *data;
  size_t len;
  if (!BIO_read_asn1(bio, &data, &len, INT_MAX)) {
    return NULL;
  }
  const uint8_t *ptr = data;
  EC_GROUP *ret = d2i_ECPKParameters(out_group, &ptr, len);
  OPENSSL_free(data);
  return ret;
}

int i2d_ECPKParameters_bio(BIO *bio, const EC_GROUP *group) {
  if (bio == NULL || group == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  uint8_t *out = NULL;
  int len = i2d_ECPKParameters(group, &out);
  if (out == NULL) {
    return 0;
  }

  int ret = BIO_write_all(bio, out, len);
  OPENSSL_free(out);
  return ret;
}

EC_KEY *o2i_ECPublicKey(EC_KEY **keyp, const uint8_t **inp, long len) {
  EC_KEY *ret = NULL;

  if (keyp == NULL || *keyp == NULL || (*keyp)->group == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }
  ret = *keyp;
  if (ret->pub_key == NULL &&
      (ret->pub_key = EC_POINT_new(ret->group)) == NULL) {
    return NULL;
  }
  if (!EC_POINT_oct2point(ret->group, ret->pub_key, *inp, len, NULL)) {
    OPENSSL_PUT_ERROR(EC, ERR_R_EC_LIB);
    return NULL;
  }
  if (EC_POINT_is_at_infinity(ret->group, ret->pub_key)) {
    OPENSSL_PUT_ERROR(EC, EC_R_POINT_AT_INFINITY);
    return NULL;
  }
  // save the point conversion form
  ret->conv_form = (point_conversion_form_t)(*inp[0] & ~0x01);
  *inp += len;
  return ret;
}

int i2o_ECPublicKey(const EC_KEY *key, uint8_t **outp) {
  if (key == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  CBB cbb;
  if (!CBB_init(&cbb, 0) ||  //
      !EC_POINT_point2cbb(&cbb, key->group, key->pub_key, key->conv_form,
                          NULL)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  int ret = CBB_finish_i2d(&cbb, outp);
  // Historically, this function used the wrong return value on error.
  return ret > 0 ? ret : 0;
}

size_t EC_get_builtin_curves(EC_builtin_curve *out_curves,
                             size_t max_num_curves) {
  if (max_num_curves > OPENSSL_ARRAY_SIZE(kAllGroups)) {
    max_num_curves = OPENSSL_ARRAY_SIZE(kAllGroups);
  }
  for (size_t i = 0; i < max_num_curves; i++) {
    const EC_GROUP *group = kAllGroups[i]();
    out_curves[i].nid = group->curve_name;
    out_curves[i].comment = group->comment;
  }
  return OPENSSL_ARRAY_SIZE(kAllGroups);
}

static size_t EC_POINT_point2buf(const EC_GROUP *group, const EC_POINT *point,
                                 point_conversion_form_t form, uint8_t **pbuf,
                                 BN_CTX *ctx) {
  size_t len;
  uint8_t *buf;

  len = EC_POINT_point2oct(group, point, form, NULL, 0, NULL);
  if (len == 0) {
    return 0;
  }
  buf = OPENSSL_malloc(len);
  if (buf == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ERR_R_MALLOC_FAILURE);
    return 0;
  }
  len = EC_POINT_point2oct(group, point, form, buf, len, ctx);
  if (len == 0) {
    OPENSSL_free(buf);
    return 0;
  }
  *pbuf = buf;
  return len;
}

BIGNUM *EC_POINT_point2bn(const EC_GROUP *group, const EC_POINT *point,
                          point_conversion_form_t form, BIGNUM *ret,
                          BN_CTX *ctx) {
  size_t buf_len = 0;
  uint8_t *buf;

  buf_len = EC_POINT_point2buf(group, point, form, &buf, ctx);

  if (buf_len == 0) {
    return NULL;
  }

  ret = BN_bin2bn(buf, buf_len, ret);

  OPENSSL_free(buf);

  return ret;
}

EC_POINT *EC_POINT_bn2point(const EC_GROUP *group, const BIGNUM *bn,
                            EC_POINT *point, BN_CTX *ctx) {
  if (group == NULL || bn == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  // Allocate buffer and length.
  size_t buf_len = BN_num_bytes(bn);
  if (buf_len == 0) {
    // See https://github.com/openssl/openssl/issues/10258.
    buf_len = 1;
  }
  uint8_t *buf = OPENSSL_malloc(buf_len);
  if (buf == NULL) {
    return NULL;
  }

  if (BN_bn2bin_padded(buf, buf_len, bn) < 0) {
    OPENSSL_free(buf);
    return NULL;
  }

  // Use the user-provided |point| if there is one. Otherwise, we allocate a new
  // |EC_POINT| if |point| is NULL.
  EC_POINT *ret;
  if (point != NULL) {
    ret = point;
  } else {
    ret = EC_POINT_new(group);
    if (ret == NULL) {
      OPENSSL_free(buf);
      return NULL;
    }
  }

  if (!EC_POINT_oct2point(group, ret, buf, buf_len, ctx)) {
    if (ret != point) {
      // If the user did not provide a |point|, we free the |EC_POINT| we
      // allocated.
      EC_POINT_free(ret);
      ret = NULL;
    }
  }

  OPENSSL_free(buf);
  return ret;
}

int ECPKParameters_print(BIO *bio, const EC_GROUP *group, int offset) {
  return 1;
}
