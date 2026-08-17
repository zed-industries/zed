/*
 * Copyright (C) 2002, 2003 The Karbon Developers
 * Copyright (C) 2006 Alexander Kellett <lypanov@kde.org>
 * Copyright (C) 2006, 2007 Rob Buis <buis@kde.org>
 * Copyright (C) 2007, 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_PARSER_H_

#include "third_party/blink/renderer/core/svg/svg_path_data.h"
#include "ui/gfx/geometry/point_f.h"

namespace blink {

class SVGPathConsumer;

namespace svg_path_parser {

template <typename SourceType, typename ConsumerType>
inline bool ParsePath(SourceType& source, ConsumerType& consumer) {
  while (source.HasMoreData()) {
    PathSegmentData segment = source.ParseSegment();
    if (segment.command == kPathSegUnknown)
      return false;

    consumer.EmitSegment(segment);
  }
  return true;
}

}  // namespace svg_path_parser

class SVGPathNormalizer {
  STACK_ALLOCATED();

 public:
  SVGPathNormalizer(SVGPathConsumer* consumer)
      : consumer_(consumer), last_command_(kPathSegUnknown) {
    DCHECK(consumer_);
  }

  void EmitSegment(const PathSegmentData&);

 protected:
  bool DecomposeArcToCubic(const gfx::PointF& current_point,
                           const PathSegmentData&);

  SVGPathConsumer* consumer_;
  gfx::PointF control_point_;
  gfx::PointF current_point_;
  gfx::PointF sub_path_point_;
  SVGPathSegType last_command_;
};

class SVGPathAbsolutizer {
  STACK_ALLOCATED();

 public:
  SVGPathAbsolutizer(SVGPathConsumer* consumer) : consumer_(consumer) {
    DCHECK(consumer_);
  }

  void EmitSegment(const PathSegmentData&);

 private:
  SVGPathConsumer* consumer_;
  gfx::PointF sub_path_point_;
  gfx::PointF current_point_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_PARSER_H_
