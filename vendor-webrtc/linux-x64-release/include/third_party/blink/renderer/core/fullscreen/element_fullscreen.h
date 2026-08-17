// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_ELEMENT_FULLSCREEN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_ELEMENT_FULLSCREEN_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Element;
class FullscreenOptions;
class ScriptState;

class CORE_EXPORT ElementFullscreen {
  STATIC_ONLY(ElementFullscreen);

 public:
  static ScriptPromise<IDLUndefined> requestFullscreen(
      ScriptState*,
      Element&,
      const FullscreenOptions*,
      ExceptionState& exception_state);

  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(fullscreenchange, kFullscreenchange)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(fullscreenerror, kFullscreenerror)

  static void webkitRequestFullscreen(Element&);
  static void webkitRequestFullscreen(Element&, const FullscreenOptions*);

  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(webkitfullscreenchange,
                                         kWebkitfullscreenchange)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(webkitfullscreenerror,
                                         kWebkitfullscreenerror)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_ELEMENT_FULLSCREEN_H_
