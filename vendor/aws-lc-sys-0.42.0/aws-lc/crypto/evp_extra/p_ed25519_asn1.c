// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/evp.h>

#include <openssl/bytestring.h>
#include <openssl/curve25519.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../fipsmodule/evp/internal.h"
#include "../internal.h"
#include "internal.h"


static void ed25519_free(EVP_PKEY *pkey) {
  OPENSSL_free(pkey->pkey.ptr);
  pkey->pkey.ptr = NULL;
}

static int ed25519_set_priv_raw(EVP_PKEY *pkey, const uint8_t *privkey,
                                size_t privkey_len, const uint8_t *pubkey,
                                size_t pubkey_len) {
  if (privkey_len != ED25519_PRIVATE_KEY_SEED_LEN) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }

  if(pubkey && pubkey_len != ED25519_PUBLIC_KEY_LEN) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }

  ED25519_KEY *key = OPENSSL_malloc(sizeof(ED25519_KEY));
  if (key == NULL) {
    return 0;
  }

  // The RFC 8032 encoding stores only the 32-byte seed, so we must recover the
  // full representation which we use from it.
  uint8_t pubkey_computed[ED25519_PUBLIC_KEY_LEN];
  ED25519_keypair_from_seed(pubkey_computed, key->key, privkey);
  key->has_private = 1;

  // If a public key was provided, validate that it matches the computed value.
  if(pubkey && OPENSSL_memcmp(pubkey_computed, pubkey, pubkey_len) != 0) {
    OPENSSL_free(key);
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }

  ed25519_free(pkey);
  pkey->pkey.ptr = key;
  return 1;
}

static int ed25519_set_pub_raw(EVP_PKEY *pkey, const uint8_t *in, size_t len) {
  if (len != ED25519_PUBLIC_KEY_LEN) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }

  ED25519_KEY *key = OPENSSL_malloc(sizeof(ED25519_KEY));
  if (key == NULL) {
    return 0;
  }

  OPENSSL_memcpy(key->key + ED25519_PUBLIC_KEY_OFFSET, in, ED25519_PUBLIC_KEY_LEN);
  key->has_private = 0;

  ed25519_free(pkey);
  pkey->pkey.ptr = key;
  return 1;
}

static int ed25519_get_priv_raw(const EVP_PKEY *pkey, uint8_t *out,
                                size_t *out_len) {
  const ED25519_KEY *key = pkey->pkey.ptr;
  if (!key->has_private) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NOT_A_PRIVATE_KEY);
    return 0;
  }

  if (out == NULL) {
    *out_len = ED25519_PRIVATE_KEY_SEED_LEN;
    return 1;
  }

  if (*out_len < 32) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  // The raw private key format is the first 32 bytes of the private key.
  OPENSSL_memcpy(out, key->key, ED25519_PRIVATE_KEY_SEED_LEN);
  *out_len = 32;
  return 1;
}

static int ed25519_get_pub_raw(const EVP_PKEY *pkey, uint8_t *out,
                               size_t *out_len) {
  const ED25519_KEY *key = pkey->pkey.ptr;
  if (out == NULL) {
    *out_len = ED25519_PUBLIC_KEY_LEN;
    return 1;
  }

  if (*out_len < ED25519_PUBLIC_KEY_LEN) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_BUFFER_TOO_SMALL);
    return 0;
  }

  OPENSSL_memcpy(out, key->key + ED25519_PUBLIC_KEY_OFFSET, ED25519_PUBLIC_KEY_LEN);
  *out_len = 32;
  return 1;
}

static int ed25519_pub_decode(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key) {
  // See RFC 8410, section 4.

  // The parameters must be omitted. Public keys have length 32.
  if (CBS_len(params) != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }

  return ed25519_set_pub_raw(out, CBS_data(key), CBS_len(key));
}

static int ed25519_pub_encode(CBB *out, const EVP_PKEY *pkey) {
  const ED25519_KEY *key = pkey->pkey.ptr;

  // See RFC 8410, section 4.
  CBB spki, algorithm, oid, key_bitstring;
  if (!CBB_add_asn1(out, &spki, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&spki, &algorithm, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&algorithm, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, ed25519_asn1_meth.oid, ed25519_asn1_meth.oid_len) ||
      !CBB_add_asn1(&spki, &key_bitstring, CBS_ASN1_BITSTRING) ||
      !CBB_add_u8(&key_bitstring, 0 /* padding */) ||
      !CBB_add_bytes(&key_bitstring, key->key + ED25519_PUBLIC_KEY_OFFSET,
                     32) ||
      !CBB_flush(out)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_ENCODE_ERROR);
    return 0;
  }

  return 1;
}

static int ed25519_pub_cmp(const EVP_PKEY *a, const EVP_PKEY *b) {
  const ED25519_KEY *a_key = a->pkey.ptr;
  const ED25519_KEY *b_key = b->pkey.ptr;
  return OPENSSL_memcmp(a_key->key + ED25519_PUBLIC_KEY_OFFSET,
                        b_key->key + ED25519_PUBLIC_KEY_OFFSET, ED25519_PUBLIC_KEY_LEN) == 0;
}

