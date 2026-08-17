// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_ON_REQUEST_CANVAS_DRAW_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_ON_REQUEST_CANVAS_DRAW_LISTENER_H_

#include <memory>
#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/modules/mediacapturefromelement/auto_canvas_draw_listener.h"
#include "third_party/blink/renderer/modules/mediacapturefromelement/canvas_capture_handler.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/skia/include/core/SkRefCnt.h"

namespace blink {

class OnRequestCanvasDrawListener : public AutoCanvasDrawListener {
 public:
  explicit OnRequestCanvasDrawListener(std::unique_ptr<CanvasCaptureHandler>);
  ~OnRequestCanvasDrawListener() override;

  NewFrameCallback GetNewFrameCallback() final;

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_ON_REQUEST_CANVAS_DRAW_LISTENER_H_
