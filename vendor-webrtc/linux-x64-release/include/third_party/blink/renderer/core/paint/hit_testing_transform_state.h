/*
 * Copyright (C) 2011 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HIT_TESTING_TRANSFORM_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HIT_TESTING_TRANSFORM_STATE_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/graphics/paint/geometry_mapper.h"
#include "third_party/blink/renderer/platform/transforms/affine_transform.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/quad_f.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class TransformPaintPropertyNode;

class HitTestingTransformState {
  STACK_ALLOCATED();

 public:
  HitTestingTransformState(const gfx::PointF& p,
                           const gfx::QuadF& quad,
                           const PhysicalRect& area)
      : last_planar_point_(p),
        last_planar_quad_(quad),
        // If the hit test area is "infinite", then mapping through the
        // transform may produce an area that no longer contains the content
        // (it gets shifted up and left). Preserve the infinite area in that
        // case.
        last_planar_area_(area != PhysicalRect(InfiniteIntRect())
                              ? std::make_optional(gfx::QuadF(gfx::RectF(area)))
                              : std::nullopt) {}

  HitTestingTransformState(const HitTestingTransformState&) = default;
  HitTestingTransformState& operator=(const HitTestingTransformState&) =
      default;

  void Translate(const gfx::Vector2dF&);
  void ApplyTransform(const TransformPaintPropertyNode&);
  void ApplyTransform(const gfx::Transform&);

  gfx::PointF MappedPoint() const;
  gfx::QuadF MappedQuad() const;
  PhysicalRect BoundsOfMappedQuad() const;
  PhysicalRect BoundsOfMappedArea() const;
  void Flatten();
  const gfx::Transform AccumulatedTransform() const {
    return accumulated_transform_;
  }

 private:
  PhysicalRect BoundsOfMappedQuadInternal(const gfx::QuadF&) const;

  gfx::PointF last_planar_point_;
  gfx::QuadF last_planar_quad_;
  std::optional<gfx::QuadF> last_planar_area_;
  gfx::Transform accumulated_transform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HIT_TESTING_TRANSFORM_STATE_H_
