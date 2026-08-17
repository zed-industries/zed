// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/pem.h>

#include <stdio.h>
#include <string.h>

#include <openssl/dh.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/pkcs8.h>
#include <openssl/rand.h>
#include <openssl/x509.h>
#include "../fipsmodule/evp/internal.h"

EVP_PKEY *PEM_read_bio_PrivateKey(BIO *bp, EVP_PKEY **x, pem_password_cb *cb,
                                  void *u) {
  char *nm = NULL;
  const unsigned char *p = NULL;
  unsigned char *data = NULL;
  long len;
  EVP_PKEY *ret = NULL;

  if (!PEM_bytes_read_bio(&data, &len, &nm, PEM_STRING_EVP_PKEY, bp, cb, u)) {
    return NULL;
  }
  p = data;

  if (strcmp(nm, PEM_STRING_PKCS8INF) == 0) {
    PKCS8_PRIV_KEY_INFO *p8inf;
    p8inf = d2i_PKCS8_PRIV_KEY_INFO(NULL, &p, len);
    if (!p8inf) {
      goto p8err;
    }
    ret = EVP_PKCS82PKEY(p8inf);
    if (x) {
      if (*x) {
        EVP_PKEY_free((EVP_PKEY *)*x);
      }
      *x = ret;
    }
    PKCS8_PRIV_KEY_INFO_free(p8inf);
  } else if (strcmp(nm, PEM_STRING_PKCS8) == 0) {
    PKCS8_PRIV_KEY_INFO *p8inf;
    X509_SIG *p8;
    char psbuf[PEM_BUFSIZE];
    p8 = d2i_X509_SIG(NULL, &p, len);
    if (!p8) {
      goto p8err;
    }

    if (!cb) {
      cb = PEM_def_callback;
    }
    int pass_len = cb(psbuf, PEM_BUFSIZE, 0, u);
    if (pass_len < 0) {
      OPENSSL_PUT_ERROR(PEM, PEM_R_BAD_PASSWORD_READ);
      X509_SIG_free(p8);
      goto err;
    }
    p8inf = PKCS8_decrypt(p8, psbuf, pass_len);
    X509_SIG_free(p8);
    OPENSSL_cleanse(psbuf, pass_len);
    if (!p8inf) {
      goto p8err;
    }
    ret = EVP_PKCS82PKEY(p8inf);
    if (x) {
      if (*x) {
        EVP_PKEY_free((EVP_PKEY *)*x);
      }
      *x = ret;
    }
    PKCS8_PRIV_KEY_INFO_free(p8inf);
  } else if (strcmp(nm, PEM_STRING_RSA) == 0) {
    // TODO(davidben): d2i_PrivateKey parses PKCS#8 along with the
    // standalone format. This and the cases below probably should not
    // accept PKCS#8.
    ret = d2i_PrivateKey(EVP_PKEY_RSA, x, &p, len);
  } else if (strcmp(nm, PEM_STRING_EC) == 0) {
    ret = d2i_PrivateKey(EVP_PKEY_EC, x, &p, len);
  } else if (strcmp(nm, PEM_STRING_DSA) == 0) {
    ret = d2i_PrivateKey(EVP_PKEY_DSA, x, &p, len);
  }
p8err:
  if (ret == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_ASN1_LIB);
  }

err:
  OPENSSL_free(nm);
  OPENSSL_free(data);
  return ret;
}

int PEM_write_bio_PrivateKey(BIO *bp, EVP_PKEY *x, const EVP_CIPHER *enc,
                             const unsigned char *pass, int pass_len,
                             pem_password_cb *cb, void *u) {
  return PEM_write_bio_PKCS8PrivateKey(bp, x, enc, (const char *)pass, pass_len,
                                       cb, u);
}

