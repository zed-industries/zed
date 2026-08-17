//===-----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___LOCALE_DIR_SUPPORT_NO_LOCALE_STRTONUM_H
#define _LIBCPP___LOCALE_DIR_SUPPORT_NO_LOCALE_STRTONUM_H

#include <__config>
#include <cstdlib>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD
namespace __locale {

//
// Strtonum functions
//
inline _LIBCPP_HIDE_FROM_ABI float __strtof(const char* __nptr, char** __endptr, __locale_t) {
  return std::strtof(__nptr, __endptr);
}

inline _LIBCPP_HIDE_FROM_ABI double __strtod(const char* __nptr, char** __endptr, __locale_t) {
  return std::strtod(__nptr, __endptr);
}

inline _LIBCPP_HIDE_FROM_ABI long double __strtold(const char* __nptr, char** __endptr, __locale_t) {
  return std::strtold(__nptr, __endptr);
}

inline _LIBCPP_HIDE_FROM_ABI long long __strtoll(const char* __nptr, char** __endptr, int __base, __locale_t) {
  return std::strtoll(__nptr, __endptr, __base);
}

inline _LIBCPP_HIDE_FROM_ABI unsigned long long
__strtoull(const char* __nptr, char** __endptr, int __base, __locale_t) {
  return std::strtoull(__nptr, __endptr, __base);
}

} // namespace __locale
_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___LOCALE_DIR_SUPPORT_NO_LOCALE_STRTONUM_H
