// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_URL_METADATA_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_URL_METADATA_UTILS_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"

namespace blink {

class LayoutObject;
struct PaintInfo;

// Traverses |layout_object| recursively to add URLs and Rects.
void AddURLRectsForInlineChildrenRecursively(
    const LayoutObject& layout_object,
    const PaintInfo& paint_info,
    const PhysicalOffset& paint_offset);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_URL_METADATA_UTILS_H_
