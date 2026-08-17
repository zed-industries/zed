/*
 * Copyright (c) 2012 Google Inc. All rights reserved.
 * Copyright (C) 2013 BlackBerry Limited. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_BOUNDS_ACCUMULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_BOUNDS_ACCUMULATOR_H_

#include "build/build_config.h"
#include "third_party/blink/renderer/platform/fonts/shaping/shape_result_run.h"
#include "ui/gfx/geometry/rect_f.h"

#if defined(USE_SIMD_FOR_COMPUTING_GLYPH_BOUNDS)
#include "third_party/highway/src/hwy/highway.h"
#endif  // defined(USE_SIMD_FOR_COMPUTING_GLYPH_BOUNDS)

namespace blink {

// Helper class to accumulate glyph bounding box.
//
// Glyph positions and bounding boxes from HarfBuzz and fonts are in physical
// coordinate, while ShapeResult::glyph_bounding_box_ is in logical coordinate.
// To minimize the number of conversions, this class accumulates the bounding
// boxes in physical coordinate, and convert the accumulated box to logical.
template <bool is_horizontal_run>
struct GlyphBoundsAccumulator {
  // The accumulated glyph bounding box in physical coordinate, until
  // ConvertVerticalRunToLogicalIfNeeded().
  //
  // We store this as a set of positions rather than a gfx::RectF,
  // because it is cheaper to do lots of Union operations when stored
  // that way, rather than as the (point, size) storage that RectF uses.
  float min_x = 0;
  float max_x = 0;
  float min_y = 0;
  float max_y = 0;

  // Unite a glyph bounding box to |bounds|.
  void Unite(gfx::RectF bounds_for_glyph,
             float origin,
             GlyphOffset glyph_offset) {
    if (bounds_for_glyph.IsEmpty()) [[unlikely]] {
      return;
    }

    // Glyphs are drawn at |origin + offset|. Move glyph_bounds to that point.
    // All positions in hb_glyph_position_t are relative to the current point.
    // https://behdad.github.io/harfbuzz/harfbuzz-Buffers.html#hb-glyph-position-t-struct
    if constexpr (is_horizontal_run) {
      bounds_for_glyph.set_x(bounds_for_glyph.x() + origin);
    } else {
      bounds_for_glyph.set_y(bounds_for_glyph.y() + origin);
    }
    bounds_for_glyph.Offset(glyph_offset);

    if (min_x == 0 && max_x == 0) [[unlikely]] {
      min_x = bounds_for_glyph.x();
      max_x = bounds_for_glyph.right();
      min_y = bounds_for_glyph.y();
      max_y = bounds_for_glyph.bottom();
    } else {
      min_x = std::min(min_x, bounds_for_glyph.x());
      max_x = std::max(max_x, bounds_for_glyph.right());
      min_y = std::min(min_y, bounds_for_glyph.y());
      max_y = std::max(max_y, bounds_for_glyph.bottom());
    }
  }

  gfx::RectF BuildBounds(const FontMetrics& font_metrics) && {
    ConvertVerticalRunToLogicalIfNeeded(font_metrics);
    return Bounds();
  }

 private:
  // Convert vertical run glyph bounding box to logical. Horizontal runs do not
  // need conversions because physical and logical are the same.
  void ConvertVerticalRunToLogicalIfNeeded(const FontMetrics& font_metrics) {
    if constexpr (is_horizontal_run) {
      return;
    }
    // Convert physical glyph_bounding_box to logical.
    std::swap(min_x, min_y);
    std::swap(max_x, max_y);

    // The glyph bounding box of a vertical run uses ideographic central
    // baseline. Adjust the box Y position because the bounding box of a
    // ShapeResult uses alphabetic baseline.
    // See diagrams of base lines at
    // https://drafts.csswg.org/css-writing-modes-3/#intro-baselines
    int baseline_adjust = font_metrics.Ascent(kCentralBaseline) -
                          font_metrics.Ascent(kAlphabeticBaseline);
    min_y += baseline_adjust;
    max_y += baseline_adjust;
  }

  gfx::RectF Bounds() const {
    return gfx::RectF(gfx::PointF(min_x, min_y),
                      gfx::SizeF(max_x - min_x, max_y - min_y));
  }
};

#if defined(USE_SIMD_FOR_COMPUTING_GLYPH_BOUNDS)

template <bool is_horizontal_run>
class VectorizedGlyphBoundsAccumulator final {
 public:
  static constexpr size_t kStride = 4;

  // Unite a glyph bounding box to |bounds|.
  ALWAYS_INLINE void Unite4(gfx::RectF bounds_for_glyph1,
                            gfx::RectF bounds_for_glyph2,
                            gfx::RectF bounds_for_glyph3,
                            gfx::RectF bounds_for_glyph4,
                            float origin1,
                            float origin2,
                            float origin3,
                            float origin4,
                            GlyphOffset glyph_offset1,
                            GlyphOffset glyph_offset2,
                            GlyphOffset glyph_offset3,
                            GlyphOffset glyph_offset4) {
    namespace hw = hwy::HWY_NAMESPACE;
    hwy::HWY_NAMESPACE::FixedTag<float, 4> tag;

    const auto limit_maxs = hw::Set(tag, std::numeric_limits<float>::max());
    const auto limit_mins = hw::Set(tag, std::numeric_limits<float>::lowest());

    // clang-format off
    alignas(16) const std::array<float, 4> x_mins{
        bounds_for_glyph1.x(), bounds_for_glyph2.x(),
        bounds_for_glyph3.x(), bounds_for_glyph4.x()
    };
    alignas(16) const std::array<float, 4> y_mins{
        bounds_for_glyph1.y(), bounds_for_glyph2.y(),
        bounds_for_glyph3.y(), bounds_for_glyph4.y()
    };

    alignas(16) const std::array<float, 4> widths{
        bounds_for_glyph1.width(), bounds_for_glyph2.width(),
        bounds_for_glyph3.width(), bounds_for_glyph4.width()};
    alignas(16) const std::array<float, 4> heights{
        bounds_for_glyph1.height(), bounds_for_glyph2.height(),
        bounds_for_glyph3.height(), bounds_for_glyph4.height()};

    alignas(16) const std::array<float, 4> origins{
      origin1, origin2, origin3, origin4
    };

    alignas(16) const std::array<float, 4> x_offsets{
        glyph_offset1.x(), glyph_offset2.x(),
        glyph_offset3.x(), glyph_offset4.x()
    };
    alignas(16) const std::array<float, 4> y_offsets{
        glyph_offset1.y(), glyph_offset2.y(),
        glyph_offset3.y(), glyph_offset4.y()
    };
    // clang-format on

    auto x_mins_v = hw::Load(tag, x_mins.data());
    auto y_mins_v = hw::Load(tag, y_mins.data());
    auto origins_v = hw::Load(tag, origins.data());
    auto x_offsets_v = hw::Load(tag, x_offsets.data());
    auto y_offsets_v = hw::Load(tag, y_offsets.data());

    if constexpr (is_horizontal_run) {
      x_mins_v = x_mins_v + origins_v + x_offsets_v;
      y_mins_v = y_mins_v + y_offsets_v;
    } else {
      x_mins_v = x_mins_v + x_offsets_v;
      y_mins_v = y_mins_v + origins_v + y_offsets_v;
    }

    auto widths_v = hw::Load(tag, widths.data());
    auto heights_v = hw::Load(tag, heights.data());

    // Doing min and cmp is faster than doing two cmps and or.
    auto is_empty_v = hw::Min(widths_v, heights_v) == hw::Zero(tag);

    auto x_maxs_v = x_mins_v + widths_v;
    auto y_maxs_v = y_mins_v + heights_v;

    // If a glyph is empty, use min/max.
    x_mins_v = hw::IfThenElse(is_empty_v, limit_maxs, x_mins_v);
    y_mins_v = hw::IfThenElse(is_empty_v, limit_maxs, y_mins_v);
    x_maxs_v = hw::IfThenElse(is_empty_v, limit_mins, x_maxs_v);
    y_maxs_v = hw::IfThenElse(is_empty_v, limit_mins, y_maxs_v);

    // Perform vertical min/max.
    auto vmins_x = hw::Min(hw::Load(tag, &current_.min_x1), x_mins_v);
    auto vmins_y = hw::Min(hw::Load(tag, &current_.min_y1), y_mins_v);
    auto vmaxs_x = hw::Max(hw::Load(tag, &current_.max_x1), x_maxs_v);
    auto vmaxs_y = hw::Max(hw::Load(tag, &current_.max_y1), y_maxs_v);

    // Store results back.
    hw::Store(vmins_x, tag, &current_.min_x1);
    hw::Store(vmins_y, tag, &current_.min_y1);
    hw::Store(vmaxs_x, tag, &current_.max_x1);
    hw::Store(vmaxs_y, tag, &current_.max_y1);
  }

  // Unite a glyph bounding box to |bounds|.
  ALWAYS_INLINE void Unite1(gfx::RectF bounds_for_glyph,
                            float origin,
                            GlyphOffset glyph_offset) {
    if (bounds_for_glyph.IsEmpty()) [[unlikely]] {
      return;
    }

    // Glyphs are drawn at |origin + offset|. Move glyph_bounds to that point.
    // All positions in hb_glyph_position_t are relative to the current point.
    // https://behdad.github.io/harfbuzz/harfbuzz-Buffers.html#hb-glyph-position-t-struct
    if constexpr (is_horizontal_run) {
      bounds_for_glyph.set_x(bounds_for_glyph.x() + origin);
    } else {
      bounds_for_glyph.set_y(bounds_for_glyph.y() + origin);
    }
    bounds_for_glyph.Offset(glyph_offset);

    current_.min_x1 = std::min(current_.min_x1, bounds_for_glyph.x());
    current_.max_x1 = std::max(current_.max_x1, bounds_for_glyph.right());
    current_.min_y1 = std::min(current_.min_y1, bounds_for_glyph.y());
    current_.max_y1 = std::max(current_.max_y1, bounds_for_glyph.bottom());
  }

  gfx::RectF BuildBounds(const FontMetrics& font_metrics) && {
    namespace hw = hwy::HWY_NAMESPACE;
    hw::FixedTag<float, 4> tag;

    // Perform horizontal min/max on vectors and place the results in the array.
    alignas(16) std::array<float, 4> result{
        hw::ReduceMin(tag, hw::Load(tag, &current_.min_x1)),
        hw::ReduceMin(tag, hw::Load(tag, &current_.min_y1)),
        hw::ReduceMax(tag, hw::Load(tag, &current_.max_x1)),
        hw::ReduceMax(tag, hw::Load(tag, &current_.max_y1))};

    auto& [min_x, min_y, max_x, max_y] = result;

    const auto mask = hw::Dup128VecFromValues(
        tag, std::numeric_limits<float>::max(),
        std::numeric_limits<float>::max(), std::numeric_limits<float>::lowest(),
        std::numeric_limits<float>::lowest());

    // If any of the points is max/min, then we have only encountered empty
    // glyphs.
    if (!hw::AllFalse(tag, hw::Load(tag, result.data()) == mask)) {
      std::fill(result.begin(), result.end(), 0.f);
    }

    ConvertVerticalRunToLogicalIfNeeded(min_x, min_y, max_x, max_y,
                                        font_metrics);

    return gfx::RectF{gfx::PointF(min_x, min_y),
                      gfx::SizeF(max_x - min_x, max_y - min_y)};
  }

 private:
  struct alignas(16) {
    // First vector.
    float min_x1 = std::numeric_limits<float>::max();
    float min_x2 = std::numeric_limits<float>::max();
    float min_x3 = std::numeric_limits<float>::max();
    float min_x4 = std::numeric_limits<float>::max();
    // Second vector.
    float min_y1 = std::numeric_limits<float>::max();
    float min_y2 = std::numeric_limits<float>::max();
    float min_y3 = std::numeric_limits<float>::max();
    float min_y4 = std::numeric_limits<float>::max();
    // Third vector.
    float max_x1 = std::numeric_limits<float>::lowest();
    float max_x2 = std::numeric_limits<float>::lowest();
    float max_x3 = std::numeric_limits<float>::lowest();
    float max_x4 = std::numeric_limits<float>::lowest();
    // Fourth vector.
    float max_y1 = std::numeric_limits<float>::lowest();
    float max_y2 = std::numeric_limits<float>::lowest();
    float max_y3 = std::numeric_limits<float>::lowest();
    float max_y4 = std::numeric_limits<float>::lowest();
  } current_;

  ALWAYS_INLINE static void ConvertVerticalRunToLogicalIfNeeded(
      float& min_x,
      float& min_y,
      float& max_x,
      float& max_y,
      const FontMetrics& font_metrics) {
    if constexpr (is_horizontal_run) {
      return;
    }
    // Convert physical glyph_bounding_box to logical.
    std::swap(min_x, min_y);
    std::swap(max_x, max_y);

    // The glyph bounding box of a vertical run uses ideographic central
    // baseline. Adjust the box Y position because the bounding box of a
    // ShapeResult uses alphabetic baseline.
    // See diagrams of base lines at
    // https://drafts.csswg.org/css-writing-modes-3/#intro-baselines
    const int baseline_adjust = font_metrics.Ascent(kCentralBaseline) -
                                font_metrics.Ascent(kAlphabeticBaseline);
    min_y += baseline_adjust;
    max_y += baseline_adjust;
  }
};

#endif  // defined(USE_SIMD_FOR_COMPUTING_GLYPH_BOUNDS)

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_BOUNDS_ACCUMULATOR_H_
