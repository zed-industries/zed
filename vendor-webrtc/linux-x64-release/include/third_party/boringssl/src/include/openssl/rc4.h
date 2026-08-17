// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_RC4_H
#define OPENSSL_HEADER_RC4_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


// RC4.


struct rc4_key_st {
  uint32_t x, y;
  uint32_t data[256];
} /* RC4_KEY */;

// RC4_set_key performs an RC4 key schedule and initialises |rc4key| with |len|
// bytes of key material from |key|.
OPENSSL_EXPORT void RC4_set_key(RC4_KEY *rc4key, unsigned len,
                                const uint8_t *key);

// RC4 encrypts (or decrypts, it's the same with RC4) |len| bytes from |in| to
// |out|.
OPENSSL_EXPORT void RC4(RC4_KEY *key, size_t len, const uint8_t *in,
                        uint8_t *out);


// Deprecated functions.

// RC4_options returns the string "rc4(ptr,int)".
OPENSSL_EXPORT const char *RC4_options(void);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_RC4_H
