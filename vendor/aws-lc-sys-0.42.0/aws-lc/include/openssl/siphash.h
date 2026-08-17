// Copyright (c) 2019, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_SIPHASH_H
#define OPENSSL_HEADER_SIPHASH_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


// SipHash is a fast, secure PRF that is often used for hash tables.


// SIPHASH_24 implements SipHash-2-4. See https://131002.net/siphash/siphash.pdf
OPENSSL_EXPORT uint64_t SIPHASH_24(const uint64_t key[2], const uint8_t *input,
                                   size_t input_len);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_SIPHASH_H
