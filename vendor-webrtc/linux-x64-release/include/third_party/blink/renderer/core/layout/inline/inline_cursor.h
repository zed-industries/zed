// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_CURSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_CURSOR_H_

#include <unicode/ubidi.h>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/layout/inline/fragment_item.h"
#include "third_party/blink/renderer/core/layout/inline/fragment_items.h"
#include "third_party/blink/renderer/core/layout/style_variant.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"

namespace blink {

class ComputedStyle;
class DisplayItemClient;
class FragmentItem;
class FragmentItems;
class InlineBackwardCursor;
class InlineBreakToken;
class InlineCursor;
class InlinePaintContext;
class LayoutBlockFlow;
class LayoutInline;
class LayoutObject;
class Node;
class PhysicalBoxFragment;
class ShapeResultView;
struct LayoutSelectionStatus;
struct PhysicalRect;
struct PhysicalSize;

// Represents a position of |InlineCursor|. This class:
// 1. Provides properties for the current position.
// 2. Allows to save |Current()|, and can move back later. Moving to |Position|
// is faster than moving to |FragmentItem|.
class CORE_EXPORT InlineCursorPosition {
  STACK_ALLOCATED();

 public:
  using ItemsSpan = FragmentItems::Span;

  const FragmentItem* Item() const { return item_; }
  const FragmentItem* operator->() const { return item_; }
  const FragmentItem& operator*() const { return *item_; }

  explicit operator bool() const { return item_; }

  bool operator==(const InlineCursorPosition& other) const {
    return item_ == other.item_;
  }
  bool operator!=(const InlineCursorPosition& other) const {
    return !operator==(other);
  }

  // True if the current position is a text. It is error to call at end.
  bool IsText() const { return item_->IsText(); }

  // True if the current position is a generatd text. It is error to call at
  // end. This includes both style-generated (e.g., `content` property, see
  // |IsStyleGenerated()|) and layout-generated (hyphens and ellipsis, see
  // |IsLayoutGeneratedText()|.)
  bool IsGeneratedText() const { return item_->IsGeneratedText(); }

  // True if fragment is layout-generated (hyphens and ellipsis.)
  bool IsLayoutGeneratedText() const {
    return item_->Type() == FragmentItem::kGeneratedText;
  }

  // True if the current position is a line break. It is error to call at end.
  bool IsLineBreak() const { return IsText() && item_->IsLineBreak(); }

  // True if the current position is an ellipsis. It is error to call at end.
  bool IsEllipsis() const { return item_->IsEllipsis(); }

  // True if the current position is a line box. It is error to call at end.
  bool IsLineBox() const { return item_->Type() == FragmentItem::kLine; }

  // True if the current position is an empty line box. It is error to call
  // other then line box.
  bool IsEmptyLineBox() const { return item_->IsEmptyLineBox(); }

  // True if the current position is an inline box. It is error to call at end.
  bool IsInlineBox() const { return item_->IsInlineBox(); }

  // True if the current position is an atomic inline. It is error to call at
  // end.
  bool IsAtomicInline() const { return item_->IsAtomicInline(); }

  // True if the current position is a list marker.
  bool IsListMarker() const { return item_->IsListMarker(); }

  // True if the current position is a box for "float"
  bool IsFloating() const { return item_->IsFloating(); }

  // True if the current position is hidden for paint. It is error to call at
  // end.
  bool IsHiddenForPaint() const { return item_->IsHiddenForPaint(); }

  // |ComputedStyle| and related functions.
  StyleVariant GetStyleVariant() const { return item_->GetStyleVariant(); }
  bool UsesFirstLineStyle() const {
    return blink::UsesFirstLineStyle(GetStyleVariant());
  }
  const ComputedStyle& Style() const { return item_->Style(); }

  // Functions to get corresponding objects for this position.
  const PhysicalBoxFragment* BoxFragment() const {
    return item_->BoxFragment();
  }
  const LayoutObject* GetLayoutObject() const {
    return item_->GetLayoutObject();
  }
  LayoutObject* GetMutableLayoutObject() const {
    return item_->GetMutableLayoutObject();
  }
  const Node* GetNode() const;
  const DisplayItemClient* GetDisplayItemClient() const {
    return item_->GetDisplayItemClient();
  }
  const DisplayItemClient* GetSelectionDisplayItemClient() const;
  wtf_size_t FragmentId() const { return item_->FragmentId(); }

  // True if fragment at the current position can have children.
  bool CanHaveChildren() const;

