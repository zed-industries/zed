// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TOUCH_ACTION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TOUCH_ACTION_H_

#include "cc/input/touch_action.h"

namespace blink {

const size_t kTouchActionBits = cc::kTouchActionBits;
using TouchAction = cc::TouchAction;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TOUCH_ACTION_H_
