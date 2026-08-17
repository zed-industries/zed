/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2009 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_REPLACED_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_REPLACED_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/layout_box.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

struct PhysicalNaturalSizingInfo;

// LayoutReplaced is the base class for a replaced element as defined by CSS:
//
// "An element whose content is outside the scope of the CSS formatting model,
// such as an image, embedded document, or applet."
// http://www.w3.org/TR/CSS2/conform.html#defs
//
// Blink consider that replaced elements have natural dimensions (e.g. the
// natural size of an image or a video). The natural dimensions are provided by
// an implementation of `GetNaturalDimensions()`.
class CORE_EXPORT LayoutReplaced : public LayoutBox {
 public:
  explicit LayoutReplaced(Element*);
  ~LayoutReplaced() override;

  // This function returns the local rect of the replaced content. The rectangle
  // is in the coordinate space of the element's physical border-box and assumes
  // no clipping.
  PhysicalRect ReplacedContentRect() const;
  virtual PhysicalRect ReplacedContentRectFrom(
      const PhysicalRect& base_content_rect) const;

  // This is used by a few special elements, e.g. <video>, <iframe> to ensure
  // a persistent sizing under different subpixel offset, because these
  // elements have a high cost to resize. The drawback is that we may overflow
  // or underflow the final content box by 1px.
  static PhysicalRect PreSnappedRectForPersistentSizing(const PhysicalRect&);

  void AddVisualEffectOverflow();
  void RecalcVisualOverflow() override;

  // These values are specified to be 300 and 150 pixels in the CSS 2.1 spec.
  // http://www.w3.org/TR/CSS2/visudet.html#inline-replaced-width
  static const int kDefaultWidth;
  static const int kDefaultHeight;
  bool CanHaveChildren() const override {
    NOT_DESTROYED();
    return false;
  }
  virtual bool DrawsBackgroundOntoContentLayer() const {
    NOT_DESTROYED();
    return false;
  }
  virtual void PaintReplaced(const PaintInfo&,
                             const PhysicalOffset& paint_offset) const {
    NOT_DESTROYED();
  }

  PhysicalRect LocalSelectionVisualRect() const final;

  bool HasObjectFit() const {
    NOT_DESTROYED();
    return StyleRef().GetObjectFit() !=
           ComputedStyleInitialValues::InitialObjectFit();
  }

  void Paint(const PaintInfo&) const override;

  // Compute the natural dimensions of the replaced content. Should not apply
  // any additional transformations (like 'object-view-box'). The natural size
  // returned should be in "zoomed CSS pixels" (i.e
  // `ComputedStyle::EffectiveZoom()` should be applied). The natural aspect
  // ratio needn't be zoomed (but can be).
  virtual PhysicalNaturalSizingInfo GetNaturalDimensions() const = 0;

  // Returns the natural dimensions of the replaced content with any additional
  // transformations - such as 'object-view-box' - applied.
  // This function is public only so we can call it when computing
  // intrinsic size in LayoutNG.
  PhysicalNaturalSizingInfo ComputeNaturalSizingInfo() const;

  // This callback must be invoked whenever the underlying natural dimensions
  // has changed.
  //
  // The natural dimensions can change due to the network (from the default
  // natural size [see above] to the actual natural dimensions) or to some CSS
  // properties like 'zoom' or 'image-orientation'.
  virtual void NaturalSizeChanged();

  bool RespectsCSSOverflow() const override;

  // Returns true if the content is guarenteed to be clipped to the element's
  // content box.
  bool ClipsToContentBox() const;

 protected:
  virtual bool ShouldApplyObjectViewBox() const {
    NOT_DESTROYED();
    return true;
  }

  bool IsInSelfHitTestingPhase(HitTestPhase phase) const override {
    NOT_DESTROYED();
    if (LayoutBox::IsInSelfHitTestingPhase(phase))
      return true;

    auto* element = DynamicTo<Element>(GetNode());
    return element && element->IsReplacedElementRespectingCSSOverflow() &&
           phase == HitTestPhase::kSelfBlockBackground;
  }

  void WillBeDestroyed() override;

  // This function calculates the placement of the replaced contents. It takes
  // natural dimensions of the replaced contents, stretch to fit CSS content
  // box according to object-fit, object-position and object-view-box.
  PhysicalRect ComputeReplacedContentRect(
      const PhysicalRect& base_content_rect,
      const PhysicalNaturalSizingInfo& sizing_info) const;

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  PositionWithAffinity PositionForPoint(const PhysicalOffset&) const override;

  bool IsLayoutReplaced() const final {
    NOT_DESTROYED();
    return true;
  }

  // ReplacedPainter doesn't support CompositeBackgroundAttachmentFixed yet.
  bool ComputeCanCompositeBackgroundAttachmentFixed() const override {
    NOT_DESTROYED();
    return false;
  }

  // ImageResourceObserver
  gfx::Size GetSpeculativeDecodeSize() const override;

 private:
  // Computes a rect, relative to the element's content's natural size, that
  // should be used as the content source when rendering this element. This
  // value is used as the input for object-fit/object-position during painting.
  std::optional<PhysicalRect> ComputeObjectViewBoxRect(
      const PhysicalNaturalSizingInfo& sizing_info) const;

  PhysicalRect ComputeObjectFitAndPositionRect(
      const PhysicalRect& base_content_rect,
      const PhysicalNaturalSizingInfo& sizing_info) const;
};

template <>
struct DowncastTraits<LayoutReplaced> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutReplaced();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_REPLACED_H_
