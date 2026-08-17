//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_MSVC_STDLIB_FORCE_INCLUDE_H
#define SUPPORT_MSVC_STDLIB_FORCE_INCLUDE_H

// This header is force-included when running the libc++ tests against the
// MSVC standard library.

#ifndef _LIBCXX_IN_DEVCRT
// Silence warnings about CRT machinery.
#  define _CRT_SECURE_NO_WARNINGS 1

// Avoid assertion dialogs.
#  define _CRT_SECURE_INVALID_PARAMETER(EXPR) ::abort()

// Declare POSIX function names. (By default, Clang -fno-ms-compatibility causes them to be omitted.)
#  define _CRT_DECLARE_NONSTDC_NAMES 1

// Silence warnings about POSIX function names.
#  define _CRT_NONSTDC_NO_WARNINGS 1

// Avoid Windows.h macroizing min() and max().
#  define NOMINMAX 1
#endif // _LIBCXX_IN_DEVCRT

#include <crtdbg.h>
#include <stdlib.h>

#if defined(_LIBCPP_VERSION)
#  error This header may not be used when targeting libc++
#endif

#ifndef _LIBCXX_IN_DEVCRT
struct AssertionDialogAvoider {
  AssertionDialogAvoider() {
    _CrtSetReportMode(_CRT_ASSERT, _CRTDBG_MODE_FILE);
    _CrtSetReportFile(_CRT_ASSERT, _CRTDBG_FILE_STDERR);

    _CrtSetReportMode(_CRT_ERROR, _CRTDBG_MODE_FILE);
    _CrtSetReportFile(_CRT_ERROR, _CRTDBG_FILE_STDERR);
  }
};

const AssertionDialogAvoider assertion_dialog_avoider{};
#endif // _LIBCXX_IN_DEVCRT

// MSVC frontend only configurations
#if !defined(__clang__)
// Simulate feature-test macros.
#  define __has_feature(X) _MSVC_HAS_FEATURE_##X
#  define _MSVC_HAS_FEATURE_cxx_exceptions 1
#  define _MSVC_HAS_FEATURE_cxx_rtti 1
#  define _MSVC_HAS_FEATURE_address_sanitizer 0
#  define _MSVC_HAS_FEATURE_hwaddress_sanitizer 0
#  define _MSVC_HAS_FEATURE_memory_sanitizer 0
#  define _MSVC_HAS_FEATURE_thread_sanitizer 0

#  define __has_attribute(X) _MSVC_HAS_ATTRIBUTE_##X
#  define _MSVC_HAS_ATTRIBUTE_vector_size 0

// Silence compiler warnings.
#  pragma warning(disable : 4180)  // qualifier applied to function type has no meaning; ignored
#  pragma warning(disable : 4324)  // structure was padded due to alignment specifier
#  pragma warning(disable : 4702)  // unreachable code
#  pragma warning(disable : 28251) // Inconsistent annotation for 'new': this instance has no annotations.
#endif                             // !defined(__clang__)

#ifndef _LIBCXX_IN_DEVCRT
// atomic_is_lock_free.pass.cpp needs this VS 2015 Update 2 fix.
#  define _ENABLE_ATOMIC_ALIGNMENT_FIX

// Restore features that are removed in C++20.
#  define _HAS_FEATURES_REMOVED_IN_CXX20 1

// Silence warnings about the unspecified complex<non-floating-point>
#  define _SILENCE_NONFLOATING_COMPLEX_DEPRECATION_WARNING

// Silence warnings about features that are deprecated in non-default language modes.
#  define _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS
#  define _SILENCE_ALL_CXX20_DEPRECATION_WARNINGS
#  define _SILENCE_ALL_CXX23_DEPRECATION_WARNINGS
#endif // _LIBCXX_IN_DEVCRT

#include <version>

#if _HAS_CXX23
#  define TEST_STD_VER 23
#elif _HAS_CXX20
#  define TEST_STD_VER 20
#elif _HAS_CXX17
#  define TEST_STD_VER 17
#else
#  define TEST_STD_VER 14
#endif

#define TEST_SHORT_WCHAR
#define TEST_ABI_MICROSOFT

#define _LIBCPP_AVAILABILITY_THROW_BAD_ANY_CAST

#endif // SUPPORT_MSVC_STDLIB_FORCE_INCLUDE_H
