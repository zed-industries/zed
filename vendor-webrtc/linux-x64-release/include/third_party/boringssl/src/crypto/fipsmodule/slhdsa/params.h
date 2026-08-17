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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_SLHDSA_PARAMS_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_SLHDSA_PARAMS_H

#include <openssl/base.h>
#include "../bcm_interface.h"

#if defined(__cplusplus)
extern "C" {
#endif

// Total height of the tree structure.
#define SLHDSA_SHA2_128S_FULL_HEIGHT 63
// Number of subtree layers.
#define SLHDSA_SHA2_128S_D 7
// Height of the trees on each layer
#define SLHDSA_SHA2_128S_TREE_HEIGHT 9
// Height of each individual FORS tree.
#define SLHDSA_SHA2_128S_FORS_HEIGHT 12
// Total number of FORS tree used.
#define SLHDSA_SHA2_128S_FORS_TREES 14
// Size of a FORS signature
#define SLHDSA_SHA2_128S_FORS_BYTES                                   \
  ((SLHDSA_SHA2_128S_FORS_HEIGHT + 1) * SLHDSA_SHA2_128S_FORS_TREES * \
   BCM_SLHDSA_SHA2_128S_N)

// Winternitz parameter and derived values
#define SLHDSA_SHA2_128S_WOTS_W 16
#define SLHDSA_SHA2_128S_WOTS_LOG_W 4
#define SLHDSA_SHA2_128S_WOTS_LEN1 32
#define SLHDSA_SHA2_128S_WOTS_LEN2 3
#define SLHDSA_SHA2_128S_WOTS_LEN 35
#define SLHDSA_SHA2_128S_WOTS_BYTES \
  (BCM_SLHDSA_SHA2_128S_N * SLHDSA_SHA2_128S_WOTS_LEN)

// XMSS sizes
#define SLHDSA_SHA2_128S_XMSS_BYTES \
  (SLHDSA_SHA2_128S_WOTS_BYTES +    \
   (BCM_SLHDSA_SHA2_128S_N * SLHDSA_SHA2_128S_TREE_HEIGHT))

// Size of the message digest (NOTE: This is only correct for the SHA-256 params
// here)
#define SLHDSA_SHA2_128S_DIGEST_SIZE                                           \
  (((SLHDSA_SHA2_128S_FORS_TREES * SLHDSA_SHA2_128S_FORS_HEIGHT) / 8) +        \
   (((SLHDSA_SHA2_128S_FULL_HEIGHT - SLHDSA_SHA2_128S_TREE_HEIGHT) / 8) + 1) + \
   (SLHDSA_SHA2_128S_TREE_HEIGHT / 8) + 1)

// Compressed address size when using SHA-256
#define SLHDSA_SHA2_128S_SHA256_ADDR_BYTES 22

// Size of the FORS message hash
#define SLHDSA_SHA2_128S_FORS_MSG_BYTES \
  ((SLHDSA_SHA2_128S_FORS_HEIGHT * SLHDSA_SHA2_128S_FORS_TREES + 7) / 8)
#define SLHDSA_SHA2_128S_TREE_BITS \
  (SLHDSA_SHA2_128S_TREE_HEIGHT * (SLHDSA_SHA2_128S_D - 1))
#define SLHDSA_SHA2_128S_TREE_BYTES ((SLHDSA_SHA2_128S_TREE_BITS + 7) / 8)
#define SLHDSA_SHA2_128S_LEAF_BITS SLHDSA_SHA2_128S_TREE_HEIGHT
#define SLHDSA_SHA2_128S_LEAF_BYTES ((SLHDSA_SHA2_128S_LEAF_BITS + 7) / 8)


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_SLHDSA_PARAMS_H
