// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DRAWING_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DRAWING_RECORDER_H_

#include <optional>

#include "base/auto_reset.h"
#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/graphics/paint/drawing_display_item.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_controller.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class GraphicsContext;

class PLATFORM_EXPORT DrawingRecorder {
  STACK_ALLOCATED();

 public:
  static bool UseCachedDrawingIfPossible(GraphicsContext& context,
                                         const DisplayItemClient& client,
                                         DisplayItem::Type type) {
    return context.GetPaintController().UseCachedItemIfPossible(client, type);
  }

  static bool UseCachedDrawingIfPossible(GraphicsContext& context,
                                         const DisplayItemClient& client,
                                         PaintPhase phase) {
    return UseCachedDrawingIfPossible(
        context, client, DisplayItem::PaintPhaseToDrawingType(phase));
  }

  // See DisplayItem::VisualRect() for the definition of visual rect.
  DrawingRecorder(GraphicsContext&,
                  const DisplayItemClient&,
                  DisplayItem::Type,
                  const gfx::Rect& visual_rect);

  DrawingRecorder(GraphicsContext& context,
                  const DisplayItemClient& client,
                  PaintPhase phase,
                  const gfx::Rect& visual_rect)
      : DrawingRecorder(context,
                        client,
                        DisplayItem::PaintPhaseToDrawingType(phase),
                        visual_rect) {}

  // This form is for recording with a paint controller without persistent
  // data, e.g. when we are recording into a PaintRecordBuilder, where visual
  // rect doesn't matter.
  DrawingRecorder(GraphicsContext& context,
                  const DisplayItemClient& client,
                  DisplayItem::Type type)
      : DrawingRecorder(context, client, type, gfx::Rect()) {
    DCHECK(!context.GetPaintController().HasPersistentData());
  }
  DrawingRecorder(GraphicsContext& context,
                  const DisplayItemClient& client,
                  PaintPhase phase)
      : DrawingRecorder(context,
                        client,
                        DisplayItem::PaintPhaseToDrawingType(phase)) {}

  DrawingRecorder(const DrawingRecorder&) = delete;
  DrawingRecorder& operator=(const DrawingRecorder&) = delete;
  ~DrawingRecorder();

  // Sometimes we don't the the exact visual rect when we create a
  // DrawingRecorder. This method allows visual rect to be added during
  // painting.
  void UniteVisualRect(const gfx::Rect& rect) { visual_rect_.Union(rect); }

 private:
  GraphicsContext& context_;
  const DisplayItemClient& client_;
  const DisplayItem::Type type_;
  gfx::Rect visual_rect_;
  std::optional<DOMNodeId> dom_node_id_to_restore_;
};

#if DCHECK_IS_ON()
class DisableListModificationCheck {
  STACK_ALLOCATED();

 public:
  DisableListModificationCheck();
  DisableListModificationCheck(const DisableListModificationCheck&) = delete;
  DisableListModificationCheck& operator=(const DisableListModificationCheck&) =
      delete;

 private:
  base::AutoReset<bool> disabler_;
};
#endif  // DCHECK_IS_ON()

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DRAWING_RECORDER_H_
