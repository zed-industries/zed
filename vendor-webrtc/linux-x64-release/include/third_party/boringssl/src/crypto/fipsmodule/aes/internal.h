// Copyright 2017 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_AES_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_AES_INTERNAL_H

#include <stdlib.h>

#include "../bcm_interface.h"
#include "../../internal.h"

extern "C" {


// block128_f is the type of an AES block cipher implementation.
//
// Unlike upstream OpenSSL, it and the other functions in this file hard-code
// |AES_KEY|. It is undefined in C to call a function pointer with anything
// other than the original type. Thus we either must match |block128_f| to the
// type signature of |BCM_aes_encrypt| and friends or pass in |void*| wrapper
// functions.
//
// These functions are called exclusively with AES, so we use the former.
typedef void (*block128_f)(const uint8_t in[16], uint8_t out[16],
                           const AES_KEY *key);

// ctr128_f is the type of a function that performs CTR-mode encryption.
typedef void (*ctr128_f)(const uint8_t *in, uint8_t *out, size_t blocks,
                         const AES_KEY *key, const uint8_t ivec[16]);

// aes_ctr_set_key initialises |*aes_key| using |key_bytes| bytes from |key|,
// where |key_bytes| must either be 16, 24 or 32. If not NULL, |*out_block| is
// set to a function that encrypts single blocks. If not NULL, |*out_is_hwaes|
// is set to whether the hardware AES implementation was used. It returns a
// function for optimised CTR-mode.
ctr128_f aes_ctr_set_key(AES_KEY *aes_key, int *out_is_hwaes,
                         block128_f *out_block, const uint8_t *key,
                         size_t key_bytes);


// AES implementations.

#if !defined(OPENSSL_NO_ASM)

#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
#define HWAES
#define HWAES_ECB

inline int hwaes_capable(void) { return CRYPTO_is_AESNI_capable(); }

#define VPAES
#define VPAES_CBC
inline int vpaes_capable(void) { return CRYPTO_is_SSSE3_capable(); }

#elif defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)
#define HWAES

inline int hwaes_capable(void) { return CRYPTO_is_ARMv8_AES_capable(); }

#if defined(OPENSSL_ARM)
#define BSAES
#define VPAES
inline int bsaes_capable(void) { return CRYPTO_is_NEON_capable(); }
inline int vpaes_capable(void) { return CRYPTO_is_NEON_capable(); }
#endif

#if defined(OPENSSL_AARCH64)
#define VPAES
#define VPAES_CBC
inline int vpaes_capable(void) { return CRYPTO_is_NEON_capable(); }
#endif

#endif

#endif  // !NO_ASM


#if defined(HWAES)

int aes_hw_set_encrypt_key(const uint8_t *user_key, int bits, AES_KEY *key);
int aes_hw_set_decrypt_key(const uint8_t *user_key, int bits, AES_KEY *key);
void aes_hw_encrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key);
void aes_hw_decrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key);
void aes_hw_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                        const AES_KEY *key, uint8_t *ivec, int enc);
void aes_hw_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out, size_t len,
                                 const AES_KEY *key, const uint8_t ivec[16]);

#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
// On x86 and x86_64, |aes_hw_set_decrypt_key| is implemented in terms of
// |aes_hw_set_encrypt_key| and a conversion function.
void aes_hw_encrypt_key_to_decrypt_key(AES_KEY *key);

