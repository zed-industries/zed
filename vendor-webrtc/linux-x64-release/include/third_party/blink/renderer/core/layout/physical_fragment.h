// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PHYSICAL_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PHYSICAL_FRAGMENT_H_

#include <unicode/ubidi.h>

#include <iterator>
#include <optional>

#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/layout/anchor_evaluator_impl.h"
#include "third_party/blink/renderer/core/layout/break_token.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/ink_overflow.h"
#include "third_party/blink/renderer/core/layout/layout_box.h"
#include "third_party/blink/renderer/core/layout/layout_inline.h"
#include "third_party/blink/renderer/core/layout/physical_fragment_link.h"
#include "third_party/blink/renderer/core/layout/style_variant.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"
#include "third_party/blink/renderer/platform/graphics/touch_action.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"

namespace blink {

class ComputedStyle;
class FragmentBuilder;
class FragmentData;
class Node;
class PaintLayer;
enum class OutlineType;
struct FragmentedOofData;
struct LogicalRect;
struct PhysicalOofPositionedNode;

// The PhysicalFragment contains the output geometry from layout. The
// fragment stores all of its information in the physical coordinate system for
// use by paint, hit-testing etc.
//
// The fragment keeps a pointer back to the LayoutObject which generated it.
// Once we have transitioned fully to LayoutNG it should be a const pointer
// such that paint/hit-testing/etc don't modify it.
//
// Layout code should only access geometry information through the
// LogicalFragment wrapper classes which transforms information into the logical
// coordinate system.
class CORE_EXPORT PhysicalFragment : public GarbageCollected<PhysicalFragment> {
 public:
  enum FragmentType {
    kFragmentBox = 0,
    kFragmentLineBox = 1,
    // When adding new values, make sure the bit size of |type_| is large
    // enough to store.
  };
  enum BoxType {
    kNormalBox,
    kInlineBox,
    // A multi-column container creates column boxes as its children, which
    // content is flowed into. https://www.w3.org/TR/css-multicol-1/#column-box
    // This is a fragmentainer.
    kColumnBox,
    // The containing block of a page. Used by printing. This fragment includes
    // a non-optional kPageBorderBox child, and up to 16 optional kPageMargin
    // children (up to three for each of the four page edges, and one for each
    // corner). This is the outermost part of what the spec refers to as "page
    // box". It is sized with respect to the destination paper size (if any),
    // not necessarily what @page size dictates. Responsible for painting any
    // @page background, which should cover then entire page container.
    // See https://drafts.csswg.org/css-page-3/#page-model
    kPageContainer,
    // The border box of a page. Used by printing. This is the innermost part of
    // what the spec refers to as "page box". It is sized with respect to any
    // given @page size when possible, and also honors scaling from print
    // settings, and automatic shrink scaling to fit wide content (rather than
    // overflowing). Responsible for painting any @page borders and outlines,
    // and the document background (typically specified on BODY or the document
    // root), which should cover the entire page border box. This fragment
    // includes a non-optional kPageArea child.
    // See https://drafts.csswg.org/css-page-3/#page-model
    kPageBorderBox,
    // Page margin fragment (e.g. author-specified header / footer). Used by
    // printing.
    kPageMargin,
    // A page area fragment. Used by printing. This is a fragmentainer, into
    // which document contents flow and get fragmented. It is sized with respect
    // to any given @page size when possible, and also honors scaling from print
    // settings, and automatic shrink scaling to fit wide content (rather than
    // overflowing). Painting it may entail scaling it down to fit on paper.
    kPageArea,
    kAtomicInline,
    kFloating,
    kOutOfFlowPositioned,
    kBlockFlowRoot,
    kRenderedLegend,
    // When adding new values, make sure the bit size of |sub_type_| is large
    // enough to store.

    // Also, add after kMinimumFormattingContextRoot if the box type is a
    // formatting context root, or before otherwise. See
    // IsFormattingContextRoot().
    kMinimumFormattingContextRoot = kAtomicInline
  };

