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

#include <string.h>

#include "../../internal.h"
#include "./address.h"
#include "./merkle.h"
#include "./params.h"
#include "./thash.h"
#include "./wots.h"


// Implements Algorithm 9: xmss_node function (page 23)
void slhdsa_treehash(uint8_t out_pk[BCM_SLHDSA_SHA2_128S_N],
                     const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                     uint32_t i /*target node index*/,
                     uint32_t z /*target node height*/,
                     const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                     uint8_t addr[32]) {
  BSSL_CHECK(z <= SLHDSA_SHA2_128S_TREE_HEIGHT);
  BSSL_CHECK(i < (uint32_t)(1 << (SLHDSA_SHA2_128S_TREE_HEIGHT - z)));

  if (z == 0) {
    slhdsa_set_type(addr, SLHDSA_SHA2_128S_ADDR_TYPE_WOTS);
    slhdsa_set_keypair_addr(addr, i);
    slhdsa_wots_pk_gen(out_pk, sk_seed, pk_seed, addr);
  } else {
    // Stores left node and right node.
    uint8_t nodes[2 * BCM_SLHDSA_SHA2_128S_N];
    slhdsa_treehash(nodes, sk_seed, 2 * i, z - 1, pk_seed, addr);
    slhdsa_treehash(nodes + BCM_SLHDSA_SHA2_128S_N, sk_seed, 2 * i + 1, z - 1,
                    pk_seed, addr);
    slhdsa_set_type(addr, SLHDSA_SHA2_128S_ADDR_TYPE_HASHTREE);
    slhdsa_set_tree_height(addr, z);
    slhdsa_set_tree_index(addr, i);
    slhdsa_thash_h(out_pk, nodes, pk_seed, addr);
  }
}

// Implements Algorithm 10: xmss_sign function (page 24)
void slhdsa_xmss_sign(uint8_t sig[SLHDSA_SHA2_128S_XMSS_BYTES],
                      const uint8_t msg[BCM_SLHDSA_SHA2_128S_N], unsigned int idx,
                      const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                      const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N],
                      uint8_t addr[32]) {
  // Build authentication path
  for (size_t j = 0; j < SLHDSA_SHA2_128S_TREE_HEIGHT; ++j) {
    unsigned int k = (idx >> j) ^ 1;
    slhdsa_treehash(sig + SLHDSA_SHA2_128S_WOTS_BYTES + j * BCM_SLHDSA_SHA2_128S_N,
                    sk_seed, k, j, pk_seed, addr);
  }

  // Compute WOTS+ signature
  slhdsa_set_type(addr, SLHDSA_SHA2_128S_ADDR_TYPE_WOTS);
  slhdsa_set_keypair_addr(addr, idx);
  slhdsa_wots_sign(sig, msg, sk_seed, pk_seed, addr);
}

