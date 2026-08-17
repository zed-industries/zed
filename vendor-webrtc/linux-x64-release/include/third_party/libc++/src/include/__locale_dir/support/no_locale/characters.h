//===-----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___LOCALE_DIR_SUPPORT_NO_LOCALE_CHARACTERS_H
#define _LIBCPP___LOCALE_DIR_SUPPORT_NO_LOCALE_CHARACTERS_H

#include <__config>
#include <__cstddef/size_t.h>
#include <cctype>
#include <cstdlib>
#include <cstring>
#include <ctime>
#if _LIBCPP_HAS_WIDE_CHARACTERS
#  include <cwctype>
#endif

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD
namespace __locale {

//
// Character manipulation functions
//
#if defined(_LIBCPP_BUILDING_LIBRARY)
inline _LIBCPP_HIDE_FROM_ABI int __islower(int __c, __locale_t) { return std::islower(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __isupper(int __c, __locale_t) { return std::isupper(__c); }
#endif

inline _LIBCPP_HIDE_FROM_ABI int __isdigit(int __c, __locale_t) { return std::isdigit(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __isxdigit(int __c, __locale_t) { return std::isxdigit(__c); }

#if defined(_LIBCPP_BUILDING_LIBRARY)
inline _LIBCPP_HIDE_FROM_ABI int __toupper(int __c, __locale_t) { return std::toupper(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __tolower(int __c, __locale_t) { return std::tolower(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __strcoll(const char* __s1, const char* __s2, __locale_t) {
  return std::strcoll(__s1, __s2);
}

inline _LIBCPP_HIDE_FROM_ABI size_t __strxfrm(char* __dest, const char* __src, size_t __n, __locale_t) {
  return std::strxfrm(__dest, __src, __n);
}

#  if _LIBCPP_HAS_WIDE_CHARACTERS
inline _LIBCPP_HIDE_FROM_ABI int __iswctype(wint_t __c, wctype_t __type, __locale_t) {
  return std::iswctype(__c, __type);
}

inline _LIBCPP_HIDE_FROM_ABI int __iswspace(wint_t __c, __locale_t) { return std::iswspace(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __iswprint(wint_t __c, __locale_t) { return std::iswprint(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __iswcntrl(wint_t __c, __locale_t) { return std::iswcntrl(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __iswupper(wint_t __c, __locale_t) { return std::iswupper(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __iswlower(wint_t __c, __locale_t) { return std::iswlower(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __iswalpha(wint_t __c, __locale_t) { return std::iswalpha(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __iswblank(wint_t __c, __locale_t) { return std::iswblank(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __iswdigit(wint_t __c, __locale_t) { return std::iswdigit(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __iswpunct(wint_t __c, __locale_t) { return std::iswpunct(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __iswxdigit(wint_t __c, __locale_t) { return std::iswxdigit(__c); }

inline _LIBCPP_HIDE_FROM_ABI wint_t __towupper(wint_t __c, __locale_t) { return std::towupper(__c); }

inline _LIBCPP_HIDE_FROM_ABI wint_t __towlower(wint_t __c, __locale_t) { return std::towlower(__c); }

inline _LIBCPP_HIDE_FROM_ABI int __wcscoll(const wchar_t* __ws1, const wchar_t* __ws2, __locale_t) {
  return std::wcscoll(__ws1, __ws2);
}

inline _LIBCPP_HIDE_FROM_ABI size_t __wcsxfrm(wchar_t* __dest, const wchar_t* __src, size_t __n, __locale_t) {
  return std::wcsxfrm(__dest, __src, __n);
}
#  endif // _LIBCPP_HAS_WIDE_CHARACTERS

inline _LIBCPP_HIDE_FROM_ABI size_t
__strftime(char* __s, size_t __max, const char* __format, const struct tm* __tm, __locale_t) {
  return std::strftime(__s, __max, __format, __tm);
}
#endif // _LIBCPP_BUILDING_LIBRARY

} // namespace __locale
_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___LOCALE_DIR_SUPPORT_NO_LOCALE_CHARACTERS_H