  struct PropagatedData : public GarbageCollected<PropagatedData> {
   public:
    PropagatedData(
        const HeapVector<Member<LayoutBoxModelObject>>* sticky_descendants,
        const HeapVector<Member<Element>>* snap_areas,
        const Member<const LayoutObject> scroll_initial_target)
        : sticky_descendants(sticky_descendants),
          snap_areas(snap_areas),
          scroll_initial_target(scroll_initial_target) {}
    void Trace(Visitor* visitor) const {
      visitor->Trace(sticky_descendants);
      visitor->Trace(snap_areas);
      visitor->Trace(scroll_initial_target);
    }
    Member<const HeapVector<Member<LayoutBoxModelObject>>> sticky_descendants;
    Member<const HeapVector<Member<Element>>> snap_areas;
    Member<const LayoutObject> scroll_initial_target;
  };

  PhysicalFragment(FragmentBuilder* builder,
                   WritingMode block_or_line_writing_mode,
                   FragmentType type,
                   unsigned sub_type);

  PhysicalFragment(const PhysicalFragment& other);

  FragmentType Type() const { return static_cast<FragmentType>(type_); }
  bool IsContainer() const {
    return Type() == FragmentType::kFragmentBox ||
           Type() == FragmentType::kFragmentLineBox;
  }
  bool IsBox() const { return Type() == FragmentType::kFragmentBox; }
  bool IsLineBox() const { return Type() == FragmentType::kFragmentLineBox; }

  // Returns the box type of this fragment.
  BoxType GetBoxType() const {
    DCHECK(IsBox());
    return static_cast<BoxType>(sub_type_);
  }
  // True if this is an inline box; e.g., <span>. Atomic inlines such as
  // replaced elements or inline block are not included.
  bool IsInlineBox() const {
    return IsBox() && GetBoxType() == BoxType::kInlineBox;
  }
  bool IsColumnBox() const {
    return IsBox() && GetBoxType() == BoxType::kColumnBox;
  }
  static bool IsFragmentainerBoxType(BoxType type) {
    return type == BoxType::kColumnBox || type == BoxType::kPageArea;
  }
  bool IsFragmentainerBox() const {
    return IsBox() && IsFragmentainerBoxType(GetBoxType());
  }
  bool IsColumnSpanAll() const {
    if (const auto* box = DynamicTo<LayoutBox>(GetLayoutObject()))
      return box->IsColumnSpanAll();
    return false;
  }
  // An atomic inline is represented as a kFragmentBox, such as inline block and
  // replaced elements.
  bool IsAtomicInline() const {
    return IsBox() && GetBoxType() == BoxType::kAtomicInline;
  }
  // True if this box is a block-in-inline, or if this line contains a
  // block-in-inline.
  bool IsBlockInInline() const { return is_block_in_inline_; }
  // True if this is a line fragment that has a block/float child in a parallel
  // fragmentation flow.
  bool IsLineForParallelFlow() const { return is_line_for_parallel_flow_; }
  // True if this fragment is in-flow in an inline formatting context.
  bool IsInline() const { return IsInlineBox() || IsAtomicInline(); }
  bool IsFloating() const {
    return IsBox() && GetBoxType() == BoxType::kFloating;
  }
  bool IsOutOfFlowPositioned() const {
    return IsBox() && GetBoxType() == BoxType::kOutOfFlowPositioned;
  }
  bool IsFixedPositioned() const {
    return IsCSSBox() && layout_object_->IsFixedPositioned();
  }
  bool IsFloatingOrOutOfFlowPositioned() const {
    return IsFloating() || IsOutOfFlowPositioned();
  }
  bool IsPositioned() const {
    if (const LayoutObject* layout_object = GetLayoutObject())
      return layout_object->IsPositioned();
    return false;
  }
  bool HasStickyConstrainedPosition() const {
    return IsCSSBox() &&
           layout_object_->StyleRef().HasStickyConstrainedPosition();
  }
  bool IsInitialLetterBox() const {
    return IsCSSBox() && layout_object_->IsInitialLetterBox();
  }
  bool IsSnapArea() const {
    return IsCSSBox() && IsA<LayoutBox>(layout_object_.Get()) &&
           layout_object_->StyleRef().GetScrollSnapAlign() !=
               cc::ScrollSnapAlign();
  }
  // Return true if this is the legend child of a fieldset that gets special
  // treatment (i.e. placed over the block-start border).
  bool IsRenderedLegend() const {
    return IsBox() && GetBoxType() == BoxType::kRenderedLegend;
  }
  bool IsMathML() const {
    return IsBox() && GetSelfOrContainerLayoutObject()->IsMathML();
  }
  bool IsMathMLFraction() const { return IsBox() && is_math_fraction_; }

