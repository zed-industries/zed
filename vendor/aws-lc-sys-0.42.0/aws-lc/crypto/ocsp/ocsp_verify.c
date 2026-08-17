// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <limits.h>
#include <string.h>
#include "../internal.h"
#include "internal.h"

#define SIGNER_IN_PROVIDED_CERTS 2
#define SIGNER_IN_OCSP_CERTS 1
#define SIGNER_NOT_FOUND 0

// Set up |X509_STORE_CTX| to verify signer and returns cert chain if verify is
// OK. A |OCSP_RESPID| can be identified either by name or its keyhash.
// https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.2.3
static X509 *ocsp_find_signer_sk(STACK_OF(X509) *certs, OCSP_RESPID *id) {
  if (certs == NULL || id == NULL) {
    return NULL;
  }

  // Easy if lookup by name.
  if (id->type == V_OCSP_RESPID_NAME) {
    return X509_find_by_subject(certs, id->value.byName);
  }

  // Lookup by key hash.
  unsigned char tmphash[SHA_DIGEST_LENGTH], *keyhash;

  // If key hash isn't SHA1 length then forget it.
  if (id->value.byKey == NULL || id->value.byKey->length != SHA_DIGEST_LENGTH) {
    return NULL;
  }
  keyhash = id->value.byKey->data;
  // Calculate hash of each key and compare.
  X509 *cert;
  for (size_t i = 0; i < sk_X509_num(certs); i++) {
    cert = sk_X509_value(certs, i);
    if (X509_pubkey_digest(cert, EVP_sha1(), tmphash, NULL)) {
      if (OPENSSL_memcmp(keyhash, tmphash, SHA_DIGEST_LENGTH) == 0) {
        return cert;
      }
    }
  }

  return NULL;
}

// Find signer in cert stack or |OCSP_BASICRESP|'s cert stack.
static int ocsp_find_signer(X509 **psigner, OCSP_BASICRESP *bs,
                            STACK_OF(X509) *certs, unsigned long flags) {
  if (psigner == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  X509 *signer;
  OCSP_RESPID *rid = bs->tbsResponseData->responderId;
  // look for signer in certs stack.
  signer = ocsp_find_signer_sk(certs, rid);
  if (signer != NULL) {
    *psigner = signer;
    return SIGNER_IN_PROVIDED_CERTS;
  }

  // look in certs stack the responder may have included in |OCSP_BASICRESP|,
  // unless the flags contain |OCSP_NOINTERN|.
  signer = ocsp_find_signer_sk(bs->certs, rid);
  if (signer != NULL && !IS_OCSP_FLAG_SET(flags, OCSP_NOINTERN)) {
    *psigner = signer;
    return SIGNER_IN_OCSP_CERTS;
  }
  // Maybe lookup from store if by subject name.

  *psigner = NULL;
  return SIGNER_NOT_FOUND;
}

// check if public key in signer matches key in |OCSP_BASICRESP|.
static int ocsp_verify_key(OCSP_BASICRESP *bs, X509 *signer) {
  if (signer == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  EVP_PKEY *skey = X509_get_pubkey(signer);
  if (skey == NULL) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_NO_SIGNER_KEY);
    return -1;
  }
  int ret =
      ASN1_item_verify(ASN1_ITEM_rptr(OCSP_RESPDATA), bs->signatureAlgorithm,
                       bs->signature, bs->tbsResponseData, skey);
  EVP_PKEY_free(skey);
  if (ret <= 0) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_SIGNATURE_FAILURE);
  }

  return ret;
}

// Set untrusted certificate stack from |OCSP_BASICRESP|.
static int ocsp_setup_untrusted(OCSP_BASICRESP *bs, STACK_OF(X509) *certs,
                                STACK_OF(X509) **untrusted,
                                unsigned long flags) {
  if (untrusted == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }
  if (IS_OCSP_FLAG_SET(flags, OCSP_NOCHAIN)) {
    *untrusted = NULL;
  } else if (bs->certs && certs) {
    *untrusted = sk_X509_dup(bs->certs);
    for (size_t i = 0; i < sk_X509_num(certs); i++) {
      if (!sk_X509_push(*untrusted, sk_X509_value(certs, i))) {
        return -1;
      }
    }
  } else if (certs != NULL) {
    *untrusted = sk_X509_dup(certs);
  } else {
    *untrusted = sk_X509_dup(bs->certs);
  }
  return 1;
}


