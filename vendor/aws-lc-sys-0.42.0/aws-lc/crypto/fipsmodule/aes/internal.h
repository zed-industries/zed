// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_AES_INTERNAL_H
#define OPENSSL_HEADER_AES_INTERNAL_H

#include <stdlib.h>

#include <openssl/cpu.h>
#include "../service_indicator/internal.h"
#include "../cpucap/internal.h"
#include "../../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


#if !defined(OPENSSL_NO_ASM)

#if defined(OPENSSL_X86_64)
OPENSSL_INLINE int avx512_xts_available(void) {
  return (CRYPTO_is_VAES_capable() &&
          CRYPTO_is_VBMI2_capable() &&
          CRYPTO_is_AVX512_capable() &&
          CRYPTO_is_VPCLMULQDQ_capable());
}
#endif

#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
#define HWAES
#define HWAES_ECB

OPENSSL_INLINE int hwaes_capable(void) { return CRYPTO_is_AESNI_capable(); }

#define VPAES
#if defined(OPENSSL_X86_64)
#define VPAES_CTR32
#define HWAES_XTS
OPENSSL_INLINE int hwaes_xts_available(void) {
  return CRYPTO_is_AESNI_capable();
}
#endif
#define VPAES_CBC
OPENSSL_INLINE int vpaes_capable(void) { return CRYPTO_is_SSSE3_capable(); }

#elif defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)
#define HWAES

OPENSSL_INLINE int hwaes_capable(void) { return CRYPTO_is_ARMv8_AES_capable(); }

#if defined(OPENSSL_ARM)
#define BSAES
#define VPAES
#define VPAES_CTR32
OPENSSL_INLINE int bsaes_capable(void) { return CRYPTO_is_NEON_capable(); }
OPENSSL_INLINE int vpaes_capable(void) { return CRYPTO_is_NEON_capable(); }
#endif

#if defined(OPENSSL_AARCH64)
#define VPAES
#define VPAES_CBC
#define VPAES_CTR32
#define HWAES_XTS
OPENSSL_INLINE int vpaes_capable(void) { return CRYPTO_is_NEON_capable(); }
OPENSSL_INLINE int hwaes_xts_available(void) {
  // same as hwaes_capable()
  return CRYPTO_is_ARMv8_AES_capable();
}
#endif

#elif defined(OPENSSL_PPC64LE)
#define HWAES

OPENSSL_INLINE int hwaes_capable(void) {
  return CRYPTO_is_PPC64LE_vcrypto_capable();
}
#endif

#endif  // !NO_ASM


#if defined(HWAES)

int aes_hw_set_encrypt_key(const uint8_t *user_key, const int bits,
                           AES_KEY *key);
int aes_hw_set_decrypt_key(const uint8_t *user_key, const int bits,
                           AES_KEY *key);
void aes_hw_encrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key);
void aes_hw_decrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key);
void aes_hw_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                        const AES_KEY *key, uint8_t *ivec, const int enc);
void aes_hw_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out, size_t len,
                                 const AES_KEY *key, const uint8_t ivec[16]);

#else

// If HWAES isn't defined then we provide dummy functions for each of the hwaes
// functions.
OPENSSL_INLINE int hwaes_capable(void) { return 0; }

OPENSSL_INLINE int aes_hw_set_encrypt_key(const uint8_t *user_key, int bits,
                                          AES_KEY *key) {
  abort();
}

OPENSSL_INLINE int aes_hw_set_decrypt_key(const uint8_t *user_key, int bits,
                                          AES_KEY *key) {
  abort();
}

OPENSSL_INLINE void aes_hw_encrypt(const uint8_t *in, uint8_t *out,
                                   const AES_KEY *key) {
  abort();
}

OPENSSL_INLINE void aes_hw_decrypt(const uint8_t *in, uint8_t *out,
                                   const AES_KEY *key) {
  abort();
}

OPENSSL_INLINE void aes_hw_cbc_encrypt(const uint8_t *in, uint8_t *out,
                                       size_t length, const AES_KEY *key,
                                       uint8_t *ivec, int enc) {
  abort();
}

OPENSSL_INLINE void aes_hw_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out,
                                                size_t len, const AES_KEY *key,
                                                const uint8_t ivec[16]) {
  abort();
}

#endif  // !HWAES


#if defined(HWAES_ECB)
void aes_hw_ecb_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                        const AES_KEY *key, const int enc);
#endif  // HWAES_ECB

#if defined(HWAES_XTS)
void aes_hw_xts_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                  const AES_KEY *key1, const AES_KEY *key2,
                  const uint8_t iv[16]);
void aes_hw_xts_decrypt(const uint8_t *in, uint8_t *out, size_t length,
                  const AES_KEY *key1, const AES_KEY *key2,
                  const uint8_t iv[16]);
OPENSSL_EXPORT int aes_hw_xts_cipher(const uint8_t *in, uint8_t *out, size_t length,
                                      const AES_KEY *key1, const AES_KEY *key2,
                                      const uint8_t iv[16], int enc);