EVP_PKEY *PEM_read_bio_Parameters(BIO *bio, EVP_PKEY **pkey) {
  if (bio == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  char *nm = NULL;
  unsigned char *data = NULL;
  long len;
  if (!PEM_bytes_read_bio(&data, &len, &nm, PEM_STRING_PARAMETERS, bio, 0,
                          NULL)) {
    return NULL;
  }
  const unsigned char *data_const = data;

  // Implementing the ASN1 logic here allows us to decouple the pem logic for
  // |EVP_PKEY|. These correspond to the historical |param_decode|
  // |EVP_PKEY_ASN1_METHOD| hooks in OpenSSL.
  EVP_PKEY *ret = EVP_PKEY_new();
  if (ret == NULL) {
    goto err;
  }
  if (strcmp(nm, PEM_STRING_ECPARAMETERS) == 0) {
    EC_KEY *ec_key = d2i_ECParameters(NULL, &data_const, len);
    if (ec_key == NULL || !EVP_PKEY_assign_EC_KEY(ret, ec_key)) {
      OPENSSL_PUT_ERROR(EVP, ERR_R_EC_LIB);
      EC_KEY_free(ec_key);
      goto err;
    }
  } else if (strcmp(nm, PEM_STRING_DSAPARAMS) == 0) {
    DSA *dsa = d2i_DSAparams(NULL, &data_const, len);
    if (dsa == NULL || !EVP_PKEY_assign_DSA(ret, dsa)) {
      OPENSSL_PUT_ERROR(EVP, ERR_R_DSA_LIB);
      DSA_free(dsa);
      goto err;
    }
  } else if (strcmp(nm, PEM_STRING_DHPARAMS) == 0) {
    DH *dh = d2i_DHparams(NULL, &data_const, len);
    if (dh == NULL || !EVP_PKEY_assign_DH(ret, dh)) {
      OPENSSL_PUT_ERROR(EVP, ERR_R_DH_LIB);
      DH_free(dh);
      goto err;
    }
  } else {
    goto err;
  }

  if (pkey != NULL) {
    EVP_PKEY_free(*pkey);
    *pkey = ret;
  }

  OPENSSL_free(nm);
  OPENSSL_free(data);
  return ret;

err:
  EVP_PKEY_free(ret);
  OPENSSL_free(nm);
  OPENSSL_free(data);
  return NULL;
}

static int i2d_ECParameters_void(const void *key, uint8_t **out) {
  return i2d_ECParameters((EC_KEY *)key, out);
}

static int i2d_DSAparams_void(const void *key, uint8_t **out) {
  return i2d_DSAparams((DSA *)key, out);
}

static int i2d_DHparams_void(const void *key, uint8_t **out) {
  return i2d_DHparams((DH *)key, out);
}

int PEM_write_bio_Parameters(BIO *bio, EVP_PKEY *pkey) {
  if (bio == NULL || pkey == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  // Implementing the ASN1 logic here allows us to decouple the pem logic for
  // |EVP_PKEY|. These correspond to the historical |param_encode|
  // |EVP_PKEY_ASN1_METHOD| hooks in OpenSSL.
  char pem_str[80];
  switch (pkey->type) {
    case EVP_PKEY_EC:
      BIO_snprintf(pem_str, 80, PEM_STRING_ECPARAMETERS);
      return PEM_ASN1_write_bio(i2d_ECParameters_void, pem_str, bio,
                                pkey->pkey.ec, NULL, NULL, 0, 0, NULL);
    case EVP_PKEY_DSA:
      BIO_snprintf(pem_str, 80, PEM_STRING_DSAPARAMS);
      return PEM_ASN1_write_bio(i2d_DSAparams_void, pem_str, bio,
                                pkey->pkey.dsa, NULL, NULL, 0, 0, NULL);
    case EVP_PKEY_DH:
      BIO_snprintf(pem_str, 80, PEM_STRING_DHPARAMS);
      return PEM_ASN1_write_bio(i2d_DHparams_void, pem_str, bio, pkey->pkey.dh,
                                NULL, NULL, 0, 0, NULL);
    default:
      return 0;
  }
}

static int i2d_PrivateKey_void(const void *key, uint8_t **out) {
  return i2d_PrivateKey((const EVP_PKEY *)key, out);
}

int PEM_write_bio_PrivateKey_traditional(BIO *bp, EVP_PKEY *x,
                                         const EVP_CIPHER *enc,
                                         unsigned char *kstr, int klen,
                                         pem_password_cb *cb, void *u) {
  if (bp == NULL || x == NULL || x->ameth == NULL ||
      x->ameth->pem_str == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  char pem_str[80];
  BIO_snprintf(pem_str, sizeof(pem_str), "%s PRIVATE KEY", x->ameth->pem_str);
  return PEM_ASN1_write_bio(i2d_PrivateKey_void, pem_str, bp, x, enc, kstr,
                            klen, cb, u);
}

EVP_PKEY *PEM_read_PrivateKey(FILE *fp, EVP_PKEY **x, pem_password_cb *cb,
                              void *u) {
  BIO *b = BIO_new_fp(fp, BIO_NOCLOSE);
  if (b == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_BUF_LIB);
    return NULL;
  }
  EVP_PKEY *ret = PEM_read_bio_PrivateKey(b, x, cb, u);
  BIO_free(b);
  return ret;
}

int PEM_write_PrivateKey(FILE *fp, EVP_PKEY *x, const EVP_CIPHER *enc,
                         const unsigned char *pass, int pass_len,
                         pem_password_cb *cb, void *u) {
  BIO *b = BIO_new_fp(fp, BIO_NOCLOSE);
  if (b == NULL) {
    OPENSSL_PUT_ERROR(PEM, ERR_R_BUF_LIB);
    return 0;
  }
  int ret = PEM_write_bio_PrivateKey(b, x, enc, pass, pass_len, cb, u);
  BIO_free(b);
  return ret;
}
