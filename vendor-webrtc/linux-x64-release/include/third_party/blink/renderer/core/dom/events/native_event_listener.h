// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_NATIVE_EVENT_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_NATIVE_EVENT_LISTENER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_listener.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// |NativeEventListener| is the base class for event listeners implemented in
// C++ and the counterpart of |JSBasedEventListener|.
class CORE_EXPORT NativeEventListener : public EventListener {
 public:
  ~NativeEventListener() override = default;

  // blink::EventListener overrides:
  bool Matches(const EventListener& other) const override {
    return this == &other;
  }

  // Helper functions for DowncastTraits.
  bool IsNativeEventListener() const override { return true; }
  virtual bool IsImageEventListener() const { return false; }

 protected:
  NativeEventListener() = default;
};

template <>
struct DowncastTraits<NativeEventListener> {
  static bool AllowFrom(const EventListener& event_listener) {
    return event_listener.IsNativeEventListener();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_EVENTS_NATIVE_EVENT_LISTENER_H_
