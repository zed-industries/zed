// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include "../asn1/internal.h"
#include "internal.h"

OCSP_ONEREQ *OCSP_request_add0_id(OCSP_REQUEST *req, OCSP_CERTID *cid) {
  OCSP_ONEREQ *one = OCSP_ONEREQ_new();
  if (one == NULL) {
    return NULL;
  }
  // Reassign |OCSP_CERTID| allocated by OCSP_ONEREQ_new().
  OCSP_CERTID_free(one->reqCert);
  one->reqCert = cid;
  if (req != NULL && !sk_OCSP_ONEREQ_push(req->tbsRequest->requestList, one)) {
    one->reqCert = NULL;  // do not free on error
    OCSP_ONEREQ_free(one);
    return NULL;
  }
  return one;
}

int OCSP_request_set1_name(OCSP_REQUEST *req, X509_NAME *nm) {
  GENERAL_NAME *gen = GENERAL_NAME_new();
  if (gen == NULL) {
    return 0;
  }
  if (!X509_NAME_set(&gen->d.directoryName, nm)) {
    GENERAL_NAME_free(gen);
    return 0;
  }
  gen->type = GEN_DIRNAME;
  GENERAL_NAME_free(req->tbsRequest->requestorName);
  req->tbsRequest->requestorName = gen;
  return 1;
}

int OCSP_request_add1_cert(OCSP_REQUEST *req, X509 *cert) {
  if (req->optionalSignature == NULL) {
    req->optionalSignature = OCSP_SIGNATURE_new();
  }
  OCSP_SIGNATURE *sig = req->optionalSignature;
  if (sig == NULL) {
    return 0;
  }
  if (cert == NULL) {
    return 1;
  }

  if (sig->certs == NULL && (sig->certs = sk_X509_new_null()) == NULL) {
    return 0;
  }

  if (!sk_X509_push(sig->certs, cert)) {
    return 0;
  }
  // sk_X509_push takes ownership.
  X509_up_ref(cert);
  return 1;
}

int OCSP_request_sign(OCSP_REQUEST *req, X509 *signer, EVP_PKEY *key,
                      const EVP_MD *dgst, STACK_OF(X509) *certs,
                      unsigned long flags) {
  if (req->optionalSignature != NULL) {
    // OpenSSL lets an |OCSP_REQUEST| be signed twice, regardless of whether
    // an |optionalSignature| exists. Signing an OCSP Request twice creates
    // a dangling |OCSP_SIGNATURE| pointer with no clear way of recovering
    // it. We disallow this behavior and only allow the |OCSP_REQUEST| to be
    // signed once. There's no indication or use case detailed in the RFC
    // that the OCSP request can or should be signed twice.
    // https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.2
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_OCSP_REQUEST_DUPLICATE_SIGNATURE);
    goto err;
  }

  if (!OCSP_request_set1_name(req, X509_get_subject_name(signer))) {
    goto err;
  }
  req->optionalSignature = OCSP_SIGNATURE_new();
  if (req->optionalSignature == NULL) {
    goto err;
  }
  if (key != NULL) {
    if (!X509_check_private_key(signer, key)) {
      OPENSSL_PUT_ERROR(OCSP, OCSP_R_PRIVATE_KEY_DOES_NOT_MATCH_CERTIFICATE);
      goto err;
    }
    const EVP_MD *init_dgst = OCSP_get_default_digest(dgst, key);
    if (init_dgst == NULL) {
      OPENSSL_PUT_ERROR(OCSP, EVP_R_NO_DEFAULT_DIGEST);
      goto err;
    }

    if (!ASN1_item_sign(ASN1_ITEM_rptr(OCSP_REQINFO),
                        req->optionalSignature->signatureAlgorithm, NULL,
                        req->optionalSignature->signature, req->tbsRequest, key,
                        init_dgst)) {
      goto err;
    }
  }

  if (!IS_OCSP_FLAG_SET(flags, OCSP_NOCERTS)) {
    if (!OCSP_request_add1_cert(req, signer)) {
      goto err;
    }
    for (size_t i = 0; i < sk_X509_num(certs); i++) {
      if (!OCSP_request_add1_cert(req, sk_X509_value(certs, i))) {
        goto err;
      }
    }
  }

  return 1;
err:
  OCSP_SIGNATURE_free(req->optionalSignature);
  req->optionalSignature = NULL;
  return 0;
}

int OCSP_response_status(OCSP_RESPONSE *resp) {
  if (resp == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }
  return ASN1_ENUMERATED_get(resp->responseStatus);
}

OCSP_BASICRESP *OCSP_response_get1_basic(OCSP_RESPONSE *resp) {
  if (resp == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  OCSP_RESPBYTES *rb = resp->responseBytes;
  if (rb == NULL) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_NO_RESPONSE_DATA);
    return NULL;
  }
  if (OBJ_obj2nid(rb->responseType) != NID_id_pkix_OCSP_basic) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_NOT_BASIC_RESPONSE);
    return NULL;
  }
  return ASN1_item_unpack(rb->response, ASN1_ITEM_rptr(OCSP_BASICRESP));
}

OCSP_SINGLERESP *OCSP_resp_get0(OCSP_BASICRESP *bs, size_t idx) {
  if (bs == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }
  if (bs->tbsResponseData == NULL) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_NO_RESPONSE_DATA);
    return NULL;
  }
  return sk_OCSP_SINGLERESP_value(bs->tbsResponseData->responses, idx);
}

