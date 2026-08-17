// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ERROR_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ERROR_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_error_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_error.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/webrtc/api/rtc_error.h"

namespace blink {

class RTCErrorEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static RTCErrorEvent* Create(const AtomicString& type,
                               const RTCErrorEventInit* event_init_dict);

  RTCErrorEvent(const AtomicString& type,
                const RTCErrorEventInit* event_init_dict);

  RTCErrorEvent(const AtomicString& type, webrtc::RTCError error);

  RTCError* error() const;

  void Trace(Visitor*) const override;

 private:
  Member<RTCError> error_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ERROR_EVENT_H_
