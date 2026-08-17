// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PHYSICAL_BOX_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PHYSICAL_BOX_FRAGMENT_H_

#include <optional>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_break_token.h"
#include "third_party/blink/renderer/core/layout/gap_fragment_data.h"
#include "third_party/blink/renderer/core/layout/geometry/box_sides.h"
#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/inline/fragment_items.h"
#include "third_party/blink/renderer/core/layout/mathml/mathml_paint_info.h"
#include "third_party/blink/renderer/core/layout/physical_fragment.h"
#include "third_party/blink/renderer/core/layout/physical_fragment_rare_data.h"
#include "third_party/blink/renderer/core/layout/table/table_borders.h"
#include "third_party/blink/renderer/core/layout/table/table_fragment_data.h"
#include "third_party/blink/renderer/core/style/style_overflow_clip_margin.h"
#include "third_party/blink/renderer/platform/graphics/overlay_scrollbar_clip_behavior.h"
#include "third_party/blink/renderer/platform/wtf/bit_field.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class BoxFragmentBuilder;
class Node;
enum class OutlineType;
struct FrameSetLayoutData;

class CORE_EXPORT PhysicalBoxFragment final : public PhysicalFragment {
 public:
  static const PhysicalBoxFragment* Create(
      BoxFragmentBuilder* builder,
      WritingMode block_or_line_writing_mode);

  // Creates a shallow copy of |other|.
  static const PhysicalBoxFragment* Clone(const PhysicalBoxFragment& other);

  // Creates a shallow copy of |other| but uses the "post-layout" fragments to
  // ensure fragment-tree consistency.
  static const PhysicalBoxFragment* CloneWithPostLayoutFragments(
      const PhysicalBoxFragment& other);

  using PassKey = base::PassKey<PhysicalBoxFragment>;
  PhysicalBoxFragment(PassKey,
                      BoxFragmentBuilder* builder,
                      bool has_scrollable_overflow,
                      const PhysicalRect& scrollable_overflow,
                      const PhysicalBoxStrut* borders,
                      const PhysicalBoxStrut* scrollbar,
                      const PhysicalBoxStrut* padding,
                      const std::optional<PhysicalRect>& inflow_bounds,
                      bool has_fragment_items,
                      WritingMode block_or_line_writing_mode);

  // Make a shallow copy. The child fragment pointers are just shallowly
  // copied. Fragment *items* are cloned (but not box fragments associated with
  // items), though. Additionally, the copy will set new overflow information,
  // based on the parameters, rather than copying it from the original fragment.
  PhysicalBoxFragment(PassKey,
                      const PhysicalBoxFragment& other,
                      bool has_scrollable_overflow,
                      const PhysicalRect& scrollable_overflow);

  ~PhysicalBoxFragment();

#if DCHECK_IS_ON()
  class AllowPostLayoutScope {
    STACK_ALLOCATED();

   public:
    AllowPostLayoutScope();
    ~AllowPostLayoutScope();
    static bool IsAllowed() { return allow_count_; }

   private:
    static unsigned allow_count_;
  };
#endif

  void TraceAfterDispatch(Visitor* visitor) const;

  const PhysicalBoxFragment* PostLayout() const;

  // Returns the children of |this|.
  //
  // Note, children in this collection maybe old generations. Items in this
  // collection are safe, but their children (grandchildren of |this|) maybe
  // from deleted nodes or LayoutObjects. Also see |PostLayoutChildren()|.
  base::span<const PhysicalFragmentLink> Children() const {
    DCHECK(children_valid_);
    return base::span(children_);
  }

  const GCedHeapVector<Member<Node>>* ReadingFlowNodes() const {
    if (rare_data_) {
      return rare_data_->reading_flow_nodes_;
    }
    return nullptr;
  }

  // Similar to |Children()| but all children are the latest generation of
  // post-layout, and therefore all descendants are safe.
  PhysicalFragment::PostLayoutChildLinkList PostLayoutChildren() const {
    DCHECK(children_valid_);
    return PostLayoutChildLinkList(base::span(children_));
  }

