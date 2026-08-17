// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LOGICAL_BOX_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LOGICAL_BOX_FRAGMENT_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/logical_fragment.h"
#include "third_party/blink/renderer/core/layout/physical_box_fragment.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT LogicalBoxFragment final : public LogicalFragment {
 public:
  LogicalBoxFragment(WritingDirectionMode writing_direction,
                     const PhysicalBoxFragment& physical_fragment)
      : LogicalFragment(writing_direction, physical_fragment) {}

  const PhysicalBoxFragment& GetPhysicalBoxFragment() const {
    return To<PhysicalBoxFragment>(physical_fragment_);
  }

  bool IsWritingModeEqual() const {
    return writing_direction_.GetWritingMode() ==
           physical_fragment_.Style().GetWritingMode();
  }

  static LayoutUnit SynthesizedBaseline(FontBaseline baseline_type,
                                        bool is_flipped_lines,
                                        LayoutUnit block_size) {
    if (baseline_type == kAlphabeticBaseline)
      return is_flipped_lines ? LayoutUnit() : block_size;

    return block_size / 2;
  }

  std::optional<LayoutUnit> FirstBaseline() const {
    if (!IsWritingModeEqual())
      return std::nullopt;

    auto baseline = GetPhysicalBoxFragment().FirstBaseline();
    if (baseline && physical_fragment_.IsScrollContainer())
      baseline = std::max(LayoutUnit(), std::min(*baseline, BlockSize()));

    return baseline;
  }

  LayoutUnit FirstBaselineOrSynthesize(FontBaseline baseline_type) const {
    if (auto first_baseline = FirstBaseline())
      return *first_baseline;

    return SynthesizedBaseline(
        baseline_type, writing_direction_.IsFlippedLines(), BlockSize());
  }

  std::optional<LayoutUnit> LastBaseline() const {
    if (!IsWritingModeEqual())
      return std::nullopt;

    auto baseline = GetPhysicalBoxFragment().LastBaseline();
    if (baseline && physical_fragment_.IsScrollContainer())
      baseline = std::max(LayoutUnit(), std::min(*baseline, BlockSize()));

    return baseline;
  }

  LayoutUnit LastBaselineOrSynthesize(FontBaseline baseline_type) const {
    if (auto last_baseline = LastBaseline())
      return *last_baseline;

    return SynthesizedBaseline(
        baseline_type, writing_direction_.IsFlippedLines(), BlockSize());
  }

  // Compute baseline metrics (ascent/descent) for this box.
  //
  // This will synthesize baseline metrics if no baseline is available. See
  // |Baseline()| for when this may occur.
  FontHeight BaselineMetrics(const LineBoxStrut& margins, FontBaseline) const;

  BoxStrut Borders() const {
    return GetPhysicalBoxFragment().Borders().ConvertToLogical(
        writing_direction_);
  }
  BoxStrut Scrollbar() const {
    return GetPhysicalBoxFragment().Scrollbar().ConvertToLogical(
        writing_direction_);
  }
  BoxStrut Padding() const {
    return GetPhysicalBoxFragment().Padding().ConvertToLogical(
        writing_direction_);
  }
  BoxStrut BoxDecorations() const {
    return Borders() + Scrollbar() + Padding();
  }

  bool HasDescendantsForTablePart() const {
    return GetPhysicalBoxFragment().HasDescendantsForTablePart();
  }

  LayoutUnit BlockEndScrollableOverflow() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LOGICAL_BOX_FRAGMENT_H_
