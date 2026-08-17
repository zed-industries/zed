// Copyright 2024 The BoringSSL Authors
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

#include <openssl/base.h>

#include <assert.h>
#include <string.h>

#include <openssl/sha.h>

#include "../../internal.h"
#include "./params.h"
#include "./thash.h"


// Internal thash function used by F, H, and T_l (Section 11.2, pages 44-46)
static void slhdsa_thash(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                         const uint8_t *input, size_t input_blocks,
                         const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                         uint8_t addr[32]) {
  SHA256_CTX sha256;
  SHA256_Init(&sha256);

  // Process pubseed with padding to full block.
  static const uint8_t kZeros[64 - BCM_SLHDSA_SHA2_128S_N] = {0};
  SHA256_Update(&sha256, pk_seed, BCM_SLHDSA_SHA2_128S_N);
  SHA256_Update(&sha256, kZeros, sizeof(kZeros));
  SHA256_Update(&sha256, addr, SLHDSA_SHA2_128S_SHA256_ADDR_BYTES);
  SHA256_Update(&sha256, input, input_blocks * BCM_SLHDSA_SHA2_128S_N);

  uint8_t hash[32];
  SHA256_Final(hash, &sha256);
  OPENSSL_memcpy(output, hash, BCM_SLHDSA_SHA2_128S_N);
}

// Implements PRF_msg function (Section 4.1, page 11 and Section 11.2, pages
// 44-46)
void slhdsa_thash_prfmsg(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                         const uint8_t sk_prf[BCM_SLHDSA_SHA2_128S_N],
                         const uint8_t entropy[BCM_SLHDSA_SHA2_128S_N],
                         const uint8_t header[BCM_SLHDSA_M_PRIME_HEADER_LEN],
                         const uint8_t *ctx, size_t ctx_len, const uint8_t *msg,
                         size_t msg_len) {
  // Compute HMAC-SHA256(sk_prf, entropy || header || ctx || msg). We inline
  // HMAC to avoid an allocation.
  uint8_t hmac_key[SHA256_CBLOCK];
  static_assert(BCM_SLHDSA_SHA2_128S_N <= SHA256_CBLOCK,
                "HMAC key is larger than block size");
  OPENSSL_memcpy(hmac_key, sk_prf, BCM_SLHDSA_SHA2_128S_N);
  for (size_t i = 0; i < BCM_SLHDSA_SHA2_128S_N; i++) {
    hmac_key[i] ^= 0x36;
  }
  OPENSSL_memset(hmac_key + BCM_SLHDSA_SHA2_128S_N, 0x36,
                 sizeof(hmac_key) - BCM_SLHDSA_SHA2_128S_N);

  SHA256_CTX sha_ctx;
  SHA256_Init(&sha_ctx);
  SHA256_Update(&sha_ctx, hmac_key, sizeof(hmac_key));
  SHA256_Update(&sha_ctx, entropy, BCM_SLHDSA_SHA2_128S_N);
  if (header) {
    SHA256_Update(&sha_ctx, header, BCM_SLHDSA_M_PRIME_HEADER_LEN);
  }
  SHA256_Update(&sha_ctx, ctx, ctx_len);
  SHA256_Update(&sha_ctx, msg, msg_len);
  uint8_t hash[SHA256_DIGEST_LENGTH];
  SHA256_Final(hash, &sha_ctx);

  for (size_t i = 0; i < BCM_SLHDSA_SHA2_128S_N; i++) {
    hmac_key[i] ^= 0x36 ^ 0x5c;
  }
  OPENSSL_memset(hmac_key + BCM_SLHDSA_SHA2_128S_N, 0x5c,
                 sizeof(hmac_key) - BCM_SLHDSA_SHA2_128S_N);

  SHA256_Init(&sha_ctx);
  SHA256_Update(&sha_ctx, hmac_key, sizeof(hmac_key));
  SHA256_Update(&sha_ctx, hash, sizeof(hash));
  SHA256_Final(hash, &sha_ctx);

  // Truncate to BCM_SLHDSA_SHA2_128S_N bytes
  OPENSSL_memcpy(output, hash, BCM_SLHDSA_SHA2_128S_N);
}

