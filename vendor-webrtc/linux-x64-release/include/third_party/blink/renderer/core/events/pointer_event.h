// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_POINTER_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_POINTER_EVENT_H_

#include "third_party/blink/public/common/input/pointer_id.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/events/mouse_event.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class PointerEventInit;

class CORE_EXPORT PointerEvent : public MouseEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PointerEvent* Create(
      const AtomicString& type,
      const PointerEventInit* initializer,
      base::TimeTicks platform_time_stamp = base::TimeTicks::Now(),
      MouseEvent::SyntheticEventType synthetic_event_type =
          kRealOrIndistinguishable,
      WebMenuSourceType menu_source_type = kMenuSourceNone,
      bool prevent_counting_as_interaction = false) {
    return MakeGarbageCollected<PointerEvent>(
        type, initializer, platform_time_stamp, synthetic_event_type,
        menu_source_type, prevent_counting_as_interaction);
  }

  PointerEvent(const AtomicString&,
               const PointerEventInit*,
               base::TimeTicks platform_time_stamp,
               MouseEvent::SyntheticEventType synthetic_event_type,
               WebMenuSourceType menu_source_type = kMenuSourceNone,
               bool prevent_counting_as_interaction = false);

  PointerId pointerId() const { return pointer_id_; }
  PointerId pointerIdForBindings() const;
  double width() const { return width_; }
  double height() const { return height_; }
  float pressure() const { return pressure_; }
  int32_t tiltX() const { return tilt_x_; }
  int32_t tiltY() const { return tilt_y_; }
  double azimuthAngle() const { return azimuth_angle_; }
  double altitudeAngle() const { return altitude_angle_; }
  float tangentialPressure() const { return tangential_pressure_; }
  int32_t twist() const { return twist_; }
  const String& pointerType() const { return pointer_type_; }
  bool isPrimary() const { return is_primary_; }

  int16_t button() const override { return RawButton(); }
  bool IsMouseEvent() const override;
  bool IsPointerEvent() const override;

  double screenX() const override {
    if (ShouldHaveIntegerCoordinates())
      return MouseEvent::screenX();
    return screen_x_;
  }
  double screenY() const override {
    if (ShouldHaveIntegerCoordinates())
      return MouseEvent::screenY();
    return screen_y_;
  }
  double clientX() const override {
    if (ShouldHaveIntegerCoordinates())
      return MouseEvent::clientX();
    return client_x_;
  }
  double clientY() const override {
    if (ShouldHaveIntegerCoordinates())
      return MouseEvent::clientY();
    return client_y_;
  }
  double pageX() const override {
    if (ShouldHaveIntegerCoordinates())
      return MouseEvent::pageX();
    return page_x_;
  }
  double pageY() const override {
    if (ShouldHaveIntegerCoordinates())
      return MouseEvent::pageY();
    return page_y_;
  }

  double offsetX() const override;
  double offsetY() const override;

  void ReceivedTarget() override;

  // Always return null for fromElement and toElement because these fields
  // (inherited from MouseEvents) are non-standard.
  Node* fromElement() const final;
  Node* toElement() const final;

  HeapVector<Member<PointerEvent>> getCoalescedEvents();
  HeapVector<Member<PointerEvent>> getPredictedEvents();
  base::TimeTicks OldestPlatformTimeStamp() const;

  DispatchEventResult DispatchEvent(EventDispatcher&) override;

  Document* GetDocument() const;

  int32_t persistentDeviceId() const { return persistent_device_id_; }

  bool GetPreventCountingAsInteraction() const {
    return prevent_counting_as_interaction_;
  }

  void Trace(Visitor*) const override;

 private:
  bool ShouldHaveIntegerCoordinates() const;

  PointerId pointer_id_;
  double width_;
  double height_;
  float pressure_;
  int32_t tilt_x_;
  int32_t tilt_y_;
  double azimuth_angle_;
  double altitude_angle_;
  float tangential_pressure_;
  int32_t twist_;
  String pointer_type_;
  bool is_primary_;

  bool coalesced_events_targets_dirty_;
  bool predicted_events_targets_dirty_;

  HeapVector<Member<PointerEvent>> coalesced_events_;

  HeapVector<Member<PointerEvent>> predicted_events_;

  int32_t persistent_device_id_;

  // See equivalent member in web_input_event.h.
  bool prevent_counting_as_interaction_ = false;
};

template <>
struct DowncastTraits<PointerEvent> {
  static bool AllowFrom(const Event& event) { return event.IsPointerEvent(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_POINTER_EVENT_H_
