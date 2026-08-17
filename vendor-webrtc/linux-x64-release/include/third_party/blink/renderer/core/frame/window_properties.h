// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_PROPERTIES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_PROPERTIES_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"

namespace blink {
class DOMWindow;

// https://webidl.spec.whatwg.org/#dfn-named-properties-object
//
// Note that in the spec, the named properties object is not implemented as
// its own idl interface. This is an implementation convenience.
// This class may only be instantiated as a base class of DOMWindow.
class CORE_EXPORT WindowProperties : public EventTarget {
  DEFINE_WRAPPERTYPEINFO();

 public:
  v8::Local<v8::Value> AnonymousNamedGetter(const AtomicString&);

 private:
  WindowProperties() = default;
  friend class DOMWindow;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_PROPERTIES_H_
