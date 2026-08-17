// Copyright 2020 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_RAND_GETRANDOM_FILLIN_H
#define OPENSSL_HEADER_CRYPTO_RAND_GETRANDOM_FILLIN_H

#include <openssl/base.h>


#if defined(OPENSSL_LINUX)

#include <sys/syscall.h>

#if defined(OPENSSL_X86_64)
#define EXPECTED_NR_getrandom 318
#elif defined(OPENSSL_X86)
#define EXPECTED_NR_getrandom 355
#elif defined(OPENSSL_AARCH64)
#define EXPECTED_NR_getrandom 278
#elif defined(OPENSSL_ARM)
#define EXPECTED_NR_getrandom 384
#elif defined(OPENSSL_RISCV64)
#define EXPECTED_NR_getrandom 278
#endif

#if defined(EXPECTED_NR_getrandom)
#define USE_NR_getrandom

#if defined(__NR_getrandom)

#if __NR_getrandom != EXPECTED_NR_getrandom
#error "system call number for getrandom is not the expected value"
#endif

#else  // __NR_getrandom

#define __NR_getrandom EXPECTED_NR_getrandom

#endif  // __NR_getrandom

#endif  // EXPECTED_NR_getrandom

#if !defined(GRND_NONBLOCK)
#define GRND_NONBLOCK 1
#endif
#if !defined(GRND_RANDOM)
#define GRND_RANDOM 2
#endif

#endif  // OPENSSL_LINUX


#endif  // OPENSSL_HEADER_CRYPTO_RAND_GETRANDOM_FILLIN_H
