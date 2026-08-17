// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_COORD_BOX_OFFSET_PATH_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_COORD_BOX_OFFSET_PATH_OPERATION_H_

#include "third_party/blink/renderer/core/style/offset_path_operation.h"

namespace blink {

class CoordBoxOffsetPathOperation final : public OffsetPathOperation {
 public:
  explicit CoordBoxOffsetPathOperation(CoordBox coord_box)
      : OffsetPathOperation(coord_box) {}

  bool IsEqualAssumingSameType(const OffsetPathOperation& o) const override {
    return true;
  }

  OperationType GetType() const override { return kCoordBox; }
};

template <>
struct DowncastTraits<CoordBoxOffsetPathOperation> {
  static bool AllowFrom(const OffsetPathOperation& op) {
    return op.GetType() == OffsetPathOperation::kCoordBox;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_COORD_BOX_OFFSET_PATH_OPERATION_H_
