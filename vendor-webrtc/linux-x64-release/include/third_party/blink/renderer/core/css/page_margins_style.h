// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PAGE_MARGINS_STYLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PAGE_MARGINS_STYLE_H_

#include <array>

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ComputedStyle;

// Computed style for @page margin boxes.
class PageMarginsStyle {
  STACK_ALLOCATED();

 public:
  enum MarginSlot {
    TopLeft,
    TopCenter,
    TopRight,
    RightTop,
    RightMiddle,
    RightBottom,
    BottomLeft,
    BottomCenter,
    BottomRight,
    LeftTop,
    LeftMiddle,
    LeftBottom,
    TopLeftCorner,
    TopRightCorner,
    BottomRightCorner,
    BottomLeftCorner,

    kTotalMarginCount
  };

  const ComputedStyle* operator[](unsigned index) const {
    CHECK_LT(index, kTotalMarginCount);
    return entries[index];
  }
  const ComputedStyle*& operator[](unsigned index) {
    CHECK_LT(index, kTotalMarginCount);
    return entries[index];
  }

 private:
  std::array<const ComputedStyle*, kTotalMarginCount> entries = {};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PAGE_MARGINS_STYLE_H_
