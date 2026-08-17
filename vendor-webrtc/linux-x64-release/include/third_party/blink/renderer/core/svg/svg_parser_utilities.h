/*
 * Copyright (C) 2002, 2003 The Karbon Developers
 * Copyright (C) 2006, 2007 Rob Buis <buis@kde.org>
 * Copyright (C) 2013 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PARSER_UTILITIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PARSER_UTILITIES_H_

#include <algorithm>

#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "third_party/blink/renderer/core/html/parser/html_parser_idioms.h"

namespace blink {

enum WhitespaceMode {
  kDisallowWhitespace = 0,
  kAllowLeadingWhitespace = 0x1,
  kAllowTrailingWhitespace = 0x2,
  kAllowLeadingAndTrailingWhitespace =
      kAllowLeadingWhitespace | kAllowTrailingWhitespace
};

// DEPRECATED: Use the following `base::span` variant to avoid unsafe buffer
// usage.
bool ParseNumber(const LChar*& ptr,
                 const LChar* end,
                 float& number,
                 WhitespaceMode = kAllowLeadingAndTrailingWhitespace);
bool ParseNumber(base::span<const LChar>& span,
                 float& number,
                 WhitespaceMode = kAllowLeadingAndTrailingWhitespace);

// DEPRECATED: Use the following `base::span` variant to avoid unsafe buffer
// usage.
bool ParseNumber(const UChar*& ptr,
                 const UChar* end,
                 float& number,
                 WhitespaceMode = kAllowLeadingAndTrailingWhitespace);
bool ParseNumber(base::span<const UChar>& span,
                 float& number,
                 WhitespaceMode = kAllowLeadingAndTrailingWhitespace);

bool ParseNumberOptionalNumber(const String& s, float& h, float& v);

// DEPRECATED: Use the following `base::span` variant to avoid unsafe buffer
// usage.
template <typename CharType>
inline bool SkipOptionalSVGSpaces(const CharType*& ptr, const CharType* end) {
  while (ptr < end && IsHTMLSpace<CharType>(*ptr)) {
    UNSAFE_TODO(ptr++);
  }
  return ptr < end;
}

template <typename CharType>
constexpr inline bool SkipOptionalSVGSpaces(base::span<const CharType>& span) {
  auto iter = std::ranges::find_if(
      span, [](const CharType c) { return !IsHTMLSpace<CharType>(c); });
  span = span.subspan(static_cast<size_t>(iter - span.begin()));
  return !span.empty();
}

// DEPRECATED: Use the following `base::span` variant to avoid unsafe buffer
// usage.
template <typename CharType>
inline bool SkipOptionalSVGSpacesOrDelimiter(const CharType*& ptr,
                                             const CharType* end,
                                             char delimiter = ',') {
  if (ptr < end && !IsHTMLSpace<CharType>(*ptr) && *ptr != delimiter) {
    return false;
  }
  if (SkipOptionalSVGSpaces(ptr, end)) {
    if (*ptr == delimiter) {
      UNSAFE_TODO(ptr++);
      SkipOptionalSVGSpaces(ptr, end);
    }
  }
  return ptr < end;
}

template <typename CharType>
constexpr inline bool SkipOptionalSVGSpacesOrDelimiter(
    base::span<const CharType>& span,
    char delimiter = ',') {
  if (!span.empty() && !IsHTMLSpace<CharType>(span[0]) &&
      span[0] != delimiter) {
    return false;
  }
  if (SkipOptionalSVGSpaces(span) && span[0] == delimiter) {
    span = span.template subspan<1u>();
    SkipOptionalSVGSpaces(span);
  }
  return !span.empty();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PARSER_UTILITIES_H_