  // True if fragment at the current position has children.
  bool HasChildren() const;

  // Returns break token for line box. It is error to call other than line box.
  const InlineBreakToken* GetInlineBreakToken() const {
    return item_->GetInlineBreakToken();
  }

  // The offset relative to the root of the inline formatting context.
  const PhysicalRect RectInContainerFragment() const {
    return item_->RectInContainerFragment();
  }
  gfx::RectF ObjectBoundingBox(const InlineCursor& cursor) const;
  const PhysicalOffset OffsetInContainerFragment() const {
    return item_->OffsetInContainerFragment();
  }
  const PhysicalSize Size() const { return item_->Size(); }

  // InkOverflow of itself, including contents if they contribute to the ink
  // overflow of this object (e.g. when not clipped,) in the local coordinate.
  const PhysicalRect InkOverflowRect() const {
    return item_->InkOverflowRect();
  }
  const PhysicalRect SelfInkOverflowRect() const {
    return item_->SelfInkOverflowRect();
  }

  void RecalcInkOverflow(const InlineCursor& cursor,
                         InlinePaintContext* inline_context) const;

  // Returns start/end of offset in text content of current text fragment.
  // It is error when this cursor doesn't point to text fragment.
  TextOffsetRange TextOffset() const { return item_->TextOffset(); }
  wtf_size_t TextStartOffset() const { return TextOffset().start; }
  wtf_size_t TextEndOffset() const { return TextOffset().end; }

  // Returns text of the current position. It is error to call other than
  // text.
  StringView Text(const InlineCursor& cursor) const;

  // Returns |ShapeResultView| of the current position. It is error to call
  // other than text.
  const ShapeResultView* TextShapeResult() const {
    return item_->TextShapeResult();
  }

  // Returns bidi level of current position. It is error to call other than
  // text and atomic inline. It is also error to call |IsGeneratedTextType()|.
  UBiDiLevel BidiLevel() const;
  // Returns text direction of current text or atomic inline. It is error to
  // call at other than text or atomic inline. Note: <span> doesn't have
  // reserved direction.
  TextDirection ResolvedDirection() const { return item_->ResolvedDirection(); }
  // Returns text direction of current line. It is error to call at other than
  // line.
  TextDirection BaseDirection() const;

  TextDirection ResolvedOrBaseDirection() const {
    return IsLineBox() ? BaseDirection() : ResolvedDirection();
  }

  // True if the current position is text or atomic inline box.
  // Note: Because of this function is used for caret rect, hit testing, etc,
  // this function returns false for hidden for paint, text overflow ellipsis,
  // and line break hyphen.
  bool IsInlineLeaf() const;

  // True if current position has soft wrap to next line. It is error to call
  // other than line.
  bool HasSoftWrapToNextLine() const;

  // LogicalRect/PhysicalRect conversions
  // |logical_rect| and |physical_rect| are converted with |Size()| as
  // "outer size".
  LogicalRect ConvertChildToLogical(const PhysicalRect& physical_rect) const;
  PhysicalRect ConvertChildToPhysical(const LogicalRect& logical_rect) const;

 private:
  void Set(const ItemsSpan::iterator& iter) {
    item_iter_ = iter;
    item_ = &*iter;
  }

  void Clear() {
    item_ = nullptr;
  }

  // True if current position is part of culled inline box |layout_inline|.
  bool IsPartOfCulledInlineBox(const LayoutInline& layout_inline) const;

  const FragmentItem* item_ = nullptr;
  ItemsSpan::iterator item_iter_;

  friend class InlineBackwardCursor;
  friend class InlineCursor;
};

// This class traverses fragments in an inline formatting context.
//
// When constructed, the initial position is empty. Call |MoveToNext()| to move
// to the first fragment.
class CORE_EXPORT InlineCursor {
  STACK_ALLOCATED();

 public:
  using ItemsSpan = FragmentItems::Span;

  explicit InlineCursor(const LayoutBlockFlow& block_flow);
  explicit InlineCursor(const PhysicalBoxFragment& box_fragment);
  InlineCursor(const PhysicalBoxFragment& box_fragment,
               const FragmentItems& items);
  explicit InlineCursor(const InlineBackwardCursor& backward_cursor);
  InlineCursor(const InlineCursor& other) = default;
  InlineCursor& operator=(const InlineCursor& other) = default;

  // Creates an |InlineCursor| without the root. Even when callers don't know
  // the root of the inline formatting context, this cursor can |MoveTo()|
  // specific |LayoutObject|.
  InlineCursor() = default;

