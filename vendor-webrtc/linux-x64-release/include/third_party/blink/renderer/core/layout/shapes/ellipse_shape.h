/*
 * Copyright (C) 2012 Adobe Systems Incorporated. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer.
 * 2. Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials
 *    provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_ELLIPSE_SHAPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_ELLIPSE_SHAPE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/shapes/shape.h"
#include "ui/gfx/geometry/point_f.h"

namespace blink {

class CORE_EXPORT EllipseShape final : public Shape {
 public:
  EllipseShape(const gfx::PointF& center,
               float radius_x,
               float radius_y,
               WritingMode writing_mode = WritingMode::kHorizontalTb)
      : center_(center),
        radius_x_(radius_x),
        radius_y_(radius_y),
        writing_mode_(writing_mode) {
    DCHECK_GE(radius_x, 0);
    DCHECK_GE(radius_y, 0);
  }

  LogicalRect ShapeMarginLogicalBoundingBox() const override;
  bool IsEmpty() const override { return !radius_x_ || !radius_y_; }
  LineSegment GetExcludedInterval(LayoutUnit logical_top,
                                  LayoutUnit logical_height) const override;
  void BuildDisplayPaths(DisplayPaths&) const override;

 private:
  // Returns a pair of the inline-axis radius and the block-axis radius.
  // They contains the ShapeMargin() value.
  std::pair<float, float> InlineAndBlockRadiiIncludingMargin() const;

  // The center point in a logical coordinate.
  gfx::PointF center_;
  // Horizontal radius.
  float radius_x_;
  // Vertical radius.
  float radius_y_;

  const WritingMode writing_mode_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_ELLIPSE_SHAPE_H_
