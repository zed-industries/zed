// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_TEST_MACROS_HPP
#define SUPPORT_TEST_MACROS_HPP

#ifdef __has_include
#  if __has_include(<version>)
#    include <version>
#  else
#    include <ciso646>
#  endif
#else
#  include <ciso646>
#endif

#define TEST_STRINGIZE_IMPL(...) #__VA_ARGS__
#define TEST_STRINGIZE(...) TEST_STRINGIZE_IMPL(__VA_ARGS__)

#define TEST_CONCAT1(X, Y) X##Y
#define TEST_CONCAT(X, Y) TEST_CONCAT1(X, Y)

#ifdef __has_feature
#define TEST_HAS_FEATURE(X) __has_feature(X)
#else
#define TEST_HAS_FEATURE(X) 0
#endif

#ifndef __has_include
#define __has_include(...) 0
#endif

#ifdef __has_extension
#define TEST_HAS_EXTENSION(X) __has_extension(X)
#else
#define TEST_HAS_EXTENSION(X) 0
#endif

#ifdef __has_warning
#define TEST_HAS_WARNING(X) __has_warning(X)
#else
#define TEST_HAS_WARNING(X) 0
#endif

#ifdef __has_builtin
#define TEST_HAS_BUILTIN(X) __has_builtin(X)
#else
#define TEST_HAS_BUILTIN(X) 0
#endif
#ifdef __is_identifier
// '__is_identifier' returns '0' if '__x' is a reserved identifier provided by
// the compiler and '1' otherwise.
#define TEST_HAS_BUILTIN_IDENTIFIER(X) !__is_identifier(X)
#else
#define TEST_HAS_BUILTIN_IDENTIFIER(X) 0
#endif

#if defined(__EDG__)
# define TEST_COMPILER_EDG
#elif defined(__clang__)
# define TEST_COMPILER_CLANG
# if defined(__apple_build_version__)
#  define TEST_COMPILER_APPLE_CLANG
# endif
#elif defined(_MSC_VER)
# define TEST_COMPILER_MSVC
#elif defined(__GNUC__)
# define TEST_COMPILER_GCC
#endif

#if defined(__apple_build_version__)
// Given AppleClang XX.Y.Z, TEST_APPLE_CLANG_VER is XXYZ (e.g. AppleClang 14.0.3 => 1403)
#define TEST_APPLE_CLANG_VER (__apple_build_version__ / 10000)
#elif defined(__clang_major__)
#define TEST_CLANG_VER (__clang_major__ * 100) + __clang_minor__
#elif defined(__GNUC__)
// Given GCC XX.YY.ZZ, TEST_GCC_VER is XXYYZZ
#define TEST_GCC_VER ((__GNUC__ * 10000) + (__GNUC_MINOR__ * 100) + __GNUC_PATCHLEVEL__)
#endif

/* Make a nice name for the standard version */
#ifndef TEST_STD_VER
#if  __cplusplus <= 199711L
# define TEST_STD_VER 3
#elif __cplusplus <= 201103L
# define TEST_STD_VER 11
#elif __cplusplus <= 201402L
# define TEST_STD_VER 14
#elif __cplusplus <= 201703L
# define TEST_STD_VER 17
#elif __cplusplus <= 202002L
# define TEST_STD_VER 20
#elif __cplusplus <= 202302L
# define TEST_STD_VER 23
#else
# define TEST_STD_VER 99    // greater than current standard
// This is deliberately different than _LIBCPP_STD_VER to discourage matching them up.
#endif
#endif

// Attempt to deduce the GLIBC version
#if (defined(__has_include) && __has_include(<features.h>)) || \
    defined(__linux__)
#include <features.h>
#if defined(__GLIBC_PREREQ)
#define TEST_HAS_GLIBC
#define TEST_GLIBC_PREREQ(major, minor) __GLIBC_PREREQ(major, minor)
#endif
#endif