  bool IsMathMLOperator() const { return IsBox() && is_math_operator_; }

  // Return true if this fragment corresponds directly to an entry in the CSS
  // box tree [1]. Note that anonymous blocks also exist in the CSS box
  // tree. Additionally, page box and page margin box fragments [2] will return
  // true.
  //
  // Returns false otherwise, i.e. if the fragment is generated by the layout
  // engine to contain fragments from CSS boxes (a line or a generated
  // fragmentainer [3], in other words). The main signification of this is
  // whether we can use the LayoutObject associated with this fragment for all
  // purposes.
  //
  // [1] https://www.w3.org/TR/css-display-3/#box-tree
  // [2] https://www.w3.org/TR/css-page-3/#page-model
  // [3] https://www.w3.org/TR/css-break-3/#fragmentation-container
  bool IsCSSBox() const { return !IsLineBox() && !IsFragmentainerBox(); }

  bool IsBlockFlow() const;
  bool IsAnonymousBlock() const {
    return IsCSSBox() && layout_object_->IsAnonymousBlock();
  }
  bool IsFrameSet() const { return IsCSSBox() && layout_object_->IsFrameSet(); }
  bool IsListMarker() const {
    return IsCSSBox() && layout_object_->IsLayoutOutsideListMarker();
  }

  bool IsSvg() const { return layout_object_->IsSVG(); }
  bool IsSvgText() const { return layout_object_->IsSVGText(); }

  bool IsTablePart() const { return is_table_part_; }

  bool IsTable() const { return IsTablePart() && layout_object_->IsTable(); }

  bool IsTableRow() const {
    return IsTablePart() && layout_object_->IsTableRow();
  }

  bool IsTableSection() const {
    return IsTablePart() && layout_object_->IsTableSection();
  }

  bool IsTableCell() const {
    return IsTablePart() && layout_object_->IsTableCell();
  }

  bool IsGrid() const { return layout_object_->IsLayoutGrid(); }

  bool IsTextControlContainer() const;
  bool IsTextControlPlaceholder() const;

  // Return true if this fragment is a container established by a fieldset
  // element. Such a fragment contains an optional rendered legend fragment and
  // an optional fieldset contents wrapper fragment (which holds everything
  // inside the fieldset except the rendered legend).
  bool IsFieldsetContainer() const { return is_fieldset_container_; }

  // Returns whether the fragment should be atomically painted.
  bool IsPaintedAtomically() const { return is_painted_atomically_; }

  // Returns whether the fragment is a table part with collapsed borders.
  bool HasCollapsedBorders() const { return has_collapsed_borders_; }

  bool IsFormattingContextRoot() const {
    return IsBox() && GetBoxType() >= BoxType::kMinimumFormattingContextRoot;
  }

  // Returns true if we have a descendant within this formatting context, which
  // is potentially above our block-start edge.
  bool MayHaveDescendantAboveBlockStart() const {
    return may_have_descendant_above_block_start_;
  }

  // The accessors in this class shouldn't be used by layout code directly,
  // instead should be accessed by the NGFragmentBase classes. These accessors
  // exist for paint, hit-testing, etc.

  // Returns the border-box size.
  PhysicalSize Size() const { return size_; }

  // Returns the rect in the local coordinate of this fragment; i.e., offset is
  // (0, 0).
  PhysicalRect LocalRect() const { return {{}, size_}; }

  StyleVariant GetStyleVariant() const {
    return static_cast<StyleVariant>(style_variant_);
  }
  bool UsesFirstLineStyle() const {
    return blink::UsesFirstLineStyle(GetStyleVariant());
  }

  // Returns the style for this fragment.
  //
  // For a line box, this returns the style of the containing block. This mostly
  // represents the style for the line box, except 1) |style.Direction()| maybe
  // incorrect, use |BaseDirection()| instead, and 2) margin/border/padding,
  // background etc. do not apply to the line box.
  const ComputedStyle& Style() const {
    return layout_object_->EffectiveStyle(GetStyleVariant());
  }

