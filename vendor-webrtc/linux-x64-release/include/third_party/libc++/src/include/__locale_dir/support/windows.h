//===-----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___LOCALE_DIR_SUPPORT_WINDOWS_H
#define _LIBCPP___LOCALE_DIR_SUPPORT_WINDOWS_H

#include <__config>
#include <__cstddef/nullptr_t.h>
#include <__utility/forward.h>
#include <clocale> // std::lconv & friends
#include <cstddef>
#include <ctype.h>  // ::_isupper_l & friends
#include <locale.h> // ::_locale_t
#include <stdio.h>  // ::_sscanf_l
#include <stdlib.h> // ::_strtod_l & friends
#include <string.h> // ::_strcoll_l
#include <string>
#include <time.h> // ::_strftime_l

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_BEGIN_NAMESPACE_STD
namespace __locale {

using __lconv_t _LIBCPP_NODEBUG = std::lconv;

class __lconv_storage {
public:
  __lconv_storage(const __lconv_t* __lc_input) {
    __lc_ = *__lc_input;

    __decimal_point_     = __lc_input->decimal_point;
    __thousands_sep_     = __lc_input->thousands_sep;
    __grouping_          = __lc_input->grouping;
    __int_curr_symbol_   = __lc_input->int_curr_symbol;
    __currency_symbol_   = __lc_input->currency_symbol;
    __mon_decimal_point_ = __lc_input->mon_decimal_point;
    __mon_thousands_sep_ = __lc_input->mon_thousands_sep;
    __mon_grouping_      = __lc_input->mon_grouping;
    __positive_sign_     = __lc_input->positive_sign;
    __negative_sign_     = __lc_input->negative_sign;

    __lc_.decimal_point     = const_cast<char*>(__decimal_point_.c_str());
    __lc_.thousands_sep     = const_cast<char*>(__thousands_sep_.c_str());
    __lc_.grouping          = const_cast<char*>(__grouping_.c_str());
    __lc_.int_curr_symbol   = const_cast<char*>(__int_curr_symbol_.c_str());
    __lc_.currency_symbol   = const_cast<char*>(__currency_symbol_.c_str());
    __lc_.mon_decimal_point = const_cast<char*>(__mon_decimal_point_.c_str());
    __lc_.mon_thousands_sep = const_cast<char*>(__mon_thousands_sep_.c_str());
    __lc_.mon_grouping      = const_cast<char*>(__mon_grouping_.c_str());
    __lc_.positive_sign     = const_cast<char*>(__positive_sign_.c_str());
    __lc_.negative_sign     = const_cast<char*>(__negative_sign_.c_str());
  }

  __lconv_t* __get() { return &__lc_; }

private:
  __lconv_t __lc_;
  std::string __decimal_point_;
  std::string __thousands_sep_;
  std::string __grouping_;
  std::string __int_curr_symbol_;
  std::string __currency_symbol_;
  std::string __mon_decimal_point_;
  std::string __mon_thousands_sep_;
  std::string __mon_grouping_;
  std::string __positive_sign_;
  std::string __negative_sign_;
};

//
// Locale management
//
#define _CATMASK(n) ((1 << (n)) >> 1)
#define _LIBCPP_COLLATE_MASK _CATMASK(LC_COLLATE)
#define _LIBCPP_CTYPE_MASK _CATMASK(LC_CTYPE)
#define _LIBCPP_MONETARY_MASK _CATMASK(LC_MONETARY)
#define _LIBCPP_NUMERIC_MASK _CATMASK(LC_NUMERIC)
#define _LIBCPP_TIME_MASK _CATMASK(LC_TIME)
#define _LIBCPP_MESSAGES_MASK _CATMASK(6)
#define _LIBCPP_ALL_MASK                                                                                               \
  (_LIBCPP_COLLATE_MASK | _LIBCPP_CTYPE_MASK | _LIBCPP_MESSAGES_MASK | _LIBCPP_MONETARY_MASK | _LIBCPP_NUMERIC_MASK |  \
   _LIBCPP_TIME_MASK)
#define _LIBCPP_LC_ALL LC_ALL

class __locale_t {
public:
  __locale_t() : __locale_(nullptr), __locale_str_(nullptr), __lc_(nullptr) {}
  __locale_t(std::nullptr_t) : __locale_(nullptr), __locale_str_(nullptr), __lc_(nullptr) {}
  __locale_t(::_locale_t __loc, const char* __loc_str) : __locale_(__loc), __locale_str_(__loc_str), __lc_(nullptr) {}
  __locale_t(const __locale_t& __loc)
      : __locale_(__loc.__locale_), __locale_str_(__loc.__locale_str_), __lc_(nullptr) {}