  bool operator==(const InlineCursor& other) const;
  bool operator!=(const InlineCursor& other) const {
    return !operator==(other);
  }

  // True if this cursor has the root to traverse. Only the default constructor
  // creates a cursor without the root.
  bool HasRoot() const { return fragment_items_; }

  const FragmentItems& Items() const {
    DCHECK(fragment_items_);
    return *fragment_items_;
  }

  // Returns the |PhysicalBoxFragment| that owns |Items|.
  const PhysicalBoxFragment& ContainerFragment() const {
    DCHECK(root_box_fragment_);
    return *root_box_fragment_;
  }

  // Return the index of the current physical box fragment of the containing
  // block. An inline formatting context may be block fragmented.
  wtf_size_t ContainerFragmentIndex() const { return fragment_index_; }

  // Returns the |LayoutBlockFlow| containing this cursor.
  // When |this| is a column box, returns the multicol container.
  const LayoutBlockFlow* GetLayoutBlockFlow() const;

  //
  // Functions to query the current position.
  //
  const InlineCursorPosition& Current() const { return current_; }

  // Returns true if cursor is out of fragment tree, e.g. before first fragment
  // or after last fragment in tree.
  bool IsNull() const { return !Current(); }
  bool IsNotNull() const { return !!Current(); }
  explicit operator bool() const { return !!Current(); }

  // True if |Current()| is at the first fragment. See |MoveToFirst()|.
  bool IsAtFirst() const;

  // Returns a new |InlineCursor| whose root is the current item. The returned
  // cursor can traverse descendants of the current item. If the current item
  // has no children, returns an empty cursor.
  InlineCursor CursorForDescendants() const;

  // Returns a new |InlineCursor| whose root is containing block or multicol
  // container for traversing fragmentainers in root.
  InlineCursor CursorForMovingAcrossFragmentainer() const;

  // If |this| is created by |CursorForDescendants()| to traverse parts of an
  // inline formatting context, expand the traversable range to the containing
  // |LayoutBlockFlow|. Does nothing if |this| is for an inline formatting
  // context.
  void ExpandRootToContainingBlock();

  // True if the current position is before soft line break. It is error to call
  // at end.
  bool IsBeforeSoftLineBreak() const;

  // |Current*| functions return an object for the current position.
  const FragmentItem* CurrentItem() const { return Current().Item(); }

  // Returns text of the current position. It is error to call other than
  // text.
  StringView CurrentText() const { return Current().Text(*this); }

  // The layout box of text in (start, end) range in local coordinate.
  // Start and end offsets must be between |CurrentTextStartOffset()| and
  // |CurrentTextEndOffset()|. It is error to call other than text.
  PhysicalRect CurrentLocalRect(unsigned start_offset,
                                unsigned end_offset) const;
  PhysicalRect CurrentLocalSelectionRectForText(
      const LayoutSelectionStatus& selection_status) const;
  PhysicalRect CurrentLocalSelectionRectForReplaced() const;

  // Return a rectangle (or just an offset) relatively to containing
  // LayoutBlockFlow, as if all the container fragments were stitched together
  // in the block direction (aka. "flow thread coordinate space").
  //
  // Example:
  // <div style="columns:2; orphans:1; widows:1; width:20px; line-height:20px;">
  //   <div id="container">line1 line2 line3 line4 line5 line6</div>
  // </div>
  //
  // The text will end up on six lines. The first three lines will end up in the
  // first column, and the last three lines will end up in the second column. So
  // we get two box fragments generated for #container - one for each column.
  //
  // The offsets returned from these methods will be
  // (OffsetInContainerFragment() values in parentheses):
  //
  // line1: 0,0   (0,0)
  // line2: 0,20  (0,20)
  // line3: 0,40  (0,40)
  // line4: 0,60  (0,0)
  // line5: 0,80  (0,20)
  // line6: 0,100 (0,40)
  //
  // We need this functionality, because we're still using the legacy layout
  // engine to calculate offsets relatively to some ancestor.
  PhysicalRect CurrentRectInBlockFlow() const;
  PhysicalOffset CurrentOffsetInBlockFlow() const {
    DCHECK_EQ(Current().OffsetInContainerFragment(),
              Current().RectInContainerFragment().offset);
    return CurrentRectInBlockFlow().offset;
  }

  // Returns inline position relative to current text fragment for
  // |LocalCaretRect|. It is error to call other than text.
  LayoutUnit CaretInlinePositionForOffset(unsigned offset) const;

