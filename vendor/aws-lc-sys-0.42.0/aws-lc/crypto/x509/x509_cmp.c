// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <string.h>

#include <openssl/asn1.h>
#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/md5.h>
#include <openssl/obj.h>
#include <openssl/sha.h>
#include <openssl/stack.h>
#include <openssl/x509.h>

#include "../internal.h"
#include "internal.h"


int X509_issuer_name_cmp(const X509 *a, const X509 *b) {
  return (X509_NAME_cmp(a->cert_info->issuer, b->cert_info->issuer));
}

int X509_subject_name_cmp(const X509 *a, const X509 *b) {
  return (X509_NAME_cmp(a->cert_info->subject, b->cert_info->subject));
}

int X509_CRL_cmp(const X509_CRL *a, const X509_CRL *b) {
  return (X509_NAME_cmp(a->crl->issuer, b->crl->issuer));
}

int X509_CRL_match(const X509_CRL *a, const X509_CRL *b) {
  return OPENSSL_memcmp(a->crl_hash, b->crl_hash, SHA256_DIGEST_LENGTH);
}

X509_NAME *X509_get_issuer_name(const X509 *a) {
  return a->cert_info->issuer;
}

uint32_t X509_issuer_name_hash(X509 *x) {
  return X509_NAME_hash(x->cert_info->issuer);
}

uint32_t X509_issuer_name_hash_old(X509 *x) {
  return (X509_NAME_hash_old(x->cert_info->issuer));
}

X509_NAME *X509_get_subject_name(const X509 *a) {
  return a->cert_info->subject;
}

ASN1_INTEGER *X509_get_serialNumber(X509 *a) {
  return a->cert_info->serialNumber;
}

const ASN1_INTEGER *X509_get0_serialNumber(const X509 *x509) {
  return x509->cert_info->serialNumber;
}

uint32_t X509_subject_name_hash(X509 *x) {
  return X509_NAME_hash(x->cert_info->subject);
}

uint32_t X509_subject_name_hash_old(X509 *x) {
  return X509_NAME_hash_old(x->cert_info->subject);
}

// Compare two certificates: they must be identical for this to work. NB:
// Although "cmp" operations are generally prototyped to take "const"
// arguments (eg. for use in STACKs), the way X509 handling is - these
// operations may involve ensuring the hashes are up-to-date and ensuring
// certain cert information is cached. So this is the point where the
// "depth-first" constification tree has to halt with an evil cast.
int X509_cmp(const X509 *a, const X509 *b) {
  // Fill in the |cert_hash| fields.
  //
  // TODO(davidben): This may fail, in which case the the hash will be all
  // zeros. This produces a consistent comparison (failures are sticky), but
  // not a good one. OpenSSL now returns -2, but this is not a consistent
  // comparison and may cause misbehaving sorts by transitivity. For now, we
  // retain the old OpenSSL behavior, which was to ignore the error. See
  // https://crbug.com/boringssl/355.
  x509v3_cache_extensions((X509 *)a);
  x509v3_cache_extensions((X509 *)b);

  return OPENSSL_memcmp(a->cert_hash, b->cert_hash, SHA256_DIGEST_LENGTH);
}

int X509_NAME_cmp(const X509_NAME *a, const X509_NAME *b) {
  int ret;

  // Ensure canonical encoding is present and up to date

  if (!a->canon_enc || a->modified) {
    ret = i2d_X509_NAME((X509_NAME *)a, NULL);
    if (ret < 0) {
      return -2;
    }
  }

  if (!b->canon_enc || b->modified) {
    ret = i2d_X509_NAME((X509_NAME *)b, NULL);
    if (ret < 0) {
      return -2;
    }
  }

  ret = a->canon_enclen - b->canon_enclen;

  if (ret) {
    return ret;
  }

  return OPENSSL_memcmp(a->canon_enc, b->canon_enc, a->canon_enclen);
}

