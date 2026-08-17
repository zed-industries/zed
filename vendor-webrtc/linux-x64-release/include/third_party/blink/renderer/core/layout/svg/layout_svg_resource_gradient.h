/*
 * Copyright (C) 2006 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2008 Eric Seidel <eric@webkit.org>
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_GRADIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_GRADIENT_H_

#include <memory>

#include "third_party/blink/renderer/core/layout/svg/layout_svg_resource_paint_server.h"
#include "third_party/blink/renderer/core/svg/svg_gradient_element.h"
#include "third_party/blink/renderer/platform/graphics/gradient.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

class AffineTransform;
class SVGLength;
struct GradientData;

class LayoutSVGResourceGradient : public LayoutSVGResourcePaintServer {
 public:
  explicit LayoutSVGResourceGradient(SVGGradientElement*);
  void Trace(Visitor*) const override;

  void RemoveAllClientsFromCache() final;
  bool RemoveClientFromCache(SVGResourceClient&) final;

  bool ApplyShader(const SVGResourceClient&,
                   const gfx::RectF& reference_box,
                   const AffineTransform* additional_transform,
                   const AutoDarkMode& auto_dark_mode,
                   cc::PaintFlags&) final;

  bool IsChildAllowed(LayoutObject* child, const ComputedStyle&) const final;

 protected:
  virtual const GradientAttributes& EnsureAttributes() const = 0;
  virtual scoped_refptr<Gradient> BuildGradient() const = 0;

  gfx::PointF ResolvePoint(SVGUnitTypes::SVGUnitType,
                           const SVGLength& x,
                           const SVGLength& y) const;
  float ResolveRadius(SVGUnitTypes::SVGUnitType type, const SVGLength& r) const;
  static Gradient::SpreadMethod PlatformSpreadMethodFromSVGType(
      SVGSpreadMethodType);

  mutable bool should_collect_gradient_attributes_ = true;

 private:
  std::unique_ptr<GradientData> BuildGradientData(
      const gfx::RectF& object_bounding_box) const;

  using GradientMap = HeapHashMap<Member<const SVGResourceClient>,
                                  std::unique_ptr<GradientData>>;
  GradientMap gradient_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_GRADIENT_H_
