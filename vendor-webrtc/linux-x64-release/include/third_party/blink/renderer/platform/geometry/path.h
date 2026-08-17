/*
 * Copyright (C) 2003, 2006, 2009 Apple Inc. All rights reserved.
 *               2006 Rob Buis <buis@kde.org>
 * Copyright (C) 2007-2008 Torch Mobile, Inc.
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PATH_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PATH_H_

#include "base/memory/raw_span.h"
#include "third_party/blink/renderer/platform/geometry/float_rounded_rect.h"
#include "third_party/blink/renderer/platform/geometry/path_types.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/skia/include/core/SkPath.h"
#include "third_party/skia/include/core/SkPathMeasure.h"
#include "ui/gfx/geometry/transform.h"

namespace gfx {
class PointF;
class QuadF;
class RectF;
}  // namespace gfx

namespace blink {

class AffineTransform;
class ContouredRect;
class StrokeData;

enum PathElementType {
  kPathElementMoveToPoint,          // The points member will contain 1 value.
  kPathElementAddLineToPoint,       // The points member will contain 1 value.
  kPathElementAddQuadCurveToPoint,  // The points member will contain 2 values.
  kPathElementAddCurveToPoint,      // The points member will contain 3 values.
  kPathElementCloseSubpath          // The points member will contain no values.
};

// The points in the structure are the same as those that would be used with the
// add... method. For example, a line returns the endpoint, while a cubic
// returns two tangent points and the endpoint.
struct PathElement {
  PathElementType type;
  base::raw_span<gfx::PointF> points;
};

// Result structure from Path::PointAndNormalAtLength() (and similar).
struct PointAndTangent {
  gfx::PointF point;
  float tangent_in_degrees = 0;
};

typedef void (*PathApplierFunction)(void* info, const PathElement&);

class PLATFORM_EXPORT Path {
  USING_FAST_MALLOC(Path);

 public:
  Path();
  ~Path();

  Path(const Path&);
  Path(const SkPath&);
  Path& operator=(const Path&);
  Path& operator=(const SkPath&);
  bool operator==(const Path&) const;
  bool operator!=(const Path& other) const { return !(*this == other); }

  bool Contains(const gfx::PointF&) const;
  bool Contains(const gfx::PointF&, WindRule) const;

  bool Intersects(const gfx::QuadF&) const;
  bool Intersects(const gfx::QuadF&, WindRule) const;

  // Determine if the path's stroke contains the point.  The transform is used
  // only to determine the precision factor when analyzing the stroke, so that
  // we return accurate results in high-zoom scenarios.
  bool StrokeContains(const gfx::PointF&,
                      const StrokeData&,
                      const AffineTransform&) const;
  SkPath StrokePath(const StrokeData&, const AffineTransform&) const;

  // Tight Bounding calculation is very expensive, but it guarantees the strict
  // bounding box. It's always included in BoundingRect. For a logical bounding
  // box (used for clipping or damage) BoundingRect is recommended.
  gfx::RectF TightBoundingRect() const;
  gfx::RectF BoundingRect() const;
  gfx::RectF StrokeBoundingRect(const StrokeData&) const;

  float length() const;
  gfx::PointF PointAtLength(float length) const;
  PointAndTangent PointAndNormalAtLength(float length) const;

  // Helper for computing a sequence of positions and normals (normal angles) on
  // a path. The best possible access pattern will be one where the |length|
  // value is strictly increasing. For other access patterns, performance will
  // vary depending on curvature and number of segments, but should never be
  // worse than that of the state-less method on Path.
  class PLATFORM_EXPORT PositionCalculator {
    USING_FAST_MALLOC(PositionCalculator);

   public:
    explicit PositionCalculator(const Path&);
    PositionCalculator(const PositionCalculator&) = delete;
    PositionCalculator& operator=(const PositionCalculator&) = delete;

    PointAndTangent PointAndNormalAtLength(float length);

   private:
    SkPath path_;
    SkPathMeasure path_measure_;
    SkScalar accumulated_length_;
  };

  bool IsEmpty() const;
  bool IsClosed() const;
  bool IsLine() const;

  const SkPath& GetSkPath() const { return path_; }

  void Apply(void* info, PathApplierFunction) const;

  // Utility factories for simple shapes.
  static Path MakeRect(const gfx::RectF&);
  // Use this form if the rect is defined by locations of a pair of opposite
  // corners, where |origin| may not be the top-left corner.
  static Path MakeRect(const gfx::PointF& origin,
                       const gfx::PointF& opposite_point);
  static Path MakeRoundedRect(const FloatRoundedRect&);
  static Path MakeContouredRect(const ContouredRect&);
  static Path MakeEllipse(const gfx::PointF& center,
                          float radius_x,
                          float radius_y);

 private:
  SkPath StrokePath(const StrokeData&, float stroke_precision) const;

  SkPath path_;
};

// Only used for DCHECKs
PLATFORM_EXPORT bool EllipseIsRenderable(float start_angle, float end_angle);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PATH_H_
