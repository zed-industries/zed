// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIELDSET_PAINT_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIELDSET_PAINT_INFO_H_

#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"

namespace blink {

class ComputedStyle;

struct FieldsetPaintInfo {
  STACK_ALLOCATED();

 public:
  // Calculate the fieldset block-start border offset and the cut-out rectangle
  // caused by the rendered legend.
  FieldsetPaintInfo(const ComputedStyle& fieldset_style,
                    const PhysicalSize& fieldset_size,
                    const PhysicalBoxStrut& fieldset_borders,
                    const PhysicalRect& legend_border_box);

  // Block-start border outset caused by the rendered legend.
  PhysicalBoxStrut border_outsets;

  // The cutout rectangle (where the border is not to be painted) occupied by
  // the legend. Note that this may intersect with other border sides than the
  // block-start one, if the legend happens to overlap with any of the other
  // borders.
  PhysicalRect legend_cutout_rect;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FIELDSET_PAINT_INFO_H_
