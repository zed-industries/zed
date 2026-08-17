/*
 * Copyright (C) 2010 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_ORIENTATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_ORIENTATION_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/character.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

enum class FontOrientation {
  // Horizontal; i.e., writing-mode: horizontal-tb
  kHorizontal = 0,
  // Baseline is vertical but use rotated horizontal typography;
  // i.e., writing-mode: vertical-*; text-orientation: sideways-*
  kVerticalRotated = 1,
  // Vertical with upright CJK and rotated non-CJK;
  // i.e., writing-mode: vertical-*, text-orientation: mixed
  kVerticalMixed = 2,
  // Vertical with all upright;
  // i.e., writing-mode: vertical-*, text-orientation: upright
  kVerticalUpright = 3,
};
const unsigned kFontOrientationBitCount = 2;
const unsigned kFontOrientationAnyUprightMask = 2;

inline bool IsVerticalAnyUpright(FontOrientation orientation) {
  return static_cast<unsigned>(orientation) & kFontOrientationAnyUprightMask;
}
inline bool IsVerticalNonCJKUpright(FontOrientation orientation) {
  return orientation == FontOrientation::kVerticalUpright;
}
inline bool IsVerticalUpright(FontOrientation orientation, UChar32 character) {
  return orientation == FontOrientation::kVerticalUpright ||
         (orientation == FontOrientation::kVerticalMixed &&
          Character::IsUprightInMixedVertical(character));
}
inline bool IsVerticalBaseline(FontOrientation orientation) {
  return orientation != FontOrientation::kHorizontal;
}

inline FontOrientation AdjustOrientationForCharacterInMixedVertical(
    FontOrientation orientation,
    UChar32 character) {
  if (orientation != FontOrientation::kVerticalMixed)
    return orientation;
  return Character::IsUprightInMixedVertical(character)
             ? FontOrientation::kVerticalUpright
             : FontOrientation::kVerticalRotated;
}

PLATFORM_EXPORT String ToString(FontOrientation);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_ORIENTATION_H_
