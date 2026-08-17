// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LOGICAL_LINE_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LOGICAL_LINE_BUILDER_H_

#include "third_party/blink/renderer/core/layout/inline/inline_item_result.h"
#include "third_party/blink/renderer/core/layout/inline/inline_node.h"

namespace blink {

class InlineBreakToken;
class InlineChildLayoutContext;
class InlineLayoutAlgorithm;
class InlineLayoutStateStack;
class LineInfo;
class LogicalLineItems;
struct InlineBoxState;
struct LogicalRubyColumn;

// This class is responsible to build a LogicalLineItems from a LineInfo.
// It's a helper for InlineLayoutAlgorithm.
class LogicalLineBuilder {
  STACK_ALLOCATED();

 public:
  LogicalLineBuilder(InlineNode node,
                     const ConstraintSpace& constraint_space,
                     const InlineBreakToken* break_token,
                     InlineLayoutStateStack* state_stack,
                     InlineChildLayoutContext* context);

  void RebuildBoxStates(const LineInfo& line_info,
                        wtf_size_t start_item_index,
                        wtf_size_t end_item_index);

  // `main_line_helper` can be nullptr if the line is for ruby annotations.
  void CreateLine(LineInfo* line_info,
                  LogicalLineItems* line_box,
                  InlineLayoutAlgorithm* main_line_helper);

  // The following four functions are available after calling CreateLine().
  const InlineItemResult* InitialLetterItemResult() const {
    return initial_letter_item_result_;
  }
  bool HasOutOfFlowPositionedItems() const {
    return has_out_of_flow_positioned_items_;
  }
  bool HasFloatingItems() const { return has_floating_items_; }
  bool HasRelativePositionedItems() const {
    return has_relative_positioned_items_;
  }

 private:
  InlineBoxState* HandleItemResults(const LineInfo& line_info,
                                    InlineItemResults& line_items,
                                    LogicalLineItems* line_box,
                                    InlineLayoutAlgorithm* main_line_helper,
                                    InlineBoxState* box);
  InlineBoxState* HandleOpenTag(const InlineItem&,
                                const InlineItemResult&,
                                LogicalLineItems*);
  InlineBoxState* HandleCloseTag(const InlineItem&,
                                 const InlineItemResult&,
                                 LogicalLineItems* line_box,
                                 InlineBoxState*);

  void PlaceControlItem(const InlineItem&,
                        const String& text_content,
                        InlineItemResult*,
                        LogicalLineItems* line_box,
                        InlineBoxState*);
  void PlaceHyphen(const InlineItemResult&,
                   LayoutUnit hyphen_inline_size,
                   LogicalLineItems* line_box,
                   InlineBoxState*);
  InlineBoxState* PlaceAtomicInline(const InlineItem&,
                                    InlineItemResult*,
                                    LogicalLineItems* line_box);
  void PlaceInitialLetterBox(const InlineItem&,
                             InlineItemResult*,
                             LogicalLineItems* line_box);
  void PlaceLayoutResult(InlineItemResult*,
                         LogicalLineItems* line_box,
                         InlineBoxState*,
                         LayoutUnit inline_offset = LayoutUnit());
  InlineBoxState* PlaceRubyColumn(const LineInfo& line_info,
                                  InlineItemResult& item_result,
                                  LogicalLineItems& line_box,
                                  InlineBoxState* box);
  void PlaceRubyAnnotation(InlineItemResult& item_result,
                           std::optional<LayoutUnit> line_available_size,
                           wtf_size_t index,
                           LineInfo& annotation_line,
                           LogicalRubyColumn& logical_column);
  void PlaceListMarker(const InlineItem&, InlineItemResult*);

  void BidiReorder(TextDirection base_direction,
                   LogicalLineItems* line_box,
                   HeapVector<Member<LogicalRubyColumn>>& column_list);

  InlineNode node_;
  const ConstraintSpace& constraint_space_;
  const InlineBreakToken* break_token_;
  InlineLayoutStateStack* box_states_;
  InlineChildLayoutContext* context_;
  const FontBaseline baseline_type_;

  // True if in quirks or limited-quirks mode, which require line-height quirks.
  // https://quirks.spec.whatwg.org/#the-line-height-calculation-quirk
  const bool quirks_mode_ : 1;

  // Output of CreateLine():

  bool has_out_of_flow_positioned_items_ : 1 = false;
  bool has_floating_items_ : 1 = false;
  bool has_relative_positioned_items_ : 1 = false;
  const InlineItemResult* initial_letter_item_result_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LOGICAL_LINE_BUILDER_H_
