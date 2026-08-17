#ifndef BENCHMARK_INTERNAL_MACROS_H_
#define BENCHMARK_INTERNAL_MACROS_H_

/* Needed to detect STL */
#include <cstdlib>

// clang-format off

#ifndef __has_feature
#define __has_feature(x) 0
#endif

#if defined(__clang__)
  #if !defined(COMPILER_CLANG)
    #define COMPILER_CLANG
  #endif
#elif defined(_MSC_VER)
  #if !defined(COMPILER_MSVC)
    #define COMPILER_MSVC
  #endif
#elif defined(__GNUC__)
  #if !defined(COMPILER_GCC)
    #define COMPILER_GCC
  #endif
#endif

#if __has_feature(cxx_attributes)
  #define BENCHMARK_NORETURN [[noreturn]]
#elif defined(__GNUC__)
  #define BENCHMARK_NORETURN __attribute__((noreturn))
#elif defined(COMPILER_MSVC)
  #define BENCHMARK_NORETURN __declspec(noreturn)
#else
  #define BENCHMARK_NORETURN
#endif

#if defined(__CYGWIN__)
  #define BENCHMARK_OS_CYGWIN 1
#elif defined(_WIN32)
  #define BENCHMARK_OS_WINDOWS 1
  // WINAPI_FAMILY_PARTITION is defined in winapifamily.h.
  // We include windows.h which implicitly includes winapifamily.h for compatibility.
  #ifndef NOMINMAX
    #define NOMINMAX
  #endif
  #include <windows.h>
  #if defined(WINAPI_FAMILY_PARTITION)
    #if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
      #define BENCHMARK_OS_WINDOWS_WIN32 1
    #elif WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
      #define BENCHMARK_OS_WINDOWS_RT 1
    #endif
  #endif
  #if defined(__MINGW32__)
    #define BENCHMARK_OS_MINGW 1
  #endif
#elif defined(__APPLE__)
  #define BENCHMARK_OS_APPLE 1
  #include "TargetConditionals.h"
  #if defined(TARGET_OS_MAC)
    #define BENCHMARK_OS_MACOSX 1
    #if defined(TARGET_OS_IPHONE)
      #define BENCHMARK_OS_IOS 1
    #endif
  #endif
#elif defined(__FreeBSD__)
  #define BENCHMARK_OS_FREEBSD 1
#elif defined(__NetBSD__)
  #define BENCHMARK_OS_NETBSD 1
#elif defined(__OpenBSD__)
  #define BENCHMARK_OS_OPENBSD 1
#elif defined(__DragonFly__)
  #define BENCHMARK_OS_DRAGONFLY 1
#elif defined(__linux__)
  #define BENCHMARK_OS_LINUX 1
#elif defined(__native_client__)
  #define BENCHMARK_OS_NACL 1
#elif defined(__EMSCRIPTEN__)
  #define BENCHMARK_OS_EMSCRIPTEN 1
#elif defined(__rtems__)
  #define BENCHMARK_OS_RTEMS 1
#elif defined(__Fuchsia__)
#define BENCHMARK_OS_FUCHSIA 1
#elif defined (__SVR4) && defined (__sun)
#define BENCHMARK_OS_SOLARIS 1
#elif defined(__QNX__)
#define BENCHMARK_OS_QNX 1
#elif defined(__MVS__)
#define BENCHMARK_OS_ZOS 1
#elif defined(__hexagon__)
#define BENCHMARK_OS_QURT 1
#endif

#if defined(__ANDROID__) && defined(__GLIBCXX__)
#define BENCHMARK_STL_ANDROID_GNUSTL 1
#endif

#if !__has_feature(cxx_exceptions) && !defined(__cpp_exceptions) \
     && !defined(__EXCEPTIONS)
  #define BENCHMARK_HAS_NO_EXCEPTIONS
#endif

#if defined(COMPILER_CLANG) || defined(COMPILER_GCC)
  #define BENCHMARK_MAYBE_UNUSED __attribute__((unused))
#else
  #define BENCHMARK_MAYBE_UNUSED
#endif

// clang-format on

#endif  // BENCHMARK_INTERNAL_MACROS_H_