  const Document& GetDocument() const {
    DCHECK(layout_object_);
    return layout_object_->GetDocument();
  }
  Node* GetNode() const {
    return IsCSSBox() ? layout_object_->GetNode() : nullptr;
  }
  Node* GeneratingNode() const {
    return IsCSSBox() ? layout_object_->GeneratingNode() : nullptr;
  }
  // The node to return when hit-testing on this fragment. This can be different
  // from GetNode() when this fragment is content of a pseudo node.
  Node* NodeForHitTest() const {
    if (IsFragmentainerBox())
      return nullptr;
    return layout_object_->NodeForHitTest();
  }

  Node* NonPseudoNode() const {
    return IsCSSBox() ? layout_object_->NonPseudoNode() : nullptr;
  }

  bool IsInSelfHitTestingPhase(HitTestPhase phase) const {
    if (IsFragmentainerBox())
      return false;
    if (const auto* box = DynamicTo<LayoutBox>(GetLayoutObject()))
      return box->IsInSelfHitTestingPhase(phase);
    if (IsInlineBox())
      return phase == HitTestPhase::kForeground;
    // Assuming this is some sort of container, e.g. a fragmentainer (they don't
    // have a LayoutObject associated).
    return phase == HitTestPhase::kSelfBlockBackground;
  }

  // Whether there is a PaintLayer associated with the fragment.
  bool HasLayer() const { return IsCSSBox() && layout_object_->HasLayer(); }

  // The PaintLayer associated with the fragment.
  PaintLayer* Layer() const {
    if (!HasLayer())
      return nullptr;
    return To<LayoutBoxModelObject>(layout_object_.Get())->Layer();
  }

  // Whether this object has a self-painting |Layer()|.
  bool HasSelfPaintingLayer() const {
    return HasLayer() && To<LayoutBoxModelObject>(layout_object_.Get())
                             ->HasSelfPaintingLayer();
  }

  // True if overflow != 'visible', except for certain boxes that do not allow
  // overflow clip; i.e., AllowOverflowClip() returns false.
  bool HasNonVisibleOverflow() const {
    return IsCSSBox() && layout_object_->HasNonVisibleOverflow();
  }

  OverflowClipAxes GetOverflowClipAxes() const {
    if (!IsCSSBox()) {
      return kNoOverflowClip;
    }
    return layout_object_->GetOverflowClipAxes();
  }

  bool HasNonVisibleBlockOverflow() const {
    OverflowClipAxes clip_axes = GetOverflowClipAxes();
    if (Style().IsHorizontalWritingMode()) {
      return clip_axes & kOverflowClipY;
    }
    return clip_axes & kOverflowClipX;
  }

  // True if this is considered a scroll-container. See
  // ComputedStyle::IsScrollContainer() for details.
  bool IsScrollContainer() const {
    return IsCSSBox() && layout_object_->IsScrollContainer();
  }

  // Return true if the given object is the effective root scroller in its
  // Document. See |effective root scroller| in page/scrolling/README.md.
  // Note: a root scroller always establishes a PaintLayer.
  // This bit is updated in
  // RootScrollerController::RecomputeEffectiveRootScroller in the LayoutClean
  // document lifecycle phase.
  bool IsEffectiveRootScroller() const {
    return IsCSSBox() && layout_object_->IsEffectiveRootScroller();
  }

  bool ShouldApplyLayoutContainment() const {
    return IsCSSBox() && layout_object_->ShouldApplyLayoutContainment();
  }

  bool ShouldClipOverflowAlongEitherAxis() const {
    return IsCSSBox() && layout_object_->ShouldClipOverflowAlongEitherAxis();
  }

  bool ShouldClipOverflowAlongBothAxis() const {
    return IsCSSBox() && layout_object_->ShouldClipOverflowAlongBothAxis();
  }

  bool ShouldApplyOverflowClipMargin() const {
    return IsCSSBox() && layout_object_->ShouldApplyOverflowClipMargin();
  }

  // Return whether we can traverse this fragment and its children directly, for
  // painting, hit-testing and other layout read operations. If false is
  // returned, we need to traverse the layout object tree instead.
  bool CanTraverse() const {
    return layout_object_->CanTraversePhysicalFragments();
  }

