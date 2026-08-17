// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_CURSOR_CONTROL_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_CURSOR_CONTROL_HANDLER_H_

#include <optional>

#include "cc/input/touch_action.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/widget/input/input_handler_proxy.h"

namespace blink {

class WebGestureEvent;
class WebInputEvent;

// Observe scroll gesture sequence from InputHandlerProxy, so we can send the
// sequence with |scroll_begin.cursor_control = true| to the main thread to
// perform cursor control.
class PLATFORM_EXPORT CursorControlHandler {
 public:
  CursorControlHandler() = default;
  ~CursorControlHandler() = default;
  CursorControlHandler(const CursorControlHandler&) = delete;
  CursorControlHandler& operator=(const CursorControlHandler&) = delete;

  std::optional<InputHandlerProxy::EventDisposition> ObserveInputEvent(
      const WebInputEvent& event);

 private:
  std::optional<InputHandlerProxy::EventDisposition> HandleGestureScrollBegin(
      const WebGestureEvent& event);
  std::optional<InputHandlerProxy::EventDisposition> HandleGestureScrollUpdate(
      const WebGestureEvent& event);
  std::optional<InputHandlerProxy::EventDisposition> HandleGestureScrollEnd(
      const WebGestureEvent& event);

  bool cursor_control_in_progress_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_CURSOR_CONTROL_HANDLER_H_
