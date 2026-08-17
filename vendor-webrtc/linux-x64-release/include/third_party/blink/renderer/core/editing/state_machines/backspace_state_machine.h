// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_BACKSPACE_STATE_MACHINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_BACKSPACE_STATE_MACHINE_H_

#include <iosfwd>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/state_machines/text_segmentation_machine_state.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace blink {

class CORE_EXPORT BackspaceStateMachine {
  STACK_ALLOCATED();

 public:
  BackspaceStateMachine();
  BackspaceStateMachine(const BackspaceStateMachine&) = delete;
  BackspaceStateMachine& operator=(const BackspaceStateMachine&) = delete;

  // Prepares by feeding preceding text.
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
  enum class BackspaceState;
  friend std::ostream& operator<<(std::ostream&, BackspaceState);

  // Updates the internal state to the |newState| then return
  // InternalState::NeedMoreCodeUnit.
  TextSegmentationMachineState MoveToNextState(BackspaceState new_state);

  // Update the internal state to BackspaceState::Finished, then return
  // MachineState::Finished.
  TextSegmentationMachineState Finish();

  // Used for composing supplementary code point with surrogate pairs.
  UChar trail_surrogate_ = 0;

  // The number of code units to be deleted.
  int code_units_to_be_deleted_ = 0;

  // The length of the previously seen variation selector.
  int last_seen_vs_code_units_ = 0;

  // The internal state.
  BackspaceState state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_BACKSPACE_STATE_MACHINE_H_
