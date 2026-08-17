// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_SHAPE_EXCLUSIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_SHAPE_EXCLUSIONS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/exclusions/exclusion_area.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// This struct represents exclusions which have shape data associated with them.
// As shapes are relatively uncommon we store these as a separate struct, and
// allocate only when necessary.
//
// This struct can belong to either a Shelf within the exclusion space, or on
// LayoutOpportunity. Outside these classes normal code shouldn't interact
// with this class.
class CORE_EXPORT ShapeExclusions : public GarbageCollected<ShapeExclusions> {
 public:
  ShapeExclusions() = default;
  ShapeExclusions(const ShapeExclusions& other)
      : line_left_shapes(other.line_left_shapes),
        line_right_shapes(other.line_right_shapes) {}
  void Trace(Visitor* visitor) const {
    visitor->Trace(line_left_shapes);
    visitor->Trace(line_right_shapes);
  }
  ExclusionAreaPtrArray line_left_shapes;
  ExclusionAreaPtrArray line_right_shapes;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_SHAPE_EXCLUSIONS_H_
