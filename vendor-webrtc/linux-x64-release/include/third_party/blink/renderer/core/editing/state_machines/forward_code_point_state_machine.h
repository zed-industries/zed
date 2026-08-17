// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_FORWARD_CODE_POINT_STATE_MACHINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_FORWARD_CODE_POINT_STATE_MACHINE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/state_machines/text_segmentation_machine_state.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace blink {

class CORE_EXPORT ForwardCodePointStateMachine {
  STACK_ALLOCATED();

 public:
  ForwardCodePointStateMachine();
  ForwardCodePointStateMachine(const ForwardCodePointStateMachine&) = delete;
  ForwardCodePointStateMachine& operator=(const ForwardCodePointStateMachine&) =
      delete;
  ~ForwardCodePointStateMachine() = default;

  // Prepares by feeding preceding text.
  TextSegmentationMachineState FeedPrecedingCodeUnit(UChar code_unit);

  // Finds boundary offset by feeding following text.
  TextSegmentationMachineState FeedFollowingCodeUnit(UChar code_unit);

  // Returns true if we are at code point boundary.
  bool AtCodePointBoundary();

  // Returns the next boundary offset.
  int GetBoundaryOffset();

  // Resets the internal state to the initial state.
  void Reset();

 private:
  enum class ForwardCodePointState;

  // The number of code units to be deleted.
  // Nothing to delete if there is an invalid surrogate pair.
  int code_units_to_be_deleted_ = 0;

  // The internal state.
  ForwardCodePointState state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_FORWARD_CODE_POINT_STATE_MACHINE_H_
