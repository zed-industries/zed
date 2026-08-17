// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_WORKLET_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_WORKLET_PAINTER_H_

#include "cc/paint/paint_worklet_input.h"
#include "cc/paint/paint_worklet_job.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// This class exists for layering needs, and it is implemented by
// PaintWorkletProxyClient.
//
// PaintWorkletProxyClient lives in modules/csspaint as it needs to
// call the worklet code there. However it is referenced from
// PaintWorkletPaintDispatcher, which lives in platform/graphics,
// which is not allowed to depend on modules/csspaint.
// PaintWorkletPaintDispatcher cannot be moved into modules/csspaint as it is
// referenced from elsewhere in core/ (which also cannot depend on modules).
// Therefore an intermediate interface is required to solve the layering issue.
//
// TODO(xidachen): consider making this a delegate class of
// PaintWorkletPaintDispatcher.
class PLATFORM_EXPORT PaintWorkletPainter : public GarbageCollectedMixin {
 public:
  virtual ~PaintWorkletPainter() = default;

  virtual int GetWorkletId() const = 0;
  virtual PaintRecord Paint(const cc::PaintWorkletInput*,
                            const cc::PaintWorkletJob::AnimatedPropertyValues&
                                animated_property_values) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_WORKLET_PAINTER_H_
