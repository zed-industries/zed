//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___CXX03___CONFIGURATION_CONFIG_SITE_SHIM_H
#define _LIBCPP___CXX03___CONFIGURATION_CONFIG_SITE_SHIM_H

#include <__config_site>

#if !_LIBCPP_ABI_FORCE_ITANIUM
#  undef _LIBCPP_ABI_FORCE_ITANIUM
#endif

#if !_LIBCPP_ABI_FORCE_MICROSOFT
#  undef _LIBCPP_ABI_FORCE_MICROSOFT
#endif

#if !_LIBCPP_HAS_THREADS
#  define _LIBCPP_HAS_NO_THREADS
#endif

#if !_LIBCPP_HAS_MONOTONIC_CLOCK
#  define _LIBCPP_HAS_NO_MONOTONIC_CLOCK
#endif

#if !_LIBCPP_HAS_MUSL_LIBC
#  undef _LIBCPP_HAS_MUSL_LIBC
#endif

#if !_LIBCPP_HAS_THREAD_API_PTHREAD
#  undef _LIBCPP_HAS_THREAD_API_PTHREAD
#endif

#if !_LIBCPP_HAS_THREAD_API_EXTERNAL
#  undef _LIBCPP_HAS_THREAD_API_EXTERNAL
#endif

#if !_LIBCPP_HAS_THREAD_API_WIN32
#  undef _LIBCPP_HAS_THREAD_API_WIN32
#endif

#undef _LIBCPP_HAS_THREAD_API_C11

#if !_LIBCPP_HAS_VENDOR_AVAILABILITY_ANNOTATIONS
#  define _LIBCPP_HAS_NO_VENDOR_AVAILABILITY_ANNOTATIONS
#endif

#if !_LIBCPP_HAS_FILESYSTEM
#  define _LIBCPP_HAS_NO_FILESYSTEM
#endif

#if !_LIBCPP_HAS_RANDOM_DEVICE
#  define _LIBCPP_HAS_NO_RANDOM_DEVICE
#endif

#if !_LIBCPP_HAS_LOCALIZATION
#  define _LIBCPP_HAS_NO_LOCALIZATION
#endif

#if !_LIBCPP_HAS_UNICODE
#  define _LIBCPP_HAS_NO_UNICODE
#endif

#if !_LIBCPP_HAS_WIDE_CHARACTERS
#  define _LIBCPP_HAS_NO_WIDE_CHARACTERS
#endif

#if !_LIBCPP_HAS_TIME_ZONE_DATABASE
#  define _LIBCPP_HAS_NO_TIME_ZONE_DATABASE
#endif

#if !_LIBCPP_INSTRUMENTED_WITH_ASAN
#  undef _LIBCPP_INSTRUMENTED_WITH_ASAN
#endif

#endif // _LIBCPP___CXX03___CONFIGURATION_CONFIG_SITE_SHIM_H
