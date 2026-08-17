// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SESSION_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SESSION_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_xr_session_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/xr/xr_session.h"

namespace blink {

class XRSessionEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static XRSessionEvent* Create() {
    return MakeGarbageCollected<XRSessionEvent>();
  }
  static XRSessionEvent* Create(const AtomicString& type, XRSession* session) {
    return MakeGarbageCollected<XRSessionEvent>(type, session);
  }

  static XRSessionEvent* Create(const AtomicString& type,
                                const XRSessionEventInit* initializer) {
    return MakeGarbageCollected<XRSessionEvent>(type, initializer);
  }

  XRSessionEvent();
  XRSessionEvent(const AtomicString& type, XRSession*);
  XRSessionEvent(const AtomicString& type,
                 XRSession*,
                 Event::Bubbles,
                 Event::Cancelable,
                 Event::ComposedMode);
  XRSessionEvent(const AtomicString& type, const XRSessionEventInit*);
  ~XRSessionEvent() override;

  XRSession* session() const { return session_.Get(); }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  Member<XRSession> session_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SESSION_EVENT_H_
