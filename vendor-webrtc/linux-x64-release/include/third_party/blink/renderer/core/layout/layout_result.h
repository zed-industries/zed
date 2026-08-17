// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_RESULT_H_

#include <optional>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/break_appeal.h"
#include "third_party/blink/renderer/core/layout/constraint_space.h"
#include "third_party/blink/renderer/core/layout/early_break.h"
#include "third_party/blink/renderer/core/layout/exclusions/exclusion_space.h"
#include "third_party/blink/renderer/core/layout/flex/devtools_flex_info.h"
#include "third_party/blink/renderer/core/layout/floats_utils.h"
#include "third_party/blink/renderer/core/layout/geometry/bfc_offset.h"
#include "third_party/blink/renderer/core/layout/geometry/margin_strut.h"
#include "third_party/blink/renderer/core/layout/logical_fragment.h"
#include "third_party/blink/renderer/core/layout/non_overflowing_scroll_range.h"
#include "third_party/blink/renderer/core/layout/physical_fragment.h"
#include "third_party/blink/renderer/core/layout/physical_fragment_link.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/wtf/bit_field.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class BoxFragmentBuilder;
class ColumnSpannerPath;
class ExclusionSpace;
class FragmentBuilder;
class LineBoxFragmentBuilder;

// The LayoutResult stores the resulting data from layout. This includes
// geometry information in form of a PhysicalFragment, which is kept around
// for painting, hit testing, etc., as well as additional data which is only
// necessary during layout and stored on this object.
// Layout code should access the PhysicalFragment through the wrappers in
// LogicalFragment et al.
class CORE_EXPORT LayoutResult final : public GarbageCollected<LayoutResult> {
 public:
  enum EStatus {
    kSuccess = 0,
    kBfcBlockOffsetResolved = 1,
    kNeedsEarlierBreak = 2,
    kOutOfFragmentainerSpace = 3,
    kNeedsLineClampRelayout = 4,
    kDisableFragmentation = 5,
    kNeedsRelayoutWithNoChildScrollbarChanges = 6,
    kTextBoxTrimEndDidNotApply = 7,
    kAlgorithmSpecific1 = 8,  // Save bits by using the same value for mutually
                              // exclusive results.
    kNeedsRelayoutWithRowCrossSizeChanges = kAlgorithmSpecific1,
    kNeedsRelayoutAsLastTableBox = kAlgorithmSpecific1,
    // When adding new values, make sure the bit size of |Bitfields::status| is
    // large enough to store.
  };

  // Make a shallow clone of the result. The fragment is cloned. Fragment
  // *items* are also cloned, but child fragments are not. Apart from that it's
  // truly shallow. Pinky promise.
  static const LayoutResult* Clone(const LayoutResult&);

  // Same as Clone(), but uses the "post-layout" fragments to ensure
  // fragment-tree consistency.
  static const LayoutResult* CloneWithPostLayoutFragments(
      const LayoutResult& other);

  // Create a copy of LayoutResult with |BfcBlockOffset| replaced by the given
  // parameter. Note, when |bfc_block_offset| is |nullopt|, |BfcBlockOffset| is
  // still replaced with |nullopt|.
  LayoutResult(const LayoutResult& other,
               const ConstraintSpace& new_space,
               const MarginStrut& new_end_margin_strut,
               LayoutUnit bfc_line_offset,
               std::optional<LayoutUnit> bfc_block_offset,
               LayoutUnit block_offset_delta);

  // Creates a copy of LayoutResult with a new (but "identical") fragment.
  LayoutResult(const LayoutResult& other,
               const PhysicalFragment* physical_fragment);

  // Delegate constructor that sets up what it can, based on the builder.
  LayoutResult(const PhysicalFragment* physical_fragment,
               FragmentBuilder* builder);

  // We don't need the copy constructor, move constructor, copy
  // assigmnment-operator, or move assignment-operator today.
  // If at some point we do need these constructors particular care will need
  // to be taken with the |rare_data_| field.
  LayoutResult(const LayoutResult&) = delete;
  LayoutResult(LayoutResult&&) = delete;
  LayoutResult& operator=(const LayoutResult& other) = delete;
  LayoutResult& operator=(LayoutResult&& other) = delete;
  LayoutResult() = delete;

  ~LayoutResult() = default;

  const PhysicalFragment& GetPhysicalFragment() const {
    DCHECK(physical_fragment_);
    DCHECK_EQ(kSuccess, Status());
    return *physical_fragment_;
  }

  int LinesUntilClamp() const {
    return rare_data_ ? rare_data_->lines_until_clamp : 0;
  }

  // Returns true if the block-end of this line box is trimmable by the
  // `text-box-trim` property. If it's true, it means that this is the line box
  // that was the candidate for block-end trimming, but this doesn't necessarily
  // mean that trimming actually took place. Trimming may be prevented by
  // non-zero trailing border / padding, for instance.
  bool IsBlockEndTrimmableLine() const {
    return rare_data_ && rare_data_->is_block_end_trimmable_line();
  }

  // Returns true if this line box is not the last line in its IFC, but only
  // because it has a line-clamp ellipsis that pushed content to the next line.
  bool WouldBeLastLineIfNotForEllipsis() const {
    return rare_data_ && rare_data_->would_be_last_line_if_not_for_ellipsis();
  }

