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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_SLHDSA_THASH_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_SLHDSA_THASH_H

#include "./params.h"

#if defined(__cplusplus)
extern "C" {
#endif


// Implements PRF_msg: a pseudo-random function that is used to generate the
// randomizer r for the randomized hashing of the message to be signed.
// (Section 4.1, page 11)
void slhdsa_thash_prfmsg(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                         const uint8_t sk_prf[BCM_SLHDSA_SHA2_128S_N],
                         const uint8_t opt_rand[BCM_SLHDSA_SHA2_128S_N],
                         const uint8_t header[BCM_SLHDSA_M_PRIME_HEADER_LEN],
                         const uint8_t *ctx, size_t ctx_len, const uint8_t *msg,
                         size_t msg_len);

// Implements H_msg: a hash function used to generate the digest of the message
// to be signed. (Section 4.1, page 11)
void slhdsa_thash_hmsg(uint8_t output[SLHDSA_SHA2_128S_DIGEST_SIZE],
                       const uint8_t r[BCM_SLHDSA_SHA2_128S_N],
                       const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                       const uint8_t pk_root[BCM_SLHDSA_SHA2_128S_N],
                       const uint8_t header[BCM_SLHDSA_M_PRIME_HEADER_LEN],
                       const uint8_t *ctx, size_t ctx_len, const uint8_t *msg,
                       size_t msg_len);

// Implements PRF: a pseudo-random function that is used to generate the secret
// values in WOTS+ and FORS private keys. (Section 4.1, page 11)
void slhdsa_thash_prf(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                      const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                      const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                      uint8_t addr[32]);

// Implements T_l: a hash function that maps an l*n-byte message to an n-byte
// message. Used for WOTS+ public key compression. (Section 4.1, page 11)
void slhdsa_thash_tl(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                     const uint8_t input[SLHDSA_SHA2_128S_WOTS_BYTES],
                     const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                     uint8_t addr[32]);

// Implements H: a hash function that takes a 2*n-byte message as input and
// produces an n-byte output. (Section 4.1, page 11)
void slhdsa_thash_h(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                    const uint8_t input[2 * BCM_SLHDSA_SHA2_128S_N],
                    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                    uint8_t addr[32]);

// Implements F: a hash function that takes an n-byte message as input and
// produces an n-byte output. (Section 4.1, page 11)
void slhdsa_thash_f(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                    const uint8_t input[BCM_SLHDSA_SHA2_128S_N],
                    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                    uint8_t addr[32]);

// Implements T_k: a hash function that maps a k*n-byte message to an n-byte
// message. Used for FORS public key compression. (Section 4.1, page 11)
void slhdsa_thash_tk(
    uint8_t output[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t input[SLHDSA_SHA2_128S_FORS_TREES * BCM_SLHDSA_SHA2_128S_N],
    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N], uint8_t addr[32]);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_SLHDSA_THASH_H
