/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef GRPC_SUPPORT_PORT_PLATFORM_H
#define GRPC_SUPPORT_PORT_PLATFORM_H

// [[deprecated]] attribute is only available since C++14
#if __cplusplus >= 201402L
#define GRPC_DEPRECATED(reason) [[deprecated(reason)]]
#else
#define GRPC_DEPRECATED(reason)
#endif  // __cplusplus >= 201402L

/*
 * Defines GPR_ABSEIL_SYNC to use synchronization features from Abseil
 */
#ifndef GPR_ABSEIL_SYNC
#if defined(__APPLE__)
// This is disabled on Apple platforms because macos/grpc_basictests_c_cpp
// fails with this. https://github.com/grpc/grpc/issues/23661
#else
#define GPR_ABSEIL_SYNC 1
#endif
#endif  // GPR_ABSEIL_SYNC

/* Get windows.h included everywhere (we need it) */
#if defined(_WIN64) || defined(WIN64) || defined(_WIN32) || defined(WIN32)
#ifndef WIN32_LEAN_AND_MEAN
#define GRPC_WIN32_LEAN_AND_MEAN_WAS_NOT_DEFINED
#define WIN32_LEAN_AND_MEAN
#endif /* WIN32_LEAN_AND_MEAN */

// GPRC_DLL
// inspired by
// https://github.com/abseil/abseil-cpp/blob/20220623.1/absl/base/config.h#L730-L747
//
// When building gRPC as a DLL, this macro expands to `__declspec(dllexport)`
// so we can annotate symbols appropriately as being exported. When used in
// headers consuming a DLL, this macro expands to `__declspec(dllimport)` so
// that consumers know the symbol is defined inside the DLL. In all other cases,
// the macro expands to nothing.
//
// Warning: shared library support for Windows (i.e. producing DLL plus import
//   library instead of a static library) is experimental. Some symbols that can
//   be linked using the static library may not be available when using the
//   dynamically linked library.
//
// Note: GRPC_DLL_EXPORTS is set in CMakeLists.txt when building shared
// grpc{,_unsecure}
//       GRPC_DLL_IMPORTS is set by us as part of the interface for consumers of
//       the DLL
#if !defined(GRPC_DLL)
#if defined(GRPC_DLL_EXPORTS)
#define GRPC_DLL __declspec(dllexport)
#elif defined(GRPC_DLL_IMPORTS)
#define GRPC_DLL __declspec(dllimport)
#else
#define GRPC_DLL
#endif  // defined(GRPC_DLL_EXPORTS)
#endif

// same for gRPC++
#if !defined(GRPCXX_DLL)
#if defined(GRPCXX_DLL_EXPORTS)
#define GRPCXX_DLL __declspec(dllexport)
#elif defined(GRPCXX_DLL_IMPORTS)
#define GRPCXX_DLL __declspec(dllimport)
#else
#define GRPCXX_DLL
#endif  // defined(GRPCXX_DLL_EXPORTS)
#endif

// same for GPR
#if !defined(GPR_DLL)
#if defined(GPR_DLL_EXPORTS)
#define GPR_DLL __declspec(dllexport)
#elif defined(GPR_DLL_IMPORTS)
#define GPR_DLL __declspec(dllimport)
#else
#define GPR_DLL
#endif  // defined(GPR_DLL_EXPORTS)
#endif

#ifndef NOMINMAX
#define GRPC_NOMINMX_WAS_NOT_DEFINED
#define NOMINMAX
#endif /* NOMINMAX */

#include <windows.h>

#ifndef _WIN32_WINNT
#error \
    "Please compile grpc with _WIN32_WINNT of at least 0x600 (aka Windows Vista)"
#else /* !defined(_WIN32_WINNT) */
#if (_WIN32_WINNT < 0x0600)
#error \
    "Please compile grpc with _WIN32_WINNT of at least 0x600 (aka Windows Vista)"
#endif /* _WIN32_WINNT < 0x0600 */

#ifdef GRPC_WIN32_LEAN_AND_MEAN_WAS_NOT_DEFINED
#undef GRPC_WIN32_LEAN_AND_MEAN_WAS_NOT_DEFINED
#undef WIN32_LEAN_AND_MEAN
#endif /* GRPC_WIN32_LEAN_AND_MEAN_WAS_NOT_DEFINED */