static int ocsp_verify_signer(X509 *signer, X509_STORE *st,
                              STACK_OF(X509) *untrusted,
                              STACK_OF(X509) **chain) {
  if (signer == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  // Set up |X509_STORE_CTX| with |*signer|, |*st|, and |*untrusted|.
  X509_STORE_CTX *ctx = X509_STORE_CTX_new();
  int ret = -1;

  if (ctx == NULL) {
    goto end;
  }
  if (!X509_STORE_CTX_init(ctx, st, signer, untrusted)) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_X509_LIB);
    goto end;
  }
  // RFC 6960 section 4.2.2.2.1: if the responder certificate has the
  // id-pkix-ocsp-nocheck extension, the CA has indicated that the client
  // should trust the responder for its lifetime without revocation checking.
  // Locally disable CRL-based revocation checking in this case.
  if (X509_get_ext_by_NID(signer, NID_id_pkix_OCSP_noCheck, -1) >= 0) {
    X509_VERIFY_PARAM *vp = X509_STORE_CTX_get0_param(ctx);
    if (vp == NULL) {
      OPENSSL_PUT_ERROR(OCSP, ERR_R_X509_LIB);
      goto end;
    }
    X509_VERIFY_PARAM_clear_flags(vp, X509_V_FLAG_CRL_CHECK);
  }
  if (!X509_STORE_CTX_set_purpose(ctx, X509_PURPOSE_OCSP_HELPER)) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_X509_LIB);
    goto end;
  }

  // Verify |X509_STORE_CTX| and return certificate chain.
  ret = X509_verify_cert(ctx);
  if (ret <= 0) {
    int err = X509_STORE_CTX_get_error(ctx);
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_CERTIFICATE_VERIFY_ERROR);
    ERR_add_error_data(2, "Verify error: ", X509_verify_cert_error_string(err));
    goto end;
  }
  if (chain != NULL) {
    *chain = X509_STORE_CTX_get1_chain(ctx);
  }

end:
  X509_STORE_CTX_free(ctx);
  return ret;
}

// Check the issuer certificate IDs for equality. If the issuer IDs all match
// then we just need to check equality against one of them. If there is a
// mismatch with the same algorithm then there's no point trying to match any
// certificates against the issuer, and |*ret| will be set to NULL.
static int ocsp_check_ids(STACK_OF(OCSP_SINGLERESP) *sresp, OCSP_CERTID **ret) {
  if (sresp == NULL || ret == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  OCSP_CERTID *tmpid, *cid;
  size_t idcount = sk_OCSP_SINGLERESP_num(sresp);
  if (idcount == 0) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_RESPONSE_CONTAINS_NO_REVOCATION_DATA);
    return -1;
  }
  cid = sk_OCSP_SINGLERESP_value(sresp, 0)->certId;

  *ret = NULL;
  for (size_t i = 1; i < idcount; i++) {
    tmpid = sk_OCSP_SINGLERESP_value(sresp, i)->certId;
    // Check to see if IDs match.
    if (OCSP_id_issuer_cmp(cid, tmpid) != 0) {
      // If algorithm mismatch, let caller deal with it instead.
      if (OBJ_cmp(tmpid->hashAlgorithm->algorithm,
                  cid->hashAlgorithm->algorithm) != 0) {
        return 1;
      }
      return 0;
    }
  }

  // All IDs match: only need to check one ID.
  *ret = cid;
  return 1;
}

