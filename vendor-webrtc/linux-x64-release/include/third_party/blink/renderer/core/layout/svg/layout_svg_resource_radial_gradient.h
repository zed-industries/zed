/*
 * Copyright (C) 2006 Nikolas Zimmermann <zimmermann@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_RADIAL_GRADIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_RADIAL_GRADIENT_H_

#include "third_party/blink/renderer/core/layout/svg/layout_svg_resource_gradient.h"
#include "third_party/blink/renderer/core/svg/radial_gradient_attributes.h"

namespace blink {

class SVGRadialGradientElement;

class LayoutSVGResourceRadialGradient final : public LayoutSVGResourceGradient {
 public:
  explicit LayoutSVGResourceRadialGradient(SVGRadialGradientElement*);
  ~LayoutSVGResourceRadialGradient() override;
  void Trace(Visitor*) const override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGResourceRadialGradient";
  }

  static const LayoutSVGResourceType kResourceType =
      kRadialGradientResourceType;
  LayoutSVGResourceType ResourceType() const override {
    NOT_DESTROYED();
    return kResourceType;
  }

  gfx::PointF CenterPoint(const RadialGradientAttributes&) const;
  gfx::PointF FocalPoint(const RadialGradientAttributes&) const;
  float Radius(const RadialGradientAttributes&) const;
  float FocalRadius(const RadialGradientAttributes&) const;

 private:
  const GradientAttributes& EnsureAttributes() const override;
  scoped_refptr<Gradient> BuildGradient() const override;

  mutable RadialGradientAttributes attributes_;
};

template <>
struct DowncastTraits<LayoutSVGResourceRadialGradient> {
  static bool AllowFrom(const LayoutSVGResourceContainer& container) {
    return container.ResourceType() == kRadialGradientResourceType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_RADIAL_GRADIENT_H_