#ifdef GRPC_NOMINMAX_WAS_NOT_DEFINED
#undef GRPC_NOMINMAX_WAS_NOT_DEFINED
#undef NOMINMAX
#endif /* GRPC_WIN32_LEAN_AND_MEAN_WAS_NOT_DEFINED */
#endif /* defined(_WIN64) || defined(WIN64) || defined(_WIN32) || \
          defined(WIN32) */
#else
#define GRPC_DLL
#define GRPCXX_DLL
#define GPR_DLL
#endif /* defined(_WIN32_WINNT) */

/* Override this file with one for your platform if you need to redefine
   things.  */

#if !defined(GPR_NO_AUTODETECT_PLATFORM)
#if defined(_WIN64) || defined(WIN64) || defined(_WIN32) || defined(WIN32)
#if defined(_WIN64) || defined(WIN64)
#define GPR_ARCH_64 1
#else
#define GPR_ARCH_32 1
#endif
#define GPR_PLATFORM_STRING "windows"
#define GPR_WINDOWS 1
#define GPR_WINDOWS_SUBPROCESS 1
#define GPR_WINDOWS_ENV
#ifdef __MSYS__
#define GPR_GETPID_IN_UNISTD_H 1
#define GPR_MSYS_TMPFILE
#define GPR_POSIX_LOG
#define GPR_POSIX_STRING
#define GPR_POSIX_TIME
#else
#define GPR_GETPID_IN_PROCESS_H 1
#define GPR_WINDOWS_TMPFILE
#define GPR_WINDOWS_LOG
#define GPR_WINDOWS_CRASH_HANDLER 1
#define GPR_WINDOWS_STAT
#define GPR_WINDOWS_STRING
#define GPR_WINDOWS_TIME
#endif
#ifdef __GNUC__
#define GPR_GCC_ATOMIC 1
#else
#define GPR_WINDOWS_ATOMIC 1
#endif
#elif defined(ANDROID) || defined(__ANDROID__)
#define GPR_PLATFORM_STRING "android"
#define GPR_ANDROID 1
#ifndef __ANDROID_API__
#error "__ANDROID_API__ must be defined for Android builds."
#endif
#if __ANDROID_API__ < 21
#error "Requires Android API v21 and above"
#endif
// TODO(apolcyn): re-evaluate support for c-ares
// on android after upgrading our c-ares dependency.
// See https://github.com/grpc/grpc/issues/18038.
#define GRPC_ARES 0
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#define GPR_CPU_POSIX 1
#define GPR_GCC_SYNC 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_ANDROID_LOG 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#define GPR_SUPPORT_CHANNELS_FROM_FD 1
#if defined(__has_include)
#if __has_include(<android/ndk-version.h>)
#include <android/ndk-version.h>
#endif /* __has_include(<android/ndk-version.h>) */
#endif /* defined(__has_include) */
#include <linux/version.h>
#elif defined(__linux__)
#define GPR_PLATFORM_STRING "linux"
#ifndef _BSD_SOURCE
#define _BSD_SOURCE
#endif
#ifndef _DEFAULT_SOURCE
#define _DEFAULT_SOURCE
#endif
#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include <features.h>
#define GPR_CPU_LINUX 1
#define GPR_GCC_ATOMIC 1
#define GPR_LINUX 1
#define GPR_LINUX_LOG
#define GPR_SUPPORT_CHANNELS_FROM_FD 1
#define GPR_LINUX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#ifdef __GLIBC__
#define GPR_POSIX_CRASH_HANDLER 1
#ifdef __GLIBC_PREREQ
#if __GLIBC_PREREQ(2, 12)
#define GPR_LINUX_PTHREAD_NAME 1
#endif
#else
// musl libc & others
#define GPR_LINUX_PTHREAD_NAME 1
#endif
#include <linux/version.h>
#else /* musl libc */
#define GPR_MUSL_LIBC_COMPAT 1
#endif
#elif defined(__ASYLO__)
#define GPR_ARCH_64 1
#define GPR_CPU_POSIX 1
#define GPR_PLATFORM_STRING "asylo"
#define GPR_GCC_SYNC 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_TIME 1
#define GPR_POSIX_ENV 1
#define GPR_ASYLO 1
#define GRPC_POSIX_SOCKET 1
#define GRPC_POSIX_SOCKETADDR
#define GRPC_POSIX_SOCKETUTILS 1
#define GRPC_TIMER_USE_GENERIC 1
#define GRPC_POSIX_NO_SPECIAL_WAKEUP_FD 1
#define GRPC_POSIX_WAKEUP_FD 1
#define GRPC_HAVE_MSG_NOSIGNAL 1
#define GRPC_HAVE_UNIX_SOCKET 1
#define GRPC_ARES 0
#define GPR_NO_AUTODETECT_PLATFORM 1
#elif defined(__APPLE__)
#include <Availability.h>
#include <TargetConditionals.h>
#ifndef _BSD_SOURCE
#define _BSD_SOURCE
#endif
#if TARGET_OS_IPHONE
#define GPR_PLATFORM_STRING "ios"
#define GPR_CPU_IPHONE 1
#define GRPC_CFSTREAM 1
#ifndef GRPC_IOS_EVENT_ENGINE_CLIENT
#define GRPC_IOS_EVENT_ENGINE_CLIENT 1
#endif /* GRPC_IOS_EVENT_ENGINE_CLIENT */
/* the c-ares resolver isn't safe to enable on iOS */
#define GRPC_ARES 0
#else /* TARGET_OS_IPHONE */
#define GPR_PLATFORM_STRING "osx"
#define GPR_CPU_POSIX 1
#define GPR_POSIX_CRASH_HANDLER 1
#endif
#define GPR_APPLE 1
#define GPR_GCC_ATOMIC 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#ifndef GRPC_CFSTREAM
#define GPR_SUPPORT_CHANNELS_FROM_FD 1
#endif
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#elif defined(__FreeBSD__)
#define GPR_PLATFORM_STRING "freebsd"
#ifndef _BSD_SOURCE
#define _BSD_SOURCE
#endif
#define GPR_FREEBSD 1
#define GPR_CPU_POSIX 1
#define GPR_GCC_ATOMIC 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#define GPR_SUPPORT_CHANNELS_FROM_FD 1
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#elif defined(__OpenBSD__)
#define GPR_PLATFORM_STRING "openbsd"
#ifndef _BSD_SOURCE
#define _BSD_SOURCE
#endif
#define GPR_OPENBSD 1
#define GPR_CPU_POSIX 1
#define GPR_GCC_ATOMIC 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#define GPR_SUPPORT_CHANNELS_FROM_FD 1
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#elif defined(__sun) && defined(__SVR4)
#define GPR_PLATFORM_STRING "solaris"
#define GPR_SOLARIS 1
#define GPR_CPU_POSIX 1
#define GPR_GCC_ATOMIC 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#elif defined(_AIX)
#define GPR_PLATFORM_STRING "aix"
#ifndef _ALL_SOURCE
#define _ALL_SOURCE
#endif
#define GPR_AIX 1
#define GPR_CPU_POSIX 1
#define GPR_GCC_ATOMIC 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#elif defined(__NetBSD__)
// NetBSD is a community-supported platform.
// Please contact Thomas Klausner <wiz@NetBSD.org> for support.
#define GPR_PLATFORM_STRING "netbsd"
#ifndef _BSD_SOURCE
#define _BSD_SOURCE
#endif
#define GPR_NETBSD 1
#define GPR_CPU_POSIX 1
#define GPR_GCC_ATOMIC 1
#define GPR_GCC_TLS 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_GETPID_IN_UNISTD_H 1
#define GPR_SUPPORT_CHANNELS_FROM_FD 1
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#elif defined(__native_client__)
#define GPR_PLATFORM_STRING "nacl"
#ifndef _BSD_SOURCE
#define _BSD_SOURCE
#endif
#ifndef _DEFAULT_SOURCE
#define _DEFAULT_SOURCE
#endif
#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#define GPR_NACL 1
#define GPR_CPU_POSIX 1
#define GPR_GCC_ATOMIC 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#elif defined(__Fuchsia__)
#define GRPC_ARES 0
#define GPR_FUCHSIA 1
#define GPR_ARCH_64 1
#define GPR_PLATFORM_STRING "fuchsia"
#include <features.h>
// Specifying musl libc affects wrap_memcpy.c. It causes memmove() to be
// invoked.
#define GPR_MUSL_LIBC_COMPAT 1
#define GPR_CPU_POSIX 1
#define GPR_GCC_ATOMIC 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#define GRPC_ROOT_PEM_PATH "/config/ssl/cert.pem"
#elif defined(__HAIKU__)
#define GPR_PLATFORM_STRING "haiku"
// Haiku is a community-supported platform.
// Please contact Jerome Duval <jerome.duval@gmail.com> for support.
#ifndef _BSD_SOURCE
#define _BSD_SOURCE
#endif
#define GPR_HAIKU 1
#define GPR_CPU_POSIX 1
#define GPR_GCC_ATOMIC 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SUBPROCESS 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#define GPR_SUPPORT_CHANNELS_FROM_FD 1
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#elif defined(__QNX__) || defined(__QNXNTO__)
#define GPR_PLATFORM_STRING "qnx"
#define GPR_CPU_POSIX 1
#define GPR_GCC_ATOMIC 1
#define GPR_POSIX_LOG 1
#define GPR_POSIX_ENV 1
#define GPR_POSIX_TMPFILE 1
#define GPR_POSIX_STAT 1
#define GPR_POSIX_STRING 1
#define GPR_POSIX_SYNC 1
#define GPR_POSIX_TIME 1
#define GPR_HAS_PTHREAD_H 1
#define GPR_GETPID_IN_UNISTD_H 1
#ifdef _LP64
#define GPR_ARCH_64 1
#else /* _LP64 */
#define GPR_ARCH_32 1
#endif /* _LP64 */
#else
#error "Could not auto-detect platform"
#endif
#endif /* GPR_NO_AUTODETECT_PLATFORM */