  // Converts the given point, relative to the fragment itself, into a position
  // in DOM tree within the range of |this|. This variation ignores the inline
  // offset, and snaps to the nearest line in the block direction.
  PositionWithAffinity PositionForPointInInlineFormattingContext(
      const PhysicalOffset& point,
      const PhysicalBoxFragment& container);
  // Find the |Position| in the line box |Current()| points to. This variation
  // ignores the block offset, and snaps to the nearest item in inline
  // direction.
  PositionWithAffinity PositionForPointInInlineBox(
      const PhysicalOffset& point) const;

  // Returns |PositionWithAffinity| in current position at x-coordinate of
  // |point_in_container| for horizontal writing mode, or y-coordinate of
  // |point_in_container| for vertical writing mode.
  // Note: Even if |point_in_container| is outside of an item of current
  // position, this function returns boundary position of an item.
  // Note: This function is used for locating caret at same x/y-coordinate as
  // previous caret after line up/down.
  PositionWithAffinity PositionForPointInChild(
      const PhysicalOffset& point_in_container) const;

  // Returns |PositionWithAffinity| in current text at |text_offset|
  PositionWithAffinity PositionForPointInText(unsigned text_offset) const;

  // Returns first/last position of |this| line. |this| should be line box.
  PositionWithAffinity PositionForStartOfLine() const;
  PositionWithAffinity PositionForEndOfLine() const;

  //
  // Functions to move the current position.
  //
  void MoveTo(const InlineCursorPosition& position);

  // Move the current position at |fragment_item|. |this| cursor must have
  // root.
  void MoveTo(const FragmentItem& fragment_item);

  // Move the current position at |cursor|. Unlinke copy constrcutr, this
  // function doesn't copy root. Note: The current position in |cursor|
  // should be part of |this| cursor.
  void MoveTo(const InlineCursor& cursor);

  // Move to the parent box or line box.
  void MoveToParent();

  // Move to containing line box. It is error if the current position is line.
  void MoveToContainingLine();

  // Move to first child of current container box. If the current position is
  // at fragment without children, this cursor points nothing.
  // See also |TryMoveToFirstChild()|.
  void MoveToFirstChild();

  // Move to the first line.
  void MoveToFirstLine();

  // Move to first logical leaf of current line box. If current line box has
  // no children, cursor becomes null.
  void MoveToFirstLogicalLeaf();

  // Move to first leaf from current position.
  // Unlike |MoveToFirstLogicalLeaf()|, this function ignores pseudo node and
  // stops at non-truncated text.
  void MoveToFirstNonPseudoLeaf();

  // Move to last child of current container box. If the current position is
  // at fragment without children, this cursor points nothing.
  // See also |TryMoveToFirstChild()|.
  void MoveToLastChild();

  // Move to the last line item. If there are no line items, the cursor becomes
  // null.
  void MoveToLastLine();

  // Move to last logical leaf of current line box. If current line box has
  // no children, cursor becomes null.
  void MoveToLastLogicalLeaf();

  // Move to last leaf from current position.
  // Unlike |MoveToLastLogicalLeaf()|, this
  // function ignores pseudo node and stops at non-truncated text.
  void MoveToLastNonPseudoLeaf();

  // Move the current position to the next fragment in pre-order DFS. When
  // the current position is at last fragment, this cursor points nothing.
  void MoveToNext();

  // Move the current position to next line. It is error to call other than line
  // box.
  void MoveToNextLine();
  void MoveToNextLineIncludingFragmentainer();

  // Same as |MoveToNext| except that this skips children even if they exist.
  void MoveToNextSkippingChildren();

  // Move the current to next/previous inline leaf.
  void MoveToNextInlineLeaf();
  void MoveToNextInlineLeafIgnoringLineBreak();
  void MoveToPreviousInlineLeaf();
  void MoveToPreviousInlineLeafIgnoringLineBreak();

  // Move the current position to next/previous inline leaf item on line.
  // Note: If the current position isn't leaf item, this function moves the
  // current position to leaf item then moves to next/previous leaf item. This
  // behavior doesn't match |MoveTo{Next,Previous}InlineLeaf()|, but AX requires
  // this. See AccessibilityLayoutTest.NextOnLine
  void MoveToNextInlineLeafOnLine();
  void MoveToPreviousInlineLeafOnLine();

  // Move the cursor position to previous fragment in pre-order DFS.
  void MoveToPrevious();

  // Move to the previous fragmentainer.
  // Valid when |CanMoveAcrossFragmentainer|.
  void MoveToPreviousFragmentainer();

