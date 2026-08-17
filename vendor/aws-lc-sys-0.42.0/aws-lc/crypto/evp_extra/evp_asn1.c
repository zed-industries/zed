// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <string.h>

#include <openssl/bytestring.h>
#include <openssl/dh.h>
#include <openssl/dsa.h>
#include <openssl/ec_key.h>
#include <openssl/err.h>
#include <openssl/rsa.h>

#include "../bytestring/internal.h"
#include "../fipsmodule/dh/internal.h"
#include "../fipsmodule/evp/internal.h"
#include "../fipsmodule/pqdsa/internal.h"
#include "../internal.h"
#include "internal.h"
#include "../fipsmodule/kem/internal.h"
#include "../fipsmodule/cpucap/internal.h"

// parse_key_type takes the algorithm cbs sequence |cbs| and extracts the OID.
// The extracted OID will be set on |out_oid| so that it may be used later in
// specific key type implementations like PQDSA.
// The OID is then searched against ASN.1 methods for a method with that OID.
// As the |OID| is read from |cbs| the buffer is advanced.
// For the case of |NID_rsa| or |NID_rsaesOaep| the method |rsa_asn1_meth| is returned.
// For the case of |EVP_PKEY_PQDSA| the method |pqdsa_asn1.meth| is returned.
// For the case of |EVP_PKEY_KEM| the method |kem_asn1.meth| is returned.
static const EVP_PKEY_ASN1_METHOD *parse_key_type(CBS *cbs, CBS *out_oid) {
  CBS oid;
  if (!CBS_get_asn1(cbs, &oid, CBS_ASN1_OBJECT)) {
    return NULL;
  }

  CBS_init(out_oid, CBS_data(&oid), CBS_len(&oid));

  const EVP_PKEY_ASN1_METHOD *const *asn1_methods =
      AWSLC_non_fips_pkey_evp_asn1_methods();
  for (size_t i = 0; i < ASN1_EVP_PKEY_METHODS; i++) {
    const EVP_PKEY_ASN1_METHOD *method = asn1_methods[i];
    if (CBS_len(&oid) == method->oid_len &&
        OPENSSL_memcmp(CBS_data(&oid), method->oid, method->oid_len) == 0) {
      return method;
    }
  }

  // Special logic to handle the rarer |NID_rsa| and |NID_rsaesOaep|.
  // NID_rsa:
  // https://www.itu.int/ITU-T/formal-language/itu-t/x/x509/2008/AlgorithmObjectIdentifiers.html
  // NID_rsaesOaep: underlying key is the same as |NID_rsa|. Used by
  // TPM 1.2 Endorsement Key certificates per TCG Credential Profiles
  // V1.2, section 3.2.7.
  int nid = OBJ_cbs2nid(&oid);
  if (nid == NID_rsa || nid == NID_rsaesOaep) {
    return &rsa_asn1_meth;
  }

  // The pkey_id for the pqdsa_asn1_meth is EVP_PKEY_PQDSA, as this holds all
  // asn1 functions for pqdsa types. However, the incoming CBS has the OID for
  // the specific algorithm. So we must search explicitly for the algorithm.
  const EVP_PKEY_ASN1_METHOD *pqdsa_method = PQDSA_find_asn1_by_nid(nid);
  if (pqdsa_method != NULL) {
    return pqdsa_method;
  }
  return KEM_find_asn1_by_nid(nid);
}

