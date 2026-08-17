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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_LINEAR_GRADIENT_ATTRIBUTES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_LINEAR_GRADIENT_ATTRIBUTES_H_

#include "third_party/blink/renderer/core/svg/gradient_attributes.h"
#include "third_party/blink/renderer/core/svg/svg_length.h"

namespace blink {

struct LinearGradientAttributes : GradientAttributes {
  DISALLOW_NEW();

 public:
  LinearGradientAttributes()
      : x1_(nullptr), y1_(nullptr), x2_(nullptr), y2_(nullptr) {}

  const SVGLength* X1() const { return x1_.Get(); }
  const SVGLength* Y1() const { return y1_.Get(); }
  const SVGLength* X2() const { return x2_.Get(); }
  const SVGLength* Y2() const { return y2_.Get(); }

  void SetX1(const SVGLength* value) { x1_ = value; }
  void SetY1(const SVGLength* value) { y1_ = value; }
  void SetX2(const SVGLength* value) { x2_ = value; }
  void SetY2(const SVGLength* value) { y2_ = value; }

  bool HasX1() const { return x1_ != nullptr; }
  bool HasY1() const { return y1_ != nullptr; }
  bool HasX2() const { return x2_ != nullptr; }
  bool HasY2() const { return y2_ != nullptr; }

  void Trace(Visitor* visitor) const {
    visitor->Trace(x1_);
    visitor->Trace(y1_);
    visitor->Trace(x2_);
    visitor->Trace(y2_);
  }

 private:
  // Properties
  Member<const SVGLength> x1_;
  Member<const SVGLength> y1_;
  Member<const SVGLength> x2_;
  Member<const SVGLength> y2_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_LINEAR_GRADIENT_ATTRIBUTES_H_
