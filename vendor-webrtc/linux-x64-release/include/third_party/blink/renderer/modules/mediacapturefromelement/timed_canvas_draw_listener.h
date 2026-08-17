// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_TIMED_CANVAS_DRAW_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_TIMED_CANVAS_DRAW_LISTENER_H_

#include <memory>
#include "base/memory/weak_ptr.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/modules/mediacapturefromelement/canvas_capture_handler.h"
#include "third_party/blink/renderer/modules/mediacapturefromelement/on_request_canvas_draw_listener.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/skia/include/core/SkRefCnt.h"

namespace blink {
class ExecutionContext;

class TimedCanvasDrawListener final : public OnRequestCanvasDrawListener {
 public:
  TimedCanvasDrawListener(std::unique_ptr<CanvasCaptureHandler>,
                          double frame_rate,
                          ExecutionContext*);
  ~TimedCanvasDrawListener() override;

  static TimedCanvasDrawListener* Create(std::unique_ptr<CanvasCaptureHandler>,
                                         double frame_rate,
                                         ExecutionContext*);
  void Trace(Visitor*) const override;

 private:
  // Implementation of TimerFiredFunction.
  void RequestFrameTimerFired(TimerBase*);

  base::TimeDelta frame_interval_;
  HeapTaskRunnerTimer<TimedCanvasDrawListener> request_frame_timer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIACAPTUREFROMELEMENT_TIMED_CANVAS_DRAW_LISTENER_H_
