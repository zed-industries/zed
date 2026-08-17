// Copyright 2001-2017 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright Amazon.com, Inc. or its affiliates.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include "../internal.h"
#include "internal.h"

int OCSP_request_onereq_count(OCSP_REQUEST *req) {
  GUARD_PTR(req);
  GUARD_PTR(req->tbsRequest);
  return (int)sk_OCSP_ONEREQ_num(req->tbsRequest->requestList);
}

OCSP_ONEREQ *OCSP_request_onereq_get0(OCSP_REQUEST *req, int i) {
  GUARD_PTR(req);
  GUARD_PTR(req->tbsRequest);
  return sk_OCSP_ONEREQ_value(req->tbsRequest->requestList, i);
}

int OCSP_id_get0_info(ASN1_OCTET_STRING **nameHash, ASN1_OBJECT **algor,
                      ASN1_OCTET_STRING **keyHash, ASN1_INTEGER **serial,
                      OCSP_CERTID *cid) {
  if (cid == NULL) {
    return 0;
  }
  if (algor != NULL) {
    *algor = cid->hashAlgorithm->algorithm;
  }
  if (nameHash != NULL) {
    *nameHash = cid->issuerNameHash;
  }
  if (keyHash != NULL) {
    *keyHash = cid->issuerKeyHash;
  }
  if (serial != NULL) {
    *serial = cid->serialNumber;
  }
  return 1;
}

int OCSP_request_is_signed(OCSP_REQUEST *req) {
  GUARD_PTR(req);
  if (req->optionalSignature != NULL) {
    return 1;
  }
  return 0;
}

OCSP_CERTID *OCSP_onereq_get0_id(OCSP_ONEREQ *one) {
  GUARD_PTR(one);
  return one->reqCert;
}

int OCSP_basic_add1_cert(OCSP_BASICRESP *resp, X509 *cert) {
  if (resp->certs == NULL && (resp->certs = sk_X509_new_null()) == NULL) {
    return 0;
  }

  if (!sk_X509_push(resp->certs, cert)) {
    return 0;
  }
  X509_up_ref(cert);
  return 1;
}

// ocsp_respid_clear frees any previously-installed content of |respid| using
// the destructor matching the current |respid->type|, so callers can install
// a new value without leaking or triggering a type-confused free.
static void ocsp_respid_clear(OCSP_RESPID *respid) {
  switch (respid->type) {
    case V_OCSP_RESPID_NAME:
      X509_NAME_free(respid->value.byName);
      respid->value.byName = NULL;
      break;
    case V_OCSP_RESPID_KEY:
      ASN1_OCTET_STRING_free(respid->value.byKey);
      respid->value.byKey = NULL;
      break;
  }
}

static int OCSP_RESPID_set_by_name(OCSP_RESPID *respid, X509 *cert) {
  GUARD_PTR(respid);
  GUARD_PTR(cert);

  X509_NAME *byName = X509_NAME_dup(X509_get_subject_name(cert));
  if (byName == NULL) {
    return 0;
  }
  ocsp_respid_clear(respid);
  respid->type = V_OCSP_RESPID_NAME;
  respid->value.byName = byName;
  return 1;
}

static int OCSP_RESPID_set_by_key(OCSP_RESPID *respid, X509 *cert) {
  GUARD_PTR(respid);
  GUARD_PTR(cert);

  // RFC2560 requires SHA1.
  unsigned char digest[SHA_DIGEST_LENGTH];
  if (!X509_pubkey_digest(cert, EVP_sha1(), digest, NULL)) {
    return 0;
  }

  ASN1_OCTET_STRING *byKey = ASN1_OCTET_STRING_new();
  if (byKey == NULL) {
    return 0;
  }

  if (!ASN1_OCTET_STRING_set(byKey, digest, SHA_DIGEST_LENGTH)) {
    ASN1_OCTET_STRING_free(byKey);
    return 0;
  }
  ocsp_respid_clear(respid);
  respid->type = V_OCSP_RESPID_KEY;
  respid->value.byKey = byKey;

  return 1;
}