  // Return true if this is an orthogonal writing-mode root that depends on the
  // size of the initial containing block.
  bool HasOrthogonalFallbackInlineSize() const {
    return space_.UsesOrthogonalFallbackInlineSize();
  }

  // Return true if there's an orthogonal writing-mode root descendant inside
  // that depends on the size of the initial containing block.
  bool HasOrthogonalFallbackSizeDescendant() const {
    return bitfields_.has_orthogonal_fallback_size_descendant;
  }

  // Return the adjustment baked into the fragment's block-offset that's caused
  // by ruby annotations.
  LayoutUnit AnnotationBlockOffsetAdjustment() const {
    if (!rare_data_) {
      return LayoutUnit();
    }
    const RareData::LineData* data = rare_data_->GetLineData();
    return data ? data->annotation_block_offset_adjustment : LayoutUnit();
  }

  // How much an annotation box overflow from this box.
  // This is for LayoutRubyColumn and line boxes.
  // 0 : No overflow
  // -N : Overflowing by N px at block-start side
  //      This happens only for LayoutRubyColumn.
  // N : Overflowing by N px at block-end side
  LayoutUnit AnnotationOverflow() const {
    return rare_data_ ? rare_data_->annotation_overflow : LayoutUnit();
  }

  // The amount of available space for block-start side annotations of the
  // next box.
  // This never be negative.
  LayoutUnit BlockEndAnnotationSpace() const {
    return rare_data_ ? rare_data_->block_end_annotation_space : LayoutUnit();
  }

  LogicalOffset OutOfFlowPositionedOffset() const {
    // The offset is either explicitly stored on the rare data, or impliclty
    // stored as the start offset of |oof_insets_for_get_computed_style_|.
    CHECK(bitfields_.has_oof_insets_for_get_computed_style);
    return rare_data_ && rare_data_->oof_positioned_offset_is_set()
               ? rare_data_->OutOfFlowPositionedOffset()
               : oof_insets_for_get_computed_style_.StartOffset();
  }

  // Returns the absolutized inset property values in the parent's writing mode.
  // Not necessarily the insets of the actual box in the container, but matches
  // the result of the `getComputedStyle()` JavaScript API.
  const BoxStrut& OutOfFlowInsetsForGetComputedStyle() const {
    CHECK(bitfields_.has_oof_insets_for_get_computed_style);
    return oof_insets_for_get_computed_style_;
  }

  // Called after subtree layout to make sure the fields for out-of-flow
  // positioned nodes are set.
  void CopyMutableOutOfFlowData(const LayoutResult& previous_result) const;

  const HeapVector<NonOverflowingScrollRange>* NonOverflowingScrollRanges()
      const {
    return rare_data_ ? rare_data_->NonOverflowingScrollRanges() : nullptr;
  }

  bool NeedsAnchorPositionScrollAdjustmentInX() const {
    return rare_data_ &&
           rare_data_->needs_anchor_position_scroll_adjustment_in_x();
  }
  bool NeedsAnchorPositionScrollAdjustmentInY() const {
    return rare_data_ &&
           rare_data_->needs_anchor_position_scroll_adjustment_in_y();
  }

  // Get the path to the column spanner (if any) that interrupted column layout.
  const ColumnSpannerPath* GetColumnSpannerPath() const {
    return rare_data_ ? rare_data_->column_spanner_path.Get() : nullptr;
  }

  // True if this result is the parent of a column spanner and is empty (i.e.
  // has no children). This is used to determine whether the column spanner
  // margins should collapse. Note that |is_empty_spanner_parent| may be false
  // even if this column spanner parent is actually empty. This can happen in
  // the case where the spanner parent has no children but has not broken
  // previously - in which case, we shouldn't collapse the spanner margins since
  // we do not want to collapse margins with a column spanner outside of this
  // parent.
  bool IsEmptySpannerParent() const {
    return bitfields_.is_empty_spanner_parent;
  }

  const EarlyBreak* GetEarlyBreak() const {
    if (!rare_data_) {
      return nullptr;
    }
    return rare_data_->early_break.Get();
  }

  const ExclusionSpace& GetExclusionSpace() const {
    if (bitfields_.has_rare_data_exclusion_space) {
      DCHECK(rare_data_);
      return rare_data_->exclusion_space;
    }

    return space_.GetExclusionSpace();
  }

  EStatus Status() const { return static_cast<EStatus>(bitfields_.status); }

  LayoutUnit BfcLineOffset() const {
    if (bitfields_.has_oof_insets_for_get_computed_style) {
      DCHECK(physical_fragment_->IsOutOfFlowPositioned());
      return LayoutUnit();
    }

    return bfc_offset_.line_offset;
  }

  const std::optional<LayoutUnit> BfcBlockOffset() const {
    if (bitfields_.has_oof_insets_for_get_computed_style) {
      DCHECK(physical_fragment_->IsOutOfFlowPositioned());
      return LayoutUnit();
    }

    if (bitfields_.is_bfc_block_offset_nullopt)
      return std::nullopt;

    return bfc_offset_.block_offset;
  }

