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

#ifndef OPENSSL_HEADER_CRYPTO_CIPHER_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_CIPHER_INTERNAL_H

#include <assert.h>
#include <stdlib.h>

#include <openssl/base.h>

#include "../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


// EVP_tls_cbc_get_padding determines the padding from the decrypted, TLS, CBC
// record in |in|. This decrypted record should not include any "decrypted"
// explicit IV. If the record is publicly invalid, it returns zero. Otherwise,
// it returns one and sets |*out_padding_ok| to all ones (0xfff..f) if the
// padding is valid and zero otherwise. It then sets |*out_len| to the length
// with the padding removed or |in_len| if invalid.
//
// If the function returns one, it runs in time independent of the contents of
// |in|. It is also guaranteed that |*out_len| >= |mac_size|, satisfying
// |EVP_tls_cbc_copy_mac|'s precondition.
int EVP_tls_cbc_remove_padding(crypto_word_t *out_padding_ok, size_t *out_len,
                               const uint8_t *in, size_t in_len,
                               size_t block_size, size_t mac_size);

// EVP_tls_cbc_copy_mac copies |md_size| bytes from the end of the first
// |in_len| bytes of |in| to |out| in constant time (independent of the concrete
// value of |in_len|, which may vary within a 256-byte window). |in| must point
// to a buffer of |orig_len| bytes.
//
// On entry:
//   orig_len >= in_len >= md_size
//   md_size <= EVP_MAX_MD_SIZE
void EVP_tls_cbc_copy_mac(uint8_t *out, size_t md_size, const uint8_t *in,
                          size_t in_len, size_t orig_len);

// EVP_tls_cbc_record_digest_supported returns 1 iff |md| is a hash function
// which EVP_tls_cbc_digest_record supports.
int EVP_tls_cbc_record_digest_supported(const EVP_MD *md);

// EVP_sha1_final_with_secret_suffix computes the result of hashing |len| bytes
// from |in| to |ctx| and writes the resulting hash to |out|. |len| is treated
// as secret and must be at most |max_len|, which is treated as public. |in|
// must point to a buffer of at least |max_len| bytes. It returns one on success
// and zero if inputs are too long.
//
// This function is exported for unit tests.
OPENSSL_EXPORT int EVP_sha1_final_with_secret_suffix(
    SHA_CTX *ctx, uint8_t out[SHA_DIGEST_LENGTH], const uint8_t *in, size_t len,
    size_t max_len);

// EVP_sha256_final_with_secret_suffix acts like
// |EVP_sha1_final_with_secret_suffix|, but for SHA-256.
//
// This function is exported for unit tests.
OPENSSL_EXPORT int EVP_sha256_final_with_secret_suffix(
    SHA256_CTX *ctx, uint8_t out[SHA256_DIGEST_LENGTH], const uint8_t *in,
    size_t len, size_t max_len);

// EVP_tls_cbc_digest_record computes the MAC of a decrypted, padded TLS
// record.
//
//   md: the hash function used in the HMAC.
//     EVP_tls_cbc_record_digest_supported must return true for this hash.
//   md_out: the digest output. At most EVP_MAX_MD_SIZE bytes will be written.
//   md_out_size: the number of output bytes is written here.
//   header: the 13-byte, TLS record header.
//   data: the record data itself
//   data_size: the secret, reported length of the data once the padding and MAC
//     have been removed.
//   data_plus_mac_plus_padding_size: the public length of the whole
//     record, including padding.
//
// On entry: by virtue of having been through one of the remove_padding
// functions, above, we know that data_plus_mac_size is large enough to contain
// a padding byte and MAC. (If the padding was invalid, it might contain the
// padding too. )
int EVP_tls_cbc_digest_record(const EVP_MD *md, uint8_t *md_out,
                              size_t *md_out_size, const uint8_t header[13],
                              const uint8_t *data, size_t data_size,
                              size_t data_plus_mac_plus_padding_size,
                              const uint8_t *mac_secret,
                              unsigned mac_secret_length);

#define POLY1305_TAG_LEN 16

// For convenience (the x86_64 calling convention allows only six parameters in
// registers), the final parameter for the assembly functions is both an input
// and output parameter.
union chacha20_poly1305_open_data {
  struct {
    alignas(16) uint8_t key[32];
    uint32_t counter;
    uint8_t nonce[12];
  } in;
  struct {
    uint8_t tag[POLY1305_TAG_LEN];
  } out;
};

union chacha20_poly1305_seal_data {
  struct {
    alignas(16) uint8_t key[32];
    uint32_t counter;
    uint8_t nonce[12];
    const uint8_t *extra_ciphertext;
    size_t extra_ciphertext_len;
  } in;
  struct {
    uint8_t tag[POLY1305_TAG_LEN];
  } out;
};

#if (defined(OPENSSL_X86_64) || defined(OPENSSL_AARCH64)) &&  \
    !defined(OPENSSL_NO_ASM)

