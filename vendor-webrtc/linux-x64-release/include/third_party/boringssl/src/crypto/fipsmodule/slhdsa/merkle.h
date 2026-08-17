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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_SLHDSA_MERKLE_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_SLHDSA_MERKLE_H

#include <openssl/base.h>

#include <sys/types.h>

#include "./params.h"

#if defined(__cplusplus)
extern "C" {
#endif


// Implements Algorithm 9: xmss_node function (page 23)
void slhdsa_treehash(uint8_t out_pk[BCM_SLHDSA_SHA2_128S_N],
                     const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                     uint32_t i /*target node index*/,
                     uint32_t z /*target node height*/,
                     const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                     uint8_t addr[32]);

// Implements Algorithm 10: xmss_sign function (page 24)
void slhdsa_xmss_sign(uint8_t sig[SLHDSA_SHA2_128S_XMSS_BYTES],
                      const uint8_t msg[BCM_SLHDSA_SHA2_128S_N], unsigned int idx,
                      const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                      const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                      uint8_t addr[32]);

// Implements Algorithm 11: xmss_pkFromSig function (page 25)
void slhdsa_xmss_pk_from_sig(
    uint8_t root[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t xmss_sig[SLHDSA_SHA2_128S_XMSS_BYTES], unsigned int idx,
    const uint8_t msg[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N], uint8_t addr[32]);

// Implements Algorithm 12: ht_sign function (page 27)
void slhdsa_ht_sign(
    uint8_t sig[SLHDSA_SHA2_128S_D * SLHDSA_SHA2_128S_XMSS_BYTES],
    const uint8_t message[BCM_SLHDSA_SHA2_128S_N], uint64_t idx_tree,
    uint32_t idx_leaf, const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N]);

// Implements Algorithm 13: ht_verify function (page 28)
int slhdsa_ht_verify(
    const uint8_t sig[SLHDSA_SHA2_128S_D * SLHDSA_SHA2_128S_XMSS_BYTES],
    const uint8_t message[BCM_SLHDSA_SHA2_128S_N], uint64_t idx_tree,
    uint32_t idx_leaf, const uint8_t pk_root[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N]);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_SLHDSA_MERKLE_H