  // The BFC block-offset where a line-box has been placed. Will be nullopt if
  // it isn't a line-box, or an empty line-box.
  //
  // This can be different (but rarely) to where the |BfcBlockOffset()|
  // resolves to, when floats are present. E.g.
  //
  // <div style="width: 100px; display: flow-root;">
  //   <div style="float: left; width: 200px; height: 20px;"></div>
  //   text
  // </div>
  //
  // In the above example the |BfcBlockOffset()| will be at 0px, where-as the
  // |LineBoxBfcBlockOffset()| will be at 20px.
  std::optional<LayoutUnit> LineBoxBfcBlockOffset() const {
    if (Status() != kSuccess || !GetPhysicalFragment().IsLineBox()) {
      return std::nullopt;
    }

    if (rare_data_) {
      if (std::optional<LayoutUnit> offset =
              rare_data_->LineBoxBfcBlockOffset()) {
        return offset;
      }
    }

    return BfcBlockOffset();
  }

  const MarginStrut EndMarginStrut() const {
    return rare_data_ ? rare_data_->end_margin_strut : MarginStrut();
  }

  // Get the intrinsic block-size of the fragment. This is the block-size the
  // fragment would get if no block-size constraints were applied and, for
  // non-replaced elements, no inline-size constraints were applied through any
  // aspect-ratio (For replaced elements, inline-size constraints ARE applied
  // through the aspect-ratio).

  // This is not supported (and should not be needed [1]) if the node got split
  // into multiple fragments.
  //
  // [1] If a node gets block-fragmented, it means that it has possibly been
  // constrained and/or stretched by something extrinsic (i.e. the
  // fragmentainer), so the value returned here wouldn't be useful.
  const LayoutUnit IntrinsicBlockSize() const {
#if DCHECK_IS_ON()
    AssertSoleBoxFragment();
#endif
    return intrinsic_block_size_;
  }

  // Return the amount of clearance that we have to add after the fragment. This
  // is used for BR clear elements.
  std::optional<LayoutUnit> ClearanceAfterLine() const {
    if (rare_data_) [[unlikely]] {
      return rare_data_->ClearanceAfterLine();
    }
    return std::nullopt;
  }

  // Return the amount to trim the block size by the `text-box-trim` property.
  std::optional<LayoutUnit> TrimBlockEndBy() const {
    if (rare_data_) [[unlikely]] {
      return rare_data_->TrimBlockEndBy();
    }
    return std::nullopt;
  }

  std::optional<LayoutUnit> MinimalSpaceShortage() const {
    if (!rare_data_ || space_.IsInitialColumnBalancingPass() ||
        rare_data_->minimal_space_shortage == kIndefiniteSize) {
      return std::nullopt;
    }
    return rare_data_->minimal_space_shortage;
  }

  LayoutUnit TallestUnbreakableBlockSize() const {
    if (!rare_data_ || !space_.IsInitialColumnBalancingPass() ||
        rare_data_->tallest_unbreakable_block_size == kIndefiniteSize) {
      return LayoutUnit();
    }
    return rare_data_->tallest_unbreakable_block_size;
  }

  // Return the block-size that this fragment will take up inside a
  // fragmentation context. This will include overflow from descendants (if it
  // is visible and supposed to affect block fragmentation), and also
  // out-of-flow positioned descendants (in the initial balancing pass), but not
  // relative offsets. kIndefiniteSize will be returned if block fragmentation
  // wasn't performed on the node (e.g. monolithic content such as line boxes,
  // or if the node isn't inside a fragmentation context at all).
  LayoutUnit BlockSizeForFragmentation() const {
    if (!rare_data_) {
      return kIndefiniteSize;
    }
    return rare_data_->block_size_for_fragmentation;
  }

  // Return true if the block-size for fragmentation (see
  // BlockSizeForFragmentation()) got clamped. If this is the case, we cannot
  // use BlockSizeForFragmentation() for cache testing.
  bool IsBlockSizeForFragmentationClamped() const {
    return bitfields_.is_block_size_for_fragmentation_clamped;
  }

  // Return true if this generating node must stay within the same fragmentation
  // flow as the parent (and not establish a parallel fragmentation flow), even
  // if it has content that overflows into the next fragmentainer.
  bool ShouldForceSameFragmentationFlow() const {
    return bitfields_.should_force_same_fragmentation_flow;
  }

  // Return the (lowest) appeal among any unforced breaks inside the resulting
  // fragment (or kBreakAppealPerfect if there are no such breaks).
  //
  // A higher value is better. Violating breaking rules decreases appeal. Forced
  // breaks always have perfect appeal.
  //
  // If a node breaks, the resulting fragment usually carries an outgoing break
  // token, but this isn't necessarily the case if the break happened inside an
  // inner fragmentation context. The block-size of an inner multicol is
  // constrained by the available block-size in the outer fragmentation
  // context. This may cause suboptimal column breaks inside. The entire inner
  // multicol container may fit in the outer fragmentation context, but we may
  // also need to consider the inner column breaks (in an inner fragmentation
  // context). If there are any suboptimal breaks, we may want to push the
  // entire multicol container to the next outer fragmentainer, if it's likely
  // that we'll avoid suboptimal column breaks inside that way.
  BreakAppeal GetBreakAppeal() const {
    return static_cast<BreakAppeal>(bitfields_.break_appeal);
  }

  SerializedScriptValue* CustomLayoutData() const {
    return rare_data_ ? rare_data_->custom_layout_data.get() : nullptr;
  }

