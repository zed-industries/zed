// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BORDER_EDGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BORDER_EDGE_H_

#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class BorderEdge {
  STACK_ALLOCATED();

 public:
  BorderEdge(int edge_width,
             const Color& edge_color,
             EBorderStyle edge_style,
             bool edge_is_present = true);
  BorderEdge();

  static EBorderStyle EffectiveStyle(EBorderStyle style, int width);

  bool HasVisibleColorAndStyle() const;
  bool ShouldRender() const;
  bool PresentButInvisible() const;
  bool ObscuresBackgroundEdge() const;
  bool ObscuresBackground() const;
  int UsedWidth() const;

  bool SharesColorWith(const BorderEdge& other) const;

  EBorderStyle BorderStyle() const { return style_; }

  enum DoubleBorderStripe {
    kDoubleBorderStripeOuter,
    kDoubleBorderStripeInner
  };

  int GetDoubleBorderStripeWidth(DoubleBorderStripe) const;

  int Width() const { return width_; }
  const Color& GetColor() const { return color_; }

  void ClampWidth(int max_width);

 private:
  Color color_;
  bool is_present_;
  EBorderStyle style_;
  int width_;
};

using BorderEdgeArray = std::array<BorderEdge, 4>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_BORDER_EDGE_H_