uint32_t X509_NAME_hash(X509_NAME *x) {
  // Make sure the X509_NAME structure contains a valid cached encoding.
  if (i2d_X509_NAME(x, NULL) < 0) {
    return 0;
  }

  uint8_t md[SHA_DIGEST_LENGTH];
  SHA1(x->canon_enc, x->canon_enclen, md);
  return CRYPTO_load_u32_le(md);
}

// I now DER encode the name and hash it.  Since I cache the DER encoding,
// this is reasonably efficient.

uint32_t X509_NAME_hash_old(X509_NAME *x) {
  // Make sure the X509_NAME structure contains a valid cached encoding.
  if (i2d_X509_NAME(x, NULL) < 0) {
    return 0;
  }

  uint8_t md[SHA_DIGEST_LENGTH];
  MD5((const uint8_t *)x->bytes->data, x->bytes->length, md);
  return CRYPTO_load_u32_le(md);
}

X509 *X509_find_by_issuer_and_serial(const STACK_OF(X509) *sk, X509_NAME *name,
                                     const ASN1_INTEGER *serial) {
  if (serial->type != V_ASN1_INTEGER && serial->type != V_ASN1_NEG_INTEGER) {
    return NULL;
  }

  for (size_t i = 0; i < sk_X509_num(sk); i++) {
    X509 *x509 = sk_X509_value(sk, i);
    if (ASN1_INTEGER_cmp(X509_get0_serialNumber(x509), serial) == 0 &&
        X509_NAME_cmp(X509_get_issuer_name(x509), name) == 0) {
      return x509;
    }
  }
  return NULL;
}

X509 *X509_find_by_subject(const STACK_OF(X509) *sk, X509_NAME *name) {
  for (size_t i = 0; i < sk_X509_num(sk); i++) {
    X509 *x509 = sk_X509_value(sk, i);
    if (X509_NAME_cmp(X509_get_subject_name(x509), name) == 0) {
      return x509;
    }
  }
  return NULL;
}

EVP_PKEY *X509_get0_pubkey(const X509 *x) {
  if (x == NULL) {
    return NULL;
  }
  return X509_PUBKEY_get0(x->cert_info->key);
}

EVP_PKEY *X509_get_pubkey(const X509 *x) {
  if (x == NULL) {
    return NULL;
  }
  return X509_PUBKEY_get(x->cert_info->key);
}

ASN1_BIT_STRING *X509_get0_pubkey_bitstr(const X509 *x) {
  if (!x) {
    return NULL;
  }
  return x->cert_info->key->public_key;
}

int X509_check_private_key(const X509 *x, const EVP_PKEY *k) {
  const EVP_PKEY *xk = X509_get0_pubkey(x);
  if (xk == NULL) {
    return 0;
  }

  int ret = EVP_PKEY_cmp(xk, k);
  if (ret > 0) {
    return 1;
  }

  switch (ret) {
    case 0:
      OPENSSL_PUT_ERROR(X509, X509_R_KEY_VALUES_MISMATCH);
      return 0;
    case -1:
      OPENSSL_PUT_ERROR(X509, X509_R_KEY_TYPE_MISMATCH);
      return 0;
    case -2:
      OPENSSL_PUT_ERROR(X509, X509_R_UNKNOWN_KEY_TYPE);
      return 0;
  }

  return 0;
}

// Not strictly speaking an "up_ref" as a STACK doesn't have a reference
// count but it has the same effect by duping the STACK and upping the ref of
// each X509 structure.
STACK_OF(X509) *X509_chain_up_ref(STACK_OF(X509) *chain) {
  STACK_OF(X509) *ret = sk_X509_dup(chain);
  if (ret == NULL) {
    return NULL;
  }
  for (size_t i = 0; i < sk_X509_num(ret); i++) {
    X509_up_ref(sk_X509_value(ret, i));
  }
  return ret;
}
