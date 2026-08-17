//===-----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___LOCALE_DIR_SUPPORT_BSD_LIKE_H
#define _LIBCPP___LOCALE_DIR_SUPPORT_BSD_LIKE_H

#include <__config>
#include <__cstddef/size_t.h>
#include <__std_mbstate_t.h>
#include <__utility/forward.h>
#include <clocale> // std::lconv
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#if _LIBCPP_HAS_WIDE_CHARACTERS
#  include <wchar.h>
#  include <wctype.h>
#endif

#include <xlocale.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD
namespace __locale {

//
// Locale management
//
#define _LIBCPP_COLLATE_MASK LC_COLLATE_MASK
#define _LIBCPP_CTYPE_MASK LC_CTYPE_MASK
#define _LIBCPP_MONETARY_MASK LC_MONETARY_MASK
#define _LIBCPP_NUMERIC_MASK LC_NUMERIC_MASK
#define _LIBCPP_TIME_MASK LC_TIME_MASK
#define _LIBCPP_MESSAGES_MASK LC_MESSAGES_MASK
#define _LIBCPP_ALL_MASK LC_ALL_MASK
#define _LIBCPP_LC_ALL LC_ALL

using __locale_t _LIBCPP_NODEBUG = ::locale_t;
#if defined(_LIBCPP_BUILDING_LIBRARY)
using __lconv_t _LIBCPP_NODEBUG = std::lconv;

inline _LIBCPP_HIDE_FROM_ABI __locale_t __newlocale(int __category_mask, const char* __locale, __locale_t __base) {
  return ::newlocale(__category_mask, __locale, __base);
}

inline _LIBCPP_HIDE_FROM_ABI void __freelocale(__locale_t __loc) { ::freelocale(__loc); }

inline _LIBCPP_HIDE_FROM_ABI char* __setlocale(int __category, char const* __locale) {
  return ::setlocale(__category, __locale);
}

inline _LIBCPP_HIDE_FROM_ABI __lconv_t* __localeconv(__locale_t& __loc) { return ::localeconv_l(__loc); }
#endif // _LIBCPP_BUILDING_LIBRARY

//
// Strtonum functions
//
inline _LIBCPP_HIDE_FROM_ABI float __strtof(const char* __nptr, char** __endptr, __locale_t __loc) {
  return ::strtof_l(__nptr, __endptr, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI double __strtod(const char* __nptr, char** __endptr, __locale_t __loc) {
  return ::strtod_l(__nptr, __endptr, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI long double __strtold(const char* __nptr, char** __endptr, __locale_t __loc) {
  return ::strtold_l(__nptr, __endptr, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI long long __strtoll(const char* __nptr, char** __endptr, int __base, __locale_t __loc) {
  return ::strtoll_l(__nptr, __endptr, __base, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI unsigned long long
__strtoull(const char* __nptr, char** __endptr, int __base, __locale_t __loc) {
  return ::strtoull_l(__nptr, __endptr, __base, __loc);
}

//
// Character manipulation functions
//
#if defined(_LIBCPP_BUILDING_LIBRARY)
inline _LIBCPP_HIDE_FROM_ABI int __islower(int __c, __locale_t __loc) { return ::islower_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __isupper(int __c, __locale_t __loc) { return ::isupper_l(__c, __loc); }
#endif

inline _LIBCPP_HIDE_FROM_ABI int __isdigit(int __c, __locale_t __loc) { return ::isdigit_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __isxdigit(int __c, __locale_t __loc) { return ::isxdigit_l(__c, __loc); }

#if defined(_LIBCPP_BUILDING_LIBRARY)
inline _LIBCPP_HIDE_FROM_ABI int __toupper(int __c, __locale_t __loc) { return ::toupper_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __tolower(int __c, __locale_t __loc) { return ::tolower_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __strcoll(const char* __s1, const char* __s2, __locale_t __loc) {
  return ::strcoll_l(__s1, __s2, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI size_t __strxfrm(char* __dest, const char* __src, size_t __n, __locale_t __loc) {
  return ::strxfrm_l(__dest, __src, __n, __loc);
}

#  if _LIBCPP_HAS_WIDE_CHARACTERS
inline _LIBCPP_HIDE_FROM_ABI int __iswctype(wint_t __c, wctype_t __type, __locale_t __loc) {
  return ::iswctype_l(__c, __type, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI int __iswspace(wint_t __c, __locale_t __loc) { return ::iswspace_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __iswprint(wint_t __c, __locale_t __loc) { return ::iswprint_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __iswcntrl(wint_t __c, __locale_t __loc) { return ::iswcntrl_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __iswupper(wint_t __c, __locale_t __loc) { return ::iswupper_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __iswlower(wint_t __c, __locale_t __loc) { return ::iswlower_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __iswalpha(wint_t __c, __locale_t __loc) { return ::iswalpha_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __iswblank(wint_t __c, __locale_t __loc) { return ::iswblank_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __iswdigit(wint_t __c, __locale_t __loc) { return ::iswdigit_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __iswpunct(wint_t __c, __locale_t __loc) { return ::iswpunct_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __iswxdigit(wint_t __c, __locale_t __loc) { return ::iswxdigit_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI wint_t __towupper(wint_t __c, __locale_t __loc) { return ::towupper_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI wint_t __towlower(wint_t __c, __locale_t __loc) { return ::towlower_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __wcscoll(const wchar_t* __ws1, const wchar_t* __ws2, __locale_t __loc) {
  return ::wcscoll_l(__ws1, __ws2, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI size_t __wcsxfrm(wchar_t* __dest, const wchar_t* __src, size_t __n, __locale_t __loc) {
  return ::wcsxfrm_l(__dest, __src, __n, __loc);
}
#  endif // _LIBCPP_HAS_WIDE_CHARACTERS

inline _LIBCPP_HIDE_FROM_ABI size_t
__strftime(char* __s, size_t __max, const char* __format, const struct tm* __tm, __locale_t __loc) {
  return ::strftime_l(__s, __max, __format, __tm, __loc);
}

//
// Other functions
//
inline _LIBCPP_HIDE_FROM_ABI decltype(MB_CUR_MAX) __mb_len_max(__locale_t __loc) { return MB_CUR_MAX_L(__loc); }

#  if _LIBCPP_HAS_WIDE_CHARACTERS
inline _LIBCPP_HIDE_FROM_ABI wint_t __btowc(int __c, __locale_t __loc) { return ::btowc_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __wctob(wint_t __c, __locale_t __loc) { return ::wctob_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI size_t
__wcsnrtombs(char* __dest, const wchar_t** __src, size_t __nwc, size_t __len, mbstate_t* __ps, __locale_t __loc) {
  return ::wcsnrtombs_l(__dest, __src, __nwc, __len, __ps, __loc); // wcsnrtombs is a POSIX extension
}

inline _LIBCPP_HIDE_FROM_ABI size_t __wcrtomb(char* __s, wchar_t __wc, mbstate_t* __ps, __locale_t __loc) {
  return ::wcrtomb_l(__s, __wc, __ps, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI size_t
__mbsnrtowcs(wchar_t* __dest, const char** __src, size_t __nms, size_t __len, mbstate_t* __ps, __locale_t __loc) {
  return ::mbsnrtowcs_l(__dest, __src, __nms, __len, __ps, __loc); // mbsnrtowcs is a POSIX extension
}

inline _LIBCPP_HIDE_FROM_ABI size_t
__mbrtowc(wchar_t* __pwc, const char* __s, size_t __n, mbstate_t* __ps, __locale_t __loc) {
  return ::mbrtowc_l(__pwc, __s, __n, __ps, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI int __mbtowc(wchar_t* __pwc, const char* __pmb, size_t __max, __locale_t __loc) {
  return ::mbtowc_l(__pwc, __pmb, __max, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI size_t __mbrlen(const char* __s, size_t __n, mbstate_t* __ps, __locale_t __loc) {
  return ::mbrlen_l(__s, __n, __ps, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI size_t
__mbsrtowcs(wchar_t* __dest, const char** __src, size_t __len, mbstate_t* __ps, __locale_t __loc) {
  return ::mbsrtowcs_l(__dest, __src, __len, __ps, __loc);
}
#  endif // _LIBCPP_HAS_WIDE_CHARACTERS
#endif   // _LIBCPP_BUILDING_LIBRARY

_LIBCPP_DIAGNOSTIC_PUSH
_LIBCPP_CLANG_DIAGNOSTIC_IGNORED("-Wgcc-compat")
_LIBCPP_GCC_DIAGNOSTIC_IGNORED("-Wformat-nonliteral") // GCC doesn't support [[gnu::format]] on variadic templates
#ifdef _LIBCPP_COMPILER_CLANG_BASED
#  define _LIBCPP_VARIADIC_ATTRIBUTE_FORMAT(...) _LIBCPP_ATTRIBUTE_FORMAT(__VA_ARGS__)
#else
#  define _LIBCPP_VARIADIC_ATTRIBUTE_FORMAT(...) /* nothing */
#endif

template <class... _Args>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_VARIADIC_ATTRIBUTE_FORMAT(__printf__, 4, 5) int __snprintf(
    char* __s, size_t __n, __locale_t __loc, const char* __format, _Args&&... __args) {
  return ::snprintf_l(__s, __n, __loc, __format, std::forward<_Args>(__args)...);
}

template <class... _Args>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_VARIADIC_ATTRIBUTE_FORMAT(__printf__, 3, 4) int __asprintf(
    char** __s, __locale_t __loc, const char* __format, _Args&&... __args) {
  return ::asprintf_l(__s, __loc, __format, std::forward<_Args>(__args)...); // non-standard
}

template <class... _Args>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_VARIADIC_ATTRIBUTE_FORMAT(__scanf__, 3, 4) int __sscanf(
    const char* __s, __locale_t __loc, const char* __format, _Args&&... __args) {
  return ::sscanf_l(__s, __loc, __format, std::forward<_Args>(__args)...);
}
_LIBCPP_DIAGNOSTIC_POP
#undef _LIBCPP_VARIADIC_ATTRIBUTE_FORMAT

} // namespace __locale
_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___LOCALE_DIR_SUPPORT_BSD_LIKE_H
