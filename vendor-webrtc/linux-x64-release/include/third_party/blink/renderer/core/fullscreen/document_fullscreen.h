/*
 * Copyright (C) 2013, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_DOCUMENT_FULLSCREEN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_DOCUMENT_FULLSCREEN_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Document;
class Element;
class ScriptState;

class CORE_EXPORT DocumentFullscreen {
  STATIC_ONLY(DocumentFullscreen);

 public:
  static bool fullscreenEnabled(Document&);
  static Element* fullscreenElement(Document&);
  static bool fullscreen(Document& document) {
    return !!fullscreenElement(document);
  }
  static ScriptPromise<IDLUndefined> exitFullscreen(ScriptState*,
                                                    Document&,
                                                    ExceptionState&);
  static void webkitExitFullscreen(Document&);

  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(fullscreenchange, kFullscreenchange)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(fullscreenerror, kFullscreenerror)

  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(webkitfullscreenchange,
                                         kWebkitfullscreenchange)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(webkitfullscreenerror,
                                         kWebkitfullscreenerror)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FULLSCREEN_DOCUMENT_FULLSCREEN_H_
