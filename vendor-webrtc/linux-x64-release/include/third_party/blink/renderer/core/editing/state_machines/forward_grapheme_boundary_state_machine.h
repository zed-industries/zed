// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_FORWARD_GRAPHEME_BOUNDARY_STATE_MACHINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_FORWARD_GRAPHEME_BOUNDARY_STATE_MACHINE_H_

#include <iosfwd>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/state_machines/text_segmentation_machine_state.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace blink {

class CORE_EXPORT ForwardGraphemeBoundaryStateMachine {
  STACK_ALLOCATED();

 public:
  ForwardGraphemeBoundaryStateMachine();
  ForwardGraphemeBoundaryStateMachine(
      const ForwardGraphemeBoundaryStateMachine&) = delete;
  ForwardGraphemeBoundaryStateMachine& operator=(
      const ForwardGraphemeBoundaryStateMachine&) = delete;

  // Find boundary offset by feeding preceding text.
  // This method must not be called after feedFollowingCodeUnit().
  TextSegmentationMachineState FeedPrecedingCodeUnit(UChar code_unit);

  // Tells the end of preceding text to the state machine.
  TextSegmentationMachineState TellEndOfPrecedingText();

  // Find boundary offset by feeding following text.
  // This method must be called after feedPrecedingCodeUnit() returns
  // NeedsFollowingCodeUnit.
  TextSegmentationMachineState FeedFollowingCodeUnit(UChar code_unit);

  // Returns the next boundary offset. This method finalizes the state machine
  // if it is not finished.
  int FinalizeAndGetBoundaryOffset();

  // Resets the internal state to the initial state.
  void Reset();

 private:
  enum class InternalState;
  friend std::ostream& operator<<(std::ostream&, InternalState);

  TextSegmentationMachineState MoveToNextState(InternalState);

  TextSegmentationMachineState StaySameState();

  // Updates the internal state to InternalState::Finished then
  // returnsTextSegmentationMachineState::Finished.
  TextSegmentationMachineState Finish();

  // Handles end of text. This method always finishes the state machine.
  void FinishWithEndOfText();

  // Used for composing supplementary code point with surrogate pairs.
  UChar pending_code_unit_ = 0;

  // The code point immediately before the boundary_offset_.
  UChar32 prev_code_point_;

  // The relative offset from the begging of this state machine.
  int boundary_offset_ = 0;

  // The number of regional indicator symbols preceding to the begging offset.
  int preceding_ris_count_ = 0;

  // The internal state.
  InternalState internal_state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_FORWARD_GRAPHEME_BOUNDARY_STATE_MACHINE_H_