#if TEST_STD_VER >= 11
# define TEST_ALIGNOF(...) alignof(__VA_ARGS__)
# define TEST_ALIGNAS(...) alignas(__VA_ARGS__)
# define TEST_CONSTEXPR constexpr
# define TEST_NOEXCEPT noexcept
# define TEST_NOEXCEPT_FALSE noexcept(false)
# define TEST_NOEXCEPT_COND(...) noexcept(__VA_ARGS__)
#else
#   if defined(TEST_COMPILER_CLANG)
#    define TEST_ALIGNOF(...) _Alignof(__VA_ARGS__)
#   else
#    define TEST_ALIGNOF(...) __alignof(__VA_ARGS__)
#   endif
# define TEST_ALIGNAS(...) __attribute__((__aligned__(__VA_ARGS__)))
# define TEST_CONSTEXPR
# define TEST_NOEXCEPT throw()
# define TEST_NOEXCEPT_FALSE
# define TEST_NOEXCEPT_COND(...)
#endif

#if TEST_STD_VER >= 11
# define TEST_THROW_SPEC(...)
#else
# define TEST_THROW_SPEC(...) throw(__VA_ARGS__)
#endif

#if defined(__cpp_lib_is_constant_evaluated) && __cpp_lib_is_constant_evaluated >= 201811L
# define TEST_IS_CONSTANT_EVALUATED std::is_constant_evaluated()
#elif TEST_HAS_BUILTIN(__builtin_is_constant_evaluated)
# define TEST_IS_CONSTANT_EVALUATED __builtin_is_constant_evaluated()
#else
# define TEST_IS_CONSTANT_EVALUATED false
#endif

#if TEST_STD_VER >= 20
#  define TEST_STD_AT_LEAST_20_OR_RUNTIME_EVALUATED true
#else
#  define TEST_STD_AT_LEAST_20_OR_RUNTIME_EVALUATED (!TEST_IS_CONSTANT_EVALUATED)
#endif

#if TEST_STD_VER >= 23
#  define TEST_STD_AT_LEAST_23_OR_RUNTIME_EVALUATED true
#else
#  define TEST_STD_AT_LEAST_23_OR_RUNTIME_EVALUATED (!TEST_IS_CONSTANT_EVALUATED)
#endif

#if TEST_STD_VER >= 26
#  define TEST_STD_AT_LEAST_26_OR_RUNTIME_EVALUATED true
#else
#  define TEST_STD_AT_LEAST_26_OR_RUNTIME_EVALUATED (!TEST_IS_CONSTANT_EVALUATED)
#endif

#if TEST_STD_VER >= 14
# define TEST_CONSTEXPR_CXX14 constexpr
#else
# define TEST_CONSTEXPR_CXX14
#endif

#if TEST_STD_VER >= 17
# define TEST_CONSTEXPR_CXX17 constexpr
#else
# define TEST_CONSTEXPR_CXX17
#endif

#if TEST_STD_VER >= 20
# define TEST_CONSTEXPR_CXX20 constexpr
#else
# define TEST_CONSTEXPR_CXX20
#endif

#if TEST_STD_VER >= 23
#  define TEST_CONSTEXPR_CXX23 constexpr
#else
#  define TEST_CONSTEXPR_CXX23
#endif

#if TEST_STD_VER >= 26
#  define TEST_CONSTEXPR_CXX26 constexpr
#else
#  define TEST_CONSTEXPR_CXX26
#endif

#define TEST_ALIGNAS_TYPE(...) TEST_ALIGNAS(TEST_ALIGNOF(__VA_ARGS__))

#if !TEST_HAS_FEATURE(cxx_rtti) && !defined(__cpp_rtti) \
    && !defined(__GXX_RTTI)
#define TEST_HAS_NO_RTTI
#endif

#if !defined(TEST_HAS_NO_RTTI)
# define RTTI_ASSERT(X) assert(X)
#else
# define RTTI_ASSERT(X)
#endif

