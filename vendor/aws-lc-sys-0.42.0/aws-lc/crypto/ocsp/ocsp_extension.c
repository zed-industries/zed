// Copyright 2015-2021 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Modifications Copyright Amazon.com, Inc. or its affiliates.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/mem.h>
#include <openssl/rand.h>
#include <string.h>

#include "../internal.h"
#include "internal.h"

#define OCSP_DEFAULT_NONCE_LENGTH 16

int OCSP_REQUEST_get_ext_by_NID(OCSP_REQUEST *req, int nid, int lastpos) {
  return X509v3_get_ext_by_NID(req->tbsRequest->requestExtensions, nid,
                               lastpos);
}

X509_EXTENSION *OCSP_REQUEST_get_ext(OCSP_REQUEST *req, int loc) {
  return X509v3_get_ext(req->tbsRequest->requestExtensions, loc);
}

int OCSP_BASICRESP_add_ext(OCSP_BASICRESP *bs, X509_EXTENSION *ex, int loc) {
  return (X509v3_add_ext(&bs->tbsResponseData->responseExtensions, ex, loc) !=
          NULL);
}

int OCSP_BASICRESP_get_ext_by_NID(OCSP_BASICRESP *bs, int nid, int lastpos) {
  return X509v3_get_ext_by_NID(bs->tbsResponseData->responseExtensions, nid,
                               lastpos);
}

X509_EXTENSION *OCSP_BASICRESP_get_ext(OCSP_BASICRESP *bs, int loc) {
  return X509v3_get_ext(bs->tbsResponseData->responseExtensions, loc);
}

X509_EXTENSION *OCSP_BASICRESP_delete_ext(OCSP_BASICRESP *x, int loc) {
  return X509v3_delete_ext(x->tbsResponseData->responseExtensions, loc);
}

int OCSP_SINGLERESP_add_ext(OCSP_SINGLERESP *sresp, X509_EXTENSION *ex,
                            int loc) {
  GUARD_PTR(sresp);
  return (X509v3_add_ext(&sresp->singleExtensions, ex, loc) != NULL);
}

int OCSP_SINGLERESP_get_ext_count(OCSP_SINGLERESP *sresp) {
  GUARD_PTR(sresp);
  return X509v3_get_ext_count(sresp->singleExtensions);
}

X509_EXTENSION *OCSP_SINGLERESP_get_ext(OCSP_SINGLERESP *sresp, int loc) {
  GUARD_PTR(sresp);
  return X509v3_get_ext(sresp->singleExtensions, loc);
}

static int ocsp_add_nonce(STACK_OF(X509_EXTENSION) **exts, unsigned char *val,
                          int len) {
  unsigned char *tmpval;
  ASN1_OCTET_STRING os;
  int ret = 0;
  if (len <= 0) {
    len = OCSP_DEFAULT_NONCE_LENGTH;
  }
  // Create the OCTET STRING manually by writing out the header and
  // appending the content octets. This avoids an extra memory allocation
  // operation in some cases. Applications should *NOT* do this because it
  // relies on library internals.
  os.length = ASN1_object_size(0, len, V_ASN1_OCTET_STRING);
  if (os.length < 0) {
    return 0;
  }

  os.data = OPENSSL_malloc(os.length);
  if (os.data == NULL) {
    goto err;
  }
  tmpval = os.data;
  ASN1_put_object(&tmpval, 0, len, V_ASN1_OCTET_STRING, V_ASN1_UNIVERSAL);
  if (val != NULL) {
    OPENSSL_memcpy(tmpval, val, len);
  } else {
    AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(tmpval, len));
  }
  if (X509V3_add1_i2d(exts, NID_id_pkix_OCSP_Nonce, &os, 0,
                      X509V3_ADD_REPLACE) <= 0) {
    goto err;
  }
  ret = 1;
err:
  OPENSSL_free(os.data);
  return ret;
}

int OCSP_request_add1_nonce(OCSP_REQUEST *req, unsigned char *val, int len) {
  if (req == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  if (val != NULL && len <= 0) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }
  return ocsp_add_nonce(&req->tbsRequest->requestExtensions, val, len);
}

int OCSP_basic_add1_nonce(OCSP_BASICRESP *resp, unsigned char *val, int len) {
  if (resp == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  if (val != NULL && len <= 0) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }
  return ocsp_add_nonce(&resp->tbsResponseData->responseExtensions, val, len);
}

int OCSP_check_nonce(OCSP_REQUEST *req, OCSP_BASICRESP *bs) {
  if (req == NULL || bs == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return OCSP_NONCE_NOT_EQUAL;
  }
  // Since we are only interested in the presence or absence of
  // the nonce and comparing its value there is no need to use
  // the X509V3 routines: this way we can avoid them allocating an
  // ASN1_OCTET_STRING structure for the value which would be
  // freed immediately anyway.

  int req_idx, resp_idx;
  X509_EXTENSION *req_ext, *resp_ext;
  req_idx = OCSP_REQUEST_get_ext_by_NID(req, NID_id_pkix_OCSP_Nonce, -1);
  resp_idx = OCSP_BASICRESP_get_ext_by_NID(bs, NID_id_pkix_OCSP_Nonce, -1);
  // Check that both are absent.
  if ((req_idx < 0) && (resp_idx < 0)) {
    return OCSP_NONCE_BOTH_ABSENT;
  }
  // Check in request only.
  if ((req_idx >= 0) && (resp_idx < 0)) {
    return OCSP_NONCE_REQUEST_ONLY;
  }
  // Check in response, but not request.
  if ((req_idx < 0) && (resp_idx >= 0)) {
    return OCSP_NONCE_RESPONSE_ONLY;
  }
  // Otherwise, there is a nonce in both the request and response, so retrieve
  // the extensions.
  req_ext = OCSP_REQUEST_get_ext(req, req_idx);
  resp_ext = OCSP_BASICRESP_get_ext(bs, resp_idx);
  if (ASN1_OCTET_STRING_cmp(X509_EXTENSION_get_data(req_ext),
                            X509_EXTENSION_get_data(resp_ext))) {
    return OCSP_NONCE_NOT_EQUAL;
  }
  return OCSP_NONCE_EQUAL;
}

int OCSP_copy_nonce(OCSP_BASICRESP *resp, OCSP_REQUEST *req) {
  GUARD_PTR(resp);
  GUARD_PTR(req);

  // Check for nonce in request.
  int req_idx = OCSP_REQUEST_get_ext_by_NID(req, NID_id_pkix_OCSP_Nonce, -1);
  // If no nonce, that's OK. We return 2 in this case.
  if (req_idx < 0) {
    return 2;
  }
  X509_EXTENSION *req_ext = OCSP_REQUEST_get_ext(req, req_idx);
  // Nonce found, but no entry at the index.
  // This shouldn't happen under normal circumstances.
  GUARD_PTR(req_ext);

  // Append the nonce.
  return OCSP_BASICRESP_add_ext(resp, req_ext, -1);
}