  ~__locale_t() { delete __lc_; }

  __locale_t& operator=(const __locale_t& __loc) {
    __locale_     = __loc.__locale_;
    __locale_str_ = __loc.__locale_str_;
    // __lc_ not copied
    return *this;
  }

  friend bool operator==(const __locale_t& __left, const __locale_t& __right) {
    return __left.__locale_ == __right.__locale_;
  }

  friend bool operator==(const __locale_t& __left, int __right) { return __left.__locale_ == nullptr && __right == 0; }

  friend bool operator==(const __locale_t& __left, long long __right) {
    return __left.__locale_ == nullptr && __right == 0;
  }

  friend bool operator==(const __locale_t& __left, std::nullptr_t) { return __left.__locale_ == nullptr; }

  friend bool operator==(int __left, const __locale_t& __right) { return __left == 0 && nullptr == __right.__locale_; }

  friend bool operator==(std::nullptr_t, const __locale_t& __right) { return nullptr == __right.__locale_; }

  friend bool operator!=(const __locale_t& __left, const __locale_t& __right) { return !(__left == __right); }

  friend bool operator!=(const __locale_t& __left, int __right) { return !(__left == __right); }

  friend bool operator!=(const __locale_t& __left, long long __right) { return !(__left == __right); }

  friend bool operator!=(const __locale_t& __left, std::nullptr_t __right) { return !(__left == __right); }

  friend bool operator!=(int __left, const __locale_t& __right) { return !(__left == __right); }

  friend bool operator!=(std::nullptr_t __left, const __locale_t& __right) { return !(__left == __right); }

  operator bool() const { return __locale_ != nullptr; }

  const char* __get_locale() const { return __locale_str_; }

  operator ::_locale_t() const { return __locale_; }