  wtf_size_t TableColumnCount() const {
    if (!rare_data_) {
      return 0;
    }
    const RareData::TableData* data = rare_data_->GetTableData();
    return data ? data->table_column_count : 0;
  }

  const GridLayoutData* GetGridLayoutData() const {
    if (!rare_data_) {
      return nullptr;
    }
    const RareData::GridData* data = rare_data_->GetGridData();
    return data ? data->grid_layout_data.get() : nullptr;
  }

  const DevtoolsFlexInfo* FlexLayoutData() const {
    if (!rare_data_) {
      return nullptr;
    }
    const RareData::FlexData* data = rare_data_->GetFlexData();
    return data ? data->flex_layout_data.get() : nullptr;
  }

  LayoutUnit MathItalicCorrection() const {
    if (!rare_data_) {
      return LayoutUnit();
    }
    const RareData::MathData* data = rare_data_->GetMathData();
    return data ? data->italic_correction : LayoutUnit();
  }

  // The break-before value on the first child needs to be propagated to the
  // container, in search of a valid class A break point.
  EBreakBetween InitialBreakBefore() const {
    return static_cast<EBreakBetween>(bitfields_.initial_break_before);
  }

  // The break-after value on the last child needs to be propagated to the
  // container, in search of a valid class A break point.
  EBreakBetween FinalBreakAfter() const {
    return static_cast<EBreakBetween>(bitfields_.final_break_after);
  }

  // Return true if the fragment broke because a forced break before a child.
  bool HasForcedBreak() const { return bitfields_.has_forced_break; }

  // Returns true if the fragment should be considered empty for margin
  // collapsing purposes (e.g. margins "collapse through").
  bool IsSelfCollapsing() const { return bitfields_.is_self_collapsing; }

  // Return true if this fragment got its block offset increased by the presence
  // of floats.
  bool IsPushedByFloats() const { return bitfields_.is_pushed_by_floats; }

  // Returns the types of preceding adjoining objects.
  // See |AdjoiningObjectTypes|.
  //
  // Adjoining floats should be treated differently when calculating clearance
  // on a block with adjoining block-start margin (in such cases we will know
  // up front that the block will need clearance, since, if it doesn't, the
  // float will be pulled along with the block, and the block will fail to
  // clear).
  AdjoiningObjectTypes GetAdjoiningObjectTypes() const {
    return bitfields_.adjoining_object_types;
  }

  // Returns true if the initial (pre-layout) block-size of this fragment was
  // indefinite. (e.g. it has "height: auto").
  bool IsInitialBlockSizeIndefinite() const {
    return bitfields_.is_initial_block_size_indefinite;
  }

  // Returns true if there is a descendant that depends on percentage
  // resolution block-size changes.
  // Some layout modes (flex-items, table-cells) have more complex child
  // percentage sizing behaviour (typically when their parent layout forces a
  // block-size on them).
  bool HasDescendantThatDependsOnPercentageBlockSize() const {
    return bitfields_.has_descendant_that_depends_on_percentage_block_size;
  }

  // Returns true if this subtree modified the incoming margin-strut (i.e.
  // appended a non-zero margin).
  bool SubtreeModifiedMarginStrut() const {
    return bitfields_.subtree_modified_margin_strut;
  }

  // Returns true if the fragment got truncated because it reached the
  // fragmentation line. This typically means that we cannot re-use (cache-hit)
  // this fragment if the fragmentation line moves.
  bool IsTruncatedByFragmentationLine() const {
    return bitfields_.is_truncated_by_fragmentation_line;
  }

  // Returns the space which generated this object for caching purposes.
  const ConstraintSpace& GetConstraintSpaceForCaching() const { return space_; }

  // Returns the most recent anchor evaluated (if there is only one anchor).
  // This value is cleared before a position fallback is applied.
  Element* AccessibilityAnchor() const {
    if (!rare_data_) {
      return nullptr;
    }
    return rare_data_->accessibility_anchor;
  }

  const GCedHeapHashSet<Member<Element>>* DisplayLocksAffectedByAnchors()
      const {
    if (!rare_data_) {
      return nullptr;
    }
    return rare_data_->display_locks_affected_by_anchors;
  }

  // This exposes a mutable part of the layout result just for the
  // |OutOfFlowLayoutPart|.
  class MutableForOutOfFlow final {
    STACK_ALLOCATED();

   protected:
    friend class OutOfFlowLayoutPart;

    void SetOutOfFlowInsetsForGetComputedStyle(const BoxStrut& insets) {
      // OOF-positioned nodes *must* always have an initial BFC-offset.
      DCHECK(layout_result_->physical_fragment_->IsOutOfFlowPositioned());
      DCHECK_EQ(layout_result_->BfcLineOffset(), LayoutUnit());
      DCHECK_EQ(layout_result_->BfcBlockOffset().value_or(LayoutUnit()),
                LayoutUnit());

      layout_result_->bitfields_.has_oof_insets_for_get_computed_style = true;
      layout_result_->oof_insets_for_get_computed_style_ = insets;
    }

