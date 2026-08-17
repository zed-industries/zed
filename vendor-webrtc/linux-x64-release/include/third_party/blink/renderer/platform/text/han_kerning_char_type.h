// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_HAN_KERNING_CHAR_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_HAN_KERNING_CHAR_TYPE_H_

#include <stdint.h>

namespace blink {

//
// Character types for the `HanKerning` class.
//
// https://drafts.csswg.org/css-text-4/#text-spacing-classes
//
enum class HanKerningCharType : uint8_t {
  kOther,
  kOpen,
  kClose,
  kMiddle,

  // Unicode General Category `Ps` and `Pe` that are not fullwidth. They are not
  // in the "Text Spacing Character Classes", but the "Fullwidth Punctuation
  // Collapsing" has them.
  // https://drafts.csswg.org/css-text-4/#fullwidth-collapsing
  kOpenNarrow,
  kCloseNarrow,

  // Following types depend on fonts. `HanKerning::GetCharType()` can resolve
  // them to types above.
  kDot,
  kColon,
  kSemicolon,
  kOpenQuote,
  kCloseQuote,

  // When adding values, ensure `CharacterProperty` has enough storage.
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_HAN_KERNING_CHAR_TYPE_H_
