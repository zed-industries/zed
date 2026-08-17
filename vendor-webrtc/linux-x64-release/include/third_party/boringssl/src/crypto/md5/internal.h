// Copyright 2018 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_MD5_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_MD5_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


#if !defined(OPENSSL_NO_ASM) && \
    (defined(OPENSSL_X86_64) || defined(OPENSSL_X86))
#define MD5_ASM
extern void md5_block_asm_data_order(uint32_t *state, const uint8_t *data,
                                     size_t num);
#endif


#if defined(__cplusplus)
}  // extern "C"
#endif

#endif  // OPENSSL_HEADER_CRYPTO_MD5_INTERNAL_H
