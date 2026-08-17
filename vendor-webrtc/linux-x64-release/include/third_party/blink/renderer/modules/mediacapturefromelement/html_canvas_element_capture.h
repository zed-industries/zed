// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_CANVAS_ELEMENT_CAPTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_CANVAS_ELEMENT_CAPTURE_H_

#include "third_party/blink/renderer/core/html/canvas/html_canvas_element.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class MediaStream;
class ExceptionState;

class HTMLCanvasElementCapture {
  STATIC_ONLY(HTMLCanvasElementCapture);

 public:
  static MediaStream* captureStream(ScriptState*,
                                    HTMLCanvasElement&,
                                    ExceptionState&);
  static MediaStream* captureStream(ScriptState*,
                                    HTMLCanvasElement&,
                                    double frame_rate,
                                    ExceptionState&);

 private:
  static MediaStream* captureStream(ScriptState*,
                                    HTMLCanvasElement&,
                                    bool given_frame_rate,
                                    double frame_rate,
                                    ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_HTML_CANVAS_ELEMENT_CAPTURE_H_
