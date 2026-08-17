// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_STRCAT_H_
#define BASE_STRINGS_STRCAT_H_

#include <initializer_list>
#include <string_view>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
// Guard against conflict with Win32 API StrCat macro:
// check StrCat wasn't and will not be redefined.
#define StrCat StrCat
#endif

namespace base {

// StrCat ----------------------------------------------------------------------
//
// StrCat is a function to perform concatenation on a sequence of strings.
// It is preferrable to a sequence of "a + b + c" because it is both faster and
// generates less code.
//
//   std::string result = base::StrCat({"foo ", result, "\nfoo ", bar});
//
// To join an array of strings with a separator, see base::JoinString in
// base/strings/string_util.h.
//
// MORE INFO
//
// StrCat can see all arguments at once, so it can allocate one return buffer
// of exactly the right size and copy once, as opposed to a sequence of
// operator+ which generates a series of temporary strings, copying as it goes.
// And by using std::string_view arguments, StrCat can avoid creating temporary
// string objects for char* constants.
//
// ALTERNATIVES
//
// Internal Google / Abseil has a similar StrCat function. That version takes
// an overloaded number of arguments instead of initializer list (overflowing
// to initializer list for many arguments). We don't have any legacy
// requirements and using only initializer_list is simpler and generates
// roughly the same amount of code at the call sites.
//
// Abseil's StrCat also allows numbers by using an intermediate class that can
// be implicitly constructed from either a string or various number types. This
// class formats the numbers into a static buffer for increased performance,
// and the call sites look nice.
//
// As-written Abseil's helper class for numbers generates slightly more code
// than the raw std::string_view version. We can de-inline the helper class'
// constructors which will cause the std::string_view constructors to be
// de-inlined for this call and generate slightly less code. This is something
// we can explore more in the future.

[[nodiscard]] BASE_EXPORT std::string StrCat(
    span<const std::string_view> pieces);
[[nodiscard]] BASE_EXPORT std::u16string StrCat(
    span<const std::u16string_view> pieces);
[[nodiscard]] BASE_EXPORT std::string StrCat(span<const std::string> pieces);
[[nodiscard]] BASE_EXPORT std::u16string StrCat(
    span<const std::u16string> pieces);

// Initializer list forwards to the array version.
inline std::string StrCat(std::initializer_list<std::string_view> pieces) {
  return StrCat(span(pieces));
}

inline std::u16string StrCat(
    std::initializer_list<std::u16string_view> pieces) {
  return StrCat(span(pieces));
}

// StrAppend -------------------------------------------------------------------
//
// Appends a sequence of strings to a destination. Prefer:
//   StrAppend(&foo, ...);
// over:
//   foo += StrCat(...);
// because it avoids a temporary string allocation and copy.

BASE_EXPORT void StrAppend(std::string* dest,
                           span<const std::string_view> pieces);
BASE_EXPORT void StrAppend(std::u16string* dest,
                           span<const std::u16string_view> pieces);
BASE_EXPORT void StrAppend(std::string* dest, span<const std::string> pieces);
BASE_EXPORT void StrAppend(std::u16string* dest,
                           span<const std::u16string> pieces);

// Initializer list forwards to the array version.
inline void StrAppend(std::string* dest,
                      std::initializer_list<std::string_view> pieces) {
  StrAppend(dest, span(pieces));
}

inline void StrAppend(std::u16string* dest,
                      std::initializer_list<std::u16string_view> pieces) {
  StrAppend(dest, span(pieces));
}

}  // namespace base

#if BUILDFLAG(IS_WIN)
#include "base/strings/strcat_win.h"
#endif

#endif  // BASE_STRINGS_STRCAT_H_
