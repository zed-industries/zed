// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_INPUT_SOURCE_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_INPUT_SOURCE_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_xr_input_source_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/xr/xr_frame.h"
#include "third_party/blink/renderer/modules/xr/xr_input_source.h"

namespace blink {

class XRInputSourceEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static XRInputSourceEvent* Create() {
    return MakeGarbageCollected<XRInputSourceEvent>();
  }
  static XRInputSourceEvent* Create(const AtomicString& type,
                                    XRFrame* frame,
                                    XRInputSource* input_source) {
    return MakeGarbageCollected<XRInputSourceEvent>(type, frame, input_source);
  }

  static XRInputSourceEvent* Create(const AtomicString& type,
                                    const XRInputSourceEventInit* initializer) {
    return MakeGarbageCollected<XRInputSourceEvent>(type, initializer);
  }

  XRInputSourceEvent();
  XRInputSourceEvent(const AtomicString& type, XRFrame*, XRInputSource*);
  XRInputSourceEvent(const AtomicString&, const XRInputSourceEventInit*);
  ~XRInputSourceEvent() override;

  XRFrame* frame() const { return frame_.Get(); }
  XRInputSource* inputSource() const { return input_source_.Get(); }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  Member<XRFrame> frame_;
  Member<XRInputSource> input_source_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_INPUT_SOURCE_EVENT_H_
