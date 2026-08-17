// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/pkcs7.h>

#include <assert.h>
#include <limits.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/pem.h>
#include <openssl/pool.h>
#include <openssl/stack.h>
#include <openssl/x509.h>

#include "../internal.h"
#include "internal.h"

OPENSSL_BEGIN_ALLOW_DEPRECATED

int PKCS7_get_certificates(STACK_OF(X509) *out_certs, CBS *cbs) {
  int ret = 0;
  const size_t initial_certs_len = sk_X509_num(out_certs);
  STACK_OF(CRYPTO_BUFFER) *raw = sk_CRYPTO_BUFFER_new_null();
  if (raw == NULL || !PKCS7_get_raw_certificates(raw, cbs, NULL)) {
    goto err;
  }

  for (size_t i = 0; i < sk_CRYPTO_BUFFER_num(raw); i++) {
    CRYPTO_BUFFER *buf = sk_CRYPTO_BUFFER_value(raw, i);
    X509 *x509 = X509_parse_from_buffer(buf);
    if (x509 == NULL || !sk_X509_push(out_certs, x509)) {
      X509_free(x509);
      goto err;
    }
  }

  ret = 1;

err:
  sk_CRYPTO_BUFFER_pop_free(raw, CRYPTO_BUFFER_free);
  if (!ret) {
    while (sk_X509_num(out_certs) != initial_certs_len) {
      X509 *x509 = sk_X509_pop(out_certs);
      X509_free(x509);
    }
  }

  return ret;
}

int PKCS7_get_CRLs(STACK_OF(X509_CRL) *out_crls, CBS *cbs) {
  CBS signed_data, crls;
  uint8_t *der_bytes = NULL;
  int ret = 0, has_crls;
  const size_t initial_crls_len = sk_X509_CRL_num(out_crls);

  // See https://tools.ietf.org/html/rfc2315#section-9.1
  if (!pkcs7_parse_header(&der_bytes, &signed_data, cbs) ||
      // Even if only CRLs are included, there may be an empty certificates
      // block. OpenSSL does this, for example.
      !CBS_get_optional_asn1(
          &signed_data, NULL, NULL,
          CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0) ||
      !CBS_get_optional_asn1(
          &signed_data, &crls, &has_crls,
          CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 1)) {
    goto err;
  }

  if (!has_crls) {
    CBS_init(&crls, NULL, 0);
  }

  while (CBS_len(&crls) > 0) {
    CBS crl_data;
    X509_CRL *crl;
    const uint8_t *inp;

    if (!CBS_get_asn1_element(&crls, &crl_data, CBS_ASN1_SEQUENCE)) {
      goto err;
    }

    if (CBS_len(&crl_data) > LONG_MAX) {
      goto err;
    }
    inp = CBS_data(&crl_data);
    crl = d2i_X509_CRL(NULL, &inp, (long)CBS_len(&crl_data));
    if (!crl) {
      goto err;
    }

    assert(inp == CBS_data(&crl_data) + CBS_len(&crl_data));

    if (sk_X509_CRL_push(out_crls, crl) == 0) {
      X509_CRL_free(crl);
      goto err;
    }
  }

  ret = 1;

err:
  OPENSSL_free(der_bytes);

  if (!ret) {
    while (sk_X509_CRL_num(out_crls) != initial_crls_len) {
      X509_CRL_free(sk_X509_CRL_pop(out_crls));
    }
  }

  return ret;
}

int PKCS7_get_PEM_certificates(STACK_OF(X509) *out_certs, BIO *pem_bio) {
  uint8_t *data;
  long len;
  int ret;

  // Even though we pass PEM_STRING_PKCS7 as the expected PEM type here, PEM
  // internally will actually allow several other values too, including
  // "CERTIFICATE".
  if (!PEM_bytes_read_bio(&data, &len, NULL /* PEM type output */,
                          PEM_STRING_PKCS7, pem_bio,
                          NULL /* password callback */,
                          NULL /* password callback argument */)) {
    return 0;
  }

  CBS cbs;
  CBS_init(&cbs, data, len);
  ret = PKCS7_get_certificates(out_certs, &cbs);
  OPENSSL_free(data);
  return ret;
}

