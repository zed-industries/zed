// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_TEXT_SPACING_TRIM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_TEXT_SPACING_TRIM_H_

namespace blink {

// Values for the `text-spacing-trim` property.
// https://drafts.csswg.org/css-text-4/#text-spacing-trim-property
enum class TextSpacingTrim {
  kNormal,
  kSpaceAll,
  kSpaceFirst,
  kTrimStart,

  kInitial = kNormal,
};

inline constexpr unsigned kTextSpacingTrimBitCount = 2;

inline bool ShouldTrimAdjacent(TextSpacingTrim value) {
  return value != TextSpacingTrim::kSpaceAll;
}

inline bool ShouldTrimStartOfParagraph(TextSpacingTrim value) {
  return value == TextSpacingTrim::kTrimStart;
}

inline bool ShouldTrimStartOfWrappedLine(TextSpacingTrim value) {
  return value == TextSpacingTrim::kSpaceFirst ||
         value == TextSpacingTrim::kTrimStart;
}

inline bool ShouldTrimEnd(TextSpacingTrim value) {
  return value != TextSpacingTrim::kSpaceAll;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_TEXT_SPACING_TRIM_H_