EVP_PKEY *EVP_parse_public_key(CBS *cbs) {
  // Parse the SubjectPublicKeyInfo.
  CBS spki, algorithm, key;
  uint8_t padding;
  if (!CBS_get_asn1(cbs, &spki, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1(&spki, &algorithm, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1(&spki, &key, CBS_ASN1_BITSTRING) ||
      CBS_len(&spki) != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return NULL;
  }

  CBS oid;

  const EVP_PKEY_ASN1_METHOD *method = parse_key_type(&algorithm, &oid);
  if (method == NULL || method->pub_decode == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    return NULL;
  }
  // Every key type defined encodes the key as a byte string with the same
  // conversion to BIT STRING, so perform that common conversion ahead of time.
  if (!CBS_get_u8(&key, &padding) ||
      padding != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return NULL;
  }

  EVP_PKEY *ret = EVP_PKEY_new();
  if (ret == NULL) {
    goto err;
  }
  evp_pkey_set0(ret, method, NULL);

  if (!method->pub_decode(ret, &oid, &algorithm, &key)) {
    goto err;
  }

  return ret;

err:
  EVP_PKEY_free(ret);
  return NULL;
}

int EVP_marshal_public_key(CBB *cbb, const EVP_PKEY *key) {
  GUARD_PTR(cbb);
  GUARD_PTR(key);
  if (key->ameth == NULL || key->ameth->pub_encode == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    return 0;
  }

  return key->ameth->pub_encode(cbb, key);
}

static const unsigned kAttributesTag =
    CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0;

static const unsigned kPublicKeyTag =
    CBS_ASN1_CONTEXT_SPECIFIC | 1;

EVP_PKEY *EVP_parse_private_key(CBS *cbs) {
  // Parse the PrivateKeyInfo (RFC 5208) or OneAsymmetricKey (RFC 5958).
  CBS pkcs8, algorithm, key, public_key;
  uint64_t version;
  if (!CBS_get_asn1(cbs, &pkcs8, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&pkcs8, &version) ||
      version > PKCS8_VERSION_TWO ||
      !CBS_get_asn1(&pkcs8, &algorithm, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1(&pkcs8, &key, CBS_ASN1_OCTETSTRING)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return NULL;
  }

  CBS oid;

  const EVP_PKEY_ASN1_METHOD *method = parse_key_type(&algorithm, &oid);
  if (method == NULL || method->priv_decode == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    return NULL;
  }

  // A PrivateKeyInfo & OneAsymmetricKey may optionally contain a SET of Attributes which
  // we ignore.
  if (CBS_peek_asn1_tag(&pkcs8, kAttributesTag)) {
    if (!CBS_get_asn1(&pkcs8, NULL, kAttributesTag)) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
      return NULL;
    }
  }

  int has_pub = 0;
  // A OneAsymmetricKey may contain an optional PublicKey BIT STRING which is
  // implicitly encoded. To support public keys that might not be a size
  // divisible by 8 we leave the first octet of the bit string present, which
  // specifies the padded bit count between 0 and 7.
  if (CBS_peek_asn1_tag(&pkcs8, kPublicKeyTag)) {
    if (version != PKCS8_VERSION_TWO || !CBS_get_asn1(&pkcs8, &public_key, kPublicKeyTag)) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
      return NULL;
    }
    has_pub = 1;
  }

  // Reject trailing data within the SEQUENCE.
  if (CBS_len(&pkcs8) != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return NULL;
  }

  // Set up an |EVP_PKEY| of the appropriate type.
  EVP_PKEY *ret = EVP_PKEY_new();
  if (ret == NULL) {
    goto err;
  }
  evp_pkey_set0(ret, method, NULL);

  if (!method->priv_decode(ret, &oid, &algorithm, &key,
                           has_pub ? &public_key : NULL)) {
    goto err;
  }

  return ret;

err:
  EVP_PKEY_free(ret);
  return NULL;
}

int EVP_marshal_private_key(CBB *cbb, const EVP_PKEY *key) {
  if (key->ameth == NULL || key->ameth->priv_encode == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    return 0;
  }

  return key->ameth->priv_encode(cbb, key);
}

int EVP_marshal_private_key_v2(CBB *cbb, const EVP_PKEY *key) {
  if (key->ameth == NULL || key->ameth->priv_encode_v2 == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_ALGORITHM);
    return 0;
  }

  return key->ameth->priv_encode_v2(cbb, key);
}

static EVP_PKEY *old_priv_decode(CBS *cbs, int type) {
  EVP_PKEY *ret = EVP_PKEY_new();
  if (ret == NULL) {
    return NULL;
  }

  switch (type) {
    case EVP_PKEY_EC: {
      EC_KEY *ec_key = EC_KEY_parse_private_key(cbs, NULL);
      if (ec_key == NULL || !EVP_PKEY_assign_EC_KEY(ret, ec_key)) {
        EC_KEY_free(ec_key);
        goto err;
      }
      return ret;
    }
    case EVP_PKEY_DSA: {
      DSA *dsa = DSA_parse_private_key(cbs);
      if (dsa == NULL || !EVP_PKEY_assign_DSA(ret, dsa)) {
        DSA_free(dsa);
        goto err;
      }
      return ret;
    }
    case EVP_PKEY_RSA: {
      RSA *rsa = RSA_parse_private_key(cbs);
      if (rsa == NULL || !EVP_PKEY_assign_RSA(ret, rsa)) {
        RSA_free(rsa);
        goto err;
      }
      return ret;
    }
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_UNKNOWN_PUBLIC_KEY_TYPE);
      goto err;
  }

err:
  EVP_PKEY_free(ret);
  return NULL;
}

