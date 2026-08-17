// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/390223051): Remove C-library calls to fix the errors.
#pragma allow_unsafe_libc_calls
#endif

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_GESTURE_EVENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_GESTURE_EVENT_H_

#include <memory>

#include "base/check.h"
#include "base/notreached.h"
#include "cc/paint/element_id.h"
#include "third_party/blink/public/common/input/web_gesture_device.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/common/input/web_pointer_properties.h"
#include "third_party/blink/public/mojom/input/gesture_event.mojom-shared.h"
#include "ui/events/types/scroll_input_type.h"
#include "ui/events/types/scroll_types.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

// WebGestureEvent ---------------------------------------------------------

class BLINK_COMMON_EXPORT WebGestureEvent : public WebInputEvent {
 public:
  using InertialPhaseState = mojom::InertialPhaseState;

  bool is_source_touch_event_set_blocking = false;

  // The pointer type for the first touch point in the gesture.
  WebPointerProperties::PointerType primary_pointer_type =
      WebPointerProperties::PointerType::kUnknown;
  // The unique_touch_event id of the first touch in the gesture sequence
  uint32_t primary_unique_touch_event_id = 0;

  // If the WebGestureEvent has source_device ==
  // mojom::GestureDevice::kTouchscreen, this field contains the unique
  // identifier for the touch event that released this event at
  // TouchDispositionGestureFilter. If the WebGestureEvents was not released
  // through a touch event (e.g. synthesized gesture events and pinches
  // or gesture events with source_device !=
  // mojom::GestureDevice::kTouchscreen), the field contains 0. See
  // crbug.com/618738.
  uint32_t unique_touch_event_id = 0;

  union {
    // Tap information must be set for GestureTap, GestureTapUnconfirmed,
    // and GestureDoubleTap events.
    struct {
      int tap_count;
      float width;
      float height;
      // |needs_wheel_event| for touchpad GestureDoubleTap has the same
      // semantics as |needs_wheel_event| for touchpad pinch described below.
      bool needs_wheel_event;
    } tap;

    struct {
      int tap_down_count;
      float width;
      float height;
    } tap_down;

    struct {
      float width;
      float height;
    } show_press;

    // This is used for GestureShortPress , GestureLongPress and GestureLongTap.
    struct {
      float width;
      float height;
    } long_press;

    struct {
      float first_finger_width;
      float first_finger_height;
    } two_finger_tap;

    struct {
      // If set, used to target a scrollable area directly instead of performing
      // a hit-test. Should be used for gestures queued up internally within
      // the renderer process. This is an ElementIdType instead of ElementId
      // due to the fact that ElementId has a non-trivial constructor that
      // can't easily participate in this union of structs. Note that while
      // this is used in scroll unification to perform a main thread hit test,
      // in which case |main_thread_hit_tested| is true, it is also used in
      // other cases like scroll events reinjected for scrollbar scrolling.
      // Using `cc::ElementId::InternalValue` because  `cc::ElementId` has a
      // non-trivial constructor and is not allowed in a union.
      cc::ElementId::InternalValue scrollable_area_element_id;
      // Initial motion that triggered the scroll.
      float delta_x_hint;
      float delta_y_hint;
      // number of pointers down.
      int pointer_count;
      // Default initialized to kScrollByPrecisePixel.
      ui::ScrollGranularity delta_hint_units;
      // The state of inertial phase scrolling. OSX has unique phases for normal
      // and momentum scroll events. Should always be kUnknownMomentumPhase for
      // touch based input as it generates GestureFlingStart instead.
      InertialPhaseState inertial_phase;
      // If true, this event will skip hit testing to find a scroll
      // target and instead just scroll the viewport.
      bool target_viewport;
      // True if this event is generated from a mousewheel or scrollbar.
      // Synthetic GSB(s) are ignored by the blink::ElasticOverscrollController.
      bool synthetic;
      // If true, this event will be used for cursor control instead of
      // scrolling. the entire scroll sequence will be used for cursor control.
      bool cursor_control;
      // If nonzero, this event has been hit tested by the main thread and the
      // result is stored in scrollable_area_element_id. Used only in scroll
      // unification when the event is sent back the the compositor for a
      // second time after the main thread hit test is complete.
      uint32_t main_thread_hit_tested_reasons;
    } scroll_begin;

    struct {
      float delta_x;
      float delta_y;
      InertialPhaseState inertial_phase;
      // Default initialized to kScrollByPrecisePixel.
      ui::ScrollGranularity delta_units;
    } scroll_update;