// There are two variants of this function, one which uses aeskeygenassist
// ("base") and one which uses aesenclast + pshufb ("alt"). aesenclast is
// overall faster but is slower on some older processors. It doesn't use AVX,
// but AVX is used as a proxy to detecting this. See
// https://groups.google.com/g/mailing.openssl.dev/c/OuFXwW4NfO8/m/7d2ZXVjkxVkJ
//
// TODO(davidben): It is unclear if the aeskeygenassist version is still
// worthwhile. However, the aesenclast version requires SSSE3. SSSE3 long
// predates AES-NI, but it's not clear if AES-NI implies SSSE3. In OpenSSL, the
// CCM AES-NI assembly seems to assume it does.
inline int aes_hw_set_encrypt_key_alt_capable(void) {
  return hwaes_capable() && CRYPTO_is_SSSE3_capable();
}
inline int aes_hw_set_encrypt_key_alt_preferred(void) {
  return hwaes_capable() && CRYPTO_is_AVX_capable();
}
int aes_hw_set_encrypt_key_base(const uint8_t *user_key, int bits,
                                AES_KEY *key);
int aes_hw_set_encrypt_key_alt(const uint8_t *user_key, int bits, AES_KEY *key);
#endif  // OPENSSL_X86 || OPENSSL_X86_64

#else

// If HWAES isn't defined then we provide dummy functions for each of the hwaes
// functions.
inline int hwaes_capable(void) { return 0; }

inline int aes_hw_set_encrypt_key(const uint8_t *user_key, int bits,
                                  AES_KEY *key) {
  abort();
}

inline int aes_hw_set_decrypt_key(const uint8_t *user_key, int bits,
                                  AES_KEY *key) {
  abort();
}

inline void aes_hw_encrypt(const uint8_t *in, uint8_t *out,
                           const AES_KEY *key) {
  abort();
}

inline void aes_hw_decrypt(const uint8_t *in, uint8_t *out,
                           const AES_KEY *key) {
  abort();
}

inline void aes_hw_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                               const AES_KEY *key, uint8_t *ivec, int enc) {
  abort();
}

inline void aes_hw_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out,
                                        size_t len, const AES_KEY *key,
                                        const uint8_t ivec[16]) {
  abort();
}

#endif  // !HWAES


#if defined(HWAES_ECB)
void aes_hw_ecb_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                        const AES_KEY *key, int enc);
#endif  // HWAES_ECB


#if defined(BSAES)
// Note |bsaes_cbc_encrypt| requires |enc| to be zero.
void bsaes_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                       const AES_KEY *key, uint8_t ivec[16], int enc);
void bsaes_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out, size_t len,
                                const AES_KEY *key, const uint8_t ivec[16]);
// VPAES to BSAES conversions are available on all BSAES platforms.
void vpaes_encrypt_key_to_bsaes(AES_KEY *out_bsaes, const AES_KEY *vpaes);
void vpaes_decrypt_key_to_bsaes(AES_KEY *out_bsaes, const AES_KEY *vpaes);
void vpaes_ctr32_encrypt_blocks_with_bsaes(const uint8_t *in, uint8_t *out,
                                           size_t blocks, const AES_KEY *key,
                                           const uint8_t ivec[16]);
#else
inline int bsaes_capable(void) { return 0; }

// On other platforms, bsaes_capable() will always return false and so the
// following will never be called.
inline void bsaes_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                              const AES_KEY *key, uint8_t ivec[16], int enc) {
  abort();
}

inline void bsaes_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out,
                                       size_t len, const AES_KEY *key,
                                       const uint8_t ivec[16]) {
  abort();
}

inline void vpaes_encrypt_key_to_bsaes(AES_KEY *out_bsaes,
                                       const AES_KEY *vpaes) {
  abort();
}

inline void vpaes_decrypt_key_to_bsaes(AES_KEY *out_bsaes,
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
void vpaes_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out, size_t len,
                                const AES_KEY *key, const uint8_t ivec[16]);
#else
inline int vpaes_capable(void) { return 0; }