int PKCS7_get_PEM_CRLs(STACK_OF(X509_CRL) *out_crls, BIO *pem_bio) {
  uint8_t *data;
  long len;
  int ret;

  // Even though we pass PEM_STRING_PKCS7 as the expected PEM type here, PEM
  // internally will actually allow several other values too, including
  // "CERTIFICATE".
  if (!PEM_bytes_read_bio(&data, &len, NULL /* PEM type output */,
                          PEM_STRING_PKCS7, pem_bio,
                          NULL /* password callback */,
                          NULL /* password callback argument */)) {
    return 0;
  }

  CBS cbs;
  CBS_init(&cbs, data, len);
  ret = PKCS7_get_CRLs(out_crls, &cbs);
  OPENSSL_free(data);
  return ret;
}

static int pkcs7_bundle_certificates_cb(CBB *out, const void *arg) {
  const STACK_OF(X509) *certs = arg;
  size_t i;
  CBB certificates;

  // See https://tools.ietf.org/html/rfc2315#section-9.1
  if (!CBB_add_asn1(out, &certificates,
                    CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0)) {
    return 0;
  }

  for (i = 0; i < sk_X509_num(certs); i++) {
    X509 *x509 = sk_X509_value(certs, i);
    uint8_t *buf;
    int len = i2d_X509(x509, NULL);

    if (len < 0 || !CBB_add_space(&certificates, &buf, len) ||
        i2d_X509(x509, &buf) < 0) {
      return 0;
    }
  }

  // |certificates| is a implicitly-tagged SET OF.
  return CBB_flush_asn1_set_of(&certificates) && CBB_flush(out);
}

int PKCS7_bundle_certificates(CBB *out, const STACK_OF(X509) *certs) {
  return pkcs7_add_signed_data(out, /*digest_algos_cb=*/NULL,
                               pkcs7_bundle_certificates_cb,
                               /*signer_infos_cb=*/NULL, certs);
}

static int pkcs7_bundle_crls_cb(CBB *out, const void *arg) {
  const STACK_OF(X509_CRL) *crls = arg;
  size_t i;
  CBB crl_data;

  // See https://tools.ietf.org/html/rfc2315#section-9.1
  if (!CBB_add_asn1(out, &crl_data,
                    CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 1)) {
    return 0;
  }

  for (i = 0; i < sk_X509_CRL_num(crls); i++) {
    X509_CRL *crl = sk_X509_CRL_value(crls, i);
    uint8_t *buf;
    int len = i2d_X509_CRL(crl, NULL);

    if (len < 0 || !CBB_add_space(&crl_data, &buf, len) ||
        i2d_X509_CRL(crl, &buf) < 0) {
      return 0;
    }
  }

  // |crl_data| is a implicitly-tagged SET OF.
  return CBB_flush_asn1_set_of(&crl_data) && CBB_flush(out);
}

int PKCS7_bundle_CRLs(CBB *out, const STACK_OF(X509_CRL) *crls) {
  return pkcs7_add_signed_data(out, /*digest_algos_cb=*/NULL,
                               pkcs7_bundle_crls_cb,
                               /*signer_infos_cb=*/NULL, crls);
}

PKCS7 *d2i_PKCS7_bio(BIO *bio, PKCS7 **out) {
  GUARD_PTR(bio);
  uint8_t *data = NULL;
  size_t len;
  // Read BIO contents into newly allocated buffer
  if (!BIO_read_asn1(bio, &data, &len, INT_MAX)) {
    return NULL;
  }
  const uint8_t *ptr = data;
  // d2i_PKCS7 handles indefinite-length BER properly, so use it instead of
  // ASN1_item_d2i_bio
  PKCS7 *ret = d2i_PKCS7(out, &ptr, len);
  OPENSSL_free(data);
  return ret;
}

int i2d_PKCS7_bio(BIO *bio, const PKCS7 *p7) {
  return ASN1_item_i2d_bio(ASN1_ITEM_rptr(PKCS7), bio, (void *)p7);
}

