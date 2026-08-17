// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_MD5_INTERNAL_H
#define OPENSSL_HEADER_MD5_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif

// MD5_CHAINING_LENGTH is the chaining length in bytes of MD5
// It corresponds to the length in bytes of the h part of the state
#define MD5_CHAINING_LENGTH 16

// MD5_Init_from_state is a low-level function that initializes |sha| with a
// custom state. |h| is the hash state in big endian. |n| is the number of bits
// processed at this point. It must be a multiple of |MD5_CBLOCK*8|.
// It returns one on success and zero on error.
// This function is for internal use only and should never be directly called.
OPENSSL_EXPORT int MD5_Init_from_state(MD5_CTX *sha,
                                       const uint8_t h[MD5_CHAINING_LENGTH],
                                       uint64_t n);

// MD5_get_state is a low-level function that exports the hash state in big
// endian into |out_n| and the number of bits processed at this point in
// |out_n|. |MD5_Final| must not have been called before (otherwise results
// are not guaranteed). Furthermore, the number of bytes processed by
// |MD5_Update| must be a multiple of the block length |MD5_CBLOCK|
// (otherwise it fails). It returns one on success and zero on error.
// This function is for internal use only and should never be directly called.
OPENSSL_EXPORT int MD5_get_state(MD5_CTX *ctx,
                                 uint8_t out_h[MD5_CHAINING_LENGTH],
                                 uint64_t *out_n);

#if !defined(OPENSSL_NO_ASM) && \
    (defined(OPENSSL_X86_64) || defined(OPENSSL_X86) || defined(OPENSSL_AARCH64))
#define MD5_ASM
extern void md5_block_asm_data_order(uint32_t *state, const uint8_t *data,
                                     size_t num);
#endif


#if defined(__cplusplus)
}  // extern "C"
#endif

#endif  // OPENSSL_HEADER_MD5_INTERNAL_H
