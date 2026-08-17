// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PATH_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PATH_BUILDER_H_

#include <optional>

#include "third_party/blink/renderer/platform/geometry/path.h"
#include "third_party/blink/renderer/platform/geometry/path_types.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/skia/include/core/SkPath.h"

namespace gfx {

class RectF;
class PointF;
class Vector2dF;

}  // namespace gfx

namespace blink {

class AffineTransform;
class ContouredRect;
class FloatRoundedRect;

// A helper for building immutable Paths.
//
// The optimal usage pattern maintains a clear separation between the build
// (mutable) phase, and the consumption (immutable) phase:
//
//   // build phase (mutable path)
//   PathBuilder builder;
//   builder.MoveTo(...);
//   builder.LineTo(...);
//   ...
//
//   // finalize a const/immutable path
//   const Path path = builder.Finalize();
//   DrawPath(path);
//   ...
//
// For the hopefully rare cases where the builder must be long-lived and access
// to the intermediate Path form is required, use CurrentPath():
//
//   PathBuilder builder;
//   builder.LineTo(...);
//   DrawPath(builder.CurrentPath());
//   builder.LineTo(...);
//   DrawPath(builder.CurrentPath());
//   ...

class PLATFORM_EXPORT PathBuilder {
  DISALLOW_NEW();

 public:
  PathBuilder();

  // This constructor makes a copy, and should be avoided if possible.  It is
  // mostly used while the code base is being updated, and ideally it should go
  // away when the conversion is complete.
  explicit PathBuilder(const Path&);
  ~PathBuilder();

  // Construct a path from the accumulated path data, and reset internal state.
  Path Finalize();

  // Access a Path based on the current path data, allowing further mutations.
  // Avoid if possible (use Finalize instead).
  // TODO(crbug.com/378688986): evaluate whether the cached path value is needed
  // once the conversion is complete.
  const Path& CurrentPath() const;
  // Gets the current point of the current path, which is conceptually the final
  // point reached by the path so far. Note the Path can be empty
  // (isEmpty() == true) and still have a current point.
  std::optional<gfx::PointF> CurrentPoint() const;

  bool IsEmpty() const { return builder_.isEmpty(); }
  void Reset();
  // Specify whether this path is volatile. Temporary paths that are discarded
  // or modified after use should be marked as volatile. This is a hint to the
  // device to not cache this path.
  void SetIsVolatile(bool is_volatile) { builder_.setIsVolatile(is_volatile); }

  gfx::RectF BoundingRect() const;

  PathBuilder& MoveTo(const gfx::PointF&);
  PathBuilder& Close();

  // These methods append to the current subpath (contour).
  PathBuilder& LineTo(const gfx::PointF&);
  PathBuilder& QuadTo(const gfx::PointF& ctrl, const gfx::PointF& pt);
  PathBuilder& CubicTo(const gfx::PointF& ctrl1,
                       const gfx::PointF& ctrl2,
                       const gfx::PointF& pt);
  PathBuilder& ArcTo(const gfx::PointF&,
                     float radius_x,
                     float radius_y,
                     float x_rotate,
                     bool large_arc,
                     bool sweep);
  PathBuilder& ArcTo(const gfx::PointF& p1,
                     const gfx::PointF& p2,
                     float radius);

  // These methods finalize the current subpath and add a new one.
  PathBuilder& AddRect(const gfx::RectF& rect);
  PathBuilder& AddRect(const gfx::PointF& origin, const gfx::PointF& opposite);
  PathBuilder& AddRoundedRect(const FloatRoundedRect&, bool clockwise = true);
  PathBuilder& AddContouredRect(const ContouredRect&);
  PathBuilder& AddEllipse(const gfx::PointF& p,
                          float radius_x,
                          float radius_y,
                          float start_angle,
                          float end_angle);
  PathBuilder& AddEllipse(const gfx::PointF& p,
                          float radius_x,
                          float radius_y,
                          float rotation,
                          float start_angle,
                          float end_angle);
  PathBuilder& AddEllipse(const gfx::PointF& center,
                          float radius_x,
                          float radius_y);
  PathBuilder& AddPath(const Path&, const AffineTransform&);

  PathBuilder& SetWindRule(WindRule);
  PathBuilder& Translate(const gfx::Vector2dF& offset);
  PathBuilder& Transform(const AffineTransform&);

 private:
  // TODO(crbug.com/378688986): switch to SkPathBuilder when ready.
  SkPath builder_;

  mutable std::optional<Path> current_path_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_PATH_BUILDER_H_
