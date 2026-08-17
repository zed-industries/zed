// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FROM_AD_STATE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FROM_AD_STATE_H_

#include "third_party/blink/public/common/common_export.h"

namespace blink {

// This enum is the cross product of two ad related status of an event: whether
// the event occurs on an ad frame, and whether it occurs with an ad script in
// the stack.
enum class FromAdState {
  // This is used for a UMA histogram. Please never alter existing values, only
  // append new ones and make sure to update enums.xml.
  kAdScriptAndAdFrame = 0,
  kNonAdScriptAndAdFrame = 1,
  kAdScriptAndNonAdFrame = 2,
  kNonAdScriptAndNonAdFrame = 3,
  kMaxValue = kNonAdScriptAndNonAdFrame,
};

// Returns the FromAdState corresponded to the cross product of |is_ad_frame|
// and |is_ad_script_in_stack|.
FromAdState BLINK_COMMON_EXPORT GetFromAdState(bool is_ad_frame,
                                               bool is_ad_script_in_stack);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FROM_AD_STATE_H_