#if !TEST_HAS_FEATURE(cxx_exceptions) && !defined(__cpp_exceptions) \
     && !defined(__EXCEPTIONS)
#define TEST_HAS_NO_EXCEPTIONS
#endif

#if TEST_HAS_FEATURE(address_sanitizer) || TEST_HAS_FEATURE(hwaddress_sanitizer) || \
    TEST_HAS_FEATURE(memory_sanitizer) || TEST_HAS_FEATURE(thread_sanitizer)
#define TEST_HAS_SANITIZERS
#define TEST_IS_EXECUTED_IN_A_SLOW_ENVIRONMENT
#endif

#ifdef _LIBCPP_USE_FROZEN_CXX03_HEADERS
#  ifdef _LIBCPP_HAS_NO_ALIGNED_ALLOCATION
#    define TEST_HAS_NO_ALIGNED_ALLOCATION
#  endif
#elif defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_ALIGNED_ALLOCATION
#  define TEST_HAS_NO_ALIGNED_ALLOCATION
#elif TEST_STD_VER < 17 && (!defined(__cpp_aligned_new) || __cpp_aligned_new < 201606L)
#  define TEST_HAS_NO_ALIGNED_ALLOCATION
#endif

#if TEST_STD_VER > 17
#  define TEST_CONSTINIT constinit
#elif __has_cpp_attribute(clang::require_constant_initialization)
#  define TEST_CONSTINIT [[clang::require_constant_initialization]]
#else
#  define TEST_CONSTINIT
#endif

#if TEST_STD_VER < 11
#define ASSERT_NOEXCEPT(...)
#define ASSERT_NOT_NOEXCEPT(...)
#else
#define ASSERT_NOEXCEPT(...) \
    static_assert(noexcept(__VA_ARGS__), "Operation must be noexcept")

#define ASSERT_NOT_NOEXCEPT(...) \
    static_assert(!noexcept(__VA_ARGS__), "Operation must NOT be noexcept")
#endif

/* Macros for testing libc++ specific behavior and extensions */
#if defined(_LIBCPP_VERSION)
#define LIBCPP_ASSERT(...) assert(__VA_ARGS__)
#define LIBCPP_STATIC_ASSERT(...) static_assert(__VA_ARGS__)
#define LIBCPP_ASSERT_NOEXCEPT(...) ASSERT_NOEXCEPT(__VA_ARGS__)
#define LIBCPP_ASSERT_NOT_NOEXCEPT(...) ASSERT_NOT_NOEXCEPT(__VA_ARGS__)
#define LIBCPP_ONLY(...) __VA_ARGS__
#else
#define LIBCPP_ASSERT(...) static_assert(true, "")
#define LIBCPP_STATIC_ASSERT(...) static_assert(true, "")
#define LIBCPP_ASSERT_NOEXCEPT(...) static_assert(true, "")
#define LIBCPP_ASSERT_NOT_NOEXCEPT(...) static_assert(true, "")
#define LIBCPP_ONLY(...) static_assert(true, "")
#endif

#if __has_cpp_attribute(nodiscard)
#  define TEST_NODISCARD [[nodiscard]]
#else
#  define TEST_NODISCARD
#endif

#define TEST_IGNORE_NODISCARD (void)

#ifdef _LIBCPP_USE_FROZEN_CXX03_HEADERS
// from-chars is a C++17 feature, so it's never available anyways
#elif !defined(_LIBCPP_VERSION) || _LIBCPP_AVAILABILITY_HAS_FROM_CHARS_FLOATING_POINT
#  define TEST_HAS_FROM_CHARS_FLOATING_POINT
#endif

namespace test_macros_detail {
template <class T, class U>
struct is_same { enum { value = 0};} ;
template <class T>
struct is_same<T, T> { enum {value = 1}; };
} // namespace test_macros_detail

#define ASSERT_SAME_TYPE(...) \
    static_assert((test_macros_detail::is_same<__VA_ARGS__>::value), \
                 "Types differ unexpectedly")

