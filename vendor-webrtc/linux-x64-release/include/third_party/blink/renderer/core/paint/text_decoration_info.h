// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_DECORATION_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_DECORATION_INFO_H_

#include <optional>

#include "base/types/strong_alias.h"
#include "cc/paint/paint_record.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/paint/line_relative_rect.h"
#include "third_party/blink/renderer/core/paint/text_paint_style.h"
#include "third_party/blink/renderer/core/style/applied_text_decoration.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/fonts/font_baseline.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/path.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/graphics/styled_stroke_data.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class ComputedStyle;
class DecoratingBox;
class Font;
class InlinePaintContext;
class SimpleFontData;
class TextDecorationOffset;

enum class ResolvedUnderlinePosition {
  kNearAlphabeticBaselineAuto,
  kNearAlphabeticBaselineFromFont,
  kUnder,
  kOver
};

using MinimumThickness1 = base::StrongAlias<class MinimumThickness1Tag, bool>;

// Container for computing and storing information for text decoration
// invalidation and painting. See also
// https://www.w3.org/TR/css-text-decor-3/#painting-order
class CORE_EXPORT TextDecorationInfo {
  STACK_ALLOCATED();

 public:
  TextDecorationInfo(
      LineRelativeOffset local_origin,
      LayoutUnit width,
      const ComputedStyle& target_style,
      const InlinePaintContext* inline_context,
      const TextDecorationLine selection_decoration_line,
      const Color selection_decoration_color,
      const AppliedTextDecoration* decoration_override = nullptr,
      const Font* font_override = nullptr,
      MinimumThickness1 minimum_thickness1 = MinimumThickness1(true),
      float scaling_factor = 1.0f);

  wtf_size_t AppliedDecorationCount() const;
  const AppliedTextDecoration& AppliedDecoration(wtf_size_t) const;
  bool HasDecorationOverride() const { return !!decoration_override_; }

  // Returns whether any of the decoration indices in AppliedTextDecoration
  // have any of the given lines.
  bool HasAnyLine(TextDecorationLine lines) const {
    return EnumHasFlags(union_all_lines_, lines);
  }

 private:
  // Returns whether the decoration currently selected by |SetDecorationIndex|
  // has any of the given lines.
  bool Has(TextDecorationLine line) const { return EnumHasFlags(lines_, line); }

 public:
  // These methods also apply to the currently selected decoration only.
  bool HasUnderline() const { return has_underline_; }
  bool HasOverline() const { return has_overline_; }
  bool HasLineThrough() const { return Has(TextDecorationLine::kLineThrough); }
  bool HasSpellingError() const {
    return Has(TextDecorationLine::kSpellingError);
  }
  bool HasGrammarError() const {
    return Has(TextDecorationLine::kGrammarError);
  }
  bool HasSpellingOrGrammerError() const {
    return HasSpellingError() || HasGrammarError();
  }

  // Set the decoration to use when painting and returning values.
  //
  // This is set to 0 when constructed, and can be called again at any time.
  // This object will use the most recently given index for any computation that
  // uses data from an AppliedTextDecoration object or a decorating box.
  //
  // The index must be a valid index the AppliedTextDecorations contained within
  // the style passed at construction.
  void SetDecorationIndex(int decoration_index);

  // Set data for one of the text decoration lines: over, under or
  // through. Must be called before trying to paint or compute bounds
  // for a line.
  void SetLineData(TextDecorationLine line, float line_offset);
  void SetUnderlineLineData(const TextDecorationOffset& decoration_offset);
  void SetOverlineLineData(const TextDecorationOffset& decoration_offset);
  void SetLineThroughLineData();
  void SetSpellingOrGrammarErrorLineData(const TextDecorationOffset&);

  // These methods do not depend on |SetDecorationIndex|.
  LayoutUnit Width() const { return width_; }
  const ComputedStyle& TargetStyle() const { return target_style_; }
  float TargetAscent() const { return target_ascent_; }
  // Returns the scaling factor for the decoration.
  // It can be different from FragmentItem::SvgScalingFactor() if the
  // text works as a resource.
  float ScalingFactor() const { return scaling_factor_; }
  float InkSkipClipUpper(float bounds_upper) const {
    return -TargetAscent() + bounds_upper - local_origin_.line_over.ToFloat();
  }

  // |SetDecorationIndex| may change the results of these methods.
  float ComputedFontSize() const { return computed_font_size_; }
  const SimpleFontData* FontData() const { return font_data_; }
  float Ascent() const { return ascent_; }
  ETextDecorationStyle DecorationStyle() const;
  ResolvedUnderlinePosition FlippedUnderlinePosition() const {
    return flipped_underline_position_;
  }
  ResolvedUnderlinePosition OriginalUnderlinePosition() const {
    return original_underline_position_;
  }
  Color LineColor() const;
  float ResolvedThickness() const { return resolved_thickness_; }
  enum StrokeStyle StrokeStyle() const;

