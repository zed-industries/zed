// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GEOMETRY_BOX_CLIP_PATH_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GEOMETRY_BOX_CLIP_PATH_OPERATION_H_

#include "third_party/blink/renderer/core/style/clip_path_operation.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"

namespace blink {

class GeometryBoxClipPathOperation final : public ClipPathOperation {
 public:
  explicit GeometryBoxClipPathOperation(GeometryBox geometry_box)
      : geometry_box_(geometry_box) {}

  GeometryBox GetGeometryBox() const { return geometry_box_; }

 private:
  bool operator==(const ClipPathOperation&) const override;
  OperationType GetType() const override { return kGeometryBox; }

  GeometryBox geometry_box_;
};

template <>
struct DowncastTraits<GeometryBoxClipPathOperation> {
  static bool AllowFrom(const ClipPathOperation& op) {
    return op.GetType() == ClipPathOperation::kGeometryBox;
  }
};

inline bool GeometryBoxClipPathOperation::operator==(
    const ClipPathOperation& o) const {
  if (!IsSameType(o)) {
    return false;
  }
  return geometry_box_ == To<GeometryBoxClipPathOperation>(o).geometry_box_;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_GEOMETRY_BOX_CLIP_PATH_OPERATION_H_
