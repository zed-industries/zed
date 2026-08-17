// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/pkcs7.h>

#include <limits.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/pem.h>
#include <openssl/pool.h>
#include <openssl/rand.h>
#include <openssl/stack.h>
#include <openssl/x509.h>

#include "../bytestring/internal.h"
#include "../fipsmodule/digest/internal.h"
#include "../internal.h"
#include "internal.h"

OPENSSL_BEGIN_ALLOW_DEPRECATED

// 1.2.840.113549.1.7.1
static const uint8_t kPKCS7Data[] = {0x2a, 0x86, 0x48, 0x86, 0xf7,
                                     0x0d, 0x01, 0x07, 0x01};

// 1.2.840.113549.1.7.2
static const uint8_t kPKCS7SignedData[] = {0x2a, 0x86, 0x48, 0x86, 0xf7,
                                           0x0d, 0x01, 0x07, 0x02};

// pkcs7_parse_header reads the non-certificate/non-CRL prefix of a PKCS#7
// SignedData blob from |cbs| and sets |*out| to point to the rest of the
// input. If the input is in BER format, then |*der_bytes| will be set to a
// pointer that needs to be freed by the caller once they have finished
// processing |*out| (which will be pointing into |*der_bytes|).
//
// It returns one on success or zero on error. On error, |*der_bytes| is
// NULL.
int pkcs7_parse_header(uint8_t **der_bytes, CBS *out, CBS *cbs) {
  CBS in, content_info, content_type, wrapped_signed_data, signed_data;
  uint64_t version;

  // The input may be in BER format.
  *der_bytes = NULL;
  if (!CBS_asn1_ber_to_der(cbs, &in, der_bytes) ||
      // See https://tools.ietf.org/html/rfc2315#section-7
      !CBS_get_asn1(&in, &content_info, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1(&content_info, &content_type, CBS_ASN1_OBJECT)) {
    goto err;
  }

  if (!CBS_mem_equal(&content_type, kPKCS7SignedData,
                     sizeof(kPKCS7SignedData))) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NOT_PKCS7_SIGNED_DATA);
    goto err;
  }

  // See https://tools.ietf.org/html/rfc2315#section-9.1
  if (!CBS_get_asn1(&content_info, &wrapped_signed_data,
                    CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0) ||
      !CBS_get_asn1(&wrapped_signed_data, &signed_data, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&signed_data, &version) ||
      !CBS_get_asn1(&signed_data, NULL /* digests */, CBS_ASN1_SET) ||
      !CBS_get_asn1(&signed_data, NULL /* content */, CBS_ASN1_SEQUENCE)) {
    goto err;
  }

  if (version < 1) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_BAD_PKCS7_VERSION);
    goto err;
  }

  CBS_init(out, CBS_data(&signed_data), CBS_len(&signed_data));
  return 1;

err:
  OPENSSL_free(*der_bytes);
  *der_bytes = NULL;
  return 0;
}

int PKCS7_get_raw_certificates(STACK_OF(CRYPTO_BUFFER) *out_certs, CBS *cbs,
                               CRYPTO_BUFFER_POOL *pool) {
  CBS signed_data, certificates;
  uint8_t *der_bytes = NULL;
  int ret = 0, has_certificates;
  const size_t initial_certs_len = sk_CRYPTO_BUFFER_num(out_certs);

  // See https://tools.ietf.org/html/rfc2315#section-9.1
  if (!pkcs7_parse_header(&der_bytes, &signed_data, cbs) ||
      !CBS_get_optional_asn1(
          &signed_data, &certificates, &has_certificates,
          CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0)) {
    goto err;
  }

  if (!has_certificates) {
    CBS_init(&certificates, NULL, 0);
  }

  while (CBS_len(&certificates) > 0) {
    CBS cert;
    if (!CBS_get_asn1_element(&certificates, &cert, CBS_ASN1_SEQUENCE)) {
      goto err;
    }

    CRYPTO_BUFFER *buf = CRYPTO_BUFFER_new_from_CBS(&cert, pool);
    if (buf == NULL || !sk_CRYPTO_BUFFER_push(out_certs, buf)) {
      CRYPTO_BUFFER_free(buf);
      goto err;
    }
  }

  ret = 1;

err:
  OPENSSL_free(der_bytes);

  if (!ret) {
    while (sk_CRYPTO_BUFFER_num(out_certs) != initial_certs_len) {
      CRYPTO_BUFFER *buf = sk_CRYPTO_BUFFER_pop(out_certs);
      CRYPTO_BUFFER_free(buf);
    }
  }

  return ret;
}

static int pkcs7_bundle_raw_certificates_cb(CBB *out, const void *arg) {
  const STACK_OF(CRYPTO_BUFFER) *certs = arg;
  CBB certificates;

  // See https://tools.ietf.org/html/rfc2315#section-9.1
  if (!CBB_add_asn1(out, &certificates,
                    CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0)) {
    return 0;
  }

  for (size_t i = 0; i < sk_CRYPTO_BUFFER_num(certs); i++) {
    CRYPTO_BUFFER *cert = sk_CRYPTO_BUFFER_value(certs, i);
    if (!CBB_add_bytes(&certificates, CRYPTO_BUFFER_data(cert),
                       CRYPTO_BUFFER_len(cert))) {
      return 0;
    }
  }

  // |certificates| is a implicitly-tagged SET OF.
  return CBB_flush_asn1_set_of(&certificates) && CBB_flush(out);
}

int PKCS7_bundle_raw_certificates(CBB *out,
                                  const STACK_OF(CRYPTO_BUFFER) *certs) {
  return pkcs7_add_signed_data(out, /*digest_algos_cb=*/NULL,
                               pkcs7_bundle_raw_certificates_cb,
                               /*signer_infos_cb=*/NULL, certs);
}

