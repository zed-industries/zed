// Copyright 2006-2021 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <openssl/bn.h>
#include <openssl/dh.h>
#include <openssl/err.h>
#include <openssl/x509.h>

#include "../fipsmodule/cpucap/internal.h"
#include "../fipsmodule/dh/internal.h"
#include "../internal.h"
#include "internal.h"

static int dh_pub_encode(CBB *out, const EVP_PKEY *key) {
  const DH *dh = key->pkey.dh;
  if (dh == NULL || dh->pub_key == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_ENCODE_ERROR);
    return 0;
  }

  CBB spki, algorithm, oid, key_bitstring;
  if (!CBB_add_asn1(out, &spki, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&spki, &algorithm, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&algorithm, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, dh_asn1_meth.oid, dh_asn1_meth.oid_len) ||
      !DH_marshal_parameters(&algorithm, key->pkey.dh) ||
      !CBB_add_asn1(&spki, &key_bitstring, CBS_ASN1_BITSTRING) ||
      !CBB_add_u8(&key_bitstring, 0 /* padding */) ||
      !BN_marshal_asn1(&key_bitstring, key->pkey.dh->pub_key) ||
      !CBB_flush(out)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_ENCODE_ERROR);
    return 0;
  }

  return 1;
}

static int dh_pub_decode(EVP_PKEY *out, CBS *oid, CBS *params, CBS *key) {
  // RFC 2786
  BIGNUM *pubkey = NULL;
  DH *dh = NULL;
  if (out == NULL || params == NULL || CBS_len(params) == 0 || key == NULL ||
      CBS_len(key) == 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    goto err;
  }

  dh = DH_parse_parameters(params);
  if (dh == NULL) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    goto err;
  }

  pubkey = BN_new();
  if (pubkey == NULL || !BN_parse_asn1_unsigned(key, pubkey) || CBS_len(key) != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    goto err;
  }

  int out_flags = 0;
  if (!DH_check_pub_key(dh, pubkey, &out_flags) || out_flags != 0) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    goto err;
  }
  dh->pub_key = pubkey;
  pubkey = NULL;

  if (!EVP_PKEY_assign_DH(out, dh)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_DECODE_ERROR);
    goto err;
  }
  return 1;

err:
  DH_free(dh);
  BN_free(pubkey);
  return 0;
}


static void dh_free(EVP_PKEY *pkey) {
  DH_free(pkey->pkey.dh);
  pkey->pkey.dh = NULL;
}

static int dh_size(const EVP_PKEY *pkey) { return DH_size(pkey->pkey.dh); }

static int dh_bits(const EVP_PKEY *pkey) { return DH_bits(pkey->pkey.dh); }

static int dh_param_missing(const EVP_PKEY *pkey) {
  const DH *dh = pkey->pkey.dh;
  return dh == NULL || DH_get0_p(dh) == NULL || DH_get0_g(dh) == NULL;
}

static int dh_param_copy(EVP_PKEY *to, const EVP_PKEY *from) {
  if (dh_param_missing(from)) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_MISSING_PARAMETERS);
    return 0;
  }

  // Ensure the target has an allocated DH structure.
  if (to->pkey.dh == NULL) {
    to->pkey.dh = DH_new();
    if (to->pkey.dh == NULL) {
      return 0;
    }
  }

  const DH *dh = from->pkey.dh;
  const BIGNUM *q_old = DH_get0_q(dh);
  BIGNUM *p = BN_dup(DH_get0_p(dh));
  BIGNUM *q = q_old == NULL ? NULL : BN_dup(q_old);
  BIGNUM *g = BN_dup(DH_get0_g(dh));
  if (p == NULL || (q_old != NULL && q == NULL) || g == NULL ||
      !DH_set0_pqg(to->pkey.dh, p, q, g)) {
    BN_free(p);
    BN_free(q);
    BN_free(g);
    return 0;
  }

  // |DH_set0_pqg| took ownership of |p|, |q|, and |g|.
  return 1;
}

static int dh_param_cmp(const EVP_PKEY *a, const EVP_PKEY *b) {
  if (dh_param_missing(a) || dh_param_missing(b)) {
    return -2;
  }

  // Matching OpenSSL, only compare p and g for PKCS#3-style Diffie-Hellman.
  // OpenSSL only checks q in X9.42-style Diffie-Hellman ("DHX").
  const DH *a_dh = a->pkey.dh;
  const DH *b_dh = b->pkey.dh;
  return BN_cmp(DH_get0_p(a_dh), DH_get0_p(b_dh)) == 0 &&
         BN_cmp(DH_get0_g(a_dh), DH_get0_g(b_dh)) == 0;
}

static int dh_pub_cmp(const EVP_PKEY *a, const EVP_PKEY *b) {
  if (dh_param_cmp(a, b) <= 0) {
    return 0;
  }

  const DH *a_dh = a->pkey.dh;
  const DH *b_dh = b->pkey.dh;
  return BN_cmp(DH_get0_pub_key(a_dh), DH_get0_pub_key(b_dh)) == 0;
}

const EVP_PKEY_ASN1_METHOD dh_asn1_meth = {
    .pkey_id = EVP_PKEY_DH,
    // 1.2.840.113549.1.3.1
    // ((1)*40 + (2)) = 42 = 0x2a
    // 840 = 0b_0000110_1001000 => 0b_1000_0110_0100_1000 = 0x86 0x48
    // 113549 = 0b_0000110_1110111_0001101 => 0b_1000_0110_1111_0111_0000_1101 = 0x86 0xF7 0x0D
    .oid = {0x2a, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x03, 0x01},
    .oid_len = 9,
    .pem_str = "DH",
    .info = "OpenSSL PKCS#3 DH method",
    .pub_cmp = dh_pub_cmp,
    .pkey_size = dh_size,
    .pkey_bits = dh_bits,
    .param_missing = dh_param_missing,
    .param_copy = dh_param_copy,
    .param_cmp = dh_param_cmp,
    .pkey_free = dh_free,
    .pub_encode = dh_pub_encode,
    .pub_decode = dh_pub_decode,
};

int EVP_PKEY_set1_DH(EVP_PKEY *pkey, DH *key) {
  SET_DIT_AUTO_RESET
  if (EVP_PKEY_assign_DH(pkey, key)) {
    DH_up_ref(key);
    return 1;
  }
  return 0;
}

int EVP_PKEY_assign_DH(EVP_PKEY *pkey, DH *key) {
  SET_DIT_AUTO_RESET
  GUARD_PTR(key);
  evp_pkey_set0(pkey, &dh_asn1_meth, key);
  return 1;
}

DH *EVP_PKEY_get0_DH(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET
  if (pkey->type != EVP_PKEY_DH) {
    OPENSSL_PUT_ERROR(EVP, EVP_R_EXPECTING_A_DH_KEY);
    return NULL;
  }
  return pkey->pkey.dh;
}

DH *EVP_PKEY_get1_DH(const EVP_PKEY *pkey) {
  SET_DIT_AUTO_RESET
  DH *dh = EVP_PKEY_get0_DH(pkey);
  if (dh != NULL) {
    DH_up_ref(dh);
  }
  return dh;
}
