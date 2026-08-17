// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_STATE_MACHINE_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_STATE_MACHINE_UTIL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"

namespace blink {

// Returns true if there is a grapheme boundary between prevCodePoint and
// nextCodePoint.
// DO NOT USE this function directly since this doesn't care about preceding
// regional indicator symbols. Use ForwardGraphemeBoundaryStateMachine or
// BackwardGraphemeBoundaryStateMachine instead.
CORE_EXPORT bool IsGraphemeBreak(UChar32 prev_code_point,
                                 UChar32 next_code_point);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_STATE_MACHINES_STATE_MACHINE_UTIL_H_
