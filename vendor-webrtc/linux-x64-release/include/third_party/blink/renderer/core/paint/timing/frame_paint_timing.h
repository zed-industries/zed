// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_FRAME_PAINT_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_FRAME_PAINT_TIMING_H_

#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/core/paint/timing/paint_timing.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_controller.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LocalFrame;

class FramePaintTiming {
  STACK_ALLOCATED();

 public:
  FramePaintTiming(GraphicsContext& context, const LocalFrame* frame)
      : context_(context), frame_(frame) {
    context_.GetPaintController().BeginFrame(frame_);
  }
  FramePaintTiming(const FramePaintTiming&) = delete;
  FramePaintTiming& operator=(const FramePaintTiming&) = delete;

  ~FramePaintTiming() {
    DCHECK(frame_->GetDocument());
    FrameFirstPaint result = context_.GetPaintController().EndFrame(frame_);
    PaintTiming::From(*frame_->GetDocument())
        .NotifyPaint(result.first_painted, result.text_painted,
                     result.image_painted);
  }

 private:
  GraphicsContext& context_;
  const LocalFrame* frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_FRAME_PAINT_TIMING_H_