#ifndef TEST_HAS_NO_EXCEPTIONS
#define TEST_THROW(...) throw __VA_ARGS__
#else
#if defined(__GNUC__)
#define TEST_THROW(...) __builtin_abort()
#else
#include <stdlib.h>
#define TEST_THROW(...) ::abort()
#endif
#endif

#if defined(__GNUC__) || defined(__clang__)
// This function can be used to hide some objects from compiler optimizations.
//
// For example, this is useful to hide the result of a call to `new` and ensure
// that the compiler doesn't elide the call to new/delete. Otherwise, elliding
// calls to new/delete is allowed by the Standard and compilers actually do it
// when optimizations are enabled.
template <class Tp>
inline Tp const& DoNotOptimize(Tp const& value) {
  // The `m` constraint is invalid in the AMDGPU backend.
#  if defined(__AMDGPU__) || defined(__NVPTX__)
  asm volatile("" : : "r"(value) : "memory");
#  else
  asm volatile("" : : "r,m"(value) : "memory");
#  endif
  return value;
}

template <class Tp>
inline Tp& DoNotOptimize(Tp& value) {
  // The `m` and `r` output constraint is invalid in the AMDGPU backend as well
  // as i8 / i1 arguments, so we just capture the pointer instead.
#  if defined(__AMDGPU__)
  Tp* tmp = &value;
  asm volatile("" : "+v"(tmp) : : "memory");
#  elif defined(__clang__)
  asm volatile("" : "+r,m"(value) : : "memory");
#  else
  asm volatile("" : "+m,r"(value) : : "memory");
#  endif
  return value;
}
#else
#include <intrin.h>
template <class Tp>
inline Tp const& DoNotOptimize(Tp const& value) {
  const volatile void* volatile unused = __builtin_addressof(value);
  static_cast<void>(unused);
  _ReadWriteBarrier();
  return value;
}
#endif

#if defined(__GNUC__)
#define TEST_ALWAYS_INLINE __attribute__((always_inline))
#define TEST_NOINLINE __attribute__((noinline))
#elif defined(_MSC_VER)
#define TEST_ALWAYS_INLINE __forceinline
#define TEST_NOINLINE __declspec(noinline)
#else
#define TEST_ALWAYS_INLINE
#define TEST_NOINLINE
#endif

#ifdef _WIN32
#define TEST_NOT_WIN32(...) ((void)0)
#else
#define TEST_NOT_WIN32(...) __VA_ARGS__
#endif

#if defined(TEST_WINDOWS_DLL) ||defined(__MVS__) || defined(_AIX)
// Macros for waiving cases when we can't count allocations done within
// the library implementation.
//
// On Windows, when libc++ is built as a DLL, references to operator new/delete
// within the DLL are bound at link time to the operator new/delete within
// the library; replacing them in the user executable doesn't override the
// calls within the library.
//
// The same goes on IBM zOS.
// The same goes on AIX.
#define ASSERT_WITH_LIBRARY_INTERNAL_ALLOCATIONS(...) ((void)(__VA_ARGS__))
#define TEST_SUPPORTS_LIBRARY_INTERNAL_ALLOCATIONS 0
#else
#define ASSERT_WITH_LIBRARY_INTERNAL_ALLOCATIONS(...) assert(__VA_ARGS__)
#define TEST_SUPPORTS_LIBRARY_INTERNAL_ALLOCATIONS 1
#endif

#if (defined(TEST_WINDOWS_DLL) && !defined(_MSC_VER)) ||                      \
    defined(__MVS__)