int pkcs7_add_signed_data(CBB *out,
                          int (*digest_algos_cb)(CBB *out, const void *arg),
                          int (*cert_crl_cb)(CBB *out, const void *arg),
                          int (*signer_infos_cb)(CBB *out, const void *arg),
                          const void *arg) {
  CBB outer_seq, oid, wrapped_seq, seq, version_bytes, digest_algos_set,
      content_info, signer_infos;

  // See https://tools.ietf.org/html/rfc2315#section-7
  if (!CBB_add_asn1(out, &outer_seq, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&outer_seq, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, kPKCS7SignedData, sizeof(kPKCS7SignedData)) ||
      !CBB_add_asn1(&outer_seq, &wrapped_seq,
                    CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0) ||
      // See https://tools.ietf.org/html/rfc2315#section-9.1
      !CBB_add_asn1(&wrapped_seq, &seq, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&seq, &version_bytes, CBS_ASN1_INTEGER) ||
      !CBB_add_u8(&version_bytes, 1) ||
      !CBB_add_asn1(&seq, &digest_algos_set, CBS_ASN1_SET) ||
      (digest_algos_cb != NULL && !digest_algos_cb(&digest_algos_set, arg)) ||
      !CBB_add_asn1(&seq, &content_info, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1(&content_info, &oid, CBS_ASN1_OBJECT) ||
      !CBB_add_bytes(&oid, kPKCS7Data, sizeof(kPKCS7Data)) ||
      (cert_crl_cb != NULL && !cert_crl_cb(&seq, arg)) ||
      !CBB_add_asn1(&seq, &signer_infos, CBS_ASN1_SET) ||
      (signer_infos_cb != NULL && !signer_infos_cb(&signer_infos, arg))) {
    return 0;
  }

  return CBB_flush(out);
}

int PKCS7_set_type(PKCS7 *p7, int type) {
  if (p7 == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  ASN1_OBJECT *obj = OBJ_nid2obj(type);
  if (obj == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNSUPPORTED_CONTENT_TYPE);
    return 0;
  }

  // Free any previously-installed content using the current |p7->type| so the
  // correct destructor runs on the existing union member, instead of the new
  // type's destructor running on type-confused memory.
  if (p7->d.ptr != NULL) {
    switch (OBJ_obj2nid(p7->type)) {
      case NID_pkcs7_signed:
        PKCS7_SIGNED_free(p7->d.sign);
        break;
      case NID_pkcs7_digest:
        PKCS7_DIGEST_free(p7->d.digest);
        break;
      case NID_pkcs7_data:
        ASN1_OCTET_STRING_free(p7->d.data);
        break;
      case NID_pkcs7_signedAndEnveloped:
        PKCS7_SIGN_ENVELOPE_free(p7->d.signed_and_enveloped);
        break;
      case NID_pkcs7_enveloped:
        PKCS7_ENVELOPE_free(p7->d.enveloped);
        break;
      case NID_pkcs7_encrypted:
        PKCS7_ENCRYPT_free(p7->d.encrypted);
        break;
      default:
        ASN1_TYPE_free(p7->d.other);
        break;
    }
    p7->d.ptr = NULL;
  }

  switch (type) {
    case NID_pkcs7_signed:
      p7->type = obj;
      p7->d.sign = PKCS7_SIGNED_new();
      if (p7->d.sign == NULL) {
        return 0;
      }
      if (!ASN1_INTEGER_set(p7->d.sign->version, 1)) {
        PKCS7_SIGNED_free(p7->d.sign);
        p7->d.sign = NULL;
        return 0;
      }
      break;
    case NID_pkcs7_digest:
      p7->type = obj;
      p7->d.digest = PKCS7_DIGEST_new();
      if (p7->d.digest == NULL) {
        return 0;
      }
      if (!ASN1_INTEGER_set(p7->d.digest->version, 0)) {
        PKCS7_DIGEST_free(p7->d.digest);
        p7->d.digest = NULL;
        return 0;
      }
      break;
    case NID_pkcs7_data:
      p7->type = obj;
      p7->d.data = ASN1_OCTET_STRING_new();
      if (p7->d.data == NULL) {
        return 0;
      }
      break;
    case NID_pkcs7_signedAndEnveloped:
      p7->type = obj;
      p7->d.signed_and_enveloped = PKCS7_SIGN_ENVELOPE_new();
      if (p7->d.signed_and_enveloped == NULL) {
        return 0;
      }
      if (!ASN1_INTEGER_set(p7->d.signed_and_enveloped->version, 1)) {
        PKCS7_SIGN_ENVELOPE_free(p7->d.signed_and_enveloped);
        p7->d.signed_and_enveloped = NULL;
        return 0;
      }
      p7->d.signed_and_enveloped->enc_data->content_type =
          OBJ_nid2obj(NID_pkcs7_data);
      break;
    case NID_pkcs7_enveloped:
      p7->type = obj;
      p7->d.enveloped = PKCS7_ENVELOPE_new();
      if (p7->d.enveloped == NULL) {
        return 0;
      }
      if (!ASN1_INTEGER_set(p7->d.enveloped->version, 0)) {
        PKCS7_ENVELOPE_free(p7->d.enveloped);
        p7->d.enveloped = NULL;
        return 0;
      }
      p7->d.enveloped->enc_data->content_type = OBJ_nid2obj(NID_pkcs7_data);
      break;
    case NID_pkcs7_encrypted:
      p7->type = obj;
      p7->d.encrypted = PKCS7_ENCRYPT_new();
      if (p7->d.encrypted == NULL) {
        return 0;
      }
      if (!ASN1_INTEGER_set(p7->d.encrypted->version, 0)) {
        PKCS7_ENCRYPT_free(p7->d.encrypted);
        p7->d.encrypted = NULL;
        return 0;
      }
      p7->d.encrypted->enc_data->content_type = OBJ_nid2obj(NID_pkcs7_data);
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNSUPPORTED_CONTENT_TYPE);
      return 0;
  }
  return 1;
}

int PKCS7_set_cipher(PKCS7 *p7, const EVP_CIPHER *cipher) {
  if (p7 == NULL || cipher == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  if (EVP_get_cipherbynid(EVP_CIPHER_nid(cipher)) == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_CIPHER_HAS_NO_OBJECT_IDENTIFIER);
    return 0;
  }

  PKCS7_ENC_CONTENT *ec;
  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_signedAndEnveloped:
      ec = p7->d.signed_and_enveloped->enc_data;
      break;
    case NID_pkcs7_enveloped:
      ec = p7->d.enveloped->enc_data;
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_WRONG_CONTENT_TYPE);
      return 0;
  }

  ec->cipher = cipher;
  return 1;
}

int PKCS7_set_content(PKCS7 *p7, PKCS7 *p7_data) {
  if (p7 == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_signed:
      PKCS7_free(p7->d.sign->contents);
      p7->d.sign->contents = p7_data;
      break;
    case NID_pkcs7_digest:
      PKCS7_free(p7->d.digest->contents);
      p7->d.digest->contents = p7_data;
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNSUPPORTED_CONTENT_TYPE);
      return 0;
  }
  return 1;
}

int PKCS7_content_new(PKCS7 *p7, int type) {
  PKCS7 *ret = PKCS7_new();
  if (ret == NULL) {
    goto err;
  }
  if (!PKCS7_set_type(ret, type)) {
    goto err;
  }
  if (!PKCS7_set_content(p7, ret)) {
    goto err;
  }
  return 1;
err:
  PKCS7_free(ret);
  return 0;
}

int PKCS7_add_recipient_info(PKCS7 *p7, PKCS7_RECIP_INFO *ri) {
  STACK_OF(PKCS7_RECIP_INFO) *sk;

  if (p7 == NULL || ri == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_signedAndEnveloped:
      sk = p7->d.signed_and_enveloped->recipientinfo;
      break;
    case NID_pkcs7_enveloped:
      sk = p7->d.enveloped->recipientinfo;
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_WRONG_CONTENT_TYPE);
      return 0;
  }

  if (!sk_PKCS7_RECIP_INFO_push(sk, ri)) {
    return 0;
  }
  return 1;
}

int PKCS7_add_signer(PKCS7 *p7, PKCS7_SIGNER_INFO *p7i) {
  GUARD_PTR(p7);
  GUARD_PTR(p7i);
  ASN1_OBJECT *obj;
  X509_ALGOR *alg;
  STACK_OF(PKCS7_SIGNER_INFO) *signer_sk;
  STACK_OF(X509_ALGOR) *md_sk;

  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_signed:
      signer_sk = p7->d.sign->signer_info;
      md_sk = p7->d.sign->md_algs;
      break;
    case NID_pkcs7_signedAndEnveloped:
      signer_sk = p7->d.signed_and_enveloped->signer_info;
      md_sk = p7->d.signed_and_enveloped->md_algs;
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_WRONG_CONTENT_TYPE);
      return 0;
  }

  obj = p7i->digest_alg->algorithm;
  // If the digest is not currently listed, add it
  int alg_found = 0;
  for (size_t i = 0; i < sk_X509_ALGOR_num(md_sk); i++) {
    alg = sk_X509_ALGOR_value(md_sk, i);
    if (OBJ_cmp(obj, alg->algorithm) == 0) {
      alg_found = 1;
      break;
    }
  }
  if (!alg_found) {
    if ((alg = X509_ALGOR_new()) == NULL ||
        (alg->parameter = ASN1_TYPE_new()) == NULL) {
      X509_ALGOR_free(alg);
      OPENSSL_PUT_ERROR(PKCS7, ERR_R_ASN1_LIB);
      return 0;
    }
    // If there is a constant copy of the ASN1 OBJECT in libcrypto, then
    // use that.  Otherwise, use a dynamically duplicated copy.
    int nid = OBJ_obj2nid(obj);
    if (nid != NID_undef) {
      alg->algorithm = OBJ_nid2obj(nid);
    } else {
      alg->algorithm = OBJ_dup(obj);
    }
    alg->parameter->type = V_ASN1_NULL;
    if (alg->algorithm == NULL || !sk_X509_ALGOR_push(md_sk, alg)) {
      X509_ALGOR_free(alg);
      return 0;
    }
  }

  if (!sk_PKCS7_SIGNER_INFO_push(signer_sk, p7i)) {
    return 0;
  }
  return 1;
}

