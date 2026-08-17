// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2002 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>

#include <openssl/bio.h>
#include <openssl/dh.h>
#include <openssl/dsa.h>
#include <openssl/evp.h>
#include <openssl/pem.h>
#include <openssl/pkcs7.h>
#include <openssl/rsa.h>
#include <openssl/x509.h>
#include "../fipsmodule/ec/internal.h"

static RSA *pkey_get_rsa(EVP_PKEY *key, RSA **rsa);
static DSA *pkey_get_dsa(EVP_PKEY *key, DSA **dsa);
static EC_KEY *pkey_get_eckey(EVP_PKEY *key, EC_KEY **eckey);

IMPLEMENT_PEM_rw(X509_REQ, X509_REQ, PEM_STRING_X509_REQ, X509_REQ)

IMPLEMENT_PEM_write(X509_REQ_NEW, X509_REQ, PEM_STRING_X509_REQ_OLD, X509_REQ)
IMPLEMENT_PEM_rw(X509_CRL, X509_CRL, PEM_STRING_X509_CRL, X509_CRL)
IMPLEMENT_PEM_rw(PKCS7, PKCS7, PEM_STRING_PKCS7, PKCS7)

// We treat RSA or DSA private keys as a special case. For private keys we
// read in an EVP_PKEY structure with PEM_read_bio_PrivateKey() and extract
// the relevant private key: this means can handle "traditional" and PKCS#8
// formats transparently.
static RSA *pkey_get_rsa(EVP_PKEY *key, RSA **rsa) {
  RSA *rtmp;
  if (!key) {
    return NULL;
  }
  rtmp = EVP_PKEY_get1_RSA(key);
  EVP_PKEY_free(key);
  if (!rtmp) {
    return NULL;
  }
  if (rsa) {
    RSA_free(*rsa);
    *rsa = rtmp;
  }
  return rtmp;
}

RSA *PEM_read_bio_RSAPrivateKey(BIO *bp, RSA **rsa, pem_password_cb *cb,
                                void *u) {
  EVP_PKEY *pktmp;
  pktmp = PEM_read_bio_PrivateKey(bp, NULL, cb, u);
  return pkey_get_rsa(pktmp, rsa);
}

RSA *PEM_read_RSAPrivateKey(FILE *fp, RSA **rsa, pem_password_cb *cb, void *u) {
  EVP_PKEY *pktmp;
  pktmp = PEM_read_PrivateKey(fp, NULL, cb, u);
  return pkey_get_rsa(pktmp, rsa);
}

IMPLEMENT_PEM_write_cb_const(RSAPrivateKey, RSA, PEM_STRING_RSA, RSAPrivateKey)


IMPLEMENT_PEM_rw_const(RSAPublicKey, RSA, PEM_STRING_RSA_PUBLIC, RSAPublicKey)
IMPLEMENT_PEM_rw(RSA_PUBKEY, RSA, PEM_STRING_PUBLIC, RSA_PUBKEY)
#ifndef OPENSSL_NO_DSA
static DSA *pkey_get_dsa(EVP_PKEY *key, DSA **dsa) {
  DSA *dtmp;
  if (!key) {
    return NULL;
  }
  dtmp = EVP_PKEY_get1_DSA(key);
  EVP_PKEY_free(key);
  if (!dtmp) {
    return NULL;
  }
  if (dsa) {
    DSA_free(*dsa);
    *dsa = dtmp;
  }
  return dtmp;
}

DSA *PEM_read_bio_DSAPrivateKey(BIO *bp, DSA **dsa, pem_password_cb *cb,
                                void *u) {
  EVP_PKEY *pktmp;
  pktmp = PEM_read_bio_PrivateKey(bp, NULL, cb, u);
  return pkey_get_dsa(pktmp, dsa);  // will free pktmp
}

IMPLEMENT_PEM_write_cb_const(DSAPrivateKey, DSA, PEM_STRING_DSA, DSAPrivateKey)

IMPLEMENT_PEM_rw(DSA_PUBKEY, DSA, PEM_STRING_PUBLIC, DSA_PUBKEY)
DSA *PEM_read_DSAPrivateKey(FILE *fp, DSA **dsa, pem_password_cb *cb, void *u) {
  EVP_PKEY *pktmp;
  pktmp = PEM_read_PrivateKey(fp, NULL, cb, u);
  return pkey_get_dsa(pktmp, dsa);  // will free pktmp
}

IMPLEMENT_PEM_rw_const(DSAparams, DSA, PEM_STRING_DSAPARAMS, DSAparams)
#endif
static EC_KEY *pkey_get_eckey(EVP_PKEY *key, EC_KEY **eckey) {
  EC_KEY *dtmp;
  if (!key) {
    return NULL;
  }
  dtmp = EVP_PKEY_get1_EC_KEY(key);
  EVP_PKEY_free(key);
  if (!dtmp) {
    return NULL;
  }
  if (eckey) {
    EC_KEY_free(*eckey);
    *eckey = dtmp;
  }
  return dtmp;
}

EC_KEY *PEM_read_bio_ECPrivateKey(BIO *bp, EC_KEY **key, pem_password_cb *cb,
                                  void *u) {
  EVP_PKEY *pktmp;
  pktmp = PEM_read_bio_PrivateKey(bp, NULL, cb, u);
  return pkey_get_eckey(pktmp, key);  // will free pktmp
}

IMPLEMENT_PEM_write_cb(ECPrivateKey, EC_KEY, PEM_STRING_ECPRIVATEKEY,
                       ECPrivateKey)

IMPLEMENT_PEM_rw(EC_PUBKEY, EC_KEY, PEM_STRING_PUBLIC, EC_PUBKEY)
EC_KEY *PEM_read_ECPrivateKey(FILE *fp, EC_KEY **eckey, pem_password_cb *cb,
                              void *u) {
  EVP_PKEY *pktmp;
  pktmp = PEM_read_PrivateKey(fp, NULL, cb, u);
  return pkey_get_eckey(pktmp, eckey);  // will free pktmp
}


IMPLEMENT_PEM_rw_const(DHparams, DH, PEM_STRING_DHPARAMS, DHparams)

IMPLEMENT_PEM_rw(PUBKEY, EVP_PKEY, PEM_STRING_PUBLIC, PUBKEY)

EC_GROUP *PEM_read_bio_ECPKParameters(BIO *bio, EC_GROUP **out_group,
                                      pem_password_cb *cb, void *u) {
  uint8_t *data = NULL;
  long len;
  if (!PEM_bytes_read_bio(&data, &len, NULL, PEM_STRING_ECPARAMETERS, bio, cb,
                          u)) {
    return NULL;
  }

  const uint8_t *data_in = data;
  EC_GROUP *ret = d2i_ECPKParameters(out_group, &data_in, len);
  if (ret == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_ASN1_LIB);
  }
  OPENSSL_free(data);
  return ret;
}

int PEM_write_bio_ECPKParameters(BIO *out, const EC_GROUP *group) {
  int ret = 0;
  unsigned char *data = NULL;

  int buf_len = i2d_ECPKParameters(group, &data);
  if(data == NULL || buf_len < 0) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_ASN1_LIB);
    goto err;
  }

  if (PEM_write_bio(out, PEM_STRING_ECPARAMETERS, NULL, data, buf_len) <= 0) {
    goto err;
  }

  ret = 1;
err:
  OPENSSL_free(data);
  return ret;
}
