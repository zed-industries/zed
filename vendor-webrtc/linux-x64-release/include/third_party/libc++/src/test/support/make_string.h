//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_TEST_MAKE_STRING_H
#define SUPPORT_TEST_MAKE_STRING_H

#include "test_macros.h"

#include <string>
#include <string_view>

#ifndef TEST_HAS_NO_WIDE_CHARACTERS
# define MKSTR_WCHAR_ONLY(...) __VA_ARGS__
# define MKSTR_AS_WCHAR_LITERAL(x) TEST_CONCAT(L, x), sizeof(TEST_CONCAT(L, x)) / sizeof(wchar_t) - 1
#else
# define MKSTR_WCHAR_ONLY(...)
#endif

#if TEST_STD_VER > 17 && defined(__cpp_char8_t)
#define MKSTR_CHAR8_ONLY(...) __VA_ARGS__
#define MKSTR_AS_U8_LITERAL(x) TEST_CONCAT(u8, x), sizeof(TEST_CONCAT(u8, x)) / sizeof(char8_t) - 1
#else
#define MKSTR_CHAR8_ONLY(...)
#endif

#if TEST_STD_VER >= 11
#define MKSTR_CXX11_ONLY(...) __VA_ARGS__
#define MKSTR_AS_U16_LITERAL(x) TEST_CONCAT(u, x), sizeof(TEST_CONCAT(u, x)) / sizeof(char16_t) - 1
#define MKSTR_AS_U32_LITERAL(x) TEST_CONCAT(U, x), sizeof(TEST_CONCAT(U, x)) / sizeof(char32_t) - 1
#else
#define MKSTR_CXX11_ONLY(...)
#endif

#define MKSTR(Str) MultiStringType(                \
    MKSTR_WCHAR_ONLY(MKSTR_AS_WCHAR_LITERAL(Str),) \
    MKSTR_CHAR8_ONLY(MKSTR_AS_U8_LITERAL(Str),)    \
    MKSTR_CXX11_ONLY(MKSTR_AS_U16_LITERAL(Str),    \
                     MKSTR_AS_U32_LITERAL(Str),)   \
    Str, sizeof(Str) - 1                           \
  )

#define MKSTR_LEN(CharT, Str) MKSTR(Str).length((const CharT*)0)

struct MultiStringType {
  MKSTR_WCHAR_ONLY(const wchar_t* w_; std::size_t wn_; )
  MKSTR_CHAR8_ONLY(const char8_t* u8_; std::size_t u8n_; )
  MKSTR_CXX11_ONLY(const char16_t* u16_; std::size_t u16n_; )
  MKSTR_CXX11_ONLY(const char32_t* u32_; std::size_t u32n_; )
  const char* s_; std::size_t sn_;

  TEST_CONSTEXPR MultiStringType(
      MKSTR_WCHAR_ONLY(const wchar_t *w, std::size_t wn,)
      MKSTR_CHAR8_ONLY(const char8_t *u8, std::size_t u8n,)
      MKSTR_CXX11_ONLY(const char16_t *u16, std::size_t u16n,)
      MKSTR_CXX11_ONLY(const char32_t *u32, std::size_t u32n,)
      const char *s, std::size_t sn)
    : MKSTR_WCHAR_ONLY(w_(w), wn_(wn),)
      MKSTR_CHAR8_ONLY(u8_(u8), u8n_(u8n),)
      MKSTR_CXX11_ONLY(u16_(u16), u16n_(u16n),)
      MKSTR_CXX11_ONLY(u32_(u32), u32n_(u32n),)
      s_(s), sn_(sn) {}

  TEST_CONSTEXPR const char *as_ptr(const char*) const { return s_; }
  MKSTR_WCHAR_ONLY(TEST_CONSTEXPR const wchar_t *as_ptr(const wchar_t*) const { return w_; })
  MKSTR_CHAR8_ONLY(constexpr const char8_t *as_ptr(const char8_t*) const { return u8_; })
  MKSTR_CXX11_ONLY(constexpr const char16_t *as_ptr(const char16_t*) const { return u16_; })
  MKSTR_CXX11_ONLY(constexpr const char32_t *as_ptr(const char32_t*) const { return u32_; })

  TEST_CONSTEXPR std::size_t length(const char*) const { return sn_; }
  MKSTR_WCHAR_ONLY(TEST_CONSTEXPR std::size_t length(const wchar_t*) const { return wn_; })
  MKSTR_CHAR8_ONLY(constexpr std::size_t length(const char8_t*) const { return u8n_; })
  MKSTR_CXX11_ONLY(constexpr std::size_t length(const char16_t*) const { return u16n_; })
  MKSTR_CXX11_ONLY(constexpr std::size_t length(const char32_t*) const { return u32n_; })

  // These implicit conversions are used by some tests. TODO: maybe eliminate them?
  TEST_CONSTEXPR operator const char*() const { return s_; }
  MKSTR_WCHAR_ONLY(TEST_CONSTEXPR operator const wchar_t*() const { return w_; })
  MKSTR_CHAR8_ONLY(constexpr operator const char8_t*() const { return u8_; })
  MKSTR_CXX11_ONLY(constexpr operator const char16_t*() const { return u16_; })
  MKSTR_CXX11_ONLY(constexpr operator const char32_t*() const { return u32_; })
};

// Helper to convert a const char* string to a const CharT*.
// This helper is used in unit tests to make them generic. The input should be
// valid ASCII which means the input is also valid UTF-8.
#define MAKE_CSTRING(CharT, Str)                                               \
  MKSTR(Str).as_ptr(static_cast<const CharT*>(nullptr))

// Like MAKE_CSTRING but makes a basic_string<CharT>. Embedded nulls are OK.
#define MAKE_STRING(CharT, Str)                                                \
  std::basic_string<CharT>(MAKE_CSTRING(CharT, Str), MKSTR_LEN(CharT, Str))

// Like MAKE_CSTRING but makes a basic_string_view<CharT>. Embedded nulls are OK.
#define MAKE_STRING_VIEW(CharT, Str)                                           \
  std::basic_string_view<CharT>(MAKE_CSTRING(CharT, Str), MKSTR_LEN(CharT, Str))

#endif