#if defined(OPENSSL_X86_64) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX) && !defined(OPENSSL_WINDOWS)
#define AES_XTS_X86_64_AVX512
void aes_hw_xts_encrypt_avx512(const uint8_t *in, uint8_t *out, size_t length,
                               const AES_KEY *key1, const AES_KEY *key2,
                               const uint8_t iv[16]);
void aes_hw_xts_decrypt_avx512(const uint8_t *in, uint8_t *out, size_t length,
                               const AES_KEY *key1, const AES_KEY *key2,
                               const uint8_t iv[16]);
int crypto_xts_avx512_enabled(void);

#endif //AES_XTS_X86_64_AVX512


#else
OPENSSL_INLINE int hwaes_xts_available(void) { return 0; }
OPENSSL_INLINE void aes_hw_xts_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                  const AES_KEY *key1, const AES_KEY *key2,
                                       const uint8_t iv[16]) {
  abort();
}
OPENSSL_INLINE void aes_hw_xts_decrypt(const uint8_t *in, uint8_t *out, size_t length,
                  const AES_KEY *key1, const AES_KEY *key2,
                  const uint8_t iv[16]) {
  abort();
}
OPENSSL_INLINE int aes_hw_xts_cipher(const uint8_t *in, uint8_t *out, size_t length,
                                      const AES_KEY *key1, const AES_KEY *key2,
                                      const uint8_t iv[16], int enc) {
  abort();
}
#endif  // HWAES_XTS

#if defined(BSAES)
// Note |bsaes_cbc_encrypt| requires |enc| to be zero.
void bsaes_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                       const AES_KEY *key, uint8_t ivec[16], int enc);
void bsaes_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out, size_t len,
                                const AES_KEY *key, const uint8_t ivec[16]);
// VPAES to BSAES conversions are available on all BSAES platforms.
void vpaes_encrypt_key_to_bsaes(AES_KEY *out_bsaes, const AES_KEY *vpaes);
void vpaes_decrypt_key_to_bsaes(AES_KEY *out_bsaes, const AES_KEY *vpaes);
#else
OPENSSL_INLINE char bsaes_capable(void) { return 0; }

// On other platforms, bsaes_capable() will always return false and so the
// following will never be called.
OPENSSL_INLINE void bsaes_cbc_encrypt(const uint8_t *in, uint8_t *out,
                                      size_t length, const AES_KEY *key,
                                      uint8_t ivec[16], int enc) {
  abort();
}

OPENSSL_INLINE void bsaes_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out,
                                               size_t len, const AES_KEY *key,
                                               const uint8_t ivec[16]) {
  abort();
}

OPENSSL_INLINE void vpaes_encrypt_key_to_bsaes(AES_KEY *out_bsaes,
                                               const AES_KEY *vpaes) {
  abort();
}

OPENSSL_INLINE void vpaes_decrypt_key_to_bsaes(AES_KEY *out_bsaes,
                                               const AES_KEY *vpaes) {
  abort();
}
#endif  // !BSAES


#if defined(VPAES)
// On platforms where VPAES gets defined (just above), then these functions are
// provided by asm.
int vpaes_set_encrypt_key(const uint8_t *userKey, int bits, AES_KEY *key);
int vpaes_set_decrypt_key(const uint8_t *userKey, int bits, AES_KEY *key);

void vpaes_encrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key);
void vpaes_decrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key);

#if defined(VPAES_CBC)
void vpaes_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                       const AES_KEY *key, uint8_t *ivec, int enc);
#endif
#if defined(VPAES_CTR32)
void vpaes_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out, size_t len,
                                const AES_KEY *key, const uint8_t ivec[16]);
#endif
#else
OPENSSL_INLINE char vpaes_capable(void) { return 0; }

// On other platforms, vpaes_capable() will always return false and so the
// following will never be called.
OPENSSL_INLINE int vpaes_set_encrypt_key(const uint8_t *userKey, int bits,
                                         AES_KEY *key) {
  abort();
}
OPENSSL_INLINE int vpaes_set_decrypt_key(const uint8_t *userKey, int bits,
                                         AES_KEY *key) {
  abort();
}
OPENSSL_INLINE void vpaes_encrypt(const uint8_t *in, uint8_t *out,
                                  const AES_KEY *key) {
  abort();
}
OPENSSL_INLINE void vpaes_decrypt(const uint8_t *in, uint8_t *out,
                                  const AES_KEY *key) {
  abort();
}
OPENSSL_INLINE void vpaes_cbc_encrypt(const uint8_t *in, uint8_t *out,
                                      size_t length, const AES_KEY *key,
                                      uint8_t *ivec, int enc) {
  abort();
}
#endif  // !VPAES


int aes_nohw_set_encrypt_key(const uint8_t *key, unsigned bits,
                             AES_KEY *aeskey);
int aes_nohw_set_decrypt_key(const uint8_t *key, unsigned bits,
                             AES_KEY *aeskey);
void aes_nohw_encrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key);
void aes_nohw_decrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key);
void aes_nohw_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out,
                                   size_t blocks, const AES_KEY *key,
                                   const uint8_t ivec[16]);
void aes_nohw_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                          const AES_KEY *key, uint8_t *ivec, const int enc);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_AES_INTERNAL_H
