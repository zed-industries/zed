// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_INPUT_SOURCES_CHANGE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_INPUT_SOURCES_CHANGE_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_xr_input_sources_change_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/xr/xr_input_source.h"
#include "third_party/blink/renderer/modules/xr/xr_session.h"

namespace blink {

template <typename IDLType>
class FrozenArray;

class XRInputSourcesChangeEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static XRInputSourcesChangeEvent* Create(
      const AtomicString& type,
      XRSession* session,
      HeapVector<Member<XRInputSource>> added,
      HeapVector<Member<XRInputSource>> removed) {
    return MakeGarbageCollected<XRInputSourcesChangeEvent>(
        type, session, std::move(added), std::move(removed));
  }

  static XRInputSourcesChangeEvent* Create(
      const AtomicString& type,
      const XRInputSourcesChangeEventInit* initializer) {
    return MakeGarbageCollected<XRInputSourcesChangeEvent>(type, initializer);
  }

  XRInputSourcesChangeEvent(const AtomicString& type,
                            XRSession* session,
                            HeapVector<Member<XRInputSource>> added,
                            HeapVector<Member<XRInputSource>> removed);
  XRInputSourcesChangeEvent(const AtomicString& type,
                            const XRInputSourcesChangeEventInit* initializer);
  ~XRInputSourcesChangeEvent() override;

  XRSession* session() const { return session_.Get(); }
  const FrozenArray<XRInputSource>& added() const { return *added_.Get(); }
  const FrozenArray<XRInputSource>& removed() const { return *removed_.Get(); }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  Member<XRSession> session_;
  Member<FrozenArray<XRInputSource>> added_;
  Member<FrozenArray<XRInputSource>> removed_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_INPUT_SOURCES_CHANGE_EVENT_H_
