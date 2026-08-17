// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_TOUCH_ACTION_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_TOUCH_ACTION_H_

#include "cc/input/touch_action.h"

namespace blink {

// The only reason of creating a WebTouchAction is that there is a virtual
// function under WebWidgetClient that is overridden by classes under content/
// and WebKit/.
using WebTouchAction = cc::TouchAction;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_TOUCH_ACTION_H_
