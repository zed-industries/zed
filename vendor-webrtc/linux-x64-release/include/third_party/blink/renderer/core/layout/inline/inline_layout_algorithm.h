// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_LAYOUT_ALGORITHM_H_

#include "base/dcheck_is_on.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/constraint_space_builder.h"
#include "third_party/blink/renderer/core/layout/inline/inline_node.h"
#include "third_party/blink/renderer/core/layout/inline/line_box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/inline/logical_line_item.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"
#include "third_party/blink/renderer/core/layout/unpositioned_float.h"
#include "third_party/blink/renderer/platform/fonts/font_baseline.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ColumnSpannerPath;
class ConstraintSpace;
class ExclusionSpace;
class InlineBreakToken;
class InlineChildLayoutContext;
class InlineItem;
class InlineLayoutStateStack;
class InlineNode;
class LineInfo;
class LogicalLineContainer;
struct InlineItemResult;
struct LeadingFloats;

// A class for laying out an inline formatting context, i.e. a block with inline
// children.
//
// This class determines the position of InlineItem and build line boxes.
//
// Uses LineBreaker to find InlineItems to form a line.
class CORE_EXPORT InlineLayoutAlgorithm final
    : public LayoutAlgorithm<InlineNode,
                             LineBoxFragmentBuilder,
                             InlineBreakToken> {
 public:
  InlineLayoutAlgorithm(InlineNode,
                        const ConstraintSpace&,
                        const InlineBreakToken*,
                        const ColumnSpannerPath*,
                        InlineChildLayoutContext* context);
  ~InlineLayoutAlgorithm();

  void CreateLine(const LineLayoutOpportunity&,
                  LineInfo*,
                  LogicalLineContainer* line_container);

  const LayoutResult* Layout();

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&) {
    NOTREACHED();
  }

#if EXPENSIVE_DCHECKS_ARE_ON()
  void CheckBoxStates(const LineInfo&) const;
#endif
  void PlaceBlockInInline(const InlineItem&,
                          InlineItemResult*,
                          LogicalLineItems* line_box);

  struct LineClampEllipsis {
    STACK_ALLOCATED();

   public:
    String text;
    const ShapeResult* shape_result;
    FontHeight text_metrics;
  };
  const std::optional<LineClampEllipsis>& GetLineClampEllipsis() {
    return line_clamp_ellipsis_;
  }

 private:
  friend class LineWidthsTest;

  void PositionLeadingFloats(ExclusionSpace&, LeadingFloats&);
  PositionedFloat PositionFloat(LayoutUnit origin_block_bfc_offset,
                                LayoutObject* floating_object,
                                ExclusionSpace*);

  void PrepareBoxStates(const LineInfo&, const InlineBreakToken*);

  void PlaceOutOfFlowObjects(const LineInfo&,
                             const FontHeight&,
                             LogicalLineItems* line_box);
  void PlaceFloatingObjects(const FontHeight&,
                            const LineLayoutOpportunity&,
                            LayoutUnit ruby_block_start_adjust,
                            LineInfo*,
                            LogicalLineItems* line_box);

  LayoutUnit ApplyTextAlign(LineInfo*);

  void ApplyTextBoxTrim(LineInfo&, bool is_truncated);

  // Add any trailing clearance requested by a BR 'clear' attribute on the line.
  // Return true if this was successful (this also includes cases where there is
  // no clearance needed). Return false if the floats that we need to clear past
  // will be resumed in a subsequent fragmentainer.
  bool AddAnyClearanceAfterLine(const LineInfo&);

  LayoutUnit SetAnnotationOverflow(
      const LineInfo& line_info,
      const LogicalLineItems& line_box,
      const FontHeight& line_box_metrics,
      std::optional<FontHeight> annotation_font_height);

  LayoutUnit SetupLineClampEllipsis();

  enum class LineClampState {
    kShow,
    kLineClampEllipsis,
    kTextOverflowEllipsis,
    kHide,
  };
  LineClampState GetLineClampState(const LineInfo*,
                                   LayoutUnit line_box_height) const;

  // Checks whether the remainder of the IFC (i.e. anything after the current
  // break token) would be able to fit in the current line if it didn't have a
  // line-clamp ellipsis that pushes some of that content to the next line.
  //
  // This method will try to compute that without performing actual line
  // breaking, but it will return `nullopt` if it can't.
  std::optional<bool> DoesRemainderFitInLineWithoutEllipsis(const LineInfo&);

  InlineLayoutStateStack* box_states_;
  InlineChildLayoutContext* context_;

  const ColumnSpannerPath* column_spanner_path_;

  MarginStrut end_margin_strut_;
  std::optional<int> lines_until_clamp_;

  std::optional<LineClampEllipsis> line_clamp_ellipsis_;

  FontBaseline baseline_type_ = FontBaseline::kAlphabeticBaseline;

  // True if in quirks or limited-quirks mode, which require line-height quirks.
  // https://quirks.spec.whatwg.org/#the-line-height-calculation-quirk
  unsigned quirks_mode_ : 1;

#if EXPENSIVE_DCHECKS_ARE_ON()
  // True if |box_states_| is taken from |context_|, to check the |box_states_|
  // is the same as when it is rebuilt.
  bool is_box_states_from_context_ = false;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_LAYOUT_ALGORITHM_H_
