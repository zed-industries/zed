// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_MOCK_INPUT_HANDLER_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_MOCK_INPUT_HANDLER_PROXY_H_

#include <memory>

#include "cc/metrics/event_metrics.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/common/input/web_coalesced_input_event.h"
#include "third_party/blink/renderer/platform/widget/input/input_handler_proxy.h"

namespace blink::test {

class MockInputHandlerProxy : public InputHandlerProxy {
 public:
  MockInputHandlerProxy(cc::InputHandler& input_handler,
                        InputHandlerProxyClient* client)
      : InputHandlerProxy(input_handler, client) {}
  ~MockInputHandlerProxy() override = default;

  MOCK_METHOD3(HandleInputEventWithLatencyInfo,
               void(std::unique_ptr<blink::WebCoalescedInputEvent> event,
                    std::unique_ptr<cc::EventMetrics> metrics,
                    EventDispositionCallback callback));
};

}  // namespace blink::test

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_MOCK_INPUT_HANDLER_PROXY_H_