static ASN1_TYPE *get_attribute(STACK_OF(X509_ATTRIBUTE) *sk, int nid) {
  for (size_t i = 0; i < sk_X509_ATTRIBUTE_num(sk); i++) {
    X509_ATTRIBUTE *attr = sk_X509_ATTRIBUTE_value(sk, i);
    ASN1_OBJECT *obj = X509_ATTRIBUTE_get0_object(attr);
    if (OBJ_obj2nid(obj) == nid) {
      return X509_ATTRIBUTE_get0_type(attr, 0);
    }
  }
  return NULL;
}

ASN1_TYPE *PKCS7_get_signed_attribute(const PKCS7_SIGNER_INFO *si, int nid) {
  if (si == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }
  return get_attribute(si->auth_attr, nid);
}

static ASN1_OCTET_STRING *PKCS7_digest_from_attributes(
    STACK_OF(X509_ATTRIBUTE) *sk) {
  if (sk == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }
  ASN1_TYPE *astype = get_attribute(sk, NID_pkcs9_messageDigest);
  if (astype == NULL || astype->type != V_ASN1_OCTET_STRING) {
    return NULL;
  }
  return astype->value.octet_string;
}

STACK_OF(PKCS7_SIGNER_INFO) *PKCS7_get_signer_info(PKCS7 *p7) {
  if (p7 == NULL || p7->d.ptr == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_signed:
      return p7->d.sign->signer_info;
    case NID_pkcs7_signedAndEnveloped:
      return p7->d.signed_and_enveloped->signer_info;
    default:
      return NULL;
  }
}

int PKCS7_SIGNER_INFO_set(PKCS7_SIGNER_INFO *p7i, X509 *x509, EVP_PKEY *pkey,
                          const EVP_MD *dgst) {
  if (!p7i || !x509 || !pkey || !dgst) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  } else if (!ASN1_INTEGER_set(p7i->version, 1)) {
    return 0;
  } else if (!X509_NAME_set(&p7i->issuer_and_serial->issuer,
                            X509_get_issuer_name(x509))) {
    return 0;
  }

  ASN1_INTEGER_free(p7i->issuer_and_serial->serial);
  if (!(p7i->issuer_and_serial->serial =
            ASN1_INTEGER_dup(X509_get0_serialNumber(x509)))) {
    return 0;
  }

  // NOTE: OpenSSL does not free |p7i->pkey| before setting it. we do so here
  // to avoid potential memory leaks.
  EVP_PKEY_free(p7i->pkey);
  EVP_PKEY_up_ref(pkey);
  p7i->pkey = pkey;

  if (!X509_ALGOR_set0(p7i->digest_alg, OBJ_nid2obj(EVP_MD_type(dgst)),
                       V_ASN1_NULL, NULL)) {
    return 0;
  }

  switch (EVP_PKEY_id(pkey)) {
    case EVP_PKEY_EC:
    case EVP_PKEY_DH: {
      int snid, hnid;
      X509_ALGOR *alg1, *alg2;
      PKCS7_SIGNER_INFO_get0_algs(p7i, NULL, &alg1, &alg2);
      if (alg1 == NULL || alg1->algorithm == NULL) {
        return 0;
      }
      hnid = OBJ_obj2nid(alg1->algorithm);
      if (hnid == NID_undef ||
          !OBJ_find_sigid_by_algs(&snid, hnid, EVP_PKEY_id(pkey)) ||
          !X509_ALGOR_set0(alg2, OBJ_nid2obj(snid), V_ASN1_UNDEF, NULL)) {
        return 0;
      }
      break;
    }
    case EVP_PKEY_RSA:
    case EVP_PKEY_RSA_PSS: {
      X509_ALGOR *alg = NULL;
      PKCS7_SIGNER_INFO_get0_algs(p7i, NULL, NULL, &alg);
      if (alg != NULL) {
        return X509_ALGOR_set0(alg, OBJ_nid2obj(NID_rsaEncryption), V_ASN1_NULL,
                               NULL);
      }
      break;
    }
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_SIGNING_NOT_SUPPORTED_FOR_THIS_KEY_TYPE);
      return 0;
  }

  return 1;
}

int PKCS7_RECIP_INFO_set(PKCS7_RECIP_INFO *p7i, X509 *x509) {
  if (!p7i || !x509) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }
  if (!ASN1_INTEGER_set(p7i->version, 0)) {
    return 0;
  } else if (!X509_NAME_set(&p7i->issuer_and_serial->issuer,
                            X509_get_issuer_name(x509))) {
    return 0;
  }

  ASN1_INTEGER_free(p7i->issuer_and_serial->serial);
  if (!(p7i->issuer_and_serial->serial =
            ASN1_INTEGER_dup(X509_get0_serialNumber(x509)))) {
    return 0;
  }

  EVP_PKEY *pkey = X509_get0_pubkey(x509);
  if (pkey == NULL) {
    return 0;
  }

  if (EVP_PKEY_id(pkey) == EVP_PKEY_RSA_PSS) {
    return 0;
  } else if (EVP_PKEY_id(pkey) == EVP_PKEY_RSA) {
    X509_ALGOR *alg;
    PKCS7_RECIP_INFO_get0_alg(p7i, &alg);
    if (!X509_ALGOR_set0(alg, OBJ_nid2obj(NID_rsaEncryption), V_ASN1_NULL,
                         NULL)) {
      return 0;
    }
  }

  // NOTE: OpenSSL does not free |p7i->cert| before setting it. we do so here
  // to avoid potential memory leaks.
  X509_free(p7i->cert);
  X509_up_ref(x509);
  p7i->cert = x509;

  return 1;
}

void PKCS7_SIGNER_INFO_get0_algs(PKCS7_SIGNER_INFO *si, EVP_PKEY **pk,
                                 X509_ALGOR **pdig, X509_ALGOR **psig) {
  if (!si) {
    return;
  }
  if (pk) {
    *pk = si->pkey;
  }
  if (pdig) {
    *pdig = si->digest_alg;
  }
  if (psig) {
    *psig = si->digest_enc_alg;
  }
}

void PKCS7_RECIP_INFO_get0_alg(PKCS7_RECIP_INFO *ri, X509_ALGOR **penc) {
  if (!ri) {
    return;
  }
  if (penc) {
    *penc = ri->key_enc_algor;
  }
}

static ASN1_OCTET_STRING *PKCS7_get_octet_string(PKCS7 *p7) {
  GUARD_PTR(p7);
  if (PKCS7_type_is_data(p7)) {
    return p7->d.data;
  }
  return NULL;
}

static int pkcs7_bio_add_digest(BIO **pbio, X509_ALGOR *alg) {
  GUARD_PTR(pbio);
  GUARD_PTR(alg);
  BIO *btmp = NULL;

  const EVP_MD *md = EVP_get_digestbynid(OBJ_obj2nid(alg->algorithm));
  if (md == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNKNOWN_DIGEST_TYPE);
    goto err;
  }

  if ((btmp = BIO_new(BIO_f_md())) == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_BIO_LIB);
    goto err;
  }

  if (BIO_set_md(btmp, (EVP_MD *)md) <= 0) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_BIO_LIB);
    goto err;
  }
  if (*pbio == NULL) {
    *pbio = btmp;
  } else if (!BIO_push(*pbio, btmp)) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_BIO_LIB);
    goto err;
  }

  return 1;

err:
  BIO_free(btmp);
  return 0;
}

