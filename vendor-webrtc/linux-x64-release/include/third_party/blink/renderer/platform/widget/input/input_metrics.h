// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_INPUT_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_INPUT_METRICS_H_

#include "cc/input/main_thread_scrolling_reason.h"
#include "third_party/blink/public/common/input/web_gesture_device.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// |reasons| is a combination of bits defined in
// |cc::MainThreadScrollingReason|.
PLATFORM_EXPORT void RecordScrollReasonsMetric(WebGestureDevice device,
                                               uint32_t reasons);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_INPUT_METRICS_H_