// Returns -1 on fatal error, 0 if there is no match and 1 if there is a
// match.
static int ocsp_match_issuerid(X509 *cert, OCSP_CERTID *cid,
                               STACK_OF(OCSP_SINGLERESP) *sresp) {
  if (cert == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  // If only one ID to match then do it.
  if (cid) {
    const EVP_MD *dgst;
    X509_NAME *iname;
    unsigned char md[EVP_MAX_MD_SIZE];
    // Set up message digest for comparison.
    dgst = EVP_get_digestbyobj(cid->hashAlgorithm->algorithm);
    if (dgst == NULL) {
      OPENSSL_PUT_ERROR(OCSP, OCSP_R_UNKNOWN_MESSAGE_DIGEST);
      return -1;
    }
    size_t mdlen = EVP_MD_size(dgst);
    iname = X509_get_subject_name(cert);
    if (!X509_NAME_digest(iname, dgst, md, NULL)) {
      return -1;
    }

    // Compare message digest with |OCSP_CERTID|
    if (cid->issuerNameHash->length >= 0 && cid->issuerKeyHash->length >= 0) {
      if (((size_t)cid->issuerNameHash->length != mdlen) ||
          (size_t)cid->issuerKeyHash->length != mdlen) {
        return 0;
      }
    }
    if (0 != OPENSSL_memcmp(md, cid->issuerNameHash->data, mdlen)) {
      return 0;
    }
    if (1 != X509_pubkey_digest(cert, dgst, md, NULL)) {
      return -1;
    }
    if (0 != OPENSSL_memcmp(md, cid->issuerKeyHash->data, mdlen)) {
      return 0;
    }
    return 1;

  }
  // We have to match the whole stack recursively, if |cid| is NULL. This
  // means that the issuer certificate's hash algorithm did not match with
  // the rest of the |certId|s in the |OCSP_SINGLERESP| stack. (Issuer
  // certificate is taken from the root of the |OCSP_SINGLERESP| stack).
  // We'll have to find a match from the signer or responder CA certificate
  // (both are certificates in the certificate chain) in the
  // |OCSP_SINGLERESP| stack instead.
  else {
    int ret;
    OCSP_CERTID *tmpid;
    for (size_t i = 0; i < sk_OCSP_SINGLERESP_num(sresp); i++) {
      tmpid = sk_OCSP_SINGLERESP_value(sresp, i)->certId;
      ret = ocsp_match_issuerid(cert, tmpid, NULL);
      if (ret <= 0) {
        return ret;
      }
    }
    return 1;
  }
}

static int ocsp_check_delegated(X509 *x) {
  if (x == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  if ((X509_get_extension_flags(x) & EXFLAG_XKUSAGE) &&
      (X509_get_extended_key_usage(x) & XKU_OCSP_SIGN)) {
    return 1;
  }
  OPENSSL_PUT_ERROR(OCSP, OCSP_R_MISSING_OCSPSIGNING_USAGE);
  return 0;
}

// check OCSP responder issuer and ids
static int ocsp_check_issuer(OCSP_BASICRESP *bs, STACK_OF(X509) *chain) {
  if (chain == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  STACK_OF(OCSP_SINGLERESP) *sresp = bs->tbsResponseData->responses;
  OCSP_CERTID *caid = NULL;
  int ret;

  if (sk_X509_num(chain) <= 0) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_NO_CERTIFICATES_IN_CHAIN);
    return -1;
  }

  // See if the issuer IDs match.
  ret = ocsp_check_ids(sresp, &caid);

  // If ID mismatch or other error then return.
  if (ret <= 0) {
    return ret;
  }

  X509 *signer, *sca;
  signer = sk_X509_value(chain, 0);
  // Check to see if OCSP responder CA matches request CA.
  if (sk_X509_num(chain) > 1) {
    sca = sk_X509_value(chain, 1);
    ret = ocsp_match_issuerid(sca, caid, sresp);
    if (ret < 0) {
      return ret;
    }
    if (ret != 0) {
      // If matches, then check extension flags.
      if (ocsp_check_delegated(signer)) {
        return 1;
      }
      return 0;
    }
  }

  // Otherwise check if OCSP request signed directly by request CA.
  return ocsp_match_issuerid(signer, caid, sresp);
}

int OCSP_basic_verify(OCSP_BASICRESP *bs, STACK_OF(X509) *certs, X509_STORE *st,
                      unsigned long flags) {
  if (bs == NULL || st == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  if (sk_X509_num(certs) > SHRT_MAX || sk_X509_num(bs->certs) > SHRT_MAX) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_OVERFLOW);
    return -1;
  }

  X509 *signer;
  STACK_OF(X509) *chain = NULL;
  STACK_OF(X509) *untrusted = NULL;

  // Look for signer certificate.
  int ret = ocsp_find_signer(&signer, bs, certs, flags);
  if (ret <= SIGNER_NOT_FOUND) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_SIGNER_CERTIFICATE_NOT_FOUND);
    goto end;
  }
  if ((ret == SIGNER_IN_PROVIDED_CERTS) &&
      IS_OCSP_FLAG_SET(flags, OCSP_TRUSTOTHER)) {
    // We skip verification if the flag to trust |certs| is set and the signer
    // is found within that stack.
    flags |= OCSP_NOVERIFY;
  }

  // Check if public key in signer matches key in |OCSP_BASICRESP|.
  ret = ocsp_verify_key(bs, signer);
  if (ret <= 0) {
    goto end;
  }
  if (!IS_OCSP_FLAG_SET(flags, OCSP_NOVERIFY)) {
    // Verify signer and if valid, check the certificate chain.
    ret = ocsp_setup_untrusted(bs, certs, &untrusted, flags);
    if (ret <= 0) {
      goto end;
    }
    ret = ocsp_verify_signer(signer, st, untrusted, &chain);
    if (ret <= 0) {
      goto end;
    }

    // At this point we have a valid certificate chain, need to verify it
    // against the OCSP issuer criteria.
    ret = ocsp_check_issuer(bs, chain);

    // If a certificate chain is not verifiable against the OCSP issuer
    // criteria, we try to check for explicit trust.
    if (ret == 0) {
      // Easy case: explicitly trusted. Get root CA and check for explicit
      // trust.
      if (IS_OCSP_FLAG_SET(flags, OCSP_NOEXPLICIT)) {
        goto end;
      }
      X509 *root_cert = sk_X509_value(chain, sk_X509_num(chain) - 1);
      if (X509_check_trust(root_cert, NID_OCSP_sign, 0) != X509_TRUST_TRUSTED) {
        OPENSSL_PUT_ERROR(OCSP, OCSP_R_ROOT_CA_NOT_TRUSTED);
        ret = 0;
        goto end;
      }
      ret = 1;
    }
  }