  // This exposes a mutable part of the fragment for |OutOfFlowLayoutPart|.
  class MutableChildrenForOutOfFlow final {
    STACK_ALLOCATED();

   protected:
    friend class OutOfFlowLayoutPart;
    base::span<PhysicalFragmentLink> Children() const { return span_; }

   private:
    friend class PhysicalBoxFragment;
    explicit MutableChildrenForOutOfFlow(base::span<PhysicalFragmentLink> span)
        : span_(span) {}

    base::span<PhysicalFragmentLink> span_;
  };

  MutableChildrenForOutOfFlow GetMutableChildrenForOutOfFlow() const {
    DCHECK(children_valid_);
    return MutableChildrenForOutOfFlow(
        base::span(const_cast<PhysicalBoxFragment*>(this)->children_));
  }

  // Returns |FragmentItems| if this fragment has one.
  bool HasItems() const {
    // Use get_concurrently because it can be called from a background thread in
    // TraceAfterDispatch().
    return bit_field_.get_concurrently<ConstHasFragmentItemsFlag>();
  }
  const FragmentItems* Items() const {
    return HasItems() ? ComputeItemsAddress() : nullptr;
  }

  std::optional<LayoutUnit> FirstBaseline() const {
    if (has_first_baseline_)
      return first_baseline_;
    return std::nullopt;
  }

  std::optional<LayoutUnit> LastBaseline() const {
    if (has_last_baseline_)
      return last_baseline_;
    return std::nullopt;
  }

  bool UseLastBaselineForInlineBaseline() const {
    return use_last_baseline_for_inline_baseline_;
  }

  // Some scroll-containers will force baseline synthesis for the inline-block
  // baseline algorithm.
  bool ForceInlineBaselineSynthesis() const {
    return use_last_baseline_for_inline_baseline_ && IsScrollContainer() &&
           !Style().ShouldIgnoreOverflowPropertyForInlineBlockBaseline();
  }

  const GapGeometry* GapGeometry() const {
    return rare_data_ ? rare_data_->gap_geometry_.Get() : nullptr;
  }

  LogicalRect TableGridRect() const {
    return rare_data_->GetField(FieldId::kTableGridRect)->table_grid_rect;
  }

  const GCedTableColumnGeometries* TableColumnGeometries() const {
    return rare_data_->table_column_geometries_.Get();
  }

  const TableBorders* TableCollapsedBorders() const {
    return rare_data_ ? rare_data_->table_collapsed_borders_.Get() : nullptr;
  }

  const CollapsedTableBordersGeometry* TableCollapsedBordersGeometry() const {
    if (const auto* field =
            GetRareField(FieldId::kTableCollapsedBordersGeometry)) {
      return field->table_collapsed_borders_geometry.get();
    }
    return nullptr;
  }

  wtf_size_t TableCellColumnIndex() const {
    return rare_data_->GetField(FieldId::kTableCellColumnIndex)
        ->table_cell_column_index;
  }

  std::optional<wtf_size_t> TableSectionStartRowIndex() const {
    DCHECK(IsTableSection());
    if (const auto* field = GetRareField(FieldId::kTableSectionStartRowIndex)) {
      return field->table_section_start_row_index;
    }
    return std::nullopt;
  }

  const Vector<LayoutUnit>* TableSectionRowOffsets() const {
    DCHECK(IsTableSection());
    if (const auto* field = GetRareField(FieldId::kTableSectionRowOffsets)) {
      return &field->table_section_row_offsets;
    }
    return nullptr;
  }

  // The name of the page (if any) to which this fragment belongs. The page name
  // is propagated all the way up to the page fragment, which is needed in order
  // to support e.g. page orientation. See https://drafts.csswg.org/css-page-3
  AtomicString PageName() const {
    if (const auto* field = GetRareField(FieldId::kPageName)) {
      return field->page_name;
    }
    return g_null_atom;
  }

  // Returns the scrollable-overflow for this fragment.
  const PhysicalRect ScrollableOverflow() const {
    if (const auto* field = GetRareField(FieldId::kScrollableOverflow)) {
      return field->scrollable_overflow;
    }
    return {{}, Size()};
  }