// On other platforms, vpaes_capable() will always return false and so the
// following will never be called.
inline int vpaes_set_encrypt_key(const uint8_t *userKey, int bits,
                                 AES_KEY *key) {
  abort();
}
inline int vpaes_set_decrypt_key(const uint8_t *userKey, int bits,
                                 AES_KEY *key) {
  abort();
}
inline void vpaes_encrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key) {
  abort();
}
inline void vpaes_decrypt(const uint8_t *in, uint8_t *out, const AES_KEY *key) {
  abort();
}
inline void vpaes_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                              const AES_KEY *key, uint8_t *ivec, int enc) {
  abort();
}
inline void vpaes_ctr32_encrypt_blocks(const uint8_t *in, uint8_t *out,
                                       size_t len, const AES_KEY *key,
                                       const uint8_t ivec[16]) {
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
                          const AES_KEY *key, uint8_t *ivec, int enc);

// Modes

inline void CRYPTO_xor16(uint8_t out[16], const uint8_t a[16],
                         const uint8_t b[16]) {
  // TODO(davidben): Ideally we'd leave this to the compiler, which could use
  // vector registers, etc. But the compiler doesn't know that |in| and |out|
  // cannot partially alias. |restrict| is slightly two strict (we allow exact
  // aliasing), but perhaps in-place could be a separate function?
  static_assert(16 % sizeof(crypto_word_t) == 0,
                "block cannot be evenly divided into words");
  for (size_t i = 0; i < 16; i += sizeof(crypto_word_t)) {
    CRYPTO_store_word_le(
        out + i, CRYPTO_load_word_le(a + i) ^ CRYPTO_load_word_le(b + i));
  }
}


// CTR.

// CRYPTO_ctr128_encrypt_ctr32 encrypts (or decrypts, it's the same in CTR mode)
// |len| bytes from |in| to |out| using |block| in counter mode. There's no
// requirement that |len| be a multiple of any value and any partial blocks are
// stored in |ecount_buf| and |*num|, which must be zeroed before the initial
// call. The counter is a 128-bit, big-endian value in |ivec| and is
// incremented by this function. If the counter overflows, it wraps around.
// |ctr| must be a function that performs CTR mode but only deals with the lower
// 32 bits of the counter.
void CRYPTO_ctr128_encrypt_ctr32(const uint8_t *in, uint8_t *out, size_t len,
                                 const AES_KEY *key, uint8_t ivec[16],
                                 uint8_t ecount_buf[16], unsigned *num,
                                 ctr128_f ctr);


// GCM.
//
// This API differs from the upstream API slightly. The |GCM128_CONTEXT| does
// not have a |key| pointer that points to the key as upstream's version does.
// Instead, every function takes a |key| parameter. This way |GCM128_CONTEXT|
// can be safely copied. Additionally, |gcm_key| is split into a separate
// struct.

// gcm_impl_t specifies an assembly implementation of AES-GCM.
enum gcm_impl_t {
  gcm_separate = 0,  // No combined AES-GCM, but may have AES-CTR and GHASH.
  gcm_x86_aesni,
  gcm_x86_vaes_avx2,
  gcm_x86_vaes_avx512,
  gcm_arm64_aes,
};

typedef struct { uint64_t hi,lo; } u128;

// gmult_func multiplies |Xi| by the GCM key and writes the result back to
// |Xi|.
typedef void (*gmult_func)(uint8_t Xi[16], const u128 Htable[16]);

// ghash_func repeatedly multiplies |Xi| by the GCM key and adds in blocks from
// |inp|. The result is written back to |Xi| and the |len| argument must be a
// multiple of 16.
typedef void (*ghash_func)(uint8_t Xi[16], const u128 Htable[16],
                           const uint8_t *inp, size_t len);

typedef struct gcm128_key_st {
  u128 Htable[16];
  gmult_func gmult;
  ghash_func ghash;
  AES_KEY aes;

  ctr128_f ctr;
  block128_f block;
  enum gcm_impl_t impl;
} GCM128_KEY;

// GCM128_CONTEXT contains state for a single GCM operation. The structure
// should be zero-initialized before use.
typedef struct {
  // The following 5 names follow names in GCM specification
  uint8_t Yi[16];
  uint8_t EKi[16];
  uint8_t EK0[16];
  struct {
    uint64_t aad;
    uint64_t msg;
  } len;
  uint8_t Xi[16];
  unsigned mres, ares;
} GCM128_CONTEXT;