#if defined(__has_include)
#if __has_include(<atomic>)
#define GRPC_HAS_CXX11_ATOMIC
#endif /* __has_include(<atomic>) */
#endif /* defined(__has_include) */

#ifndef GPR_PLATFORM_STRING
#warning "GPR_PLATFORM_STRING not auto-detected"
#define GPR_PLATFORM_STRING "unknown"
#endif

#ifdef GPR_GCOV
#undef GPR_FORBID_UNREACHABLE_CODE
#define GPR_FORBID_UNREACHABLE_CODE 1
#endif

#ifdef _MSC_VER
#if _MSC_VER < 1700
typedef __int8 int8_t;
typedef __int16 int16_t;
typedef __int32 int32_t;
typedef __int64 int64_t;
typedef unsigned __int8 uint8_t;
typedef unsigned __int16 uint16_t;
typedef unsigned __int32 uint32_t;
typedef unsigned __int64 uint64_t;
#else
#include <stdint.h>
#endif /* _MSC_VER < 1700 */
#else
#include <stdint.h>
#endif /* _MSC_VER */

/* Type of cycle clock implementation */
#ifdef GPR_LINUX
/* Disable cycle clock by default.
   TODO(soheil): enable when we support fallback for unstable cycle clocks.
#if defined(__i386__)
#define GPR_CYCLE_COUNTER_RDTSC_32 1
#elif defined(__x86_64__) || defined(__amd64__)
#define GPR_CYCLE_COUNTER_RDTSC_64 1
#else
#define GPR_CYCLE_COUNTER_FALLBACK 1
#endif
*/
#define GPR_CYCLE_COUNTER_FALLBACK 1
#else
#define GPR_CYCLE_COUNTER_FALLBACK 1
#endif /* GPR_LINUX */

