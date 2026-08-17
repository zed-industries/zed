/*
 * Copyright (C) 2007 Eric Seidel <eric@webkit.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORM_DISTANCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORM_DISTANCE_H_

#include "third_party/blink/renderer/core/svg/svg_transform.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class AffineTransform;

class SVGTransformDistance {
  STACK_ALLOCATED();

 public:
  SVGTransformDistance();
  SVGTransformDistance(const SVGTransform* from_transform,
                       const SVGTransform* to_transform);

  SVGTransformDistance ScaledDistance(float scale_factor) const;
  SVGTransform* AddToSVGTransform(const SVGTransform*) const;

  static SVGTransform* AddSVGTransforms(const SVGTransform*,
                                        const SVGTransform*,
                                        unsigned repeat_count = 1);

  float Distance() const;

 private:
  SVGTransformDistance(SVGTransformType,
                       float angle,
                       float cx,
                       float cy,
                       const AffineTransform&);

  SVGTransformType transform_type_;
  float angle_;
  float cx_;
  float cy_;
  AffineTransform
      transform_;  // for storing scale, translation or matrix transforms
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORM_DISTANCE_H_
