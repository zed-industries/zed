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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_SHA_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_SHA_INTERNAL_H

#include <openssl/base.h>

#include "../../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif

// Define SHA{n}[_{variant}]_ASM if sha{n}_block_data_order[_{variant}] is
// defined in assembly.

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_ARM)

#define SHA1_ASM_NOHW
#define SHA256_ASM_NOHW
#define SHA512_ASM_NOHW

#define SHA1_ASM_HW
inline int sha1_hw_capable(void) { return CRYPTO_is_ARMv8_SHA1_capable(); }

#define SHA1_ASM_NEON
void sha1_block_data_order_neon(uint32_t state[5], const uint8_t *data,
                                size_t num);

#define SHA256_ASM_HW
inline int sha256_hw_capable(void) { return CRYPTO_is_ARMv8_SHA256_capable(); }

#define SHA256_ASM_NEON
void sha256_block_data_order_neon(uint32_t state[8], const uint8_t *data,
                                  size_t num);

// Armv8.2 SHA-512 instructions are not available in 32-bit.
#define SHA512_ASM_NEON
void sha512_block_data_order_neon(uint64_t state[8], const uint8_t *data,
                                  size_t num);

#elif !defined(OPENSSL_NO_ASM) && defined(OPENSSL_AARCH64)

#define SHA1_ASM_NOHW
#define SHA256_ASM_NOHW
#define SHA512_ASM_NOHW

#define SHA1_ASM_HW
inline int sha1_hw_capable(void) { return CRYPTO_is_ARMv8_SHA1_capable(); }

#define SHA256_ASM_HW
inline int sha256_hw_capable(void) { return CRYPTO_is_ARMv8_SHA256_capable(); }

#define SHA512_ASM_HW
inline int sha512_hw_capable(void) { return CRYPTO_is_ARMv8_SHA512_capable(); }

#elif !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86)

#define SHA1_ASM_NOHW
#define SHA256_ASM_NOHW
#define SHA512_ASM_NOHW

#define SHA1_ASM_SSSE3
inline int sha1_ssse3_capable(void) {
  return CRYPTO_is_SSSE3_capable();
}
void sha1_block_data_order_ssse3(uint32_t state[5], const uint8_t *data,
                                 size_t num);

#define SHA1_ASM_AVX
inline int sha1_avx_capable(void) {
  // AMD CPUs have slow SHLD/SHRD. See also the discussion in sha1-586.pl.
  //
  // TODO(crbug.com/42290564): Should we enable SHAEXT on 32-bit x86?
  return CRYPTO_is_AVX_capable() && CRYPTO_is_intel_cpu();
}
void sha1_block_data_order_avx(uint32_t state[5], const uint8_t *data,
                               size_t num);

#define SHA256_ASM_SSSE3
inline int sha256_ssse3_capable(void) {
  return CRYPTO_is_SSSE3_capable();
}
void sha256_block_data_order_ssse3(uint32_t state[8], const uint8_t *data,
                                   size_t num);

#define SHA256_ASM_AVX
inline int sha256_avx_capable(void) {
  // AMD CPUs have slow SHLD/SHRD. See also the discussion in sha1-586.pl.
  //
  // TODO(crbug.com/42290564): Should we enable SHAEXT on 32-bit x86?
  return CRYPTO_is_AVX_capable() && CRYPTO_is_intel_cpu();
}
void sha256_block_data_order_avx(uint32_t state[8], const uint8_t *data,
                                 size_t num);

#define SHA512_ASM_SSSE3
inline int sha512_ssse3_capable(void) {
  return CRYPTO_is_SSSE3_capable();
}
void sha512_block_data_order_ssse3(uint64_t state[8], const uint8_t *data,
                                   size_t num);

#elif !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64)

#define SHA1_ASM_NOHW
#define SHA256_ASM_NOHW
#define SHA512_ASM_NOHW

#define SHA1_ASM_HW
inline int sha1_hw_capable(void) {
  return CRYPTO_is_x86_SHA_capable() && CRYPTO_is_SSSE3_capable();
}

#define SHA1_ASM_AVX2
inline int sha1_avx2_capable(void) {
  return CRYPTO_is_AVX2_capable() && CRYPTO_is_BMI2_capable() &&
         CRYPTO_is_BMI1_capable();
}
void sha1_block_data_order_avx2(uint32_t state[5], const uint8_t *data,
                                size_t num);

#define SHA1_ASM_AVX
inline int sha1_avx_capable(void) {
  // AMD CPUs have slow SHLD/SHRD. See also the discussion in sha1-586.pl. Zen
  // added the SHA extension, so this is moot on newer AMD CPUs.
  return CRYPTO_is_AVX_capable() && CRYPTO_is_intel_cpu();
}
void sha1_block_data_order_avx(uint32_t state[5], const uint8_t *data,
                               size_t num);

#define SHA1_ASM_SSSE3
inline int sha1_ssse3_capable(void) { return CRYPTO_is_SSSE3_capable(); }
void sha1_block_data_order_ssse3(uint32_t state[5], const uint8_t *data,
                                 size_t num);

#define SHA256_ASM_HW
inline int sha256_hw_capable(void) {
  // Note that the original assembly did not check SSSE3.
  return CRYPTO_is_x86_SHA_capable() && CRYPTO_is_SSSE3_capable();
}

#define SHA256_ASM_AVX
inline int sha256_avx_capable(void) {
  // AMD CPUs have slow SHLD/SHRD. See also the discussion in sha1-586.pl. Zen
  // added the SHA extension, so this is moot on newer AMD CPUs.
  return CRYPTO_is_AVX_capable() && CRYPTO_is_intel_cpu();
}
void sha256_block_data_order_avx(uint32_t state[8], const uint8_t *data,
                                 size_t num);

#define SHA256_ASM_SSSE3
inline int sha256_ssse3_capable(void) { return CRYPTO_is_SSSE3_capable(); }
void sha256_block_data_order_ssse3(uint32_t state[8], const uint8_t *data,
                                   size_t num);

#define SHA512_ASM_AVX
inline int sha512_avx_capable(void) {
  // AMD CPUs have slow SHLD/SHRD. See also the discussion in sha1-586.pl.
  //
  // TODO(crbug.com/42290564): Fixing and enabling the AVX2 implementation would
  // mitigate this on newer AMD CPUs.
  return CRYPTO_is_AVX_capable() && CRYPTO_is_intel_cpu();
}
void sha512_block_data_order_avx(uint64_t state[8], const uint8_t *data,
                                 size_t num);

#endif

#if defined(SHA1_ASM_HW)
void sha1_block_data_order_hw(uint32_t state[5], const uint8_t *data,
                              size_t num);
#endif
#if defined(SHA1_ASM_NOHW)
void sha1_block_data_order_nohw(uint32_t state[5], const uint8_t *data,
                                size_t num);
#endif

#if defined(SHA256_ASM_HW)
void sha256_block_data_order_hw(uint32_t state[8], const uint8_t *data,
                                size_t num);
#endif
#if defined(SHA256_ASM_NOHW)
void sha256_block_data_order_nohw(uint32_t state[8], const uint8_t *data,
                                  size_t num);
#endif

#if defined(SHA512_ASM_HW)
void sha512_block_data_order_hw(uint64_t state[8], const uint8_t *data,
                                size_t num);
#endif

#if defined(SHA512_ASM_NOHW)
void sha512_block_data_order_nohw(uint64_t state[8], const uint8_t *data,
                                  size_t num);
#endif

#if defined(__cplusplus)
}  // extern "C"
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_SHA_INTERNAL_H
