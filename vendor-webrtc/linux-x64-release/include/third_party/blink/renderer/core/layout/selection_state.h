// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SELECTION_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SELECTION_STATE_H_

#include <iosfwd>
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

// Each LayoutObject has a SelectionState and it represents how the
// LayoutObject is selected. This enum is used to paint/invalidate selection
// highlight for the LayoutObject.
enum class SelectionState {
  // The LayoutObject is not selected.
  kNone,
  // kStart, kInside, kEnd and kStartAndEnd represent the LayoutObject
  // is somehow selected to paint and either LayoutText or LayoutReplaced.
  // The start of a selection.
  kStart,
  // Inside a selection.
  kInside,
  // The end of a selection.
  kEnd,
  // The LayoutObject contains an entire selection.
  kStartAndEnd,
  // The LayoutObject has at least one LayoutObject child which SelectionState
  // is not KNone.
  // This property is used to invalidate LayoutObject.
  kContain
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const SelectionState);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SELECTION_STATE_H_
