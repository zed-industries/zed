// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SHAPE_OFFSET_PATH_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SHAPE_OFFSET_PATH_OPERATION_H_

#include "third_party/blink/renderer/core/style/basic_shapes.h"
#include "third_party/blink/renderer/core/style/offset_path_operation.h"

namespace blink {

class ShapeOffsetPathOperation final : public OffsetPathOperation {
 public:
  ShapeOffsetPathOperation(const BasicShape* shape, CoordBox coord_box)
      : OffsetPathOperation(coord_box), shape_(std::move(shape)) {
    DCHECK(shape_);
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(shape_);
    OffsetPathOperation::Trace(visitor);
  }

  bool IsEqualAssumingSameType(const OffsetPathOperation& o) const override {
    return *shape_ == *To<ShapeOffsetPathOperation>(o).shape_;
  }

  OperationType GetType() const override { return kShape; }

  const BasicShape& GetBasicShape() const { return *shape_; }

 private:
  Member<const BasicShape> shape_;
};

template <>
struct DowncastTraits<ShapeOffsetPathOperation> {
  static bool AllowFrom(const OffsetPathOperation& op) {
    return op.GetType() == OffsetPathOperation::kShape;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SHAPE_OFFSET_PATH_OPERATION_H_