static int pkcs7_encode_rinfo(PKCS7_RECIP_INFO *ri, unsigned char *key,
                              int keylen) {
  GUARD_PTR(ri);
  GUARD_PTR(key);
  EVP_PKEY_CTX *pctx = NULL;
  EVP_PKEY *pkey = NULL;
  unsigned char *ek = NULL;
  int ret = 0;
  size_t eklen;

  if ((pkey = X509_get0_pubkey(ri->cert)) == NULL) {
    goto err;
  }

  pctx = EVP_PKEY_CTX_new(pkey, NULL);
  if (pctx == NULL || EVP_PKEY_encrypt_init(pctx) <= 0 ||
      EVP_PKEY_encrypt(pctx, NULL, &eklen, key, keylen) <= 0 ||
      (NULL == (ek = OPENSSL_malloc(eklen))) ||
      EVP_PKEY_encrypt(pctx, ek, &eklen, key, keylen) <= 0) {
    goto err;
  }

  ASN1_STRING_set0(ri->enc_key, ek, eklen);
  ek = NULL;  // NULL out |ek| ptr because |ri| takes ownership of the alloc

  ret = 1;

err:
  EVP_PKEY_CTX_free(pctx);
  OPENSSL_free(ek);
  return ret;
}

BIO *PKCS7_dataInit(PKCS7 *p7, BIO *bio) {
  GUARD_PTR(p7);
  BIO *out = NULL, *btmp = NULL;
  const EVP_CIPHER *evp_cipher = NULL;
  STACK_OF(PKCS7_RECIP_INFO) *rsk = NULL;
  STACK_OF(X509_ALGOR) *md_sk = NULL;
  X509_ALGOR *xalg = NULL;
  PKCS7_RECIP_INFO *ri = NULL;
  ASN1_OCTET_STRING *content = NULL;

  // The content field in the PKCS7 ContentInfo is optional, but that only
  // applies to inner content (precisely, detached signatures). When reading
  // content, missing outer content is therefore treated as an error. When
  // creating content, PKCS7_content_new() must be called before calling this
  // method, so a NULL p7->d is always an error.
  if (p7->d.ptr == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NO_CONTENT);
    return NULL;
  }

  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_signed:
      md_sk = p7->d.sign->md_algs;
      content = PKCS7_get_octet_string(p7->d.sign->contents);
      break;
    case NID_pkcs7_signedAndEnveloped:
      md_sk = p7->d.signed_and_enveloped->md_algs;
      rsk = p7->d.signed_and_enveloped->recipientinfo;
      xalg = p7->d.signed_and_enveloped->enc_data->algorithm;
      evp_cipher = p7->d.signed_and_enveloped->enc_data->cipher;
      if (evp_cipher == NULL) {
        OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_CIPHER_NOT_INITIALIZED);
        goto err;
      }
      break;
    case NID_pkcs7_enveloped:
      rsk = p7->d.enveloped->recipientinfo;
      xalg = p7->d.enveloped->enc_data->algorithm;
      evp_cipher = p7->d.enveloped->enc_data->cipher;
      if (evp_cipher == NULL) {
        OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_CIPHER_NOT_INITIALIZED);
        goto err;
      }
      break;
    case NID_pkcs7_digest:
      content = PKCS7_get_octet_string(p7->d.digest->contents);
      if (!pkcs7_bio_add_digest(&out, p7->d.digest->digest_alg)) {
        goto err;
      }
      break;
    case NID_pkcs7_data:
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNSUPPORTED_CONTENT_TYPE);
      goto err;
  }


  if (md_sk != NULL && sk_X509_ALGOR_num(md_sk) > SHRT_MAX) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_OVERFLOW);
    goto err;
  }
  for (size_t i = 0; i < sk_X509_ALGOR_num(md_sk); i++) {
    if (!pkcs7_bio_add_digest(&out, sk_X509_ALGOR_value(md_sk, i))) {
      goto err;
    }
  }

  if (evp_cipher != NULL) {
    unsigned char key[EVP_MAX_KEY_LENGTH];
    unsigned char iv[EVP_MAX_IV_LENGTH];
    int keylen, ivlen;
    EVP_CIPHER_CTX *ctx;

    if ((btmp = BIO_new(BIO_f_cipher())) == NULL) {
      OPENSSL_PUT_ERROR(PKCS7, ERR_R_BIO_LIB);
      goto err;
    }
    if (!BIO_get_cipher_ctx(btmp, &ctx)) {
      OPENSSL_PUT_ERROR(PKCS7, ERR_R_BIO_LIB);
      goto err;
    }
    keylen = EVP_CIPHER_key_length(evp_cipher);
    ivlen = EVP_CIPHER_iv_length(evp_cipher);
    ASN1_OBJECT_free(xalg->algorithm);
    xalg->algorithm = OBJ_nid2obj(EVP_CIPHER_nid(evp_cipher));
    if (ivlen > 0) {
      AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(iv, ivlen));
    }
    if (keylen > 0) {
      AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(key, keylen));
    }

    if (EVP_CipherInit_ex(ctx, evp_cipher, NULL, key, iv, /*enc*/ 1) <= 0) {
      OPENSSL_cleanse(key, keylen);
      goto err;
    }

    if (ivlen > 0) {
      ASN1_TYPE_free(xalg->parameter);
      xalg->parameter = ASN1_TYPE_new();
      if (xalg->parameter == NULL) {
        goto err;
      }
      xalg->parameter->type = V_ASN1_OCTET_STRING;
      xalg->parameter->value.octet_string = ASN1_OCTET_STRING_new();
      if (xalg->parameter->value.octet_string == NULL) {
        goto err;
      }
      // Set |p7|'s parameter value to the IV
      if (!ASN1_OCTET_STRING_set(xalg->parameter->value.octet_string, iv,
                                 ivlen)) {
        goto err;
      }
    }

    for (size_t i = 0; i < sk_PKCS7_RECIP_INFO_num(rsk); i++) {
      ri = sk_PKCS7_RECIP_INFO_value(rsk, i);
      if (pkcs7_encode_rinfo(ri, key, keylen) <= 0) {
        OPENSSL_cleanse(key, keylen);
        goto err;
      }
    }
    OPENSSL_cleanse(key, keylen);

    if (out == NULL) {
      out = btmp;
    } else {
      BIO_push(out, btmp);
    }
    btmp = NULL;
  }

  if (bio == NULL) {
    bio = BIO_new(BIO_s_mem());
    if (bio == NULL) {
      goto err;
    }
    BIO_set_mem_eof_return(bio, /*eof_value*/ 0);
    if (!PKCS7_is_detached(p7) && content && content->length > 0) {
      // |bio| needs a copy of |content->data| instead of a pointer because the
      // data may be used after |content| has been freed
      if (BIO_write(bio, content->data, content->length) != content->length) {
        BIO_free(bio);
        goto err;
      }
    }
  }
  if (out) {
    BIO_push(out, bio);
  } else {
    out = bio;
  }
  return out;

err:
  BIO_free_all(out);
  BIO_free_all(btmp);
  return NULL;
}

int PKCS7_is_detached(PKCS7 *p7) {
  GUARD_PTR(p7);
  if (PKCS7_type_is_signed(p7)) {
    return (p7->d.sign == NULL || p7->d.sign->contents->d.ptr == NULL);
  }
  return 0;
}

int PKCS7_set_detached(PKCS7 *p7, int detach) {
  GUARD_PTR(p7);
  if (detach != 0 && detach != 1) {
    // |detach| is meant to be used as a boolean int.
    return 0;
  }

  if (PKCS7_type_is_signed(p7)) {
    if (p7->d.sign == NULL) {
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NO_CONTENT);
      return 0;
    }
    if (detach && PKCS7_type_is_data(p7->d.sign->contents)) {
      ASN1_OCTET_STRING_free(p7->d.sign->contents->d.data);
      p7->d.sign->contents->d.data = NULL;
    }
    return 1;
  } else {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_OPERATION_NOT_SUPPORTED_ON_THIS_TYPE);
    return 0;
  }
}

int PKCS7_get_detached(PKCS7 *p7) { return PKCS7_is_detached(p7); }


static BIO *pkcs7_find_digest(EVP_MD_CTX **pmd, BIO *bio, int nid) {
  GUARD_PTR(pmd);
  while (bio != NULL) {
    bio = BIO_find_type(bio, BIO_TYPE_MD);
    if (bio == NULL) {
      return NULL;
    }
    if (!BIO_get_md_ctx(bio, pmd) || *pmd == NULL) {
      OPENSSL_PUT_ERROR(PKCS7, ERR_R_INTERNAL_ERROR);
      return NULL;
    }
    if (EVP_MD_CTX_type(*pmd) == nid) {
      return bio;
    }
    bio = BIO_next(bio);
  }
  OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNABLE_TO_FIND_MESSAGE_DIGEST);
  return NULL;
}

