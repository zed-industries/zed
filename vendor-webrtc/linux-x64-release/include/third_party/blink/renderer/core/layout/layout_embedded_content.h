/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Simon Hausmann <hausmann@kde.org>
 * Copyright (C) 2006, 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_EMBEDDED_CONTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_EMBEDDED_CONTENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/html_frame_owner_element.h"
#include "third_party/blink/renderer/core/layout/layout_replaced.h"
#include "third_party/blink/renderer/platform/transforms/affine_transform.h"

namespace ui {
class Cursor;
}

namespace blink {

class EmbeddedContentView;
class FrameView;
class WebPluginContainerImpl;

// LayoutObject for frames via LayoutFrame and LayoutIFrame, and plugins via
// LayoutEmbeddedObject.
class CORE_EXPORT LayoutEmbeddedContent : public LayoutReplaced {
 public:
  explicit LayoutEmbeddedContent(HTMLFrameOwnerElement*);

  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation&,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase) override;

  // LayoutEmbeddedContent::ChildFrameView returns the LocalFrameView associated
  // with the current Node, if Node is HTMLFrameOwnerElement. This is different
  // to LayoutObject::GetFrameView which returns the LocalFrameView associated
  // with the root Document Frame.
  FrameView* ChildFrameView() const;
  LayoutView* ChildLayoutView() const;
  WebPluginContainerImpl* Plugin() const;
  EmbeddedContentView* GetEmbeddedContentView() const;

  // Subtracts border/padding, and other offsets if they exist.
  PhysicalOffset EmbeddedContentFromBorderBox(const PhysicalOffset&) const;
  gfx::PointF EmbeddedContentFromBorderBox(const gfx::PointF&) const;
  // Adds border/padding, and other offsets if they exist.
  PhysicalOffset BorderBoxFromEmbeddedContent(const PhysicalOffset&) const;
  gfx::Rect BorderBoxFromEmbeddedContent(const gfx::Rect&) const;

  PhysicalRect ReplacedContentRectFrom(
      const PhysicalRect& base_content_rect) const final;

  void UpdateOnEmbeddedContentViewChange();
  void UpdateGeometry(EmbeddedContentView&);

  bool IsLayoutEmbeddedContent() const final {
    NOT_DESTROYED();
    return true;
  }

  bool IsThrottledFrameView() const;

  // The size of the child frame when it should be "frozen"; i.e., it should not
  // change even when the size of |this| changes.
  virtual const std::optional<PhysicalSize> FrozenFrameSize() const;

  // A transform mapping from the coordinate space of the embedded content
  // rendered by this object to the object's border-box.
  AffineTransform EmbeddedContentTransform() const;

 protected:
  PaintLayerType LayerTypeRequired() const override;

  PhysicalNaturalSizingInfo GetNaturalDimensions() const override;

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) final;
  void PaintReplaced(const PaintInfo&,
                     const PhysicalOffset& paint_offset) const override;
  CursorDirective GetCursor(const PhysicalOffset&, ui::Cursor&) const final;

  bool CanBeSelectionLeafInternal() const final {
    NOT_DESTROYED();
    return true;
  }

  HTMLFrameOwnerElement* GetFrameOwnerElement() const {
    NOT_DESTROYED();
    return To<HTMLFrameOwnerElement>(GetNode());
  }

 private:
  void WillBeDestroyed() final;

  bool NodeAtPointOverEmbeddedContentView(
      HitTestResult&,
      const HitTestLocation&,
      const PhysicalOffset& accumulated_offset,
      HitTestPhase);

  bool PointOverResizer(const HitTestResult&,
                        const HitTestLocation&,
                        const PhysicalOffset& accumulated_offset) const;

  void PropagateZoomFactor(double zoom_factor);
};

template <>
struct DowncastTraits<LayoutEmbeddedContent> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutEmbeddedContent();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_EMBEDDED_CONTENT_H_