    struct {
      // The original delta units the ScrollBegin and ScrollUpdates
      // were sent as.
      ui::ScrollGranularity delta_units;
      // The state of inertial phase scrolling. OSX has unique phases for normal
      // and momentum scroll events. Should always be kUnknownMomentumPhase for
      // touch based input as it generates GestureFlingStart instead.
      InertialPhaseState inertial_phase;
      // True if this event is generated from a wheel event with synthetic
      // phase.
      bool synthetic;
      // True if this event is generated by the fling controller. The GSE
      // generated at the end of a fling should not get pushed to the
      // debouncing_deferral_queue_. This attribute is not mojofied and will not
      // get transferred across since it is only used on the browser.
      bool generated_by_fling_controller;
    } scroll_end;

    struct {
      float velocity_x;
      float velocity_y;
      // If true, this event will skip hit testing to find a scroll
      // target and instead just scroll the viewport.
      bool target_viewport;
    } fling_start;

    struct {
      bool target_viewport;
      // If set to true, don't treat fling_cancel
      // as a part of fling boost events sequence.
      bool prevent_boosting;
    } fling_cancel;

    // Note that for the pinch event types, |needs_wheel_event| and
    // |zoom_disabled| are browser side implementation details for touchpad
    // pinch zoom. From the renderer's perspective, both are always false.
    // TODO(mcnee): Remove these implementation details once the browser has its
    // own representation of a touchpad pinch event. See
    // https://crbug.com/563730
    struct {
      // |needs_wheel_event| is used to indicate the phase of handling a
      // touchpad pinch. When the event is created it is set to true, so that
      // the InputRouter knows to offer the event to the renderer as a wheel
      // event. Once the wheel event is acknowledged, |needs_wheel_event| is set
      // to false and the event is resent. When the InputRouter receives it the
      // second time, it knows to send the real gesture event to the renderer.
      bool needs_wheel_event;
      // If true, this event will not cause a change in page scale, but will
      // still be offered as a wheel event to any handlers.
      bool zoom_disabled;
      float scale;
    } pinch_update;

    struct {
      bool needs_wheel_event;
    } pinch_begin;

    struct {
      bool needs_wheel_event;
    } pinch_end;
  } data;

 private:
  // Widget coordinate, which is relative to the bound of current RenderWidget
  // (e.g. a plugin or OOPIF inside a RenderView). Similar to viewport
  // coordinates but without DevTools emulation transform or overscroll applied.
  gfx::PointF position_in_widget_;

  // Screen coordinate
  gfx::PointF position_in_screen_;

  mojom::GestureDevice source_device_ = mojom::GestureDevice::kUninitialized;

 public:
  WebGestureEvent(
      Type type,
      int modifiers,
      base::TimeTicks time_stamp,
      mojom::GestureDevice device = mojom::GestureDevice::kUninitialized)
      : WebInputEvent(type, modifiers, time_stamp), source_device_(device) {
    memset(&data, 0, sizeof(data));
  }

  WebGestureEvent() { memset(&data, 0, sizeof(data)); }

  const gfx::PointF& PositionInWidget() const { return position_in_widget_; }
  const gfx::PointF& PositionInScreen() const { return position_in_screen_; }

  std::unique_ptr<WebInputEvent> Clone() const override;
  bool CanCoalesce(const WebInputEvent& event) const override;
  void Coalesce(const WebInputEvent& event) override;

  // Returns the input type of a scroll event. Should not be called on
  // non-scroll events.
  ui::ScrollInputType GetScrollInputType() const;

  void SetPositionInWidget(const gfx::PointF& point) {
    position_in_widget_ = point;
  }

  void SetPositionInScreen(const gfx::PointF& point) {
    position_in_screen_ = point;
  }

  mojom::GestureDevice SourceDevice() const { return source_device_; }
  void SetSourceDevice(mojom::GestureDevice device) { source_device_ = device; }

  float DeltaXInRootFrame() const;
  float DeltaYInRootFrame() const;
  ui::ScrollGranularity DeltaUnits() const;
  gfx::PointF PositionInRootFrame() const;
  InertialPhaseState InertialPhase() const;
  bool Synthetic() const;

  gfx::SizeF TapAreaInRootFrame() const;
  int TapCount() const;
  int TapDownCount() const;

  void ApplyTouchAdjustment(const gfx::PointF& root_frame_coords);

