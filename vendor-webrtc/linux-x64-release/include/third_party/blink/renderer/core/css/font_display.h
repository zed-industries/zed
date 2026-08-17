// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_DISPLAY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_DISPLAY_H_

#include <cstdint>

namespace blink {

class CSSValue;

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class FontDisplay : uint8_t {
  kAuto,
  kBlock,
  kSwap,
  kFallback,
  kOptional,
  kMaxValue = kOptional,
};

FontDisplay CSSValueToFontDisplay(const CSSValue*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_DISPLAY_H_
