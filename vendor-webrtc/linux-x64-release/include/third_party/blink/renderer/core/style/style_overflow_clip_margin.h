// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_OVERFLOW_CLIP_MARGIN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_OVERFLOW_CLIP_MARGIN_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

class StyleOverflowClipMargin {
  DISALLOW_NEW();

 public:
  enum class ReferenceBox { kBorderBox, kPaddingBox, kContentBox };
  StyleOverflowClipMargin(ReferenceBox reference_box, LayoutUnit margin)
      : reference_box_(reference_box), margin_(margin) {}

  static StyleOverflowClipMargin CreateContent() {
    return StyleOverflowClipMargin(ReferenceBox::kContentBox, LayoutUnit());
  }

  StyleOverflowClipMargin() = default;

  ReferenceBox GetReferenceBox() const { return reference_box_; }
  LayoutUnit GetMargin() const { return margin_; }

  bool operator==(const StyleOverflowClipMargin& o) const {
    return GetReferenceBox() == o.GetReferenceBox() &&
           GetMargin() == o.GetMargin();
  }

  bool operator!=(const StyleOverflowClipMargin& o) const {
    return !(*this == o);
  }

 private:
  ReferenceBox reference_box_;
  LayoutUnit margin_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_OVERFLOW_CLIP_MARGIN_H_
