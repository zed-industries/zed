// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_REFERENCE_SPACE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_REFERENCE_SPACE_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_xr_reference_space_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"

namespace blink {

class XRReferenceSpace;
class XRRigidTransform;

class XRReferenceSpaceEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static XRReferenceSpaceEvent* Create() {
    return MakeGarbageCollected<XRReferenceSpaceEvent>();
  }
  static XRReferenceSpaceEvent* Create(const AtomicString& type,
                                       XRReferenceSpace* reference_space) {
    return MakeGarbageCollected<XRReferenceSpaceEvent>(type, reference_space);
  }
  static XRReferenceSpaceEvent* Create(
      const AtomicString& type,
      const XRReferenceSpaceEventInit* initializer) {
    return MakeGarbageCollected<XRReferenceSpaceEvent>(type, initializer);
  }

  XRReferenceSpaceEvent();
  XRReferenceSpaceEvent(const AtomicString& type, XRReferenceSpace*);
  XRReferenceSpaceEvent(const AtomicString& type,
                        const XRReferenceSpaceEventInit*);
  ~XRReferenceSpaceEvent() override;

  XRReferenceSpace* referenceSpace() const { return reference_space_.Get(); }
  XRRigidTransform* transform() const { return transform_.Get(); }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  Member<XRReferenceSpace> reference_space_;
  Member<XRRigidTransform> transform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_REFERENCE_SPACE_EVENT_H_
