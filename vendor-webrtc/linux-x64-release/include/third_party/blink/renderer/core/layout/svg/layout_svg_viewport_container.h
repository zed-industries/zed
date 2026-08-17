/*
 * Copyright (C) 2004, 2005, 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2007 Rob Buis <buis@kde.org>
 * Copyright (C) 2009 Google, Inc.
 * Copyright (C) 2009 Apple Inc. All rights reserved.
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_VIEWPORT_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_VIEWPORT_CONTAINER_H_

#include "third_party/blink/renderer/core/layout/svg/layout_svg_container.h"

namespace blink {

class SVGSVGElement;

// This is used for non-root <svg> elements which are SVGTransformable thus we
// inherit from LayoutSVGContainer instead of LayoutSVGTransformableContainer.
class LayoutSVGViewportContainer final : public LayoutSVGContainer {
 public:
  explicit LayoutSVGViewportContainer(SVGSVGElement*);
  gfx::RectF Viewport() const {
    NOT_DESTROYED();
    return viewport_;
  }

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGViewportContainer";
  }

  AffineTransform LocalToSVGParentTransform() const override {
    NOT_DESTROYED();
    return local_to_parent_transform_;
  }

  AffineTransform LocalSVGTransform() const override {
    NOT_DESTROYED();
    return local_transform_;
  }

  gfx::RectF ViewBoxRect() const;

  void IntersectChildren(HitTestResult&, const HitTestLocation&) const;

  AffineTransform ComputeViewboxTransform() const;

 private:
  bool IsSVGViewportContainer() const final {
    NOT_DESTROYED();
    return true;
  }

  SVGLayoutResult UpdateSVGLayout(const SVGLayoutInfo&) override;

  SVGTransformChange UpdateLocalTransform(
      const gfx::RectF& reference_box) override;

  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation&,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase) final;

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  gfx::RectF viewport_;
  mutable AffineTransform local_to_parent_transform_;
  // TODO: Inherit `LayoutSVGViewportContainer` from
  // `LayoutSVGTransformableContainer so we can remove this member.
  mutable AffineTransform local_transform_;
};

template <>
struct DowncastTraits<LayoutSVGViewportContainer> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsSVGViewportContainer();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_VIEWPORT_CONTAINER_H_