/* Cache line alignment */
#ifndef GPR_CACHELINE_SIZE_LOG
#if defined(__i386__) || defined(__x86_64__)
#define GPR_CACHELINE_SIZE_LOG 6
#endif
#ifndef GPR_CACHELINE_SIZE_LOG
/* A reasonable default guess. Note that overestimates tend to waste more
   space, while underestimates tend to waste more time. */
#define GPR_CACHELINE_SIZE_LOG 6
#endif /* GPR_CACHELINE_SIZE_LOG */
#endif /* GPR_CACHELINE_SIZE_LOG */

#define GPR_CACHELINE_SIZE (1 << GPR_CACHELINE_SIZE_LOG)

/* scrub GCC_ATOMIC if it's not available on this compiler */
#if defined(GPR_GCC_ATOMIC) && !defined(__ATOMIC_RELAXED)
#undef GPR_GCC_ATOMIC
#define GPR_GCC_SYNC 1
#endif

/* Validate platform combinations */
#if defined(GPR_GCC_ATOMIC) + defined(GPR_GCC_SYNC) + \
        defined(GPR_WINDOWS_ATOMIC) !=                \
    1
#error Must define exactly one of GPR_GCC_ATOMIC, GPR_GCC_SYNC, GPR_WINDOWS_ATOMIC
#endif