int PKCS7_type_is_data(const PKCS7 *p7) {
  return OBJ_obj2nid(p7->type) == NID_pkcs7_data;
}

int PKCS7_type_is_digest(const PKCS7 *p7) {
  return OBJ_obj2nid(p7->type) == NID_pkcs7_digest;
}

int PKCS7_type_is_encrypted(const PKCS7 *p7) {
  return OBJ_obj2nid(p7->type) == NID_pkcs7_encrypted;
}

int PKCS7_type_is_enveloped(const PKCS7 *p7) {
  return OBJ_obj2nid(p7->type) == NID_pkcs7_enveloped;
}

int PKCS7_type_is_signed(const PKCS7 *p7) {
  return OBJ_obj2nid(p7->type) == NID_pkcs7_signed;
}

int PKCS7_type_is_signedAndEnveloped(const PKCS7 *p7) {
  return OBJ_obj2nid(p7->type) == NID_pkcs7_signedAndEnveloped;
}

// write_sha256_ai writes an AlgorithmIdentifier for SHA-256 to
// |digest_algos_set|.
static int write_sha256_ai(CBB *digest_algos_set, const void *arg) {
  CBB seq;
  return CBB_add_asn1(digest_algos_set, &seq, CBS_ASN1_SEQUENCE) &&
         OBJ_nid2cbb(&seq, NID_sha256) &&  //
         // https://datatracker.ietf.org/doc/html/rfc5754#section-2
         // "Implementations MUST generate SHA2 AlgorithmIdentifiers with absent
         //  parameters."
         CBB_flush(digest_algos_set);
}

// sign_sha256 writes at most |max_out_sig| bytes of the signature of |data| by
// |pkey| to |out_sig| and sets |*out_sig_len| to the number of bytes written.
// It returns one on success or zero on error.
static int sign_sha256(uint8_t *out_sig, size_t *out_sig_len,
                       size_t max_out_sig, EVP_PKEY *pkey, BIO *data) {
  static const size_t kBufSize = 4096;
  uint8_t *buffer = OPENSSL_malloc(kBufSize);
  if (!buffer) {
    return 0;
  }

  EVP_MD_CTX ctx;
  EVP_MD_CTX_init(&ctx);

  int ret = 0;
  if (!EVP_DigestSignInit(&ctx, NULL, EVP_sha256(), NULL, pkey)) {
    goto out;
  }

  for (;;) {
    const int n = BIO_read(data, buffer, kBufSize);
    if (n == 0) {
      break;
    } else if (n < 0 || !EVP_DigestSignUpdate(&ctx, buffer, n)) {
      goto out;
    }
  }

  *out_sig_len = max_out_sig;
  if (!EVP_DigestSignFinal(&ctx, out_sig, out_sig_len)) {
    goto out;
  }

  ret = 1;

out:
  EVP_MD_CTX_cleanup(&ctx);
  OPENSSL_free(buffer);
  return ret;
}

struct signer_info_data {
  const X509 *sign_cert;
  uint8_t *signature;
  size_t signature_len;
};

// write_signer_info writes the SignerInfo structure from
// https://datatracker.ietf.org/doc/html/rfc2315#section-9.2 to |out|. It
// returns one on success or zero on error.
static int write_signer_info(CBB *out, const void *arg) {
  const struct signer_info_data *const si_data = arg;

  int ret = 0;
  uint8_t *subject_bytes = NULL;
  uint8_t *serial_bytes = NULL;

  const int subject_len =
      i2d_X509_NAME(X509_get_subject_name(si_data->sign_cert), &subject_bytes);
  const int serial_len = i2d_ASN1_INTEGER(
      (ASN1_INTEGER *)X509_get0_serialNumber(si_data->sign_cert),
      &serial_bytes);

  CBB seq, issuer_and_serial, signing_algo, null, signature;
  if (subject_len < 0 || serial_len < 0 ||
      !CBB_add_asn1(out, &seq, CBS_ASN1_SEQUENCE) ||
      // version
      !CBB_add_asn1_uint64(&seq, 1) ||
      !CBB_add_asn1(&seq, &issuer_and_serial, CBS_ASN1_SEQUENCE) ||
      !CBB_add_bytes(&issuer_and_serial, subject_bytes, subject_len) ||
      !CBB_add_bytes(&issuer_and_serial, serial_bytes, serial_len) ||
      !write_sha256_ai(&seq, NULL) ||
      !CBB_add_asn1(&seq, &signing_algo, CBS_ASN1_SEQUENCE) ||
      !OBJ_nid2cbb(&signing_algo, NID_rsaEncryption) ||
      !CBB_add_asn1(&signing_algo, &null, CBS_ASN1_NULL) ||
      !CBB_add_asn1(&seq, &signature, CBS_ASN1_OCTETSTRING) ||
      !CBB_add_bytes(&signature, si_data->signature, si_data->signature_len) ||
      !CBB_flush(out)) {
    goto out;
  }

  ret = 1;

out:
  OPENSSL_free(subject_bytes);
  OPENSSL_free(serial_bytes);
  return ret;
}

