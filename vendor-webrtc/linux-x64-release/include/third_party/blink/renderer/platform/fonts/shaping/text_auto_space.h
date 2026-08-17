// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_TEXT_AUTO_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_TEXT_AUTO_SPACE_H_

#include <unicode/umachine.h>
#include <ostream>

#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class Font;

class PLATFORM_EXPORT TextAutoSpace {
  STACK_ALLOCATED();

 public:
  enum CharType { kOther, kIdeograph, kLetterOrNumeral };

  // Returns the `CharType` according to:
  // https://drafts.csswg.org/css-text-4/#text-spacing-classes
  static CharType GetType(UChar32 ch);

  // `GetType` and advance the `offset` by one character (grapheme cluster.)
  static CharType GetTypeAndNext(const WTF::String& text, wtf_size_t& offset);
  // `GetType` of the character before `offset`.
  static CharType GetPrevType(const WTF::String& text, wtf_size_t offset);

  // `CharType::kIdeograph` is `USCRIPT_HAN`, except characters in this range
  // may be other scripts.
  constexpr static UChar32 kNonHanIdeographMin = 0x3041;
  constexpr static UChar32 kNonHanIdeographMax = 0x31FF;

  // Get the inter-script auto-spacing width.
  // https://drafts.csswg.org/css-text-4/#inter-script-spacing
  static float GetSpacingWidth(const Font* font);
};

PLATFORM_EXPORT std::ostream& operator<<(std::ostream& ostream,
                                         TextAutoSpace::CharType);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_TEXT_AUTO_SPACE_H_
