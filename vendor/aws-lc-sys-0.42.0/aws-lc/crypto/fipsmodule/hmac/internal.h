// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef AWSLC_HEADER_HMAC_INTERNAL_H
#define AWSLC_HEADER_HMAC_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif

// HMAC_with_precompute is only used in FIPS ACVP harness, in order to test
// the computation of HMAC using precomputed keys (internally). It should not
// be used for any other purposes as it outputs the same results as |HMAC| and is
// slower than |HMAC|.
// This function does not update the FIPS service indicator.
OPENSSL_EXPORT uint8_t *HMAC_with_precompute(const EVP_MD *evp_md,
                                             const void *key, size_t key_len,
                                             const uint8_t *data,
                                             size_t data_len, uint8_t *out,
                                             unsigned int *out_len);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // AWSLC_HEADER_HMAC_INTERNAL_H
