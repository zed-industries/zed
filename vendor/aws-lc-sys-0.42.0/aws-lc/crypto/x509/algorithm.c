// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/x509.h>

#include <openssl/asn1.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/obj.h>

#include "../fipsmodule/pqdsa/internal.h"
#include "../fipsmodule/evp/internal.h"
#include "internal.h"

// Restrict the digests that are allowed in X509 certificates
int x509_digest_nid_ok(const int digest_nid) {
  switch (digest_nid) {
    case NID_md4:
    case NID_md5:
      return 0;
  }
  return 1;
}

int x509_digest_sign_algorithm(EVP_MD_CTX *ctx, X509_ALGOR *algor) {
  EVP_PKEY *pkey = EVP_PKEY_CTX_get0_pkey(ctx->pctx);
  if (pkey == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_CONTEXT_NOT_INITIALISED);
    return 0;
  }

  if (EVP_PKEY_id(pkey) == EVP_PKEY_RSA) {
    int pad_mode;
    if (!EVP_PKEY_CTX_get_rsa_padding(ctx->pctx, &pad_mode)) {
      return 0;
    }
    // RSA-PSS has special signature algorithm logic.
    if (pad_mode == RSA_PKCS1_PSS_PADDING) {
      return x509_rsa_ctx_to_pss(ctx, algor);
    }
  }

  if (EVP_PKEY_id(pkey) == EVP_PKEY_ED25519) {
    return X509_ALGOR_set0(algor, OBJ_nid2obj(NID_ED25519), V_ASN1_UNDEF, NULL);
  }

  if (EVP_PKEY_id(pkey) == EVP_PKEY_PQDSA) {
    return X509_ALGOR_set0(algor, OBJ_nid2obj(pkey->pkey.pqdsa_key->pqdsa->nid), V_ASN1_UNDEF, NULL);
  }

  // Default behavior: look up the OID for the algorithm/hash pair and encode
  // that.
  const EVP_MD *digest = EVP_MD_CTX_md(ctx);
  if (digest == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_CONTEXT_NOT_INITIALISED);
    return 0;
  }

  const int digest_nid = EVP_MD_type(digest);
  int sign_nid;
  if (!x509_digest_nid_ok(digest_nid) ||
      !OBJ_find_sigid_by_algs(&sign_nid, digest_nid, EVP_PKEY_id(pkey))) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_DIGEST_AND_KEY_TYPE_NOT_SUPPORTED);
    return 0;
  }

  // RSA signature algorithms include an explicit NULL parameter. Others omit
  // it.
  int paramtype =
      (EVP_PKEY_id(pkey) == EVP_PKEY_RSA) ? V_ASN1_NULL : V_ASN1_UNDEF;
  return X509_ALGOR_set0(algor, OBJ_nid2obj(sign_nid), paramtype, NULL);
}

int x509_digest_verify_init(EVP_MD_CTX *ctx, const X509_ALGOR *sigalg,
                            EVP_PKEY *pkey) {
  // Convert the signature OID into digest and public key OIDs.
  int sigalg_nid = OBJ_obj2nid(sigalg->algorithm);
  int digest_nid, pkey_nid;
  if (!OBJ_find_sigid_algs(sigalg_nid, &digest_nid, &pkey_nid)) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_UNKNOWN_SIGNATURE_ALGORITHM);
    return 0;
  }

  // Check the public key OID matches the public key type.
  // One special case is |EVP_PKEY_id(pkey)| may be |EVP_PKEY_RSA_PSS|.
  // This case happens when |OBJ_find_sigid_algs| returns |NID_rsaEncryption|
  // when |sigalg_nid| is |NID_rsassaPss|.
  if (pkey_nid != EVP_PKEY_id(pkey) &&
      !(sigalg_nid == NID_rsassaPss && pkey_nid == NID_rsaEncryption &&
        EVP_PKEY_id(pkey) == EVP_PKEY_RSA_PSS) &&
      !(sigalg_nid == NID_MLDSA44 && pkey_nid == NID_MLDSA44 &&
        EVP_PKEY_id(pkey) == EVP_PKEY_PQDSA) &&
      !(sigalg_nid == NID_MLDSA65 && pkey_nid == NID_MLDSA65 &&
        EVP_PKEY_id(pkey) == EVP_PKEY_PQDSA) &&
      !(sigalg_nid == NID_MLDSA87 && pkey_nid == NID_MLDSA87 &&
        EVP_PKEY_id(pkey) == EVP_PKEY_PQDSA)) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_WRONG_PUBLIC_KEY_TYPE);
    return 0;
  }

  // Check for permitted digest algorithms
  if (!x509_digest_nid_ok(digest_nid)) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_DIGEST_AND_KEY_TYPE_NOT_SUPPORTED);
    return 0;
  }

  // NID_undef signals that there are custom parameters to set.
  if (digest_nid == NID_undef) {
    if (sigalg_nid == NID_rsassaPss) {
      return x509_rsa_pss_to_ctx(ctx, sigalg, pkey);
    }

    if (sigalg_nid == NID_ED25519 || sigalg_nid == NID_MLDSA44 ||
        sigalg_nid == NID_MLDSA65 || sigalg_nid == NID_MLDSA87) {
      if (sigalg->parameter != NULL) {
        OPENSSL_PUT_ERROR(X509, X509_R_INVALID_PARAMETER);
        return 0;
      }
      return EVP_DigestVerifyInit(ctx, NULL, NULL, NULL, pkey);
    }
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_UNKNOWN_SIGNATURE_ALGORITHM);
    return 0;
  }

  // The parameter should be an explicit NULL for RSA and omitted for ECDSA. For
  // compatibility, we allow either for both algorithms. See b/167375496.
  //
  // TODO(davidben): Chromium's verifier allows both forms for RSA, but enforces
  // ECDSA more strictly. Align with Chromium and add a flag for b/167375496.
  if (sigalg->parameter != NULL && sigalg->parameter->type != V_ASN1_NULL) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_PARAMETER);
    return 0;
  }

  // Otherwise, initialize with the digest from the OID.
  const EVP_MD *digest = EVP_get_digestbynid(digest_nid);
  if (digest == NULL) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_UNKNOWN_MESSAGE_DIGEST_ALGORITHM);
    return 0;
  }

  return EVP_DigestVerifyInit(ctx, NULL, digest, NULL, pkey);
}