  // Same as |MoveToPrevious|, except this moves to the previous fragmentainer
  // if |Current| is at the end of a fragmentainer.
  void MoveToPreviousIncludingFragmentainer();

  // Move the current position to previous line. It is error to call other than
  // line box.
  void MoveToPreviousLine();

  // Returns true if the current position moves to first child.
  bool TryMoveToFirstChild();

  // Returns true if the current position moves to first inline leaf child.
  bool TryMoveToFirstInlineLeafChild();

  // Returns true if the current position moves to last child.
  bool TryMoveToLastChild();

  //
  // Moving across fragmentainers.
  //
  // When rooted at |LayoutBlockFlow|, |this| can move the current position
  // across fragmentainers. Other root objects (e.g. |FragmentItems|) can
  // contain only one fragmentainer that such cursors cannot move to different
  // fragmentainers. See |CanMoveAcrossFragmentainer()|.
  //
  // However, |MoveToNext| etc. does not move the current position across
  // fragmentainers. Use following functions when moving to different
  // fragmentainers.

  bool IsBlockFragmented() const { return max_fragment_index_ > 0; }

  // Move to the first item of the first fragmentainer.
  void MoveToFirstIncludingFragmentainer();

  // Move to the next fragmentainer. Valid when |CanMoveAcrossFragmentainer|.
  void MoveToNextFragmentainer();

  // Same as |MoveToNext|, except this moves to the next fragmentainer if
  // |Current| is at the end of a fragmentainer.
  void MoveToNextIncludingFragmentainer();

  //
  // Functions to enumerate fragments for a |LayoutObject|.
  //

  // Move to the first `FragmentItem` associated to the `layout_object`.
  // This cursor points nothing if any of the following conditions are true:
  // * The `layout_object` has no associated fragments.
  // * The `layout_object.IsOutOfFlowPositioned()`.
  void MoveTo(const LayoutObject& layout_object);

  // Same as |MoveTo|, except that this enumerates fragments for descendants
  // if |layout_object| is a culled inline.
  //
  // Note, for a culled inline, fragments may not be in the visual order in
  // the inline direction if RTL or mixed bidi for a performance reason.
  void MoveToIncludingCulledInline(const LayoutObject& layout_object);

  // Move the current position to next fragment on same layout object.
  void MoveToNextForSameLayoutObject();

  // Move the current position to the last fragment on same layout object.
  void MoveToLastForSameLayoutObject();

  // Move the current position to the last fragment on the same layout object,
  // in visual order. This is the same as |MoveToLastForSameLayoutObject|,
  // except for culled inlines.
  //
  // Note that this method will only consider fragments reachable through
  // |MoveToNextForSameLayoutObject|.
  void MoveToVisualLastForSameLayoutObject();

  // Move the current position to the first fragment on the same layout object,
  // in visual order.
  //
  // Note that this method will only consider fragments reachable through
  // |MoveToNextForSameLayoutObject|. For non-culled inlines, this means this
  // method is a no-op.
  void MoveToVisualFirstForSameLayoutObject();

#if DCHECK_IS_ON()
  void CheckValid(const InlineCursorPosition& position) const;
#else
  void CheckValid(const InlineCursorPosition&) const {}
#endif

 private:
  InlineCursor(const PhysicalBoxFragment& box_fragment,
               const FragmentItems& fragment_items,
               ItemsSpan items);

  // Returns true if |this| is only for a part of an inline formatting context;
  // in other words, if |this| is created by |CursorForDescendants|.
  bool IsDescendantsCursor() const {
    if (fragment_items_)
      return !fragment_items_->Equals(items_);
    return false;
  }
  bool CanMoveAcrossFragmentainer() const {
    return root_block_flow_ && HasRoot() && !IsDescendantsCursor();
  }

  // True if the current position is a last line in inline block. It is error
  // to call at end or the current position is not line.
  bool IsLastLineInInlineBlock() const;

  // Index conversions for |IsDescendantsCursor()|.
  wtf_size_t SpanBeginItemIndex() const;
  wtf_size_t SpanIndexFromItemIndex(unsigned index) const;

  // Make the current position points nothing, e.g. cursor moves over start/end
  // fragment, cursor moves to first/last child to parent has no children.
  void MakeNull() { current_.Clear(); }

  // Move the cursor position to the first fragment in tree.
  void MoveToFirst();