#if defined(GPR_ARCH_32) + defined(GPR_ARCH_64) != 1
#error Must define exactly one of GPR_ARCH_32, GPR_ARCH_64
#endif

#if defined(GPR_CPU_LINUX) + defined(GPR_CPU_POSIX) + defined(GPR_WINDOWS) + \
        defined(GPR_CPU_IPHONE) + defined(GPR_CPU_CUSTOM) !=                 \
    1
#error Must define exactly one of GPR_CPU_LINUX, GPR_CPU_POSIX, GPR_WINDOWS, GPR_CPU_IPHONE, GPR_CPU_CUSTOM
#endif

/* maximum alignment needed for any type on this platform, rounded up to a
   power of two */
#define GPR_MAX_ALIGNMENT 16

#ifndef GRPC_ARES
#define GRPC_ARES 1
#endif

#ifndef GRPC_IF_NAMETOINDEX
#define GRPC_IF_NAMETOINDEX 1
#endif

#ifndef GRPC_UNUSED
#if defined(__GNUC__) && !defined(__MINGW32__)
#define GRPC_UNUSED __attribute__((unused))
#else
#define GRPC_UNUSED
#endif
#endif

#ifndef GPR_PRINT_FORMAT_CHECK
#ifdef __GNUC__
#define GPR_PRINT_FORMAT_CHECK(FORMAT_STR, ARGS) \
  __attribute__((format(printf, FORMAT_STR, ARGS)))
#else
#define GPR_PRINT_FORMAT_CHECK(FORMAT_STR, ARGS)
#endif
#endif /* GPR_PRINT_FORMAT_CHECK */

#ifndef GPR_HAS_CPP_ATTRIBUTE
#ifdef __has_cpp_attribute
#define GPR_HAS_CPP_ATTRIBUTE(a) __has_cpp_attribute(a)
#else
#define GPR_HAS_CPP_ATTRIBUTE(a) 0
#endif
#endif /* GPR_HAS_CPP_ATTRIBUTE */

#if defined(__GNUC__) && !defined(__MINGW32__)
#define GPR_ALIGN_STRUCT(n) __attribute__((aligned(n)))
#else
#define GPR_ALIGN_STRUCT(n)
#endif

#ifndef GRPC_MUST_USE_RESULT
#if GPR_HAS_CPP_ATTRIBUTE(nodiscard)
#define GRPC_MUST_USE_RESULT [[nodiscard]]
#elif defined(__GNUC__) && !defined(__MINGW32__)
#define GRPC_MUST_USE_RESULT __attribute__((warn_unused_result))
#else
#define GRPC_MUST_USE_RESULT
#endif
#ifdef USE_STRICT_WARNING
/* When building with USE_STRICT_WARNING (which -Werror), types with this
   attribute will be treated as annotated with warn_unused_result, enforcing
   returned values of this type should be used.
   This is added in grpc::Status in mind to address the issue where it always
   has this annotation internally but OSS doesn't, sometimes causing internal
   build failure. To prevent this, this is added while not introducing
   a breaking change to existing user code which may not use returned values
   of grpc::Status. */
#define GRPC_MUST_USE_RESULT_WHEN_USE_STRICT_WARNING GRPC_MUST_USE_RESULT
#else
#define GRPC_MUST_USE_RESULT_WHEN_USE_STRICT_WARNING
#endif
#endif

#ifndef GRPC_REINITIALIZES
#if defined(__clang__)
#if GPR_HAS_CPP_ATTRIBUTE(clang::reinitializes)
#define GRPC_REINITIALIZES [[clang::reinitializes]]
#else
#define GRPC_REINITIALIZES
#endif
#else
#define GRPC_REINITIALIZES
#endif
#endif

