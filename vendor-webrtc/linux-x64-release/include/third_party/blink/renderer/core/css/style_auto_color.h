// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_AUTO_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_AUTO_COLOR_H_

#include "third_party/blink/renderer/core/css/style_color.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class StyleAutoColor : public StyleColor {
  DISALLOW_NEW();

 public:
  explicit StyleAutoColor(StyleColor&& color) : StyleColor(color) {}

  static StyleAutoColor AutoColor() {
    return StyleAutoColor(StyleColor(CSSValueID::kAuto));
  }

  bool IsAutoColor() const { return color_keyword_ == CSSValueID::kAuto; }

  const StyleColor& ToStyleColor() const {
    DCHECK(!IsAutoColor());
    return *this;
  }
};

inline bool operator==(const StyleAutoColor& a, const StyleAutoColor& b) {
  if (a.IsAutoColor() || b.IsAutoColor()) {
    return a.IsAutoColor() && b.IsAutoColor();
  }
  return a.ToStyleColor() == b.ToStyleColor();
}

inline bool operator!=(const StyleAutoColor& a, const StyleAutoColor& b) {
  return !(a == b);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_AUTO_COLOR_H_