  __lconv_t* __store_lconv(const __lconv_t* __input_lc) {
    delete __lc_;
    __lc_ = new __lconv_storage(__input_lc);
    return __lc_->__get();
  }

private:
  ::_locale_t __locale_;
  const char* __locale_str_;
  __lconv_storage* __lc_ = nullptr;
};

#if defined(_LIBCPP_BUILDING_LIBRARY)
_LIBCPP_EXPORTED_FROM_ABI __locale_t __newlocale(int __mask, const char* __locale, __locale_t __base);
inline _LIBCPP_HIDE_FROM_ABI void __freelocale(__locale_t __loc) { ::_free_locale(__loc); }
inline _LIBCPP_HIDE_FROM_ABI char* __setlocale(int __category, const char* __locale) {
  char* __new_locale = ::setlocale(__category, __locale);
  if (__new_locale == nullptr)
    std::__throw_bad_alloc();
  return __new_locale;
}
_LIBCPP_EXPORTED_FROM_ABI __lconv_t* __localeconv(__locale_t& __loc);
#endif // _LIBCPP_BUILDING_LIBRARY

//
// Strtonum functions
//

// the *_l functions are prefixed on Windows, only available for msvcr80+, VS2005+
#if defined(_LIBCPP_MSVCRT)
inline _LIBCPP_HIDE_FROM_ABI float __strtof(const char* __nptr, char** __endptr, __locale_t __loc) {
  return ::_strtof_l(__nptr, __endptr, __loc);
}
inline _LIBCPP_HIDE_FROM_ABI long double __strtold(const char* __nptr, char** __endptr, __locale_t __loc) {
  return ::_strtold_l(__nptr, __endptr, __loc);
}
#else
_LIBCPP_EXPORTED_FROM_ABI float __strtof(const char*, char**, __locale_t);
_LIBCPP_EXPORTED_FROM_ABI long double __strtold(const char*, char**, __locale_t);
#endif

inline _LIBCPP_HIDE_FROM_ABI double __strtod(const char* __nptr, char** __endptr, __locale_t __loc) {
  return ::_strtod_l(__nptr, __endptr, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI long long __strtoll(const char* __nptr, char** __endptr, int __base, __locale_t __loc) {
  return ::_strtoi64_l(__nptr, __endptr, __base, __loc);
}
inline _LIBCPP_HIDE_FROM_ABI unsigned long long
__strtoull(const char* __nptr, char** __endptr, int __base, __locale_t __loc) {
  return ::_strtoui64_l(__nptr, __endptr, __base, __loc);
}

//
// Character manipulation functions
//
#if defined(_LIBCPP_BUILDING_LIBRARY)
inline _LIBCPP_HIDE_FROM_ABI int __islower(int __c, __locale_t __loc) { return _islower_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __isupper(int __c, __locale_t __loc) { return _isupper_l(__c, __loc); }
#endif

inline _LIBCPP_HIDE_FROM_ABI int __isdigit(int __c, __locale_t __loc) { return _isdigit_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __isxdigit(int __c, __locale_t __loc) { return _isxdigit_l(__c, __loc); }

#if defined(_LIBCPP_BUILDING_LIBRARY)
inline _LIBCPP_HIDE_FROM_ABI int __toupper(int __c, __locale_t __loc) { return ::_toupper_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __tolower(int __c, __locale_t __loc) { return ::_tolower_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __strcoll(const char* __s1, const char* __s2, __locale_t __loc) {
  return ::_strcoll_l(__s1, __s2, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI size_t __strxfrm(char* __dest, const char* __src, size_t __n, __locale_t __loc) {
  return ::_strxfrm_l(__dest, __src, __n, __loc);
}

#  if _LIBCPP_HAS_WIDE_CHARACTERS
inline _LIBCPP_HIDE_FROM_ABI int __iswctype(wint_t __c, wctype_t __type, __locale_t __loc) {
  return ::_iswctype_l(__c, __type, __loc);
}
inline _LIBCPP_HIDE_FROM_ABI int __iswspace(wint_t __c, __locale_t __loc) { return ::_iswspace_l(__c, __loc); }
inline _LIBCPP_HIDE_FROM_ABI int __iswprint(wint_t __c, __locale_t __loc) { return ::_iswprint_l(__c, __loc); }
inline _LIBCPP_HIDE_FROM_ABI int __iswcntrl(wint_t __c, __locale_t __loc) { return ::_iswcntrl_l(__c, __loc); }
inline _LIBCPP_HIDE_FROM_ABI int __iswupper(wint_t __c, __locale_t __loc) { return ::_iswupper_l(__c, __loc); }
inline _LIBCPP_HIDE_FROM_ABI int __iswlower(wint_t __c, __locale_t __loc) { return ::_iswlower_l(__c, __loc); }
inline _LIBCPP_HIDE_FROM_ABI int __iswalpha(wint_t __c, __locale_t __loc) { return ::_iswalpha_l(__c, __loc); }
// TODO: use locale to determine blank characters
inline _LIBCPP_HIDE_FROM_ABI int __iswblank(wint_t __c, __locale_t /*loc*/) { return (__c == L' ' || __c == L'\t'); }
inline _LIBCPP_HIDE_FROM_ABI int __iswdigit(wint_t __c, __locale_t __loc) { return ::_iswdigit_l(__c, __loc); }
inline _LIBCPP_HIDE_FROM_ABI int __iswpunct(wint_t __c, __locale_t __loc) { return ::_iswpunct_l(__c, __loc); }
inline _LIBCPP_HIDE_FROM_ABI int __iswxdigit(wint_t __c, __locale_t __loc) { return ::_iswxdigit_l(__c, __loc); }
inline _LIBCPP_HIDE_FROM_ABI wint_t __towupper(wint_t __c, __locale_t __loc) { return ::_towupper_l(__c, __loc); }
inline _LIBCPP_HIDE_FROM_ABI wint_t __towlower(wint_t __c, __locale_t __loc) { return ::_towlower_l(__c, __loc); }

inline _LIBCPP_HIDE_FROM_ABI int __wcscoll(const wchar_t* __ws1, const wchar_t* __ws2, __locale_t __loc) {
  return ::_wcscoll_l(__ws1, __ws2, __loc);
}

inline _LIBCPP_HIDE_FROM_ABI size_t __wcsxfrm(wchar_t* __dest, const wchar_t* __src, size_t __n, __locale_t __loc) {
  return ::_wcsxfrm_l(__dest, __src, __n, __loc);
}
#  endif // _LIBCPP_HAS_WIDE_CHARACTERS

#  if defined(__MINGW32__) && __MSVCRT_VERSION__ < 0x0800
_LIBCPP_EXPORTED_FROM_ABI size_t __strftime(char*, size_t, const char*, const struct tm*, __locale_t);
#  else
inline _LIBCPP_HIDE_FROM_ABI size_t
__strftime(char* __ret, size_t __n, const char* __format, const struct tm* __tm, __locale_t __loc) {
  return ::_strftime_l(__ret, __n, __format, __tm, __loc);
}
#  endif

//
// Other functions
//
_LIBCPP_EXPORTED_FROM_ABI decltype(MB_CUR_MAX) __mb_len_max(__locale_t);
_LIBCPP_EXPORTED_FROM_ABI wint_t __btowc(int, __locale_t);
_LIBCPP_EXPORTED_FROM_ABI int __wctob(wint_t, __locale_t);
_LIBCPP_EXPORTED_FROM_ABI size_t
__wcsnrtombs(char* __restrict, const wchar_t** __restrict, size_t, size_t, mbstate_t* __restrict, __locale_t);
_LIBCPP_EXPORTED_FROM_ABI size_t __wcrtomb(char* __restrict, wchar_t, mbstate_t* __restrict, __locale_t);
_LIBCPP_EXPORTED_FROM_ABI size_t
__mbsnrtowcs(wchar_t* __restrict, const char** __restrict, size_t, size_t, mbstate_t* __restrict, __locale_t);
_LIBCPP_EXPORTED_FROM_ABI size_t
__mbrtowc(wchar_t* __restrict, const char* __restrict, size_t, mbstate_t* __restrict, __locale_t);

inline _LIBCPP_HIDE_FROM_ABI int __mbtowc(wchar_t* __pwc, const char* __pmb, size_t __max, __locale_t __loc) {
  return ::_mbtowc_l(__pwc, __pmb, __max, __loc);
}

_LIBCPP_EXPORTED_FROM_ABI size_t __mbrlen(const char* __restrict, size_t, mbstate_t* __restrict, __locale_t);

_LIBCPP_EXPORTED_FROM_ABI size_t
__mbsrtowcs(wchar_t* __restrict, const char** __restrict, size_t, mbstate_t* __restrict, __locale_t);
#endif // _LIBCPP_BUILDING_LIBRARY

_LIBCPP_EXPORTED_FROM_ABI _LIBCPP_ATTRIBUTE_FORMAT(__printf__, 4, 5) int __snprintf(
    char* __ret, size_t __n, __locale_t __loc, const char* __format, ...);

_LIBCPP_EXPORTED_FROM_ABI
_LIBCPP_ATTRIBUTE_FORMAT(__printf__, 3, 4) int __asprintf(char** __ret, __locale_t __loc, const char* __format, ...);

_LIBCPP_DIAGNOSTIC_PUSH
_LIBCPP_CLANG_DIAGNOSTIC_IGNORED("-Wgcc-compat")
_LIBCPP_GCC_DIAGNOSTIC_IGNORED("-Wformat-nonliteral") // GCC doesn't support [[gnu::format]] on variadic templates
#ifdef _LIBCPP_COMPILER_CLANG_BASED
#  define _LIBCPP_VARIADIC_ATTRIBUTE_FORMAT(...) _LIBCPP_ATTRIBUTE_FORMAT(__VA_ARGS__)
#else
#  define _LIBCPP_VARIADIC_ATTRIBUTE_FORMAT(...) /* nothing */
#endif

template <class... _Args>
_LIBCPP_HIDE_FROM_ABI _LIBCPP_VARIADIC_ATTRIBUTE_FORMAT(__scanf__, 3, 4) int __sscanf(
    const char* __dest, __locale_t __loc, const char* __format, _Args&&... __args) {
  return ::_sscanf_l(__dest, __format, __loc, std::forward<_Args>(__args)...);
}
_LIBCPP_DIAGNOSTIC_POP
#undef _LIBCPP_VARIADIC_ATTRIBUTE_FORMAT

#if defined(_LIBCPP_BUILDING_LIBRARY)
struct __locale_guard {
  _LIBCPP_HIDE_FROM_ABI __locale_guard(__locale_t __l) : __status(_configthreadlocale(_ENABLE_PER_THREAD_LOCALE)) {
    // Setting the locale can be expensive even when the locale given is
    // already the current locale, so do an explicit check to see if the
    // current locale is already the one we want.
    const char* __lc = __locale::__setlocale(LC_ALL, nullptr);
    // If every category is the same, the locale string will simply be the
    // locale name, otherwise it will be a semicolon-separated string listing
    // each category.  In the second case, we know at least one category won't
    // be what we want, so we only have to check the first case.
    if (std::strcmp(__l.__get_locale(), __lc) != 0) {
      __locale_all = _strdup(__lc);
      if (__locale_all == nullptr)
        std::__throw_bad_alloc();
      __locale::__setlocale(LC_ALL, __l.__get_locale());
    }
  }
  _LIBCPP_HIDE_FROM_ABI ~__locale_guard() {
    // The CRT documentation doesn't explicitly say, but setlocale() does the
    // right thing when given a semicolon-separated list of locale settings
    // for the different categories in the same format as returned by
    // setlocale(LC_ALL, nullptr).
    if (__locale_all != nullptr) {
      __locale::__setlocale(LC_ALL, __locale_all);
      free(__locale_all);
    }
    _configthreadlocale(__status);
  }
  int __status;
  char* __locale_all = nullptr;
};
#endif // _LIBCPP_BUILDING_LIBRARY

} // namespace __locale
_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP___LOCALE_DIR_SUPPORT_WINDOWS_H
