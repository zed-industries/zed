// Copyright 2019 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_SIPHASH_H
#define OPENSSL_HEADER_SIPHASH_H

#include <openssl/base.h>   // IWYU pragma: export

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