int OCSP_resp_find(OCSP_BASICRESP *bs, OCSP_CERTID *id, int last) {
  if (bs == NULL || id == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }
  if (bs->tbsResponseData == NULL) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_NO_RESPONSE_DATA);
    return -1;
  }

  // |OCSP_SINGLERESP| stack is in |OCSP_BASICRESP| responseData, we look for
  // |OCSP_CERTID| in here.
  STACK_OF(OCSP_SINGLERESP) *sresp = bs->tbsResponseData->responses;
  OCSP_SINGLERESP *single;

  if (last < 0) {
    last = 0;
  } else {
    last++;
  }
  for (size_t i = last; i < sk_OCSP_SINGLERESP_num(sresp); i++) {
    single = sk_OCSP_SINGLERESP_value(sresp, i);
    if (!OCSP_id_cmp(id, single->certId)) {
      return i;
    }
  }
  return -1;
}

int OCSP_single_get0_status(OCSP_SINGLERESP *single, int *reason,
                            ASN1_GENERALIZEDTIME **revtime,
                            ASN1_GENERALIZEDTIME **thisupd,
                            ASN1_GENERALIZEDTIME **nextupd) {
  if (single == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }
  OCSP_CERTSTATUS *cst = single->certStatus;
  if (cst == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }
  int status = cst->type;

  // If certificate status is revoked, we look up certificate revocation time
  // and reason.
  if (status == V_OCSP_CERTSTATUS_REVOKED) {
    OCSP_REVOKEDINFO *rev = cst->value.revoked;
    if (rev != NULL) {
      if (revtime != NULL) {
        *revtime = rev->revocationTime;
      }
      if (reason != NULL) {
        if (rev->revocationReason) {
          *reason = ASN1_ENUMERATED_get(rev->revocationReason);
        } else {
          *reason = -1;
        }
      }
    }
  }
  // Send back when certificate was last updated and when is the next update
  // time.
  if (thisupd != NULL) {
    *thisupd = single->thisUpdate;
  }
  if (nextupd != NULL) {
    *nextupd = single->nextUpdate;
  }
  return status;
}

int OCSP_resp_find_status(OCSP_BASICRESP *bs, OCSP_CERTID *id, int *status,
                          int *reason, ASN1_GENERALIZEDTIME **revtime,
                          ASN1_GENERALIZEDTIME **thisupd,
                          ASN1_GENERALIZEDTIME **nextupd) {
  if (bs == NULL || id == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  // we look for index of equivalent |OCSP_CERTID| of issuer certificate in
  // |OCSP_BASICRESP|
  int single_idx = OCSP_resp_find(bs, id, -1);
  if (single_idx < 0) {
    return 0;
  }
  OCSP_SINGLERESP *single = OCSP_resp_get0(bs, single_idx);

  // Extract the update time and revocation status of certificate sent back
  // from OCSP responder
  int single_status =
      OCSP_single_get0_status(single, reason, revtime, thisupd, nextupd);
  if (status != NULL) {
    *status = single_status;
  }
  return 1;
}

int OCSP_check_validity(ASN1_GENERALIZEDTIME *thisUpdate,
                        ASN1_GENERALIZEDTIME *nextUpdate,
                        long drift_num_seconds, long max_age_seconds) {
  int ret = 1;
  int64_t t_tmp;
  int64_t t_now = time(NULL);

  // Check |thisUpdate| is valid and not more than |drift_num_seconds| in the
  // future.
  if (!ASN1_GENERALIZEDTIME_check(thisUpdate)) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_ERROR_IN_THISUPDATE_FIELD);
    ret = 0;
  } else {
    t_tmp = t_now + drift_num_seconds;
    if (X509_cmp_time_posix(thisUpdate, t_tmp) > 0) {
      OPENSSL_PUT_ERROR(OCSP, OCSP_R_STATUS_NOT_YET_VALID);
      ret = 0;
    }

    // If |max_num_seconds| is specified, check that |thisUpdate| is not more
    // than |max_num_seconds| in the past.
    if (max_age_seconds >= 0) {
      t_tmp = t_now - max_age_seconds;
      if (X509_cmp_time_posix(thisUpdate, t_tmp) < 0) {
        OPENSSL_PUT_ERROR(OCSP, OCSP_R_STATUS_TOO_OLD);
        ret = 0;
      }
    }
  }

  // If |nextUpdate| field is empty, we have validated everything we can at
  // this point.
  if (nextUpdate == NULL) {
    return ret;
  }

  // Check |nextUpdate| is valid and not more than |drift_num_seconds| in the
  // past.
  if (!ASN1_GENERALIZEDTIME_check(nextUpdate)) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_ERROR_IN_NEXTUPDATE_FIELD);
    ret = 0;
  } else {
    t_tmp = t_now - drift_num_seconds;
    if (X509_cmp_time_posix(nextUpdate, t_tmp) < 0) {
      OPENSSL_PUT_ERROR(OCSP, OCSP_R_STATUS_EXPIRED);
      ret = 0;
    }
  }

  // Also don't allow |nextUpdate| to precede |thisUpdate|.
  if (ASN1_STRING_cmp(nextUpdate, thisUpdate) < 0) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_NEXTUPDATE_BEFORE_THISUPDATE);
    ret = 0;
  }

  return ret;
}

int OCSP_resp_count(OCSP_BASICRESP *bs) {
  if (bs == NULL) {
    return -1;
  }
  return (int)sk_OCSP_SINGLERESP_num(bs->tbsResponseData->responses);
}

const OCSP_CERTID *OCSP_SINGLERESP_get0_id(const OCSP_SINGLERESP *single) {
  return single->certId;
}
