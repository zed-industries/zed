/*
 * (C) 1999 Lars Knoll (knoll@kde.org)
 * (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004-2009, 2013 Apple Inc. All rights reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TEXT_H_

#include <iterator>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/memory/scoped_refptr.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/text.h"
#include "third_party/blink/renderer/core/layout/inline/inline_item_span.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/platform/geometry/length_functions.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class AbstractInlineTextBox;
class ContentCaptureManager;
class OffsetMapping;
struct InlineItemsData;
struct InlineItemSpan;
struct TextDiffRange;
struct VariableLengthTransformResult;

// LayoutText is the root class for anything that represents
// a text node (see core/dom/text.h).
//
// This is a common node in the tree so to the limit memory overhead,
// this class inherits directly from LayoutObject.
// Also this class is used by both CSS and SVG layouts so LayoutObject
// was a natural choice.
//
// The actual layout of text is handled by the containing inline
// (LayoutInline) or block (LayoutBlockFlow). They will invoke the Unicode
// Bidirectional Algorithm to break the text into actual lines.
// The result of layout is the line box tree, which represents lines
// on the screen. It is stored into m_firstTextBox and m_lastTextBox.
// To understand how lines are broken by the bidi algorithm, read e.g.
// LayoutBlockFlow::LayoutInlineChildren.
//
// The previous comment applies also for painting. See e.g.
// BlockFlowPainter::paintContents in particular the use of LineBoxListPainter.
class CORE_EXPORT LayoutText : public LayoutObject {
 public:
  // FIXME: If the node argument is not a Text node or the string argument is
  // not the content of the Text node, updating text-transform property
  // doesn't re-transform the string.
  LayoutText(Node*, String);

  void Trace(Visitor*) const override;

  static LayoutText* CreateEmptyAnonymous(Document&, const ComputedStyle*);

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutText";
  }

  bool IsLayoutNGObject() const override {
    NOT_DESTROYED();
    return true;
  }

  bool IsTextFragment() const {
    NOT_DESTROYED();
    return is_text_fragment_;
  }
  virtual bool IsWordBreak() const;

  // Returns a string in the corresponding Text node.
  // Returns a null string for an element-based LayoutText such as LayoutBR
  // and LayoutWordBreak.
  virtual String OriginalText() const;
  // This should not be called for LayoutBR.
  unsigned OriginalTextLength() const;

  bool HasInlineFragments() const final;
  wtf_size_t FirstInlineFragmentItemIndex() const final;
  void ClearFirstInlineFragmentItemIndex() final;
  void SetFirstInlineFragmentItemIndex(wtf_size_t) final;

  // This function returns a string that is the result of applying
  // text-transform and -webkit-text-security to the original text.
  // Whitespace collapsing is not applied.  The length of the string might
  // be different from the original text length.
  const String& TransformedText() const {
    NOT_DESTROYED();
    return text_;
  }
  // Returns the length of transformed text.  Do not use this.  This function
  // is rarely useful, and we can use TransformedText().length().
  unsigned TransformedTextLength() const {
    NOT_DESTROYED();
    return text_.length();
  }

  virtual unsigned TextStartOffset() const {
    NOT_DESTROYED();
    return 0;
  }
  virtual String PlainText() const;

  // Returns true if text-transform or -webkit-text-security changes the text
  // length.
  bool HasVariableLengthTransform() const {
    NOT_DESTROYED();
    return has_variable_length_transform_;
  }
  VariableLengthTransformResult GetVariableLengthTransformResult() const;
  void ClearHasVariableLengthTransform();

  // Returns first letter part of |LayoutTextFragment|.
  virtual LayoutText* GetFirstLetterPart() const {
    NOT_DESTROYED();
    return nullptr;
  }

  void QuadsInAncestorInternal(Vector<gfx::QuadF>&,
                               const LayoutBoxModelObject* ancestor,
                               MapCoordinatesFlags) const final;
  void AbsoluteQuadsForRange(Vector<gfx::QuadF>&,
                             unsigned start_offset = 0,
                             unsigned end_offset = INT_MAX) const;
  gfx::RectF LocalBoundingBoxRectForAccessibility() const final;

  std::optional<bool> IgnoreWhitespaceForAccessibility() const {
    NOT_DESTROYED();
    if (has_cached_ignore_whitespace_for_accessibility_) {
      return static_cast<bool>(ignore_whitespace_for_accessibility_);
    }
    return std::nullopt;
  }

  void SetIgnoreWhitespaceForAccessibility(bool value) const {
    NOT_DESTROYED();
    ignore_whitespace_for_accessibility_ = value;
    has_cached_ignore_whitespace_for_accessibility_ = true;
  }

  enum ClippingOption { kNoClipping, kClipToEllipsis };
  void LocalQuadsInFlippedBlocksDirection(Vector<gfx::QuadF>&,
                                          ClippingOption = kNoClipping) const;

  PositionWithAffinity PositionForPoint(const PhysicalOffset&) const override;

  bool HasEmptyText() const {
    NOT_DESTROYED();
    return text_.empty();
  }

  // Get characters after whitespace collapsing was applied. Returns 0 if there
  // were no characters left. If whitespace collapsing is disabled (i.e.
  // white-space: pre), returns characters without whitespace collapsing.
  UChar32 FirstCharacterAfterWhitespaceCollapsing() const;
  UChar32 LastCharacterAfterWhitespaceCollapsing() const;

  virtual PhysicalRect PhysicalLinesBoundingBox() const;

  // Returns the bounding box of visual overflow rects of all line boxes,
  // in containing block's physical coordinates with flipped blocks direction.
  PhysicalRect VisualOverflowRect() const;

  void InvalidateVisualOverflow();

  PhysicalOffset FirstLineBoxTopLeft() const;

  void SetTextIfNeeded(String);
  void ForceSetText(String);
  void SetTextWithOffset(String, const TextDiffRange&);
  void SetTextInternal(String);

  // Apply text-transform and -webkit-text-security to OriginalText(), and
  // store its result to text_.
  virtual void TransformAndSecureOriginalText();
  // Apply text-transform and -webkit-text-security to the specified string.
  String TransformAndSecureText(const String& original,
                                TextOffsetMap& offset_map) const;

  PhysicalRect LocalSelectionVisualRect() const final;
  PhysicalRect LocalCaretRect(int caret_offset) const override;

  // Compute the rect and offset of text boxes for this LayoutText.
  struct TextBoxInfo {
    PhysicalRect local_rect;
    unsigned dom_start_offset;
    unsigned dom_length;
  };
  Vector<TextBoxInfo> GetTextBoxInfo() const;

  // Returns the Position in DOM that corresponds to the given offset in the
  // original text.
  virtual Position PositionForCaretOffset(unsigned) const;

  // Returns the offset in the original text that corresponds to the given
  // position in DOM; Returns nullopt is the position is not in this LayoutText.
  virtual std::optional<unsigned> CaretOffsetForPosition(const Position&) const;

  // Returns true if the offset (0-based in the original text) is next to a
  // non-collapsed non-linebreak character, or before a forced linebreak (<br>,
  // or segment break in node with style white-space: pre/pre-line/pre-wrap).
  // TODO(editing-dev): The behavior is introduced by crrev.com/e3eb4e in
  // InlineTextBox::ContainsCaretOffset(). Try to understand it.
  bool ContainsCaretOffset(int) const;

  // Return true if the offset (0-based in the original text) is before/after a
  // non-collapsed character in this LayoutText, respectively.
  bool IsBeforeNonCollapsedCharacter(unsigned) const;
  bool IsAfterNonCollapsedCharacter(unsigned) const;

  virtual int CaretMinOffset() const;
  virtual int CaretMaxOffset() const;
  unsigned ResolvedTextLength() const;

  // True if any character remains after CSS white-space collapsing.
  bool HasNonCollapsedText() const;

  bool IsSecure() const {
    NOT_DESTROYED();
    return StyleRef().TextSecurity() != ETextSecurity::kNone;
  }

  bool HasTextTransform() const {
    NOT_DESTROYED();
    return StyleRef().TextTransform() != ETextTransform::kNone;
  }

  void MomentarilyRevealLastTypedCharacter(
      unsigned last_typed_character_offset);

  bool IsAllCollapsibleWhitespace() const;

  void RemoveAndDestroyTextBoxes();

  AbstractInlineTextBox* FirstAbstractInlineTextBox();

  bool HasAbstractInlineTextBox() const {
    NOT_DESTROYED();
    return has_abstract_inline_text_box_;
  }

  void SetHasAbstractInlineTextBox() {
    NOT_DESTROYED();
    has_abstract_inline_text_box_ = true;
  }

  PhysicalRect DebugRect() const override;

  void AutosizingMultiplerChanged() {
    NOT_DESTROYED();
    // The font size is changing, so we need to make sure to rebuild everything.
    valid_ng_items_ = false;
    SetNeedsCollectInlines();
  }

  virtual UChar PreviousCharacter() const;

  // Returns the OffsetMapping object when the current text is laid out with
  // LayoutNG.
  // Note that the text can be in legacy layout even when LayoutNG is enabled,
  // so we can't simply check the RuntimeEnabledFeature.
  const OffsetMapping* GetOffsetMapping() const;

  // Map DOM offset to LayoutNG text content offset.
  // Returns false if all characters in this LayoutText are collapsed.
  bool MapDOMOffsetToTextContentOffset(const OffsetMapping&,
                                       unsigned* start,
                                       unsigned* end) const;
  DOMNodeId EnsureNodeId();
  bool HasNodeId() const {
    NOT_DESTROYED();
    return node_id_ != kInvalidDOMNodeId;
  }

  void SetInlineItems(InlineItemsData* data, wtf_size_t begin, wtf_size_t size);
  void ClearInlineItems();
  bool HasValidInlineItems() const {
    NOT_DESTROYED();
    return valid_ng_items_;
  }
  const InlineItemSpan& InlineItems() const;
  // Inline items depends on context. It needs to be invalidated not only when
  // it was inserted/changed but also it was moved.
  void InvalidateInlineItems() {
    NOT_DESTROYED();
    valid_ng_items_ = false;
  }

  bool HasNoControlItems() const {
    NOT_DESTROYED();
    return has_no_control_items_;
  }
  void SetHasNoControlItems() {
    NOT_DESTROYED();
    has_no_control_items_ = true;
  }
  void ClearHasNoControlItems() {
    NOT_DESTROYED();
    has_no_control_items_ = false;
  }
  bool HasBidiControlInlineItems() const {
    NOT_DESTROYED();
    return has_bidi_control_items_;
  }
  void SetHasBidiControlInlineItems() {
    NOT_DESTROYED();
    has_bidi_control_items_ = true;
  }
  void ClearHasBidiControlInlineItems() {
    NOT_DESTROYED();
    has_bidi_control_items_ = false;
  }

  const InlineItemSpan* GetInlineItems() const {
    NOT_DESTROYED();
    return &inline_items_;
  }
  InlineItemSpan* GetInlineItems() {
    NOT_DESTROYED();
    return &inline_items_;
  }

  void InvalidateSubtreeLayoutForFontUpdates() override;

  void DetachAxHooksIfNeeded();

  // Returns the logical location of the first line box, and the logical height
  // of the LayoutText.
  void LogicalStartingPointAndHeight(LogicalOffset& logical_starting_point,
                                     LayoutUnit& logical_height) const;

  // For LayoutShiftTracker. Saves the value of LogicalStartingPoint() value
  // during the previous paint invalidation.
  LogicalOffset PreviousLogicalStartingPoint() const {
    NOT_DESTROYED();
    return previous_logical_starting_point_;
  }
  // This is const because LayoutObjects are const for paint invalidation.
  void SetPreviousLogicalStartingPoint(const LogicalOffset& point) const {
    NOT_DESTROYED();
    DCHECK_EQ(GetDocument().Lifecycle().GetState(),
              DocumentLifecycle::kInPrePaint);
    previous_logical_starting_point_ = point;
  }
  static LogicalOffset UninitializedLogicalStartingPoint() {
    return {LayoutUnit::Max(), LayoutUnit::Max()};
  }

#if DCHECK_IS_ON()
  void RecalcVisualOverflow() override;
#endif

 protected:
  void WillBeDestroyed() override;

  void StyleWillChange(StyleDifference, const ComputedStyle&) final;

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  void InLayoutNGInlineFormattingContextWillChange(bool) final;

  virtual void TextDidChange();

  void InvalidatePaint(const PaintInvalidatorContext&) const final;
  void InvalidateDisplayItemClients(PaintInvalidationReason) const final;

  bool CanBeSelectionLeafInternal() const final {
    NOT_DESTROYED();
    return true;
  }

  // Override |LayoutObject| implementation to invalidate |LayoutTextCombine|.
  // Note: This isn't a virtual function.
  void SetNeedsLayoutAndIntrinsicWidthsRecalcAndFullPaintInvalidation(
      LayoutInvalidationReasonForTracing reason);

 private:
  void TextDidChangeWithoutInvalidation();

  // PhysicalRectCollector should be like a function:
  // void (const PhysicalRect&).
  template <typename PhysicalRectCollector>
  void CollectLineBoxRects(const PhysicalRectCollector&,
                           ClippingOption option = kNoClipping) const;

  // See the class comment as to why we shouldn't call this function directly.
  void Paint(const PaintInfo&) const final {
    NOT_DESTROYED();
    NOTREACHED();
  }
  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation&,
                   const PhysicalOffset&,
                   HitTestPhase) final {
    NOT_DESTROYED();
    NOTREACHED();
  }

  void DeleteTextBoxes();

  std::pair<String, TextOffsetMap> SecureText(const String& plain,
                                              UChar mask) const;
  void SetVariableLengthTransformResult(wtf_size_t original_length,
                                        const TextOffsetMap& offset_map);

  bool IsText() const final {
    NOT_DESTROYED();
    return true;
  }

  const DisplayItemClient* GetSelectionDisplayItemClient() const final;

  // We put the bitfield first to minimize padding on 64-bit.
 protected:
  // Whether the InlineItems associated with this object are valid. Set after
  // layout and cleared whenever the LayoutText is modified.
  unsigned valid_ng_items_ : 1;

  // Caches if there are no `IsControlItemCharacter()` characters. Set in
  // `InlineItemsBuilder` only for preserved whitespace.
  unsigned has_no_control_items_ : 1 = false;

  // Whether there is any BidiControl type InlineItem associated with this
  // object. Set after layout when associating items.
  unsigned has_bidi_control_items_ : 1;

  unsigned is_text_fragment_ : 1;

 private:
  ContentCaptureManager* GetOrResetContentCaptureManager();
  void DetachAxHooks();
  void ClearBlockFlowCachedData();

  virtual unsigned NonCollapsedCaretMaxOffset() const;

  // Used for LayoutNG with accessibility. True if inline fragments are
  // associated to |AbstractInlineTextBox|.
  unsigned has_abstract_inline_text_box_ : 1;

  unsigned has_variable_length_transform_ : 1;

  // If the entire text is whitespace, the node may be ignored for
  // accessibility. ignore_whitespace_for_accessibility_ caches whether
  // this node is such an ignored node. Only meaningful if
  // has_cached_ignore_whitespace_for_accessibility_ is true.
  mutable unsigned ignore_whitespace_for_accessibility_ : 1 = 0;
  mutable unsigned has_cached_ignore_whitespace_for_accessibility_ : 1 = 0;

  DOMNodeId node_id_ = kInvalidDOMNodeId;

  String text_;

  // This is mutable for paint invalidation.
  mutable LogicalOffset previous_logical_starting_point_ =
      UninitializedLogicalStartingPoint();

  InlineItemSpan inline_items_;

  // The index of the first fragment item associated with this object in
  // |FragmentItems::Items()|. Zero means there are no such item.
  // Valid only when IsInLayoutNGInlineFormattingContext().
  wtf_size_t first_fragment_item_index_ = 0u;
};

inline wtf_size_t LayoutText::FirstInlineFragmentItemIndex() const {
  NOT_DESTROYED();
  if (!IsInLayoutNGInlineFormattingContext())
    return 0u;
  return first_fragment_item_index_;
}

inline void LayoutText::DetachAxHooksIfNeeded() {
  NOT_DESTROYED();
  if (has_abstract_inline_text_box_) [[unlikely]] {
    DetachAxHooks();
  }
  if (!IsInLayoutNGInlineFormattingContext()) {
    return;
  }

  ClearBlockFlowCachedData();
}

template <>
struct DowncastTraits<LayoutText> {
  static bool AllowFrom(const LayoutObject& object) { return object.IsText(); }
};

inline LayoutText* Text::GetLayoutObject() const {
  return To<LayoutText>(CharacterData::GetLayoutObject());
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TEXT_H_
