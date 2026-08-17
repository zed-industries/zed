// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_INFO_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_break_token.h"
#include "third_party/blink/renderer/core/layout/geometry/bfc_offset.h"
#include "third_party/blink/renderer/core/layout/inline/inline_break_token.h"
#include "third_party/blink/renderer/core/layout/inline/inline_item_result.h"
#include "third_party/blink/renderer/core/layout/inline/inline_item_text_index.h"
#include "third_party/blink/renderer/core/layout/layout_result.h"
#include "third_party/blink/renderer/core/style/computed_style_base_constants.h"

namespace blink {

class ComputedStyle;
class InlineBreakToken;
class InlineNode;
struct InlineItemsData;

// Represents a line to build.
//
// This is a transient context object only while building line boxes.
//
// LineBreaker produces, and InlineLayoutAlgorithm consumes.
class CORE_EXPORT LineInfo {
  DISALLOW_NEW();

 public:
  void Trace(Visitor* visitor) const;
  void Reset();

  const InlineItemsData& ItemsData() const {
    DCHECK(items_data_);
    return *items_data_;
  }

  // The style to use for the line.
  const ComputedStyle& LineStyle() const {
    DCHECK(line_style_);
    return *line_style_;
  }
  void SetLineStyle(const InlineNode&,
                    const InlineItemsData&,
                    bool use_first_line_style);
  void OverrideLineStyle(const ComputedStyle& style) { line_style_ = style; }

  // True if this line is a first formatted line.
  // https://drafts.csswg.org/css-pseudo-4/#first-formatted-line
  bool IsFirstFormattedLine() const { return is_first_formatted_line_; }
  void SetIsFirstFormattedLine(bool value) { is_first_formatted_line_ = value; }

  // Use ::first-line style if true.
  // https://drafts.csswg.org/css-pseudo/#selectordef-first-line
  // This is false for the "first formatted line" if '::first-line' rule is not
  // used in the document.
  // https://www.w3.org/TR/CSS22/selector.html#first-formatted-line
  bool UseFirstLineStyle() const { return use_first_line_style_; }

  // The last line of a block, or the line ends with a forced line break.
  // https://drafts.csswg.org/css-text-3/#propdef-text-align-last
  bool IsLastLine() const { return is_last_line_; }
  void SetIsLastLine(bool is_last_line) { is_last_line_ = is_last_line; }

  // Whether this line has ended with a forced break or not. Note, the forced
  // break item may not be the last item if trailing items are included, or even
  // does not exist if synthesized for block-in-inline.
  bool HasForcedBreak() const { return has_forced_break_; }
  void SetHasForcedBreak() { has_forced_break_ = true; }

  // If the line is marked as empty, it means that there's no content that
  // requires it to be present at all, e.g. when there are only close tags with
  // no margin/border/padding.
  bool IsEmptyLine() const { return is_empty_line_; }
  void SetIsEmptyLine() { is_empty_line_ = true; }

  // If this line is empty, but still should have height as editable.
  bool HasLineEvenIfEmpty() const { return has_line_even_if_empty_; }
  void SetHasLineEvenIfEmpty() { has_line_even_if_empty_ = true; }

  // Returns true if this line is a block-in-inline.
  bool IsBlockInInline() const { return is_block_in_inline_; }
  void SetIsBlockInInline() { is_block_in_inline_ = true; }

  bool IsRubyBase() const { return is_ruby_base_; }
  void SetIsRubyBase() { is_ruby_base_ = true; }
  bool IsRubyText() const { return is_ruby_text_; }
  void SetIsRubyText() { is_ruby_text_ = true; }

  // InlineItemResults for this line.
  InlineItemResults* MutableResults() { return &results_; }
  const InlineItemResults& Results() const { return results_; }

  const InlineBreakToken* GetBreakToken() const { return break_token_; }
  void SetBreakToken(const InlineBreakToken* break_token) {
    break_token_ = break_token;
  }
  // True if this line ends a paragraph; i.e., ends a block or has a forced
  // break.
  bool IsEndParagraph() const { return !GetBreakToken() || HasForcedBreak(); }

  HeapVector<Member<const InlineBreakToken>>& ParallelFlowBreakTokens() {
    return parallel_flow_break_tokens_;
  }
  void PropagateParallelFlowBreakToken(const InlineBreakToken* token) {
    DCHECK(token->IsInParallelBlockFlow());
    parallel_flow_break_tokens_.push_back(token);
  }
  void RemoveParallelFlowBreakToken(unsigned item_index);

  std::optional<LayoutUnit> MinimumSpaceShortage() const {
    return minimum_space_shortage_;
  }
  void PropagateMinimumSpaceShortage(LayoutUnit shortage) {
    DCHECK_GT(shortage, LayoutUnit());
    if (minimum_space_shortage_) {
      minimum_space_shortage_ = std::min(*minimum_space_shortage_, shortage);
    } else {
      minimum_space_shortage_ = shortage;
    }
  }

