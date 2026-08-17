// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_DECORATION_LINE_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_DECORATION_LINE_PAINTER_H_

#include "cc/paint/paint_flags.h"
#include "third_party/blink/renderer/core/paint/text_decoration_info.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

struct AutoDarkMode;
class GraphicsContext;
class StyledStrokeData;

// Helper class for painting a text decorations. Each instance paints a single
// decoration.
class DecorationLinePainter final {
  STACK_ALLOCATED();

 public:
  DecorationLinePainter(GraphicsContext& context,
                        const TextDecorationInfo& decoration_info)
      : context_(context), decoration_info_(decoration_info) {}

  void Paint(const Color& color, const cc::PaintFlags* flags = nullptr);

  static void DrawLineForText(GraphicsContext&,
                              const gfx::PointF& pt,
                              float width,
                              const StyledStrokeData& styled_stroke,
                              const AutoDarkMode& auto_dark_mode,
                              const cc::PaintFlags* paint_flags = nullptr);
  static Path GetPathForTextLine(const gfx::PointF& pt,
                                 float width,
                                 float stroke_thickness,
                                 StrokeStyle stroke_style);

 private:
  void PaintWavyTextDecoration(const AutoDarkMode&);

  GraphicsContext& context_;
  const TextDecorationInfo& decoration_info_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_DECORATION_LINE_PAINTER_H_
