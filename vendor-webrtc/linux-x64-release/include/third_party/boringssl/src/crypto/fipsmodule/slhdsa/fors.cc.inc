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

#include "../../internal.h"
#include "./address.h"
#include "./fors.h"
#include "./params.h"
#include "./thash.h"

// Compute the base 2^12 representation of `message` (algorithm 4, page 16).
static void fors_base_b(
    uint16_t indices[SLHDSA_SHA2_128S_FORS_TREES],
    const uint8_t message[SLHDSA_SHA2_128S_FORS_MSG_BYTES]) {
  static_assert(SLHDSA_SHA2_128S_FORS_HEIGHT == 12, "");
  static_assert((SLHDSA_SHA2_128S_FORS_TREES & 1) == 0, "");

  const uint8_t *msg = message;
  for (size_t i = 0; i < SLHDSA_SHA2_128S_FORS_TREES; i += 2) {
    uint32_t val = ((uint32_t)msg[0] << 16) | ((uint32_t)msg[1] << 8) | msg[2];
    indices[i] = (val >> 12) & 0xFFF;
    indices[i + 1] = val & 0xFFF;
    msg += 3;
  }
}

// Implements Algorithm 14: fors_skGen function (page 29)
void slhdsa_fors_sk_gen(uint8_t fors_sk[BCM_SLHDSA_SHA2_128S_N], uint32_t idx,
                        const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                        const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                        uint8_t addr[32]) {
  uint8_t sk_addr[32];
  OPENSSL_memcpy(sk_addr, addr, sizeof(sk_addr));

  slhdsa_set_type(sk_addr, SLHDSA_SHA2_128S_ADDR_TYPE_FORSPRF);
  slhdsa_copy_keypair_addr(sk_addr, addr);
  slhdsa_set_tree_index(sk_addr, idx);
  slhdsa_thash_prf(fors_sk, pk_seed, sk_seed, sk_addr);
}

// Implements Algorithm 15: fors_node function (page 30)
void slhdsa_fors_treehash(uint8_t root_node[BCM_SLHDSA_SHA2_128S_N],
                          const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                          uint32_t i /*target node index*/,
                          uint32_t z /*target node height*/,
                          const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                          uint8_t addr[32]) {
  BSSL_CHECK(z <= SLHDSA_SHA2_128S_FORS_HEIGHT);
  BSSL_CHECK(i < (uint32_t)(SLHDSA_SHA2_128S_FORS_TREES *
                            (1 << (SLHDSA_SHA2_128S_FORS_HEIGHT - z))));

  if (z == 0) {
    uint8_t sk[BCM_SLHDSA_SHA2_128S_N];
    slhdsa_set_tree_height(addr, 0);
    slhdsa_set_tree_index(addr, i);
    slhdsa_fors_sk_gen(sk, i, sk_seed, pk_seed, addr);
    slhdsa_thash_f(root_node, sk, pk_seed, addr);
  } else {
    // Stores left node and right node.
    uint8_t nodes[2 * BCM_SLHDSA_SHA2_128S_N];
    slhdsa_fors_treehash(nodes, sk_seed, 2 * i, z - 1, pk_seed, addr);
    slhdsa_fors_treehash(nodes + BCM_SLHDSA_SHA2_128S_N, sk_seed, 2 * i + 1, z - 1,
                         pk_seed, addr);
    slhdsa_set_tree_height(addr, z);
    slhdsa_set_tree_index(addr, i);
    slhdsa_thash_h(root_node, nodes, pk_seed, addr);
  }
}

// Implements Algorithm 16: fors_sign function (page 31)
void slhdsa_fors_sign(uint8_t fors_sig[SLHDSA_SHA2_128S_FORS_BYTES],
                      const uint8_t message[SLHDSA_SHA2_128S_FORS_MSG_BYTES],
                      const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                      const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                      uint8_t addr[32]) {
  uint16_t indices[SLHDSA_SHA2_128S_FORS_TREES];

  // Derive FORS indices compatible with the NIST changes.
  fors_base_b(indices, message);

  for (size_t i = 0; i < SLHDSA_SHA2_128S_FORS_TREES; ++i) {
    slhdsa_set_tree_height(addr, 0);
    // Write the FORS secret key element to the correct position.
    slhdsa_fors_sk_gen(
        fors_sig + i * BCM_SLHDSA_SHA2_128S_N * (SLHDSA_SHA2_128S_FORS_HEIGHT + 1),
        i * (1 << SLHDSA_SHA2_128S_FORS_HEIGHT) + indices[i], sk_seed, pk_seed,
        addr);
    for (size_t j = 0; j < SLHDSA_SHA2_128S_FORS_HEIGHT; ++j) {
      size_t s = (indices[i] / (1 << j)) ^ 1;
      // Write the FORS auth path element to the correct position.
      slhdsa_fors_treehash(
          fors_sig + BCM_SLHDSA_SHA2_128S_N *
                         (i * (SLHDSA_SHA2_128S_FORS_HEIGHT + 1) + j + 1),
          sk_seed, i * (1ULL << (SLHDSA_SHA2_128S_FORS_HEIGHT - j)) + s, j,
          pk_seed, addr);
    }
  }
}

