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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_POLYGON_SHAPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_POLYGON_SHAPE_H_

#include <memory>
#include "third_party/blink/renderer/core/layout/shapes/shape.h"
#include "third_party/blink/renderer/core/layout/shapes/shape_interval.h"
#include "third_party/blink/renderer/platform/geometry/float_polygon.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

class OffsetPolygonEdge final : public VertexPair {
  DISALLOW_NEW();

 public:
  OffsetPolygonEdge(const FloatPolygonEdge& edge, const gfx::Vector2dF& offset)
      : vertex1_(edge.Vertex1() + offset), vertex2_(edge.Vertex2() + offset) {}

  const gfx::PointF& Vertex1() const override { return vertex1_; }
  const gfx::PointF& Vertex2() const override { return vertex2_; }

  bool IsWithinYRange(float y1, float y2) const {
    return y1 <= MinY() && y2 >= MaxY();
  }
  bool OverlapsYRange(float y1, float y2) const {
    return y2 >= MinY() && y1 <= MaxY();
  }
  float XIntercept(float y) const;
  FloatShapeInterval ClippedEdgeXRange(float y1, float y2) const;

 private:
  gfx::PointF vertex1_;
  gfx::PointF vertex2_;
};

class PolygonShape final : public Shape {
 public:
  PolygonShape(Vector<gfx::PointF> vertices, WindRule fill_rule)
      : Shape(), polygon_(std::move(vertices)) {}
  PolygonShape(const PolygonShape&) = delete;
  PolygonShape& operator=(const PolygonShape&) = delete;

  LogicalRect ShapeMarginLogicalBoundingBox() const override;
  bool IsEmpty() const override { return polygon_.IsEmpty(); }
  LineSegment GetExcludedInterval(LayoutUnit logical_top,
                                  LayoutUnit logical_height) const override;
  void BuildDisplayPaths(DisplayPaths&) const override;

 private:
  FloatPolygon polygon_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_POLYGON_SHAPE_H_
