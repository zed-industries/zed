// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEM_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEM_RESULT_H_

#include <optional>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/core/layout/inline/hyphen_result.h"
#include "third_party/blink/renderer/core/layout/inline/inline_item_text_index.h"
#include "third_party/blink/renderer/core/layout/inline/text_offset_range.h"
#include "third_party/blink/renderer/core/layout/positioned_float.h"
#include "third_party/blink/renderer/platform/fonts/shaping/shape_result.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class InlineItem;
class LayoutResult;
class ShapeResult;
class ShapeResultView;
struct InlineItemResultRubyColumn;
struct PositionedFloat;

// The result of measuring InlineItem.
//
// This is a transient context object only while building line boxes.
// Produced while determining the line break points, but these data are needed
// to create line boxes.
//
// LineBreaker produces, and InlineLayoutAlgorithm consumes.
struct CORE_EXPORT InlineItemResult {
  DISALLOW_NEW();

  // A wrapper around PositionedFloat that acts like an std::optional but traces
  // the underlying value regardless of whether or not it was initialized. It is
  // uni-directional in the sense that is never reset after being assigned to.
  class OptionalPositionedFloat {
    DISALLOW_NEW();

   public:
    OptionalPositionedFloat& operator=(PositionedFloat value) {
      has_value_ = true;
      value_ = value;
      return *this;
    }
    explicit operator bool() const { return has_value_; }
    PositionedFloat* operator->() {
      DCHECK(has_value_);
      return &value_;
    }
    const PositionedFloat* operator->() const {
      return const_cast<OptionalPositionedFloat*>(this)->operator->();
    }

    void Trace(Visitor* visitor) const { visitor->Trace(value_); }

   private:
    bool has_value_ = false;
    PositionedFloat value_;
  };

 public:
  InlineItemResult() = default;
  InlineItemResult(const InlineItem&,
                   unsigned index,
                   const TextOffsetRange& text_offset,
                   bool break_anywhere_if_overflow,
                   bool should_create_line_box,
                   bool has_unpositioned_floats);

  const TextOffsetRange& TextOffset() const { return text_offset; }
  wtf_size_t StartOffset() const { return text_offset.start; }
  wtf_size_t EndOffset() const { return text_offset.end; }
  wtf_size_t Length() const { return text_offset.Length(); }

  InlineItemTextIndex Start() const { return {item_index, StartOffset()}; }
  InlineItemTextIndex End() const { return {item_index, EndOffset()}; }

  // Return `true` if the InlineItem type is kOpenRubyColumn and this contains
  // data for the base and annotation lines.
  bool IsRubyColumn() const { return ruby_column; }

  // Compute/clear |hyphen_string| and |hyphen_shape_result|.
  void ShapeHyphen();

  void Trace(Visitor* visitor) const;
#if DCHECK_IS_ON()
  void CheckConsistency(bool allow_null_shape_result = false) const;
#endif
  // `indent` is prepended to the content. If the content consists of multiple
  // lines, `indent` is prepended to each of lines.
  String ToString(const String& ifc_text_content,
                  const String& indent = "") const;

  // The InlineItem and its index.
  // Note that use `item_index` with caution, which may not always be the actual
  // item index in the items list. See `LineBreaker::AddItem`.
  Member<const InlineItem> item;
  unsigned item_index = 0;

  // The range of text content for this item.
  TextOffsetRange text_offset;

  // Inline size of this item.
  LayoutUnit inline_size;

  // Non-zero if text-combine after non-ideographic character
  // See "text-combine-justify.html".
  LayoutUnit spacing_before;

  // Pending inline-end overhang amount for RubyColumn.
  // This is committed if a following item meets conditions.
  LayoutUnit pending_end_overhang;

  // ShapeResult for text items. Maybe different from InlineItem if re-shape
  // is needed in the line breaker.
  Member<const ShapeResultView> shape_result;

  // Hyphen character and its |ShapeResult|.
  // Use |is_hyphenated| to determine whether this item is hyphenated or not.
  // This field may be set even when this item is not hyphenated.
  HyphenResult hyphen;

  // LayoutResult for atomic inline items.
  Member<const LayoutResult> layout_result;

  // Data for kOpenRubyColumn type. This member is null for other types.
  Member<InlineItemResultRubyColumn> ruby_column;

  // PositionedFloat for floating inline items. Should only be present for
  // positioned floats (not unpositioned). It indicates where it was placed
  // within the BFC.
  OptionalPositionedFloat positioned_float;
  ExclusionSpace exclusion_space_before_position_float;

  // Margins, borders, and padding for open tags.
  // Margins are set for atomic inlines too.
  LineBoxStrut margins;
  LineBoxStrut borders;
  LineBoxStrut padding;

  // Inside of this may be breakable. False means there are no break
  // opportunities, or has CSS properties that prohibit breaking.
  // Used only during line breaking.
  bool may_break_inside = false;

  // Lines can break after this item. Set for all items.
  // Used only during line breaking.
  bool can_break_after = false;

  // True if this item contains only trailing spaces that may hang with
  // 'white-space: pre-wrap'.
  // Trailing spaces are measured differently that they are split from other
  // text items.
  bool has_only_pre_wrap_trailing_spaces = false;

  // True if this item contains only trailing spaces whose bidirectional
  // character type is WS (whitespace neutral).
  // The direction and bidi level of such items will be ignored and treated as
  // if they had the base direction.
  bool has_only_bidi_trailing_spaces = false;

  // The previous value of |break_anywhere_if_overflow| in the
  // InlineItemResults list. Like |should_create_line_box|, this value is used
  // to rewind properly.
  bool break_anywhere_if_overflow = false;

  // We don't create "certain zero-height line boxes".
  // https://drafts.csswg.org/css2/visuren.html#phantom-line-box
  // Such line boxes do not prevent two margins being "adjoining", and thus
  // collapsing.
  // https://drafts.csswg.org/css2/box.html#collapsing-margins
  //
  // This field should be initialized to the previous value in the
  // InlineItemResults list. If line breaker rewinds InlineItemResults
  // list, we can still look at the last value in the list to determine if we
  // need a line box. E.g.
  // [float should_create_line_box: false], [text should_create_line_box: true]
  //
  // If "text" doesn't fit, and we rewind so that we only have "float", we can
  // correctly determine that we don't need a line box.
  bool should_create_line_box = false;

  // This field should be initialized and maintained like
  // |should_create_line_box|. It indicates if there are (at the current
  // position) any unpositioned floats.
  bool has_unpositioned_floats = false;

  // True if this is hyphenated. The hyphen is in |hyphen_string| and
  // |hyphen_shape_result|.
  bool is_hyphenated = false;
};

// Represents a set of InlineItemResult that form a line box.
using InlineItemResults = HeapVector<InlineItemResult, 32>;

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::InlineItemResult)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEM_RESULT_H_
