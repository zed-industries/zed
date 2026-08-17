// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_CHACHA_H
#define OPENSSL_HEADER_CHACHA_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif

// ChaCha20.
//
// ChaCha20 is a stream cipher. See https://tools.ietf.org/html/rfc8439.


// CRYPTO_chacha_20 encrypts |in_len| bytes from |in| with the given key and
// nonce and writes the result to |out|. If |in| and |out| alias, they must be
// equal. The initial block counter is specified by |counter|. |in| must be
// aligned on a block boundary.
//
// This function implements a 32-bit block counter as in RFC 8439. On overflow,
// the counter wraps. Reusing a key, nonce, and block counter combination is not
// secure, so wrapping is usually a bug in the caller. While it is possible to
// wrap without reuse with a large initial block counter, this is not
// recommended and may not be portable to other ChaCha20 implementations.
OPENSSL_EXPORT void CRYPTO_chacha_20(uint8_t *out, const uint8_t *in,
                                     size_t in_len, const uint8_t key[32],
                                     const uint8_t nonce[12], uint32_t counter);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CHACHA_H
