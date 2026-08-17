//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBCXX_TEST_SUPPORT_LOCALE_HELPERS_H
#define LIBCXX_TEST_SUPPORT_LOCALE_HELPERS_H

#include <string>
#include "platform_support.h"
#include "test_macros.h"
#include "make_string.h"

#ifndef TEST_HAS_NO_WIDE_CHARACTERS

#include <cwctype>

#endif // TEST_HAS_NO_WIDE_CHARACTERS

namespace LocaleHelpers {

#ifndef TEST_HAS_NO_WIDE_CHARACTERS

std::wstring convert_thousands_sep(std::wstring const& in, wchar_t sep) {
  std::wstring out;
  bool seen_num_start = false;
  bool seen_decimal = false;
  for (unsigned i = 0; i < in.size(); ++i) {
    seen_decimal |= in[i] == L',';
    seen_num_start |= in[i] == L'-' || std::iswdigit(in[i]);
    if (seen_decimal || !seen_num_start || in[i] != L' ') {
      out.push_back(in[i]);
      continue;
    }
    assert(in[i] == L' ');
    out.push_back(sep);
  }
  return out;
}

std::wstring negate_en_US(std::wstring s) {
#if defined(_WIN32)
  return L"(" + s + L")";
#else
  return L"-" + s;
#endif
}

wchar_t thousands_sep_or_default(std::wstring s) { return !s.empty() ? s[0] : L','; }

wchar_t mon_thousands_sep_or_default(std::wstring s) { return thousands_sep_or_default(s); }

wchar_t decimal_point_or_default(std::wstring s) { return !s.empty() ? s[0] : L'.'; }

#endif // TEST_HAS_NO_WIDE_CHARACTERS

std::string negate_en_US(std::string s) {
#if defined(_WIN32)
  return "(" + s + ")";
#else
  return "-" + s;
#endif
}

MultiStringType currency_symbol_ru_RU() {
#if defined(_CS_GNU_LIBC_VERSION)
  if (glibc_version_less_than("2.24"))
    return MKSTR("\u0440\u0443\u0431");
  else
    return MKSTR("\u20BD"); // U+20BD RUBLE SIGN
#elif defined(_WIN32) || defined(__FreeBSD__) || defined(_AIX)
  return MKSTR("\u20BD"); // U+20BD RUBLE SIGN
#else
  return MKSTR("\u0440\u0443\u0431.");
#endif
}

MultiStringType currency_symbol_zh_CN() {
#if defined(_WIN32)
  return MKSTR("\u00A5"); // U+00A5 YEN SIGN
#else
  return MKSTR("\uFFE5"); // U+FFE5 FULLWIDTH YEN SIGN
#endif
}

} // namespace LocaleHelpers

#endif // LIBCXX_TEST_SUPPORT_LOCALE_HELPERS_H
