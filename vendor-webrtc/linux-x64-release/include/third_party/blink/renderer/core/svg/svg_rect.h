/*
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RECT_H_

#include "third_party/blink/renderer/core/svg/properties/svg_property.h"
#include "third_party/blink/renderer/core/svg/svg_parsing_error.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class SVGRectTearOff;

class SVGRect final : public SVGPropertyBase {
 public:
  typedef SVGRectTearOff TearOffType;

  static SVGRect* CreateInvalid() {
    SVGRect* rect = MakeGarbageCollected<SVGRect>();
    rect->is_valid_ = false;
    return rect;
  }

  SVGRect() = default;
  SVGRect(float x, float y, float width, float height)
      : x_(x), y_(y), width_(width), height_(height) {}

  SVGRect* Clone() const;

  // Negative width_/height_ will be clamped to 0.
  gfx::RectF Rect() const { return gfx::RectF(x_, y_, width_, height_); }

  float X() const { return x_; }
  float Y() const { return y_; }
  float Width() const { return width_; }
  float Height() const { return height_; }
  void SetX(float f) { x_ = f; }
  void SetY(float f) { y_ = f; }
  void SetWidth(float f) { width_ = f; }
  void SetHeight(float f) { height_ = f; }

  String ValueAsString() const override;
  SVGParsingError SetValueAsString(const String&);

  void Add(const SVGPropertyBase*, const SVGElement*) override;
  void CalculateAnimatedValue(
      const SMILAnimationEffectParameters&,
      float percentage,
      unsigned repeat_count,
      const SVGPropertyBase* from,
      const SVGPropertyBase* to,
      const SVGPropertyBase* to_at_end_of_duration_value,
      const SVGElement* context_element) override;
  float CalculateDistance(const SVGPropertyBase* to,
                          const SVGElement* context_element) const override;

  static AnimatedPropertyType ClassType() { return kAnimatedRect; }
  AnimatedPropertyType GetType() const override { return ClassType(); }

 private:
  friend class SVGFitToViewBox;
  bool IsValid() const { return is_valid_; }

  template <typename CharType>
  SVGParsingError Parse(const CharType*& ptr, const CharType* end);
  void Set(float x, float y, float width, float height);
  void Add(float x, float y, float width, float height);

  bool is_valid_ = true;
  // Not using gfx::RectF because width_ and height_ can be negative.
  float x_ = 0;
  float y_ = 0;
  float width_ = 0;
  float height_ = 0;
};

template <>
struct DowncastTraits<SVGRect> {
  static bool AllowFrom(const SVGPropertyBase& value) {
    return value.GetType() == SVGRect::ClassType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RECT_H_