// OCSP_basic_sign_ctx does the actual signing operation for |OCSP_basic_sign|,
// but with an initialized |EVP_MD_CTX|.
static int OCSP_basic_sign_ctx(OCSP_BASICRESP *resp, X509 *signer,
                               EVP_MD_CTX *ctx, STACK_OF(X509) *certs,
                               unsigned long flags) {
  GUARD_PTR(resp);
  GUARD_PTR(signer);

  if (ctx == NULL || ctx->pctx == NULL) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_NO_SIGNER_KEY);
    return 0;
  }

  EVP_PKEY *pkey = EVP_PKEY_CTX_get0_pkey(ctx->pctx);
  if (pkey == NULL || !X509_check_private_key(signer, pkey)) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_PRIVATE_KEY_DOES_NOT_MATCH_CERTIFICATE);
    return 0;
  }

  // Add relevant certificates to the response. This is optional according to
  // the RFC.
  if (!IS_OCSP_FLAG_SET(flags, OCSP_NOCERTS)) {
    if (!OCSP_basic_add1_cert(resp, signer)) {
      return 0;
    }
    for (size_t i = 0; i < sk_X509_num(certs); i++) {
      X509 *tmpcert = sk_X509_value(certs, i);
      if (!OCSP_basic_add1_cert(resp, tmpcert)) {
        return 0;
      }
    }
  }

  // Set |responderId| of response.
  OCSP_RESPID *rid = resp->tbsResponseData->responderId;
  if (IS_OCSP_FLAG_SET(flags, OCSP_RESPID_KEY)) {
    if (!OCSP_RESPID_set_by_key(rid, signer)) {
      return 0;
    }
  } else if (!OCSP_RESPID_set_by_name(rid, signer)) {
    // The OCSP responder is identified by name by default.
    return 0;
  }

  // Set |producedAt| time field of response. Although this can be
  // excluded with |OCSP_NOTIME|, a definitive response should include
  // the generation time.
  if (!IS_OCSP_FLAG_SET(flags, OCSP_NOTIME)) {
    if (!ASN1_GENERALIZEDTIME_set(resp->tbsResponseData->producedAt,
                                  time(NULL))) {
      return 0;
    }
  }

  // Do the actual signing. This is mandatory according to the RFC.
  if (!ASN1_item_sign_ctx(ASN1_ITEM_rptr(OCSP_RESPDATA),
                          resp->signatureAlgorithm, NULL, resp->signature,
                          resp->tbsResponseData, ctx)) {
    return 0;
  }

  return 1;
}

const EVP_MD *OCSP_get_default_digest(const EVP_MD *dgst, EVP_PKEY *signer) {
  if (dgst != NULL) {
    return dgst;
  }
  int pkey_nid = EVP_PKEY_id(signer);
  if (pkey_nid == EVP_PKEY_EC || pkey_nid == EVP_PKEY_RSA ||
      pkey_nid == EVP_PKEY_DSA) {
    return EVP_sha256();
  }
  return NULL;
}

int OCSP_basic_sign(OCSP_BASICRESP *resp, X509 *signer, EVP_PKEY *key,
                    const EVP_MD *dgst, STACK_OF(X509) *certs,
                    unsigned long flags) {
  GUARD_PTR(resp);
  GUARD_PTR(signer);
  GUARD_PTR(key);

  const EVP_MD *init_dgst = OCSP_get_default_digest(dgst, key);
  if (init_dgst == NULL) {
    OPENSSL_PUT_ERROR(OCSP, EVP_R_NO_DEFAULT_DIGEST);
    return 0;
  }

  EVP_MD_CTX *ctx = EVP_MD_CTX_new();
  if (ctx == NULL) {
    return 0;
  }

  if (!EVP_DigestSignInit(ctx, NULL, init_dgst, NULL, key)) {
    EVP_MD_CTX_free(ctx);
    return 0;
  }
  int ret = OCSP_basic_sign_ctx(resp, signer, ctx, certs, flags);
  EVP_MD_CTX_free(ctx);
  return ret;
}