  void SetRoot(const PhysicalBoxFragment& box_fragment,
               const FragmentItems& items);
  void SetRoot(const PhysicalBoxFragment& box_fragment,
               const FragmentItems& fragment_items,
               ItemsSpan items);
  void SetRoot(const LayoutBlockFlow& block_flow);
  bool SetRoot(const LayoutBlockFlow& block_flow, wtf_size_t fragment_index);

  bool TrySetRootFragmentItems();

  // Returns true and move to current position to |fragment_item|, otherwise
  // returns false.
  bool TryMoveTo(const FragmentItem& fragment_item);

  void MoveToItem(const ItemsSpan::iterator& iter);

  void SlowMoveToFirstFor(const LayoutObject& layout_object);
  void SlowMoveToNextForSameLayoutObject(const LayoutObject& layout_object);
  void SlowMoveToForIfNeeded(const LayoutObject& layout_object);

  // |MoveToNextForSameLayoutObject| that doesn't check |culled_inline_|.
  void MoveToNextForSameLayoutObjectExceptCulledInline();

  // Used for |MoveToVisualLastForSameLayoutObject| and
  // |MoveToVisualFirstForSameLayoutObject|.
  void MoveToVisualFirstOrLastForCulledInline(bool last);

  // Returns text_offset for the last position of caret in current line
  // including the case of empty line.
  wtf_size_t GetTextOffsetForEndOfLine(InlineCursor& cursor) const;

  // A helper class to enumerate |LayoutObject|s that contribute to a culled
  // inline.
  class CulledInlineTraversal {
    STACK_ALLOCATED();

   public:
    CulledInlineTraversal() = default;

    const LayoutInline* GetLayoutInline() const { return layout_inline_; }

    explicit operator bool() const { return layout_inline_; }
    void Reset() { layout_inline_ = nullptr; }

    bool UseFragmentTree() const { return use_fragment_tree_; }
    void SetUseFragmentTree(const LayoutInline& layout_inline);

    // Returns first/next |LayoutObject| that contribute to |layout_inline|.
    const LayoutObject* MoveToFirstFor(const LayoutInline& layout_inline);
    const LayoutObject* MoveToNext();

   private:
    const LayoutObject* Find(const LayoutObject* child) const;

    const LayoutObject* current_object_ = nullptr;
    const LayoutInline* layout_inline_ = nullptr;
    bool use_fragment_tree_ = false;
  };

  void MoveToFirstForCulledInline(const LayoutInline& layout_inline);
  void MoveToNextForCulledInline();
  void MoveToNextCulledInlineDescendantIfNeeded();

  void ResetFragmentIndex();
  void DecrementFragmentIndex();
  void IncrementFragmentIndex();

  wtf_size_t ToSpanIndex(const ItemsSpan::iterator& iter) const {
    return base::checked_cast<wtf_size_t>(std::distance(items_.begin(), iter));
  }

  InlineCursorPosition current_;

  ItemsSpan items_;
  const FragmentItems* fragment_items_ = nullptr;
  const PhysicalBoxFragment* root_box_fragment_ = nullptr;

  CulledInlineTraversal culled_inline_;

  // Used to traverse multiple |FragmentItems| when block fragmented.
  const LayoutBlockFlow* root_block_flow_ = nullptr;

  // Block-size consumed in previous container fragments, when an
  // inline formatting context is block-fragmented.
  LayoutUnit previously_consumed_block_size_;

  wtf_size_t fragment_index_ = 0;
  wtf_size_t max_fragment_index_ = 0;

  friend class InlineBackwardCursor;
};

// This class provides the |MoveToPreviousSibling| functionality, but as a
// separate class because it consumes memory, and only rarely used.
class CORE_EXPORT InlineBackwardCursor {
  STACK_ALLOCATED();

 public:
  // |cursor| should be the first child of root or descendants, e.g. the first
  // item in |InlineCursor::items_|.
  explicit InlineBackwardCursor(const InlineCursor& cursor);

  const InlineCursorPosition& Current() const { return current_; }
  explicit operator bool() const { return !!Current(); }

  InlineCursor CursorForDescendants() const;

  void MoveToPreviousSibling();

  const PhysicalBoxFragment& ContainerFragment() const {
    return cursor_.ContainerFragment();
  }

 private:
  InlineCursorPosition current_;
  const InlineCursor& cursor_;
  Vector<InlineCursor::ItemsSpan::iterator, 16> sibling_item_iterators_;
  wtf_size_t current_index_;

  friend class InlineCursor;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const InlineCursor&);
CORE_EXPORT std::ostream& operator<<(std::ostream&, const InlineCursor*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_CURSOR_H_