int EVP_PKEY_check(EVP_PKEY_CTX *ctx) {
  if (ctx == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  EVP_PKEY *pkey = ctx->pkey;

  if (pkey == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }

  switch (pkey->type) {
    case EVP_PKEY_EC: {
      EC_KEY *ec = pkey->pkey.ec;
      // For EVP_PKEY_check, ensure the private key exists for EC keys
      if (EC_KEY_get0_private_key(ec) == NULL) {
        OPENSSL_PUT_ERROR(EVP, EC_R_MISSING_PRIVATE_KEY);
        return 0;
      }
      return EC_KEY_check_key(ec);
    }
    case EVP_PKEY_RSA:
      return RSA_check_key(pkey->pkey.rsa);
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
}

int EVP_PKEY_public_check(EVP_PKEY_CTX *ctx) {
  if (ctx == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  EVP_PKEY *pkey = ctx->pkey;

  if (pkey == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }
  switch (pkey->type) {
    case EVP_PKEY_EC:
      return EC_KEY_check_key(pkey->pkey.ec);
    case EVP_PKEY_RSA:
      return RSA_check_key(pkey->pkey.rsa);
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
}

int EVP_PKEY_param_check(EVP_PKEY_CTX *ctx) {
  if (ctx == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  EVP_PKEY *pkey = ctx->pkey;
  if (pkey == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NO_KEY_SET);
    return 0;
  }

  int err_flags = 0;
  switch (pkey->type) {
    case EVP_PKEY_DH:
      return DH_check(pkey->pkey.dh, &err_flags) && err_flags == 0;
    default:
      OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }
}

EVP_PKEY *d2i_PrivateKey(int type, EVP_PKEY **out, const uint8_t **inp,
                         long len) {
  if (len < 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return NULL;
  }

  // Parse with the legacy format.
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  EVP_PKEY *ret = old_priv_decode(&cbs, type);
  if (ret == NULL) {
    // Try again with PKCS#8.
    ERR_clear_error();
    CBS_init(&cbs, *inp, (size_t)len);
    ret = EVP_parse_private_key(&cbs);
    if (ret == NULL) {
      return NULL;
    }
    if (ret->type != type) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_DIFFERENT_KEY_TYPES);
      EVP_PKEY_free(ret);
      return NULL;
    }
  }

  if (out != NULL) {
    EVP_PKEY_free(*out);
    *out = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

// num_elements parses one SEQUENCE from |in| and returns the number of elements
// in it. On parse error, it returns zero.
static size_t num_elements(const uint8_t *in, size_t in_len) {
  CBS cbs, sequence;
  CBS_init(&cbs, in, (size_t)in_len);

  if (!CBS_get_asn1(&cbs, &sequence, CBS_ASN1_SEQUENCE)) {
    return 0;
  }

  size_t count = 0;
  while (CBS_len(&sequence) > 0) {
    if (!CBS_get_any_asn1_element(&sequence, NULL, NULL, NULL)) {
      return 0;
    }

    count++;
  }

  return count;
}

EVP_PKEY *d2i_AutoPrivateKey(EVP_PKEY **out, const uint8_t **inp, long len) {
  if (len < 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return NULL;
  }

  // Parse the input as a PKCS#8 PrivateKeyInfo.
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  EVP_PKEY *ret = EVP_parse_private_key(&cbs);
  if (ret != NULL) {
    if (out != NULL) {
      EVP_PKEY_free(*out);
      *out = ret;
    }
    *inp = CBS_data(&cbs);
    return ret;
  }
  ERR_clear_error();

  // Count the elements to determine the legacy key format.
  switch (num_elements(*inp, (size_t)len)) {
    case 4:
      return d2i_PrivateKey(EVP_PKEY_EC, out, inp, len);

    case 6:
      return d2i_PrivateKey(EVP_PKEY_DSA, out, inp, len);

    default:
      return d2i_PrivateKey(EVP_PKEY_RSA, out, inp, len);
  }
}

int i2d_PublicKey(const EVP_PKEY *key, uint8_t **outp) {
  switch (key->type) {
    case EVP_PKEY_RSA:
      return i2d_RSAPublicKey(key->pkey.rsa, outp);
    case EVP_PKEY_DSA:
      return i2d_DSAPublicKey(key->pkey.dsa, outp);
    case EVP_PKEY_EC:
      return i2o_ECPublicKey(key->pkey.ec, outp);
    default: {
      // Fall back to SubjectPublicKeyInfo for key types without legacy
      // formats (e.g. Ed25519).
      CBB cbb;
      if (!CBB_init(&cbb, 128) ||
          !EVP_marshal_public_key(&cbb, key)) {
        CBB_cleanup(&cbb);
        OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE);
        return -1;
      }
      return CBB_finish_i2d(&cbb, outp);
    }
  }
}

EVP_PKEY *d2i_PublicKey(int type, EVP_PKEY **out, const uint8_t **inp,
                        long len) {
  EVP_PKEY *ret = EVP_PKEY_new();
  if (ret == NULL) {
    return NULL;
  }

  CBS cbs;
  CBS_init(&cbs, *inp, len < 0 ? 0 : (size_t)len);
  switch (type) {
    case EVP_PKEY_RSA: {
      RSA *rsa = RSA_parse_public_key(&cbs);
      if (rsa == NULL || !EVP_PKEY_assign_RSA(ret, rsa)) {
        RSA_free(rsa);
        goto err;
      }
      break;
    }

    // Unlike OpenSSL, we do not support EC keys with this API. The raw EC
    // public key serialization requires knowing the group. In OpenSSL, calling
    // this function with |EVP_PKEY_EC| and setting |out| to NULL does not work.
    // It requires |*out| to include a partially-initialized |EVP_PKEY| to
    // extract the group.
    default: {
      // Fall back to SubjectPublicKeyInfo for key types without legacy
      // formats (e.g. Ed25519).
      EVP_PKEY_free(ret);
      ret = NULL;
      ERR_clear_error();
      ret = EVP_parse_public_key(&cbs);
      if (ret == NULL) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE);
        goto err;
      }
      if (ret->type != type) {
        OPENSSL_PUT_ERROR(EVP, EVP_R_DIFFERENT_KEY_TYPES);
        goto err;
      }
      break;
    }
  }

  *inp = CBS_data(&cbs);
  if (out != NULL) {
    EVP_PKEY_free(*out);
    *out = ret;
  }
  return ret;

err:
  EVP_PKEY_free(ret);
  return NULL;
}

EVP_PKEY *d2i_PUBKEY(EVP_PKEY **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  EVP_PKEY *ret = EVP_parse_public_key(&cbs);
  if (ret == NULL) {
    return NULL;
  }
  if (out != NULL) {
    EVP_PKEY_free(*out);
    *out = ret;
  }
  *inp = CBS_data(&cbs);
  return ret;
}

int i2d_PUBKEY(const EVP_PKEY *pkey, uint8_t **outp) {
  if (pkey == NULL) {
    return 0;
  }

  CBB cbb;
  if (!CBB_init(&cbb, 128) ||
      !EVP_marshal_public_key(&cbb, pkey)) {
    CBB_cleanup(&cbb);
    return -1;
  }
  return CBB_finish_i2d(&cbb, outp);
}

RSA *d2i_RSA_PUBKEY(RSA **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  EVP_PKEY *pkey = EVP_parse_public_key(&cbs);
  if (pkey == NULL) {
    return NULL;
  }
  RSA *rsa = EVP_PKEY_get1_RSA(pkey);
  EVP_PKEY_free(pkey);
  if (rsa == NULL) {
    return NULL;
  }
  if (out != NULL) {
    RSA_free(*out);
    *out = rsa;
  }
  *inp = CBS_data(&cbs);
  return rsa;
}

int i2d_RSA_PUBKEY(const RSA *rsa, uint8_t **outp) {
  if (rsa == NULL) {
    return 0;
  }

  int ret = -1;
  EVP_PKEY *pkey = EVP_PKEY_new();
  if (pkey == NULL ||
      !EVP_PKEY_set1_RSA(pkey, (RSA *)rsa)) {
    goto err;
  }

  ret = i2d_PUBKEY(pkey, outp);

err:
  EVP_PKEY_free(pkey);
  return ret;
}

DSA *d2i_DSA_PUBKEY(DSA **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  EVP_PKEY *pkey = EVP_parse_public_key(&cbs);
  if (pkey == NULL) {
    return NULL;
  }
  DSA *dsa = EVP_PKEY_get1_DSA(pkey);
  EVP_PKEY_free(pkey);
  if (dsa == NULL) {
    return NULL;
  }
  if (out != NULL) {
    DSA_free(*out);
    *out = dsa;
  }
  *inp = CBS_data(&cbs);
  return dsa;
}

int i2d_DSA_PUBKEY(const DSA *dsa, uint8_t **outp) {
  if (dsa == NULL) {
    return 0;
  }

  int ret = -1;
  EVP_PKEY *pkey = EVP_PKEY_new();
  if (pkey == NULL ||
      !EVP_PKEY_set1_DSA(pkey, (DSA *)dsa)) {
    goto err;
  }

  ret = i2d_PUBKEY(pkey, outp);

err:
  EVP_PKEY_free(pkey);
  return ret;
}

EC_KEY *d2i_EC_PUBKEY(EC_KEY **out, const uint8_t **inp, long len) {
  if (len < 0) {
    return NULL;
  }
  CBS cbs;
  CBS_init(&cbs, *inp, (size_t)len);
  EVP_PKEY *pkey = EVP_parse_public_key(&cbs);
  if (pkey == NULL) {
    return NULL;
  }
  EC_KEY *ec_key = EVP_PKEY_get1_EC_KEY(pkey);
  EVP_PKEY_free(pkey);
  if (ec_key == NULL) {
    return NULL;
  }
  if (out != NULL) {
    EC_KEY_free(*out);
    *out = ec_key;
  }
  *inp = CBS_data(&cbs);
  return ec_key;
}

int i2d_EC_PUBKEY(const EC_KEY *ec_key, uint8_t **outp) {
  if (ec_key == NULL) {
    return 0;
  }

  int ret = -1;
  EVP_PKEY *pkey = EVP_PKEY_new();
  if (pkey == NULL ||
      !EVP_PKEY_set1_EC_KEY(pkey, (EC_KEY *)ec_key)) {
    goto err;
  }

  ret = i2d_PUBKEY(pkey, outp);

err:
  EVP_PKEY_free(pkey);
  return ret;
}

int EVP_PKEY_asn1_get_count(void) { return asn1_evp_pkey_methods_size; }

const EVP_PKEY_ASN1_METHOD *EVP_PKEY_asn1_get0(int idx) {
  if (idx < 0 || idx >= EVP_PKEY_asn1_get_count()) {
    return NULL;
  }
  return asn1_evp_pkey_methods[idx];
}

const EVP_PKEY_ASN1_METHOD *EVP_PKEY_asn1_find(ENGINE **_pe, int type) {
  for (size_t i = 0; i < (size_t)EVP_PKEY_asn1_get_count(); i++) {
    const EVP_PKEY_ASN1_METHOD *ameth = EVP_PKEY_asn1_get0(i);
    if (ameth->pkey_id == type) {
      return ameth;
    }
  }
  return NULL;
}

const EVP_PKEY_ASN1_METHOD *EVP_PKEY_asn1_find_str(ENGINE **_pe,
                                                   const char *name, int len) {
  if (len < 0) {
    return NULL;
  }
  // OPENSSL_strnlen returns an i, where str[i] == 0
  const size_t name_len = OPENSSL_strnlen(name, len);

  for (size_t i = 0; i < (size_t)EVP_PKEY_asn1_get_count(); i++) {
    const EVP_PKEY_ASN1_METHOD *ameth = EVP_PKEY_asn1_get0(i);

    const size_t pem_str_len =
        OPENSSL_strnlen(ameth->pem_str, MAX_PEM_STR_LEN);

    // OPENSSL_strncasecmp(a, b, n) compares up to index n-1
    if (name_len != pem_str_len) {
      continue;
    }
    if (0 == OPENSSL_strncasecmp(ameth->pem_str, name, name_len)) {
      return ameth;
    }
  }
  return NULL;
}

int EVP_PKEY_asn1_get0_info(int *ppkey_id, int *pkey_base_id, int *ppkey_flags,
                            const char **pinfo, const char **ppem_str,
                            const EVP_PKEY_ASN1_METHOD *ameth) {
  if (!ameth) {
    return 0;
  }
  if (ppkey_id) {
    *ppkey_id = ameth->pkey_id;
  }
  if (pkey_base_id) {
    *pkey_base_id = ameth->pkey_id;
  }
  // This value is not supported.
  if (ppkey_flags) {
    *ppkey_flags = 0;
  }
  if (pinfo) {
    *pinfo = ameth->info;
  }
  if (ppem_str) {
    *ppem_str = ameth->pem_str;
  }
  return 1;
}

int EVP_PKEY_get_private_seed(const EVP_PKEY *key, uint8_t *out,
  size_t *out_len) {
  SET_DIT_AUTO_RESET;
  GUARD_PTR(key);
  GUARD_PTR(out_len);

  if (key->ameth == NULL || key->ameth->get_priv_seed == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE);
    return 0;
  }

  return key->ameth->get_priv_seed(key, out, out_len);
}