  // This fragment is hidden for paint purpose, but exists for querying layout
  // information. Used for `text-overflow: ellipsis`.
  bool IsHiddenForPaint() const {
    return is_hidden_for_paint_ || layout_object_->IsTruncated();
  }

  // This fragment is opaque for layout and paint, as if it does not exist and
  // does not paint its backgrounds and borders, but it can have regular
  // children and paint properties such as filters can apply.
  bool IsOpaque() const { return is_opaque_; }

  // Return true if this fragment is monolithic, as far as block fragmentation
  // is concerned.
  bool IsMonolithic() const;

  // Returns true this fragment is used as the implicit anchor for another
  // element in CSS anchor positioning.
  // Should only be called during layout as it inspects DOM.
  bool IsImplicitAnchor() const;

  bool IsExplicitAnchor() const { return IsCSSBox() && Style().AnchorName(); }

  bool IsAnchor() const { return IsExplicitAnchor() || IsImplicitAnchor(); }

  // GetLayoutObject should only be used when necessary for compatibility
  // with LegacyLayout.
  //
  // For a line box, |layout_object_| has its containing block but this function
  // returns |nullptr| for the historical reasons. TODO(kojii): We may change
  // this in future. Use |IsLineBox()| instead of testing this is |nullptr|.
  const LayoutObject* GetLayoutObject() const {
    return IsCSSBox() ? layout_object_.Get() : nullptr;
  }
  // TODO(kojii): We should not have mutable version at all, the use of this
  // function should be eliminiated over time.
  LayoutObject* GetMutableLayoutObject() const {
    return IsCSSBox() ? layout_object_.Get() : nullptr;
  }
  // Similar to |GetLayoutObject|, but returns the |LayoutObject| of its
  // container for |!IsCSSBox()| fragments instead of |nullptr|.
  const LayoutObject* GetSelfOrContainerLayoutObject() const {
    return layout_object_.Get();
  }

  const FragmentData* GetFragmentData() const;

  // |PhysicalFragment| may live longer than the corresponding |LayoutObject|.
  // Though |PhysicalFragment| is immutable, |layout_object_| is cleared to
  // |nullptr| when it was destroyed to avoid reading destroyed objects.
  bool IsLayoutObjectDestroyedOrMoved() const { return !layout_object_; }
  void LayoutObjectWillBeDestroyed() const {
    const_cast<PhysicalFragment*>(this)->layout_object_ = nullptr;
  }

  // Returns the latest generation of the post-layout fragment. Returns
  // |nullptr| if |this| is the one.
  //
  // When subtree relayout occurs at the relayout boundary, its containing block
  // may keep the reference to old generations of this fragment. Callers can
  // check if there were newer generations.
  const PhysicalFragment* PostLayout() const;

  // Helper functions to convert between |PhysicalRect| and |LogicalRect| of a
  // child.
  LogicalRect ConvertChildToLogical(const PhysicalRect& physical_rect) const;

  String ToString() const;

  void CheckType() const;

  enum DumpFlag {
    DumpHeaderText = 0x1,
    DumpSubtree = 0x2,
    DumpIndentation = 0x4,
    DumpType = 0x8,
    DumpOffset = 0x10,
    DumpSize = 0x20,
    DumpTextOffsets = 0x40,
    DumpSelfPainting = 0x80,
    DumpNodeName = 0x100,
    DumpItems = 0x200,
    DumpLegacyDescendants = 0x400,
    DumpAll = -1
  };
  typedef int DumpFlags;

  // Dump the fragment tree, optionally mark |target| if it's found. If not
  // found, the subtree established by |target| will be dumped as well.
  [[nodiscard]] String DumpFragmentTree(
      DumpFlags,
      const PhysicalFragment* target = nullptr,
      std::optional<PhysicalOffset> = std::nullopt,
      unsigned indent = 2) const;

  // Dump the fragment tree, starting at |root| (searching inside legacy
  // subtrees to find all fragments), optionally mark |target| if it's found. If
  // not found, the subtree established by |target| will be dumped as well.
  //
  // Note that if we're in the middle of layout somewhere inside the subtree,
  // behavior is undefined.
  [[nodiscard]] static String DumpFragmentTree(
      const LayoutObject& root,
      DumpFlags,
      const PhysicalFragment* target = nullptr);

