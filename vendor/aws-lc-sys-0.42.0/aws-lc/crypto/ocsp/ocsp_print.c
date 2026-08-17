// Copyright 2000-2016 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ocsp.h>
#include <openssl/pem.h>

#include "internal.h"

static int ocsp_certid_print(BIO *bp, OCSP_CERTID *certid, int indent) {
  BIO_printf(bp, "%*sCertificate ID:\n", indent, "");
  indent += 2;
  BIO_printf(bp, "%*sHash Algorithm: ", indent, "");
  i2a_ASN1_OBJECT(bp, certid->hashAlgorithm->algorithm);
  BIO_printf(bp, "\n%*sIssuer Name Hash: ", indent, "");
  i2a_ASN1_STRING(bp, certid->issuerNameHash, 0);
  BIO_printf(bp, "\n%*sIssuer Key Hash: ", indent, "");
  i2a_ASN1_STRING(bp, certid->issuerKeyHash, 0);
  BIO_printf(bp, "\n%*sSerial Number: ", indent, "");
  i2a_ASN1_INTEGER(bp, certid->serialNumber);
  BIO_printf(bp, "\n");
  return 1;
}

typedef struct {
  long t;
  const char *m;
} OCSP_TBLSTR;

static const char *do_table2string(long s, const OCSP_TBLSTR *ts, size_t len) {
  size_t i;
  for (i = 0; i < len; i++) {
    if (ts[i].t == s) {
      return ts[i].m;
    }
  }
  return "(UNKNOWN)";
}

const char *OCSP_response_status_str(long status_code) {
  static const OCSP_TBLSTR rstat_tbl[] = {
      {OCSP_RESPONSE_STATUS_SUCCESSFUL, "successful"},
      {OCSP_RESPONSE_STATUS_MALFORMEDREQUEST, "malformedrequest"},
      {OCSP_RESPONSE_STATUS_INTERNALERROR, "internalerror"},
      {OCSP_RESPONSE_STATUS_TRYLATER, "trylater"},
      {OCSP_RESPONSE_STATUS_SIGREQUIRED, "sigrequired"},
      {OCSP_RESPONSE_STATUS_UNAUTHORIZED, "unauthorized"}};
  size_t tbl_size = (sizeof(rstat_tbl) / sizeof((rstat_tbl)[0]));
  return do_table2string(status_code, rstat_tbl, tbl_size);
}

const char *OCSP_cert_status_str(long status_code) {
  static const OCSP_TBLSTR cstat_tbl[] = {
      {V_OCSP_CERTSTATUS_GOOD, "good"},
      {V_OCSP_CERTSTATUS_REVOKED, "revoked"},
      {V_OCSP_CERTSTATUS_UNKNOWN, "unknown"}};
  size_t tbl_size = (sizeof(cstat_tbl) / sizeof((cstat_tbl)[0]));
  return do_table2string(status_code, cstat_tbl, tbl_size);
}

const char *OCSP_crl_reason_str(long s) {
  static const OCSP_TBLSTR reason_tbl[] = {
      {OCSP_REVOKED_STATUS_UNSPECIFIED, "unspecified"},
      {OCSP_REVOKED_STATUS_KEYCOMPROMISE, "keyCompromise"},
      {OCSP_REVOKED_STATUS_CACOMPROMISE, "cACompromise"},
      {OCSP_REVOKED_STATUS_AFFILIATIONCHANGED, "affiliationChanged"},
      {OCSP_REVOKED_STATUS_SUPERSEDED, "superseded"},
      {OCSP_REVOKED_STATUS_CESSATIONOFOPERATION, "cessationOfOperation"},
      {OCSP_REVOKED_STATUS_CERTIFICATEHOLD, "certificateHold"},
      {OCSP_REVOKED_STATUS_REMOVEFROMCRL, "removeFromCRL"},
      {OCSP_REVOKED_STATUS_PRIVILEGEWITHDRAWN, "privilegeWithdrawn"},
      {OCSP_REVOKED_STATUS_AACOMPROMISE, "aACompromise"}};
  size_t tbl_size = (sizeof(reason_tbl) / sizeof((reason_tbl)[0]));
  return do_table2string(s, reason_tbl, tbl_size);
}