  bool HasScrollableOverflow() const {
    return GetRareField(FieldId::kScrollableOverflow);
  }

  const PhysicalBoxStrut Borders() const {
    if (const auto* field = GetRareField(FieldId::kBorders)) {
      return field->borders;
    }
    return PhysicalBoxStrut();
  }

  const PhysicalBoxStrut Scrollbar() const {
    if (const auto* field = GetRareField(FieldId::kScrollbar)) {
      return field->scrollbar;
    }
    return PhysicalBoxStrut();
  }

  const PhysicalBoxStrut Padding() const {
    if (const auto* field = GetRareField(FieldId::kPadding)) {
      return field->padding;
    }
    return PhysicalBoxStrut();
  }

  const PhysicalBoxStrut Margins() const {
    if (const auto* field = GetRareField(FieldId::kMargins)) {
      return field->margins;
    }
    return PhysicalBoxStrut();
  }

  PhysicalOffset ContentOffset() const {
    if (!HasBorders() && !HasPadding())
      return PhysicalOffset();
    PhysicalOffset offset;
    if (HasBorders())
      offset += Borders().Offset();
    if (HasPadding())
      offset += Padding().Offset();
    return offset;
  }

  PhysicalRect ContentRect() const;

  // Returns the bounds of any inflow children for this fragment (specifically
  // no out-of-flow positioned objects). This will return |std::nullopt| if:
  //  - The fragment is *not* a scroll container.
  //  - The scroll container contains no inflow children.
  // This is normally the union of all inflow children's border-box rects
  // (without relative positioning applied), however for grid layout it is the
  // size and position of the grid instead.
  // This is used for scrollable overflow calculations.
  const std::optional<PhysicalRect> InflowBounds() const {
    if (const auto* field = GetRareField(FieldId::kInflowBounds)) {
      return field->inflow_bounds;
    }
    return std::nullopt;
  }

  // Return true if this is either a container that establishes an inline
  // formatting context, or if it's non-atomic inline content participating in
  // one. Empty blocks don't establish an inline formatting context.
  //
  // The return value from this method is undefined and irrelevant if the object
  // establishes a different type of formatting context than block/inline, such
  // as table or flexbox.
  //
  // Example:
  // <div>                                       <!-- false -->
  //   <div>                                     <!-- true -->
  //     <div style="float:left;"></div>         <!-- false -->
  //     <div style="float:left;">               <!-- true -->
  //       xxx                                   <!-- true -->
  //     </div>
  //     <div style="float:left;">               <!-- false -->
  //       <div style="float:left;"></div>       <!-- false -->
  //     </div>
  //     <span>                                  <!-- true -->
  //       xxx                                   <!-- true -->
  //       <span style="display:inline-block;">  <!-- false -->
  //         <div></div>                         <!-- false -->
  //       </span>
  //       <span style="display:inline-block;">  <!-- true -->
  //         xxx                                 <!-- true -->
  //       </span>
  //       <span style="display:inline-flex;">   <!-- N/A -->
  bool IsInlineFormattingContext() const {
    return bit_field_.get<IsInlineFormattingContextFlag>();
  }

  // The |LayoutBox| whose |PhysicalFragments()| contains |this|. This is
  // different from |GetLayoutObject()| if |this.IsColumnBox()|.
  const LayoutBox* OwnerLayoutBox() const;
  LayoutBox* MutableOwnerLayoutBox() const;

  // Returns the offset in the |OwnerLayoutBox| coordinate system. This is only
  // supported for CSS boxes (i.e. not for fragmentainers, for instance).
  PhysicalOffset OffsetFromOwnerLayoutBox() const;