int PKCS7_set_digest(PKCS7 *p7, const EVP_MD *md) {
  GUARD_PTR(p7);
  GUARD_PTR(md);
  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_digest:
      if (EVP_MD_nid(md) == NID_undef) {
        OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNKNOWN_DIGEST_TYPE);
        return 0;
      }
      if (p7->d.digest->digest_alg == NULL) {
        OPENSSL_PUT_ERROR(PKCS7, ERR_R_ASN1_LIB);
        return 0;
      }
      OPENSSL_free(p7->d.digest->digest_alg->parameter);
      if ((p7->d.digest->digest_alg->parameter = ASN1_TYPE_new()) == NULL) {
        OPENSSL_PUT_ERROR(PKCS7, ERR_R_ASN1_LIB);
        return 0;
      }
      p7->d.digest->md = md;
      p7->d.digest->digest_alg->parameter->type = V_ASN1_NULL;
      p7->d.digest->digest_alg->algorithm = OBJ_nid2obj(EVP_MD_nid(md));
      return 1;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_WRONG_CONTENT_TYPE);
      return 0;
  }
}


STACK_OF(PKCS7_RECIP_INFO) *PKCS7_get_recipient_info(PKCS7 *p7) {
  GUARD_PTR(p7);
  GUARD_PTR(p7->d.ptr);
  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_enveloped:
      return p7->d.enveloped->recipientinfo;
    default:
      return NULL;
  }
}

int PKCS7_dataFinal(PKCS7 *p7, BIO *bio) {
  GUARD_PTR(p7);
  GUARD_PTR(bio);
  int ret = 0;
  BIO *bio_tmp = NULL;
  PKCS7_SIGNER_INFO *si;
  EVP_MD_CTX *md_ctx = NULL, *md_ctx_tmp;
  STACK_OF(PKCS7_SIGNER_INFO) *si_sk = NULL;
  ASN1_OCTET_STRING *content = NULL;

  if (p7->d.ptr == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NO_CONTENT);
    return 0;
  }

  md_ctx_tmp = EVP_MD_CTX_new();
  if (md_ctx_tmp == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_EVP_LIB);
    return 0;
  }

  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_data:
      content = p7->d.data;
      break;
    case NID_pkcs7_signedAndEnveloped:
      si_sk = p7->d.signed_and_enveloped->signer_info;
      content = p7->d.signed_and_enveloped->enc_data->enc_data;
      if (content == NULL) {
        content = ASN1_OCTET_STRING_new();
        if (content == NULL) {
          OPENSSL_PUT_ERROR(PKCS7, ERR_R_ASN1_LIB);
          goto err;
        }
        p7->d.signed_and_enveloped->enc_data->enc_data = content;
      }
      break;
    case NID_pkcs7_enveloped:
      content = p7->d.enveloped->enc_data->enc_data;
      if (content == NULL) {
        content = ASN1_OCTET_STRING_new();
        if (content == NULL) {
          OPENSSL_PUT_ERROR(PKCS7, ERR_R_ASN1_LIB);
          goto err;
        }
        p7->d.enveloped->enc_data->enc_data = content;
      }
      break;
    case NID_pkcs7_signed:
      si_sk = p7->d.sign->signer_info;
      // clang-format off
      content = PKCS7_get_octet_string(p7->d.sign->contents);
      // If detached data then the content is excluded
      if (PKCS7_type_is_data(p7->d.sign->contents) && PKCS7_is_detached(p7)) {
        // clang-format on
        ASN1_OCTET_STRING_free(content);
        content = NULL;
        p7->d.sign->contents->d.data = NULL;
      }
      break;

    case NID_pkcs7_digest:
      content = PKCS7_get_octet_string(p7->d.digest->contents);
      // If detached data, then the content is excluded
      if (PKCS7_type_is_data(p7->d.digest->contents) && PKCS7_is_detached(p7)) {
        ASN1_OCTET_STRING_free(content);
        content = NULL;
        p7->d.digest->contents->d.data = NULL;
      }
      break;

    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNSUPPORTED_CONTENT_TYPE);
      goto err;
  }

  if (si_sk != NULL) {
    for (size_t ii = 0; ii < sk_PKCS7_SIGNER_INFO_num(si_sk); ii++) {
      si = sk_PKCS7_SIGNER_INFO_value(si_sk, ii);
      if (si == NULL || si->pkey == NULL) {
        continue;
      }
      int sign_nid = OBJ_obj2nid(si->digest_alg->algorithm);
      bio_tmp = pkcs7_find_digest(&md_ctx, bio, sign_nid);
      if (bio_tmp == NULL) {
        goto err;
      }
      // We now have the EVP_MD_CTX, lets do the signing.
      if (!EVP_MD_CTX_copy_ex(md_ctx_tmp, md_ctx)) {
        goto err;
      }
      // We don't currently support signed attributes
      if (sk_X509_ATTRIBUTE_num(si->auth_attr) > 0) {
        OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_PKCS7_DATASIGN);
        goto err;
      }
      unsigned char *abuf = NULL;
      unsigned int abuflen = EVP_PKEY_size(si->pkey);
      if (abuflen == 0 || (abuf = OPENSSL_malloc(abuflen)) == NULL) {
        goto err;
      }
      // |md_ctx_tmp| was initialized by |BIO_set_md| called on |bio|. Do not
      // modify that context, as it contains the content digest, and we need
      // to calculate the signature over it. Proceed to finalization.
      if (!EVP_SignFinal(md_ctx_tmp, abuf, &abuflen, si->pkey)) {
        OPENSSL_free(abuf);
        OPENSSL_PUT_ERROR(PKCS7, ERR_R_EVP_LIB);
        goto err;
      }
      ASN1_STRING_set0(si->enc_digest, abuf, abuflen);
    }
  } else if (OBJ_obj2nid(p7->type) == NID_pkcs7_digest) {
    unsigned char md_data[EVP_MAX_MD_SIZE];
    unsigned int md_len;
    if (!pkcs7_find_digest(&md_ctx, bio, EVP_MD_nid(p7->d.digest->md)) ||
        !EVP_DigestFinal_ex(md_ctx, md_data, &md_len) ||
        !ASN1_OCTET_STRING_set(p7->d.digest->digest, md_data, md_len)) {
      goto err;
    }
  }

  if (!PKCS7_is_detached(p7)) {
    if (content == NULL) {
      goto err;
    }
    const uint8_t *cont;
    size_t contlen;
    bio_tmp = BIO_find_type(bio, BIO_TYPE_MEM);
    if (bio_tmp == NULL) {
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNABLE_TO_FIND_MEM_BIO);
      goto err;
    }
    if (!BIO_mem_contents(bio_tmp, &cont, &contlen)) {
      goto err;
    }
    // Mark the BIO read only then we can use its copy of the data instead of
    // making an extra copy. |content| will intentionally point directly into
    // the relevant BIO's buffer, as the BIO's lifetime is owned by |p7|.
    BIO_set_flags(bio_tmp, BIO_FLAGS_MEM_RDONLY);
    BIO_set_mem_eof_return(bio_tmp, /*eof_value*/ 0);
    ASN1_STRING_set0(content, (unsigned char *)cont, contlen);
  }

  ret = 1;
err:
  EVP_MD_CTX_free(md_ctx_tmp);
  return ret;
}

// pkcs7_bio_copy_content copies the contents of |src| into |dst|. |src| must be
// non-null. If |dst| is null, |src| is read from until empty and its contents
// are discarded. If |dst| is present, only full copies are considered
// successful. It returns 1 on success and 0 on failure.
static int pkcs7_bio_copy_content(BIO *src, BIO *dst) {
  uint8_t buf[1024];
  int bytes_processed = 0, ret = 0;
  while ((bytes_processed = BIO_read(src, buf, sizeof(buf))) > 0) {
    if (dst && !BIO_write_all(dst, buf, bytes_processed)) {
      goto err;
    }
  }
  if (bytes_processed < 0 || (dst && !BIO_flush(dst))) {
    goto err;
  }
  ret = 1;
err:
  OPENSSL_cleanse(buf, sizeof(buf));
  return ret;
}

