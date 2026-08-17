// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/asn1.h>
#include <openssl/cipher.h>
#include <openssl/evp.h>
#include <openssl/obj.h>
#include <openssl/x509.h>

#include "internal.h"
#include "openssl/x509v3.h"


long X509_get_version(const X509 *x509) {
  // The default version is v1(0).
  if (x509->cert_info->version == NULL) {
    return X509_VERSION_1;
  }
  return ASN1_INTEGER_get(x509->cert_info->version);
}

int X509_set_version(X509 *x, long version) {
  if (x == NULL) {
    return 0;
  }

  if (version < X509_VERSION_1 || version > X509_VERSION_3) {
    OPENSSL_PUT_ERROR(X509, X509_R_INVALID_VERSION);
    return 0;
  }

  // v1(0) is default and is represented by omitting the version.
  if (version == X509_VERSION_1) {
    ASN1_INTEGER_free(x->cert_info->version);
    x->cert_info->version = NULL;
    return 1;
  }

  if (x->cert_info->version == NULL) {
    x->cert_info->version = ASN1_INTEGER_new();
    if (x->cert_info->version == NULL) {
      return 0;
    }
  }
  return ASN1_INTEGER_set_int64(x->cert_info->version, version);
}

int X509_set_serialNumber(X509 *x, const ASN1_INTEGER *serial) {
  if (serial->type != V_ASN1_INTEGER && serial->type != V_ASN1_NEG_INTEGER) {
    OPENSSL_PUT_ERROR(ASN1, ASN1_R_WRONG_TYPE);
    return 0;
  }

  ASN1_INTEGER *in;
  if (x == NULL) {
    return 0;
  }
  in = x->cert_info->serialNumber;
  if (in != serial) {
    in = ASN1_INTEGER_dup(serial);
    if (in != NULL) {
      ASN1_INTEGER_free(x->cert_info->serialNumber);
      x->cert_info->serialNumber = in;
    }
  }
  return in != NULL;
}

int X509_set_issuer_name(X509 *x, X509_NAME *name) {
  if ((x == NULL) || (x->cert_info == NULL)) {
    return 0;
  }
  return (X509_NAME_set(&x->cert_info->issuer, name));
}

int X509_set_subject_name(X509 *x, X509_NAME *name) {
  if ((x == NULL) || (x->cert_info == NULL)) {
    return 0;
  }
  return (X509_NAME_set(&x->cert_info->subject, name));
}

int X509_set1_notBefore(X509 *x, const ASN1_TIME *tm) {
  ASN1_TIME *in;

  if ((x == NULL) || (x->cert_info->validity == NULL)) {
    return 0;
  }
  in = x->cert_info->validity->notBefore;
  if (in != tm) {
    in = ASN1_STRING_dup(tm);
    if (in != NULL) {
      ASN1_TIME_free(x->cert_info->validity->notBefore);
      x->cert_info->validity->notBefore = in;
    }
  }
  return in != NULL;
}

int X509_set_notBefore(X509 *x, const ASN1_TIME *tm) {
  return X509_set1_notBefore(x, tm);
}

const ASN1_TIME *X509_get0_notBefore(const X509 *x) {
  return x->cert_info->validity->notBefore;
}

ASN1_TIME *X509_getm_notBefore(X509 *x) {
  // Note this function takes a const |X509| pointer in OpenSSL. We require
  // non-const as this allows mutating |x|. If it comes up for compatibility,
  // we can relax this.
  return x->cert_info->validity->notBefore;
}

ASN1_TIME *X509_get_notBefore(const X509 *x509) {
  // In OpenSSL, this function is an alias for |X509_getm_notBefore|, but our
  // |X509_getm_notBefore| is const-correct. |X509_get_notBefore| was
  // originally a macro, so it needs to capture both get0 and getm use cases.
  return x509->cert_info->validity->notBefore;
}

int X509_set1_notAfter(X509 *x, const ASN1_TIME *tm) {
  ASN1_TIME *in;

  if ((x == NULL) || (x->cert_info->validity == NULL)) {
    return 0;
  }
  in = x->cert_info->validity->notAfter;
  if (in != tm) {
    in = ASN1_STRING_dup(tm);
    if (in != NULL) {
      ASN1_TIME_free(x->cert_info->validity->notAfter);
      x->cert_info->validity->notAfter = in;
    }
  }
  return in != NULL;
}

int X509_set_notAfter(X509 *x, const ASN1_TIME *tm) {
  return X509_set1_notAfter(x, tm);
}

