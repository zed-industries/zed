// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_TOUCH_EVENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_TOUCH_EVENT_H_

#include <array>

#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/input/web_touch_point.h"

namespace blink {

// WebTouchEvent --------------------------------------------------------------

// TODO(e_hakkinen): Replace with WebPointerEvent. crbug.com/508283
class BLINK_COMMON_EXPORT WebTouchEvent : public WebInputEvent {
 public:
  // Maximum number of simultaneous touches supported on
  // Ash/Aura.
  enum { kTouchesLengthCap = 16 };

  unsigned touches_length = 0;
  // List of all touches, regardless of state.
  std::array<WebTouchPoint, kTouchesLengthCap> touches = {};

  // Whether the event is blocking, non-blocking, all event
  // listeners were passive or was forced to be non-blocking.
  DispatchType dispatch_type = DispatchType::kBlocking;

  // For a single touch, this is true after the touch-point has moved beyond
  // the platform slop region. For a multitouch, this is true after any
  // touch-point has moved (by whatever amount).
  bool moved_beyond_slop_region = false;

  // True for events from devices like some pens that support hovering
  // over digitizer and the events are sent while the device was hovering.
  bool hovering = false;

  // Whether this touch event is a touchstart or a first touchmove event per
  // scroll.
  bool touch_start_or_first_touch_move = false;

  // A unique identifier for the touch event. Valid ids start at one and
  // increase monotonically. Zero means an unknown id.
  uint32_t unique_touch_event_id = 0;

  WebTouchEvent() = default;

  WebTouchEvent(Type type, int modifiers, base::TimeTicks time_stamp)
      : WebInputEvent(type, modifiers, time_stamp) {}

  std::unique_ptr<WebInputEvent> Clone() const override;
  bool CanCoalesce(const WebInputEvent& event) const override;
  void Coalesce(const WebInputEvent& event) override;

  // Sets any scaled values to be their computed values and sets |frame_scale_|
  // back to 1 and |frame_translate_| X and Y coordinates back to 0.
  WebTouchEvent FlattenTransform() const;

  // Return a scaled WebTouchPoint in root frame coordinates.
  WebTouchPoint TouchPointInRootFrame(unsigned touch_point) const;

  bool IsCancelable() const { return dispatch_type == DispatchType::kBlocking; }

  // Returns whether this event represents a transition from no active
  // touches to some active touches (the start of a new "touch sequence").
  bool IsTouchSequenceStart() const;

  // Returns whether this event represents a transition from active
  // touches to no active touches (the end of a "touch sequence").
  bool IsTouchSequenceEnd() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_TOUCH_EVENT_H_