static int pkcs7_add_signature(PKCS7 *p7, X509 *x509, EVP_PKEY *pkey) {
  // OpenSSL's docs say that this defaults to SHA1, but appears to actually
  // default to SHA256 in 1.1.x and 3.x for RSA, DSA, and EC(DSA).
  // https://linux.die.net/man/3/pkcs7_sign
  // https://github.com/openssl/openssl/blob/79c98fc6ccab49f02528e06cc046ac61f841a753/crypto/rsa/rsa_ameth.c#L438
  const EVP_MD *digest = EVP_sha256();
  PKCS7_SIGNER_INFO *si = NULL;

  switch (EVP_PKEY_id(pkey)) {
    case EVP_PKEY_RSA:
    case EVP_PKEY_DSA:
    case EVP_PKEY_EC:
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_NO_DEFAULT_DIGEST);
      goto err;
  }

  // We add the signer's info below, including the static |digest|. We delegate
  // initialization of the |digest| into an |EVP_MD_CTX| to |BIO_f_md|.
  if ((si = PKCS7_SIGNER_INFO_new()) == NULL ||
      !PKCS7_SIGNER_INFO_set(si, x509, pkey, digest) ||
      !PKCS7_add_signer(p7, si)) {  // |p7| takes ownership of |si| here
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_PKCS7_DATASIGN);
    goto err;
  }
  return 1;
err:
  PKCS7_SIGNER_INFO_free(si);
  return 0;
}

static int pkcs7_sign_add_signer(PKCS7 *p7, X509 *signcert, EVP_PKEY *pkey) {
  if (!X509_check_private_key(signcert, pkey)) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_PRIVATE_KEY_DOES_NOT_MATCH_CERTIFICATE);
    return 0;
  }

  if (!pkcs7_add_signature(p7, signcert, pkey)) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_PKCS7_ADD_SIGNATURE_ERROR);
    return 0;
  }

  if (!PKCS7_add_certificate(p7, signcert)) {
    return 0;
  }

  return 1;
}

static PKCS7 *pkcs7_do_general_sign(X509 *sign_cert, EVP_PKEY *pkey,
                                    struct stack_st_X509 *certs, BIO *data,
                                    int flags) {
  PKCS7 *ret = NULL;
  if ((ret = PKCS7_new()) == NULL || !PKCS7_set_type(ret, NID_pkcs7_signed) ||
      !PKCS7_content_new(ret, NID_pkcs7_data)) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PKCS7_LIB);
    goto err;
  }

  if (!pkcs7_sign_add_signer(ret, sign_cert, pkey)) {
    OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_PKCS7_ADD_SIGNER_ERROR);
    goto err;
  }

  for (size_t i = 0; i < sk_X509_num(certs); i++) {
    if (!PKCS7_add_certificate(ret, sk_X509_value(certs, i))) {
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_PKCS7_ADD_SIGNER_ERROR);
      goto err;
    }
  }

  if ((flags & PKCS7_DETACHED) && PKCS7_type_is_data(ret->d.sign->contents)) {
    ASN1_OCTET_STRING_free(ret->d.sign->contents->d.data);
    ret->d.sign->contents->d.data = NULL;
  }

  if (!pkcs7_final(ret, data)) {
    goto err;
  }

  return ret;
