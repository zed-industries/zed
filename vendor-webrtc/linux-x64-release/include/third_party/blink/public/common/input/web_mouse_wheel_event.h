// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_MOUSE_WHEEL_EVENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_MOUSE_WHEEL_EVENT_H_

#include "third_party/blink/public/common/input/web_mouse_event.h"
#include "ui/events/types/scroll_types.h"

namespace blink {

// WebMouseWheelEvent ---------------------------------------------------------

class BLINK_COMMON_EXPORT WebMouseWheelEvent : public WebMouseEvent {
 public:
  enum Phase {
    // No phase information is avaiable.
    kPhaseNone = 0,
    // This wheel event is the beginning of a scrolling sequence.
    kPhaseBegan = 1 << 0,
    // Shows that scrolling is ongoing but the scroll delta for this wheel event
    // is zero.
    kPhaseStationary = 1 << 1,
    // Shows that a scroll is ongoing and the scroll delta for this wheel event
    // is non-zero.
    kPhaseChanged = 1 << 2,
    // This wheel event is the last event of a scrolling sequence.
    kPhaseEnded = 1 << 3,
    // A wheel event with phase cancelled shows that the scrolling sequence is
    // cancelled.
    kPhaseCancelled = 1 << 4,
    // A wheel event with phase may begin shows that a scrolling sequence may
    // start.
    kPhaseMayBegin = 1 << 5,
    // A wheel event with momentum phase blocked shows that a scrolling sequence
    // will not be followed by a momentum fling. This should only ever be set on
    // the momentum phase of an event.
    kPhaseBlocked = 1 << 6,
  };

  // A hint at the outcome of a wheel event should it not get canceled.
  enum class EventAction : int {
    // When the wheel event would result in page zoom,
    kPageZoom = 0,
    // When the wheel event would scroll but the direction is not (known to be)
    // fixed to a certain axis,
    kScroll,
    // When the wheel event would scroll along X axis,
    kScrollHorizontal,
    // When the wheel event would scroll along Y axis,
    kScrollVertical
  };

  float delta_x = 0.0f;
  float delta_y = 0.0f;
  float wheel_ticks_x = 0.0f;
  float wheel_ticks_y = 0.0f;

  float acceleration_ratio_x = 1.0f;
  float acceleration_ratio_y = 1.0f;

  Phase phase = kPhaseNone;
  Phase momentum_phase = kPhaseNone;

  RailsMode rails_mode = kRailsModeFree;

  // Whether the event is blocking, non-blocking, all event
  // listeners were passive or was forced to be non-blocking.
  DispatchType dispatch_type = DispatchType::kBlocking;

  // The expected result of this wheel event (if not canceled).
  EventAction event_action = EventAction::kPageZoom;

  // True when phase information is added in mouse_wheel_phase_handler based
  // on its timer.
  bool has_synthetic_phase = false;

  // The units of delta_x and delta_y. Currently only supports
  // kScrollByPrecisePixel, kScrollByPixel, and kScrollByPage, as they are
  // the only values expected after converting an OS event to a
  // WebMouseWheelEvent.
  ui::ScrollGranularity delta_units = ui::ScrollGranularity::kScrollByPixel;

  WebMouseWheelEvent(Type type, int modifiers, base::TimeTicks time_stamp)
      : WebMouseEvent(type, modifiers, time_stamp, kMousePointerId) {}

  WebMouseWheelEvent() : WebMouseEvent(kMousePointerId) {}

  float DeltaXInRootFrame() const;
  float DeltaYInRootFrame() const;

  // Sets any scaled values to be their computed values and sets |frame_scale_|
  // back to 1 and |frame_translate_| X and Y coordinates back to 0.
  WebMouseWheelEvent FlattenTransform() const;

  bool IsCancelable() const { return dispatch_type == DispatchType::kBlocking; }

  std::unique_ptr<WebInputEvent> Clone() const override;
  bool CanCoalesce(const WebInputEvent& event) const override;
  void Coalesce(const WebInputEvent& event) override;

  // Return the platform specific default event action given the mouse wheel
  // event. Can be used to determine the appropriate value for |event_action|.
  static EventAction GetPlatformSpecificDefaultEventAction(
      const WebMouseWheelEvent& event);

 private:
  bool HaveConsistentPhase(const WebMouseWheelEvent& event) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_MOUSE_WHEEL_EVENT_H_