  // TODO(layout-dev): These three methods delegate to legacy layout for now,
  // update them to use LayoutNG based overflow information from the fragment
  // and change them to use NG geometry types once LayoutNG supports overflow.
  PhysicalRect OverflowClipRect(
      const PhysicalOffset& location,
      OverlayScrollbarClipBehavior = kIgnoreOverlayScrollbarSize) const;
  PhysicalRect OverflowClipRect(
      const PhysicalOffset& location,
      const BlockBreakToken* incoming_break_token,
      OverlayScrollbarClipBehavior = kIgnoreOverlayScrollbarSize) const;
  gfx::Vector2d PixelSnappedScrolledContentOffset() const;
  PhysicalSize ScrollSize() const;

  InkOverflow::Type InkOverflowType() const {
    return static_cast<InkOverflow::Type>(
        bit_field_.get<InkOverflowTypeValue>());
  }
  bool IsInkOverflowComputed() const {
    return InkOverflowType() != InkOverflow::Type::kNotSet &&
           InkOverflowType() != InkOverflow::Type::kInvalidated;
  }
  bool HasInkOverflow() const {
    return InkOverflowType() != InkOverflow::Type::kNone;
  }

  // 3 types of ink overflows:
  // * |SelfInkOverflowRect| includes box decorations that are outside of the
  //   border box.
  //   Returns |LocalRect| when there are no overflow.
  // * |ContentsInkOverflowRect| includes anything that would bleed out of the
  //   box and would be clipped by the overflow clip ('overflow' != visible).
  //   This corresponds to children that overflows their parent.
  //   Returns an empty rect when there are no overflow.
  // * |InkOverflowRect| includes self and contents ink overflow, unless it has
  //   clipping, in that case it includes self ink overflow only.
  //   Returns |LocalRect| when there are no overflow.
  PhysicalRect InkOverflowRect() const;
  PhysicalRect SelfInkOverflowRect() const;
  PhysicalRect ContentsInkOverflowRect() const;

  // Fast check if |NodeAtPoint| may find a hit.
  bool MayIntersect(const HitTestResult& result,
                    const HitTestLocation& hit_test_location,
                    const PhysicalOffset& accumulated_offset) const;

  // In order to paint united outline rectangles, the "owner" fragment paints
  // outlines for non-owner fragments.
  bool IsOutlineOwner() const {
    return !IsInlineBox() || InlineContainerFragmentIfOutlineOwner();
  }
  const PhysicalBoxFragment* InlineContainerFragmentIfOutlineOwner() const;

  // Fragment offset is this fragment's offset from parent.
  // Needed to compensate for LayoutInline Legacy code offsets.
  void AddSelfOutlineRects(const PhysicalOffset& additional_offset,
                           OutlineType include_block_overflows,
                           OutlineRectCollector& collector,
                           LayoutObject::OutlineInfo* info) const;
  // Same as |AddSelfOutlineRects|, except when |this.IsInlineBox()|, in which
  // case the coordinate system is relative to the inline formatting context.
  void AddOutlineRects(const PhysicalOffset& additional_offset,
                       OutlineType include_block_overflows,
                       OutlineRectCollector& collector) const;

  PositionWithAffinity PositionForPoint(PhysicalOffset) const;

  // The outsets to apply to the border-box of this fragment for
  // |overflow-clip-margin|.
  PhysicalBoxStrut OverflowClipMarginOutsets() const;

  PhysicalBoxSides SidesToInclude() const {
    return PhysicalBoxSides(IncludeBorderTop(), IncludeBorderRight(),
                            IncludeBorderBottom(), IncludeBorderLeft());
  }

  const BlockBreakToken* GetBreakToken() const {
    return To<BlockBreakToken>(PhysicalFragment::GetBreakToken());
  }

  // Return true if this is the first fragment generated from a node.
  bool IsFirstForNode() const { return bit_field_.get<IsFirstForNodeFlag>(); }

  // Return true if this is the only fragment generated from a node.
  bool IsOnlyForNode() const { return IsFirstForNode() && !GetBreakToken(); }

  bool HasDescendantsForTablePart() const {
    DCHECK(IsTablePart() || IsTableCell());
    return children_.size() || NeedsOOFPositionedInfoPropagation();
  }

  bool IsFragmentationContextRoot() const {
    return bit_field_.get<IsFragmentationContextRootFlag>();
  }

