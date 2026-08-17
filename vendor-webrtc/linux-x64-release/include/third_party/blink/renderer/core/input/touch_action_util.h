// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_TOUCH_ACTION_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_TOUCH_ACTION_UTIL_H_

#include "third_party/blink/public/common/input/web_pointer_event.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/touch_action.h"

namespace blink {

class Node;

namespace touch_action_util {
CORE_EXPORT TouchAction ComputeEffectiveTouchAction(const Node&);

TouchAction EffectiveTouchActionAtPointerDown(const WebPointerEvent& event,
                                              const Node* pointerdown_node);

TouchAction EffectiveTouchActionAtPointer(const WebPointerEvent& event,
                                          const Node* node_at_pointer);

}  // namespace touch_action_util

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_TOUCH_ACTION_UTIL_H_
