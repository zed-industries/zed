// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_PATTERN_H_
#define BASE_STRINGS_PATTERN_H_

#include <string_view>

#include "base/base_export.h"

namespace base {

// Returns true if the |string| passed in matches the |pattern|. The pattern
// string can contain wildcards like * and ?.
//
// The backslash character (\) is an escape character for * and ?.
// ? matches 0 or 1 character, while * matches 0 or more characters.
BASE_EXPORT bool MatchPattern(std::string_view string,
                              std::string_view pattern);
BASE_EXPORT bool MatchPattern(std::u16string_view string,
                              std::u16string_view pattern);

}  // namespace base

#endif  // BASE_STRINGS_PATTERN_H_