  void Trace(Visitor*) const;
  void TraceAfterDispatch(Visitor*) const;

  // Same as |base::span<const PhysicalFragmentLink>|, except that:
  // * Each |PhysicalFragmentLink| has the latest generation of post-layout. See
  //   |PhysicalFragment::PostLayout()| for more details.
  // * The iterator skips fragments for destroyed or moved |LayoutObject|.
  class PostLayoutChildLinkList {
    STACK_ALLOCATED();

   public:
    PostLayoutChildLinkList(base::span<const PhysicalFragmentLink> buffer)
        : buffer_(buffer) {}

    class ConstIterator {
      STACK_ALLOCATED();
      using BaseIterator =
          base::span<const PhysicalFragmentLink>::const_iterator;

     public:
      using iterator_category = std::bidirectional_iterator_tag;
      using value_type = PhysicalFragmentLink;
      using difference_type = ptrdiff_t;
      using pointer = value_type*;
      using reference = value_type&;

      ConstIterator() = default;

      ConstIterator(BaseIterator current, BaseIterator end)
          : current_(current), end_(end) {
        SkipInvalidAndSetPostLayout();
      }

      const PhysicalFragmentLink& operator*() const { return post_layout_; }
      const PhysicalFragmentLink* operator->() const { return &post_layout_; }

      ConstIterator& operator++() {
        ++current_;
        SkipInvalidAndSetPostLayout();
        return *this;
      }
      ConstIterator operator++(int) {
        ConstIterator copy = *this;
        ++*this;
        return copy;
      }
      bool operator==(const ConstIterator& other) const {
        return current_ == other.current_;
      }
      bool operator!=(const ConstIterator& other) const {
        return current_ != other.current_;
      }

     private:
      void SkipInvalidAndSetPostLayout() {
        for (; current_ != end_; ++current_) {
          const PhysicalFragment* fragment = current_->fragment.Get();
          if (fragment->IsLayoutObjectDestroyedOrMoved()) [[unlikely]] {
            continue;
          }
          if (const PhysicalFragment* post_layout = fragment->PostLayout()) {
            post_layout_.fragment = post_layout;
            post_layout_.offset = current_->offset;
            return;
          }
        }
      }

      BaseIterator current_;
      BaseIterator end_;
      PhysicalFragmentLink post_layout_;
    };
    using const_iterator = ConstIterator;

    const_iterator begin() const {
      return const_iterator(buffer_.begin(), buffer_.end());
    }
    const_iterator end() const {
      return const_iterator(buffer_.end(), buffer_.end());
    }

    wtf_size_t size() const { return buffer_.size(); }
    bool empty() const { return buffer_.empty(); }

   private:
    base::span<const PhysicalFragmentLink> buffer_;
  };

  const BreakToken* GetBreakToken() const { return break_token_.Get(); }

  base::span<const PhysicalFragmentLink> Children() const;

  PostLayoutChildLinkList PostLayoutChildren() const;

  // Returns true if we have any floating descendants which need to be
  // traversed during the float paint phase.
  bool HasFloatingDescendantsForPaint() const {
    return has_floating_descendants_for_paint_;
  }

  // Returns true if we have any adjoining-object descendants (floats, or
  // inline-level OOF-positioned objects).
  bool HasAdjoiningObjectDescendants() const {
    return has_adjoining_object_descendants_;
  }

  // Returns true if we aren't able to re-use this fragment if the
  // |ConstraintSpace::PercentageResolutionBlockSize| changes.
  bool DependsOnPercentageBlockSize() const {
    return depends_on_percentage_block_size_;
  }

  void SetChildrenInvalid() const;
  bool ChildrenValid() const { return children_valid_; }

  const HeapVector<Member<LayoutBoxModelObject>>* StickyDescendants() const {
    return propagated_data_ ? propagated_data_->sticky_descendants.Get()
                            : nullptr;
  }
  const HeapVector<Member<LayoutBoxModelObject>>* PropagatedStickyDescendants()
      const {
    return IsScrollContainer() ? nullptr : StickyDescendants();
  }

