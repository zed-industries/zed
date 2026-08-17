/*
 * Copyright (C) Research In Motion Limited 2009-2010. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_PAINT_SERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_PAINT_SERVER_H_

#include "cc/paint/paint_flags.h"
#include "third_party/blink/renderer/core/layout/svg/layout_svg_resource_container.h"

namespace blink {

struct AutoDarkMode;

class LayoutSVGResourcePaintServer : public LayoutSVGResourceContainer {
 public:
  explicit LayoutSVGResourcePaintServer(SVGElement* element)
      : LayoutSVGResourceContainer(element) {}

  virtual bool ApplyShader(const SVGResourceClient&,
                           const gfx::RectF& reference_box,
                           const AffineTransform* additional_transform,
                           const AutoDarkMode& auto_dark_mode,
                           cc::PaintFlags&) = 0;

 protected:
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
};

template <>
struct DowncastTraits<LayoutSVGResourcePaintServer> {
  static bool AllowFrom(const LayoutSVGResourceContainer& container) {
    return container.IsSVGPaintServer();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_PAINT_SERVER_H_
