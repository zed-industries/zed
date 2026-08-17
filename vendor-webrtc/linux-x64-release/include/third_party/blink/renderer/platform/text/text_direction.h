/*
 * Copyright (C) 2003, 2006 Apple Computer, Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_DIRECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_DIRECTION_H_

#include <cstdint>
#include <iosfwd>
#include "base/i18n/rtl.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// The direction of text in bidirectional scripts such as Arabic or Hebrew.
//
// Used for explicit directions such as in the HTML dir attribute or the CSS
// 'direction' property.
// https://html.spec.whatwg.org/C/#the-dir-attribute
// https://drafts.csswg.org/css-writing-modes/#direction
//
// Also used for resolved directions by UAX#9 UNICODE BIDIRECTIONAL ALGORITHM.
// http://unicode.org/reports/tr9/
enum class TextDirection : uint8_t { kLtr = 0, kRtl = 1 };

inline bool IsLtr(TextDirection direction) {
  return direction == TextDirection::kLtr;
}

inline bool IsRtl(TextDirection direction) {
  return direction != TextDirection::kLtr;
}

inline TextDirection DirectionFromLevel(unsigned level) {
  return level & 1 ? TextDirection::kRtl : TextDirection::kLtr;
}

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, TextDirection);

inline base::i18n::TextDirection ToBaseTextDirection(TextDirection direction) {
  switch (direction) {
    case TextDirection::kLtr:
      return base::i18n::TextDirection::LEFT_TO_RIGHT;
    case TextDirection::kRtl:
      return base::i18n::TextDirection::RIGHT_TO_LEFT;
  }
  NOTREACHED();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_TEXT_DIRECTION_H_