  const Member<const LayoutObject> ScrollInitialTarget() const {
    return propagated_data_ ? propagated_data_->scroll_initial_target : nullptr;
  }
  const Member<const LayoutObject> PropagatedScrollInitialTarget() const {
    return IsScrollContainer() ? nullptr : ScrollInitialTarget();
  }

  const HeapVector<Member<Element>>* SnapAreas() const {
    return propagated_data_ ? propagated_data_->snap_areas.Get() : nullptr;
  }
  const HeapVector<Member<Element>>* PropagatedSnapAreas() const {
    return IsScrollContainer() ? nullptr : SnapAreas();
  }

  bool HasPropagatedLayoutObjects() const {
    return PropagatedStickyDescendants() || PropagatedScrollInitialTarget() ||
           PropagatedSnapAreas();
  }

  class OofData : public GarbageCollected<OofData> {
   public:
    virtual ~OofData() = default;
    virtual void Trace(Visitor* visitor) const;
    HeapVector<PhysicalOofPositionedNode>& OofPositionedDescendants() {
      return oof_positioned_descendants_;
    }
    void SetAnchorQuery(PhysicalAnchorQuery* query) { anchor_query_ = query; }
    const PhysicalAnchorQuery* AnchorQuery() const { return anchor_query_; }
    PhysicalAnchorQuery& EnsureAnchorQuery();

   private:
    HeapVector<PhysicalOofPositionedNode> oof_positioned_descendants_;
    Member<PhysicalAnchorQuery> anchor_query_;
  };

  // Returns true if some child is OOF in the fragment tree. This happens if
  // it's the containing block of the OOF, or if it's a fragmentation context
  // root containing them.
  bool HasOutOfFlowFragmentChild() const {
    return has_out_of_flow_fragment_child_;
  }

  // If there is an OOF contained within a fragmentation context, this will
  // return true for all fragments in the chain from the OOF's CB to the
  // fragmentainer that the CB resides in.
  bool HasOutOfFlowInFragmentainerSubtree() const {
    return has_out_of_flow_in_fragmentainer_subtree_;
  }

  bool HasOutOfFlowPositionedDescendants() const {
    return oof_data_ && !oof_data_->OofPositionedDescendants().empty();
  }

  base::span<PhysicalOofPositionedNode> OutOfFlowPositionedDescendants() const;

  bool HasAnchorQuery() const {
    return oof_data_ && oof_data_->AnchorQuery() &&
           !oof_data_->AnchorQuery()->IsEmpty();
  }
  bool HasAnchorQueryToPropagate() const {
    return HasAnchorQuery() || IsAnchor();
  }
  const PhysicalAnchorQuery* AnchorQuery() const {
    if (!HasAnchorQuery())
      return nullptr;
    return oof_data_->AnchorQuery();
  }

  const FragmentedOofData* GetFragmentedOofData() const;

  // Return true if there are nested multicol container descendants with OOFs
  // inside.
  bool HasNestedMulticolsWithOOFs() const;

  // Figure out if the child has any out-of-flow positioned descendants, in
  // which case we'll need to propagate this to the fragment builder.
  bool NeedsOOFPositionedInfoPropagation() const;

 protected:
  ~PhysicalFragment() = default;

  const ComputedStyle& SlowEffectiveStyle() const;

  void AddOutlineRectsForNormalChildren(
      OutlineRectCollector& collector,
      const PhysicalOffset& additional_offset,
      OutlineType outline_type,
      const LayoutBoxModelObject* containing_block) const;
  void AddOutlineRectsForCursor(OutlineRectCollector& collector,
                                const PhysicalOffset& additional_offset,
                                OutlineType outline_type,
                                const LayoutBoxModelObject* containing_block,
                                InlineCursor* cursor) const;
  void AddOutlineRectsForDescendant(
      const PhysicalFragmentLink& descendant,
      OutlineRectCollector& collector,
      const PhysicalOffset& additional_offset,
      OutlineType outline_type,
      const LayoutBoxModelObject* containing_block) const;

  static bool DependsOnPercentageBlockSize(const FragmentBuilder&);

