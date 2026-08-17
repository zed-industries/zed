// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_COMBINE_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_COMBINE_PAINTER_H_

#include "third_party/blink/renderer/core/paint/text_painter.h"

namespace blink {

class ComputedStyle;
class LayoutTextCombine;
struct LineRelativeOffset;

// The painter for painting text decorations and emphasis marks for
// LayoutTextCombine.
class TextCombinePainter final : public TextPainter {
 public:
  TextCombinePainter(GraphicsContext& context,
                     const SvgContextPaints* svg_context_paints,
                     const gfx::Rect& visual_rect,
                     const ComputedStyle& style,
                     const LineRelativeOffset& text_origin);
  ~TextCombinePainter();

  static void Paint(const PaintInfo& paint_info,
                    const PhysicalOffset& paint_offset,
                    const LayoutTextCombine& text_combine);

  static bool ShouldPaint(const LayoutTextCombine& text_combine);

 protected:
  void ClipDecorationsStripe(const TextFragmentPaintInfo&,
                             float upper,
                             float stripe_width,
                             float dilation) override;

 private:
  // Paints emphasis mark as for ideographic full stop character. Callers of
  // this function should rotate canvas to paint emphasis mark at left/right
  // side instead of top/bottom side.
  // `emphasis_mark_font` is used for painting emphasis mark because `font_`
  // may be compressed font (width variants).
  void PaintEmphasisMark(const TextPaintStyle& text_style,
                         const Font& emphasis_mark_font);

  const ComputedStyle& style_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_COMBINE_PAINTER_H_