// PKCS7_final copies the contents of |data| into |p7| before finalizing |p7|.
int pkcs7_final(PKCS7 *p7, BIO *data) {
  BIO *p7bio;
  int ret = 0;

  if ((p7bio = PKCS7_dataInit(p7, NULL)) == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PKCS7_LIB);
    goto err;
  }

  if (!pkcs7_bio_copy_content(data, p7bio)) {
    goto err;
  }

  if (!PKCS7_dataFinal(p7, p7bio)) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PKCS7_LIB);
    goto err;
  }
  ret = 1;
err:
  BIO_free_all(p7bio);

  return ret;
}

PKCS7 *PKCS7_encrypt(STACK_OF(X509) *certs, BIO *in, const EVP_CIPHER *cipher,
                     int flags) {
  GUARD_PTR(certs);
  GUARD_PTR(in);
  GUARD_PTR(cipher);
  PKCS7 *p7;
  X509 *x509;

  if ((p7 = PKCS7_new()) == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PKCS7_LIB);
    return NULL;
  }
  if (!PKCS7_set_type(p7, NID_pkcs7_enveloped)) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_WRONG_CONTENT_TYPE);
    goto err;
  }
  if (!PKCS7_set_cipher(p7, cipher)) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_ERROR_SETTING_CIPHER);
    goto err;
  }

  for (size_t i = 0; i < sk_X509_num(certs); i++) {
    x509 = sk_X509_value(certs, i);
    if (!PKCS7_add_recipient(p7, x509)) {
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_ERROR_ADDING_RECIPIENT);
      goto err;
    }
  }

  if (pkcs7_final(p7, in)) {
    return p7;
  }

err:
  PKCS7_free(p7);
  return NULL;
}

static int pkcs7_decrypt_rinfo(unsigned char **ek_out, PKCS7_RECIP_INFO *ri,
                               EVP_PKEY *pkey) {
  GUARD_PTR(ri);
  GUARD_PTR(ek_out);
  unsigned char *ek = NULL;
  int ret = 0;

  EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new(pkey, /*engine*/ NULL);
  if (ctx == NULL || !EVP_PKEY_decrypt_init(ctx)) {
    goto err;
  }
  size_t len;
  if (!EVP_PKEY_decrypt(ctx, NULL, &len, ri->enc_key->data,
                        ri->enc_key->length) ||
      (ek = OPENSSL_malloc(len)) == NULL) {
    OPENSSL_PUT_ERROR(EVP, ERR_R_EVP_LIB);
    goto err;
  }

  int ok =
      EVP_PKEY_decrypt(ctx, ek, &len, ri->enc_key->data, ri->enc_key->length);
  // We return 0 any failure except for decryption failure. On decrypt failure,
  // we still need to set |ek| to NULL to signal decryption failure to callers
  // so they can use random bytes as content encryption key for MMA defense.
  if (!ok) {
    OPENSSL_free(ek);
    ek = NULL;
  }

  ret = 1;
  *ek_out = ek;

err:
  EVP_PKEY_CTX_free(ctx);
  return ret;
}

// pkcs7_cmp_ri is a comparison function, so it returns 0 if |ri| and |pcert|
// match and 1 if they do not.
static int pkcs7_cmp_ri(PKCS7_RECIP_INFO *ri, X509 *pcert) {
  if (ri == NULL || ri->issuer_and_serial == NULL || pcert == NULL) {
    return 1;
  }
  int ret =
      X509_NAME_cmp(ri->issuer_and_serial->issuer, X509_get_issuer_name(pcert));
  if (ret) {
    return ret;
  }
  return ASN1_INTEGER_cmp(X509_get0_serialNumber(pcert),
                          ri->issuer_and_serial->serial);
}

static BIO *pkcs7_data_decode(PKCS7 *p7, EVP_PKEY *pkey, X509 *pcert) {
  GUARD_PTR(p7);
  GUARD_PTR(pkey);
  BIO *out = NULL, *cipher_bio = NULL, *data_bio = NULL;
  ASN1_OCTET_STRING *data_body = NULL;
  const EVP_CIPHER *cipher = NULL;
  X509_ALGOR *enc_alg = NULL;
  STACK_OF(PKCS7_RECIP_INFO) *rsk = NULL;
  PKCS7_RECIP_INFO *ri = NULL;
  uint8_t *cek = NULL, *dummy_key = NULL;  // cek means "content encryption key"

  if (p7->d.ptr == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NO_CONTENT);
    return NULL;
  }

  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_enveloped:
      rsk = p7->d.enveloped->recipientinfo;
      enc_alg = p7->d.enveloped->enc_data->algorithm;
      if (enc_alg == NULL || enc_alg->parameter == NULL ||
          enc_alg->parameter->type != V_ASN1_OCTET_STRING ||
          enc_alg->parameter->value.octet_string == NULL ||
          enc_alg->algorithm == NULL) {
        OPENSSL_PUT_ERROR(PKCS7, ERR_R_PKCS7_LIB);
        goto err;
      }
      // |data_body| is NULL if the optional EncryptedContent is missing.
      data_body = p7->d.enveloped->enc_data->enc_data;
      cipher = EVP_get_cipherbynid(OBJ_obj2nid(enc_alg->algorithm));
      if (cipher == NULL) {
        OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNSUPPORTED_CIPHER_TYPE);
        goto err;
      }
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNSUPPORTED_CONTENT_TYPE);
      goto err;
  }

  // envelopedData must have data content to decrypt
  if (data_body == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NO_CONTENT);
    goto err;
  }

  if (sk_PKCS7_RECIP_INFO_num(rsk) > SHRT_MAX) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_OVERFLOW);
    goto err;
  }

  if ((cipher_bio = BIO_new(BIO_f_cipher())) == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_BIO_LIB);
    goto err;
  }

  // RFC 3218 provides an overview of, and mitigations for, the "Million Message
  // Attack" (MMA) on RSA encryption with PKCS-1 padding. Section 2.3 describes
  // implementor countermeasures. We implement the following countermeasures, as
  // does OpenSSL.
  //
  // 1. Do not branch on |cek| decryption failure when checking recip infos
  // 2. Clear error state after |cek| decrypt is attempted
  // 3. If no cek was decrypted, use same-size random bytes
  //    to output gibberish "plaintext"
  // 4. Always pay same allocation costs, regardless of |cek| decrypt result

  // If |pcert| was specified, find the matching recipient info
  if (pcert) {
    for (size_t ii = 0; ii < sk_PKCS7_RECIP_INFO_num(rsk); ii++) {
      ri = sk_PKCS7_RECIP_INFO_value(rsk, ii);
      // No decryption operation here, so we can return early without divulging
      // information that could be used in MMA.
      if (!pkcs7_cmp_ri(ri, pcert)) {
        break;
      }
#if !defined(BORINGSSL_UNSAFE_FUZZER_MODE)
      // For fuzz testing, we do not want to bail out early.
      ri = NULL;
#endif
    }
    if (ri == NULL) {
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NO_RECIPIENT_MATCHES_CERTIFICATE);
      goto err;
    }
    // |pkcs7_decrypt_rinfo| will only return false on critical failure, not
    // on decryption failure. Decryption check happens below, after we populate
    // |dummy_key| with random bytes.
    if (!pkcs7_decrypt_rinfo(&cek, ri, pkey)) {
      goto err;
    }
  } else {
    // Attempt to decrypt every recipient info. Don't exit early as
    // countermeasure for MMA.
    for (size_t ii = 0; ii < sk_PKCS7_RECIP_INFO_num(rsk); ii++) {
      ri = sk_PKCS7_RECIP_INFO_value(rsk, ii);
      uint8_t *tmp_cek;
      // |pkcs7_decrypt_rinfo| will only return false on critical failure, not
      // on decryption failure. Check whether |tmp_cek| is present after the
      // call to determine if decryption succeeded.
      if (!pkcs7_decrypt_rinfo(&tmp_cek, ri, pkey)) {
        goto err;
      }
      // OpenSSL sets encryption key to last successfully decrypted key. Copy
      // that behavior, but free previously allocated key memory.
      if (tmp_cek) {
        OPENSSL_free(cek);
        cek = tmp_cek;
      }
    }
  }
  // Clear any decryption errors to minimize behavioral difference under MMA
  ERR_clear_error();

  EVP_CIPHER_CTX *evp_ctx = NULL;
  if (!BIO_get_cipher_ctx(cipher_bio, &evp_ctx) ||
      !EVP_CipherInit_ex(evp_ctx, cipher, NULL, NULL, NULL, 0)) {
    goto err;
  }
  const int expected_iv_len = EVP_CIPHER_CTX_iv_length(evp_ctx);
  if (enc_alg->parameter->value.octet_string->length != expected_iv_len) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PKCS7_LIB);
    goto err;
  }
  uint8_t iv[EVP_MAX_IV_LENGTH];
  OPENSSL_memcpy(iv, enc_alg->parameter->value.octet_string->data,
                 expected_iv_len);
  if (!EVP_CipherInit_ex(evp_ctx, NULL, NULL, NULL, iv, 0)) {
    goto err;
  }
  // Get the key length from cipher context so we don't condition on |cek_len|
  int len = EVP_CIPHER_CTX_key_length(evp_ctx);
  if (!len) {
    goto err;
  }
  // Always generate random bytes for the dummy key, regardless of |cek| decrypt
  dummy_key = OPENSSL_malloc(len);
  if (dummy_key == NULL) {
    goto err;
  }
  AWSLC_ABORT_IF_NOT_ONE(RAND_bytes(dummy_key, len));
  // At this point, null |cek| indicates that no content encryption key was
  // successfully decrypted. We don't want to return early due to MMA. So, swap
  // in the dummy key and proceed. Content decryption result will be gibberish.
  if (cek == NULL) {
    cek = dummy_key;
    dummy_key = NULL;
  }

  if (!EVP_CipherInit_ex(evp_ctx, NULL, NULL, cek, NULL, 0)) {
    goto err;
  }

  OPENSSL_free(cek);
  OPENSSL_free(dummy_key);
  cek = NULL;
  dummy_key = NULL;
  out = cipher_bio;
  cipher_bio = NULL;

  // We verify data_body != NULL above
  if (data_body->length > 0) {
    data_bio = BIO_new_mem_buf(data_body->data, data_body->length);
  } else {
    data_bio = BIO_new(BIO_s_mem());
    if (data_bio == NULL || !BIO_set_mem_eof_return(data_bio, 0)) {
      goto err;
    }
  }
  if (data_bio == NULL) {
    goto err;
  }
  BIO_push(out, data_bio);
  return out;

