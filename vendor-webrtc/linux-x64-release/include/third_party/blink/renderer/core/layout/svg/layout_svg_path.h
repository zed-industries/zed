/*
 * Copyright (C) 2004, 2005, 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005 Rob Buis <buis@kde.org>
 * Copyright (C) 2005 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2006 Apple Computer, Inc
 * Copyright (C) 2009 Google, Inc.
 * Copyright (C) 2011 Renata Hodovan <reni@webkit.org>
 * Copyright (C) 2011 University of Szeged
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_PATH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_PATH_H_

#include "third_party/blink/renderer/core/layout/svg/layout_svg_shape.h"

namespace blink {

class LayoutSVGPath final : public LayoutSVGShape {
 public:
  explicit LayoutSVGPath(SVGGeometryElement*);
  ~LayoutSVGPath() override;

  const Vector<MarkerPosition>* MarkerPositions() const override {
    NOT_DESTROYED();
    return &marker_positions_;
  }

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGPath";
  }

 private:
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
  void WillBeDestroyed() override;

  bool CalculateGeometryDependsOnViewport() const;
  gfx::RectF UpdateShapeFromElement() override;

  const StylePath* GetStylePath() const;
  void UpdateMarkerPositions();
  void UpdateMarkerBounds() override;

  Vector<MarkerPosition> marker_positions_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_PATH_H_