err:
  PKCS7_free(ret);
  return NULL;
}

PKCS7 *PKCS7_sign(X509 *sign_cert, EVP_PKEY *pkey, STACK_OF(X509) *certs,
                  BIO *data, int flags) {
  CBB cbb;
  if (!CBB_init(&cbb, 2048)) {
    return NULL;
  }

  uint8_t *der = NULL;
  size_t len;
  PKCS7 *ret = NULL;

  if (sign_cert == NULL && pkey == NULL && flags == PKCS7_DETACHED) {
    // Caller just wants to bundle certificates.
    if (!PKCS7_bundle_certificates(&cbb, certs)) {
      goto out;
    }
  } else if (sign_cert != NULL && pkey != NULL && certs == NULL &&
             data != NULL &&
             flags == (PKCS7_NOATTR | PKCS7_BINARY | PKCS7_NOCERTS |
                       PKCS7_DETACHED) &&
             EVP_PKEY_id(pkey) == NID_rsaEncryption) {
    // sign-file.c from the Linux kernel.
    const size_t signature_max_len = EVP_PKEY_size(pkey);
    struct signer_info_data si_data = {
        .sign_cert = sign_cert,
        .signature = OPENSSL_malloc(signature_max_len),
    };

    if (!si_data.signature ||
        !sign_sha256(si_data.signature, &si_data.signature_len,
                     signature_max_len, pkey, data) ||
        !pkcs7_add_signed_data(&cbb, write_sha256_ai, /*cert_crl_cb=*/NULL,
                               write_signer_info, &si_data)) {
      OPENSSL_free(si_data.signature);
      goto out;
    }
    OPENSSL_free(si_data.signature);
  } else if (sign_cert != NULL && pkey != NULL && data != NULL &&
             !(flags & PKCS7_NOCERTS)) {
    ret = pkcs7_do_general_sign(sign_cert, pkey, certs, data, flags);
    goto out;
  } else {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    goto out;
  }

  if (!CBB_finish(&cbb, &der, &len)) {
    goto out;
  }

  const uint8_t *const_der = der;
  ret = d2i_PKCS7(NULL, &const_der, len);

out:
  CBB_cleanup(&cbb);
  OPENSSL_free(der);
  return ret;
}

int PKCS7_add_certificate(PKCS7 *p7, X509 *x509) {
  STACK_OF(X509) **sk;

  if (p7 == NULL || x509 == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_signed:
      sk = &(p7->d.sign->cert);
      break;
    case NID_pkcs7_signedAndEnveloped:
      sk = &(p7->d.signed_and_enveloped->cert);
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_WRONG_CONTENT_TYPE);
      return 0;
  }

  if (*sk == NULL) {
    *sk = sk_X509_new_null();
  }
  if (*sk == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_CRYPTO_LIB);
    return 0;
  }

  if (!sk_X509_push(*sk, x509)) {
    return 0;
  }
  X509_up_ref(x509);
  return 1;
}

int PKCS7_add_crl(PKCS7 *p7, X509_CRL *crl) {
  STACK_OF(X509_CRL) **sk;

  if (p7 == NULL || crl == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  switch (OBJ_obj2nid(p7->type)) {
    case NID_pkcs7_signed:
      sk = &(p7->d.sign->crl);
      break;
    case NID_pkcs7_signedAndEnveloped:
      sk = &(p7->d.signed_and_enveloped->crl);
      break;
    default:
      OPENSSL_PUT_ERROR(PKCS7, PKCS7_R_WRONG_CONTENT_TYPE);
      return 0;
  }

  if (*sk == NULL) {
    *sk = sk_X509_CRL_new_null();
  }
  if (*sk == NULL) {
    OPENSSL_PUT_ERROR(PKCS7, ERR_R_CRYPTO_LIB);
    return 0;
  }

  if (!sk_X509_CRL_push(*sk, crl)) {
    return 0;
  }
  X509_CRL_up_ref(crl);
  return 1;
}

OPENSSL_END_ALLOW_DEPRECATED
