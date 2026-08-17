/*
 * Copyright (C) 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_TEXT_PATH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_TEXT_PATH_H_

#include <memory>

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/layout/svg/layout_svg_inline.h"

namespace blink {

struct PointAndTangent;

// This class maps a 1D location in the "path space"; [0, path length] to a
// (2D) point on the path and provides the normal (angle from the x-axis) for
// said point.
class PathPositionMapper {
  USING_FAST_MALLOC(PathPositionMapper);

 public:
  PathPositionMapper(const Path&,
                     float computed_path_length,
                     float start_offset);

  enum PositionType {
    kOnPath,
    kBeforePath,
    kAfterPath,
  };
  PositionType PointAndNormalAtLength(float length, PointAndTangent&);
  float length() const { return path_length_; }
  float StartOffset() const { return path_start_offset_; }

 private:
  Path::PositionCalculator position_calculator_;
  float path_length_;
  float path_start_offset_;
};

class LayoutSVGTextPath final : public LayoutSVGInline {
 public:
  explicit LayoutSVGTextPath(Element*);

  std::unique_ptr<PathPositionMapper> LayoutPath() const;

  bool IsChildAllowed(LayoutObject*, const ComputedStyle&) const override;

  bool IsSVGTextPath() const final {
    NOT_DESTROYED();
    return true;
  }

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGTextPath";
  }
};

template <>
struct DowncastTraits<LayoutSVGTextPath> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsSVGTextPath();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_TEXT_PATH_H_
