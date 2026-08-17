// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_NODE_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/inline_node_data.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"
#include "third_party/blink/renderer/core/layout/layout_input_node.h"
#include "third_party/blink/renderer/core/layout/svg/svg_character_data.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class BreakToken;
class ColumnSpannerPath;
class ConstraintSpace;
class InlineChildLayoutContext;
class LayoutResult;
class OffsetMapping;
struct InlineItemsData;
struct SvgTextContentRange;
struct TextDiffRange;

// Represents an anonymous block box to be laid out, that contains consecutive
// inline nodes and their descendants.
class CORE_EXPORT InlineNode : public LayoutInputNode {
 public:
  explicit InlineNode(LayoutBlockFlow*);
  InlineNode(std::nullptr_t) : LayoutInputNode(nullptr) {}

  LayoutBlockFlow* GetLayoutBlockFlow() const {
    return To<LayoutBlockFlow>(box_.Get());
  }

  const LayoutResult* Layout(const ConstraintSpace&,
                             const BreakToken*,
                             const ColumnSpannerPath*,
                             InlineChildLayoutContext* context) const;

  // Computes the value of min-content and max-content for this anonymous block
  // box. min-content is the inline size when lines wrap at every break
  // opportunity, and max-content is when lines do not wrap at all.
  MinMaxSizesResult ComputeMinMaxSizes(WritingMode container_writing_mode,
                                       const ConstraintSpace&,
                                       const MinMaxSizesFloatInput&) const;

  // Instruct to re-compute |PrepareLayout| on the next layout.
  void InvalidatePrepareLayoutForTest() {
    LayoutBlockFlow* block_flow = GetLayoutBlockFlow();
    block_flow->ResetInlineNodeData();
    DCHECK(!IsPrepareLayoutFinished());
  }

  const InlineItemsData& ItemsData(bool is_first_line) const {
    return Data().ItemsData(is_first_line);
  }

  // There's a special intrinsic size measure quirk for images that are direct
  // children of table cells that have auto inline-size: When measuring
  // intrinsic min/max inline sizes, we pretend that it's not possible to break
  // between images, or between text and images. Note that this only applies
  // when measuring. During actual layout, on the other hand, standard breaking
  // rules are to be followed.
  // See https://quirks.spec.whatwg.org/#the-table-cell-width-calculation-quirk
  bool IsStickyImagesQuirkForContentSize() const;

  // Returns the text content to use for content sizing. This is normally the
  // same as |items_data.text_content|, except when sticky images quirk is
  // needed.
  static String TextContentForStickyImagesQuirk(const InlineItemsData&);

  // Returns true if we don't need to collect inline items after replacing
  // |layout_text| after deleting replacing subtext from |offset| to |length|
  // |new_text| is new text of |layout_text|.
  // This is optimized version of |PrepareLayout()|.
  static bool SetTextWithOffset(LayoutText* layout_text,
                                String new_text,
                                const TextDiffRange&);

  // Returns the DOM to text content offset mapping of this block. If it is not
  // computed before, compute and store it in InlineNodeData.
  // This function must be called with clean layout.
  const OffsetMapping* ComputeOffsetMappingIfNeeded() const;

  // Get |OffsetMapping| for the |layout_block_flow|. |layout_block_flow|
  // should be laid out. This function works for both new and legacy layout.
  static const OffsetMapping* GetOffsetMapping(
      LayoutBlockFlow* layout_block_flow);

  bool IsBidiEnabled() const { return Data().is_bidi_enabled_; }
  TextDirection BaseDirection() const { return Data().BaseDirection(); }

  bool HasFloats() const { return Data().HasFloats(); }
  bool HasInitialLetterBox() const { return Data().has_initial_letter_box_; }
  bool HasRuby() const { return Data().has_ruby_; }

  bool IsBlockLevel() { return EnsureData().is_block_level_; }

  // True if this node can't use the bisection in `ParagraphLineBreaker`.
  bool IsBisectLineBreakDisabled() const {
    return Data().IsBisectLineBreakDisabled();
  }
  // True if this node can't use the `ScoreLineBreaker`, that can be
  // determined by `CollectInlines`. Conditions that can change without
  // `CollectInlines` are in `LineBreaker::ShouldDisableScoreLineBreak()`.
  bool IsScoreLineBreakDisabled() const {
    return Data().IsScoreLineBreakDisabled();
  }

