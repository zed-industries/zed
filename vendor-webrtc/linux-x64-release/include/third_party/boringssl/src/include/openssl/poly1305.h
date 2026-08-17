// Copyright 2014 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_POLY1305_H
#define OPENSSL_HEADER_POLY1305_H

#include <openssl/base.h>   // IWYU pragma: export

#ifdef  __cplusplus
extern "C" {
#endif


typedef uint8_t poly1305_state[512];

// CRYPTO_poly1305_init sets up |state| so that it can be used to calculate an
// authentication tag with the one-time key |key|. Note that |key| is a
// one-time key and therefore there is no `reset' method because that would
// enable several messages to be authenticated with the same key.
OPENSSL_EXPORT void CRYPTO_poly1305_init(poly1305_state *state,
                                         const uint8_t key[32]);

// CRYPTO_poly1305_update processes |in_len| bytes from |in|. It can be called
// zero or more times after poly1305_init.
OPENSSL_EXPORT void CRYPTO_poly1305_update(poly1305_state *state,
                                           const uint8_t *in, size_t in_len);

// CRYPTO_poly1305_finish completes the poly1305 calculation and writes a 16
// byte authentication tag to |mac|.
OPENSSL_EXPORT void CRYPTO_poly1305_finish(poly1305_state *state,
                                           uint8_t mac[16]);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_POLY1305_H
