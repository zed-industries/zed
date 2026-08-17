// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_TEXT_SEGMENTATION_MACHINE_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_TEXT_SEGMENTATION_MACHINE_STATE_H_

#include <ostream>

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

enum class TextSegmentationMachineState {
  // Indicates the state machine is in invalid state.
  kInvalid,
  // Indicates the state machine needs more code units to transit the state.
  kNeedMoreCodeUnit,
  // Indicates the state machine needs following code units to transit the
  // state.
  kNeedFollowingCodeUnit,
  // Indicates the state machine found a boundary.
  kFinished,
};

CORE_EXPORT std::ostream& operator<<(std::ostream&,
                                     TextSegmentationMachineState);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_TEXT_SEGMENTATION_MACHINE_STATE_H_
