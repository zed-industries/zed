// Copyright 2016 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_POLY1305_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_POLY1305_INTERNAL_H

#include <openssl/base.h>
#include <openssl/poly1305.h>

#if defined(__cplusplus)
extern "C" {
#endif

#if defined(OPENSSL_ARM) && !defined(OPENSSL_NO_ASM) && !defined(OPENSSL_APPLE)
#define OPENSSL_POLY1305_NEON

void CRYPTO_poly1305_init_neon(poly1305_state *state, const uint8_t key[32]);

void CRYPTO_poly1305_update_neon(poly1305_state *state, const uint8_t *in,
                                 size_t in_len);

void CRYPTO_poly1305_finish_neon(poly1305_state *state, uint8_t mac[16]);
#endif


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_POLY1305_INTERNAL_H
