// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_WHITE_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_WHITE_SPACE_H_

#include <bit>
#include <cstdint>

#include "third_party/blink/renderer/core/style/computed_style_base_constants.h"

namespace blink {

//
// This file contains definitions of the `white-space` shorthand property and
// its longhands.
// https://w3c.github.io/csswg-drafts/css-text-4/#propdef-white-space
//

//
// The `white-space-collapse` property.
// https://w3c.github.io/csswg-drafts/css-text-4/#white-space-collapsing
//
enum class WhiteSpaceCollapse : uint8_t {
  kCollapse = 0,
  kPreserve = 1,
  // `KPreserve` is a bit-flag, but bit 2 is shared by two different behaviors
  // below to save memory. Use functions below instead of direct comparisons.
  kPreserveBreaks = 2,
  kBreakSpaces = kPreserve | 2,
  // Ensure `kWhiteSpaceCollapseBits` can hold all values.
};

// Ensure this is in sync with `css_properties.json5`.
static constexpr int kWhiteSpaceCollapseBits = 2;
static constexpr uint8_t kWhiteSpaceCollapseMask =
    (1 << kWhiteSpaceCollapseBits) - 1;

inline bool IsWhiteSpaceCollapseAny(WhiteSpaceCollapse value,
                                    WhiteSpaceCollapse flags) {
  return static_cast<uint8_t>(value) & static_cast<uint8_t>(flags);
}

// Whether to collapse or preserve all whitespaces: spaces (U+0020), tabs
// (U+0009), and segment breaks.
// https://w3c.github.io/csswg-drafts/css-text-4/#white-space
inline bool ShouldPreserveWhiteSpaces(WhiteSpaceCollapse collapse) {
  return IsWhiteSpaceCollapseAny(collapse, WhiteSpaceCollapse::kPreserve);
}
inline bool ShouldCollapseWhiteSpaces(WhiteSpaceCollapse collapse) {
  return !ShouldPreserveWhiteSpaces(collapse);
}
// Whether to collapse or preserve segment breaks.
// https://w3c.github.io/csswg-drafts/css-text-4/#segment-break
inline bool ShouldPreserveBreaks(WhiteSpaceCollapse collapse) {
  return collapse != WhiteSpaceCollapse::kCollapse;
}
inline bool ShouldCollapseBreaks(WhiteSpaceCollapse collapse) {
  return !ShouldPreserveBreaks(collapse);
}
inline bool ShouldBreakSpaces(WhiteSpaceCollapse collapse) {
  return collapse == WhiteSpaceCollapse::kBreakSpaces;
}

//
// The `text-wrap-mode` property.
// https://drafts.csswg.org/css-text-4/#propdef-text-wrap-mode
//
inline constexpr unsigned kTextWrapModeBits =
    std::bit_width(static_cast<unsigned>(TextWrapMode::kMaxEnumValue));

// Returns `true` if lines should wrap.
inline bool ShouldWrapLine(TextWrapMode mode) {
  return mode != TextWrapMode::kNowrap;
}

//
// The `text-wrap-style` property.
// https://drafts.csswg.org/css-text-4/#propdef-text-wrap-style
//

// Returns `true` if the greedy line breaker should be used.
inline bool ShouldWrapLineGreedy(TextWrapStyle style) {
  return style == TextWrapStyle::kAuto || style == TextWrapStyle::kStable;
}

//
// The `white-space` property.
// https://w3c.github.io/csswg-drafts/css-text-4/#propdef-white-space
//
// `EWhiteSpace` is represented by bit-flags of combinations of all possible
// longhand values. Thus `ToWhiteSpace()` may return values that are not defined
// as the `EWhiteSpace` value. `IsValidWhiteSpace()` can check if a value is one
// of pre-defined keywords.
//
constexpr uint8_t ToWhiteSpaceValue(WhiteSpaceCollapse collapse,
                                    TextWrapMode wrap) {
  return static_cast<uint8_t>(collapse) |
         (static_cast<uint8_t>(wrap) << kWhiteSpaceCollapseBits);
}

enum class EWhiteSpace : uint8_t {
  kNormal =
      ToWhiteSpaceValue(WhiteSpaceCollapse::kCollapse, TextWrapMode::kWrap),
  kNowrap =
      ToWhiteSpaceValue(WhiteSpaceCollapse::kCollapse, TextWrapMode::kNowrap),
  kPre =
      ToWhiteSpaceValue(WhiteSpaceCollapse::kPreserve, TextWrapMode::kNowrap),
  kPreLine = ToWhiteSpaceValue(WhiteSpaceCollapse::kPreserveBreaks,
                               TextWrapMode::kWrap),
  kPreWrap =
      ToWhiteSpaceValue(WhiteSpaceCollapse::kPreserve, TextWrapMode::kWrap),
  kBreakSpaces =
      ToWhiteSpaceValue(WhiteSpaceCollapse::kBreakSpaces, TextWrapMode::kWrap),
};

static_assert(kWhiteSpaceCollapseBits + kTextWrapModeBits <=
              sizeof(EWhiteSpace) * 8);

// Convert longhands of `white-space` to `EWhiteSpace`. The return value may not
// be one of the defined enum values. Please see the comment above.
inline EWhiteSpace ToWhiteSpace(WhiteSpaceCollapse collapse,
                                TextWrapMode wrap) {
  return static_cast<EWhiteSpace>(ToWhiteSpaceValue(collapse, wrap));
}

inline bool IsValidWhiteSpace(EWhiteSpace whitespace) {
  return whitespace == EWhiteSpace::kNormal ||
         whitespace == EWhiteSpace::kNowrap ||
         whitespace == EWhiteSpace::kPre ||
         whitespace == EWhiteSpace::kPreLine ||
         whitespace == EWhiteSpace::kPreWrap ||
         whitespace == EWhiteSpace::kBreakSpaces;
}

// Convert `EWhiteSpace` to longhands.
inline WhiteSpaceCollapse ToWhiteSpaceCollapse(EWhiteSpace whitespace) {
  return static_cast<WhiteSpaceCollapse>(static_cast<uint8_t>(whitespace) &
                                         kWhiteSpaceCollapseMask);
}
inline TextWrapMode ToTextWrapMode(EWhiteSpace whitespace) {
  return static_cast<TextWrapMode>(static_cast<uint8_t>(whitespace) >>
                                   kWhiteSpaceCollapseBits);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_WHITE_SPACE_H_