    void SetOutOfFlowPositionedOffset(const LogicalOffset& offset) {
      CHECK(layout_result_->bitfields_.has_oof_insets_for_get_computed_style);
      // To minimize the chance of creating a rare data, we explicitly store
      // |offset| on rare data only if:
      // 1. There's already an offset stored on rare data, in which case we
      //    simply update it regardlessly.
      // 2. It no longer matches the start offset of the stored insets.
      if ((layout_result_->rare_data_ &&
           layout_result_->rare_data_->oof_positioned_offset_is_set()) ||
          offset != layout_result_->oof_insets_for_get_computed_style_
                        .StartOffset()) {
        layout_result_->EnsureRareData()->SetOutOfFlowPositionedOffset(offset);
      }
    }

    void SetNeedsScrollAdjustment(bool needs_scroll_adjustment_in_x,
                                  bool needs_scroll_adjustment_in_y) {
      if (!needs_scroll_adjustment_in_x && !needs_scroll_adjustment_in_y) {
        return;
      }
      layout_result_->EnsureRareData()
          ->set_needs_anchor_position_scroll_adjustment_in_x(
              needs_scroll_adjustment_in_x);
      layout_result_->EnsureRareData()
          ->set_needs_anchor_position_scroll_adjustment_in_y(
              needs_scroll_adjustment_in_y);
    }

    void SetNonOverflowingScrollRanges(
        const HeapVector<NonOverflowingScrollRange>& non_overflowing_ranges) {
      if (layout_result_->rare_data_ || !non_overflowing_ranges.empty()) {
        layout_result_->EnsureRareData()->SetNonOverflowingScrollRanges(
            non_overflowing_ranges);
      }
    }

    void SetAccessibilityAnchor(Element* anchor);

    void SetDisplayLocksAffectedByAnchors(
        GCedHeapHashSet<Member<Element>>* display_locks);

   private:
    friend class LayoutResult;
    MutableForOutOfFlow(const LayoutResult* layout_result)
        : layout_result_(const_cast<LayoutResult*>(layout_result)) {}

    LayoutResult* layout_result_;
  };

  MutableForOutOfFlow GetMutableForOutOfFlow() const {
    return MutableForOutOfFlow(this);
  }

  class MutableForLayoutBoxCachedResults final {
    STACK_ALLOCATED();

   protected:
    friend class LayoutBox;
    friend class MeasureCache;

    void SetFragmentChildrenInvalid() {
      layout_result_->physical_fragment_->SetChildrenInvalid();
    }

   private:
    friend class LayoutResult;
    explicit MutableForLayoutBoxCachedResults(const LayoutResult* layout_result)
        : layout_result_(const_cast<LayoutResult*>(layout_result)) {}

    LayoutResult* layout_result_;
  };

  MutableForLayoutBoxCachedResults GetMutableForLayoutBoxCachedResults() const {
    return MutableForLayoutBoxCachedResults(this);
  }

#if DCHECK_IS_ON()
  void CheckSameForSimplifiedLayout(const LayoutResult&,
                                    bool check_same_block_size = true,
                                    bool check_no_fragmentation = true) const;
#endif

  using FragmentBuilderPassKey = base::PassKey<FragmentBuilder>;
  // This constructor is for a non-success status.
  LayoutResult(FragmentBuilderPassKey, EStatus, FragmentBuilder*);

  // This constructor requires a non-null fragment and sets a success status.
  using BoxFragmentBuilderPassKey = base::PassKey<BoxFragmentBuilder>;
  LayoutResult(BoxFragmentBuilderPassKey,
               const PhysicalFragment* physical_fragment,
               BoxFragmentBuilder*);

  using LineBoxFragmentBuilderPassKey = base::PassKey<LineBoxFragmentBuilder>;
  // This constructor requires a non-null fragment and sets a success status.
  LayoutResult(LineBoxFragmentBuilderPassKey,
               const PhysicalFragment* physical_fragment,
               LineBoxFragmentBuilder*);

  void Trace(Visitor*) const;

 private:
  friend class MutableForOutOfFlow;

  static ExclusionSpace MergeExclusionSpaces(
      const LayoutResult& other,
      const ExclusionSpace& new_input_exclusion_space,
      LayoutUnit bfc_line_offset,
      LayoutUnit block_offset_delta);

  struct RareData final : public GarbageCollected<RareData> {
   public:
    // RareData has fields which are mutually exclusive. They are grouped into
    // unions.
    //
    // NOTE: Make sure that data_union_type has enough bits to express all these
    // enum values.
    enum DataUnionType {
      kNone,
      kFlexData,
      kGridData,
      kLineSmallData,
      kLineData,
      kMathData,
      kTableData,
    };

    using BitField = WTF::ConcurrentlyReadBitField<uint16_t>;
    using LineBoxBfcBlockOffsetIsSetFlag = BitField::DefineFirstValue<bool, 1>;
    using OutOfFlowPositionedOffsetIsSetFlag =
        LineBoxBfcBlockOffsetIsSetFlag::DefineNextValue<bool, 1>;
    using NeedsAnchorPositionScrollAdjustmentInXFlag =
        OutOfFlowPositionedOffsetIsSetFlag::DefineNextValue<bool, 1>;
    using NeedsAnchorPositionScrollAdjustmentInYFlag =
        NeedsAnchorPositionScrollAdjustmentInXFlag::DefineNextValue<bool, 1>;
    using DataUnionTypeValue =
        NeedsAnchorPositionScrollAdjustmentInYFlag::DefineNextValue<uint8_t, 3>;
    using IsBlockEndTrimmableLineFlag =
        DataUnionTypeValue::DefineNextValue<bool, 1>;
    using WouldBeLastLineIfNotForEllipsis =
        IsBlockEndTrimmableLineFlag::DefineNextValue<bool, 1>;