  // Return true if this is the layout root fragment for pagination
  // (aka. printing).
  bool IsPaginatedRoot() const {
    return IsFragmentationContextRoot() && layout_object_->IsLayoutView();
  }

  bool IsMonolithic() const { return bit_field_.get<IsMonolithicFlag>(); }

  bool IsMonolithicOverflowPropagationDisabled() const {
    return bit_field_.get<IsMonolithicOverflowPropagationDisabledFlag>();
  }

  // Returns true if we've called moved children in the block direction (for
  // alignment). See: `BoxFragmentBuilder::MoveChildrenInBlockDirection`.
  bool HasMovedChildrenInBlockDirection() const {
    return bit_field_.get<HasMovedChildrenInBlockDirectionFlag>();
  }

#if DCHECK_IS_ON()
  void CheckSameForSimplifiedLayout(const PhysicalBoxFragment&,
                                    bool check_same_block_size,
                                    bool check_no_fragmentation) const;
#endif

  const FrameSetLayoutData* GetFrameSetLayoutData() const {
    return rare_data_->GetField(FieldId::kFrameSetLayoutData)
        ->frame_set_layout_data.get();
  }

  bool HasExtraMathMLPainting() const {
    if (IsMathMLFraction())
      return true;
    return rare_data_ && rare_data_->mathml_paint_info_;
  }
  const MathMLPaintInfo& GetMathMLPaintInfo() const {
    return *rare_data_->mathml_paint_info_;
  }

  class MutableForStyleRecalc {
    STACK_ALLOCATED();

   public:
    MutableForStyleRecalc(base::PassKey<PhysicalBoxFragment>,
                          PhysicalBoxFragment& fragment);
    void SetScrollableOverflow(const PhysicalRect& scrollable_overflow);

   private:
    PhysicalBoxFragment& fragment_;
  };
  MutableForStyleRecalc GetMutableForStyleRecalc() const;

  class MutableForContainerLayout {
    STACK_ALLOCATED();

   public:
    MutableForContainerLayout(base::PassKey<PhysicalBoxFragment>,
                              PhysicalBoxFragment& fragment);
    void SetMargins(const PhysicalBoxStrut& margins);

   private:
    PhysicalBoxFragment& fragment_;
  };

  MutableForContainerLayout GetMutableForContainerLayout() const;

  // Painters can use const methods only, except for these explicitly declared
  // methods.
  class MutableForPainting {
    STACK_ALLOCATED();

   public:
    void RecalcInkOverflow() { fragment_.RecalcInkOverflow(); }
    void RecalcInkOverflow(const PhysicalRect& contents) {
      fragment_.RecalcInkOverflow(contents);
    }
#if DCHECK_IS_ON()
    void InvalidateInkOverflow() { fragment_.InvalidateInkOverflow(); }
#endif

   private:
    friend class PhysicalBoxFragment;
    explicit MutableForPainting(const PhysicalBoxFragment& fragment)
        : fragment_(const_cast<PhysicalBoxFragment&>(fragment)) {}

    PhysicalBoxFragment& fragment_;
  };
  MutableForPainting GetMutableForPainting() const {
    return MutableForPainting(*this);
  }

  class MutableForCloning {
    STACK_ALLOCATED();
    friend class FragmentRepeater;
    friend class PhysicalBoxFragment;

   public:
    void ClearIsFirstForNode() {
      fragment_.bit_field_.set<IsFirstForNodeFlag>(false);
    }
    void ClearPropagatedOOFs() { fragment_.ClearOofData(); }
    void SetBreakToken(const BlockBreakToken* token) {
      fragment_.break_token_ = token;
    }
    base::span<PhysicalFragmentLink> Children() const {
      DCHECK(fragment_.children_valid_);
      return base::span(fragment_.children_);
    }

    // Remove existing children, and add those from new_fragment.
    void ReplaceChildren(const PhysicalBoxFragment& new_fragment);

   private:
    explicit MutableForCloning(const PhysicalBoxFragment& fragment)
        : fragment_(const_cast<PhysicalBoxFragment&>(fragment)) {}

