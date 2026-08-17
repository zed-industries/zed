// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATED_SVG_PATH_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATED_SVG_PATH_SOURCE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/animation/svg_path_seg_interpolation_functions.h"
#include "third_party/blink/renderer/core/svg/svg_path_data.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class InterpolatedSVGPathSource {
  STACK_ALLOCATED();

 public:
  InterpolatedSVGPathSource(const InterpolableList& list_value,
                            const Vector<SVGPathSegType>& path_seg_types)
      : current_index_(0),
        interpolable_path_segs_(list_value),
        path_seg_types_(path_seg_types) {
    DCHECK_EQ(interpolable_path_segs_.length(), path_seg_types_.size());
  }

  InterpolatedSVGPathSource(const InterpolatedSVGPathSource&) = delete;
  InterpolatedSVGPathSource& operator=(const InterpolatedSVGPathSource&) =
      delete;

  bool HasMoreData() const;
  PathSegmentData ParseSegment();

 private:
  PathCoordinates current_coordinates_;
  wtf_size_t current_index_;
  const InterpolableList& interpolable_path_segs_;
  const Vector<SVGPathSegType>& path_seg_types_;
};

bool InterpolatedSVGPathSource::HasMoreData() const {
  return current_index_ < interpolable_path_segs_.length();
}

PathSegmentData InterpolatedSVGPathSource::ParseSegment() {
  PathSegmentData segment =
      SVGPathSegInterpolationFunctions::ConsumeInterpolablePathSeg(
          *interpolable_path_segs_.Get(current_index_),
          path_seg_types_.at(current_index_), current_coordinates_);
  current_index_++;
  return segment;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATED_SVG_PATH_SOURCE_H_
