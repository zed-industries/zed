// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_MEDIA_ELEMENT_CAPTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_MEDIA_ELEMENT_CAPTURE_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ExceptionState;
class HTMLMediaElement;
class MediaStream;
class ScriptState;

class HTMLMediaElementCapture {
  STATIC_ONLY(HTMLMediaElementCapture);

 public:
  static MediaStream* captureStream(ScriptState*,
                                    HTMLMediaElement&,
                                    ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_MEDIA_ELEMENT_CAPTURE_H_