    struct FlexData {
      FlexData() = default;
      FlexData(const FlexData& other) {
        flex_layout_data =
            std::make_unique<DevtoolsFlexInfo>(*other.flex_layout_data);
      }

      std::unique_ptr<const DevtoolsFlexInfo> flex_layout_data;
    };

    struct GridData {
      GridData() = default;
      GridData(const GridData& other) {
        grid_layout_data =
            std::make_unique<GridLayoutData>(*other.grid_layout_data);
      }

      std::unique_ptr<const GridLayoutData> grid_layout_data;
    };

    // `LineSmallData` can save allocations When only fields in it are needed.
    struct LineSmallData {
      std::optional<LayoutUnit> ClearanceAfterLine() const {
        return clearance_after_line.NullOptIfMin();
      }
      std::optional<LayoutUnit> TrimBlockEndBy() const {
        return trim_block_end_by.NullOptIfMin();
      }

      LayoutUnit clearance_after_line = LayoutUnit::Min();
      LayoutUnit trim_block_end_by = LayoutUnit::Min();
    };

    // `LineData` is allocated separately as it's larger than data unions.
    struct LineData : public LineSmallData {
      LayoutUnit annotation_block_offset_adjustment;
    };

    struct LineDataPtr {
      LineDataPtr() = default;
      LineDataPtr(const LineDataPtr& other) {
        line_data = std::make_unique<LineData>(*other.line_data);
      }

      std::unique_ptr<LineData> line_data = std::make_unique<LineData>();
    };

    struct MathData {
      // See https://w3c.github.io/mathml-core/#box-model
      LayoutUnit italic_correction;
    };

    struct TableData {
      wtf_size_t table_column_count = 0;
    };

    bool line_box_bfc_block_offset_is_set() const {
      return bit_field.get<LineBoxBfcBlockOffsetIsSetFlag>();
    }

    void set_line_box_bfc_block_offset_is_set(bool flag) {
      return bit_field.set<LineBoxBfcBlockOffsetIsSetFlag>(flag);
    }

    bool oof_positioned_offset_is_set() const {
      return bit_field.get<OutOfFlowPositionedOffsetIsSetFlag>();
    }

    void set_oof_positioned_offset_is_set(bool flag) {
      return bit_field.set<OutOfFlowPositionedOffsetIsSetFlag>(flag);
    }

    bool needs_anchor_position_scroll_adjustment_in_x() const {
      return bit_field.get<NeedsAnchorPositionScrollAdjustmentInXFlag>();
    }

    void set_needs_anchor_position_scroll_adjustment_in_x(bool flag) {
      return bit_field.set<NeedsAnchorPositionScrollAdjustmentInXFlag>(flag);
    }

    bool needs_anchor_position_scroll_adjustment_in_y() const {
      return bit_field.get<NeedsAnchorPositionScrollAdjustmentInYFlag>();
    }

    void set_needs_anchor_position_scroll_adjustment_in_y(bool flag) {
      return bit_field.set<NeedsAnchorPositionScrollAdjustmentInYFlag>(flag);
    }

    DataUnionType data_union_type() const {
      return static_cast<DataUnionType>(
          bit_field.get_concurrently<DataUnionTypeValue>());
    }

    void set_data_union_type(DataUnionType data_type) {
      return bit_field.set<DataUnionTypeValue>(static_cast<uint8_t>(data_type));
    }

    bool is_block_end_trimmable_line() const {
      return bit_field.get<IsBlockEndTrimmableLineFlag>();
    }
    void set_is_block_end_trimmable_line() {
      bit_field.set<IsBlockEndTrimmableLineFlag>(true);
    }

    bool would_be_last_line_if_not_for_ellipsis() const {
      return bit_field.get<WouldBeLastLineIfNotForEllipsis>();
    }
    void set_would_be_last_line_if_not_for_ellipsis() {
      bit_field.set<WouldBeLastLineIfNotForEllipsis>(true);
    }

    template <typename DataType>
    DataType* EnsureData(DataType* address, DataUnionType data_type) {
      DataUnionType old_data_type = data_union_type();
      DCHECK(old_data_type == kNone || old_data_type == data_type);
      if (old_data_type != data_type) {
        set_data_union_type(data_type);
        new (address) DataType();
      }
      return address;
    }
    bool HasData(DataUnionType data_type) const {
      return data_union_type() == data_type;
    }
    template <typename DataType>
    const DataType* GetData(const DataType* address,
                            DataUnionType data_type) const {
      return data_union_type() == data_type ? address : nullptr;
    }