static int ed25519_priv_decode(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key, CBS *pubkey) {
  // See RFC 8410, section 7.

  // Parameters must be empty. The key is a 32-byte value wrapped in an extra
  // OCTET STRING layer.
  CBS inner;
  if (CBS_len(params) != 0 ||
      !CBS_get_asn1(key, &inner, CBS_ASN1_OCTETSTRING) ||
      CBS_len(key) != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    return 0;
  }

  const uint8_t *public= NULL;
  size_t public_len = 0;
  if (pubkey) {
    // pubkey is encoded as an ASN.1 BIT STRING, so we handle the padding here
    // byte here.
    uint8_t padding;
    if (!CBS_get_u8(pubkey, &padding) || padding != 0) {
      OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
      return 0;
    }
    public = CBS_data(pubkey);
    public_len = CBS_len(pubkey);
  }

  return ed25519_set_priv_raw(out, CBS_data(&inner), CBS_len(&inner), public, public_len);
}

static int ed25519_priv_encode(CBB *out, const EVP_PKEY *pkey) {
  ED25519_KEY *key = pkey->pkey.ptr;
  if (!key->has_private) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NOT_A_PRIVATE_KEY);
    return 0;
  }

  // See RFC 8410, section 7.
  CBB pkcs8, algorithm, oid, private_key, inner;
  if (!CBB_add_asn1(out, &pkcs8, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&pkcs8, PKCS8_VERSION_ONE /* version */) ||
      !CBB_add_asn1(&pkcs8, &algorithm, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&algorithm, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, ed25519_asn1_meth.oid, ed25519_asn1_meth.oid_len) ||
      !CBB_add_asn1(&pkcs8, &private_key, CBS_ASN1_OCTETSTRING) ||
      !CBB_add_asn1(&private_key, &inner, CBS_ASN1_OCTETSTRING) ||
      // The PKCS#8 encoding stores only the 32-byte seed which is the first 32
      // bytes of the private key.
      !CBB_add_bytes(&inner, key->key, 32) ||
      !CBB_flush(out)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_ENCODE_ERROR);
    return 0;
  }

  return 1;
}

static int ed25519_priv_encode_v2(CBB *out, const EVP_PKEY *pkey) {
  ED25519_KEY *key = pkey->pkey.ptr;
  if (!key->has_private) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_NOT_A_PRIVATE_KEY);
    return 0;
  }

  // See RFC 8410, section 7.
  CBB pkcs8, algorithm, oid, private_key, inner, public_key;
  if (!CBB_add_asn1(out, &pkcs8, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&pkcs8, PKCS8_VERSION_TWO /* version */) ||
      !CBB_add_asn1(&pkcs8, &algorithm, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&algorithm, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, ed25519_asn1_meth.oid, ed25519_asn1_meth.oid_len) ||
      !CBB_add_asn1(&pkcs8, &private_key, CBS_ASN1_OCTETSTRING) ||
      !CBB_add_asn1(&private_key, &inner, CBS_ASN1_OCTETSTRING) ||
      // The PKCS#8 encoding stores only the 32-byte seed which is the first 32
      // bytes of the private key.
      !CBB_add_bytes(&inner, key->key, ED25519_PRIVATE_KEY_SEED_LEN) ||
      !CBB_add_asn1(&pkcs8, &public_key, CBS_ASN1_CONTEXT_SPECIFIC | 1) ||
      !CBB_add_u8(&public_key, 0 /*no padding required*/) ||
      // The last 32-bytes of the key is the public key
      !CBB_add_bytes(&public_key, &key->key[32], ED25519_PUBLIC_KEY_LEN) || !CBB_flush(out)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_ENCODE_ERROR);
    return 0;
  }

  return 1;
}

static int ed25519_size(const EVP_PKEY *pkey) { return 64; }

static int ed25519_bits(const EVP_PKEY *pkey) { return 253; }

const EVP_PKEY_ASN1_METHOD ed25519_asn1_meth = {
    EVP_PKEY_ED25519,
    {0x2b, 0x65, 0x70},
    3,
    "ED25519",
    "OpenSSL ED25519 algorithm",
    ed25519_pub_decode,
    ed25519_pub_encode,
    ed25519_pub_cmp,
    ed25519_priv_decode,
    ed25519_priv_encode,
    ed25519_priv_encode_v2,
    ed25519_set_priv_raw,
    ed25519_set_pub_raw,
    ed25519_get_priv_raw,
    ed25519_get_pub_raw,
    NULL /* get_priv_seed */,
    NULL /* pkey_opaque */,
    ed25519_size,
    ed25519_bits,
    NULL /* param_missing */,
    NULL /* param_copy */,
    NULL /* param_cmp */,
    ed25519_free,
};

const EVP_PKEY_ASN1_METHOD ed25519ph_asn1_meth = {
    EVP_PKEY_ED25519PH,
    {0xFF}, /* oid */
    0, /* oid_len */
    "ED25519ph", /* pem_str */
    "OpenSSL ED25519ph algorithm", /* info */
    NULL, /* pub_decode */
    NULL, /* pub_encode */
    NULL, /* pub_cmp */
    NULL, /* priv_decode */
    NULL, /* priv_encode */
    NULL, /* priv_encode_v2 */
    ed25519_set_priv_raw,
    ed25519_set_pub_raw,
    ed25519_get_priv_raw,
    ed25519_get_pub_raw,
    NULL /* get_priv_seed */,
    NULL /* pkey_opaque */,
    ed25519_size,
    ed25519_bits,
    NULL /* param_missing */,
    NULL /* param_copy */,
    NULL /* param_cmp */,
    ed25519_free,
};
