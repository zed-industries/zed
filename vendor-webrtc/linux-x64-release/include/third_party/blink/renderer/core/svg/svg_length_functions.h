/*
 * Copyright (C) Research In Motion Limited 2011. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LENGTH_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LENGTH_FUNCTIONS_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/point_f.h"

namespace gfx {
class SizeF;
class Vector2dF;
}  // namespace gfx

namespace blink {

class ComputedStyle;
class LayoutObject;
class Length;
class SVGElement;
class UnzoomedLength;

enum class SVGLengthMode { kWidth, kHeight, kOther };

class SVGViewportResolver {
  STACK_ALLOCATED();

 public:
  explicit SVGViewportResolver(const LayoutObject* context)
      : context_object_(context) {}
  explicit SVGViewportResolver(const LayoutObject& context)
      : context_object_(&context) {}
  explicit SVGViewportResolver(const SVGElement& context);

  gfx::SizeF ResolveViewport() const;
  float ViewportDimension(SVGLengthMode) const;

 private:
  const LayoutObject* context_object_;
};

float ValueForLength(const Length& length, float zoom, float dimension);
float ValueForLength(const Length& length,
                     const ComputedStyle&,
                     float dimension);
float ValueForLength(const Length&,
                     const SVGViewportResolver&,
                     float zoom,
                     SVGLengthMode mode);
float ValueForLength(const Length&,
                     const SVGViewportResolver&,
                     const ComputedStyle&,
                     SVGLengthMode = SVGLengthMode::kOther);
float ValueForLength(const UnzoomedLength&,
                     const SVGViewportResolver&,
                     SVGLengthMode = SVGLengthMode::kOther);

gfx::Vector2dF VectorForLengthPair(const Length& x_length,
                                   const Length& y_length,
                                   float zoom,
                                   const gfx::SizeF& viewport_size);
gfx::Vector2dF VectorForLengthPair(const Length& x_length,
                                   const Length& y_length,
                                   const SVGViewportResolver&,
                                   const ComputedStyle&);

inline gfx::PointF PointForLengthPair(
    const Length& x_length,
    const Length& y_length,
    const SVGViewportResolver& viewport_resolver,
    const ComputedStyle& style) {
  return gfx::PointAtOffsetFromOrigin(
      VectorForLengthPair(x_length, y_length, viewport_resolver, style));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LENGTH_FUNCTIONS_H_