    FlexData* EnsureFlexData() {
      return EnsureData<FlexData>(&flex_data, kFlexData);
    }
    const FlexData* GetFlexData() const {
      return GetData<FlexData>(&flex_data, kFlexData);
    }
    GridData* EnsureGridData() {
      return EnsureData<GridData>(&grid_data, kGridData);
    }
    const GridData* GetGridData() const {
      return GetData<GridData>(&grid_data, kGridData);
    }
    // When both `EnsureLineData()` and `EnsureLineSmallData()` are needed,
    // `EnsureLineData()` must be done first. Upgrading `kLineSmallData` to
    // `kLineData` isn't supported due to the lack of the needs.
    LineData* EnsureLineData() {
      return EnsureData(&line_data, kLineData)->line_data.get();
    }
    const LineData* GetLineData() const {
      const LineDataPtr* data = GetData(&line_data, kLineData);
      return data ? data->line_data.get() : nullptr;
    }
    LineSmallData* EnsureLineSmallData() {
      if (HasData(kLineData)) [[unlikely]] {
        return EnsureLineData();
      }
      return EnsureData(&line_small_data, kLineSmallData);
    }
    const LineSmallData* GetLineSmallData() const {
      if (HasData(kLineData)) [[unlikely]] {
        return GetLineData();
      }
      return GetData(&line_small_data, kLineSmallData);
    }
    MathData* EnsureMathData() {
      return EnsureData<MathData>(&math_data, kMathData);
    }
    const MathData* GetMathData() const {
      return GetData<MathData>(&math_data, kMathData);
    }
    TableData* EnsureTableData() {
      return EnsureData<TableData>(&table_data, kTableData);
    }
    const TableData* GetTableData() const {
      return GetData<TableData>(&table_data, kTableData);
    }

    RareData() : bit_field(DataUnionTypeValue::encode(kNone)) {}

    RareData(const RareData& rare_data)
        : early_break(rare_data.early_break),
          column_spanner_path(rare_data.column_spanner_path),
          end_margin_strut(rare_data.end_margin_strut),
          // This will initialize "both" members of the union.
          tallest_unbreakable_block_size(
              rare_data.tallest_unbreakable_block_size),
          block_size_for_fragmentation(rare_data.block_size_for_fragmentation),
          exclusion_space(rare_data.exclusion_space),
          custom_layout_data(rare_data.custom_layout_data),
          annotation_overflow(rare_data.annotation_overflow),
          block_end_annotation_space(rare_data.block_end_annotation_space),
          lines_until_clamp(rare_data.lines_until_clamp),
          line_box_bfc_block_offset(rare_data.line_box_bfc_block_offset),
          non_overflowing_scroll_ranges(
              rare_data.non_overflowing_scroll_ranges),
          oof_positioned_offset(rare_data.oof_positioned_offset),
          bit_field(rare_data.bit_field) {
      switch (data_union_type()) {
        case kNone:
          break;
        case kFlexData:
          new (&flex_data) FlexData(rare_data.flex_data);
          break;
        case kGridData:
          new (&grid_data) GridData(rare_data.grid_data);
          break;
        case kLineSmallData:
          new (&line_small_data) LineSmallData(rare_data.line_small_data);
          break;
        case kLineData:
          new (&line_data) LineDataPtr(rare_data.line_data);
          break;
        case kMathData:
          new (&math_data) MathData(rare_data.math_data);
          break;
        case kTableData:
          new (&table_data) TableData(rare_data.table_data);
          break;
        default:
          NOTREACHED();
      }
    }

    ~RareData() {
      switch (data_union_type()) {
        case kNone:
          break;
        case kFlexData:
          flex_data.~FlexData();
          break;
        case kGridData:
          grid_data.~GridData();
          break;
        case kLineSmallData:
          line_small_data.~LineSmallData();
          break;
        case kLineData:
          line_data.~LineDataPtr();
          break;
        case kMathData:
          math_data.~MathData();
          break;
        case kTableData:
          table_data.~TableData();
          break;
        default:
          NOTREACHED();
      }
    }

    void SetLineBoxBfcBlockOffset(LayoutUnit offset) {
      line_box_bfc_block_offset = offset;
      set_line_box_bfc_block_offset_is_set(true);
    }
    std::optional<LayoutUnit> LineBoxBfcBlockOffset() const {
      if (!line_box_bfc_block_offset_is_set())
        return std::nullopt;
      return line_box_bfc_block_offset;
    }

    std::optional<LayoutUnit> ClearanceAfterLine() const {
      const RareData::LineSmallData* data = GetLineSmallData();
      return data ? data->ClearanceAfterLine() : std::nullopt;
    }

    std::optional<LayoutUnit> TrimBlockEndBy() const {
      const RareData::LineSmallData* data = GetLineSmallData();
      return data ? data->TrimBlockEndBy() : std::nullopt;
    }

    void SetNonOverflowingScrollRanges(
        const HeapVector<NonOverflowingScrollRange>& non_overflowing_ranges) {
      non_overflowing_scroll_ranges = non_overflowing_ranges;
    }
    const HeapVector<NonOverflowingScrollRange>* NonOverflowingScrollRanges()
        const {
      if (non_overflowing_scroll_ranges.empty()) {
        return nullptr;
      }
      return &non_overflowing_scroll_ranges;
    }

    void SetOutOfFlowPositionedOffset(const LogicalOffset& offset) {
      oof_positioned_offset = offset;
      set_oof_positioned_offset_is_set(true);
    }
    LogicalOffset OutOfFlowPositionedOffset() const {
      CHECK(oof_positioned_offset_is_set());
      return oof_positioned_offset;
    }

    void Trace(Visitor* visitor) const;

    Member<const EarlyBreak> early_break;
    Member<const ColumnSpannerPath> column_spanner_path;
    MarginStrut end_margin_strut;
    union {
      // Only set in the initial column balancing layout pass, when we have no
      // clue what the column block-size is going to be.
      LayoutUnit tallest_unbreakable_block_size;