  void SetTextIndent(LayoutUnit indent) { text_indent_ = indent; }
  LayoutUnit TextIndent() const { return text_indent_; }

  ETextAlign TextAlign() const { return text_align_; }
  // Update |TextAlign()| and related fields. This depends on |IsLastLine()| and
  // that must be called after |SetIsLastLine()|.
  void UpdateTextAlign();

  BfcOffset GetBfcOffset() const { return bfc_offset_; }
  LayoutUnit AvailableWidth() const { return available_width_; }

  // The width of this line, including the hanging width from trailing spaces.
  // Negative width created by negative 'text-indent' is clamped to zero.
  LayoutUnit Width() const { return width_.ClampNegativeToZero(); }
  // Same as |Width()| but returns negatives value as is. The hanging width
  // (e.g. from preserved trailing spaces) may or may not be included, depends
  // on |ShouldHangTrailingSpaces()|.
  LayoutUnit WidthForAlignment() const {
    return width_ - HangWidthForAlignment();
  }
  // Width that hangs over the end of the line; e.g., preserved trailing spaces.
  // See https://drafts.csswg.org/css-text/#hanging.
  LayoutUnit HangWidth() const { return hang_width_; }
  // Same as |HangWidth()| but it may be 0 depending on
  // |ShouldHangTrailingSpaces()|.
  LayoutUnit HangWidthForAlignment() const {
    return allow_hang_for_alignment_ ? hang_width_ : LayoutUnit();
  }
  // Compute |Width()| from |Results()|. Used during line breaking, before
  // |Width()| is set. After line breaking, this should match to |Width()|
  // without clamping.
  LayoutUnit ComputeWidth() const;

#if DCHECK_IS_ON()
  // Returns width in float. This function is used for avoiding |LayoutUnit|
  // saturated addition of items in line.
  float ComputeWidthInFloat() const;
#endif

  bool HasTrailingSpaces() const { return has_trailing_spaces_; }
  void SetHasTrailingSpaces() { has_trailing_spaces_ = true; }

  // True if this line has overflow, excluding preserved trailing spaces.
  bool HasOverflow() const { return has_overflow_; }
  void SetHasOverflow(bool value = true) { has_overflow_ = value; }

  // True if this line is hyphenated.
  bool IsHyphenated() const;

  void SetBfcOffset(const BfcOffset& bfc_offset) { bfc_offset_ = bfc_offset; }
  void SetWidth(LayoutUnit available_width, LayoutUnit width) {
    available_width_ = available_width;
    width_ = width;
  }

  // Start offset of this line.
  const InlineItemTextIndex& Start() const { return start_; }
  unsigned StartOffset() const { return start_.text_offset; }
  void SetStart(const InlineItemTextIndex& index) { start_ = index; }

  // Start text offset of this line, excluding out-of-flow objects, and
  // zero-length items.
  // Returns EndTextOffset() if the line is empty or all item results are
  // excluded.
  unsigned InflowStartOffset() const;

  // End offset of this line. This is the same as the start offset of the next
  // line, or the end of block if this is the last line.
  InlineItemTextIndex End() const;
  unsigned EndTextOffset() const;
  // End text offset of this line, excluding out-of-flow objects such as
  // floating or positioned.
  unsigned InflowEndOffset() const {
    return InflowEndOffsetInternal(/* skip_forced_break */ false);
  }
  // In addition to the above, forced breaks and collapsed spaces are excluded.
  unsigned InflowEndOffsetWithoutForcedBreak() const {
    return InflowEndOffsetInternal(/* skip_forced_break */ true);
  }
  // End text offset for `text-align: justify`. This excludes preserved trailing
  // spaces. Available only when |TextAlign()| is |kJustify|.
  unsigned EndOffsetForJustify() const {
    DCHECK_EQ(text_align_, ETextAlign::kJustify);
    return end_offset_for_justify_;
  }
  // End item index of this line.
  unsigned EndItemIndex() const { return end_item_index_; }
  void SetEndItemIndex(unsigned index) { end_item_index_ = index; }

  bool GlyphCountIsGreaterThan(wtf_size_t limit) const;

  // The base direction of this line for the bidi algorithm.
  TextDirection BaseDirection() const { return base_direction_; }
  void SetBaseDirection(TextDirection direction) {
    base_direction_ = direction;
  }

  // Whether an accurate end position is needed, typically for end, center, and
  // justify alignment.
  bool NeedsAccurateEndPosition() const { return needs_accurate_end_position_; }