#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
// crypto_gcm_clmul_enabled returns one if the CLMUL implementation of GCM is
// used.
int crypto_gcm_clmul_enabled(void);
#endif

// CRYPTO_ghash_init writes a precomputed table of powers of |gcm_key| to
// |out_table| and sets |*out_mult| and |*out_hash| to (potentially hardware
// accelerated) functions for performing operations in the GHASH field.
void CRYPTO_ghash_init(gmult_func *out_mult, ghash_func *out_hash,
                       u128 out_table[16], const uint8_t gcm_key[16]);

// CRYPTO_gcm128_init_aes_key initialises |gcm_key| to with AES key |key|.
void CRYPTO_gcm128_init_aes_key(GCM128_KEY *gcm_key, const uint8_t *key,
                                size_t key_bytes);

// CRYPTO_gcm128_init_ctx initializes |ctx| to encrypt with |key| and |iv|.
void CRYPTO_gcm128_init_ctx(const GCM128_KEY *key, GCM128_CONTEXT *ctx,
                            const uint8_t *iv, size_t iv_len);

// CRYPTO_gcm128_aad adds to the authenticated data for an instance of GCM.
// This must be called before and data is encrypted. |key| must be the same
// value that was passed to |CRYPTO_gcm128_init_ctx|. It returns one on success
// and zero otherwise.
int CRYPTO_gcm128_aad(const GCM128_KEY *key, GCM128_CONTEXT *ctx,
                      const uint8_t *aad, size_t aad_len);

// CRYPTO_gcm128_encrypt encrypts |len| bytes from |in| to |out|. |key| must be
// the same value that was passed to |CRYPTO_gcm128_init_ctx|. It returns one on
// success and zero otherwise.
int CRYPTO_gcm128_encrypt(const GCM128_KEY *key, GCM128_CONTEXT *ctx,
                          const uint8_t *in, uint8_t *out, size_t len);

// CRYPTO_gcm128_decrypt decrypts |len| bytes from |in| to |out|. |key| must be
// the same value that was passed to |CRYPTO_gcm128_init_ctx|. It returns one on
// success and zero otherwise.
int CRYPTO_gcm128_decrypt(const GCM128_KEY *key, GCM128_CONTEXT *ctx,
                          const uint8_t *in, uint8_t *out, size_t len);

// CRYPTO_gcm128_finish calculates the authenticator and compares it against
// |len| bytes of |tag|. |key| must be the same value that was passed to
// |CRYPTO_gcm128_init_ctx|. It returns one on success and zero otherwise.
int CRYPTO_gcm128_finish(const GCM128_KEY *key, GCM128_CONTEXT *ctx,
                         const uint8_t *tag, size_t len);

// CRYPTO_gcm128_tag calculates the authenticator and copies it into |tag|.
// The minimum of |len| and 16 bytes are copied into |tag|. |key| must be the
// same value that was passed to |CRYPTO_gcm128_init_ctx|.
void CRYPTO_gcm128_tag(const GCM128_KEY *key, GCM128_CONTEXT *ctx, uint8_t *tag,
                       size_t len);


// GCM assembly.

void gcm_init_nohw(u128 Htable[16], const uint64_t H[2]);
void gcm_gmult_nohw(uint8_t Xi[16], const u128 Htable[16]);
void gcm_ghash_nohw(uint8_t Xi[16], const u128 Htable[16], const uint8_t *inp,
                    size_t len);

#if !defined(OPENSSL_NO_ASM)

#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
#define GCM_FUNCREF
void gcm_init_clmul(u128 Htable[16], const uint64_t Xi[2]);
void gcm_gmult_clmul(uint8_t Xi[16], const u128 Htable[16]);
void gcm_ghash_clmul(uint8_t Xi[16], const u128 Htable[16], const uint8_t *inp,
                     size_t len);