      // Only set in subsequent column balancing passes, when we have set a
      // tentative column block-size. At every column boundary we'll record
      // space shortage, and store the smallest one here. If the columns
      // couldn't fit all the content, and we're allowed to stretch columns
      // further, we'll perform another pass with the column block-size
      // increased by this amount.
      LayoutUnit minimal_space_shortage = kIndefiniteSize;
    };
    LayoutUnit block_size_for_fragmentation = kIndefiniteSize;
    ExclusionSpace exclusion_space;
    scoped_refptr<SerializedScriptValue> custom_layout_data;

    LayoutUnit annotation_overflow;
    LayoutUnit block_end_annotation_space;
    int lines_until_clamp;
    Member<Element> accessibility_anchor;
    Member<GCedHeapHashSet<Member<Element>>> display_locks_affected_by_anchors;

   private:
    // Only valid if line_box_bfc_block_offset_is_set
    LayoutUnit line_box_bfc_block_offset;

    HeapVector<NonOverflowingScrollRange> non_overflowing_scroll_ranges;

    // Only valid if oof_positioned_offset_is_set
    LogicalOffset oof_positioned_offset;

    BitField bit_field;

    union {
      FlexData flex_data;
      GridData grid_data;
      LineSmallData line_small_data;
      LineDataPtr line_data;
      MathData math_data;
      TableData table_data;
    };
  };
  RareData* EnsureRareData();

#if DCHECK_IS_ON()
  void AssertSoleBoxFragment() const;
#endif

  struct Bitfields {
    DISALLOW_NEW();

   public:
    Bitfields()
        : Bitfields(
              /* is_self_collapsing */ false,
              /* is_pushed_by_floats */ false,
              /* adjoining_object_types */ kAdjoiningNone,
              /* has_descendant_that_depends_on_percentage_block_size */
              false,
              /* subtree_modified_margin_strut */ false) {}
    Bitfields(bool is_self_collapsing,
              bool is_pushed_by_floats,
              AdjoiningObjectTypes adjoining_object_types,
              bool has_descendant_that_depends_on_percentage_block_size,
              bool subtree_modified_margin_strut)
        : has_rare_data_exclusion_space(false),
          has_oof_insets_for_get_computed_style(false),
          is_bfc_block_offset_nullopt(false),
          has_forced_break(false),
          break_appeal(kBreakAppealPerfect),
          is_empty_spanner_parent(false),
          is_block_size_for_fragmentation_clamped(false),
          should_force_same_fragmentation_flow(false),
          is_self_collapsing(is_self_collapsing),
          is_pushed_by_floats(is_pushed_by_floats),
          adjoining_object_types(static_cast<unsigned>(adjoining_object_types)),
          is_initial_block_size_indefinite(false),
          has_descendant_that_depends_on_percentage_block_size(
              has_descendant_that_depends_on_percentage_block_size),
          subtree_modified_margin_strut(subtree_modified_margin_strut),
          initial_break_before(static_cast<unsigned>(EBreakBetween::kAuto)),
          final_break_after(static_cast<unsigned>(EBreakBetween::kAuto)),
          status(static_cast<unsigned>(kSuccess)),
          is_truncated_by_fragmentation_line(false),
          has_orthogonal_fallback_size_descendant(false) {}

    unsigned has_rare_data_exclusion_space : 1;
    unsigned has_oof_insets_for_get_computed_style : 1;
    unsigned is_bfc_block_offset_nullopt : 1;

    unsigned has_forced_break : 1;
    unsigned break_appeal : kBreakAppealBitsNeeded;
    unsigned is_empty_spanner_parent : 1;
    unsigned is_block_size_for_fragmentation_clamped : 1;
    unsigned should_force_same_fragmentation_flow : 1;

    unsigned is_self_collapsing : 1;
    unsigned is_pushed_by_floats : 1;
    unsigned adjoining_object_types : 3;  // AdjoiningObjectTypes

    unsigned is_initial_block_size_indefinite : 1;
    unsigned has_descendant_that_depends_on_percentage_block_size : 1;

    unsigned subtree_modified_margin_strut : 1;

    unsigned initial_break_before : 4;  // EBreakBetween
    unsigned final_break_after : 4;     // EBreakBetween

    unsigned status : 4;  // EStatus
    unsigned is_truncated_by_fragmentation_line : 1;
    unsigned has_orthogonal_fallback_size_descendant : 1;
  };

  // The constraint space which generated this layout result.
  const ConstraintSpace space_;

  Member<const PhysicalFragment> physical_fragment_;

  // |rare_data_| cannot be stored in the union because it is difficult to have
  // a const bitfield for it and it cannot be traced.
  // Note that |bfc_offset_| and |oof_insets_for_get_computed_style_| cannot be
  // both valid at the same time, because an OOF-positioned node's BFC offset is
  // *always* the initial value.
  Member<RareData> rare_data_;
  union {
    BfcOffset bfc_offset_;
    // This is the absolutized inset property values of an OOF-positioned object
    // in its parent's writing-mode. This is set by the |OutOfFlowLayoutPart|
    // while generating this layout result.
    BoxStrut oof_insets_for_get_computed_style_;
  };

  LayoutUnit intrinsic_block_size_;
  Bitfields bitfields_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_RESULT_H_
