// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_COALESCED_INPUT_EVENT_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_COALESCED_INPUT_EVENT_MOJOM_TRAITS_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/input/web_coalesced_input_event.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/mojom/input/input_handler.mojom.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::EventDataView,
                 std::unique_ptr<blink::WebCoalescedInputEvent>> {
  static blink::WebInputEvent::Type type(
      const std::unique_ptr<blink::WebCoalescedInputEvent>& event) {
    return event->Event().GetType();
  }

  static int32_t modifiers(
      const std::unique_ptr<blink::WebCoalescedInputEvent>& event) {
    return event->Event().GetModifiers();
  }

  static base::TimeTicks timestamp(
      const std::unique_ptr<blink::WebCoalescedInputEvent>& event) {
    return event->Event().TimeStamp();
  }

  static const ui::LatencyInfo& latency(
      const std::unique_ptr<blink::WebCoalescedInputEvent>& event) {
    return event->latency_info();
  }

  static const ui::EventLatencyMetadata& event_latency_metadata(
      const std::unique_ptr<blink::WebCoalescedInputEvent>& event) {
    return event->Event().GetEventLatencyMetadata();
  }

  static blink::mojom::KeyDataPtr key_data(
      const std::unique_ptr<blink::WebCoalescedInputEvent>& event);
  static blink::mojom::PointerDataPtr pointer_data(
      const std::unique_ptr<blink::WebCoalescedInputEvent>& event);
  static blink::mojom::GestureDataPtr gesture_data(
      const std::unique_ptr<blink::WebCoalescedInputEvent>& event);
  static blink::mojom::TouchDataPtr touch_data(
      const std::unique_ptr<blink::WebCoalescedInputEvent>& event);

  static bool Read(blink::mojom::EventDataView r,
                   std::unique_ptr<blink::WebCoalescedInputEvent>* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INPUT_WEB_COALESCED_INPUT_EVENT_MOJOM_TRAITS_H_
