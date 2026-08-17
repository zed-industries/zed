// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_INPUT_DEVICE_CAPABILITIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_INPUT_DEVICE_CAPABILITIES_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class InputDeviceCapabilitiesInit;

class CORE_EXPORT InputDeviceCapabilities final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static InputDeviceCapabilities* Create(bool fires_touch_events) {
    return MakeGarbageCollected<InputDeviceCapabilities>(fires_touch_events);
  }

  static InputDeviceCapabilities* Create(
      const InputDeviceCapabilitiesInit* initializer) {
    return MakeGarbageCollected<InputDeviceCapabilities>(initializer);
  }

  InputDeviceCapabilities(bool fires_touch_events);
  InputDeviceCapabilities(const InputDeviceCapabilitiesInit*);

  bool firesTouchEvents() const { return fires_touch_events_; }

 private:
  // Whether this device dispatches touch events. This mainly lets developers
  // avoid handling both touch and mouse events dispatched for a single user
  // action.
  bool fires_touch_events_;
};

// Grouping constant-valued InputDeviceCapabilities objects together,
// which is kept and used by each 'view' (DOMWindow) that dispatches
// events parameterized over InputDeviceCapabilities.
//
// TODO(sof): lazily instantiate InputDeviceCapabilities instances upon
// UIEvent access instead. This would allow internal tracking of such
// capabilities by value.
class InputDeviceCapabilitiesConstants final
    : public GarbageCollected<InputDeviceCapabilitiesConstants> {
 public:
  // Returns an InputDeviceCapabilities which has
  // |firesTouchEvents| set to value of |firesTouch|.
  InputDeviceCapabilities* FiresTouchEvents(bool fires_touch);

  void Trace(Visitor* visitor) const {
    visitor->Trace(fires_touch_events_);
    visitor->Trace(doesnt_fire_touch_events_);
  }

 private:
  Member<InputDeviceCapabilities> fires_touch_events_;
  Member<InputDeviceCapabilities> doesnt_fire_touch_events_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_INPUT_DEVICE_CAPABILITIES_H_
