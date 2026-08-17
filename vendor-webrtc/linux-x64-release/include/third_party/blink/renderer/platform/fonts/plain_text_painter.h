// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_PLAIN_TEXT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_PLAIN_TEXT_PAINTER_H_

#include "third_party/blink/renderer/platform/fonts/font.h"
#include "third_party/blink/renderer/platform/instrumentation/memory_pressure_listener.h"

namespace gfx {
class PointF;
class RectF;
}  // namespace gfx

namespace cc {
class PaintCanvas;
class PaintFlags;
}  // namespace cc

namespace blink {

class FrameShapeCache;
class PlainTextNode;
class TextRun;

// PlainTextPainter provides functionality to render plain text, excluding
// CSS and SVG.
//
// This class supports two modes: kCanvas and kShared.
// - kCanvas: Designed for use with HTML <canvas> and OffscreenCanvas.
// - kShared: Intended for purposes other than canvas rendering.
//
// The modes differ in how whitespace within the text is handled and how
// caching behaves.
//
// Instances in kCanvas mode are created one per canvas instance.
// Instances in kShared mode are created only once and accessed via
// PlainTextPainter::Shared().
class PLATFORM_EXPORT PlainTextPainter
    : public GarbageCollected<PlainTextPainter>,
      public MemoryPressureListener {
 public:
  enum Mode { kCanvas, kShared };
  explicit PlainTextPainter(Mode mode);
  void Trace(Visitor* visitor) const override;

  PlainTextPainter(const PlainTextPainter&) = delete;
  PlainTextPainter& operator=(const PlainTextPainter&) = delete;

  // Return the shared instance for non-<canvas>.
  static PlainTextPainter& Shared();

  // Returns a PlainTextNode instance for `run` and `font`.
  // This function applied Bidi reorder and word segmentation.
  const PlainTextNode& SegmentAndShape(const TextRun& run, const Font& font);

  // Draw the specified text. This doesn't apply BiDi reorder.
  void DrawWithoutBidi(const TextRun& run,
                       const Font& font,
                       cc::PaintCanvas& canvas,
                       const gfx::PointF& location,
                       const cc::PaintFlags& flags,
                       Font::DrawType = Font::DrawType::kGlyphsOnly);

  // Draw the specified text, from `from_index` to `to_index` (exclusive). This
  // applies BiDi reorder.
  // This function returns `false` if a web font `font` is not ready and
  // `action` is `kDoNotPaintIfFontNotReady`.
  bool DrawWithBidiReorder(const TextRun& run,
                           unsigned from_index,
                           unsigned to_index,
                           const Font& font,
                           Font::CustomFontNotReadyAction action,
                           cc::PaintCanvas& canvas,
                           const gfx::PointF& location,
                           const cc::PaintFlags& flags,
                           Font::DrawType = Font::DrawType::kGlyphsOnly);

  // Glyph bounds will be the minimum rect containing all glyph strokes, in
  // coordinates using (<text run x position>, <baseline position>) as the
  // origin. If the pointer is not null, glyph_bounds is expected to be
  // default-initialized.
  float ComputeInlineSize(const TextRun& run,
                          const Font& font,
                          gfx::RectF* glyph_bounds = nullptr);
  float ComputeSubInlineSize(const TextRun&,
                             unsigned from_index,
                             unsigned to_index,
                             const Font& font,
                             gfx::RectF* glyph_bounds = nullptr);
  // This doesn't apply BiDi reorder for compatibility.
  float ComputeInlineSizeWithoutBidi(const TextRun& run, const Font& font);

  // Returns text offset in `run` for the specified pixel position.
  // This doesn't apply BiDi reorder for compatibility.
  int OffsetForPositionWithoutBidi(const TextRun& run,
                                   const Font& font,
                                   float position,
                                   IncludePartialGlyphsOption partial_option,
                                   BreakGlyphsOption break_option);

  // Returns a bounding rectangle for a sub-range from `from_index` to
  // `to_index` (exclusive) of `run`.
  // This doesn't apply BiDi reorder for compatibility.
  gfx::RectF SelectionRectForTextWithoutBidi(const TextRun& run,
                                             unsigned from_index,
                                             unsigned to_index,
                                             const Font& font,
                                             const gfx::PointF& left_baseline,
                                             float height);

  // This function should be called between the end of an animation frame and
  // the beginning of the next animation frame. This is for <canvas>, and we
  // don't need to call this for the shared instance.
  void DidSwitchFrame();

 private:
  const PlainTextNode& CreateNode(const TextRun& text_run,
                                  const Font& font,
                                  bool supports_bidi = true);
  FrameShapeCache* GetCacheFor(const Font& font);

  // MemoryPressureListener override:
  void OnPurgeMemory() override;

  // A map from a FontFallbackList to a FrameShapeCache.
  // We don't need to worry about Web Fonts. When a Web Font loading state is
  // changed, affected FontFallbackLists are invalidated, and are disconnected
  // from owner Fonts. They will be removed from `cache_map_` by GC.
  HeapHashMap<WeakMember<FontFallbackList>, Member<FrameShapeCache>> cache_map_;
  const Mode mode_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_PLAIN_TEXT_PAINTER_H_
