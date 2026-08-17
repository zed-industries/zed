// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_UTF_OSTREAM_OPERATORS_H_
#define BASE_STRINGS_UTF_OSTREAM_OPERATORS_H_

#include <iosfwd>
#include <string>
#include <string_view>

#include "base/base_export.h"

// Note that "The behavior of a C++ program is undefined if it adds declarations
// or definitions to namespace std or to a namespace within namespace std unless
// otherwise specified." --C++11[namespace.std]
//
// We've checked that this particular definition has the intended behavior on
// our implementations, but it's prone to breaking in the future, and please
// don't imitate this in your own definitions without checking with some
// standard library experts.
namespace std {
// These functions are provided as a convenience for logging, which is where we
// use streams (it is against Google style to use streams in other places). It
// is designed to allow you to emit non-ASCII Unicode strings to the log file,
// which is normally ASCII. It is relatively slow, so try not to use it for
// common cases. Non-ASCII characters will be converted to UTF-8 by these
// operators.
//
// The `std::basic_string<T>` overloads are necessary to allow logging types
// which are implicitly convertible to `std::basic_string<T>`. Simply taking
// `std::basic_string_view<T>` would not work because C++ only allows one
// implicit conversion.
BASE_EXPORT std::ostream& operator<<(std::ostream& out, const wchar_t* wstr);
BASE_EXPORT std::ostream& operator<<(std::ostream& out, std::wstring_view wstr);
BASE_EXPORT std::ostream& operator<<(std::ostream& out,
                                     const std::wstring& wstr);

BASE_EXPORT std::ostream& operator<<(std::ostream& out, const char16_t* str16);
BASE_EXPORT std::ostream& operator<<(std::ostream& out,
                                     std::u16string_view str16);
BASE_EXPORT std::ostream& operator<<(std::ostream& out,
                                     const std::u16string& str16);
}  // namespace std

#endif  // BASE_STRINGS_UTF_OSTREAM_OPERATORS_H_
