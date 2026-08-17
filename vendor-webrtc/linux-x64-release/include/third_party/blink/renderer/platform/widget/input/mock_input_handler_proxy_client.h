// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_MOCK_INPUT_HANDLER_PROXY_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_MOCK_INPUT_HANDLER_PROXY_CLIENT_H_

#include "cc/input/overscroll_behavior.h"
#include "cc/input/touch_action.h"
#include "cc/metrics/event_metrics.h"
#include "third_party/blink/public/common/input/web_gesture_event.h"
#include "third_party/blink/public/common/input/web_input_event_attribution.h"
#include "third_party/blink/renderer/platform/widget/input/input_handler_proxy_client.h"
#include "ui/gfx/geometry/point.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink::test {

class MockInputHandlerProxyClient : public InputHandlerProxyClient {
 public:
  MockInputHandlerProxyClient() = default;
  MockInputHandlerProxyClient(const MockInputHandlerProxyClient&) = delete;
  MockInputHandlerProxyClient& operator=(const MockInputHandlerProxyClient&) =
      delete;

  ~MockInputHandlerProxyClient() override = default;

  void WillShutdown() override {}

  MOCK_METHOD3(GenerateScrollBeginAndSendToMainThread,
               void(const WebGestureEvent& update_event,
                    const WebInputEventAttribution&,
                    const cc::EventMetrics*));

  MOCK_METHOD5(DidOverscroll,
               void(const gfx::Vector2dF& accumulated_overscroll,
                    const gfx::Vector2dF& latest_overscroll_delta,
                    const gfx::Vector2dF& current_fling_velocity,
                    const gfx::PointF& causal_event_viewport_point,
                    const cc::OverscrollBehavior& overscroll_behavior));
  void DidStartScrollingViewport() override {}
  MOCK_METHOD1(SetAllowedTouchAction, void(cc::TouchAction touch_action));
  bool AllowsScrollResampling() override { return true; }
};

}  // namespace blink::test
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_MOCK_INPUT_HANDLER_PROXY_CLIENT_H_
