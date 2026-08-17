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
#include <stdint.h>
#include <string.h>

#include "../../internal.h"
#include "./address.h"
#include "./params.h"
#include "./thash.h"
#include "./wots.h"


// Implements Algorithm 5: chain function, page 18
static void chain(uint8_t output[BCM_SLHDSA_SHA2_128S_N],
                  const uint8_t input[BCM_SLHDSA_SHA2_128S_N], uint32_t start,
                  uint32_t steps, const uint8_t pub_seed[BCM_SLHDSA_SHA2_128S_N],
                  uint8_t addr[32]) {
  assert(start < SLHDSA_SHA2_128S_WOTS_W);
  assert(steps < SLHDSA_SHA2_128S_WOTS_W);

  OPENSSL_memcpy(output, input, BCM_SLHDSA_SHA2_128S_N);

  for (size_t i = start; i < (start + steps) && i < SLHDSA_SHA2_128S_WOTS_W;
       ++i) {
    slhdsa_set_hash_addr(addr, i);
    slhdsa_thash_f(output, output, pub_seed, addr);
  }
}

static void slhdsa_wots_do_chain(uint8_t out[BCM_SLHDSA_SHA2_128S_N],
                                 uint8_t sk_addr[32], uint8_t addr[32],
                                 uint8_t value,
                                 const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                                 const uint8_t pub_seed[BCM_SLHDSA_SHA2_128S_N],
                                 uint32_t chain_index) {
  uint8_t tmp_sk[BCM_SLHDSA_SHA2_128S_N];
  slhdsa_set_chain_addr(sk_addr, chain_index);
  slhdsa_thash_prf(tmp_sk, pub_seed, sk_seed, sk_addr);
  slhdsa_set_chain_addr(addr, chain_index);
  chain(out, tmp_sk, 0, value, pub_seed, addr);
}

// Implements Algorithm 6: wots_pkGen function, page 18
void slhdsa_wots_pk_gen(uint8_t pk[BCM_SLHDSA_SHA2_128S_N],
                        const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                        const uint8_t pub_seed[BCM_SLHDSA_SHA2_128S_N],
                        uint8_t addr[32]) {
  uint8_t wots_pk_addr[32], sk_addr[32];
  OPENSSL_memcpy(wots_pk_addr, addr, sizeof(wots_pk_addr));
  OPENSSL_memcpy(sk_addr, addr, sizeof(sk_addr));
  slhdsa_set_type(sk_addr, SLHDSA_SHA2_128S_ADDR_TYPE_WOTSPRF);
  slhdsa_copy_keypair_addr(sk_addr, addr);

  uint8_t tmp[SLHDSA_SHA2_128S_WOTS_BYTES];
  for (size_t i = 0; i < SLHDSA_SHA2_128S_WOTS_LEN; ++i) {
    slhdsa_wots_do_chain(tmp + i * BCM_SLHDSA_SHA2_128S_N, sk_addr, addr,
                         SLHDSA_SHA2_128S_WOTS_W - 1, sk_seed, pub_seed, i);
  }

  // Compress pk
  slhdsa_set_type(wots_pk_addr, SLHDSA_SHA2_128S_ADDR_TYPE_WOTSPK);
  slhdsa_copy_keypair_addr(wots_pk_addr, addr);
  slhdsa_thash_tl(pk, tmp, pub_seed, wots_pk_addr);
}