const ASN1_TIME *X509_get0_notAfter(const X509 *x) {
  return x->cert_info->validity->notAfter;
}

ASN1_TIME *X509_getm_notAfter(X509 *x) {
  // Note this function takes a const |X509| pointer in OpenSSL. We require
  // non-const as this allows mutating |x|. If it comes up for compatibility,
  // we can relax this.
  return x->cert_info->validity->notAfter;
}

ASN1_TIME *X509_get_notAfter(const X509 *x509) {
  // In OpenSSL, this function is an alias for |X509_getm_notAfter|, but our
  // |X509_getm_notAfter| is const-correct. |X509_get_notAfter| was
  // originally a macro, so it needs to capture both get0 and getm use cases.
  return x509->cert_info->validity->notAfter;
}

void X509_get0_uids(const X509 *x509, const ASN1_BIT_STRING **out_issuer_uid,
                    const ASN1_BIT_STRING **out_subject_uid) {
  if (out_issuer_uid != NULL) {
    *out_issuer_uid = x509->cert_info->issuerUID;
  }
  if (out_subject_uid != NULL) {
    *out_subject_uid = x509->cert_info->subjectUID;
  }
}

int X509_set_pubkey(X509 *x, EVP_PKEY *pkey) {
  if ((x == NULL) || (x->cert_info == NULL)) {
    return 0;
  }
  return (X509_PUBKEY_set(&(x->cert_info->key), pkey));
}

const STACK_OF(X509_EXTENSION) *X509_get0_extensions(const X509 *x) {
  return x->cert_info->extensions;
}

const X509_ALGOR *X509_get0_tbs_sigalg(const X509 *x) {
  return x->cert_info->signature;
}

X509_PUBKEY *X509_get_X509_PUBKEY(const X509 *x509) {
  return x509->cert_info->key;
}

static int X509_SIG_INFO_get(const X509_SIG_INFO *sig_info, int *digest_nid,
                             int *pubkey_nid, int *sec_bits, uint32_t *flags) {
  if (sig_info == NULL) {
    OPENSSL_PUT_ERROR(X509, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  if (digest_nid != NULL) {
    *digest_nid = sig_info->digest_nid;
  }
  if (pubkey_nid != NULL) {
    *pubkey_nid = sig_info->pubkey_nid;
  }
  if (sec_bits != NULL) {
    *sec_bits = sig_info->sec_bits;
  }
  if (flags != NULL) {
    *flags = sig_info->flags;
  }
  return (sig_info->flags & X509_SIG_INFO_VALID) != 0;
}

int X509_get_signature_info(X509 *x509, int *digest_nid, int *pubkey_nid,
                            int *sec_bits, uint32_t *flags) {
  if (x509 == NULL) {
    OPENSSL_PUT_ERROR(X509, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  // The return value of |x509v3_cache_extensions| is not checked because
  // |X509_get_signature_info|'s function contract does not encapsulate failures
  // if any invalid extensions do exist.
  x509v3_cache_extensions(x509);
  return X509_SIG_INFO_get(&x509->sig_info, digest_nid, pubkey_nid, sec_bits,
                           flags);
}

int x509_init_signature_info(X509 *x509) {
  int pubkey_nid, digest_nid;
  const EVP_MD *md;

  x509->sig_info.digest_nid = NID_undef;
  x509->sig_info.pubkey_nid = NID_undef;
  x509->sig_info.sec_bits = -1;
  x509->sig_info.flags = 0;
  if (!OBJ_find_sigid_algs(OBJ_obj2nid(x509->sig_alg->algorithm), &digest_nid,
                           &pubkey_nid) ||
      pubkey_nid == NID_undef) {
    OPENSSL_PUT_ERROR(X509, X509_R_UNKNOWN_SIGID_ALGS);
    return 0;
  }
  x509->sig_info.pubkey_nid = pubkey_nid;
  x509->sig_info.digest_nid = digest_nid;
  x509->sig_info.flags |= X509_SIG_INFO_VALID;

  md = EVP_get_digestbynid(digest_nid);
  if (md == NULL) {
    // Some valid signature algorithms have an undefined digest. See
    // crypto/obj/obj_xref.c.
    return 1;
  }
  // Security bits: half number of bits in digest.
  x509->sig_info.sec_bits = (int)EVP_MD_size(md) * 4;

  switch (digest_nid) {
    case NID_sha1:
    case NID_sha256:
    case NID_sha384:
    case NID_sha512:
      x509->sig_info.flags |= X509_SIG_INFO_TLS;
  }
  return 1;
}
