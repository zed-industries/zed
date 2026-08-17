// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_PKCS7_INTERNAL_H
#define OPENSSL_HEADER_PKCS7_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif



DECLARE_ASN1_FUNCTIONS(PKCS7_ISSUER_AND_SERIAL)
DECLARE_ASN1_FUNCTIONS(PKCS7_SIGNED)
DECLARE_ASN1_FUNCTIONS(PKCS7_ENC_CONTENT)
DECLARE_ASN1_FUNCTIONS(PKCS7_ENCRYPT)
DECLARE_ASN1_FUNCTIONS(PKCS7_ENVELOPE)
DECLARE_ASN1_FUNCTIONS(PKCS7_DIGEST)
DECLARE_ASN1_FUNCTIONS(PKCS7_SIGN_ENVELOPE)

DECLARE_ASN1_ITEM(PKCS7_ATTR_VERIFY)

DEFINE_STACK_OF(PKCS7)

// ASN.1 defined here https://datatracker.ietf.org/doc/html/rfc2315#section-10.1
//
//   EncryptedContentInfo ::= SEQUENCE {
//     contentType ContentType,
//     contentEncryptionAlgorithm
//       ContentEncryptionAlgorithmIdentifier,
//     encryptedContent
//       [0] IMPLICIT EncryptedContent OPTIONAL }
//
//   EncryptedContent ::= OCTET STRING
struct pkcs7_enc_content_st {
  ASN1_OBJECT *content_type;
  X509_ALGOR *algorithm;
  ASN1_OCTET_STRING *enc_data;
  const EVP_CIPHER *cipher;  // NOTE: |cipher| is not serialized
};

// ASN.1 defined here https://datatracker.ietf.org/doc/html/rfc2315#section-12
//
//   DigestedData ::= SEQUENCE {
//     version Version,
//     digestAlgorithm DigestAlgorithmIdentifier,
//     contentInfo ContentInfo,
//     digest Digest }
//
//   Digest ::= OCTET STRING
struct pkcs7_digest_st {
  ASN1_INTEGER *version;
  X509_ALGOR *digest_alg;
  PKCS7 *contents;
  ASN1_OCTET_STRING *digest;
  const EVP_MD *md;  // NOTE: |md| is not serialized
};

// ASN.1 defined here https://datatracker.ietf.org/doc/html/rfc2315#section-13
//
//   EncryptedData ::= SEQUENCE {
//     version Version,
//     encryptedContentInfo EncryptedContentInfo }
struct pkcs7_encrypt_st {
  ASN1_INTEGER *version;
  PKCS7_ENC_CONTENT *enc_data;
};

// pkcs7_parse_header reads the non-certificate/non-CRL prefix of a PKCS#7
// SignedData blob from |cbs| and sets |*out| to point to the rest of the
// input. If the input is in BER format, then |*der_bytes| will be set to a
// pointer that needs to be freed by the caller once they have finished
// processing |*out| (which will be pointing into |*der_bytes|).
//
// It returns one on success or zero on error. On error, |*der_bytes| is
// NULL.
int pkcs7_parse_header(uint8_t **der_bytes, CBS *out, CBS *cbs);

// pkcs7_add_signed_data writes a PKCS#7, SignedData structure to |out|. While
// doing so it makes callbacks to let the caller fill in parts of the structure.
// All callbacks are ignored if NULL and return one on success or zero on error.
//
//   digest_algos_cb: may write AlgorithmIdentifiers into the given CBB, which
//       is a SET of digest algorithms.
//   cert_crl_cb: may write the |certificates| or |crls| fields.
//       (See https://datatracker.ietf.org/doc/html/rfc2315#section-9.1)
//   signer_infos_cb: may write the contents of the |signerInfos| field.
//       (See https://datatracker.ietf.org/doc/html/rfc2315#section-9.1)
//
// pkcs7_add_signed_data returns one on success or zero on error.
int pkcs7_add_signed_data(CBB *out,
                          int (*digest_algos_cb)(CBB *out, const void *arg),
                          int (*cert_crl_cb)(CBB *out, const void *arg),
                          int (*signer_infos_cb)(CBB *out, const void *arg),
                          const void *arg);

// BIO_f_cipher is used internally by the pkcs7 module. It is not recommended
// for external use.
OPENSSL_EXPORT const BIO_METHOD *BIO_f_cipher(void);

// BIO_get_cipher_ctx writes a reference of |b|'s EVP_CIPHER_CTX* to |*ctx|
int BIO_get_cipher_ctx(BIO *b, EVP_CIPHER_CTX **ctx);

// BIO_set_cipher is used internally for testing. It is not recommended for
// external use.
OPENSSL_EXPORT int BIO_set_cipher(BIO *b, const EVP_CIPHER *cipher,
                                  const unsigned char *key,
                                  const unsigned char *iv, int enc);

// BIO_get_cipher_status returns 1 if the cipher is in a healthy state or 0
// otherwise. Unhealthy state could indicate decryption failure or other
// abnormalities. Data read from an unhealthy cipher should not be considered
// authentic.
OPENSSL_EXPORT int BIO_get_cipher_status(BIO *b);

// pkcs7_final initializes a data BIO using |p7|, copies all of |data| into it,
// before final finalizing |p7|. It returns 1 on success and 0 on failure.
int pkcs7_final(PKCS7 *p7, BIO *data);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_PKCS7_INTERNAL_H