// Implements Algorithm 7: wots_sign function, page 20
void slhdsa_wots_sign(uint8_t sig[SLHDSA_SHA2_128S_WOTS_BYTES],
                      const uint8_t msg[BCM_SLHDSA_SHA2_128S_N],
                      const uint8_t sk_seed[BCM_SLHDSA_SHA2_128S_N],
                      const uint8_t pub_seed[BCM_SLHDSA_SHA2_128S_N],
                      uint8_t addr[32]) {
  // Compute checksum
  static_assert(SLHDSA_SHA2_128S_WOTS_LEN1 == BCM_SLHDSA_SHA2_128S_N * 2, "");
  uint16_t csum = 0;
  for (size_t i = 0; i < BCM_SLHDSA_SHA2_128S_N; ++i) {
    csum += SLHDSA_SHA2_128S_WOTS_W - 1 - (msg[i] >> 4);
    csum += SLHDSA_SHA2_128S_WOTS_W - 1 - (msg[i] & 15);
  }

  // Compute chains
  uint8_t sk_addr[32];
  OPENSSL_memcpy(sk_addr, addr, sizeof(sk_addr));
  slhdsa_set_type(sk_addr, SLHDSA_SHA2_128S_ADDR_TYPE_WOTSPRF);
  slhdsa_copy_keypair_addr(sk_addr, addr);

  uint32_t chain_index = 0;
  for (size_t i = 0; i < BCM_SLHDSA_SHA2_128S_N; ++i) {
    slhdsa_wots_do_chain(sig, sk_addr, addr, msg[i] >> 4, sk_seed, pub_seed,
                         chain_index++);
    sig += BCM_SLHDSA_SHA2_128S_N;

    slhdsa_wots_do_chain(sig, sk_addr, addr, msg[i] & 15, sk_seed, pub_seed,
                         chain_index++);
    sig += BCM_SLHDSA_SHA2_128S_N;
  }

  // Include the SLHDSA_SHA2_128S_WOTS_LEN2 checksum values.
  slhdsa_wots_do_chain(sig, sk_addr, addr, (csum >> 8) & 15, sk_seed, pub_seed,
                       chain_index++);
  sig += BCM_SLHDSA_SHA2_128S_N;
  slhdsa_wots_do_chain(sig, sk_addr, addr, (csum >> 4) & 15, sk_seed, pub_seed,
                       chain_index++);
  sig += BCM_SLHDSA_SHA2_128S_N;
  slhdsa_wots_do_chain(sig, sk_addr, addr, csum & 15, sk_seed, pub_seed,
                       chain_index++);
}

static void slhdsa_wots_pk_from_sig_do_chain(
    uint8_t out[SLHDSA_SHA2_128S_WOTS_BYTES], uint8_t addr[32],
    const uint8_t in[SLHDSA_SHA2_128S_WOTS_BYTES], uint8_t value,
    const uint8_t pub_seed[BCM_SLHDSA_SHA2_128S_N], uint32_t chain_index) {
  slhdsa_set_chain_addr(addr, chain_index);
  chain(out + chain_index * BCM_SLHDSA_SHA2_128S_N,
        in + chain_index * BCM_SLHDSA_SHA2_128S_N, value,
        SLHDSA_SHA2_128S_WOTS_W - 1 - value, pub_seed, addr);
}

// Implements Algorithm 8: wots_pkFromSig function, page 21
void slhdsa_wots_pk_from_sig(uint8_t pk[BCM_SLHDSA_SHA2_128S_N],
                             const uint8_t sig[SLHDSA_SHA2_128S_WOTS_BYTES],
                             const uint8_t msg[BCM_SLHDSA_SHA2_128S_N],
                             const uint8_t pub_seed[BCM_SLHDSA_SHA2_128S_N],
                             uint8_t addr[32]) {
  // Compute checksum
  static_assert(SLHDSA_SHA2_128S_WOTS_LEN1 == BCM_SLHDSA_SHA2_128S_N * 2, "");
  uint16_t csum = 0;
  for (size_t i = 0; i < BCM_SLHDSA_SHA2_128S_N; ++i) {
    csum += SLHDSA_SHA2_128S_WOTS_W - 1 - (msg[i] >> 4);
    csum += SLHDSA_SHA2_128S_WOTS_W - 1 - (msg[i] & 15);
  }

  uint8_t tmp[SLHDSA_SHA2_128S_WOTS_BYTES];
  uint8_t wots_pk_addr[32];
  OPENSSL_memcpy(wots_pk_addr, addr, sizeof(wots_pk_addr));

  uint32_t chain_index = 0;
  static_assert(SLHDSA_SHA2_128S_WOTS_LEN1 == BCM_SLHDSA_SHA2_128S_N * 2, "");
  for (size_t i = 0; i < BCM_SLHDSA_SHA2_128S_N; ++i) {
    slhdsa_wots_pk_from_sig_do_chain(tmp, addr, sig, msg[i] >> 4, pub_seed,
                                     chain_index++);
    slhdsa_wots_pk_from_sig_do_chain(tmp, addr, sig, msg[i] & 15, pub_seed,
                                     chain_index++);
  }

  slhdsa_wots_pk_from_sig_do_chain(tmp, addr, sig, csum >> 8, pub_seed,
                                   chain_index++);
  slhdsa_wots_pk_from_sig_do_chain(tmp, addr, sig, (csum >> 4) & 15, pub_seed,
                                   chain_index++);
  slhdsa_wots_pk_from_sig_do_chain(tmp, addr, sig, csum & 15, pub_seed,
                                   chain_index++);

  // Compress pk
  slhdsa_set_type(wots_pk_addr, SLHDSA_SHA2_128S_ADDR_TYPE_WOTSPK);
  slhdsa_copy_keypair_addr(wots_pk_addr, addr);
  slhdsa_thash_tl(pk, tmp, pub_seed, wots_pk_addr);
}
