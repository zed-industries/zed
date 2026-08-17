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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_SHAPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_SHAPE_H_

#include <memory>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_rect.h"
#include "third_party/blink/renderer/core/style/basic_shapes.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/geometry/path.h"
#include "third_party/blink/renderer/platform/graphics/image_orientation.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"

namespace blink {

class FloatRoundedRect;
struct LogicalSize;

struct LineSegment {
  STACK_ALLOCATED();

 public:
  LineSegment() : logical_left(0), logical_right(0), is_valid(false) {}

  LineSegment(float logical_left, float logical_right)
      : logical_left(logical_left),
        logical_right(logical_right),
        is_valid(true) {}

  LayoutUnit logical_left;
  LayoutUnit logical_right;
  bool is_valid;
};

// A representation of a BasicShape that enables layout code to determine how to
// break a line up into segments that will fit within or around a shape. The
// line is defined by a pair of logical Y coordinates and the computed segments
// are returned as pairs of logical X coordinates. The BasicShape itself is
// defined in physical coordinates.

class CORE_EXPORT Shape {
  USING_FAST_MALLOC(Shape);

 public:
  struct DisplayPaths {
    STACK_ALLOCATED();

   public:
    Path shape;
    Path margin_shape;
  };
  static std::unique_ptr<Shape> CreateShape(const BasicShape*,
                                            const LogicalSize& logical_box_size,
                                            WritingMode,
                                            float margin);
  static std::unique_ptr<Shape> CreateRasterShape(
      Image*,
      float threshold,
      int content_block_size,
      const gfx::Rect& image_rect,
      const gfx::Rect& margin_logical_rect,
      WritingMode,
      float margin,
      RespectImageOrientationEnum);
  static std::unique_ptr<Shape> CreateLayoutBoxShape(const FloatRoundedRect&,
                                                     WritingMode,
                                                     float margin);

  virtual ~Shape() = default;

  virtual LogicalRect ShapeMarginLogicalBoundingBox() const = 0;
  virtual bool IsEmpty() const = 0;
  virtual LineSegment GetExcludedInterval(LayoutUnit logical_top,
                                          LayoutUnit logical_height) const = 0;

  bool LineOverlapsShapeMarginBounds(LayoutUnit line_top,
                                     LayoutUnit line_height) const {
    return LineOverlapsBoundingBox(line_top, line_height,
                                   ShapeMarginLogicalBoundingBox());
  }
  virtual void BuildDisplayPaths(DisplayPaths&) const = 0;

  void SetShapeMarginForTesting(float margin) { margin_ = margin; }

 protected:
  float ShapeMargin() const { return margin_; }

 private:
  static std::unique_ptr<Shape> CreateEmptyRasterShape(WritingMode,
                                                       float margin);

  bool LineOverlapsBoundingBox(LayoutUnit line_top,
                               LayoutUnit line_height,
                               const LogicalRect& rect) const {
    if (rect.IsEmpty())
      return false;
    const LayoutUnit rect_line_top = rect.offset.block_offset;
    return (line_top < rect.BlockEndOffset() &&
            line_top + line_height > rect_line_top) ||
           (!line_height && line_top == rect_line_top);
  }

  WritingMode writing_mode_ = WritingMode::kHorizontalTb;
  float margin_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_SHAPE_H_
