// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_PKCS8_INTERNAL_H
#define OPENSSL_HEADER_PKCS8_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


struct pkcs8_priv_key_info_st {
  ASN1_INTEGER *version;
  X509_ALGOR *pkeyalg;
  ASN1_OCTET_STRING *pkey;
  STACK_OF(X509_ATTRIBUTE) *attributes;
};

// pkcs8_pbe_decrypt decrypts |in| using the PBE scheme described by
// |algorithm|, which should be a serialized AlgorithmIdentifier structure. On
// success, it sets |*out| to a newly-allocated buffer containing the decrypted
// result and returns one. Otherwise, it returns zero.
int pkcs8_pbe_decrypt(uint8_t **out, size_t *out_len, CBS *algorithm,
                      const char *pass, size_t pass_len, const uint8_t *in,
                      size_t in_len);

#define PKCS12_KEY_ID 1
#define PKCS12_IV_ID 2
#define PKCS12_MAC_ID 3
#define PKCS12_SALT_LEN 16

// pkcs12_key_gen runs the PKCS#12 key derivation function as specified in
// RFC 7292, appendix B. On success, it writes the resulting |out_len| bytes of
// key material to |out| and returns one. Otherwise, it returns zero. |id|
// should be one of the |PKCS12_*_ID| values.
int pkcs12_key_gen(const char *pass, size_t pass_len, const uint8_t *salt,
                   size_t salt_len, uint8_t id, uint32_t iterations,
                   size_t out_len, uint8_t *out, const EVP_MD *md);

// pkcs12_pbe_encrypt_init configures |ctx| for encrypting with a PBES1 scheme
// defined in PKCS#12. It writes the corresponding AlgorithmIdentifier to |out|.
int pkcs12_pbe_encrypt_init(CBB *out, EVP_CIPHER_CTX *ctx, int alg,
                            uint32_t iterations, const char *pass,
                            size_t pass_len, const uint8_t *salt,
                            size_t salt_len);

struct pbe_suite {
  int pbe_nid;
  uint8_t oid[10];
  uint8_t oid_len;
  const EVP_CIPHER *(*cipher_func)(void);
  const EVP_MD *(*md_func)(void);
  // decrypt_init initialize |ctx| for decrypting. The password is specified by
  // |pass| and |pass_len|. |param| contains the serialized parameters field of
  // the AlgorithmIdentifier.
  //
  // It returns one on success and zero on error.
  int (*decrypt_init)(const struct pbe_suite *suite, EVP_CIPHER_CTX *ctx,
                      const char *pass, size_t pass_len, CBS *param);
};

int PKCS5_pbe2_decrypt_init(const struct pbe_suite *suite, EVP_CIPHER_CTX *ctx,
                            const char *pass, size_t pass_len, CBS *param);

// PKCS5_pbe2_encrypt_init configures |ctx| for encrypting with PKCS #5 PBES2,
// as defined in RFC 2998, with the specified parameters. It writes the
// corresponding AlgorithmIdentifier to |out|.
int PKCS5_pbe2_encrypt_init(CBB *out, EVP_CIPHER_CTX *ctx,
                            const EVP_CIPHER *cipher, uint32_t iterations,
                            const char *pass, size_t pass_len,
                            const uint8_t *salt, size_t salt_len);

// pkcs12_iterations_acceptable returns one if |iterations| is a reasonable
// number of PBKDF2 iterations and zero otherwise.
int pkcs12_iterations_acceptable(uint64_t iterations);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_PKCS8_INTERNAL_H
