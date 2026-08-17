// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_VIEW_TRANSITION_CONTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_VIEW_TRANSITION_CONTENT_H_

#include "base/memory/scoped_refptr.h"
#include "cc/layers/view_transition_content_layer.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/core/layout/layout_replaced.h"
#include "third_party/blink/renderer/core/view_transition/view_transition_content_element.h"

namespace blink {

class CORE_EXPORT LayoutViewTransitionContent : public LayoutReplaced {
 public:
  explicit LayoutViewTransitionContent(ViewTransitionContentElement*);
  ~LayoutViewTransitionContent() override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutViewTransitionContent";
  }
  void OnIntrinsicSizeUpdated(
      const gfx::RectF& captured_rect,
      const gfx::RectF& reference_rect_in_enclosing_layer_space,
      bool propagate_max_extent_rect);

  bool IsViewTransitionContent() const override {
    NOT_DESTROYED();
    return true;
  }

 protected:
  PaintLayerType LayerTypeRequired() const override;
  void NaturalSizeChanged() override { NOT_DESTROYED(); }
  void PaintReplaced(const PaintInfo&,
                     const PhysicalOffset& paint_offset) const override;

 private:
  PhysicalRect ReplacedContentRectForCapturedContent() const;
  PhysicalNaturalSizingInfo GetNaturalDimensions() const override;

  scoped_refptr<cc::ViewTransitionContentLayer> layer_;
  gfx::RectF captured_rect_;
  gfx::RectF reference_rect_in_enclosing_layer_space_;
  bool propagate_max_extent_rect_;
};

template <>
struct DowncastTraits<LayoutViewTransitionContent> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsViewTransitionContent();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_VIEW_TRANSITION_CONTENT_H_
