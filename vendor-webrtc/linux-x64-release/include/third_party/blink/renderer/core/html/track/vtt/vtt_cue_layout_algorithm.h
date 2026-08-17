// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_CUE_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_CUE_LAYOUT_ALGORITHM_H_

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class Rect;
}

namespace blink {

class LayoutBox;
class VTTCueBox;
struct PhysicalSize;

// VttCueLayoutAlgorithm is responsible to do step 10 of
// https://w3c.github.io/webvtt/#apply-webvtt-cue-settings .
//
// This class is used in a ResizeObserver callback for VTTCueBox.
// See https://bit.ly/3vfUajH for more details.
class VttCueLayoutAlgorithm {
  STACK_ALLOCATED();
  using PassKey = base::PassKey<VttCueLayoutAlgorithm>;

 public:
  explicit VttCueLayoutAlgorithm(VTTCueBox& cue);

  void Layout();

 private:
  void AdjustPositionWithSnapToLines();
  void AdjustPositionWithoutSnapToLines();

  // Helpers for AdjustPositionWithSnapToLines():

  static PhysicalSize FirstInlineBoxSize(const LayoutBox& cue_box);
  LayoutUnit ComputeInitialPositionAdjustment(LayoutUnit max_dimension,
                                              const gfx::Rect& controls_rect);
  static gfx::Rect CueBoundingBox(const LayoutBox& cue_box);
  bool IsOutside(const gfx::Rect& title_area) const;
  bool IsOverlapping(const gfx::Rect& controls_rect) const;
  bool ShouldSwitchDirection(LayoutUnit cue_block_position,
                             LayoutUnit cue_block_size,
                             LayoutUnit full_dimension) const;

  VTTCueBox& cue_;
  float snap_to_lines_position_;

  // |margin_| and |step_| are data members because they are accessed by
  // multiple member functions, and we'd like to simplify their arguments.
  LayoutUnit margin_;
  LayoutUnit step_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_CUE_LAYOUT_ALGORITHM_H_
