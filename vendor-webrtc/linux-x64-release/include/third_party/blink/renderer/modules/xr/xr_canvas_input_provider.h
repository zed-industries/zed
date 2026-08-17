// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_CANVAS_INPUT_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_CANVAS_INPUT_PROVIDER_H_

#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

class EventListener;
class HTMLCanvasElement;
class PointerEvent;
class XRInputSource;
class XRSession;

class XRCanvasInputProvider : public GarbageCollected<XRCanvasInputProvider>,
                              public NameClient {
 public:
  XRCanvasInputProvider(XRSession*, HTMLCanvasElement*);
  ~XRCanvasInputProvider() override;

  XRSession* session() const { return session_.Get(); }
  HTMLCanvasElement* canvas() const { return canvas_.Get(); }

  // Remove all event listeners.
  void Stop();

  bool ShouldProcessEvents();

  void OnPointerDown(PointerEvent*);
  void OnPointerUp(PointerEvent*);

  XRInputSource* GetInputSource();

  virtual void Trace(Visitor*) const;
  const char* NameInHeapSnapshot() const override {
    return "XRCanvasInputProvider";
  }

 private:
  void UpdateInputSource(PointerEvent*);
  void ClearInputSource();

  const Member<XRSession> session_;
  Member<HTMLCanvasElement> canvas_;
  Member<EventListener> listener_;
  Member<XRInputSource> input_source_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_CANVAS_INPUT_PROVIDER_H_
