/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_WEB_INPUT_EVENT_CONVERSION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_WEB_INPUT_EVENT_CONVERSION_H_

#include "third_party/blink/public/common/input/web_coalesced_input_event.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/input/web_keyboard_event.h"
#include "third_party/blink/public/common/input/web_mouse_wheel_event.h"
#include "third_party/blink/public/common/input/web_pointer_event.h"
#include "third_party/blink/public/common/input/web_touch_event.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"

namespace blink {

class KeyboardEvent;
class LayoutObject;
class LocalFrameView;
class MouseEvent;
class TouchEvent;
class WebGestureEvent;
class WebKeyboardEvent;

// These classes are used for conversion from Blink events to WebInputEvents
// (for plugins).

class CORE_EXPORT WebMouseEventBuilder : public WebMouseEvent {
 public:
  // Converts a MouseEvent to a corresponding WebMouseEvent.
  // NOTE: This is only implemented for mousemove, mouseover, mouseout,
  // mousedown and mouseup. If the event mapping fails, the event type will
  // be set to Undefined.
  WebMouseEventBuilder(const LayoutObject*, const MouseEvent&);
  WebMouseEventBuilder(const LayoutObject*, const TouchEvent&);
};

// Converts a KeyboardEvent to a corresponding WebKeyboardEvent.
// NOTE: For KeyboardEvent, this is only implemented for keydown,
// keyup, and keypress. If the event mapping fails, the event type will be set
// to Undefined.
class CORE_EXPORT WebKeyboardEventBuilder : public WebKeyboardEvent {
 public:
  WebKeyboardEventBuilder(const KeyboardEvent&);
};

// Return a new transformed WebGestureEvent by applying the Widget's scale
// and translation.
CORE_EXPORT WebGestureEvent TransformWebGestureEvent(LocalFrameView*,
                                                     const WebGestureEvent&);
CORE_EXPORT WebMouseEvent TransformWebMouseEvent(LocalFrameView*,
                                                 const WebMouseEvent&);

CORE_EXPORT WebMouseWheelEvent
TransformWebMouseWheelEvent(LocalFrameView*, const WebMouseWheelEvent&);

CORE_EXPORT WebPointerEvent TransformWebPointerEvent(LocalFrameView*,
                                                     const WebPointerEvent&);

Vector<WebMouseEvent> CORE_EXPORT TransformWebMouseEventVector(
    LocalFrameView*,
    const std::vector<std::unique_ptr<WebInputEvent>>&);
Vector<WebPointerEvent> CORE_EXPORT TransformWebPointerEventVector(
    LocalFrameView*,
    const std::vector<std::unique_ptr<WebInputEvent>>&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_WEB_INPUT_EVENT_CONVERSION_H_
