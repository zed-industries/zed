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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PATTERN_ATTRIBUTES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PATTERN_ATTRIBUTES_H_

#include "third_party/blink/renderer/core/svg/svg_length.h"
#include "third_party/blink/renderer/core/svg/svg_preserve_aspect_ratio.h"
#include "third_party/blink/renderer/core/svg/svg_unit_types.h"
#include "third_party/blink/renderer/platform/transforms/affine_transform.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class SVGPatternElement;

class PatternAttributes final {
  DISALLOW_NEW();

 public:
  PatternAttributes()
      : x_(nullptr),
        y_(nullptr),
        width_(nullptr),
        height_(nullptr),
        preserve_aspect_ratio_(nullptr),
        pattern_content_element_(nullptr),
        view_box_set_(false),
        pattern_units_set_(false),
        pattern_content_units_set_(false),
        pattern_transform_set_(false) {}

  const SVGLength* X() const { return x_.Get(); }
  const SVGLength* Y() const { return y_.Get(); }
  const SVGLength* Width() const { return width_.Get(); }
  const SVGLength* Height() const { return height_.Get(); }
  gfx::RectF ViewBox() const { return view_box_; }
  const SVGPreserveAspectRatio* PreserveAspectRatio() const {
    return preserve_aspect_ratio_.Get();
  }
  SVGUnitTypes::SVGUnitType PatternUnits() const { return pattern_units_; }
  SVGUnitTypes::SVGUnitType PatternContentUnits() const {
    return pattern_content_units_;
  }
  const AffineTransform& PatternTransform() const { return pattern_transform_; }
  const SVGPatternElement* PatternContentElement() const {
    return pattern_content_element_.Get();
  }

  void SetX(const SVGLength* value) { x_ = value; }
  void SetY(const SVGLength* value) { y_ = value; }
  void SetWidth(const SVGLength* value) { width_ = value; }
  void SetHeight(const SVGLength* value) { height_ = value; }
  void SetViewBox(const gfx::RectF& value) {
    view_box_ = value;
    view_box_set_ = true;
  }
  void SetPreserveAspectRatio(const SVGPreserveAspectRatio* value) {
    preserve_aspect_ratio_ = value;
  }
  void SetPatternUnits(SVGUnitTypes::SVGUnitType value) {
    pattern_units_ = value;
    pattern_units_set_ = true;
  }
  void SetPatternContentUnits(SVGUnitTypes::SVGUnitType value) {
    pattern_content_units_ = value;
    pattern_content_units_set_ = true;
  }
  void SetPatternTransform(const AffineTransform& value) {
    pattern_transform_ = value;
    pattern_transform_set_ = true;
  }
  void SetPatternContentElement(const SVGPatternElement& value) {
    pattern_content_element_ = value;
  }

  bool HasX() const { return x_ != nullptr; }
  bool HasY() const { return y_ != nullptr; }
  bool HasWidth() const { return width_ != nullptr; }
  bool HasHeight() const { return height_ != nullptr; }
  bool HasViewBox() const { return view_box_set_; }
  bool HasPreserveAspectRatio() const {
    return preserve_aspect_ratio_ != nullptr;
  }
  bool HasPatternUnits() const { return pattern_units_set_; }
  bool HasPatternContentUnits() const { return pattern_content_units_set_; }
  bool HasPatternTransform() const { return pattern_transform_set_; }
  bool HasPatternContentElement() const {
    return pattern_content_element_ != nullptr;
  }

  void Trace(Visitor* visitor) const {
    visitor->Trace(x_);
    visitor->Trace(y_);
    visitor->Trace(width_);
    visitor->Trace(height_);
    visitor->Trace(preserve_aspect_ratio_);
    visitor->Trace(pattern_content_element_);
  }

 private:
  // Properties
  Member<const SVGLength> x_;
  Member<const SVGLength> y_;
  Member<const SVGLength> width_;
  Member<const SVGLength> height_;
  gfx::RectF view_box_;
  Member<const SVGPreserveAspectRatio> preserve_aspect_ratio_;
  SVGUnitTypes::SVGUnitType pattern_units_ =
      SVGUnitTypes::kSvgUnitTypeObjectboundingbox;
  SVGUnitTypes::SVGUnitType pattern_content_units_ =
      SVGUnitTypes::kSvgUnitTypeUserspaceonuse;
  AffineTransform pattern_transform_;
  Member<const SVGPatternElement> pattern_content_element_;

  // Property states
  bool view_box_set_ : 1;
  bool pattern_units_set_ : 1;
  bool pattern_content_units_set_ : 1;
  bool pattern_transform_set_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PATTERN_ATTRIBUTES_H_
