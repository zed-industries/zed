// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_CRYPTO_FORK_UBE_DETECT_H
#define OPENSSL_HEADER_CRYPTO_FORK_UBE_DETECT_H

#include <openssl/base.h>

#include <stdint.h>

#if defined(__cplusplus)
extern "C" {
#endif


// CRYPTO_get_fork_ube_generation returns the fork generation number for the
// current process, or zero if not supported on the platform. The fork
// generation number is a non-zero, strictly-monotonic counter with the property
// that, if queried in an address space and then again in a subsequently forked
// copy, the forked address space will observe a greater value.
//
// This function may be used to clear cached values across a fork. When
// initializing a cache, record the fork generation. Before using the cache,
// check if the fork generation has changed. If so, drop the cache and update
// the save fork generation. Note this logic transparently handles platforms
// which always return zero.
//
// This is not reliably supported on all platforms which implement |fork|, so it
// should only be used as a hardening measure.
OPENSSL_EXPORT uint64_t CRYPTO_get_fork_ube_generation(void);

// CRYPTO_fork_detect_ignore_wipeonfork_FOR_TESTING is an internal detail
// used for testing purposes.
OPENSSL_EXPORT void CRYPTO_fork_detect_ignore_wipeonfork_FOR_TESTING(void);

// CRYPTO_fork_detect_ignore_inheritzero_FOR_TESTING is an internal detail
// used for testing purposes.
OPENSSL_EXPORT void CRYPTO_fork_detect_ignore_inheritzero_FOR_TESTING(void);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FORK_UBE_DETECT_H