// Implements Algorithm 17: fors_pkFromSig function (page 32)
void slhdsa_fors_pk_from_sig(
    uint8_t fors_pk[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t fors_sig[SLHDSA_SHA2_128S_FORS_BYTES],
    const uint8_t message[SLHDSA_SHA2_128S_FORS_MSG_BYTES],
    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N], uint8_t addr[32]) {
  uint16_t indices[SLHDSA_SHA2_128S_FORS_TREES];
  uint8_t tmp[2 * BCM_SLHDSA_SHA2_128S_N];
  uint8_t roots[SLHDSA_SHA2_128S_FORS_TREES * BCM_SLHDSA_SHA2_128S_N];

  // Derive FORS indices compatible with the NIST changes.
  fors_base_b(indices, message);

  for (size_t i = 0; i < SLHDSA_SHA2_128S_FORS_TREES; ++i) {
    // Pointer to current sk and authentication path
    const uint8_t *sk =
        fors_sig + i * BCM_SLHDSA_SHA2_128S_N * (SLHDSA_SHA2_128S_FORS_HEIGHT + 1);
    const uint8_t *auth =
        fors_sig + i * BCM_SLHDSA_SHA2_128S_N * (SLHDSA_SHA2_128S_FORS_HEIGHT + 1) +
        BCM_SLHDSA_SHA2_128S_N;
    uint8_t nodes[2 * BCM_SLHDSA_SHA2_128S_N];

    slhdsa_set_tree_height(addr, 0);
    slhdsa_set_tree_index(
        addr, (i * (1 << SLHDSA_SHA2_128S_FORS_HEIGHT)) + indices[i]);

    slhdsa_thash_f(nodes, sk, pk_seed, addr);

    for (size_t j = 0; j < SLHDSA_SHA2_128S_FORS_HEIGHT; ++j) {
      slhdsa_set_tree_height(addr, j + 1);

      // Even node
      if (((indices[i] / (1 << j)) % 2) == 0) {
        slhdsa_set_tree_index(addr, slhdsa_get_tree_index(addr) / 2);
        OPENSSL_memcpy(tmp, nodes, BCM_SLHDSA_SHA2_128S_N);
        OPENSSL_memcpy(tmp + BCM_SLHDSA_SHA2_128S_N, auth + j * BCM_SLHDSA_SHA2_128S_N,
                       BCM_SLHDSA_SHA2_128S_N);
        slhdsa_thash_h(nodes + BCM_SLHDSA_SHA2_128S_N, tmp, pk_seed, addr);
      } else {
        slhdsa_set_tree_index(addr, (slhdsa_get_tree_index(addr) - 1) / 2);
        OPENSSL_memcpy(tmp, auth + j * BCM_SLHDSA_SHA2_128S_N, BCM_SLHDSA_SHA2_128S_N);
        OPENSSL_memcpy(tmp + BCM_SLHDSA_SHA2_128S_N, nodes, BCM_SLHDSA_SHA2_128S_N);
        slhdsa_thash_h(nodes + BCM_SLHDSA_SHA2_128S_N, tmp, pk_seed, addr);
      }
      OPENSSL_memcpy(nodes, nodes + BCM_SLHDSA_SHA2_128S_N, BCM_SLHDSA_SHA2_128S_N);
    }
    OPENSSL_memcpy(roots + i * BCM_SLHDSA_SHA2_128S_N, nodes, BCM_SLHDSA_SHA2_128S_N);
  }

  uint8_t forspk_addr[32];
  OPENSSL_memcpy(forspk_addr, addr, sizeof(forspk_addr));
  slhdsa_set_type(forspk_addr, SLHDSA_SHA2_128S_ADDR_TYPE_FORSPK);
  slhdsa_copy_keypair_addr(forspk_addr, addr);
  slhdsa_thash_tk(fors_pk, roots, pk_seed, forspk_addr);
}