  OofData* OofDataFromBuilder(FragmentBuilder*);
  OofData* FragmentedOofDataFromBuilder(FragmentBuilder*);
  void ClearOofData();
  OofData* CloneOofData() const;

  Member<LayoutObject> layout_object_;
  PhysicalSize size_;

  const uint8_t type_ : 1;           // FragmentType
  const uint8_t sub_type_ : 4;       // BoxType, TextItemType, or LineBoxType
  const uint8_t style_variant_ : 2;  // StyleVariant
  const uint8_t is_hidden_for_paint_ : 1;
  uint8_t : 0;  // NOLINT, zero-length bitfield used to allow the compiler to
                // split memory locations. If the above bitfields are part of
                // the same memory location as the bitfields below, they will
                // all be updated together, which will result in races.

  uint8_t has_floating_descendants_for_paint_ : 1;  // NOLINT
  uint8_t has_adjoining_object_descendants_ : 1;    // NOLINT
  uint8_t depends_on_percentage_block_size_ : 1;    // NOLINT
  mutable uint8_t children_valid_ : 1;              // NOLINT

  // The following bitfields are only to be used by PhysicalLineBoxFragment
  // (it's defined here to save memory, since that class has no bitfields).
  uint8_t has_propagated_descendants_ : 1;             // NOLINT
  uint8_t has_hanging_ : 1;                            // NOLINT
  uint8_t is_opaque_ : 1;                              // NOLINT
  uint8_t is_block_in_inline_ : 1;                     // NOLINT
  uint8_t is_line_for_parallel_flow_ : 1;              // NOLINT
  uint8_t is_math_fraction_ : 1;                       // NOLINT
  uint8_t is_math_operator_ : 1;                       // NOLINT
  uint8_t may_have_descendant_above_block_start_ : 1;  // NOLINT

  // The following are only used by PhysicalBoxFragment but are initialized
  // for all types to allow methods using them to be inlined.
  uint8_t is_fieldset_container_ : 1;                           // NOLINT
  uint8_t is_table_part_ : 1;                                   // NOLINT
  uint8_t is_painted_atomically_ : 1;                           // NOLINT
  uint8_t has_collapsed_borders_ : 1;                           // NOLINT
  uint8_t has_first_baseline_ : 1;                              // NOLINT
  uint8_t has_last_baseline_ : 1;                               // NOLINT
  uint8_t use_last_baseline_for_inline_baseline_ : 1;           // NOLINT
  const uint8_t has_fragmented_out_of_flow_data_ : 1;           // NOLINT
  uint8_t has_out_of_flow_fragment_child_ : 1;                  // NOLINT
  const uint8_t has_out_of_flow_in_fragmentainer_subtree_ : 1;  // NOLINT

  // The following are only used by PhysicalLineBoxFragment.
  uint8_t base_direction_ : 1;  // NOLINT, TextDirection

  Member<const PropagatedData> propagated_data_;
  Member<const BreakToken> break_token_;
  Member<OofData> oof_data_;
};

template <>
struct ThreadingTrait<PhysicalFragment> {
  static constexpr ThreadAffinity kAffinity = kMainThreadOnly;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const PhysicalFragment*);
CORE_EXPORT std::ostream& operator<<(std::ostream&, const PhysicalFragment&);

#if !DCHECK_IS_ON()
inline void PhysicalFragment::CheckType() const {}
#endif

}  // namespace blink

#if DCHECK_IS_ON()
// Outside the blink namespace for ease of invocation from a debugger.

// Output the fragment tree to the log.
// See DumpFragmentTree().
CORE_EXPORT void ShowFragmentTree(const blink::PhysicalFragment*);

// Output the fragment tree(s) inside |root| to the log.
// See DumpFragmentTree(const LayoutObject& ...).
CORE_EXPORT void ShowFragmentTree(
    const blink::LayoutObject& root,
    const blink::PhysicalFragment* target = nullptr);

// Output the fragment tree(s) from the entire document to the log.
// See DumpFragmentTree(const LayoutObject& ...).
CORE_EXPORT void ShowEntireFragmentTree(const blink::LayoutObject& target);
CORE_EXPORT void ShowEntireFragmentTree(const blink::PhysicalFragment* target);
#endif  // DCHECK_IS_ON()

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PHYSICAL_FRAGMENT_H_