static_assert(sizeof(union chacha20_poly1305_open_data) == 48,
              "wrong chacha20_poly1305_open_data size");
static_assert(sizeof(union chacha20_poly1305_seal_data) == 48 + 8 + 8,
              "wrong chacha20_poly1305_seal_data size");

inline int chacha20_poly1305_asm_capable(void) {
#if defined(OPENSSL_X86_64)
  return CRYPTO_is_SSE4_1_capable();
#elif defined(OPENSSL_AARCH64)
  return CRYPTO_is_NEON_capable();
#endif
}

// chacha20_poly1305_open is defined in chacha20_poly1305_*.pl. It decrypts
// |plaintext_len| bytes from |ciphertext| and writes them to |out_plaintext|.
// Additional input parameters are passed in |aead_data->in|. On exit, it will
// write calculated tag value to |aead_data->out.tag|, which the caller must
// check.
#if defined(OPENSSL_X86_64)
extern void chacha20_poly1305_open_sse41(
    uint8_t *out_plaintext, const uint8_t *ciphertext, size_t plaintext_len,
    const uint8_t *ad, size_t ad_len, union chacha20_poly1305_open_data *data);
extern void chacha20_poly1305_open_avx2(
    uint8_t *out_plaintext, const uint8_t *ciphertext, size_t plaintext_len,
    const uint8_t *ad, size_t ad_len, union chacha20_poly1305_open_data *data);
inline void chacha20_poly1305_open(uint8_t *out_plaintext,
                                   const uint8_t *ciphertext,
                                   size_t plaintext_len, const uint8_t *ad,
                                   size_t ad_len,
                                   union chacha20_poly1305_open_data *data) {
  if (CRYPTO_is_AVX2_capable() && CRYPTO_is_BMI2_capable()) {
    chacha20_poly1305_open_avx2(out_plaintext, ciphertext, plaintext_len, ad,
                                ad_len, data);
  } else {
    chacha20_poly1305_open_sse41(out_plaintext, ciphertext, plaintext_len, ad,
                                 ad_len, data);
  }
}
#else
extern void chacha20_poly1305_open(uint8_t *out_plaintext,
                                   const uint8_t *ciphertext,
                                   size_t plaintext_len, const uint8_t *ad,
                                   size_t ad_len,
                                   union chacha20_poly1305_open_data *data);
#endif

// chacha20_poly1305_open is defined in chacha20_poly1305_*.pl. It encrypts
// |plaintext_len| bytes from |plaintext| and writes them to |out_ciphertext|.
// Additional input parameters are passed in |aead_data->in|. The calculated tag
// value is over the computed ciphertext concatenated with |extra_ciphertext|
// and written to |aead_data->out.tag|.
#if defined(OPENSSL_X86_64)
extern void chacha20_poly1305_seal_sse41(
    uint8_t *out_ciphertext, const uint8_t *plaintext, size_t plaintext_len,
    const uint8_t *ad, size_t ad_len, union chacha20_poly1305_seal_data *data);
extern void chacha20_poly1305_seal_avx2(
    uint8_t *out_ciphertext, const uint8_t *plaintext, size_t plaintext_len,
    const uint8_t *ad, size_t ad_len, union chacha20_poly1305_seal_data *data);
inline void chacha20_poly1305_seal(uint8_t *out_ciphertext,
                                   const uint8_t *plaintext,
                                   size_t plaintext_len, const uint8_t *ad,
                                   size_t ad_len,
                                   union chacha20_poly1305_seal_data *data) {
  if (CRYPTO_is_AVX2_capable() && CRYPTO_is_BMI2_capable()) {
    chacha20_poly1305_seal_avx2(out_ciphertext, plaintext, plaintext_len, ad,
                                ad_len, data);
  } else {
    chacha20_poly1305_seal_sse41(out_ciphertext, plaintext, plaintext_len, ad,
                                 ad_len, data);
  }
}
#else
extern void chacha20_poly1305_seal(uint8_t *out_ciphertext,
                                   const uint8_t *plaintext,
                                   size_t plaintext_len, const uint8_t *ad,
                                   size_t ad_len,
                                   union chacha20_poly1305_seal_data *data);
#endif

#else

inline int chacha20_poly1305_asm_capable(void) { return 0; }

inline void chacha20_poly1305_open(uint8_t *out_plaintext,
                                   const uint8_t *ciphertext,
                                   size_t plaintext_len, const uint8_t *ad,
                                   size_t ad_len,
                                   union chacha20_poly1305_open_data *data) {
  abort();
}

inline void chacha20_poly1305_seal(uint8_t *out_ciphertext,
                                   const uint8_t *plaintext,
                                   size_t plaintext_len, const uint8_t *ad,
                                   size_t ad_len,
                                   union chacha20_poly1305_seal_data *data) {
  abort();
}
#endif


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_CIPHER_INTERNAL_H
