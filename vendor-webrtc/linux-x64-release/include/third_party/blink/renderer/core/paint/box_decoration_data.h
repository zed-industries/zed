// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_DECORATION_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_DECORATION_DATA_H_

#include <optional>

#include "third_party/blink/renderer/core/css/properties/longhands.h"
#include "third_party/blink/renderer/core/layout/background_bleed_avoidance.h"
#include "third_party/blink/renderer/core/layout/layout_box.h"
#include "third_party/blink/renderer/core/layout/layout_replaced.h"
#include "third_party/blink/renderer/core/layout/physical_box_fragment.h"
#include "third_party/blink/renderer/core/paint/paint_info.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/platform/graphics/color.h"

namespace blink {

// Data for box decoration painting.
class BoxDecorationData {
  STACK_ALLOCATED();

 public:
  BoxDecorationData(const PaintInfo& paint_info,
                    const LayoutReplaced& layout_replaced)
      : BoxDecorationData(paint_info,
                          layout_replaced,
                          layout_replaced.StyleRef(),
                          layout_replaced.StyleRef().HasBorderDecoration()) {}

  BoxDecorationData(const PaintInfo& paint_info,
                    const PhysicalFragment& fragment,
                    const ComputedStyle& style)
      : BoxDecorationData(
            paint_info,
            To<LayoutBox>(*fragment.GetLayoutObject()),
            style,
            !fragment.HasCollapsedBorders() && style.HasBorderDecoration()) {}

  BoxDecorationData(const PaintInfo& paint_info,
                    const PhysicalFragment& fragment)
      : BoxDecorationData(paint_info, fragment, fragment.Style()) {}

  BoxDecorationData BackgroundOnly() const {
    DCHECK(should_paint_background_);
    return BoxDecorationData(*this, /*should_paint_background=*/true,
                             /*should_paint_border=*/false);
  }
  BoxDecorationData BorderOnly() const {
    DCHECK(should_paint_border_);
    return BoxDecorationData(*this, /*should_paint_background=*/false,
                             /*should_paint_border=*/true);
  }

  bool IsPaintingBackgroundInContentsSpace() const {
    return paint_info_.IsPaintingBackgroundInContentsSpace();
  }
  bool HasAppearance() const { return has_appearance_; }
  bool ShouldPaintBackground() const { return should_paint_background_; }
  bool ShouldPaintBorder() const { return should_paint_border_; }
  bool ShouldPaintShadow() const { return should_paint_shadow_; }
  bool ShouldPaintGapDecorations() const {
    return should_paint_gap_decorations_;
  }

  BackgroundBleedAvoidance GetBackgroundBleedAvoidance() const {
    if (!bleed_avoidance_)
      bleed_avoidance_ = ComputeBleedAvoidance();
    return *bleed_avoidance_;
  }

  bool ShouldPaint() const {
    return HasAppearance() || ShouldPaintBackground() || ShouldPaintBorder() ||
           ShouldPaintShadow() || ShouldPaintGapDecorations();
  }

  // This is not cached because the caller is unlikely to call this repeatedly.
  Color BackgroundColor() const {
    return style_.VisitedDependentColor(GetCSSPropertyBackgroundColor());
  }

 private:
  BoxDecorationData(const PaintInfo& paint_info,
                    const LayoutBox& layout_box,
                    const ComputedStyle& style,
                    const bool has_non_collapsed_border_decoration)
      : paint_info_(paint_info),
        layout_box_(layout_box),
        style_(style),
        has_appearance_(style.HasEffectiveAppearance()),
        should_paint_background_(ComputeShouldPaintBackground()),
        should_paint_border_(
            ComputeShouldPaintBorder(has_non_collapsed_border_decoration)),
        should_paint_shadow_(ComputeShouldPaintShadow()),
        should_paint_gap_decorations_(ComputeShouldPaintGapDecorations()) {}

  // For BackgroundOnly() and BorderOnly().
  BoxDecorationData(const BoxDecorationData& data,
                    bool should_paint_background,
                    bool should_paint_border)
      : paint_info_(data.paint_info_),
        layout_box_(data.layout_box_),
        style_(data.style_),
        has_appearance_(false),
        should_paint_background_(should_paint_background),
        should_paint_border_(should_paint_border),
        should_paint_shadow_(false),
        should_paint_gap_decorations_(false) {
    DCHECK(!data.has_appearance_);
    DCHECK(!data.should_paint_shadow_);
    DCHECK(!data.should_paint_gap_decorations_);
  }

  bool ComputeShouldPaintBackground() const {
    // The page border box fragment paints the document background, so we cannot
    // trust its computed style when it comes to background properties.
    //
    // See https://drafts.csswg.org/css-page-3/#painting
    //
    // TODO(crbug.com/40286153): This is a false positive. We should be able to
    // remove this once we have a better way to determine whether there is a
    // background.
    bool has_background =
        style_.HasBackground() ||
        GetBoxFragmentType() == PhysicalFragment::kPageBorderBox;
    return has_background && !layout_box_.BackgroundTransfersToView() &&
           !paint_info_.ShouldSkipBackground();
  }

  bool ComputeShouldPaintBorder(
      bool has_non_collapsed_border_decoration) const {
    if (paint_info_.IsPaintingBackgroundInContentsSpace())
      return false;
    return has_non_collapsed_border_decoration;
  }

  bool ComputeShouldPaintShadow() const {
    return !paint_info_.IsPaintingBackgroundInContentsSpace() &&
           style_.BoxShadow();
  }

  bool ComputeShouldPaintGapDecorations() const {
    return style_.HasColumnRule() || style_.HasRowRule();
  }

  bool BorderObscuresBackgroundEdge() const;
  BackgroundBleedAvoidance ComputeBleedAvoidance() const;

  PhysicalFragment::BoxType GetBoxFragmentType() const {
    if (!layout_box_.PhysicalFragmentCount()) {
      return PhysicalFragment::kNormalBox;
    }
    return layout_box_.GetPhysicalFragment(0)->GetBoxType();
  }

  // Inputs.
  const PaintInfo& paint_info_;
  const LayoutBox& layout_box_;
  const ComputedStyle& style_;

  // Outputs that are initialized in the constructor.
  const bool has_appearance_;
  const bool should_paint_background_;
  const bool should_paint_border_;
  const bool should_paint_shadow_;
  const bool should_paint_gap_decorations_;
  // This is lazily initialized.
  mutable std::optional<BackgroundBleedAvoidance> bleed_avoidance_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_DECORATION_DATA_H_