void gcm_init_ssse3(u128 Htable[16], const uint64_t Xi[2]);
void gcm_gmult_ssse3(uint8_t Xi[16], const u128 Htable[16]);
void gcm_ghash_ssse3(uint8_t Xi[16], const u128 Htable[16], const uint8_t *in,
                     size_t len);

#if defined(OPENSSL_X86_64)
#define GHASH_ASM_X86_64
void gcm_init_avx(u128 Htable[16], const uint64_t Xi[2]);
void gcm_gmult_avx(uint8_t Xi[16], const u128 Htable[16]);
void gcm_ghash_avx(uint8_t Xi[16], const u128 Htable[16], const uint8_t *in,
                   size_t len);

#define HW_GCM
size_t aesni_gcm_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                         const AES_KEY *key, uint8_t ivec[16],
                         const u128 Htable[16], uint8_t Xi[16]);
size_t aesni_gcm_decrypt(const uint8_t *in, uint8_t *out, size_t len,
                         const AES_KEY *key, uint8_t ivec[16],
                         const u128 Htable[16], uint8_t Xi[16]);

void gcm_init_vpclmulqdq_avx2(u128 Htable[16], const uint64_t H[2]);
void gcm_gmult_vpclmulqdq_avx2(uint8_t Xi[16], const u128 Htable[16]);
void gcm_ghash_vpclmulqdq_avx2(uint8_t Xi[16], const u128 Htable[16],
                               const uint8_t *in, size_t len);
void aes_gcm_enc_update_vaes_avx2(const uint8_t *in, uint8_t *out, size_t len,
                                  const AES_KEY *key, const uint8_t ivec[16],
                                  const u128 Htable[16], uint8_t Xi[16]);
void aes_gcm_dec_update_vaes_avx2(const uint8_t *in, uint8_t *out, size_t len,
                                  const AES_KEY *key, const uint8_t ivec[16],
                                  const u128 Htable[16], uint8_t Xi[16]);

void gcm_init_vpclmulqdq_avx512(u128 Htable[16], const uint64_t H[2]);
void gcm_gmult_vpclmulqdq_avx512(uint8_t Xi[16], const u128 Htable[16]);
void gcm_ghash_vpclmulqdq_avx512(uint8_t Xi[16], const u128 Htable[16],
                                 const uint8_t *in, size_t len);
void aes_gcm_enc_update_vaes_avx512(const uint8_t *in, uint8_t *out, size_t len,
                                    const AES_KEY *key, const uint8_t ivec[16],
                                    const u128 Htable[16], uint8_t Xi[16]);
void aes_gcm_dec_update_vaes_avx512(const uint8_t *in, uint8_t *out, size_t len,
                                    const AES_KEY *key, const uint8_t ivec[16],
                                    const u128 Htable[16], uint8_t Xi[16]);

#endif  // OPENSSL_X86_64

#if defined(OPENSSL_X86)
#define GHASH_ASM_X86
#endif  // OPENSSL_X86

#elif defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)

#define GHASH_ASM_ARM
#define GCM_FUNCREF

inline int gcm_pmull_capable(void) { return CRYPTO_is_ARMv8_PMULL_capable(); }

void gcm_init_v8(u128 Htable[16], const uint64_t H[2]);
void gcm_gmult_v8(uint8_t Xi[16], const u128 Htable[16]);
void gcm_ghash_v8(uint8_t Xi[16], const u128 Htable[16], const uint8_t *inp,
                  size_t len);

inline int gcm_neon_capable(void) { return CRYPTO_is_NEON_capable(); }

void gcm_init_neon(u128 Htable[16], const uint64_t H[2]);
void gcm_gmult_neon(uint8_t Xi[16], const u128 Htable[16]);
void gcm_ghash_neon(uint8_t Xi[16], const u128 Htable[16], const uint8_t *inp,
                    size_t len);

