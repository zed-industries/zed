// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_TEST_MOJO_INTERFACE_REQUEST_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_TEST_MOJO_INTERFACE_REQUEST_EVENT_H_

#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/event_interface_names.h"

namespace blink {

class MojoHandle;
class MojoInterfaceRequestEventInit;

// An event dispatched to a MojoInterfaceInterceptor when its frame sends an
// outgoing request for the interface it was configured to intercept. The event
// includes the intercepted MojoHandle of the request pipe, which a listener may
// use to bind the request to e.g. a mock interface implementation.
class MojoInterfaceRequestEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~MojoInterfaceRequestEvent() override;

  static MojoInterfaceRequestEvent* Create(
      const AtomicString& type,
      const MojoInterfaceRequestEventInit* initializer) {
    return MakeGarbageCollected<MojoInterfaceRequestEvent>(type, initializer);
  }

  explicit MojoInterfaceRequestEvent(MojoHandle*);
  MojoInterfaceRequestEvent(const AtomicString& type,
                            const MojoInterfaceRequestEventInit*);

  MojoHandle* handle() const { return handle_.Get(); }

  const AtomicString& InterfaceName() const override {
    return event_interface_names::kMojoInterfaceRequestEvent;
  }

  void Trace(Visitor*) const override;

 private:
  Member<MojoHandle> handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_TEST_MOJO_INTERFACE_REQUEST_EVENT_H_
