// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLOATS_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLOATS_UTILS_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

class ExclusionSpace;
struct PositionedFloat;
struct UnpositionedFloat;

// Calculate and return the inline size of the unpositioned float.
LayoutUnit ComputeMarginBoxInlineSizeForUnpositionedFloat(UnpositionedFloat*);

// Position and lay out a float.
PositionedFloat PositionFloat(UnpositionedFloat*, ExclusionSpace*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLOATS_UTILS_H_