#if defined(OPENSSL_AARCH64)
#define HW_GCM
// These functions are defined in aesv8-gcm-armv8.pl.
void aes_gcm_enc_kernel(const uint8_t *in, uint64_t in_bits, void *out,
                        void *Xi, uint8_t *ivec, const AES_KEY *key,
                        const u128 Htable[16]);
void aes_gcm_dec_kernel(const uint8_t *in, uint64_t in_bits, void *out,
                        void *Xi, uint8_t *ivec, const AES_KEY *key,
                        const u128 Htable[16]);
#endif

#endif
#endif  // OPENSSL_NO_ASM


// CBC.

// cbc128_f is the type of a function that performs CBC-mode encryption.
typedef void (*cbc128_f)(const uint8_t *in, uint8_t *out, size_t len,
                         const AES_KEY *key, uint8_t ivec[16], int enc);

// CRYPTO_cbc128_encrypt encrypts |len| bytes from |in| to |out| using the
// given IV and block cipher in CBC mode. The input need not be a multiple of
// 128 bits long, but the output will round up to the nearest 128 bit multiple,
// zero padding the input if needed. The IV will be updated on return.
void CRYPTO_cbc128_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                           const AES_KEY *key, uint8_t ivec[16],
                           block128_f block);

// CRYPTO_cbc128_decrypt decrypts |len| bytes from |in| to |out| using the
// given IV and block cipher in CBC mode. If |len| is not a multiple of 128
// bits then only that many bytes will be written, but a multiple of 128 bits
// is always read from |in|. The IV will be updated on return.
void CRYPTO_cbc128_decrypt(const uint8_t *in, uint8_t *out, size_t len,
                           const AES_KEY *key, uint8_t ivec[16],
                           block128_f block);


// OFB.

// CRYPTO_ofb128_encrypt encrypts (or decrypts, it's the same with OFB mode)
// |len| bytes from |in| to |out| using |block| in OFB mode. There's no
// requirement that |len| be a multiple of any value and any partial blocks are
// stored in |ivec| and |*num|, the latter must be zero before the initial
// call.
void CRYPTO_ofb128_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                           const AES_KEY *key, uint8_t ivec[16], unsigned *num,
                           block128_f block);


// CFB.

// CRYPTO_cfb128_encrypt encrypts (or decrypts, if |enc| is zero) |len| bytes
// from |in| to |out| using |block| in CFB mode. There's no requirement that
// |len| be a multiple of any value and any partial blocks are stored in |ivec|
// and |*num|, the latter must be zero before the initial call.
void CRYPTO_cfb128_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                           const AES_KEY *key, uint8_t ivec[16], unsigned *num,
                           int enc, block128_f block);

// CRYPTO_cfb128_8_encrypt encrypts (or decrypts, if |enc| is zero) |len| bytes
// from |in| to |out| using |block| in CFB-8 mode. Prior to the first call
// |num| should be set to zero.
void CRYPTO_cfb128_8_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                             const AES_KEY *key, uint8_t ivec[16],
                             unsigned *num, int enc, block128_f block);

// CRYPTO_cfb128_1_encrypt encrypts (or decrypts, if |enc| is zero) |len| bytes
// from |in| to |out| using |block| in CFB-1 mode. Prior to the first call
// |num| should be set to zero.
void CRYPTO_cfb128_1_encrypt(const uint8_t *in, uint8_t *out, size_t bits,
                             const AES_KEY *key, uint8_t ivec[16],
                             unsigned *num, int enc, block128_f block);

size_t CRYPTO_cts128_encrypt_block(const uint8_t *in, uint8_t *out, size_t len,
                                   const AES_KEY *key, uint8_t ivec[16],
                                   block128_f block);


}  // extern C

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_AES_INTERNAL_H