end:
  sk_X509_pop_free(chain, X509_free);
  sk_X509_free(untrusted);
  return ret;
}

// ocsp_req_find_signer assigns |*psigner| to the signing certificate and
// returns |SIGNER_IN_OCSP_CERTS| if it was found within |req| or returns
// |SIGNER_IN_TRUSTED_CERTS| if it was found within |certs|. It returns
// |SIGNER_NOT_FOUND| if the signing certificate is not found.
static int ocsp_req_find_signer(X509 **psigner, OCSP_REQUEST *req,
                                X509_NAME *nm, STACK_OF(X509) *certs,
                                unsigned long flags) {
  X509 *signer = NULL;
  if (!IS_OCSP_FLAG_SET(flags, OCSP_NOINTERN)) {
    signer = X509_find_by_subject(req->optionalSignature->certs, nm);
    if (signer != NULL) {
      *psigner = signer;
      return SIGNER_IN_OCSP_CERTS;
    }
  }

  signer = X509_find_by_subject(certs, nm);
  if (signer != NULL) {
    *psigner = signer;
    return SIGNER_IN_PROVIDED_CERTS;
  }
  return SIGNER_NOT_FOUND;
}

int OCSP_request_verify(OCSP_REQUEST *req, STACK_OF(X509) *certs,
                        X509_STORE *store, unsigned long flags) {
  GUARD_PTR(req);
  GUARD_PTR(req->tbsRequest);
  GUARD_PTR(store);

  // Check if |req| has signature to check against.
  if (req->optionalSignature == NULL) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_REQUEST_NOT_SIGNED);
    return 0;
  }

  GENERAL_NAME *gen = req->tbsRequest->requestorName;
  if (gen == NULL || gen->type != GEN_DIRNAME) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_UNSUPPORTED_REQUESTORNAME_TYPE);
    return 0;
  }

  // Find |signer| from |certs| or |req->optionalSignature->certs| against criteria.
  X509 *signer = NULL;
  int signer_status =
      ocsp_req_find_signer(&signer, req, gen->d.directoryName, certs, flags);
  if (signer_status <= SIGNER_NOT_FOUND || signer == NULL) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_SIGNER_CERTIFICATE_NOT_FOUND);
    return 0;
  }
  if (signer_status == SIGNER_IN_PROVIDED_CERTS &&
      IS_OCSP_FLAG_SET(flags, OCSP_TRUSTOTHER)) {
    // We skip certificate verification if the flag to trust |certs| is set and
    // the signer is found within that stack.
    flags |= OCSP_NOVERIFY;
  }

  // Validate |req|'s signature.
  EVP_PKEY *skey = X509_get0_pubkey(signer);
  if (skey == NULL) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_NO_SIGNER_KEY);
    return 0;
  }
  if (ASN1_item_verify(ASN1_ITEM_rptr(OCSP_REQINFO),
                       req->optionalSignature->signatureAlgorithm,
                       req->optionalSignature->signature, req->tbsRequest,
                       skey) <= 0) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_SIGNATURE_FAILURE);
    return 0;
  }

  // Set up |ctx| and start doing the actual verification.
  X509_STORE_CTX *ctx = X509_STORE_CTX_new();
  if (ctx == NULL) {
    return 0;
  }

  // Validate the signing certificate.
  int ret = 0;
  if (!IS_OCSP_FLAG_SET(flags, OCSP_NOVERIFY)) {
    // Initialize and set purpose of |ctx| for verification.
    if (1 != X509_STORE_CTX_init(ctx, store, signer, NULL) ||
        1 != X509_STORE_CTX_set_purpose(ctx, X509_PURPOSE_OCSP_HELPER)) {
      OPENSSL_PUT_ERROR(OCSP, ERR_R_X509_LIB);
      goto end;
    }
    if (!IS_OCSP_FLAG_SET(flags, OCSP_NOCHAIN)) {
      X509_STORE_CTX_set_chain(ctx, req->optionalSignature->certs);
    }

    // Do the verification.
    if (X509_verify_cert(ctx) <= 0) {
      int err = X509_STORE_CTX_get_error(ctx);
      OPENSSL_PUT_ERROR(OCSP, OCSP_R_CERTIFICATE_VERIFY_ERROR);
      ERR_add_error_data(2,
                         "Verify error:", X509_verify_cert_error_string(err));
      goto end;
    }
  }
  ret = 1;

end:
  X509_STORE_CTX_free(ctx);
  return ret;
}