    PhysicalBoxFragment& fragment_;
  };
  friend class MutableForCloning;
  MutableForCloning GetMutableForCloning() const {
    return MutableForCloning(*this);
  }

  // Fragmented out-of-flow layout is special. An OOF fragment becomes a direct
  // child of a fragmentainer (the fragmentainer that contains the actual
  // containing block of the OOF). This takes place after regular fragmentation
  // context layout (in order to calculate the correct block-offsets (and thus
  // which fragmentainer to start in)), so it will have to mutate the fragment
  // that was created during regular layout, by adding additional children
  // afterwards. This is nowhere close to as good as performing proper layout,
  // but it's mostly good enough for this purpose (there are correctness
  // issues). For one, the break token isn't updated (or created). Since OOF
  // fragmentation is handled specially, this is fine, and even necessary, in
  // fact. All we care about, is to update overflow.
  class MutableForOofFragmentation {
    STACK_ALLOCATED();

   public:
    explicit MutableForOofFragmentation(const PhysicalBoxFragment& fragment)
        : fragment_(const_cast<PhysicalBoxFragment&>(fragment)) {}

    // Merge relevant parts of the specified fragmentainer into this one. This
    // means that all children will be copied over, and they will all be assumed
    // to be out-of-flow. All other necessary bits of information will also be
    // merged over. This includes information inside the break token, as well as
    // anchor queries. The overflow rectangle may also be updated. It's useful
    // to keep in mind that the placeholder fragmentainer has been generated by
    // SimplifiedOofLayoutAlgorithm (which means that we should only copy over
    // information and flags that this algorithm outputs correctly).
    void Merge(const PhysicalBoxFragment& placeholder_fragmentainer);

    // Append a fragmentainer to an existing multicol container fragment.
    void AddChildFragmentainer(const PhysicalBoxFragment& child_fragment,
                               LogicalOffset child_offset);

    // After having added one or more children with AddChildFragment() or
    // Merge(), overflow may have to be updated.
    void UpdateOverflow();

   private:
    PhysicalBoxFragment& fragment_;
  };
  friend class MutableForOofFragmentation;
  MutableForOofFragmentation GetMutableForOofFragmentation() const {
    return MutableForOofFragmentation(*this);
  }

  // Returns if this fragment can compute ink overflow.
  bool CanUseFragmentsForInkOverflow() const {
    return !layout_object_->IsLayoutReplaced();
  }
  // Recalculates and updates |*InkOverflow|.
  void RecalcInkOverflow();
  // |RecalcInkOverflow| using the given contents ink overflow rect.
  void RecalcInkOverflow(const PhysicalRect& contents);

#if DCHECK_IS_ON()
  void InvalidateInkOverflow();
  void AssertFragmentTreeSelf() const;
  void AssertFragmentTreeChildren(bool allow_destroyed_or_moved = false) const;
#endif

 private:
  using BitField = WTF::ConcurrentlyReadBitField<uint32_t>;
  using ConstHasFragmentItemsFlag =
      BitField::DefineFirstValue<bool, 1, WTF::BitFieldValueConstness::kConst>;
  using IsInlineFormattingContextFlag =
      ConstHasFragmentItemsFlag::DefineNextValue<bool, 1>;
  using IncludeBorderTopFlag =
      IsInlineFormattingContextFlag::DefineNextValue<bool, 1>;
  using IncludeBorderRightFlag = IncludeBorderTopFlag::DefineNextValue<bool, 1>;
  using IncludeBorderBottomFlag =
      IncludeBorderRightFlag::DefineNextValue<bool, 1>;
  using IncludeBorderLeftFlag =
      IncludeBorderBottomFlag::DefineNextValue<bool, 1>;
  using InkOverflowTypeValue =
      IncludeBorderLeftFlag::DefineNextValue<uint8_t, InkOverflow::kTypeBits>;
  using IsFirstForNodeFlag = InkOverflowTypeValue::DefineNextValue<bool, 1>;
  using IsFragmentationContextRootFlag =
      IsFirstForNodeFlag::DefineNextValue<bool, 1>;
  using IsMonolithicFlag =
      IsFragmentationContextRootFlag::DefineNextValue<bool, 1>;
  using IsMonolithicOverflowPropagationDisabledFlag =
      IsMonolithicFlag::DefineNextValue<bool, 1>;
  using HasMovedChildrenInBlockDirectionFlag =
      IsMonolithicOverflowPropagationDisabledFlag::DefineNextValue<bool, 1>;