  // Sets any scaled values to be their computed values and sets |frame_scale_|
  // back to 1 and |frame_translate_| X and Y coordinates back to 0.
  void FlattenTransform();

  bool IsScrollEvent() const {
    switch (type_) {
      case Type::kGestureScrollBegin:
      case Type::kGestureScrollEnd:
      case Type::kGestureScrollUpdate:
      case Type::kGestureFlingStart:
      case Type::kGestureFlingCancel:
      case Type::kGesturePinchBegin:
      case Type::kGesturePinchEnd:
      case Type::kGesturePinchUpdate:
        return true;
      case Type::kGestureTap:
      case Type::kGestureTapUnconfirmed:
      case Type::kGestureTapDown:
      case Type::kGestureShowPress:
      case Type::kGestureTapCancel:
      case Type::kGestureTwoFingerTap:
      case Type::kGestureShortPress:
      case Type::kGestureLongPress:
      case Type::kGestureLongTap:
      case Type::kGestureDoubleTap:
        return false;
      default:
        NOTREACHED();
    }
  }

  bool IsTargetViewport() const {
    switch (type_) {
      case Type::kGestureScrollBegin:
        return data.scroll_begin.target_viewport;
      case Type::kGestureFlingStart:
        return data.fling_start.target_viewport;
      case Type::kGestureFlingCancel:
        return data.fling_cancel.target_viewport;
      default:
        return false;
    }
  }

  bool IsTouchpadZoomEvent() const {
    // Touchpad GestureDoubleTap also causes a page scale change like a touchpad
    // pinch gesture.
    return source_device_ == mojom::GestureDevice::kTouchpad &&
           (WebInputEvent::IsPinchGestureEventType(type_) ||
            type_ == Type::kGestureDoubleTap);
  }

  bool NeedsWheelEvent() const {
    DCHECK(IsTouchpadZoomEvent());
    switch (type_) {
      case Type::kGesturePinchBegin:
        return data.pinch_begin.needs_wheel_event;
      case Type::kGesturePinchUpdate:
        return data.pinch_update.needs_wheel_event;
      case Type::kGesturePinchEnd:
        return data.pinch_end.needs_wheel_event;
      case Type::kGestureDoubleTap:
        return data.tap.needs_wheel_event;
      default:
        NOTREACHED();
    }
  }

  void SetNeedsWheelEvent(bool needs_wheel_event) {
    DCHECK(!needs_wheel_event || IsTouchpadZoomEvent());
    switch (type_) {
      case Type::kGesturePinchBegin:
        data.pinch_begin.needs_wheel_event = needs_wheel_event;
        break;
      case Type::kGesturePinchUpdate:
        data.pinch_update.needs_wheel_event = needs_wheel_event;
        break;
      case Type::kGesturePinchEnd:
        data.pinch_end.needs_wheel_event = needs_wheel_event;
        break;
      case Type::kGestureDoubleTap:
        data.tap.needs_wheel_event = needs_wheel_event;
        break;
      default:
        break;
    }
  }

  // Coalesce the |new_event| with |last_event| and optionally
  // |second_last_event|. Scroll and pinch are two separate gestures so they
  // would need separate events that is why this method returns a pair.
  static std::pair<std::unique_ptr<WebGestureEvent>,
                   std::unique_ptr<WebGestureEvent>>
  CoalesceScrollAndPinch(const WebGestureEvent* second_last_event,
                         const WebGestureEvent& last_event,
                         const WebGestureEvent& new_event);

  // Whether |event_in_queue| is a touchscreen GesturePinchUpdate or
  // GestureScrollUpdate and has the same modifiers/source as the new
  // scroll/pinch event. Compatible touchscreen scroll and pinch event pairs
  // can be logically coalesced.
  static bool IsCompatibleScrollorPinch(const WebGestureEvent& new_event,
                                        const WebGestureEvent& event_in_queue);

  // For a scrollbar gesture, generate a scroll gesture event (begin, update,
  // or end), based on the parameters passed in. Populates the data field of
  // the created WebGestureEvent based on the type.
  static std::unique_ptr<blink::WebGestureEvent>
  GenerateInjectedScrollbarGestureScroll(WebInputEvent::Type type,
                                         base::TimeTicks timestamp,
                                         gfx::PointF position_in_widget,
                                         gfx::Vector2dF scroll_delta,
                                         ui::ScrollGranularity granularity);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_GESTURE_EVENT_H_