err:
  OPENSSL_free(cek);
  OPENSSL_free(dummy_key);
  BIO_free_all(out);
  BIO_free_all(cipher_bio);
  BIO_free_all(data_bio);
  return NULL;
}

PKCS7_RECIP_INFO *PKCS7_add_recipient(PKCS7 *p7, X509 *x509) {
  GUARD_PTR(p7);
  GUARD_PTR(x509);
  PKCS7_RECIP_INFO *ri;
  if ((ri = PKCS7_RECIP_INFO_new()) == NULL ||
      !PKCS7_RECIP_INFO_set(ri, x509) || !PKCS7_add_recipient_info(p7, ri)) {
    PKCS7_RECIP_INFO_free(ri);
    return NULL;
  }
  return ri;
}

int PKCS7_decrypt(PKCS7 *p7, EVP_PKEY *pkey, X509 *cert, BIO *data,
                  int _flags) {
  GUARD_PTR(p7);
  GUARD_PTR(pkey);
  GUARD_PTR(data);
  BIO *bio = NULL;
  int ret = 0;

  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_enveloped:
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_WRONG_CONTENT_TYPE);
      goto err;
  }

  if (cert && !X509_check_private_key(cert, pkey)) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_PRIVATE_KEY_DOES_NOT_MATCH_CERTIFICATE);
    goto err;
  }

  if ((bio = pkcs7_data_decode(p7, pkey, cert)) == NULL ||
      !pkcs7_bio_copy_content(bio, data)) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_DECRYPT_ERROR);
    goto err;
  }

  // Check whether content decryption was successful
  if (1 != BIO_get_cipher_status(bio)) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_DECRYPT_ERROR);
    goto err;
  }

  ret = 1;

err:
  BIO_free_all(bio);
  return ret;
}

static STACK_OF(X509) *pkcs7_get0_certificates(const PKCS7 *p7) {
  GUARD_PTR(p7);
  GUARD_PTR(p7->d.ptr);
  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_signed:
      return p7->d.sign->cert;
    default:
      return NULL;
  }
}

STACK_OF(X509) *PKCS7_get0_signers(PKCS7 *p7, STACK_OF(X509) *certs,
                                          int flags) {
  GUARD_PTR(p7);
  STACK_OF(X509) *signers = NULL;
  X509 *signer = NULL;

  STACK_OF(X509) *included_certs = pkcs7_get0_certificates(p7);
  STACK_OF(PKCS7_SIGNER_INFO) *sinfos = PKCS7_get_signer_info(p7);

  if (sk_PKCS7_SIGNER_INFO_num(sinfos) <= 0) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NO_SIGNERS);
    return NULL;
  }

  if ((signers = sk_X509_new_null()) == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_CRYPTO_LIB);
    return NULL;
  }

  for (size_t i = 0; i < sk_PKCS7_SIGNER_INFO_num(sinfos); i++) {
    PKCS7_SIGNER_INFO *si = sk_PKCS7_SIGNER_INFO_value(sinfos, i);
    PKCS7_ISSUER_AND_SERIAL *ias = si->issuer_and_serial;
    // Prioritize |certs| passed by caller
    signer = X509_find_by_issuer_and_serial(certs, ias->issuer, ias->serial);
    if (!(flags & PKCS7_NOINTERN) && !signer) {
      signer = X509_find_by_issuer_and_serial(included_certs, ias->issuer,
                                              ias->serial);
    }
    if (!signer) {  // Signer cert not found in bundled/caller-specified certs
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_SIGNER_CERTIFICATE_NOT_FOUND);
      sk_X509_free(signers);
      return NULL;
    }
    if (!sk_X509_push(signers, signer)) {
      OPENSSL_PUT_ERROR(PKCS7, ERR_R_PKCS7_LIB);
      sk_X509_free(signers);
      return NULL;
    }
  }

  return signers;
}

static int pkcs7_x509_add_cert_new(STACK_OF(X509) **p_sk, X509 *cert) {
  GUARD_PTR(p_sk);
  if (*p_sk == NULL && (*p_sk = sk_X509_new_null()) == NULL) {
    goto err;
  }
  if (!sk_X509_push(*p_sk, cert)) {
    goto err;
  }
  return 1;
err:
  OPENSSL_PUT_ERROR(X509, ERR_R_CRYPTO_LIB);
  return 0;
}

static int pkcs7_x509_add_certs_new(STACK_OF(X509) **p_sk,
                                    STACK_OF(X509) *certs) {
  GUARD_PTR(p_sk);
  if (!certs) {  // |certs| can be null in the caller
    return 1;
  }
  for (size_t i = 0; i < sk_X509_num(certs); i++) {
    if (!pkcs7_x509_add_cert_new(p_sk, sk_X509_value(certs, i)))
      return 0;
  }
  return 1;
}

