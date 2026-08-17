// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FONT_UNIQUE_NAME_LOOKUP_ICU_FOLD_CASE_UTIL_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FONT_UNIQUE_NAME_LOOKUP_ICU_FOLD_CASE_UTIL_H_

#include <string>
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// Executes ICU's UnicodeString locale-independent foldCase method on
// |name_request| and returns a case folded string suitable for case-insensitive
// bitwise comparison. Used by FontTableMatcher and FontUniqueNameLookup for
// storing and comparing case folded font names.
std::string BLINK_COMMON_EXPORT IcuFoldCase(const std::string& name_request);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FONT_UNIQUE_NAME_LOOKUP_ICU_FOLD_CASE_UTIL_H_
