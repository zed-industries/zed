// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_STRING_SPLIT_WIN_H_
#define BASE_STRINGS_STRING_SPLIT_WIN_H_

#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/strings/string_split.h"

namespace base {

// The following section contains overloads of the cross-platform APIs for
// std::wstring and std::wstring_view.
[[nodiscard]] BASE_EXPORT std::vector<std::wstring> SplitString(
    std::wstring_view input,
    std::wstring_view separators,
    WhitespaceHandling whitespace,
    SplitResult result_type);

[[nodiscard]] BASE_EXPORT std::vector<std::wstring_view> SplitStringPiece(
    std::wstring_view input LIFETIME_BOUND,
    std::wstring_view separators,
    WhitespaceHandling whitespace,
    SplitResult result_type);

[[nodiscard]] BASE_EXPORT std::vector<std::wstring> SplitStringUsingSubstr(
    std::wstring_view input,
    std::wstring_view delimiter,
    WhitespaceHandling whitespace,
    SplitResult result_type);

[[nodiscard]] BASE_EXPORT std::vector<std::wstring_view>
SplitStringPieceUsingSubstr(std::wstring_view input LIFETIME_BOUND,
                            std::wstring_view delimiter,
                            WhitespaceHandling whitespace,
                            SplitResult result_type);

}  // namespace base

#endif  // BASE_STRINGS_STRING_SPLIT_WIN_H_
