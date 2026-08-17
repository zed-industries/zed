// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_INPUT_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_INPUT_NODE_H_

#include <optional>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/display_lock/display_lock_context.h"
#include "third_party/blink/renderer/core/layout/geometry/axis.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/layout/layout_box.h"
#include "third_party/blink/renderer/core/layout/layout_box_utils.h"
#include "third_party/blink/renderer/core/layout/list/layout_outside_list_marker.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class BlockNode;
class ComputedStyle;
class Document;
class LayoutObject;
class LayoutBox;
struct PhysicalSize;

// The input to the min/max inline size calculation algorithm for child nodes.
// Child nodes within the same formatting context need to know which floats are
// beside them.
struct MinMaxSizesFloatInput {
  explicit MinMaxSizesFloatInput() = default;
  LayoutUnit float_left_inline_size;
  LayoutUnit float_right_inline_size;
};

// Represents the input to a layout algorithm for a given node. The layout
// engine should use the style, node type to determine which type of layout
// algorithm to use to produce fragments for this node.
class CORE_EXPORT LayoutInputNode {
  DISALLOW_NEW();

 public:
  enum LayoutInputNodeType {
    kBlock,
    kInline
    // When adding new values, ensure type_ below has enough bits.
  };

  static LayoutInputNode Create(LayoutBox* box, LayoutInputNodeType type) {
    // This function should create an instance of the subclass. This works
    // because subclasses are not virtual and do not add fields.
    return LayoutInputNode(box, type);
  }

  LayoutInputNode(std::nullptr_t) : box_(nullptr), type_(kBlock) {}

  LayoutInputNodeType Type() const {
    return static_cast<LayoutInputNodeType>(type_);
  }
  bool IsInline() const { return type_ == kInline; }
  bool IsBlock() const { return type_ == kBlock; }

  bool IsBlockFlow() const { return IsBlock() && box_->IsLayoutBlockFlow(); }
  bool IsBlockInInline() const { return box_->IsBlockInInline(); }
  bool IsCustom() const { return IsBlock() && box_->IsLayoutCustom(); }
  bool IsColumnSpanAll() const { return IsBlock() && box_->IsColumnSpanAll(); }
  bool IsFloating() const { return IsBlock() && box_->IsFloating(); }
  bool IsOutOfFlowPositioned() const {
    return IsBlock() && box_->IsOutOfFlowPositioned();
  }
  bool IsFloatingOrOutOfFlowPositioned() const {
    return IsFloating() || IsOutOfFlowPositioned();
  }
  bool IsReplaced() const { return box_->IsLayoutReplaced(); }
  bool IsAbsoluteContainer() const {
    return box_->CanContainAbsolutePositionObjects();
  }
  bool IsFixedContainer() const {
    return box_->CanContainFixedPositionObjects();
  }
  bool IsBody() const { return IsBlock() && box_->IsBody(); }
  bool IsView() const { return IsBlock() && box_->IsLayoutView(); }
  bool IsDocumentElement() const { return box_->IsDocumentElement(); }
  bool IsFlexItem() const { return IsBlock() && box_->IsFlexItem(); }
  bool IsFlexibleBox() const { return IsBlock() && box_->IsFlexibleBox(); }
  bool IsGrid() const { return IsBlock() && box_->IsLayoutGrid(); }
  bool IsMasonry() const { return IsBlock() && box_->IsLayoutMasonry(); }
  bool ShouldBeConsideredAsReplaced() const {
    return box_->ShouldBeConsideredAsReplaced();
  }
  bool IsListItem() const { return IsBlock() && box_->IsLayoutListItem(); }
  // Returns the list marker if |this.IsListItem()| with an outside list marker.
  // Otherwise |nullptr|.
  BlockNode ListMarkerBlockNodeIfListItem() const;
  bool IsListMarker() const {
    return IsBlock() && box_->IsLayoutOutsideListMarker();
  }
  bool ListMarkerOccupiesWholeLine() const {
    DCHECK(IsListMarker());
    return To<LayoutOutsideListMarker>(box_.Get())->NeedsOccupyWholeLine();
  }
  bool IsButtonOrInputButton() const {
    return IsBlock() && box_->IsButtonOrInputButton();
  }
  bool IsFieldsetContainer() const { return IsBlock() && box_->IsFieldset(); }
  bool IsInitialLetterBox() const { return box_->IsInitialLetterBox(); }
  bool IsMedia() const { return box_->IsMedia(); }
  bool IsCanvas() const { return box_->IsCanvas(); }