  // @return if this node can contain the "first formatted line".
  // https://www.w3.org/TR/CSS22/selector.html#first-formatted-line
  bool CanContainFirstFormattedLine() const {
    DCHECK(GetLayoutBlockFlow());
    return GetLayoutBlockFlow()->CanContainFirstFormattedLine();
  }

  bool UseFirstLineStyle() const;
  void CheckConsistency() const;

  // This function is available after PrepareLayout(), only for SVG <text>.
  const Vector<std::pair<unsigned, SvgCharacterData>>& SvgCharacterDataList()
      const;
  // This function is available after PrepareLayout(), only for SVG <text>.
  const HeapVector<SvgTextContentRange>& SvgTextLengthRangeList() const;
  // This function is available after PrepareLayout(), only for SVG <text>.
  const HeapVector<SvgTextContentRange>& SvgTextPathRangeList() const;

  String ToString() const;

  struct FloatingObject {
    DISALLOW_NEW();

    void Trace(Visitor* visitor) const {
      visitor->Trace(float_style);
      visitor->Trace(style);
    }

    Member<const ComputedStyle> float_style;
    Member<const ComputedStyle> style;
    LayoutUnit float_inline_max_size_with_margin;
  };

  static bool NeedsShapingForTesting(const InlineItem& item);

  // Prepare inline and text content for layout. Must be called before
  // calling the Layout method.
  void PrepareLayoutIfNeeded() const;

 protected:
  FRIEND_TEST_ALL_PREFIXES(InlineNodeTest, SegmentBidiChangeSetsNeedsLayout);

  bool IsPrepareLayoutFinished() const;

  void PrepareLayout(InlineNodeData* previous_data) const;

  void CollectInlines(InlineNodeData*,
                      InlineNodeData* previous_data = nullptr) const;
  const SvgTextChunkOffsets* FindSvgTextChunks(LayoutBlockFlow& block,
                                               InlineNodeData& data) const;
  void SegmentText(InlineNodeData*, InlineNodeData* previous_data) const;
  void SegmentScriptRuns(InlineNodeData*, InlineNodeData* previous_data) const;
  void SegmentFontOrientation(InlineNodeData*) const;
  void SegmentBidiRuns(InlineNodeData*) const;
  void ShapeText(InlineItemsData*,
                 const String* previous_text = nullptr,
                 const InlineItems* previous_items = nullptr,
                 const Font* override_font = nullptr) const;
  void ShapeTextForFirstLineIfNeeded(InlineNodeData*) const;
  void ShapeTextIncludingFirstLine(InlineNodeData* data,
                                   const String* previous_text,
                                   const InlineItems* previous_items) const;
  void AssociateItemsWithInlines(InlineNodeData*) const;
  bool IsNGShapeCacheAllowed(const String&,
                             const Font*,
                             const InlineItems&,
                             ShapeResultSpacing<String>&) const;

  InlineNodeData* MutableData() const {
    return To<LayoutBlockFlow>(box_.Get())->GetInlineNodeData();
  }
  const InlineNodeData& Data() const {
    DCHECK(IsPrepareLayoutFinished());
    DCHECK(!GetLayoutBlockFlow()->NeedsCollectInlines());
    return *To<LayoutBlockFlow>(box_.Get())->GetInlineNodeData();
  }
  // Same as |Data()| but can access even when |NeedsCollectInlines()| is set.
  const InlineNodeData& MaybeDirtyData() const {
    DCHECK(IsPrepareLayoutFinished());
    return *To<LayoutBlockFlow>(box_.Get())->GetInlineNodeData();
  }
  const InlineNodeData& EnsureData() const;

  void AdjustFontForTextCombineUprightAll() const;

  static void ComputeOffsetMapping(LayoutBlockFlow* layout_block_flow,
                                   InlineNodeData* data);

  friend class LineBreakerTest;
};

inline bool InlineNode::IsStickyImagesQuirkForContentSize() const {
  // See https://quirks.spec.whatwg.org/#the-table-cell-width-calculation-quirk
  if (GetDocument().InQuirksMode()) [[unlikely]] {
    const ComputedStyle& style = Style();
    if (style.Display() == EDisplay::kTableCell) [[unlikely]] {
      if (style.LogicalWidth().IsAuto()) {
        return true;
      }
    }
  }
  return false;
}

template <>
struct DowncastTraits<InlineNode> {
  static bool AllowFrom(const LayoutInputNode& node) { return node.IsInline(); }
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::InlineNode::FloatingObject)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_NODE_H_
