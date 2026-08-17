// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/x509.h>

#include <limits.h>

#include <openssl/asn1.h>
#include <openssl/asn1t.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/obj.h>

#include "../internal.h"
#include "internal.h"


static void x509_pubkey_changed(X509_PUBKEY *pub) {
  EVP_PKEY_free(pub->pkey);
  pub->pkey = NULL;

  // Re-encode the |X509_PUBKEY| to DER and parse it with EVP's APIs.
  uint8_t *spki = NULL;
  int spki_len = i2d_X509_PUBKEY(pub, &spki);
  if (spki_len < 0) {
    goto err;
  }

  CBS cbs;
  CBS_init(&cbs, spki, (size_t)spki_len);
  EVP_PKEY *pkey = EVP_parse_public_key(&cbs);
  if (pkey == NULL || CBS_len(&cbs) != 0) {
    EVP_PKEY_free(pkey);
    goto err;
  }

  pub->pkey = pkey;

err:
  OPENSSL_free(spki);
  // If the operation failed, clear errors. An |X509_PUBKEY| whose key we cannot
  // parse is still a valid SPKI. It just cannot be converted to an |EVP_PKEY|.
  ERR_clear_error();
}

static int pubkey_cb(int operation, ASN1_VALUE **pval, const ASN1_ITEM *it,
                     void *exarg) {
  X509_PUBKEY *pubkey = (X509_PUBKEY *)*pval;
  if (operation == ASN1_OP_FREE_POST) {
    EVP_PKEY_free(pubkey->pkey);
  } else if (operation == ASN1_OP_D2I_POST) {
    x509_pubkey_changed(pubkey);
  }
  return 1;
}

ASN1_SEQUENCE_cb(X509_PUBKEY, pubkey_cb) = {
    ASN1_SIMPLE(X509_PUBKEY, algor, X509_ALGOR),
    ASN1_SIMPLE(X509_PUBKEY, public_key, ASN1_BIT_STRING),
} ASN1_SEQUENCE_END_cb(X509_PUBKEY, X509_PUBKEY)

IMPLEMENT_ASN1_FUNCTIONS_const(X509_PUBKEY)

int X509_PUBKEY_set(X509_PUBKEY **x, EVP_PKEY *pkey) {
  X509_PUBKEY *pk = NULL;
  uint8_t *spki = NULL;
  size_t spki_len;

  if (x == NULL) {
    return 0;
  }

  CBB cbb;
  if (!CBB_init(&cbb, 0) ||  //
      !EVP_marshal_public_key(&cbb, pkey) ||
      !CBB_finish(&cbb, &spki, &spki_len) ||  //
      spki_len > LONG_MAX) {
    CBB_cleanup(&cbb);
    OPENSSL_PUT_ERROR(X509, X509_R_PUBLIC_KEY_ENCODE_ERROR);
    goto error;
  }

  const uint8_t *p = spki;
  pk = d2i_X509_PUBKEY(NULL, &p, (long)spki_len);
  if (pk == NULL || p != spki + spki_len) {
    OPENSSL_PUT_ERROR(X509, X509_R_PUBLIC_KEY_DECODE_ERROR);
    goto error;
  }

  OPENSSL_free(spki);
  X509_PUBKEY_free(*x);
  *x = pk;

  return 1;
error:
  X509_PUBKEY_free(pk);
  OPENSSL_free(spki);
  return 0;
}

EVP_PKEY *X509_PUBKEY_get0(const X509_PUBKEY *key) {
  if (key == NULL) {
    return NULL;
  }

  if (key->pkey == NULL) {
    OPENSSL_PUT_ERROR(X509, X509_R_PUBLIC_KEY_DECODE_ERROR);
    return NULL;
  }

  return key->pkey;
}

EVP_PKEY *X509_PUBKEY_get(const X509_PUBKEY *key) {
  EVP_PKEY *pkey = X509_PUBKEY_get0(key);
  if (pkey != NULL) {
    EVP_PKEY_up_ref(pkey);
  }
  return pkey;
}

int X509_PUBKEY_set0_param(X509_PUBKEY *pub, ASN1_OBJECT *obj, int param_type,
                           void *param_value, uint8_t *key, int key_len) {
  if (!X509_ALGOR_set0(pub->algor, obj, param_type, param_value)) {
    return 0;
  }

  ASN1_STRING_set0(pub->public_key, key, key_len);
  // Set the number of unused bits to zero.
  pub->public_key->flags &= ~(ASN1_STRING_FLAG_BITS_LEFT | 0x07);
  pub->public_key->flags |= ASN1_STRING_FLAG_BITS_LEFT;

  x509_pubkey_changed(pub);
  return 1;
}

int X509_PUBKEY_get0_param(ASN1_OBJECT **out_obj, const uint8_t **out_key,
                           int *out_key_len, X509_ALGOR **out_alg,
                           X509_PUBKEY *pub) {
  if (out_obj != NULL) {
    *out_obj = pub->algor->algorithm;
  }
  if (out_key != NULL) {
    *out_key = pub->public_key->data;
    *out_key_len = pub->public_key->length;
  }
  if (out_alg != NULL) {
    *out_alg = pub->algor;
  }
  return 1;
}

const ASN1_BIT_STRING *X509_PUBKEY_get0_public_key(const X509_PUBKEY *pub) {
  return pub->public_key;
}
