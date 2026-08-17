/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_TEXT_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_TEXT_METRICS_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_baselines.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_canvas_text_align.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_canvas_text_baseline.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_text_cluster_options.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/canvas/text_cluster.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/fonts/font.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/text/text_run.h"

namespace blink {

class DOMRectReadOnly;
class ExceptionState;
class PlainTextPainter;
class TextClusterOptions;

class CORE_EXPORT TextMetrics final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  TextMetrics();
  // `text_painter` must be non-null if `CanvasTextNg` flag is enabled.
  TextMetrics(const Font* font,
              const TextDirection& direction,
              V8CanvasTextBaseline::Enum baseline,
              V8CanvasTextAlign::Enum align,
              const String& text,
              PlainTextPainter* text_painter);

  double width() const { return width_; }
  double actualBoundingBoxLeft() const { return actual_bounding_box_left_; }
  double actualBoundingBoxRight() const { return actual_bounding_box_right_; }
  double fontBoundingBoxAscent() const { return font_bounding_box_ascent_; }
  double fontBoundingBoxDescent() const { return font_bounding_box_descent_; }
  double actualBoundingBoxAscent() const { return actual_bounding_box_ascent_; }
  double alphabeticBaseline() const { return baselines_->alphabetic(); }
  double hangingBaseline() const { return baselines_->hanging(); }
  double ideographicBaseline() const { return baselines_->ideographic(); }
  double actualBoundingBoxDescent() const {
    return actual_bounding_box_descent_;
  }
  double emHeightAscent() const { return em_height_ascent_; }
  double emHeightDescent() const { return em_height_descent_; }

  static float GetFontBaseline(const V8CanvasTextBaseline::Enum,
                               const SimpleFontData&);

  unsigned getIndexFromOffset(double x);

  const HeapVector<Member<DOMRectReadOnly>> getSelectionRects(
      uint32_t start,
      uint32_t end,
      ExceptionState& exception_state);
  DOMRectReadOnly* getActualBoundingBox(uint32_t start,
                                        uint32_t end,
                                        ExceptionState& exception_state);
  HeapVector<Member<TextCluster>> getTextClusters(
      uint32_t start,
      uint32_t end,
      const TextClusterOptions* options,
      ExceptionState& exception_state);
  HeapVector<Member<TextCluster>> getTextClusters(
      const TextClusterOptions* options);

  const Font* GetFont() const { return font_; }

  void Trace(Visitor*) const override;

  struct RunWithOffset {
    DISALLOW_NEW();

    void Trace(Visitor* visitor) const { visitor->Trace(shape_result_); }

    Member<const ShapeResult> shape_result_{
        nullptr, Member<const ShapeResult>::AtomicInitializerTag{}};
    String text_;
    TextDirection direction_;
    unsigned character_offset_;
    unsigned num_characters_;
    float x_position_;
  };

 private:
  void Update(const Font*,
              const TextDirection& direction,
              V8CanvasTextBaseline::Enum baseline,
              V8CanvasTextAlign::Enum align,
              const String&,
              PlainTextPainter* text_painter);
  // A helper for Update().  This function updates `runs_with_offset_`, and
  // returns a pair of the total width and the glyph bounding rectangle.
  std::pair<float, gfx::RectF> MeasureRuns(PlainTextPainter* text_painter);

  void ShapeTextIfNeeded();
  unsigned CorrectForMixedBidi(HeapVector<RunWithOffset>::reverse_iterator&,
                               unsigned);

  HeapVector<Member<TextCluster>> getTextClustersImpl(
      uint32_t start,
      uint32_t end,
      const TextClusterOptions* options,
      ExceptionState* exception_state);

  // x-direction
  double width_ = 0.0;
  double actual_bounding_box_left_ = 0.0;
  double actual_bounding_box_right_ = 0.0;
  // Delta needed for handling textAlign correctly.
  float text_align_dx_ = 0.0;
  // y-direction
  double font_bounding_box_ascent_ = 0.0;
  double font_bounding_box_descent_ = 0.0;
  double actual_bounding_box_ascent_ = 0.0;
  double actual_bounding_box_descent_ = 0.0;
  double em_height_ascent_ = 0.0;
  double em_height_descent_ = 0.0;
  float baseline_y = 0.0;
  Member<Baselines> baselines_;

  // Needed for selection rects, bounding boxes and caret position.
  Member<const Font> font_;
  TextDirection direction_;
  String text_;

  // Values from the canvas context at the moment the text was measured.
  V8CanvasTextAlign::Enum ctx_text_align_ = V8CanvasTextAlign::Enum::kStart;
  V8CanvasTextBaseline::Enum ctx_text_baseline_ =
      V8CanvasTextBaseline::Enum::kAlphabetic;

  // Cache of ShapeResults that is lazily created the first time it's needed.
  HeapVector<RunWithOffset> runs_with_offset_;
  bool shaping_needed_ = false;
  // This flag should be removed on removal of "CanvasTextNg" origin trial.
  bool split_by_word_ = false;
};

}  // namespace blink

namespace WTF {

template <>
struct VectorTraits<blink::TextMetrics::RunWithOffset>
    : VectorTraitsBase<blink::TextMetrics::RunWithOffset> {
  static constexpr bool kCanClearUnusedSlotsWithMemset = true;
  static constexpr bool kCanTraceConcurrently = true;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_TEXT_METRICS_H_
