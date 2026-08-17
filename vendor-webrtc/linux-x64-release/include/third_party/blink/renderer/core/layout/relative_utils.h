// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_RELATIVE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_RELATIVE_UTILS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/layout/physical_box_fragment.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"

namespace blink {

class ConstraintSpace;

// Implements relative positioning:
// https://www.w3.org/TR/css-position-3/#rel-pos
// Returns the relative position offset as defined by |child_style|.
CORE_EXPORT LogicalOffset
ComputeRelativeOffset(const ComputedStyle& child_style,
                      WritingDirectionMode container_writing_direction,
                      const LogicalSize& available_size);

CORE_EXPORT LogicalOffset ComputeRelativeOffsetForBoxFragment(
    const PhysicalBoxFragment& fragment,
    WritingDirectionMode container_writing_direction,
    const LogicalSize& available_size);

CORE_EXPORT LogicalOffset
ComputeRelativeOffsetForInline(const ConstraintSpace& space,
                               const ComputedStyle& child_style);

CORE_EXPORT LogicalOffset
ComputeRelativeOffsetForOOFInInline(const ConstraintSpace& space,
                                    const ComputedStyle& child_style);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_RELATIVE_UTILS_H_
