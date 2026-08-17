/*
 * Copyright (C) Research In Motion Limited 2010-2011. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_METRICS_H_

#include <optional>

#include "base/types/strong_alias.h"
#include "third_party/blink/renderer/platform/fonts/font_baseline.h"
#include "third_party/blink/renderer/platform/fonts/font_height.h"
#include "third_party/blink/renderer/platform/fonts/font_metrics_override.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"

class SkFont;

namespace blink {

class FontPlatformData;

class PLATFORM_EXPORT FontMetrics {
  DISALLOW_NEW();

 public:
  using ApplyBaselineTable =
      base::StrongAlias<class ApplyBaselineTableTag, bool>;

  float FloatAscent(FontBaseline baseline_type = kAlphabeticBaseline,
                    ApplyBaselineTable apply_baseline_table =
                        ApplyBaselineTable(false)) const {
    if (baseline_type == kAlphabeticBaseline)
      return float_ascent_;
    return FloatAscentInternal(baseline_type, apply_baseline_table);
  }

  void SetAscent(float ascent) {
    float_ascent_ = ascent;
    int_ascent_ = static_cast<int>(lroundf(ascent));
  }

  float FloatDescent(FontBaseline baseline_type = kAlphabeticBaseline,
                     ApplyBaselineTable apply_baseline_table =
                         ApplyBaselineTable(false)) const {
    if (baseline_type == kAlphabeticBaseline)
      return float_descent_;
    return FloatHeight() -
           FloatAscentInternal(baseline_type, apply_baseline_table);
  }

  void SetDescent(float descent) {
    float_descent_ = descent;
    int_descent_ = static_cast<int>(lroundf(descent));
  }

  float FloatHeight() const { return float_ascent_ + float_descent_; }

  float ConvertBaseline(float value,
                        FontBaseline to,
                        FontBaseline from = kAlphabeticBaseline) const {
    return from == to ? value : FloatAscent(to) - FloatAscent(from) + value;
  }
  float Alphabetic(FontBaseline baseline_type) const {
    return ConvertBaseline(0, baseline_type);
  }

  float CapHeight() const { return cap_height_; }
  float CapHeight(FontBaseline baseline_type) const {
    return ConvertBaseline(CapHeight(), baseline_type);
  }
  void SetCapHeight(float cap_height) { cap_height_ = cap_height; }

  int LineGap() const { return static_cast<int>(lroundf(line_gap_)); }
  void SetLineGap(float line_gap) { line_gap_ = line_gap; }

  int LineSpacing() const { return static_cast<int>(lroundf(line_spacing_)); }
  void SetLineSpacing(float line_spacing) { line_spacing_ = line_spacing; }

  float XHeight() const { return x_height_; }
  float XHeight(FontBaseline baseline_type) const {
    return ConvertBaseline(XHeight(), baseline_type);
  }
  void SetXHeight(float x_height) {
    x_height_ = x_height;
    has_x_height_ = true;
  }

  bool HasXHeight() const { return has_x_height_ && x_height_ > 0; }
  void SetHasXHeight(bool has_x_height) { has_x_height_ = has_x_height; }

  // Integer variants of certain metrics, used for HTML rendering.
  int Ascent(FontBaseline baseline_type = kAlphabeticBaseline,
             ApplyBaselineTable apply_baseline_table =
                 ApplyBaselineTable(false)) const {
    if (baseline_type == kAlphabeticBaseline)
      return int_ascent_;
    return IntAscentInternal(baseline_type, apply_baseline_table);
  }

  int Descent(FontBaseline baseline_type = kAlphabeticBaseline,
              ApplyBaselineTable apply_baseline_table =
                  ApplyBaselineTable(false)) const {
    if (baseline_type == kAlphabeticBaseline)
      return int_descent_;
    return Height() - IntAscentInternal(baseline_type, apply_baseline_table);
  }

  int Height() const { return int_ascent_ + int_descent_; }

  // LayoutUnit variants of certain metrics.
  // LayoutNG should use LayoutUnit for the block progression metrics.
  // TODO(kojii): Consider keeping converted values.
  LayoutUnit FixedAscent(
      FontBaseline baseline_type = kAlphabeticBaseline) const {
    return LayoutUnit::FromFloatRound(FloatAscent(baseline_type));
  }

  LayoutUnit FixedDescent(
      FontBaseline baseline_type = kAlphabeticBaseline) const {
    return LayoutUnit::FromFloatRound(FloatDescent(baseline_type));
  }

  LayoutUnit FixedAlphabetic(FontBaseline baseline_type) const {
    return LayoutUnit::FromFloatRound(Alphabetic(baseline_type));
  }

  LayoutUnit FixedCapHeight(FontBaseline baseline_type) const {
    return LayoutUnit::FromFloatRound(CapHeight(baseline_type));
  }

  LayoutUnit FixedXHeight(FontBaseline baseline_type) const {
    return LayoutUnit::FromFloatRound(XHeight(baseline_type));
  }

  LayoutUnit FixedLineSpacing() const {
    return LayoutUnit::FromFloatRound(line_spacing_);
  }

  FontHeight GetFloatFontHeight(FontBaseline baseline_type) const {
    return FontHeight(FixedAscent(baseline_type), FixedDescent(baseline_type));
  }

  FontHeight GetFontHeight(
      FontBaseline baseline_type = kAlphabeticBaseline) const {
    // TODO(kojii): In future, we'd like to use LayoutUnit metrics to support
    // sub-CSS-pixel layout.
    return FontHeight(LayoutUnit(Ascent(baseline_type)),
                      LayoutUnit(Descent(baseline_type)));
  }

  float ZeroWidth() const { return zero_width_; }
  void SetZeroWidth(float zero_width) {
    zero_width_ = zero_width;
    has_zero_width_ = true;
  }

  bool HasZeroWidth() const { return has_zero_width_; }
  void SetHasZeroWidth(bool has_zero_width) {
    has_zero_width_ = has_zero_width;
  }

  std::optional<float> UnderlineThickness() const {
    return underline_thickness_;
  }
  void SetUnderlineThickness(float underline_thickness) {
    underline_thickness_ = underline_thickness;
  }

  std::optional<float> UnderlinePosition() const { return underline_position_; }
  void SetUnderlinePosition(float underline_position) {
    underline_position_ = underline_position;
  }

  void SetIdeographicBaseline(std::optional<float> value) {
    ideographic_baseline_position_ = value;
  }

  std::optional<float> IdeographicBaseline() const {
    return ideographic_baseline_position_;
  }

  void SetAlphabeticBaseline(std::optional<float> value) {
    alphabetic_baseline_position_ = value;
  }

  std::optional<float> AlphabeticBaseline() const {
    return alphabetic_baseline_position_;
  }

  void SetHangingBaseline(std::optional<float> value) {
    hanging_baseline_position_ = value;
  }

  std::optional<float> HangingBaseline() const {
    return hanging_baseline_position_;
  }

  // Unfortunately we still need to keep metrics adjustments for certain Mac
  // fonts, see crbug.com/445830. Also, a potentially better solution for the
  // subpixel_ascent_descent flag would be to move line layout to LayoutUnit
  // instead of int boundaries, see crbug.com/707807 and crbug.com/706298.
  static void AscentDescentWithHacks(
      float& ascent,
      float& descent,
      const FontPlatformData&,
      const SkFont&,
      bool subpixel_ascent_descent = false,
      std::optional<float> ascent_override = std::nullopt,
      std::optional<float> descent_override = std::nullopt);

 private:
  friend class SimpleFontData;

  void Reset() {
    cap_height_ = 0;
    float_ascent_ = 0;
    float_descent_ = 0;
    int_ascent_ = 0;
    int_descent_ = 0;
    line_gap_ = 0;
    line_spacing_ = 0;
    x_height_ = 0;
    has_x_height_ = false;
    underline_thickness_.reset();
    underline_position_.reset();
    ideographic_baseline_position_.reset();
    alphabetic_baseline_position_.reset();
    hanging_baseline_position_.reset();
  }

  float FloatAscentInternal(FontBaseline baseline_type,
                            ApplyBaselineTable apply_baseline_table) const;
  int IntAscentInternal(FontBaseline baseline_type,
                        ApplyBaselineTable apply_baseline_table) const;

  float cap_height_ = 0;
  float float_ascent_ = 0;
  float float_descent_ = 0;
  float line_gap_ = 0;
  float line_spacing_ = 0;
  float x_height_ = 0;
  float zero_width_ = 0;
  std::optional<float> underline_thickness_;
  std::optional<float> underline_position_;
  std::optional<float> ideographic_baseline_position_;
  std::optional<float> alphabetic_baseline_position_;
  std::optional<float> hanging_baseline_position_;
  int int_ascent_ = 0;
  int int_descent_ = 0;
  bool has_x_height_ = false;
  bool has_zero_width_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_METRICS_H_
