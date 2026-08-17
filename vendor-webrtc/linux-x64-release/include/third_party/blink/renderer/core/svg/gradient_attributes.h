/*
 * Copyright (C) 2006 Nikolas Zimmermann <zimmermann@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRADIENT_ATTRIBUTES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRADIENT_ATTRIBUTES_H_

#include "third_party/blink/renderer/core/svg/svg_gradient_element.h"
#include "third_party/blink/renderer/core/svg/svg_unit_types.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/size_assertions.h"

namespace blink {

struct GradientAttributes {
  DISALLOW_NEW();
  GradientAttributes()
      : spread_method_(kSVGSpreadMethodPad),
        gradient_units_(SVGUnitTypes::kSvgUnitTypeObjectboundingbox),
        spread_method_set_(false),
        gradient_units_set_(false),
        gradient_transform_set_(false) {}

  SVGSpreadMethodType SpreadMethod() const {
    return static_cast<SVGSpreadMethodType>(spread_method_);
  }
  SVGUnitTypes::SVGUnitType GradientUnits() const {
    return static_cast<SVGUnitTypes::SVGUnitType>(gradient_units_);
  }
  const AffineTransform& GradientTransform() const {
    return gradient_transform_;
  }
  const Vector<Gradient::ColorStop>& Stops() const { return stops_; }

  void SetSpreadMethod(SVGSpreadMethodType value) {
    spread_method_ = value;
    spread_method_set_ = true;
  }

  void SetGradientUnits(SVGUnitTypes::SVGUnitType unit_type) {
    gradient_units_ = unit_type;
    gradient_units_set_ = true;
  }

  void SetGradientTransform(const AffineTransform& value) {
    gradient_transform_ = value;
    gradient_transform_set_ = true;
  }

  void SetStops(Vector<Gradient::ColorStop>&& value) {
    stops_ = std::move(value);
  }

  bool HasSpreadMethod() const { return spread_method_set_; }
  bool HasGradientUnits() const { return gradient_units_set_; }
  bool HasGradientTransform() const { return gradient_transform_set_; }
  bool HasStops() const { return !stops_.empty(); }

 private:
  // Properties
  AffineTransform gradient_transform_;
  Vector<Gradient::ColorStop> stops_;

  unsigned spread_method_ : 2;
  unsigned gradient_units_ : 2;

  // Property states
  unsigned spread_method_set_ : 1;
  unsigned gradient_units_set_ : 1;
  unsigned gradient_transform_set_ : 1;
};

struct SameSizeAsGradientAttributes {
  DISALLOW_NEW();
  AffineTransform a;
  Vector<Gradient::ColorStop> b;
  unsigned c : 7;
};

ASSERT_SIZE(GradientAttributes, SameSizeAsGradientAttributes);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_GRADIENT_ATTRIBUTES_H_
