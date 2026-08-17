// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_STYLE_VARIANT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_STYLE_VARIANT_H_

#include <cstdint>

#include "base/compiler_specific.h"

namespace blink {

// LayoutObject can have multiple style variations.
enum class StyleVariant : uint8_t {
  kStandard = 0,
  kFirstLine = 1,
  kStandardEllipsis = 2,
  kFirstLineEllipsis = 3
};

inline bool UsesFirstLineStyle(StyleVariant variant) {
  return static_cast<uint8_t>(variant) &
         static_cast<uint8_t>(StyleVariant::kFirstLine);
}

inline bool IsEllipsis(StyleVariant variant) {
  return static_cast<uint8_t>(variant) &
         static_cast<uint8_t>(StyleVariant::kStandardEllipsis);
}

inline StyleVariant ToParentStyleVariant(StyleVariant variant) {
  // Ancestors of ellipsis should not use the ellipsis style variant.
  return static_cast<StyleVariant>(
      static_cast<uint8_t>(variant) &
      ~static_cast<uint8_t>(StyleVariant::kStandardEllipsis));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_STYLE_VARIANT_H_