int OCSP_REQUEST_print(BIO *bp, OCSP_REQUEST *req, unsigned long flags) {
  if(bp == NULL|| req ==NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  long l;
  OCSP_CERTID *cid = NULL;
  OCSP_ONEREQ *one = NULL;
  OCSP_REQINFO *inf = req->tbsRequest;
  OCSP_SIGNATURE *sig = req->optionalSignature;

  if (BIO_puts(bp, "OCSP Request Data:\n") <= 0) {
    return 0;
  }
  l = ASN1_INTEGER_get(inf->version);
  if (BIO_printf(bp, "    Version: %ld (0x%ld)", l + 1, l) <= 0) {
    return 0;
  }
  if (inf->requestorName != NULL) {
    if (BIO_puts(bp, "\n    Requestor Name: ") <= 0) {
      return 0;
    }
    GENERAL_NAME_print(bp, inf->requestorName);
  }
  if (BIO_puts(bp, "\n    Requestor List:\n") <= 0) {
    return 0;
  }
  for (size_t i = 0; i < sk_OCSP_ONEREQ_num(inf->requestList); i++) {
    one = sk_OCSP_ONEREQ_value(inf->requestList, i);
    cid = one->reqCert;
    ocsp_certid_print(bp, cid, 8);
    if (!X509V3_extensions_print(bp, "Request Single Extensions",
                                 one->singleRequestExtensions, flags, 8)) {
      return 0;
    }
  }
  if (!X509V3_extensions_print(bp, "Request Extensions", inf->requestExtensions,
                               flags, 4)) {
    return 0;
  }
  if (sig != NULL) {
    X509_signature_print(bp, sig->signatureAlgorithm, sig->signature);
    for (size_t i = 0; i < sk_X509_num(sig->certs); i++) {
      X509_print(bp, sk_X509_value(sig->certs, i));
      PEM_write_bio_X509(bp, sk_X509_value(sig->certs, i));
    }
  }
  return 1;
}

int OCSP_RESPONSE_print(BIO *bp, OCSP_RESPONSE *resp, unsigned long flags) {
  if(bp == NULL|| resp ==NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  int ret = 0;
  long l;
  OCSP_CERTID *cid = NULL;
  OCSP_BASICRESP *br = NULL;
  OCSP_RESPID *rid = NULL;
  OCSP_RESPDATA *rd = NULL;
  OCSP_CERTSTATUS *cst = NULL;
  OCSP_REVOKEDINFO *rev = NULL;
  OCSP_SINGLERESP *single = NULL;
  OCSP_RESPBYTES *rb = resp->responseBytes;

  if (BIO_puts(bp, "OCSP Response Data:\n") <= 0) {
    goto err;
  }
  l = ASN1_ENUMERATED_get(resp->responseStatus);
  if (BIO_printf(bp, "    OCSP Response Status: %s (0x%ld)\n",
                 OCSP_response_status_str(l), l) <= 0) {
    goto err;
  }
  if (rb == NULL) {
    return 1;
  }
  if (BIO_puts(bp, "    Response Type: ") <= 0) {
    goto err;
  }
  if (i2a_ASN1_OBJECT(bp, rb->responseType) <= 0) {
    goto err;
  }
  if (OBJ_obj2nid(rb->responseType) != NID_id_pkix_OCSP_basic) {
    BIO_puts(bp, " (unknown response type)\n");
    return 1;
  }

  if ((br = OCSP_response_get1_basic(resp)) == NULL) {
    goto err;
  }
  rd = br->tbsResponseData;
  l = ASN1_INTEGER_get(rd->version);
  if (BIO_printf(bp, "\n    Version: %ld (0x%ld)\n", l + 1, l) <= 0) {
    goto err;
  }
  if (BIO_puts(bp, "    Responder Id: ") <= 0) {
    goto err;
  }

  rid = rd->responderId;
  switch (rid->type) {
    case V_OCSP_RESPID_NAME:
      if (rid->value.byName == NULL) {
        goto err;
      }
      if (X509_NAME_print_ex(bp, rid->value.byName, 0, XN_FLAG_ONELINE) < 0) {
        goto err;
      }
      break;
    case V_OCSP_RESPID_KEY:
      if (rid->value.byKey == NULL) {
        goto err;
      }
      i2a_ASN1_STRING(bp, rid->value.byKey, 0);
      break;
    default:
      goto err;
  }

  if (BIO_printf(bp, "\n    Produced At: ") <= 0) {
    goto err;
  }
  if (!ASN1_GENERALIZEDTIME_print(bp, rd->producedAt)) {
    goto err;
  }
  if (BIO_printf(bp, "\n    Responses:\n") <= 0) {
    goto err;
  }
  for (size_t i = 0; i < sk_OCSP_SINGLERESP_num(rd->responses); i++) {
    if (!sk_OCSP_SINGLERESP_value(rd->responses, i)) {
      continue;
    }
    single = sk_OCSP_SINGLERESP_value(rd->responses, i);
    cid = single->certId;
    if (ocsp_certid_print(bp, cid, 4) <= 0) {
      goto err;
    }
    cst = single->certStatus;
    if (BIO_printf(bp, "    Cert Status: %s",
                   OCSP_cert_status_str(cst->type)) <= 0) {
      goto err;
    }
    if (cst->type == V_OCSP_CERTSTATUS_REVOKED) {
      rev = cst->value.revoked;
      if (BIO_printf(bp, "\n    Revocation Time: ") <= 0) {
        goto err;
      }
      if (!ASN1_GENERALIZEDTIME_print(bp, rev->revocationTime)) {
        goto err;
      }
      if (rev->revocationReason) {
        l = ASN1_ENUMERATED_get(rev->revocationReason);
        if (BIO_printf(bp, "\n    Revocation Reason: %s (0x%ld)",
                       OCSP_crl_reason_str(l), l) <= 0) {
          goto err;
        }
      }
    }
    if (BIO_printf(bp, "\n    This Update: ") <= 0) {
      goto err;
    }
    if (!ASN1_GENERALIZEDTIME_print(bp, single->thisUpdate)) {
      goto err;
    }
    if (single->nextUpdate) {
      if (BIO_printf(bp, "\n    Next Update: ") <= 0) {
        goto err;
      }
      if (!ASN1_GENERALIZEDTIME_print(bp, single->nextUpdate)) {
        goto err;
      }
    }
    if (BIO_puts(bp, "\n") <= 0) {
      goto err;
    }
    if (!X509V3_extensions_print(bp, "Response Single Extensions",
                                 single->singleExtensions, flags, 8)) {
      goto err;
    }
    if (BIO_puts(bp, "\n") <= 0) {
      goto err;
    }
  }
  if (!X509V3_extensions_print(bp, "Response Extensions",
                               rd->responseExtensions, flags, 4)) {
    goto err;
  }
  if (X509_signature_print(bp, br->signatureAlgorithm, br->signature) <= 0) {
    goto err;
  }

  for (size_t i = 0; i < sk_X509_num(br->certs); i++) {
    X509_print(bp, sk_X509_value(br->certs, i));
    PEM_write_bio_X509(bp, sk_X509_value(br->certs, i));
  }

  ret = 1;
err:
  OCSP_BASICRESP_free(br);
  return ret;
}
