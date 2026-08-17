// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRIVATE_ATTRIBUTION_WINDOW_PRIVATE_ATTRIBUTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRIVATE_ATTRIBUTION_WINDOW_PRIVATE_ATTRIBUTION_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class LocalDOMWindow;
class PrivateAttribution;

// Implement privateAttribution attribute under Window.
class WindowPrivateAttribution final
    : public GarbageCollected<WindowPrivateAttribution>,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  static WindowPrivateAttribution& From(LocalDOMWindow&);
  static PrivateAttribution* privateAttribution(LocalDOMWindow&);

  explicit WindowPrivateAttribution(LocalDOMWindow&);

  PrivateAttribution* privateAttribution();

  void Trace(Visitor*) const override;

 private:
  Member<PrivateAttribution> private_attribution_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRIVATE_ATTRIBUTION_WINDOW_PRIVATE_ATTRIBUTION_H_