  bool IncludeBorderTop() const {
    return bit_field_.get<IncludeBorderTopFlag>();
  }
  bool IncludeBorderRight() const {
    return bit_field_.get<IncludeBorderRightFlag>();
  }
  bool IncludeBorderBottom() const {
    return bit_field_.get<IncludeBorderBottomFlag>();
  }
  bool IncludeBorderLeft() const {
    return bit_field_.get<IncludeBorderLeftFlag>();
  }
  bool HasBorders() const { return !!GetRareField(FieldId::kBorders); }
  bool HasPadding() const { return !!GetRareField(FieldId::kPadding); }
  bool HasInflowBounds() const {
    return !!GetRareField(FieldId::kInflowBounds);
  }

  static size_t AdditionalByteSize(bool has_fragment_items);

  using FieldId = PhysicalFragmentRareData::FieldId;
  ALWAYS_INLINE const PhysicalFragmentRareData::RareField* GetRareField(
      FieldId id) const {
    if (rare_data_) {
      return rare_data_->GetField(id);
    }
    return nullptr;
  }
  PhysicalFragmentRareData::RareField& EnsureRareField(FieldId id);

  const FragmentItems* ComputeItemsAddress() const {
    DCHECK(HasItems());
    // SAFETY: FragmentItems is placed just after this object. So `this + 1`
    // is valid. See Create() and AdditionalByteSize().
    return reinterpret_cast<const FragmentItems*>(base::bits::AlignUp(
        reinterpret_cast<const uint8_t*>(UNSAFE_BUFFERS(this + 1)),
        alignof(FragmentItems)));
  }

  void SetInkOverflow(const PhysicalRect& self, const PhysicalRect& contents);
  void SetInkOverflowType(InkOverflow::Type type) {
    bit_field_.set<InkOverflowTypeValue>(static_cast<uint8_t>(type));
  }
  PhysicalRect RecalcContentsInkOverflow();
  PhysicalRect ComputeSelfInkOverflow() const;

  void AddOutlineRects(const PhysicalOffset& additional_offset,
                       OutlineType include_block_overflows,
                       bool inline_container_relative,
                       OutlineRectCollector& collector) const;
  void AddOutlineRectsForInlineBox(PhysicalOffset additional_offset,
                                   OutlineType include_block_overflows,
                                   bool inline_container_relative,
                                   OutlineRectCollector& collector) const;

  PositionWithAffinity PositionForPointByClosestChild(
      PhysicalOffset point_in_contents) const;

  PositionWithAffinity PositionForPointInBlockFlowDirection(
      PhysicalOffset point_in_contents) const;

  PositionWithAffinity PositionForPointInTable(
      PhysicalOffset point_in_contents) const;

  PositionWithAffinity PositionForPointRespectingEditingBoundaries(
      const PhysicalBoxFragment& child,
      PhysicalOffset point_in_child) const;

#if DCHECK_IS_ON()
  void CheckIntegrity() const;
#endif

  BitField bit_field_;

  LayoutUnit first_baseline_;
  LayoutUnit last_baseline_;
  Member<PhysicalFragmentRareData> rare_data_;
  InkOverflow ink_overflow_;
  HeapVector<PhysicalFragmentLink> children_;
  // fragment_items is after |children_| if they are not empty/initial.
};

template <>
struct DowncastTraits<PhysicalBoxFragment> {
  static bool AllowFrom(const PhysicalFragment& fragment) {
    return fragment.Type() == PhysicalFragment::kFragmentBox;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PHYSICAL_BOX_FRAGMENT_H_
