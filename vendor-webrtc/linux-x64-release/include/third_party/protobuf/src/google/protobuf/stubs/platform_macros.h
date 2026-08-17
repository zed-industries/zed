// Protocol Buffers - Google's data interchange format
// Copyright 2012 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_PLATFORM_MACROS_H_
#define GOOGLE_PROTOBUF_PLATFORM_MACROS_H_

#define GOOGLE_PROTOBUF_PLATFORM_ERROR \
#error "Host platform was not detected as supported by protobuf"

// Processor architecture detection.  For more info on what's defined, see:
//   http://msdn.microsoft.com/en-us/library/b0084kay.aspx
//   http://www.agner.org/optimize/calling_conventions.pdf
//   or with gcc, run: "echo | gcc -E -dM -"
#if defined(_M_X64) || defined(__x86_64__)
#define GOOGLE_PROTOBUF_ARCH_X64 1
#define GOOGLE_PROTOBUF_ARCH_64_BIT 1
#elif defined(_M_IX86) || defined(__i386__)
#define GOOGLE_PROTOBUF_ARCH_IA32 1
#define GOOGLE_PROTOBUF_ARCH_32_BIT 1
#elif defined(__QNX__)
#define GOOGLE_PROTOBUF_ARCH_ARM_QNX 1
#if defined(__aarch64__)
#define GOOGLE_PROTOBUF_ARCH_64_BIT 1
#else
#define GOOGLE_PROTOBUF_ARCH_32_BIT 1
#endif
#elif defined(_M_ARM) || defined(__ARMEL__)
#define GOOGLE_PROTOBUF_ARCH_ARM 1
#define GOOGLE_PROTOBUF_ARCH_32_BIT 1
#elif defined(_M_ARM64)
#define GOOGLE_PROTOBUF_ARCH_ARM 1
#define GOOGLE_PROTOBUF_ARCH_64_BIT 1
#elif defined(__aarch64__)
#define GOOGLE_PROTOBUF_ARCH_AARCH64 1
#define GOOGLE_PROTOBUF_ARCH_64_BIT 1
#elif defined(__mips__)
#if defined(__LP64__)
#define GOOGLE_PROTOBUF_ARCH_MIPS64 1
#define GOOGLE_PROTOBUF_ARCH_64_BIT 1
#else
#define GOOGLE_PROTOBUF_ARCH_MIPS 1
#define GOOGLE_PROTOBUF_ARCH_32_BIT 1
#endif
#elif defined(__pnacl__)
#define GOOGLE_PROTOBUF_ARCH_32_BIT 1
#elif defined(sparc)
#define GOOGLE_PROTOBUF_ARCH_SPARC 1
#if defined(__sparc_v9__) || defined(__sparcv9) || defined(__arch64__)
#define GOOGLE_PROTOBUF_ARCH_64_BIT 1
#else
#define GOOGLE_PROTOBUF_ARCH_32_BIT 1
#endif
#elif defined(_POWER) || defined(__powerpc64__) || defined(__PPC64__)
#define GOOGLE_PROTOBUF_ARCH_POWER 1
#define GOOGLE_PROTOBUF_ARCH_64_BIT 1
#elif defined(__PPC__)
#define GOOGLE_PROTOBUF_ARCH_PPC 1
#define GOOGLE_PROTOBUF_ARCH_32_BIT 1
#elif defined(__GNUC__)
# if (((__GNUC__ == 4) && (__GNUC_MINOR__ >= 7)) || (__GNUC__ > 4))
// We fallback to the generic Clang/GCC >= 4.7 implementation in atomicops.h
# elif defined(__clang__)
#  if !__has_extension(c_atomic)
GOOGLE_PROTOBUF_PLATFORM_ERROR
#  endif
// We fallback to the generic Clang/GCC >= 4.7 implementation in atomicops.h
# endif
# if __LP64__
#  define GOOGLE_PROTOBUF_ARCH_64_BIT 1
# else
#  define GOOGLE_PROTOBUF_ARCH_32_BIT 1
# endif
#else
GOOGLE_PROTOBUF_PLATFORM_ERROR
#endif

#if defined(__APPLE__)
#define GOOGLE_PROTOBUF_OS_APPLE
#include <Availability.h>
#include <TargetConditionals.h>
#if TARGET_OS_IPHONE
#define GOOGLE_PROTOBUF_OS_IPHONE
#endif
#elif defined(__EMSCRIPTEN__)
#define GOOGLE_PROTOBUF_OS_EMSCRIPTEN
#elif defined(__native_client__)
#define GOOGLE_PROTOBUF_OS_NACL
#elif defined(sun)
#define GOOGLE_PROTOBUF_OS_SOLARIS
#elif defined(_AIX)
#define GOOGLE_PROTOBUF_OS_AIX
#elif defined(__ANDROID__)
#define GOOGLE_PROTOBUF_OS_ANDROID
#endif

#undef GOOGLE_PROTOBUF_PLATFORM_ERROR

#if defined(GOOGLE_PROTOBUF_OS_ANDROID) || defined(GOOGLE_PROTOBUF_OS_IPHONE)
// Android ndk does not support the __thread keyword very well yet. Here
// we use pthread_key_create()/pthread_getspecific()/... methods for
// TLS support on android.
// iOS also does not support the __thread keyword.
#define GOOGLE_PROTOBUF_NO_THREADLOCAL
#endif

#if defined(__MAC_OS_X_VERSION_MIN_REQUIRED) && __MAC_OS_X_VERSION_MIN_REQUIRED < 1070
// __thread keyword requires at least 10.7
#define GOOGLE_PROTOBUF_NO_THREADLOCAL
#endif

#endif  // GOOGLE_PROTOBUF_PLATFORM_MACROS_H_
