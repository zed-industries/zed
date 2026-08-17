// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TRANSFORM_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TRANSFORM_UTILS_H_

#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"

namespace blink {

class LayoutBox;
class PhysicalBoxFragment;

// Compute the transform reference box, based on the computed 'transform-box'
// property, for the specified entity.
PhysicalRect ComputeReferenceBox(const PhysicalBoxFragment&);
PhysicalRect ComputeReferenceBox(const LayoutBox&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TRANSFORM_UTILS_H_