#ifndef GPR_HAS_ATTRIBUTE
#ifdef __has_attribute
#define GPR_HAS_ATTRIBUTE(a) __has_attribute(a)
#else
#define GPR_HAS_ATTRIBUTE(a) 0
#endif
#endif /* GPR_HAS_ATTRIBUTE */

#if GPR_HAS_ATTRIBUTE(noreturn)
#define GPR_ATTRIBUTE_NORETURN __attribute__((noreturn))
#else
#define GPR_ATTRIBUTE_NORETURN
#endif

#if defined(GPR_FORBID_UNREACHABLE_CODE) && GPR_FORBID_UNREACHABLE_CODE
#define GPR_UNREACHABLE_CODE(STATEMENT)
#else
#ifdef __cplusplus
extern "C" {
#endif
extern void gpr_unreachable_code(const char* reason, const char* file,
                                 int line) GPR_ATTRIBUTE_NORETURN;
#ifdef __cplusplus
}
#endif
#define GPR_UNREACHABLE_CODE(STATEMENT)                   \
  do {                                                    \
    gpr_unreachable_code(#STATEMENT, __FILE__, __LINE__); \
    STATEMENT;                                            \
  } while (0)
#endif /* GPR_FORBID_UNREACHABLE_CODE */

#ifndef GPRAPI
#define GPRAPI
#endif

#ifndef GRPCAPI
#define GRPCAPI GPRAPI
#endif

#ifndef CENSUSAPI
#define CENSUSAPI GRPCAPI
#endif

#ifndef GPR_HAS_FEATURE
#ifdef __has_feature
#define GPR_HAS_FEATURE(a) __has_feature(a)
#else
#define GPR_HAS_FEATURE(a) 0
#endif
#endif /* GPR_HAS_FEATURE */

#ifndef GPR_ATTRIBUTE_NOINLINE
#if GPR_HAS_ATTRIBUTE(noinline) || (defined(__GNUC__) && !defined(__clang__))
#define GPR_ATTRIBUTE_NOINLINE __attribute__((noinline))
#define GPR_HAS_ATTRIBUTE_NOINLINE 1
#else
#define GPR_ATTRIBUTE_NOINLINE
#endif
#endif /* GPR_ATTRIBUTE_NOINLINE */

#ifndef GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION
#ifdef __cplusplus
#if GPR_HAS_CPP_ATTRIBUTE(clang::always_inline)
#define GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION [[clang::always_inline]]
#elif GPR_HAS_ATTRIBUTE(always_inline)
#define GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION __attribute__((always_inline))
#else
// TODO(ctiller): add __forceinline for MSVC
#define GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION
#endif
#else
// Disable for C code
#define GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION
#endif
#endif /* GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION */

#ifndef GPR_NO_UNIQUE_ADDRESS
#if GPR_HAS_CPP_ATTRIBUTE(no_unique_address)
#define GPR_NO_UNIQUE_ADDRESS [[no_unique_address]]
#else
#define GPR_NO_UNIQUE_ADDRESS
#endif
#endif /* GPR_NO_UNIQUE_ADDRESS */

#ifndef GRPC_DEPRECATED
#if GPR_HAS_CPP_ATTRIBUTE(deprecated)
#define GRPC_DEPRECATED(reason) [[deprecated(reason)]]
#else
#define GRPC_DEPRECATED(reason)
#endif
#endif /* GRPC_DEPRECATED */

#ifndef GPR_ATTRIBUTE_WEAK
/* Attribute weak is broken on LLVM/windows:
 * https://bugs.llvm.org/show_bug.cgi?id=37598 */
#if (GPR_HAS_ATTRIBUTE(weak) || (defined(__GNUC__) && !defined(__clang__))) && \
    !(defined(__llvm__) && defined(_WIN32))
#define GPR_ATTRIBUTE_WEAK __attribute__((weak))
#define GPR_HAS_ATTRIBUTE_WEAK 1
#else
#define GPR_ATTRIBUTE_WEAK
#endif
#endif /* GPR_ATTRIBUTE_WEAK */

#ifndef GPR_ATTRIBUTE_NO_TSAN /* (1) */
#if GPR_HAS_FEATURE(thread_sanitizer)
#define GPR_ATTRIBUTE_NO_TSAN __attribute__((no_sanitize("thread")))
#endif                        /* GPR_HAS_FEATURE */
#ifndef GPR_ATTRIBUTE_NO_TSAN /* (2) */
#define GPR_ATTRIBUTE_NO_TSAN
#endif /* GPR_ATTRIBUTE_NO_TSAN (2) */
#endif /* GPR_ATTRIBUTE_NO_TSAN (1) */

/* GRPC_TSAN_ENABLED will be defined, when compiled with thread sanitizer. */
#ifndef GRPC_TSAN_SUPPRESSED
#if defined(__SANITIZE_THREAD__)
#define GRPC_TSAN_ENABLED
#elif GPR_HAS_FEATURE(thread_sanitizer)
#define GRPC_TSAN_ENABLED
#endif
#endif

/* GRPC_ASAN_ENABLED will be defined, when compiled with address sanitizer. */
#ifndef GRPC_ASAN_SUPPRESSED
#if defined(__SANITIZE_ADDRESS__)
#define GRPC_ASAN_ENABLED
#elif GPR_HAS_FEATURE(address_sanitizer)
#define GRPC_ASAN_ENABLED
#endif
#endif

/* GRPC_ALLOW_EXCEPTIONS should be 0 or 1 if exceptions are allowed or not */
#ifndef GRPC_ALLOW_EXCEPTIONS
#ifdef GPR_WINDOWS
#if defined(_MSC_VER) && defined(_CPPUNWIND)
#define GRPC_ALLOW_EXCEPTIONS 1
#elif defined(__EXCEPTIONS)
#define GRPC_ALLOW_EXCEPTIONS 1
#else
#define GRPC_ALLOW_EXCEPTIONS 0
#endif
#else /* GPR_WINDOWS */
#ifdef __EXCEPTIONS
#define GRPC_ALLOW_EXCEPTIONS 1
#else /* __EXCEPTIONS */
#define GRPC_ALLOW_EXCEPTIONS 0
#endif /* __EXCEPTIONS */
#endif /* __GPR_WINDOWS */
#endif /* GRPC_ALLOW_EXCEPTIONS */

#ifdef __has_builtin
#define GRPC_HAS_BUILTIN(a) __has_builtin(a)
#else
#define GRPC_HAS_BUILTIN(a) 0
#endif

/* Use GPR_LIKELY only in cases where you are sure that a certain outcome is the
 * most likely. Ideally, also collect performance numbers to justify the claim.
 */
#ifdef __GNUC__
#define GPR_LIKELY(x) __builtin_expect((x), 1)
#define GPR_UNLIKELY(x) __builtin_expect((x), 0)
#else /* __GNUC__ */
#define GPR_LIKELY(x) (x)
#define GPR_UNLIKELY(x) (x)
#endif /* __GNUC__ */

#ifndef __STDC_FORMAT_MACROS
#define __STDC_FORMAT_MACROS
#endif

/* MSVC doesn't do the empty base class optimization in debug builds by default,
 * and because of ABI likely won't.
 * This enables it for specific types, use as:
 * class GPR_MSVC_EMPTY_BASE_CLASS_WORKAROUND Foo : public A, public B, public C
 * {}; */
#ifndef GPR_MSVC_EMPTY_BASE_CLASS_WORKAROUND
#ifdef GPR_WINDOWS
#define GPR_MSVC_EMPTY_BASE_CLASS_WORKAROUND __declspec(empty_bases)
#else
#define GPR_MSVC_EMPTY_BASE_CLASS_WORKAROUND
#endif
#endif

#define GRPC_CALLBACK_API_NONEXPERIMENTAL

/* clang 12 and lower with msan miscompiles destruction of [[no_unique_address]]
 * members of zero size - for a repro see:
 * test/core/compiler_bugs/miscompile_with_no_unique_address_test.cc
 */
#ifdef __clang__
#if __clang__ && __clang_major__ <= 12 && __has_feature(memory_sanitizer)
#undef GPR_NO_UNIQUE_ADDRESS
#define GPR_NO_UNIQUE_ADDRESS
#endif
#endif

#endif /* GRPC_SUPPORT_PORT_PLATFORM_H */
