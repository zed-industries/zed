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

#ifndef OPENSSL_HEADER_CRYPTO_CHACHA_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_CHACHA_INTERNAL_H

#include <openssl/base.h>

#include "../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


// CRYPTO_hchacha20 computes the HChaCha20 function, which should only be used
// as part of XChaCha20.
void CRYPTO_hchacha20(uint8_t out[32], const uint8_t key[32],
                      const uint8_t nonce[16]);

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86)

#define CHACHA20_ASM_NOHW

#define CHACHA20_ASM_SSSE3
inline int ChaCha20_ctr32_ssse3_capable(size_t len) {
  // Unlike the x86_64 version, the x86 SSSE3 routine runs for all non-zero
  // lengths.
  return len > 0 && CRYPTO_is_SSSE3_capable();
}
void ChaCha20_ctr32_ssse3(uint8_t *out, const uint8_t *in, size_t in_len,
                          const uint32_t key[8], const uint32_t counter[4]);

#elif !defined(OPENSSL_NO_ASM) && \
    (defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64))

#define CHACHA20_ASM_NOHW

#define CHACHA20_ASM_NEON
inline int ChaCha20_ctr32_neon_capable(size_t len) {
  return len >= 192 && CRYPTO_is_NEON_capable();
}
void ChaCha20_ctr32_neon(uint8_t *out, const uint8_t *in, size_t in_len,
                         const uint32_t key[8], const uint32_t counter[4]);
#elif !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64)
#define CHACHA20_ASM_NOHW

#define CHACHA20_ASM_AVX2
inline int ChaCha20_ctr32_avx2_capable(size_t len) {
  return len > 128 && CRYPTO_is_AVX2_capable();
}
void ChaCha20_ctr32_avx2(uint8_t *out, const uint8_t *in, size_t in_len,
                         const uint32_t key[8], const uint32_t counter[4]);

#define CHACHA20_ASM_SSSE3_4X
inline int ChaCha20_ctr32_ssse3_4x_capable(size_t len) {
  int capable = len > 128 && CRYPTO_is_SSSE3_capable();
  int faster = len > 192 || !CRYPTO_cpu_perf_is_like_silvermont();
  return capable && faster;
}
void ChaCha20_ctr32_ssse3_4x(uint8_t *out, const uint8_t *in, size_t in_len,
                             const uint32_t key[8], const uint32_t counter[4]);

#define CHACHA20_ASM_SSSE3
inline int ChaCha20_ctr32_ssse3_capable(size_t len) {
  return len > 128 && CRYPTO_is_SSSE3_capable();
}
void ChaCha20_ctr32_ssse3(uint8_t *out, const uint8_t *in, size_t in_len,
                          const uint32_t key[8], const uint32_t counter[4]);
#endif

#if defined(CHACHA20_ASM_NOHW)
// ChaCha20_ctr32_nohw encrypts |in_len| bytes from |in| and writes the result
// to |out|. If |in| and |out| alias, they must be equal. |in_len| may not be
// zero.
//
// |counter[0]| is the initial 32-bit block counter, and the remainder is the
// 96-bit nonce. If the counter overflows, the output is undefined. The function
// will produce output, but the output may vary by machine and may not be
// self-consistent. (On some architectures, the assembly implements a mix of
// 64-bit and 32-bit counters.)
void ChaCha20_ctr32_nohw(uint8_t *out, const uint8_t *in, size_t in_len,
                         const uint32_t key[8], const uint32_t counter[4]);
#endif


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_CHACHA_INTERNAL_H
