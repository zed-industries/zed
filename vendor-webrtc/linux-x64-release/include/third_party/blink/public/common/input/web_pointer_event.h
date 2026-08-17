// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_POINTER_EVENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_POINTER_EVENT_H_

#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/input/web_mouse_event.h"
#include "third_party/blink/public/common/input/web_pointer_properties.h"
#include "third_party/blink/public/common/input/web_touch_event.h"

namespace blink {

// WebPointerEvent
// This is a WIP and currently used only in Blink and only for touch.
// TODO(nzolghadr): We should unify the fields in this class into
// WebPointerProperties and not have pointertype specific attributes here.
// --------------------------------------------------------------

class BLINK_COMMON_EXPORT WebPointerEvent : public WebInputEvent,
                                            public WebPointerProperties {
 public:
  WebPointerEvent() : WebPointerProperties(0) {}
  WebPointerEvent(WebInputEvent::Type type_param,
                  WebPointerProperties web_pointer_properties_param,
                  float width_param,
                  float height_param)
      : WebPointerProperties(web_pointer_properties_param),
        width(width_param),
        height(height_param) {
    SetType(type_param);
  }
  WebPointerEvent(const WebTouchEvent&, const WebTouchPoint&);
  WebPointerEvent(WebInputEvent::Type, const WebMouseEvent&);

  std::unique_ptr<WebInputEvent> Clone() const override;
  bool CanCoalesce(const WebInputEvent& event) const override;
  void Coalesce(const WebInputEvent& event) override;

  static WebPointerEvent CreatePointerCausesUaActionEvent(
      WebPointerProperties::PointerType,
      base::TimeTicks time_stamp);

  // ------------ Touch Point Specific ------------

  float rotation_angle = 0.0f;

  // ------------ Touch Event Specific ------------

  // A unique identifier for the touch event. Valid ids start at one and
  // increase monotonically. Zero means an unknown id.
  uint32_t unique_touch_event_id = 0;

  // Whether the event is blocking, non-blocking, all event
  // listeners were passive or was forced to be non-blocking.
  DispatchType dispatch_type = DispatchType::kBlocking;

  // For a single touch, this is true after the touch-point has moved beyond
  // the platform slop region. For a multitouch, this is true after any
  // touch-point has moved (by whatever amount).
  bool moved_beyond_slop_region = false;

  // Whether this touch event is a touchstart or a first touchmove event per
  // scroll.
  bool touch_start_or_first_touch_move = false;

  // ------------ Common fields across pointer types ------------

  // True if this pointer was hovering and false otherwise. False value entails
  // the event was processed as part of gesture detection and it may cause
  // scrolling.
  bool hovering = false;

  // TODO(crbug.com/736014): We need a clarified definition of the scale and
  // the coordinate space on these attributes.
  float width = 0.0f;
  float height = 0.0f;

  bool IsCancelable() const { return dispatch_type == DispatchType::kBlocking; }
  bool HasWidth() const { return !std::isnan(width); }
  bool HasHeight() const { return !std::isnan(height); }

  WebPointerEvent WebPointerEventInRootFrame() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_POINTER_EVENT_H_
