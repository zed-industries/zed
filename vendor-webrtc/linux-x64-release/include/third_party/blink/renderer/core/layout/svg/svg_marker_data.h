/*
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_MARKER_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_MARKER_DATA_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/svg/svg_path_consumer.h"
#include "third_party/blink/renderer/platform/geometry/path.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"

namespace blink {

enum SVGMarkerType { kStartMarker, kMidMarker, kEndMarker };

class LayoutSVGResourceMarker;
class SVGPathByteStream;

struct MarkerPosition {
  DISALLOW_NEW();
  MarkerPosition(SVGMarkerType use_type,
                 const gfx::PointF& use_origin,
                 float use_angle)
      : type(use_type), origin(use_origin), angle(use_angle) {}

  LayoutSVGResourceMarker* SelectMarker(
      LayoutSVGResourceMarker* marker_start,
      LayoutSVGResourceMarker* marker_mid,
      LayoutSVGResourceMarker* marker_end) const {
    switch (type) {
      case kStartMarker:
        return marker_start;
      case kMidMarker:
        return marker_mid;
      case kEndMarker:
        return marker_end;
    }
    NOTREACHED();
  }

  SVGMarkerType type;
  gfx::PointF origin;
  float angle;
};

class SVGMarkerDataBuilder : private SVGPathConsumer {
  STACK_ALLOCATED();

 public:
  explicit SVGMarkerDataBuilder(Vector<MarkerPosition>& positions)
      : positions_(positions),
        last_moveto_index_(0),
        last_element_type_(kPathElementMoveToPoint) {}

  // Build marker data for a Path.
  void Build(const Path&);

  // Build marker data for a SVGPathByteStream.
  //
  // A SVGPathByteStream is semantically higher-level than a Path, and thus
  // this allows those higher-level constructs (for example arcs) to be handled
  // correctly. This should be used in cases where the original path data can
  // contain such higher-level constructs.
  void Build(const SVGPathByteStream&);

 private:
  // SVGPathConsumer
  void EmitSegment(const PathSegmentData&) override;

  static void UpdateFromPathElement(void* info, const PathElement&);

  enum AngleType {
    kBisecting,
    kInbound,
    kOutbound,
  };

  double CurrentAngle(AngleType) const;
  AngleType DetermineAngleType(bool ends_subpath) const;

  void UpdateAngle(bool ends_subpath);

  struct SegmentData {
    gfx::Vector2dF start_tangent;  // Tangent in the start point of the segment.
    gfx::Vector2dF end_tangent;    // Tangent in the end point of the segment.
    gfx::PointF position;          // The end point of the segment.
  };

  static void ComputeQuadTangents(SegmentData&,
                                  const gfx::PointF& start,
                                  const gfx::PointF& control,
                                  const gfx::PointF& end);
  SegmentData ExtractPathElementFeatures(const PathElement&) const;
  void UpdateFromPathElement(const PathElement&);
  void Flush();

  Vector<MarkerPosition>& positions_;
  unsigned last_moveto_index_;
  PathElementType last_element_type_;
  gfx::PointF origin_;
  gfx::PointF subpath_start_;
  gfx::Vector2dF in_slope_;
  gfx::Vector2dF out_slope_;
  gfx::Vector2dF last_moveto_out_slope_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_MARKER_DATA_H_