static int pkcs7_signature_verify(BIO *in_bio, PKCS7 *p7, PKCS7_SIGNER_INFO *si,
                                  X509 *signer) {
  GUARD_PTR(in_bio);
  GUARD_PTR(p7);
  GUARD_PTR(si);
  GUARD_PTR(si->digest_alg);
  GUARD_PTR(signer);
  int ret = 0;

  // Create a new |EVP_MD_CTX| to be consumed, so that the original |EVP_MD_CTX|
  // is still in a usable state.
  EVP_MD_CTX *mdc_tmp = EVP_MD_CTX_new();
  if (mdc_tmp == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_MALLOC_FAILURE);
    goto out;
  }

  const int md_type = OBJ_obj2nid(si->digest_alg->algorithm);
  if (md_type == NID_undef) {
    goto out;
  }
  EVP_MD_CTX *mdc = NULL;
  BIO *bio = in_bio;
  // There may be multiple MD-type BIOs in the chain, so iterate until we find
  // the BIO with MD type we're looking for.
  while (bio) {
    if ((bio = BIO_find_type(bio, BIO_TYPE_MD)) == NULL) {
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNABLE_TO_FIND_MESSAGE_DIGEST);
      goto out;
    }
    if (!BIO_get_md_ctx(bio, &mdc) || !mdc) {
      OPENSSL_PUT_ERROR(PKCS7, ERR_R_PKCS7_LIB);
      goto out;
    }
    if (EVP_MD_CTX_type(mdc) == md_type) {  // found it!
      break;
    }
    bio = BIO_next(bio);
  }

  // mdc is the digest ctx that we want, unless there are attributes, in
  // which case the digest is the signed attributes.
  if (!EVP_MD_CTX_copy_ex(mdc_tmp, mdc)) {
    goto out;
  }

  if (si->auth_attr != NULL && sk_X509_ATTRIBUTE_num(si->auth_attr) != 0) {
    unsigned char md_data[EVP_MAX_MD_SIZE], *abuf = NULL;
    unsigned int md_len;

    if (!EVP_DigestFinal_ex(mdc_tmp, md_data, &md_len)) {
      goto out;
    }
    ASN1_OCTET_STRING *message_digest =
        PKCS7_digest_from_attributes(si->auth_attr);
    if (message_digest == NULL) {
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_UNABLE_TO_FIND_MESSAGE_DIGEST);
      goto out;
    }
    if (message_digest->length != (int)md_len ||
        CRYPTO_memcmp(message_digest->data, md_data, md_len) != 0) {
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_DIGEST_FAILURE);
      goto out;
    }

    const EVP_MD *md = EVP_get_digestbynid(md_type);
    if (md == NULL || !EVP_VerifyInit_ex(mdc_tmp, md, NULL)) {
      goto out;
    }

    int alen = ASN1_item_i2d((ASN1_VALUE *)si->auth_attr, &abuf,
                             ASN1_ITEM_rptr(PKCS7_ATTR_VERIFY));
    if (alen <= 0 || abuf == NULL) {
      OPENSSL_PUT_ERROR(PKCS7, ERR_R_ASN1_LIB);
      goto out;
    }
    if (!EVP_VerifyUpdate(mdc_tmp, abuf, alen)) {
      OPENSSL_free(abuf);
      goto out;
    }
    OPENSSL_free(abuf);
  }

  EVP_PKEY *pkey = X509_get0_pubkey(signer);
  if (pkey == NULL) {
    goto out;
  }

  ASN1_OCTET_STRING *data_body = si->enc_digest;
  if (!EVP_VerifyFinal(mdc_tmp, data_body->data, data_body->length, pkey)) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_SIGNATURE_FAILURE);
    goto out;
  }

  ret = 1;

out:
  EVP_MD_CTX_free(mdc_tmp);
  return ret;
}

int PKCS7_verify(PKCS7 *p7, STACK_OF(X509) *certs, X509_STORE *store,
                 BIO *indata, BIO *outdata, int flags) {
  GUARD_PTR(p7);
  GUARD_PTR(store);
  STACK_OF(X509) *signers = NULL, *untrusted = NULL;
  X509_STORE_CTX *cert_ctx = NULL;
  BIO *p7bio = NULL;
  int ret = 0;

  if (!PKCS7_type_is_signed(p7)) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_WRONG_CONTENT_TYPE);
    goto out;
  }

  // If |p7| is detached, caller must supply data via |indata|
  if (PKCS7_is_detached(p7) && indata == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NO_CONTENT);
    goto out;
  }

  // We enforce OpenSSL's PKCS7_NO_DUAL_CONTENT flag in all cases for signed
  if (!PKCS7_is_detached(p7) && indata) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_CONTENT_AND_DATA_PRESENT);
    goto out;
  }

  STACK_OF(PKCS7_SIGNER_INFO) *sinfos = PKCS7_get_signer_info(p7);
  if (sinfos == NULL || sk_PKCS7_SIGNER_INFO_num(sinfos) == 0UL) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NO_SIGNATURES_ON_DATA);
    goto out;
  }
  if (sk_PKCS7_SIGNER_INFO_num(sinfos) > SHRT_MAX) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_OVERFLOW);
    goto out;
  }

  if ((signers = PKCS7_get0_signers(p7, certs, flags)) == NULL) {
    goto out;
  }

  if (!(flags & PKCS7_NOVERIFY)) {
    STACK_OF(X509) *included_certs = pkcs7_get0_certificates(p7);
    if ((cert_ctx = X509_STORE_CTX_new()) == NULL ||
        !pkcs7_x509_add_certs_new(&untrusted, certs) ||
        !pkcs7_x509_add_certs_new(&untrusted, included_certs)) {
      OPENSSL_PUT_ERROR(PKCS7, ERR_R_PKCS7_LIB);
      goto out;
    }

    for (size_t k = 0; k < sk_X509_num(signers); k++) {
      X509 *signer = sk_X509_value(signers, k);
      if (!X509_STORE_CTX_init(cert_ctx, store, signer, untrusted)) {
        OPENSSL_PUT_ERROR(PKCS7, ERR_R_X509_LIB);
        goto out;
      }
      if (!X509_STORE_CTX_set_default(cert_ctx, "smime_sign")) {
        goto out;
      }
      X509_STORE_CTX_set0_crls(cert_ctx, p7->d.sign->crl);
      // NOTE: unlike most of our functions, |X509_verify_cert| can return <= 0
      if (X509_verify_cert(cert_ctx) <= 0) {
#if !defined(BORINGSSL_UNSAFE_FUZZER_MODE)
        // For fuzz testing, we do not want to bail out early.
        OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_CERTIFICATE_VERIFY_ERROR);
        goto out;
#endif
      }
    }
  }

  // In copying data into out, we also read it through digest filters on |p7| to
  // calculate digest for verification.
  if ((p7bio = PKCS7_dataInit(p7, indata)) == NULL ||
      !pkcs7_bio_copy_content(p7bio, outdata)) {
    goto out;
  }

  // Verify signatures against signers
  for (size_t ii = 0; ii < sk_PKCS7_SIGNER_INFO_num(sinfos); ii++) {
    PKCS7_SIGNER_INFO *si = sk_PKCS7_SIGNER_INFO_value(sinfos, ii);
    X509 *signer = sk_X509_value(signers, ii);
    if (pkcs7_signature_verify(p7bio, p7, si, signer) != 1) {
#if !defined(BORINGSSL_UNSAFE_FUZZER_MODE)
      // For fuzz testing, we do not want to bail out early.
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_SIGNATURE_FAILURE);
      goto out;
#endif
    }
  }

  ret = 1;

out:
  X509_STORE_CTX_free(cert_ctx);
  // If |indata| was passed for detached signature, |PKCS7_dataInit| has pushed
  // it onto the end of |p7bio|'s chain. Walk the chain freeing BIOs until we
  // find |indata| so the caller retains ownership
  while (p7bio != NULL && p7bio != indata) {
    BIO *b = BIO_pop(p7bio);
    BIO_free(p7bio);
    p7bio = b;
  }
  sk_X509_free(signers);
  sk_X509_free(untrusted);
  return ret;
}

PKCS7 *SMIME_read_PKCS7(BIO *in, BIO **bcont) { return NULL; }

int SMIME_write_PKCS7(BIO *out, PKCS7 *p7, BIO *data, int flags) { return 0; }

OPENSSL_END_ALLOW_DEPRECATED