  // Return true if this is the legend child of a fieldset that gets special
  // treatment (i.e. placed over the block-start border).
  bool IsRenderedLegend() const {
    return IsBlock() && box_->IsRenderedLegend();
  }
  // Return true if this node is for <input type=range>.
  bool IsSlider() const;
  // Return true if this node is for a slider thumb in <input type=range>.
  bool IsSliderThumb() const;
  bool IsSvgText() const;
  bool IsTable() const { return IsBlock() && box_->IsTable(); }
  bool IsTextCombine() const { return box_->IsLayoutTextCombine(); }

  bool IsTableCaption() const { return IsBlock() && box_->IsTableCaption(); }
  bool IsTableSection() const { return IsBlock() && box_->IsTableSection(); }
  bool IsTableRow() const { return IsBlock() && box_->IsTableRow(); }
  bool IsTableCell() const { return IsBlock() && box_->IsTableCell(); }

  // Section with empty rows is considered empty.
  bool IsEmptyTableSection() const;

  bool IsTableCol() const {
    return Style().Display() == EDisplay::kTableColumn;
  }

  bool IsTableColgroup() const {
    return Style().Display() == EDisplay::kTableColumnGroup;
  }

  wtf_size_t TableColumnSpan() const;

  wtf_size_t TableCellColspan() const;

  wtf_size_t TableCellRowspan() const;

  bool IsTextArea() const { return box_->IsTextArea(); }
  bool IsTextControl() const { return box_->IsTextControl(); }
  bool IsTextControlPlaceholder() const;
  bool IsTextField() const { return box_->IsTextField(); }

  bool IsMathRoot() const { return box_->IsMathMLRoot(); }
  bool IsMathML() const { return box_->IsMathML(); }

  bool IsAnonymous() const { return box_->IsAnonymous(); }
  bool IsAnonymousBlock() const { return box_->IsAnonymousBlock(); }

  // If the node is a quirky container for margin collapsing, see:
  // https://html.spec.whatwg.org/C/#margin-collapsing-quirks
  // NOTE: The spec appears to only somewhat match reality.
  bool IsQuirkyContainer() const {
    return box_->GetDocument().InQuirksMode() &&
           (box_->IsBody() || box_->IsTableCell());
  }

  bool IsHorizontalWritingMode() const {
    return box_->IsHorizontalWritingMode();
  }
  bool IsHorizontalTypographicMode() const {
    return box_->IsHorizontalTypographicMode();
  }

  // Return true if this node is monolithic for block fragmentation.
  bool IsMonolithic() const {
    // Lines are always monolithic. We cannot block-fragment inside them.
    if (IsInline())
      return true;
    return box_->IsMonolithic();
  }

  AtomicString PageName() const {
    return IsBlock() ? Style().Page() : AtomicString();
  }

  bool IsScrollContainer() const {
    return IsBlock() && box_->IsScrollContainer();
  }

  // Return true if this is the document root and it is paginated. A paginated
  // root establishes a fragmentation context.
  bool IsPaginatedRoot() const;

  bool CreatesNewFormattingContext() const {
    return IsBlock() && box_->CreatesNewFormattingContext();
  }

  // Returns intrinsic sizing information for replaced elements.
  // ComputeReplacedSize can use it to compute actual replaced size.
  // Corresponds to Legacy's LayoutReplaced::IntrinsicSizingInfo.
  // Use BlockNode::GetAspectRatio to get the aspect ratio.
  void IntrinsicSize(std::optional<LayoutUnit>* computed_inline_size,
                     std::optional<LayoutUnit>* computed_block_size) const;

  // Returns the next sibling.
  LayoutInputNode NextSibling() const;

  Document& GetDocument() const { return box_->GetDocument(); }