// Implements Algorithm 11: xmss_pkFromSig function (page 25)
void slhdsa_xmss_pk_from_sig(
    uint8_t root[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t xmss_sig[SLHDSA_SHA2_128S_XMSS_BYTES], unsigned int idx,
    const uint8_t msg[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N], uint8_t addr[32]) {
  // Stores node[0] and node[1] from Algorithm 11
  slhdsa_set_type(addr, SLHDSA_SHA2_128S_ADDR_TYPE_WOTS);
  slhdsa_set_keypair_addr(addr, idx);
  uint8_t node[2 * BCM_SLHDSA_SHA2_128S_N];
  slhdsa_wots_pk_from_sig(node, xmss_sig, msg, pk_seed, addr);

  slhdsa_set_type(addr, SLHDSA_SHA2_128S_ADDR_TYPE_HASHTREE);
  slhdsa_set_tree_index(addr, idx);

  uint8_t tmp[2 * BCM_SLHDSA_SHA2_128S_N];
  const uint8_t *const auth = xmss_sig + SLHDSA_SHA2_128S_WOTS_BYTES;
  for (size_t k = 0; k < SLHDSA_SHA2_128S_TREE_HEIGHT; ++k) {
    slhdsa_set_tree_height(addr, k + 1);
    if (((idx >> k) & 1) == 0) {
      slhdsa_set_tree_index(addr, slhdsa_get_tree_index(addr) >> 1);
      OPENSSL_memcpy(tmp, node, BCM_SLHDSA_SHA2_128S_N);
      OPENSSL_memcpy(tmp + BCM_SLHDSA_SHA2_128S_N, auth + k * BCM_SLHDSA_SHA2_128S_N,
                     BCM_SLHDSA_SHA2_128S_N);
      slhdsa_thash_h(node + BCM_SLHDSA_SHA2_128S_N, tmp, pk_seed, addr);
    } else {
      slhdsa_set_tree_index(addr, (slhdsa_get_tree_index(addr) - 1) >> 1);
      OPENSSL_memcpy(tmp, auth + k * BCM_SLHDSA_SHA2_128S_N, BCM_SLHDSA_SHA2_128S_N);
      OPENSSL_memcpy(tmp + BCM_SLHDSA_SHA2_128S_N, node, BCM_SLHDSA_SHA2_128S_N);
      slhdsa_thash_h(node + BCM_SLHDSA_SHA2_128S_N, tmp, pk_seed, addr);
    }
    OPENSSL_memcpy(node, node + BCM_SLHDSA_SHA2_128S_N, BCM_SLHDSA_SHA2_128S_N);
  }
  OPENSSL_memcpy(root, node, BCM_SLHDSA_SHA2_128S_N);
}

// Implements Algorithm 12: ht_sign function (page 27)
void slhdsa_ht_sign(
    uint8_t sig[SLHDSA_SHA2_128S_XMSS_BYTES * SLHDSA_SHA2_128S_D],
    const uint8_t message[BCM_SLHDSA_SHA2_128S_N], uint64_t idx_tree,
    uint32_t idx_leaf, const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N]) {
  uint8_t addr[32] = {0};
  slhdsa_set_tree_addr(addr, idx_tree);

  // Layer 0
  slhdsa_xmss_sign(sig, message, idx_leaf, sk_seed, pk_seed, addr);
  uint8_t root[BCM_SLHDSA_SHA2_128S_N];
  slhdsa_xmss_pk_from_sig(root, sig, idx_leaf, message, pk_seed, addr);
  sig += SLHDSA_SHA2_128S_XMSS_BYTES;

  // All other layers
  for (size_t j = 1; j < SLHDSA_SHA2_128S_D; ++j) {
    idx_leaf = idx_tree % (1 << SLHDSA_SHA2_128S_TREE_HEIGHT);
    idx_tree = idx_tree >> SLHDSA_SHA2_128S_TREE_HEIGHT;
    slhdsa_set_layer_addr(addr, j);
    slhdsa_set_tree_addr(addr, idx_tree);
    slhdsa_xmss_sign(sig, root, idx_leaf, sk_seed, pk_seed, addr);
    if (j < (SLHDSA_SHA2_128S_D - 1)) {
      slhdsa_xmss_pk_from_sig(root, sig, idx_leaf, root, pk_seed, addr);
    }

    sig += SLHDSA_SHA2_128S_XMSS_BYTES;
  }
}

// Implements Algorithm 13: ht_verify function (page 28)
int slhdsa_ht_verify(
    const uint8_t sig[SLHDSA_SHA2_128S_D * SLHDSA_SHA2_128S_XMSS_BYTES],
    const uint8_t message[BCM_SLHDSA_SHA2_128S_N], uint64_t idx_tree,
    uint32_t idx_leaf, const uint8_t pk_root[BCM_SLHDSA_SHA2_128S_N],
    const uint8_t pk_seed[BCM_SLHDSA_SHA2_128S_N]) {
  uint8_t addr[32] = {0};
  slhdsa_set_tree_addr(addr, idx_tree);

  uint8_t node[BCM_SLHDSA_SHA2_128S_N];
  slhdsa_xmss_pk_from_sig(node, sig, idx_leaf, message, pk_seed, addr);

  for (size_t j = 1; j < SLHDSA_SHA2_128S_D; ++j) {
    idx_leaf = idx_tree % (1 << SLHDSA_SHA2_128S_TREE_HEIGHT);
    idx_tree = idx_tree >> SLHDSA_SHA2_128S_TREE_HEIGHT;
    slhdsa_set_layer_addr(addr, j);
    slhdsa_set_tree_addr(addr, idx_tree);

    slhdsa_xmss_pk_from_sig(node, sig + j * SLHDSA_SHA2_128S_XMSS_BYTES,
                            idx_leaf, node, pk_seed, addr);
  }
  return memcmp(node, pk_root, BCM_SLHDSA_SHA2_128S_N) == 0;
}
