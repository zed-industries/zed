// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef AWSLC_HEADER_CRYPTO_RAND_EXTRA_RAND_INTERNAL_H
#define AWSLC_HEADER_CRYPTO_RAND_EXTRA_RAND_INTERNAL_H

#include <openssl/ctrdrbg.h>

#if defined(BORINGSSL_UNSAFE_DETERMINISTIC_MODE)
#define OPENSSL_RAND_DETERMINISTIC
#elif defined(OPENSSL_WINDOWS)
#define OPENSSL_RAND_WINDOWS
#elif defined(OPENSSL_MACOS) || defined(OPENSSL_OPENBSD) || \
    defined(OPENSSL_FREEBSD)  || defined(OPENSSL_NETBSD) || \
    defined(OPENSSL_SOLARIS) || defined(OPENSSL_WASM) || \
    (defined(OPENSSL_LINUX) && !defined(HAVE_LINUX_RANDOM_H))
#define OPENSSL_RAND_GETENTROPY
// OPENSSL_APPLE is always defined when OPENSSL_MACOS is defined, so this
// branch must come after the OPENSSL_MACOS branch above.
#elif defined(OPENSSL_APPLE)
#define OPENSSL_RAND_CCRANDOMGENERATEBYTES
#else
#define OPENSSL_RAND_URANDOM
#endif

#if defined(__cplusplus)
extern "C" {
#endif

// Functions:
// CRYPTO_sysrand
// CRYPTO_sysrand_if_available
// are the operating system entropy source interface used in the randomness
// generation implementation.

// CRYPTO_sysrand fills |len| bytes at |buf| with entropy from the operating
// system.
OPENSSL_EXPORT void CRYPTO_sysrand(uint8_t *buf, size_t len);

#if defined(OPENSSL_RAND_URANDOM)
// CRYPTO_sysrand_if_available fills |len| bytes at |buf| with entropy from the
// operating system, or early /dev/urandom data, and returns 1, _if_ the entropy
// pool is initialized or if getrandom() is not available. Otherwise it will not
// block and will instead fill |buf| with all zeros and return 0.
int CRYPTO_sysrand_if_available(uint8_t *buf, size_t len);
#else
OPENSSL_INLINE int CRYPTO_sysrand_if_available(uint8_t *buf, size_t len) {
  CRYPTO_sysrand(buf, len);
  return 1;
}
#endif  // defined(OPENSSL_RAND_URANDOM)

// Don't retry forever. There is no science in picking this number and can be
// adjusted in the future if need be. We do not backoff forever, because we
// believe that it is easier to detect failing calls than detecting infinite
// spinning loops.
#define MAX_BACKOFF_RETRIES 9

OPENSSL_EXPORT int vm_ube_fallback_get_seed(
    uint8_t seed[CTR_DRBG_ENTROPY_LEN]);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // AWSLC_HEADER_CRYPTO_RAND_EXTRA_RAND_INTERNAL_H
