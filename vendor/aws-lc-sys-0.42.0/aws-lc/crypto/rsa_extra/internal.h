// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/base.h>
#include <openssl/bytestring.h>

#if defined(__cplusplus)
extern "C" {
#endif

#define PSS_DEFAULT_SALT_LEN 20
#define PSS_TRAILER_FIELD_VALUE 1

// Section 2.1. https://tools.ietf.org/html/rfc4055#section-2.1
typedef struct rsa_pss_supported_algor_st {
  int nid;
  uint8_t oid[9];
  uint8_t oid_len;
} RSA_PSS_SUPPORTED_ALGOR;

typedef struct rsa_algor_identifier_st {
  int nid;
} RSA_ALGOR_IDENTIFIER;

typedef struct rsa_mga_identifier_st {
  // Mask generation algorithm identifier.
  RSA_ALGOR_IDENTIFIER *mask_gen;
  // Associated hash algorithm identifier.
  RSA_ALGOR_IDENTIFIER *one_way_hash;
} RSA_MGA_IDENTIFIER;

typedef struct rsa_integer_st {
  int64_t value;
} RSA_INTEGER;

// RSASSA-PSS-params is defined on rfc4055 section 3.1.
// See https://tools.ietf.org/html/rfc4055#section-3.1
struct rsassa_pss_params_st {
  RSA_ALGOR_IDENTIFIER *hash_algor;
  RSA_MGA_IDENTIFIER *mask_gen_algor;
  RSA_INTEGER *salt_len;
  RSA_INTEGER *trailer_field;
};

// RSASSA_PSS_PARAMS related malloc functions.
RSA_INTEGER *RSA_INTEGER_new(void);
RSA_ALGOR_IDENTIFIER *RSA_ALGOR_IDENTIFIER_new(void);
RSA_MGA_IDENTIFIER *RSA_MGA_IDENTIFIER_new(void);
OPENSSL_EXPORT RSASSA_PSS_PARAMS *RSASSA_PSS_PARAMS_new(void);

// RSASSA_PSS_PARAMS related free functions.
void RSA_INTEGER_free(RSA_INTEGER *ptr);
void RSA_ALGOR_IDENTIFIER_free(RSA_ALGOR_IDENTIFIER *ptr);
void RSA_MGA_IDENTIFIER_free(RSA_MGA_IDENTIFIER *identifier);
OPENSSL_EXPORT void RSASSA_PSS_PARAMS_free(RSASSA_PSS_PARAMS *params);

// RSASSA_PSS_parse_params return one on success and zero on failure.
// If success and the |params| are not empty, |*pss| will be allocated
// and have the bytes parsed. Otherwise, |*pss| will be NULL.
OPENSSL_EXPORT int RSASSA_PSS_parse_params(CBS *params,
                                           RSASSA_PSS_PARAMS **pss);

// RSASSA_PSS_PARAMS_create return one on success and zero on failure.
// When success and the given algorithms are not default (sha1),
// |*out| will hold the allocated |RSASSA_PSS_PARAMS|.
OPENSSL_EXPORT int RSASSA_PSS_PARAMS_create(const EVP_MD *sigmd,
                                            const EVP_MD *mgf1md, int saltlen,
                                            RSASSA_PSS_PARAMS **out);

// RSASSA_PSS_PARAMS_get return one on success and zero on failure.
// When success, |*md|, |*mgf1md| and |saltlen| will get assigned.
OPENSSL_EXPORT int RSASSA_PSS_PARAMS_get(const RSASSA_PSS_PARAMS *pss,
                                         const EVP_MD **md,
                                         const EVP_MD **mgf1md, int *saltlen);

int RSA_padding_check_PKCS1_OAEP_mgf1(uint8_t *out, size_t *out_len,
                                      size_t max_out, const uint8_t *from,
                                      size_t from_len, const uint8_t *param,
                                      size_t param_len, const EVP_MD *md,
                                      const EVP_MD *mgf1md);

#if defined(__cplusplus)
}  // extern C

extern "C++" {

BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(RSASSA_PSS_PARAMS, RSASSA_PSS_PARAMS_free)

BSSL_NAMESPACE_END

}  // extern C++

#endif

