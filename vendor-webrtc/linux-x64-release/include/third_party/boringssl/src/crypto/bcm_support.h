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

#ifndef OPENSSL_HEADER_CRYPTO_BCM_SUPPORT_H
#define OPENSSL_HEADER_CRYPTO_BCM_SUPPORT_H

#include <openssl/base.h>

#include <stdio.h>

// Provided by libcrypto, called from BCM

#if defined(__cplusplus)
extern "C" {
#endif

// Provided by libcrypto, called from BCM

// CRYPTO_init_sysrand initializes long-lived resources needed to draw entropy
// from the operating system, if the operating system requires initialization.
void CRYPTO_init_sysrand(void);

// CRYPTO_sysrand fills |len| bytes at |buf| with entropy from the operating
// system.
void CRYPTO_sysrand(uint8_t *buf, size_t len);

// CRYPTO_sysrand_if_available fills |len| bytes at |buf| with entropy from the
// operating system, or early /dev/urandom data, and returns 1, _if_ the entropy
// pool is initialized or if getrandom() is not available and not in FIPS mode.
// Otherwise it will not block and will instead fill |buf| with all zeros and
// return 0.
int CRYPTO_sysrand_if_available(uint8_t *buf, size_t len);

// CRYPTO_sysrand_for_seed fills |len| bytes at |buf| with entropy from the
// operating system. It may draw from the |GRND_RANDOM| pool on Android,
// depending on the vendor's configuration.
void CRYPTO_sysrand_for_seed(uint8_t *buf, size_t len);

// RAND_need_entropy is called whenever the BCM module has stopped because it
// has run out of entropy.
void RAND_need_entropy(size_t bytes_needed);

// crypto_get_fork_generation returns the fork generation number for the current
// process, or zero if not supported on the platform. The fork generation number
// is a non-zero, strictly-monotonic counter with the property that, if queried
// in an address space and then again in a subsequently forked copy, the forked
// address space will observe a greater value.
//
// This function may be used to clear cached values across a fork. When
// initializing a cache, record the fork generation. Before using the cache,
// check if the fork generation has changed. If so, drop the cache and update
// the save fork generation. Note this logic transparently handles platforms
// which always return zero.
//
// This is not reliably supported on all platforms which implement |fork|, so it
// should only be used as a hardening measure.
OPENSSL_EXPORT uint64_t CRYPTO_get_fork_generation(void);

// CRYPTO_fork_detect_force_madv_wipeonfork_for_testing is an internal detail
// used for testing purposes.
OPENSSL_EXPORT void CRYPTO_fork_detect_force_madv_wipeonfork_for_testing(
    int on);

// CRYPTO_get_stderr returns stderr. This function exists to avoid BCM needing
// a data dependency on libc.
FILE *CRYPTO_get_stderr(void);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_BCM_SUPPORT_H
