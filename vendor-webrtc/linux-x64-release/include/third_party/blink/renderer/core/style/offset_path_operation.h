// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_OFFSET_PATH_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_OFFSET_PATH_OPERATION_H_

#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class OffsetPathOperation : public GarbageCollected<OffsetPathOperation> {
 public:
  enum OperationType { kReference, kShape, kCoordBox };

  OffsetPathOperation(const OffsetPathOperation&) = delete;
  OffsetPathOperation& operator=(const OffsetPathOperation&) = delete;
  virtual ~OffsetPathOperation() = default;

  virtual void Trace(Visitor*) const {}

  bool operator==(const OffsetPathOperation& o) const {
    return IsSameType(o) && IsEqualAssumingSameType(o) &&
           coord_box_ == o.coord_box_;
  }
  bool operator!=(const OffsetPathOperation& o) const { return !(*this == o); }

  virtual OperationType GetType() const = 0;
  bool IsSameType(const OffsetPathOperation& o) const {
    return o.GetType() == GetType();
  }

  CoordBox GetCoordBox() const { return coord_box_; }

 protected:
  explicit OffsetPathOperation(CoordBox coord_box) : coord_box_(coord_box) {}
  virtual bool IsEqualAssumingSameType(const OffsetPathOperation& o) const = 0;

  CoordBox coord_box_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_OFFSET_PATH_OPERATION_H_
