// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_CRYPTO_RAND_GETRANDOM_FILLIN_H
#define OPENSSL_HEADER_CRYPTO_RAND_GETRANDOM_FILLIN_H

#include <openssl/base.h>
#include "internal.h"


#if defined(OPENSSL_RAND_URANDOM) && defined(OPENSSL_LINUX)
#include <sys/syscall.h>

#if defined(OPENSSL_X86_64)
#define EXPECTED_NR_getrandom 318
#elif defined(OPENSSL_X86)
#define EXPECTED_NR_getrandom 355
#elif defined(OPENSSL_AARCH64)
#define EXPECTED_NR_getrandom 278
#elif defined(OPENSSL_ARM)
#define EXPECTED_NR_getrandom 384
#elif defined(OPENSSL_PPC64LE) || defined(OPENSSL_PPC64BE) || defined(OPENSSL_PPC32BE) || defined(OPENSSL_PPC32LE)
#define EXPECTED_NR_getrandom 359
#elif defined(OPENSSL_RISCV64)
#define EXPECTED_NR_getrandom 278
#elif defined(OPENSSL_S390X)
#define EXPECTED_NR_getrandom 349
#elif defined(OPENSSL_LOONGARCH64)
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

#endif  // OPENSSL_RAND_URANDOM && OPENSSL_LINUX


#endif  // OPENSSL_HEADER_CRYPTO_RAND_GETRANDOM_FILLIN_H