  // The block-in-inline layout result.
  const LayoutResult* BlockInInlineLayoutResult() const {
    return block_in_inline_layout_result_;
  }
  void SetBlockInInlineLayoutResult(const LayoutResult* layout_result) {
    block_in_inline_layout_result_ = std::move(layout_result);
  }

  // |MayHaveTextCombineOrRubyItem()| is a flag for special text handling
  // during "text-align:justify".
  bool MayHaveTextCombineOrRubyItem() const {
    return may_have_text_combine_or_ruby_item_;
  }
  void SetHaveTextCombineOrRubyItem() {
    may_have_text_combine_or_ruby_item_ = true;
  }

  // True if the line might contain ruby overhang. It affects min-max
  // computation.
  bool MayHaveRubyOverhang() const { return may_have_ruby_overhang_; }
  void SetMayHaveRubyOverhang() { may_have_ruby_overhang_ = true; }

  // Returns annotation block start adjustment base on annotation and initial
  // letter.
  LayoutUnit ComputeAnnotationBlockOffsetAdjustment() const;

  // Returns block start adjustment for line base on annotation and initial
  // letter.
  LayoutUnit ComputeBlockStartAdjustment() const;

  // Returns block start adjustment for initial letter box base on annotation
  // and initial letter.
  LayoutUnit ComputeInitialLetterBoxBlockStartAdjustment() const;

  // Returns total block size of this line to check whether we should use next
  // layout opportunity or not base on `line_height`, annotation and initial
  // letter box.
  LayoutUnit ComputeTotalBlockSize(
      LayoutUnit line_height,
      LayoutUnit annotation_overflow_block_end) const;

  void SetAnnotationBlockStartAdjustment(LayoutUnit amount) {
    DCHECK(!IsEmptyLine());
    annotation_block_start_adjustment_ = amount;
  }

  void SetInitialLetterBlockStartAdjustment(LayoutUnit amount) {
    DCHECK_GE(amount, LayoutUnit());
    DCHECK(!IsEmptyLine());
    initial_letter_box_block_start_adjustment_ = amount;
  }

  void SetInitialLetterBoxBlockSize(LayoutUnit block_size) {
    DCHECK_GE(block_size, LayoutUnit());
    initial_letter_box_block_size_ = block_size;
  }

 private:
  ETextAlign GetTextAlign(bool is_last_line = false) const;
  bool ComputeNeedsAccurateEndPosition() const;

  // The width of preserved trailing spaces.
  LayoutUnit ComputeTrailingSpaceWidth(
      unsigned* end_offset_out = nullptr) const;
  unsigned InflowEndOffsetInternal(bool skip_forced_break) const;

  Member<const InlineItemsData> items_data_;
  Member<const ComputedStyle> line_style_;
  InlineItemResults results_;

  BfcOffset bfc_offset_;

  Member<const InlineBreakToken> break_token_;
  HeapVector<Member<const InlineBreakToken>> parallel_flow_break_tokens_;

  Member<const LayoutResult> block_in_inline_layout_result_;

  std::optional<LayoutUnit> minimum_space_shortage_;

  LayoutUnit available_width_;
  LayoutUnit width_;
  LayoutUnit hang_width_;
  LayoutUnit text_indent_;

  LayoutUnit annotation_block_start_adjustment_;
  LayoutUnit initial_letter_box_block_start_adjustment_;
  LayoutUnit initial_letter_box_block_size_;

  InlineItemTextIndex start_;
  unsigned end_item_index_;
  unsigned end_offset_for_justify_;

  ETextAlign text_align_ = ETextAlign::kLeft;
  TextDirection base_direction_ = TextDirection::kLtr;

  bool is_first_formatted_line_ = false;
  bool use_first_line_style_ = false;
  bool is_last_line_ = false;
  bool has_forced_break_ = false;
  bool is_empty_line_ = false;
  bool has_line_even_if_empty_ = false;
  bool is_block_in_inline_ = false;
  bool has_overflow_ = false;
  bool has_trailing_spaces_ = false;
  bool needs_accurate_end_position_ = false;
  bool is_ruby_base_ = false;
  bool is_ruby_text_ = false;
  // Even if text combine item causes line break, this variable is not reset.
  // This variable is used to add spacing before/after text combine items if
  // "text-align: justify".
  // Also, the variable is used to represent existence of <ruby>, which needs
  // special handling for "text-align: justify".
  // Note: To avoid scanning |InlineItemResults|, this variable is true
  // when |InlineItemResult| to |results_|.
  bool may_have_text_combine_or_ruby_item_ = false;
  // True if the last processed line might contain ruby overhang.
  bool may_have_ruby_overhang_ = false;
  bool allow_hang_for_alignment_ = false;

  // When adding fields, pelase ensure `Reset()` is in sync.
};

std::ostream& operator<<(std::ostream& ostream, const LineInfo& line_info);

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::LineInfo)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_INFO_H_
