// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LOGICAL_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LOGICAL_FRAGMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/physical_fragment.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/text/writing_direction_mode.h"

namespace blink {

struct LogicalSize;

class CORE_EXPORT LogicalFragment {
  STACK_ALLOCATED();

 public:
  LogicalFragment(WritingDirectionMode writing_direction,
                  const PhysicalFragment& physical_fragment)
      : physical_fragment_(physical_fragment),
        writing_direction_(writing_direction) {}

  // Returns the border-box size.
  LayoutUnit InlineSize() const {
    return writing_direction_.IsHorizontal() ? physical_fragment_.Size().width
                                             : physical_fragment_.Size().height;
  }
  LayoutUnit BlockSize() const {
    return writing_direction_.IsHorizontal() ? physical_fragment_.Size().height
                                             : physical_fragment_.Size().width;
  }
  LogicalSize Size() const {
    return ToLogicalSize(physical_fragment_.Size(),
                         writing_direction_.GetWritingMode());
  }
  WritingDirectionMode GetWritingDirection() const {
    return writing_direction_;
  }

 protected:
  const PhysicalFragment& physical_fragment_;
  const WritingDirectionMode writing_direction_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LOGICAL_FRAGMENT_H_