  Node* GetDOMNode() const { return box_->GetNode(); }

  // Return the DOM node of this, or, if none, that of the nearest ancestor that
  // has one.
  //
  // Anonymous objects have no DOM node.
  Node* EnclosingDOMNode() const { return box_->EnclosingNode(); }

  PhysicalSize InitialContainingBlockSize() const;

  // Returns the LayoutObject which is associated with this node.
  LayoutBox* GetLayoutBox() const { return box_.Get(); }

  const ComputedStyle& Style() const { return box_->StyleRef(); }

  bool ShouldApplySizeContainment() const {
    return box_->ShouldApplySizeContainment();
  }
  // Return true if we should apply at least inline-size containment
  // (i.e. "contain" is "size" or "inline-size").
  bool ShouldApplyInlineSizeContainment() const {
    return box_->ShouldApplyInlineSizeContainment();
  }
  // Return true if we should apply at least block-size containment
  // (i.e. "contain" is "size" or "block-size").
  bool ShouldApplyBlockSizeContainment() const {
    return box_->ShouldApplyBlockSizeContainment();
  }

  bool CanMatchSizeContainerQueries() const {
    return box_->CanMatchSizeContainerQueries();
  }

  LogicalAxes ContainedAxes() const {
    LogicalAxes axes = kLogicalAxesNone;
    if (ShouldApplyInlineSizeContainment())
      axes |= kLogicalAxesInline;
    if (ShouldApplyBlockSizeContainment())
      axes |= kLogicalAxesBlock;
    return axes;
  }

  // CSS intrinsic sizing getters.
  // https://drafts.csswg.org/css-sizing-4/#intrinsic-size-override
  // Note that this returns kIndefiniteSize if the override was not specified.
  LayoutUnit OverrideIntrinsicContentInlineSize() const {
    return box_->OverrideIntrinsicContentInlineSize();
  }
  LayoutUnit OverrideIntrinsicContentBlockSize() const {
    return box_->OverrideIntrinsicContentBlockSize();
  }

  LayoutUnit DefaultIntrinsicContentInlineSize() const {
    return box_->DefaultIntrinsicContentInlineSize();
  }
  LayoutUnit DefaultIntrinsicContentBlockSize() const {
    return box_->DefaultIntrinsicContentBlockSize();
  }

  bool ChildLayoutBlockedByDisplayLock() const {
    return box_->ChildLayoutBlockedByDisplayLock();
  }

  CustomLayoutChild* GetCustomLayoutChild() const {
    // TODO(ikilpatrick): Support InlineNode.
    DCHECK(IsBlock());
    return box_->GetCustomLayoutChild();
  }

  // Return whether we can directly traverse fragments generated from this node
  // (for painting, hit-testing and other layout read operations). If false is
  // returned, we need to traverse the layout object tree instead.
  bool CanTraversePhysicalFragments() const {
    return box_->CanTraversePhysicalFragments();
  }

  String ToString() const;

  explicit operator bool() const { return box_ != nullptr; }

  bool operator==(const LayoutInputNode& other) const {
    return box_ == other.box_ && type_ == other.type_;
  }

  bool operator!=(const LayoutInputNode& other) const {
    return !(*this == other);
  }

#if DCHECK_IS_ON()
  String DumpNodeTree(const LayoutInputNode* target = nullptr) const;

  // Dump the node tree for the entire document, and mark `this` with an
  // asterisk.
  String DumpNodeTreeFromRoot() const;

  void ShowNodeTree(const LayoutInputNode* target = nullptr) const;
  void ShowNodeTreeFromRoot() const;
#endif

  void Trace(Visitor* visitor) const { visitor->Trace(box_); }

 protected:
  LayoutInputNode(LayoutBox* box, LayoutInputNodeType type)
      : box_(box), type_(type) {}

  void GetOverrideIntrinsicSize(
      std::optional<LayoutUnit>* computed_inline_size,
      std::optional<LayoutUnit>* computed_block_size) const;

  Member<LayoutBox> box_;

  unsigned type_ : 1;  // LayoutInputNodeType
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_INPUT_NODE_H_