OCSP_SINGLERESP *OCSP_basic_add1_status(OCSP_BASICRESP *resp, OCSP_CERTID *cid,
                                        int status, int revoked_reason,
                                        ASN1_TIME *revoked_time,
                                        ASN1_TIME *this_update,
                                        ASN1_TIME *next_update) {
  if (resp == NULL || resp->tbsResponseData == NULL || cid == NULL ||
      this_update == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  // Ambiguous status values are not allowed.
  if (status < V_OCSP_CERTSTATUS_GOOD || status > V_OCSP_CERTSTATUS_UNKNOWN) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_UNKNOWN_FIELD_VALUE);
    return NULL;
  }

  OCSP_SINGLERESP *single = OCSP_SINGLERESP_new();
  if (single == NULL) {
    goto err;
  }

  // Init |resp->tbsResponseData->responses| if NULL.
  if (resp->tbsResponseData->responses == NULL) {
    resp->tbsResponseData->responses = sk_OCSP_SINGLERESP_new_null();
    if (resp->tbsResponseData->responses == NULL) {
      goto err;
    }
  }

  if (!ASN1_TIME_to_generalizedtime(this_update, &single->thisUpdate)) {
    goto err;
  }
  if (next_update != NULL) {
    // |next_update| is allowed to be NULL. Only set |single->nextUpdate| if
    // |next_update| is non-NULL.
    if (!ASN1_TIME_to_generalizedtime(next_update, &single->nextUpdate)) {
      goto err;
    }
  }

  // Reset |single->certId|.
  OCSP_CERTID_free(single->certId);
  single->certId = OCSP_CERTID_dup(cid);
  if (single->certId == NULL) {
    goto err;
  }

  single->certStatus->type = status;
  switch (single->certStatus->type) {
    case V_OCSP_CERTSTATUS_REVOKED:
      if (revoked_time == NULL) {
        OPENSSL_PUT_ERROR(OCSP, OCSP_R_NO_REVOKED_TIME);
        goto err;
      }

      single->certStatus->value.revoked = OCSP_REVOKEDINFO_new();
      if (single->certStatus->value.revoked == NULL) {
        goto err;
      }

      // Start assigning values to |info| once initialized successfully.
      OCSP_REVOKEDINFO *info = single->certStatus->value.revoked;
      if (!ASN1_TIME_to_generalizedtime(revoked_time, &info->revocationTime)) {
        goto err;
      }
      // https://www.rfc-editor.org/rfc/rfc5280#section-5.3.1 specifies the only
      // valid reason codes are 0-10. Value 7 is not used.
      if (revoked_reason < OCSP_REVOKED_STATUS_UNSPECIFIED ||
          revoked_reason > OCSP_REVOKED_STATUS_AACOMPROMISE ||
          revoked_reason == OCSP_UNASSIGNED_REVOKED_STATUS) {
        OPENSSL_PUT_ERROR(OCSP, OCSP_R_UNKNOWN_FIELD_VALUE);
        goto err;
      }
      info->revocationReason = ASN1_ENUMERATED_new();
      if (info->revocationReason == NULL ||
          !ASN1_ENUMERATED_set(info->revocationReason, revoked_reason)) {
        goto err;
      }

      break;

    case V_OCSP_CERTSTATUS_GOOD:
      single->certStatus->value.good = ASN1_NULL_new();
      if (single->certStatus->value.good == NULL) {
        goto err;
      }
      break;

    case V_OCSP_CERTSTATUS_UNKNOWN:
      single->certStatus->value.unknown = ASN1_NULL_new();
      if (single->certStatus->value.unknown == NULL) {
        goto err;
      }
      break;

    default:
      goto err;
  }

  // Finally add the |OCSP_SINGLERESP| we were working with to |resp|.
  if (!sk_OCSP_SINGLERESP_push(resp->tbsResponseData->responses, single)) {
    goto err;
  }
  return single;

err:
  OCSP_SINGLERESP_free(single);
  return NULL;
}

OCSP_RESPONSE *OCSP_response_create(int status, OCSP_BASICRESP *bs) {
  if (status < OCSP_RESPONSE_STATUS_SUCCESSFUL ||
      status > OCSP_RESPONSE_STATUS_UNAUTHORIZED ||
      // 4 is not a valid response status code.
      status == OCSP_UNASSIGNED_RESPONSE_STATUS) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_UNKNOWN_FIELD_VALUE);
    return NULL;
  }

  OCSP_RESPONSE *rsp = OCSP_RESPONSE_new();
  if (rsp == NULL) {
    goto err;
  }
  if (!ASN1_ENUMERATED_set(rsp->responseStatus, status)) {
    goto err;
  }
  if (bs == NULL) {
    // |bs| is allowed to be NULL.
    return rsp;
  }

  rsp->responseBytes = OCSP_RESPBYTES_new();
  if (rsp->responseBytes == NULL) {
    goto err;
  }

  // responseType will be id-pkix-ocsp-basic according to RFC6960.
  //
  // https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1
  rsp->responseBytes->responseType = OBJ_nid2obj(NID_id_pkix_OCSP_basic);
  if (!ASN1_item_pack(bs, ASN1_ITEM_rptr(OCSP_BASICRESP),
                      &rsp->responseBytes->response)) {
    goto err;
  }
  return rsp;

err:
  OCSP_RESPONSE_free(rsp);
  return NULL;
}
