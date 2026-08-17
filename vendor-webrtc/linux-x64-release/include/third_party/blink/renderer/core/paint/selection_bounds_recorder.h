// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SELECTION_BOUNDS_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SELECTION_BOUNDS_RECORDER_H_

#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/selection_state.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class FrameSelection;
class LayoutObject;
class PaintController;

// This class is used for recording painted selection bounds. Based on the
// SelectionState and provided |selection_rect|, records the appropriate bounds
// via the paint controller. These bounds are consumed at composition time by
// PaintArtifactCompositor and pushed to the LayerTreeHost. All of the work
// happens in the destructor to ensure this information recorded after any
// painting is completed, even if a cached drawing is re-used.
class SelectionBoundsRecorder {
  STACK_ALLOCATED();

 public:
  SelectionBoundsRecorder(SelectionState,
                          PhysicalRect,
                          PaintController&,
                          TextDirection,
                          WritingMode,
                          const LayoutObject&);

  ~SelectionBoundsRecorder();

  static bool ShouldRecordSelection(const FrameSelection&, SelectionState);

  static bool IsVisible(const LayoutObject& rect_layout_object,
                        const PhysicalOffset& edge_start_in_layer,
                        const PhysicalOffset& edge_end_in_layer);

 private:
  const SelectionState state_;
  PhysicalRect selection_rect_;
  PaintController& paint_controller_;
  TextDirection text_direction_;
  WritingMode writing_mode_;
  const LayoutObject& selection_layout_object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SELECTION_BOUNDS_RECORDER_H_
