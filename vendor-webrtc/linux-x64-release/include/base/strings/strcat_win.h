// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_STRCAT_WIN_H_
#define BASE_STRINGS_STRCAT_WIN_H_

#include <initializer_list>
#include <string>
#include <string_view>

#include "base/base_export.h"
#include "base/containers/span.h"

namespace base {

// The following section contains overloads of the cross-platform APIs for
// std::wstring and std::wstring_view.
BASE_EXPORT void StrAppend(std::wstring* dest,
                           span<const std::wstring_view> pieces);
BASE_EXPORT void StrAppend(std::wstring* dest, span<const std::wstring> pieces);

inline void StrAppend(std::wstring* dest,
                      std::initializer_list<std::wstring_view> pieces) {
  StrAppend(dest, span(pieces));
}

[[nodiscard]] BASE_EXPORT std::wstring StrCat(
    span<const std::wstring_view> pieces);
[[nodiscard]] BASE_EXPORT std::wstring StrCat(span<const std::wstring> pieces);

inline std::wstring StrCat(std::initializer_list<std::wstring_view> pieces) {
  return StrCat(span(pieces));
}

}  // namespace base

#endif  // BASE_STRINGS_STRCAT_WIN_H_