  // SetLineData must be called before using the remaining methods.
  gfx::PointF StartPoint() const;
  float DoubleOffset() const;
  bool ShouldAntialias() const;

  // Compute bounds for the given line and the current decoration.
  gfx::RectF Bounds() const;

  // Returns tile record and coordinates for wavy decorations.
  cc::PaintRecord WavyTileRecord() const;
  gfx::RectF WavyPaintRect() const;
  gfx::RectF WavyTileRect() const;

  // Overrides the line color with the given topmost active highlight ‘color’
  // (for originating decorations being painted in highlight overlays), or the
  // highlight ‘text-decoration-color’ resolved with the correct ‘currentColor’
  // (for decorations introduced by highlight pseudos).
  void SetHighlightOverrideColor(const std::optional<Color>&);

 private:
  LayoutUnit OffsetFromDecoratingBox() const;
  float ComputeThickness() const;
  float ComputeUnderlineThickness(
      const TextDecorationThickness& applied_decoration_thickness,
      const ComputedStyle* decorating_box_style) const;
  void ComputeWavyLineData(gfx::RectF& pattern_rect,
                           cc::PaintRecord& tile_record) const;

  gfx::RectF BoundsForDottedOrDashed() const;
  gfx::RectF BoundsForWavy() const;
  Path PrepareDottedOrDashedStrokePath() const;
  bool IsSpellingOrGrammarError() const {
    return line_data_.line == TextDecorationLine::kSpellingError ||
           line_data_.line == TextDecorationLine::kGrammarError;
  }

  void UpdateForDecorationIndex();

  // The |ComputedStyle| of the target text/box to paint decorations for.
  const ComputedStyle& target_style_;
  // The |ComputedStyle| of the [decorating box]. Decorations are computed from
  // this style.
  // [decorating box]: https://drafts.csswg.org/css-text-decor-3/#decorating-box
  const ComputedStyle* decorating_box_style_ = nullptr;

  // Decorating box properties for the current |decoration_index_|.
  const InlinePaintContext* const inline_context_ = nullptr;
  const DecoratingBox* decorating_box_ = nullptr;
  const AppliedTextDecoration* applied_text_decoration_ = nullptr;
  const TextDecorationLine selection_decoration_line_ =
      TextDecorationLine::kNone;
  const Color selection_decoration_color_;
  const Font* font_ = nullptr;
  const SimpleFontData* font_data_ = nullptr;

  // These "overrides" fields force using the specified style or font instead
  // of the one from the decorating box. Note that using them means that the
  // [decorating box] is not supported.
  const AppliedTextDecoration* const decoration_override_ = nullptr;
  const Font* const font_override_ = nullptr;

  // Geometry of the target text/box.
  const LineRelativeOffset local_origin_;
  const LayoutUnit width_;

  // Cached properties for the current |decoration_index_|.
  const float target_ascent_ = 0.f;
  float ascent_ = 0.f;
  float computed_font_size_ = 0.f;
  float resolved_thickness_ = 0.f;
  const float scaling_factor_;

  int decoration_index_ = 0;

  // |lines_| represents the lines in the current |decoration_index_|, while
  // |union_all_lines_| represents the lines found in any |decoration_index_|.
  //
  // Ideally we would build a vector of the TextDecorationLine instances needing
  // ‘line-through’, but this is a rare case so better to avoid vector overhead.
  TextDecorationLine lines_ = TextDecorationLine::kNone;
  TextDecorationLine union_all_lines_ = TextDecorationLine::kNone;

  ResolvedUnderlinePosition original_underline_position_ =
      ResolvedUnderlinePosition::kNearAlphabeticBaselineAuto;
  ResolvedUnderlinePosition flipped_underline_position_ =
      ResolvedUnderlinePosition::kNearAlphabeticBaselineAuto;

  bool has_underline_ = false;
  bool has_overline_ = false;
  bool flip_underline_and_overline_ = false;
  bool use_decorating_box_ = false;
  const bool minimum_thickness_is_one_ = false;
  bool antialias_ = false;

  struct LineData {
    STACK_ALLOCATED();

   public:
    TextDecorationLine line;
    float line_offset;
    float double_offset;

    // Only used for kDotted and kDashed lines.
    std::optional<Path> stroke_path;

    // Only used for kWavy lines.
    int wavy_offset_factor;
    gfx::RectF wavy_pattern_rect;
    cc::PaintRecord wavy_tile_record;
  };
  LineData line_data_;
  std::optional<Color> highlight_override_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TEXT_DECORATION_INFO_H_
