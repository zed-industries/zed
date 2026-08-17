// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/obj.h>

#include "../internal.h"


typedef struct {
  int sign_nid;
  int digest_nid;
  int pkey_nid;
} nid_triple;

static const nid_triple kTriples[] = {
    // RSA PKCS#1.
    {NID_md4WithRSAEncryption, NID_md4, NID_rsaEncryption},
    {NID_md5WithRSAEncryption, NID_md5, NID_rsaEncryption},
    {NID_sha1WithRSAEncryption, NID_sha1, NID_rsaEncryption},
    {NID_sha224WithRSAEncryption, NID_sha224, NID_rsaEncryption},
    {NID_sha256WithRSAEncryption, NID_sha256, NID_rsaEncryption},
    {NID_sha384WithRSAEncryption, NID_sha384, NID_rsaEncryption},
    {NID_sha512WithRSAEncryption, NID_sha512, NID_rsaEncryption},
    // RSA ITU-T X.509. These are rare and we map them to the more
    // common |NID_rsaEncryption| instead for simplicity.
    {NID_md5WithRSA, NID_md5, NID_rsaEncryption},
    {NID_sha1WithRSA, NID_sha1, NID_rsaEncryption},
    // DSA.
    {NID_dsaWithSHA1, NID_sha1, NID_dsa},
    {NID_dsaWithSHA1_2, NID_sha1, NID_dsa_2},
    {NID_dsa_with_SHA224, NID_sha224, NID_dsa},
    {NID_dsa_with_SHA256, NID_sha256, NID_dsa},
    // ECDSA.
    {NID_ecdsa_with_SHA1, NID_sha1, NID_X9_62_id_ecPublicKey},
    {NID_ecdsa_with_SHA224, NID_sha224, NID_X9_62_id_ecPublicKey},
    {NID_ecdsa_with_SHA256, NID_sha256, NID_X9_62_id_ecPublicKey},
    {NID_ecdsa_with_SHA384, NID_sha384, NID_X9_62_id_ecPublicKey},
    {NID_ecdsa_with_SHA512, NID_sha512, NID_X9_62_id_ecPublicKey},
    // The following algorithms use more complex (or simpler) parameters. The
    // digest "undef" indicates the caller should handle this explicitly.
    {NID_rsassaPss, NID_undef, NID_rsaEncryption},
    {NID_ED25519, NID_undef, NID_ED25519},
    {NID_MLDSA44, NID_undef, NID_MLDSA44},
    {NID_MLDSA65, NID_undef, NID_MLDSA65},
    {NID_MLDSA87, NID_undef, NID_MLDSA87},
};

int OBJ_find_sigid_algs(int sign_nid, int *out_digest_nid, int *out_pkey_nid) {
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kTriples); i++) {
    if (kTriples[i].sign_nid == sign_nid) {
      if (out_digest_nid != NULL) {
        *out_digest_nid = kTriples[i].digest_nid;
      }
      if (out_pkey_nid != NULL) {
        *out_pkey_nid = kTriples[i].pkey_nid;
      }
      return 1;
    }
  }

  return 0;
}

int OBJ_find_sigid_by_algs(int *out_sign_nid, int digest_nid, int pkey_nid) {
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kTriples); i++) {
    if (kTriples[i].digest_nid == digest_nid &&
        kTriples[i].pkey_nid == pkey_nid) {
      if (out_sign_nid != NULL) {
        *out_sign_nid = kTriples[i].sign_nid;
      }
      return 1;
    }
  }

  return 0;
}
