// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/mem.h>
#include <string.h>

#include "internal.h"

OCSP_CERTID *OCSP_cert_to_id(const EVP_MD *dgst, const X509 *subject,
                             const X509 *issuer) {
  if (issuer == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  const X509_NAME *iname;
  const ASN1_INTEGER *serial;
  ASN1_BIT_STRING *ikey;

  if (dgst == NULL) {
    dgst = EVP_sha1();
  }
  if (subject != NULL) {
    iname = X509_get_issuer_name(subject);
    serial = X509_get0_serialNumber(subject);
  } else {
    iname = X509_get_subject_name(issuer);
    serial = NULL;
  }
  ikey = X509_get0_pubkey_bitstr(issuer);
  return OCSP_cert_id_new(dgst, iname, ikey, serial);
}

OCSP_CERTID *OCSP_cert_id_new(const EVP_MD *dgst, const X509_NAME *issuerName,
                              const ASN1_BIT_STRING *issuerKey,
                              const ASN1_INTEGER *serialNumber) {
  if (dgst == NULL || issuerName == NULL || issuerKey == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  int nid;
  unsigned int i;
  X509_ALGOR *alg;
  unsigned char md[EVP_MAX_MD_SIZE];
  OCSP_CERTID *cid = OCSP_CERTID_new();
  if (cid == NULL) {
    return NULL;
  }

  alg = cid->hashAlgorithm;
  ASN1_OBJECT_free(alg->algorithm);
  nid = EVP_MD_type(dgst);
  if (nid == NID_undef) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_UNKNOWN_NID);
    goto err;
  }
  alg->algorithm = OBJ_nid2obj(nid);
  if (alg->algorithm == NULL) {
    goto err;
  }
  alg->parameter = ASN1_TYPE_new();
  if (alg->parameter == NULL) {
    goto err;
  }
  alg->parameter->type = V_ASN1_NULL;

  if (!X509_NAME_digest(issuerName, dgst, md, &i)) {
    OPENSSL_PUT_ERROR(OCSP, OCSP_R_DIGEST_ERR);
    goto err;
  }
  if (!(ASN1_OCTET_STRING_set(cid->issuerNameHash, md, i))) {
    goto err;
  }
  // Calculate the issuerKey hash, excluding tag and length
  if (!EVP_Digest(issuerKey->data, issuerKey->length, md, &i, dgst, NULL)) {
    goto err;
  }
  if (!(ASN1_OCTET_STRING_set(cid->issuerKeyHash, md, i))) {
    goto err;
  }
  // Only copy |serialNumber| if available. This may be empty.
  if (serialNumber != NULL) {
    if (ASN1_STRING_copy(cid->serialNumber, serialNumber) == 0) {
      goto err;
    }
  }
  return cid;

err:
  OCSP_CERTID_free(cid);
  return NULL;
}

int OCSP_id_issuer_cmp(const OCSP_CERTID *a, const OCSP_CERTID *b) {
  if (a == NULL || b == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  if (a->hashAlgorithm == NULL || b->hashAlgorithm == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  int ret = OBJ_cmp(a->hashAlgorithm->algorithm, b->hashAlgorithm->algorithm);
  if (ret != 0) {
    return ret;
  }
  ret = ASN1_OCTET_STRING_cmp(a->issuerNameHash, b->issuerNameHash);
  if (ret != 0) {
    return ret;
  }
  ret = ASN1_OCTET_STRING_cmp(a->issuerKeyHash, b->issuerKeyHash);
  return ret;
}

int OCSP_id_cmp(const OCSP_CERTID *a, const OCSP_CERTID *b) {
  if (a == NULL || b == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return -1;
  }

  // Compare OCSP issuer name and key
  int ret = OCSP_id_issuer_cmp(a, b);
  if (ret != 0) {
    return ret;
  }
  // Compare certificate serialNumber
  ret = ASN1_INTEGER_cmp(a->serialNumber, b->serialNumber);
  return ret;
}

int OCSP_parse_url(const char *url, char **phost, char **pport, char **ppath,
                   int *pssl) {
  if (url == NULL || phost == NULL || pport == NULL || ppath == NULL ||
      pssl == NULL) {
    OPENSSL_PUT_ERROR(OCSP, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  char *parser, *buffer;
  char *host = NULL;
  char *port = NULL;

  *phost = NULL;
  *pport = NULL;
  *ppath = NULL;
  *pssl = 0;

  // Duplicate into the buffer since the contents are going to be changed.
  buffer = OPENSSL_strdup(url);
  if (buffer == NULL) {
    goto mem_err;
  }

  // Check for initial colon
  parser = strchr(buffer, ':');
  if (parser == NULL) {
    goto parse_err;
  }
  *(parser++) = '\0';

  // Set default ports for http and https. If a port is specified later, this
  // will be overwritten. |pssl| will be set to true, if https is being used.
  if (strcmp(buffer, "https") == 0) {
    *pssl = 1;
    port = (char *)"443";
  } else if (strcmp(buffer, "http") == 0) {
    *pssl = 0;
    port = (char *)"80";
  } else {
    goto parse_err;
  }

  // Check for double slash.
  if ((parser[0] != '/') || (parser[1] != '/')) {
    goto parse_err;
  }
  parser += 2;
  host = parser;

  // Check for trailing part of path.
  parser = strchr(parser, '/');
  if (parser == NULL) {
    // Default is "/" if there is no trailing path in the URL.
    *ppath = OPENSSL_strdup("/");
  } else {
    *ppath = OPENSSL_strdup(parser);
    // Set start of path to 0 so hostname is valid.
    *parser = '\0';
  }
  if (*ppath == NULL) {
    goto mem_err;
  }

  parser = host;
  // Checks if the host is an ipv6 host address.
  if (host[0] == '[') {
    host++;
    parser = strchr(host, ']');
    if (parser == NULL) {
      goto parse_err;
    }
    *parser = '\0';
    parser++;
  }

  // Look for optional ':' for port number.
  if ((parser = strchr(parser, ':'))) {
    *parser = 0;
    port = parser + 1;
  }

  *pport = OPENSSL_strdup(port);
  if (*pport == NULL) {
    goto mem_err;
  }

  *phost = OPENSSL_strdup(host);
  if (*phost == NULL) {
    goto mem_err;
  }
  OPENSSL_free(buffer);
  return 1;

mem_err:
  OPENSSL_PUT_ERROR(OCSP, ERR_R_MALLOC_FAILURE);
  goto err;
parse_err:
  OPENSSL_PUT_ERROR(OCSP, OCSP_R_ERROR_PARSING_URL);
err:
  OPENSSL_free(buffer);
  OPENSSL_free(*ppath);
  *ppath = NULL;
  *pssl = 0;
  OPENSSL_free(*pport);
  *pport = NULL;
  OPENSSL_free(*phost);
  *phost = NULL;
  return 0;
}
