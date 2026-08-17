// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_HTML_VIDEO_ELEMENT_PICTURE_IN_PICTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_HTML_VIDEO_ELEMENT_PICTURE_IN_PICTURE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"

namespace blink {

class ExceptionState;
class HTMLVideoElement;
class PictureInPictureWindow;
class ScriptState;

class HTMLVideoElementPictureInPicture {
  STATIC_ONLY(HTMLVideoElementPictureInPicture);

 public:
  static ScriptPromise<PictureInPictureWindow>
  requestPictureInPicture(ScriptState*, HTMLVideoElement&, ExceptionState&);

  static bool FastHasAttribute(const HTMLVideoElement&, const QualifiedName&);

  static void SetBooleanAttribute(HTMLVideoElement&,
                                  const QualifiedName&,
                                  bool);

  static void CheckIfPictureInPictureIsAllowed(HTMLVideoElement&,
                                               ExceptionState&);

  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(enterpictureinpicture,
                                         kEnterpictureinpicture)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(leavepictureinpicture,
                                         kLeavepictureinpicture)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_HTML_VIDEO_ELEMENT_PICTURE_IN_PICTURE_H_
