// Copyright (C) 2013 Google Inc. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_CONTEXT_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_CONTEXT_STATE_H_

#include <memory>

#include "base/check_op.h"
#include "base/memory/ptr_util.h"
#include "cc/paint/draw_looper.h"
#include "cc/paint/paint_flags.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context_types.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/skia/include/core/SkRefCnt.h"

namespace blink {

class StrokeData;

// Encapsulates the state information we store for each pushed graphics state.
// Only GraphicsContext can use this class.
class PLATFORM_EXPORT GraphicsContextState final {
  USING_FAST_MALLOC(GraphicsContextState);

 public:
  static std::unique_ptr<GraphicsContextState> CreateAndCopy(
      const GraphicsContextState& other) {
    return base::WrapUnique(new GraphicsContextState(other));
  }

  GraphicsContextState();

  void Copy(const GraphicsContextState&);

  // cc::PaintFlags objects that reflect the current state.
  const cc::PaintFlags& StrokeFlags() const { return stroke_flags_; }
  const cc::PaintFlags& FillFlags() const { return fill_flags_; }

  uint16_t SaveCount() const { return save_count_; }
  void IncrementSaveCount() { ++save_count_; }
  void DecrementSaveCount() { --save_count_; }

  // Stroke data
  Color StrokeColor() const {
    // This conversion from SkColor4f to Color will lose information about the
    // original Color.
    return Color::FromSkColor4f(stroke_flags_.getColor4f());
  }
  void SetStrokeColor(const Color&);

  float GetStrokeThickness() const { return stroke_flags_.getStrokeWidth(); }
  void SetStrokeThickness(float);

  void SetStroke(const StrokeData& stroke_data);

  // Fill data
  Color FillColor() const {
    // This conversion from SkColor4f to Color will lose information about the
    // original Color.
    return Color::FromSkColor4f(fill_flags_.getColor4f());
  }
  void SetFillColor(const Color&);

  void SetTextPaintOrder(TextPaintOrder order) { text_paint_order_ = order; }
  TextPaintOrder GetTextPaintOrder() const { return text_paint_order_; }

  // Shadow. (This will need tweaking if we use draw loopers for other things.)
  cc::DrawLooper* DrawLooper() const {
    DCHECK_EQ(fill_flags_.getLooper(), stroke_flags_.getLooper());
    return fill_flags_.getLooper().get();
  }
  void SetDrawLooper(sk_sp<cc::DrawLooper>);

  // Text. (See TextModeFill & friends.)
  TextDrawingModeFlags TextDrawingMode() const { return text_drawing_mode_; }
  void SetTextDrawingMode(TextDrawingModeFlags mode) {
    text_drawing_mode_ = mode;
  }

  // Image interpolation control.
  InterpolationQuality GetInterpolationQuality() const {
    return interpolation_quality_;
  }
  void SetInterpolationQuality(InterpolationQuality);

  DynamicRangeLimit GetDynamicRangeLimit() const {
    return dynamic_range_limit_;
  }
  void SetDynamicRangeLimit(DynamicRangeLimit limit);

  bool ShouldAntialias() const { return should_antialias_; }
  void SetShouldAntialias(bool);

 private:
  explicit GraphicsContextState(const GraphicsContextState&);
  GraphicsContextState& operator=(const GraphicsContextState&) = delete;

  cc::PaintFlags stroke_flags_;
  cc::PaintFlags fill_flags_;
  TextPaintOrder text_paint_order_ = kFillStroke;

  TextDrawingModeFlags text_drawing_mode_ = kTextModeFill;

  InterpolationQuality interpolation_quality_ =
      GetDefaultInterpolationQuality();
  DynamicRangeLimit dynamic_range_limit_{
      cc::PaintFlags::DynamicRangeLimit::kHigh};

  uint16_t save_count_ = 0;

  bool should_antialias_ : 1 = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_CONTEXT_STATE_H_
