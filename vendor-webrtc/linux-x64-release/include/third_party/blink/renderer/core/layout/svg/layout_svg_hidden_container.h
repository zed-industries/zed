/*
 * Copyright (C) 2007 Eric Seidel <eric@webkit.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_HIDDEN_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_HIDDEN_CONTAINER_H_

#include "third_party/blink/renderer/core/layout/svg/layout_svg_container.h"

namespace blink {

class SVGElement;

// This class is for containers which are never drawn, but do need to support
// style <defs>, <linearGradient>, <radialGradient> are all good examples
class LayoutSVGHiddenContainer : public LayoutSVGContainer {
 public:
  explicit LayoutSVGHiddenContainer(SVGElement*);

  void SetNeedsTransformUpdate() override { NOT_DESTROYED(); }

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGHiddenContainer";
  }

 protected:
  SVGLayoutResult UpdateSVGLayout(const SVGLayoutInfo&) override;

  bool IsSVGHiddenContainer() const final {
    NOT_DESTROYED();
    return true;
  }

 private:
  // LayoutSVGHiddenContainer paints nothing.
  void Paint(const PaintInfo&) const final { NOT_DESTROYED(); }
  gfx::RectF VisualRectInLocalSVGCoordinates() const final {
    NOT_DESTROYED();
    return gfx::RectF();
  }
  void QuadsInAncestorInternal(Vector<gfx::QuadF>&,
                               const LayoutBoxModelObject* ancestor,
                               MapCoordinatesFlags) const final {
    NOT_DESTROYED();
  }

  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation&,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase) final;
};

template <>
struct DowncastTraits<LayoutSVGHiddenContainer> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsSVGHiddenContainer();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_HIDDEN_CONTAINER_H_
