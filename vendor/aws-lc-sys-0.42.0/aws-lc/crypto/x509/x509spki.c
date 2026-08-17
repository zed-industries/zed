// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <limits.h>
#include <string.h>

#include <openssl/base64.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/x509.h>
#include "internal.h"

int NETSCAPE_SPKI_set_pubkey(NETSCAPE_SPKI *x, EVP_PKEY *pkey) {
  if ((x == NULL) || (x->spkac == NULL)) {
    return 0;
  }
  return (X509_PUBKEY_set(&(x->spkac->pubkey), pkey));
}

EVP_PKEY *NETSCAPE_SPKI_get_pubkey(const NETSCAPE_SPKI *x) {
  if ((x == NULL) || (x->spkac == NULL)) {
    return NULL;
  }
  return (X509_PUBKEY_get(x->spkac->pubkey));
}

// Load a Netscape SPKI from a base64 encoded string

NETSCAPE_SPKI *NETSCAPE_SPKI_b64_decode(const char *str, ossl_ssize_t len) {
  unsigned char *spki_der;
  const unsigned char *p;
  size_t spki_len;
  NETSCAPE_SPKI *spki;

  if (!str || (str[0] == 0)) {
    return NULL;
  }
  if (len <= 0) {
    len = strlen(str);
  }
  if (len > SHRT_MAX) {
    OPENSSL_PUT_ERROR(X509, ERR_R_OVERFLOW);
    return NULL;
  }
  if (!EVP_DecodedLength(&spki_len, len)) {
    OPENSSL_PUT_ERROR(X509, X509_R_BASE64_DECODE_ERROR);
    return NULL;
  }
  if (!(spki_der = OPENSSL_malloc(spki_len))) {
    return NULL;
  }
  if (!EVP_DecodeBase64(spki_der, &spki_len, spki_len, (const uint8_t *)str,
                        len)) {
    OPENSSL_PUT_ERROR(X509, X509_R_BASE64_DECODE_ERROR);
    OPENSSL_free(spki_der);
    return NULL;
  }
  p = spki_der;
  spki = d2i_NETSCAPE_SPKI(NULL, &p, spki_len);
  OPENSSL_free(spki_der);
  return spki;
}

// Generate a base64 encoded string from an SPKI

char *NETSCAPE_SPKI_b64_encode(NETSCAPE_SPKI *spki) {
  unsigned char *der_spki, *p;
  char *b64_str;
  size_t b64_len;
  int der_len;
  der_len = i2d_NETSCAPE_SPKI(spki, NULL);
  if (!EVP_EncodedLength(&b64_len, der_len)) {
    OPENSSL_PUT_ERROR(X509, ERR_R_OVERFLOW);
    return NULL;
  }
  der_spki = OPENSSL_malloc(der_len);
  if (der_spki == NULL) {
    return NULL;
  }
  b64_str = OPENSSL_malloc(b64_len);
  if (b64_str == NULL) {
    OPENSSL_free(der_spki);
    return NULL;
  }
  p = der_spki;
  i2d_NETSCAPE_SPKI(spki, &p);
  EVP_EncodeBlock((unsigned char *)b64_str, der_spki, der_len);
  OPENSSL_free(der_spki);
  return b64_str;
}

int NETSCAPE_SPKI_print(BIO *out, NETSCAPE_SPKI *spki) {
  if (out == NULL || spki == NULL || spki->spkac == NULL ||
      spki->spkac->pubkey == NULL || spki->sig_algor == NULL ||
      spki->sig_algor->algorithm == NULL || spki->signature == NULL ||
      spki->signature->data == NULL) {
    OPENSSL_PUT_ERROR(X509, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  BIO_printf(out, "Netscape SPKI:\n");

  // Print out public key algorithm and contents.
  ASN1_OBJECT *spkioid;
  X509_PUBKEY_get0_param(&spkioid, NULL, NULL, NULL, spki->spkac->pubkey);
  int spkioid_nid = OBJ_obj2nid(spkioid);
  BIO_printf(out, "  Public Key Algorithm: %s\n",
             (spkioid_nid == NID_undef) ? "UNKNOWN" : OBJ_nid2ln(spkioid_nid));
  EVP_PKEY *pkey = X509_PUBKEY_get0(spki->spkac->pubkey);
  if (pkey == NULL) {
    BIO_printf(out, "  Unable to load public key\n");
  } else {
    EVP_PKEY_print_public(out, pkey, 4, NULL);
  }

  ASN1_IA5STRING *chal = spki->spkac->challenge;
  if (chal != NULL && chal->length != 0) {
    BIO_printf(out, "  Challenge String: %.*s\n", chal->length, chal->data);
  }

  // Print out signature algorithm and contents.
  BIO_printf(out, "  Signature Algorithm: %s",
             (OBJ_obj2nid(spki->sig_algor->algorithm) == NID_undef)
                 ? "UNKNOWN"
                 : OBJ_nid2ln(OBJ_obj2nid(spki->sig_algor->algorithm)));
  for (int i = 0; i < spki->signature->length; i++) {
    if ((i % 18) == 0) {
      BIO_printf(out, "\n      ");
    }
    BIO_printf(out, "%02x%s", (unsigned char)spki->signature->data[i],
               ((i + 1) == spki->signature->length) ? "" : ":");
  }
  BIO_write(out, "\n", 1);
  return 1;
}
