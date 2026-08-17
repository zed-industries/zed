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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_RADIAL_GRADIENT_ATTRIBUTES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_RADIAL_GRADIENT_ATTRIBUTES_H_

#include "third_party/blink/renderer/core/svg/gradient_attributes.h"
#include "third_party/blink/renderer/core/svg/svg_length.h"

namespace blink {

struct RadialGradientAttributes final : GradientAttributes {
  DISALLOW_NEW();

 public:
  RadialGradientAttributes()
      : cx_(nullptr),
        cy_(nullptr),
        r_(nullptr),
        fx_(nullptr),
        fy_(nullptr),
        fr_(nullptr) {}

  const SVGLength* Cx() const { return cx_.Get(); }
  const SVGLength* Cy() const { return cy_.Get(); }
  const SVGLength* R() const { return r_.Get(); }
  const SVGLength* Fx() const { return fx_.Get(); }
  const SVGLength* Fy() const { return fy_.Get(); }
  const SVGLength* Fr() const { return fr_.Get(); }

  void SetCx(const SVGLength* value) { cx_ = value; }
  void SetCy(const SVGLength* value) { cy_ = value; }
  void SetR(const SVGLength* value) { r_ = value; }
  void SetFx(const SVGLength* value) { fx_ = value; }
  void SetFy(const SVGLength* value) { fy_ = value; }
  void SetFr(const SVGLength* value) { fr_ = value; }

  bool HasCx() const { return cx_ != nullptr; }
  bool HasCy() const { return cy_ != nullptr; }
  bool HasR() const { return r_ != nullptr; }
  bool HasFx() const { return fx_ != nullptr; }
  bool HasFy() const { return fy_ != nullptr; }
  bool HasFr() const { return fr_ != nullptr; }

  void Trace(Visitor* visitor) const {
    visitor->Trace(cx_);
    visitor->Trace(cy_);
    visitor->Trace(r_);
    visitor->Trace(fx_);
    visitor->Trace(fy_);
    visitor->Trace(fr_);
  }

 private:
  // Properties
  Member<const SVGLength> cx_;
  Member<const SVGLength> cy_;
  Member<const SVGLength> r_;
  Member<const SVGLength> fx_;
  Member<const SVGLength> fy_;
  Member<const SVGLength> fr_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_RADIAL_GRADIENT_ATTRIBUTES_H_
