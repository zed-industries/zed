// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_NODE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_NODE_DATA_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/inline_items_data.h"
#include "third_party/blink/renderer/core/layout/svg/svg_inline_node_data.h"

namespace blink {

template <typename OffsetMappingBuilder>
class InlineItemsBuilderTemplate;

// Data which is required for inline nodes.
struct CORE_EXPORT InlineNodeData final : InlineItemsData {
 public:
  InlineNodeData() = default;
  // Returns true if `text_content` contains non-latin1 characters other than
  // kObjectReplacementCharacter.
  bool HasNonOrc16BitCharacters() const { return has_non_orc_16bit_; }
  bool IsBidiEnabled() const { return is_bidi_enabled_; }
  TextDirection BaseDirection() const {
    return static_cast<TextDirection>(base_direction_);
  }

  bool HasFloats() const { return has_floats_; }
  bool HasInitialLetterBox() const { return has_initial_letter_box_; }
  bool HasRuby() const { return has_ruby_; }

  bool IsBlockLevel() const { return is_block_level_; }

  // True if this node can't use the bisection in `ParagraphLineBreaker`.
  bool IsBisectLineBreakDisabled() const {
    return is_bisect_line_break_disabled_;
  }
  // True if this node can't use the `ScoreLineBreaker`, that can be
  // determined by `CollectInlines`. Conditions that can change without
  // `CollectInlines` are in `LineBreaker::ShouldDisableScoreLineBreak()`.
  bool IsScoreLineBreakDisabled() const {
    return is_score_line_break_disabled_;
  }

  const InlineItemsData& ItemsData(bool is_first_line) const {
    return !is_first_line || !first_line_items_ ? (const InlineItemsData&)*this
                                                : *first_line_items_;
  }

  void Trace(Visitor* visitor) const override;

 private:
  void SetBaseDirection(TextDirection direction) {
    base_direction_ = static_cast<unsigned>(direction);
  }

  friend class InlineItemsBuilderTest;
  friend class InlineNode;
  friend class InlineNodeForTest;
  friend class OffsetMappingTest;

  template <typename OffsetMappingBuilder>
  friend class InlineItemsBuilderTemplate;

  // Items to use for the first line, when the node has :first-line rules.
  //
  // Items have different ComputedStyle, and may also have different
  // text_content and ShapeResult if 'text-transform' is applied or fonts are
  // different.
  Member<InlineItemsData> first_line_items_;

  Member<SvgInlineNodeData> svg_node_data_;

  unsigned has_non_orc_16bit_ : 1;
  unsigned is_bidi_enabled_ : 1;
  unsigned base_direction_ : 1;  // TextDirection

  unsigned has_floats_ : 1;

  // True if this node contains initial letter box. This value is used for
  // clearing. To control whether subsequent blocks overlap with initial
  // letter[1].
  //   ****** his node ends here.
  //     *    This text from subsequent block one.
  //     *    This text from subsequent block two.
  //     *    This text from subsequent block three.
  // [1] https://drafts.csswg.org/css-inline/#initial-letter-paragraphs
  unsigned has_initial_letter_box_ : 1;

  // The node contains <ruby>.
  unsigned has_ruby_ : 1;

  // We use this flag to determine if we have *only* floats, and OOF-positioned
  // children. If so we consider them block-level, and run the
  // |BlockLayoutAlgorithm| instead of the |InlineLayoutAlgorithm|. This is
  // done to pick up block-level static-position behaviour.
  unsigned is_block_level_ : 1;

  // True if changes to an item may affect different layout of earlier lines.
  // May not be able to use line caches even when the line or earlier lines are
  // not dirty.
  unsigned changes_may_affect_earlier_lines_ : 1;

  unsigned is_bisect_line_break_disabled_ : 1;
  unsigned is_score_line_break_disabled_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_NODE_DATA_H_
