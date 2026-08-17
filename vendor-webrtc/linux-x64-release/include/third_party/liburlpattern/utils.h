// Copyright 2020 The Chromium Authors
// Copyright 2014 Blake Embrey (hello@blakeembrey.com)
// Use of this source code is governed by an MIT-style license that can be
// found in the LICENSE file or at https://opensource.org/licenses/MIT.

#ifndef THIRD_PARTY_LIBURLPATTERN_UTILS_H_
#define THIRD_PARTY_LIBURLPATTERN_UTILS_H_

#include <string>
#include <string_view>

#include "base/component_export.h"
#include "third_party/icu/source/common/unicode/uchar.h"

namespace liburlpattern {

// The "full wildcard" regex pattern.  This regex value is treated specially
// resulting in a kFullWildcard Part instead of a kRegex Part.
constexpr const char* kFullWildcardRegex = ".*";

// Return the expected length of the value returned by EscapeString().
COMPONENT_EXPORT(LIBURLPATTERN)
size_t EscapedRegexpStringLength(std::string_view input);

// Escape an input string so that it may be safely included in a
// regular expression.
COMPONENT_EXPORT(LIBURLPATTERN)
std::string EscapeRegexpString(std::string_view input);

// Escape the input string so that it may be safely included in a
// regular expression and append the result directly to the given target.
void EscapeRegexpStringAndAppend(std::string_view input,
                                 std::string& append_target);

// Escape a fixed input string so that it may be safely included in a
// pattern string.  Appends the result directly to the given target.
COMPONENT_EXPORT(LIBURLPATTERN)
void EscapePatternStringAndAppend(std::string_view input,
                                  std::string& append_target);

// Return `true` if the given codepoint `c` is valid for a `:foo` name.  The
// `first_codepoint` argument can be set if this codepoint is intended to be
// the first codepoint in a name.  If its false, then the codepoint is treated
// as a trailing character.
bool IsNameCodepoint(UChar32 c, bool first_codepoint);

}  // namespace liburlpattern

#endif  // THIRD_PARTY_LIBURLPATTERN_UTILS_H_