// Normally, a replaced e.g. 'operator new' ends up used if the user code
// does a call to e.g. 'operator new[]'; it's enough to replace the base
// versions and have it override all of them.
//
// When the fallback operators are located within the libc++ library and we
// can't override the calls within it (see above), this fallback mechanism
// doesn't work either.
//
// On Windows, when using the MSVC vcruntime, the operator new/delete fallbacks
// are linked separately from the libc++ library, linked statically into
// the end user executable, and these fallbacks work even in DLL configurations.
// In MinGW configurations when built as a DLL, and on zOS, these fallbacks
// don't work though.
#define ASSERT_WITH_OPERATOR_NEW_FALLBACKS(...) ((void)(__VA_ARGS__))
#else
#define ASSERT_WITH_OPERATOR_NEW_FALLBACKS(...) assert(__VA_ARGS__)
#endif

#ifdef _WIN32
#define TEST_WIN_NO_FILESYSTEM_PERMS_NONE
#endif

// Support for carving out parts of the test suite, like removing wide characters, etc.
#if defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_WIDE_CHARACTERS
#  define TEST_HAS_NO_WIDE_CHARACTERS
#endif

#if defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_UNICODE
#  define TEST_HAS_NO_UNICODE
#elif defined(_MSVC_EXECUTION_CHARACTER_SET) && _MSVC_EXECUTION_CHARACTER_SET != 65001
#  define TEST_HAS_NO_UNICODE
#endif

#ifdef _LIBCPP_USE_FROZEN_CXX03_HEADERS
#  ifdef _LIBCPP_HAS_OPEN_WITH_WCHAR
#    define TEST_HAS_OPEN_WITH_WCHAR
#  endif
#elif defined(_LIBCPP_VERSION) && _LIBCPP_HAS_OPEN_WITH_WCHAR
#  define TEST_HAS_OPEN_WITH_WCHAR
#endif

#ifdef _LIBCPP_USE_FROZEN_CXX03_HEADERS
#  ifdef _LIBCPP_HAS_NO_INT128
#    define TEST_HAS_NO_INT128
#  endif
#elif (defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_INT128) || defined(_MSVC_STL_VERSION)
#  define TEST_HAS_NO_INT128
#endif

#if defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_LOCALIZATION
#  define TEST_HAS_NO_LOCALIZATION
#endif

#if TEST_STD_VER <= 17 || !defined(__cpp_char8_t)
#  define TEST_HAS_NO_CHAR8_T
#endif

#if defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_THREADS
#  define TEST_HAS_NO_THREADS
#endif

#if defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_FILESYSTEM
#  define TEST_HAS_NO_FILESYSTEM
#endif

#ifdef _LIBCPP_USE_FROZEN_CXX03_HEADERS
#  ifdef _LIBCPP_HAS_NO_C8RTOMB_MBRTOC8
#    define TEST_HAS_NO_C8RTOMB_MBRTOC8
#  endif
#elif defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_C8RTOMB_MBRTOC8
#  define TEST_HAS_NO_C8RTOMB_MBRTOC8
#endif

#if defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_RANDOM_DEVICE
#  define TEST_HAS_NO_RANDOM_DEVICE
#endif

#ifdef _LIBCPP_USE_FROZEN_CXX03_HEADERS
// This is a C++20 feature, so it's never available anyways
#  define TEST_HAS_NO_EXPERIMENTAL_TZDB
#elif defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_EXPERIMENTAL_TZDB
#  define TEST_HAS_NO_EXPERIMENTAL_TZDB
#endif

#if defined(_LIBCPP_VERSION) && !_LIBCPP_HAS_TIME_ZONE_DATABASE
#  define TEST_HAS_NO_TIME_ZONE_DATABASE
#endif