// Implements H_msg function (Section 4.1, page 11 and Section 11.2, pages
// 44-46)
void slhdsa_thash_hmsg(uint8_t output[SLHDSA_SHA2_128S_DIGEST_SIZE],
                       const uint8_t r[BCM_SLHDSA_SHA2_128S_N],
                       const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                       const uint8_t pk_root[BCM_SLHDSA_SHA2_128S_N],
                       const uint8_t header[BCM_SLHDSA_M_PRIME_HEADER_LEN],
                       const uint8_t *ctx, size_t ctx_len, const uint8_t *msg,
                       size_t msg_len) {
  // MGF1-SHA-256(R || PK.seed || SHA-256(R || PK.seed || PK.root || header ||
  // ctx || M), m) input_buffer stores R || PK_SEED || SHA256(..) || 4-byte
  // index
  uint8_t input_buffer[2 * BCM_SLHDSA_SHA2_128S_N + 32 + 4] = {0};
  OPENSSL_memcpy(input_buffer, r, BCM_SLHDSA_SHA2_128S_N);
  OPENSSL_memcpy(input_buffer + BCM_SLHDSA_SHA2_128S_N, pk_seed,
                 BCM_SLHDSA_SHA2_128S_N);

  // Inner hash
  SHA256_CTX sha_ctx;
  SHA256_Init(&sha_ctx);
  SHA256_Update(&sha_ctx, r, BCM_SLHDSA_SHA2_128S_N);
  SHA256_Update(&sha_ctx, pk_seed, BCM_SLHDSA_SHA2_128S_N);
  SHA256_Update(&sha_ctx, pk_root, BCM_SLHDSA_SHA2_128S_N);
  if (header) {
    SHA256_Update(&sha_ctx, header, BCM_SLHDSA_M_PRIME_HEADER_LEN);
  }
  SHA256_Update(&sha_ctx, ctx, ctx_len);
  SHA256_Update(&sha_ctx, msg, msg_len);
  // Write directly into the input buffer
  SHA256_Final(input_buffer + 2 * BCM_SLHDSA_SHA2_128S_N, &sha_ctx);

  // MGF1-SHA-256
  uint8_t hash[32];
  static_assert(SLHDSA_SHA2_128S_DIGEST_SIZE < sizeof(hash),
                "More MGF1 iterations required");
  SHA256(input_buffer, sizeof(input_buffer), hash);
  OPENSSL_memcpy(output, hash, SLHDSA_SHA2_128S_DIGEST_SIZE);
}

// Implements PRF function (Section 4.1, page 11 and Section 11.2, pages 44-46)
void slhdsa_thash_prf(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                      const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                      const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                      uint8_t addr[32]) {
  slhdsa_thash(output, sk_seed, 1, pk_seed, addr);
}

// Implements T_l function for WOTS+ public key compression (Section 4.1, page
// 11 and Section 11.2, pages 44-46)
void slhdsa_thash_tl(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                     const uint8_t input[SLHDSA_SHA2_128S_WOTS_BYTES],
                     const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                     uint8_t addr[32]) {
  slhdsa_thash(output, input, SLHDSA_SHA2_128S_WOTS_LEN, pk_seed, addr);
}

// Implements H function (Section 4.1, page 11 and Section 11.2, pages 44-46)
void slhdsa_thash_h(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                    const uint8_t input[2 * BCM_SLHDSA_SHA2_128S_N],
                    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                    uint8_t addr[32]) {
  slhdsa_thash(output, input, 2, pk_seed, addr);
}

// Implements F function (Section 4.1, page 11 and Section 11.2, pages 44-46)
void slhdsa_thash_f(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                    const uint8_t input[BCM_SLHDSA_SHA2_128S_N],
                    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                    uint8_t addr[32]) {
  slhdsa_thash(output, input, 1, pk_seed, addr);
}

// Implements T_k function for FORS public key compression (Section 4.1, page 11
// and Section 11.2, pages 44-46)
void slhdsa_thash_tk(
    uint8_t output[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t input[SLHDSA_SHA2_128S_FORS_TREES * BCM_SLHDSA_SHA2_128S_N],
    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N], uint8_t addr[32]) {
  slhdsa_thash(output, input, SLHDSA_SHA2_128S_FORS_TREES, pk_seed, addr);
}
