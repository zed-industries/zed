// Copyright 2014 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_RAND_H
#define OPENSSL_HEADER_RAND_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


// Random number generation.


// RAND_bytes writes |len| bytes of random data to |buf| and returns one. In the
// event that sufficient random data can not be obtained, |abort| is called.
OPENSSL_EXPORT int RAND_bytes(uint8_t *buf, size_t len);


// Obscure functions.

#if !defined(OPENSSL_WINDOWS)
// RAND_enable_fork_unsafe_buffering indicates that clones of the address space,
// e.g. via |fork|, will never call into BoringSSL. It may be used to disable
// BoringSSL's more expensive fork-safety measures. However, calling this
// function and then using BoringSSL across |fork| calls will leak secret keys.
// |fd| must be -1.
//
// WARNING: This function affects BoringSSL for the entire address space. Thus
// this function should never be called by library code, only by code with
// global knowledge of the application's use of BoringSSL.
//
// Do not use this function unless a performance issue was measured with the
// default behavior. BoringSSL can efficiently detect forks on most platforms,
// in which case this function is a no-op and is unnecessary. In particular,
// Linux kernel versions 4.14 or later provide |MADV_WIPEONFORK|. Future
// versions of BoringSSL will remove this functionality when older kernels are
// sufficiently rare.
//
// This function has an unusual name because it historically controlled internal
// buffers, but no longer does.
OPENSSL_EXPORT void RAND_enable_fork_unsafe_buffering(int fd);

// RAND_disable_fork_unsafe_buffering restores BoringSSL's default fork-safety
// protections. See also |RAND_enable_fork_unsafe_buffering|.
OPENSSL_EXPORT void RAND_disable_fork_unsafe_buffering(void);
#endif

#if defined(FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION)
// RAND_reset_for_fuzzing resets the fuzzer-only deterministic RNG. This
// function is only defined in the fuzzer-only build configuration.
OPENSSL_EXPORT void RAND_reset_for_fuzzing(void);
#endif

// RAND_get_system_entropy_for_custom_prng writes |len| bytes of random data
// from a system entropy source to |buf|. The maximum length of entropy which
// may be requested is 256 bytes. If more than 256 bytes of data is requested,
// or if sufficient random data can not be obtained, |abort| is called.
// |RAND_bytes| should normally be used instead of this function. This function
// should only be used for seed values or where |malloc| should not be called
// from BoringSSL. This function is not FIPS compliant.
OPENSSL_EXPORT void RAND_get_system_entropy_for_custom_prng(uint8_t *buf,
                                                            size_t len);


// Deprecated functions

// RAND_pseudo_bytes is a wrapper around |RAND_bytes|.
OPENSSL_EXPORT int RAND_pseudo_bytes(uint8_t *buf, size_t len);

// RAND_seed reads a single byte of random data to ensure that any file
// descriptors etc are opened.
OPENSSL_EXPORT void RAND_seed(const void *buf, int num);

// RAND_load_file returns a nonnegative number.
OPENSSL_EXPORT int RAND_load_file(const char *path, long num);

// RAND_file_name returns NULL.
OPENSSL_EXPORT const char *RAND_file_name(char *buf, size_t num);

// RAND_add does nothing.
OPENSSL_EXPORT void RAND_add(const void *buf, int num, double entropy);

// RAND_egd returns 255.
OPENSSL_EXPORT int RAND_egd(const char *);

// RAND_poll returns one.
OPENSSL_EXPORT int RAND_poll(void);

// RAND_status returns one.
OPENSSL_EXPORT int RAND_status(void);

// RAND_cleanup does nothing.
OPENSSL_EXPORT void RAND_cleanup(void);

// rand_meth_st is typedefed to |RAND_METHOD| in base.h. It isn't used; it
// exists only to be the return type of |RAND_SSLeay|. It's
// external so that variables of this type can be initialized.
struct rand_meth_st {
  void (*seed) (const void *buf, int num);
  int (*bytes) (uint8_t *buf, size_t num);
  void (*cleanup) (void);
  void (*add) (const void *buf, int num, double entropy);
  int (*pseudorand) (uint8_t *buf, size_t num);
  int (*status) (void);
};

// RAND_SSLeay returns a pointer to a dummy |RAND_METHOD|.
OPENSSL_EXPORT RAND_METHOD *RAND_SSLeay(void);

// RAND_OpenSSL returns a pointer to a dummy |RAND_METHOD|.
OPENSSL_EXPORT RAND_METHOD *RAND_OpenSSL(void);

// RAND_get_rand_method returns |RAND_SSLeay()|.
OPENSSL_EXPORT const RAND_METHOD *RAND_get_rand_method(void);

// RAND_set_rand_method returns one.
OPENSSL_EXPORT int RAND_set_rand_method(const RAND_METHOD *);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_RAND_H