#if defined(TEST_COMPILER_CLANG)
#  define TEST_DIAGNOSTIC_PUSH _Pragma("clang diagnostic push")
#  define TEST_DIAGNOSTIC_POP _Pragma("clang diagnostic pop")
#  define TEST_CLANG_DIAGNOSTIC_IGNORED(str) _Pragma(TEST_STRINGIZE(clang diagnostic ignored str))
#  define TEST_GCC_DIAGNOSTIC_IGNORED(str)
#  define TEST_MSVC_DIAGNOSTIC_IGNORED(num)
#elif defined(TEST_COMPILER_GCC)
#  define TEST_DIAGNOSTIC_PUSH _Pragma("GCC diagnostic push")
#  define TEST_DIAGNOSTIC_POP _Pragma("GCC diagnostic pop")
#  define TEST_CLANG_DIAGNOSTIC_IGNORED(str)
#  define TEST_GCC_DIAGNOSTIC_IGNORED(str) _Pragma(TEST_STRINGIZE(GCC diagnostic ignored str))
#  define TEST_MSVC_DIAGNOSTIC_IGNORED(num)
#elif defined(TEST_COMPILER_MSVC)
#  define TEST_DIAGNOSTIC_PUSH _Pragma("warning(push)")
#  define TEST_DIAGNOSTIC_POP _Pragma("warning(pop)")
#  define TEST_CLANG_DIAGNOSTIC_IGNORED(str)
#  define TEST_GCC_DIAGNOSTIC_IGNORED(str)
#  define TEST_MSVC_DIAGNOSTIC_IGNORED(num) _Pragma(TEST_STRINGIZE(warning(disable: num)))
#else
#  define TEST_DIAGNOSTIC_PUSH
#  define TEST_DIAGNOSTIC_POP
#  define TEST_CLANG_DIAGNOSTIC_IGNORED(str)
#  define TEST_GCC_DIAGNOSTIC_IGNORED(str)
#  define TEST_MSVC_DIAGNOSTIC_IGNORED(num)
#endif

#if __has_cpp_attribute(msvc::no_unique_address)
#define TEST_NO_UNIQUE_ADDRESS [[msvc::no_unique_address]]
#elif __has_cpp_attribute(no_unique_address)
#define TEST_NO_UNIQUE_ADDRESS [[no_unique_address]]
#else
#define TEST_NO_UNIQUE_ADDRESS
#endif

#ifdef _LIBCPP_SHORT_WCHAR
#  define TEST_SHORT_WCHAR
#endif

#ifdef _LIBCPP_ABI_MICROSOFT
#  define TEST_ABI_MICROSOFT
#endif

// This is a temporary workaround for user-defined `operator new` definitions
// not being picked up on Apple platforms in some circumstances. This is under
// investigation and should be short-lived.
#ifdef __APPLE__
#  define TEST_WORKAROUND_BUG_109234844_WEAK __attribute__((weak))
#else
#  define TEST_WORKAROUND_BUG_109234844_WEAK /* nothing */
#endif

#ifdef _AIX
#  define TEST_IF_AIX(arg_true, arg_false) arg_true
#else
#  define TEST_IF_AIX(arg_true, arg_false) arg_false
#endif

// Clang-18 has support for deducing this, but it does not set the FTM.
#ifdef _LIBCPP_USE_FROZEN_CXX03_HEADERS
// This is a C++20 featue, so we don't care whether the compiler could support it
#elif defined(_LIBCPP_VERSION) && _LIBCPP_HAS_EXPLICIT_THIS_PARAMETER
#  define TEST_HAS_EXPLICIT_THIS_PARAMETER
#endif

// Placement `operator new`/`operator new[]` are not yet constexpr in C++26
// when using MS ABI, because they are from <vcruntime_new.h>.
#if defined(__cpp_lib_constexpr_new) && __cpp_lib_constexpr_new >= 202406L
#  define TEST_CONSTEXPR_OPERATOR_NEW constexpr
#else
#  define TEST_CONSTEXPR_OPERATOR_NEW
#endif

#if defined(_MSC_VER) || __SIZEOF_LONG_DOUBLE__ == __SIZEOF_DOUBLE__
#  define TEST_LONG_DOUBLE_IS_DOUBLE
#endif

#if defined(__LDBL_MANT_DIG__) && __LDBL_MANT_DIG__ == 64
#  define TEST_LONG_DOUBLE_IS_80_BIT
#endif

#endif // SUPPORT_TEST_MACROS_HPP
