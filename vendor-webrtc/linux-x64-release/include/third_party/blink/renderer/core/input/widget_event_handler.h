// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_WIDGET_EVENT_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_WIDGET_EVENT_HANDLER_H_

#include <memory>
#include <vector>

#include "third_party/blink/public/platform/web_input_event_result.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class LocalFrame;
class WebCoalescedInputEvent;
class WebGestureEvent;
class WebInputEvent;
class WebKeyboardEvent;
class WebMouseEvent;
class WebMouseWheelEvent;
class WebPointerEvent;

// Event handling subclass for frame and popup widgets. Dispatches
// WebInputEvents to type-specific virtual handlers.
class CORE_EXPORT WidgetEventHandler {
 protected:
  virtual void HandleMouseMove(
      LocalFrame& main_frame,
      const WebMouseEvent&,
      const std::vector<std::unique_ptr<WebInputEvent>>&,
      const std::vector<std::unique_ptr<WebInputEvent>>&);
  virtual void HandleMouseLeave(LocalFrame& local_root, const WebMouseEvent&);
  virtual void HandleMouseDown(LocalFrame& local_root, const WebMouseEvent&);
  virtual WebInputEventResult HandleMouseUp(LocalFrame& local_root,
                                            const WebMouseEvent&);
  virtual WebInputEventResult HandleMouseWheel(LocalFrame& local_root,
                                               const WebMouseWheelEvent&);
  virtual WebInputEventResult HandleKeyEvent(const WebKeyboardEvent&) = 0;
  virtual WebInputEventResult HandleCharEvent(const WebKeyboardEvent&) = 0;
  virtual WebInputEventResult HandleGestureEvent(const WebGestureEvent&) = 0;
  virtual WebInputEventResult HandlePointerEvent(
      LocalFrame& main_frame,
      const WebPointerEvent&,
      const std::vector<std::unique_ptr<WebInputEvent>>&,
      const std::vector<std::unique_ptr<WebInputEvent>>&);
  virtual ~WidgetEventHandler() = default;

  WebInputEventResult HandleInputEvent(
      const WebCoalescedInputEvent& coalesced_event,
      LocalFrame* root);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_WIDGET_EVENT_HANDLER_H_
