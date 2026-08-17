/*
 * Copyright (C) 2006 Apple Computer, Inc.
 * Copyright (C) 2012 Nokia Corporation and/or its subsidiary(-ies)
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
 *
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_LOCATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_LOCATION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/geometry/path_types.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "ui/gfx/geometry/quad_f.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class ContouredRect;
class Path;

class CORE_EXPORT HitTestLocation {
  DISALLOW_NEW();

 public:
  // Note that all points are in contents (aka "page") coordinate space for the
  // document that is being hit tested. All points and size are in root frame
  // coordinates (physical pixel scaled by page_scale when zoom for dsf is
  // enabled; otherwise in dip scaled by page_scale), Which means the points
  // should already applied page_scale_factor, but not page_zoom_factor and
  // scroll offset. See:
  // http://www.chromium.org/developers/design-documents/blink-coordinate-spaces
  HitTestLocation();
  explicit HitTestLocation(const PhysicalOffset&);
  explicit HitTestLocation(const gfx::Point&);
  explicit HitTestLocation(const gfx::PointF&);
  explicit HitTestLocation(const gfx::PointF&, const gfx::QuadF&);
  explicit HitTestLocation(const PhysicalRect&);

  // The bounding box isn't always a 1x1 rect even when the hit test is not
  // rect-based. When we hit test a transformed box and transform the hit test
  // location into the box's local coordinate space, the bounding box should
  // also be transformed accordingly.
  //
  // TODO(mustaq): Clean up the mix of coordinate units in params (floating
  // point vs LayoutUnit).
  explicit HitTestLocation(const gfx::PointF& point,
                           const PhysicalRect& bounding_box);

  HitTestLocation(const HitTestLocation&, const PhysicalOffset& offset);
  HitTestLocation(const HitTestLocation&, wtf_size_t fragment_index);
  HitTestLocation(const HitTestLocation&);
  HitTestLocation& operator=(const HitTestLocation&);

  const PhysicalOffset& Point() const { return point_; }
  gfx::Point RoundedPoint() const { return ToRoundedPoint(point_); }

  int FragmentIndex() const { return fragment_index_; }

  // Rect-based hit test related methods.
  bool IsRectBasedTest() const { return is_rect_based_; }
  bool IsRectilinear() const { return is_rectilinear_; }
  const PhysicalRect& BoundingBox() const { return bounding_box_; }
  gfx::Rect ToEnclosingRect() const {
    return ::blink::ToEnclosingRect(bounding_box_);
  }

  // Returns the 1px x 1px hit test rect for a point.
  static PhysicalRect RectForPoint(const PhysicalOffset& point) {
    return PhysicalRect(point, PhysicalSize(LayoutUnit(1), LayoutUnit(1)));
  }

  bool Intersects(const PhysicalRect&) const;

  // Uses floating-point intersection, which uses inclusive intersection
  // (see PhysicalRect::InclusiveIntersect for a definition)
  bool Intersects(const gfx::RectF&) const;
  bool Intersects(const ContouredRect&) const;
  bool Intersects(const gfx::QuadF&) const;
  bool ContainsPoint(const gfx::PointF&) const;

  bool Intersects(const Path&) const;
  bool Intersects(const Path&, WindRule) const;

  bool IntersectsStroke(const gfx::RectF& rect, float stroke_width) const;

  bool IntersectsEllipse(const gfx::PointF& center,
                         const gfx::SizeF& radii) const;
  bool IntersectsCircleStroke(const gfx::PointF& center,
                              float radius,
                              float stroke_width) const;

  const gfx::PointF& TransformedPoint() const { return transformed_point_; }
  const gfx::QuadF& TransformedRect() const { return transformed_rect_; }

 private:
  void Move(const PhysicalOffset& offset);

  // These are cached forms of the more accurate |transformed_point_| and
  // |transformed_rect_|, below.
  PhysicalOffset point_;
  PhysicalRect bounding_box_;

  gfx::PointF transformed_point_;
  gfx::QuadF transformed_rect_;

  // Index of fragment (FragmentData) to hit-test. If it's -1, all fragments
  // will be hit-tested. This is used to hit test items inside one NG block
  // fragment at a time. This is necessary for relatively positioned non-atomic
  // inlines. Note that this member is intentionally NOT copied when copying the
  // object.
  int fragment_index_ = -1;

  bool is_rect_based_;
  bool is_rectilinear_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_LOCATION_H_
