// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_EXCLUSION_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_EXCLUSION_SPACE_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/bfc_offset.h"
#include "third_party/blink/renderer/core/layout/geometry/bfc_rect.h"
#include "third_party/blink/renderer/core/layout/exclusions/exclusion_area.h"
#include "third_party/blink/renderer/core/layout/exclusions/layout_opportunity.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

typedef HeapVector<LayoutOpportunity, 1> LayoutOpportunityVector;

// This class is an implementation detail. For use of the exclusion space,
// see ExclusionSpace below. ExclusionSpace was designed to be cheap
// to construct and cheap to copy if empty.
class CORE_EXPORT ExclusionSpaceInternal final {
  USING_FAST_MALLOC(ExclusionSpaceInternal);

 public:
  ExclusionSpaceInternal();
  ExclusionSpaceInternal(const ExclusionSpaceInternal&);
  ExclusionSpaceInternal(ExclusionSpaceInternal&&) noexcept;
  ExclusionSpaceInternal& operator=(const ExclusionSpaceInternal&);
  ExclusionSpaceInternal& operator=(ExclusionSpaceInternal&&) noexcept;
  // See `ExclusionSpace::CopyFrom()`.
  void CopyFrom(const ExclusionSpaceInternal&);
  ~ExclusionSpaceInternal() = default;

  void Add(const ExclusionArea* exclusion);

  LayoutOpportunity FindLayoutOpportunity(
      const BfcOffset& offset,
      const LayoutUnit available_inline_size,
      const LayoutUnit minimum_inline_size) const {
    const LayoutUnit max_clear_offset =
        std::max({left_clear_offset_, right_clear_offset_,
                  initial_letter_left_clear_offset_,
                  initial_letter_right_clear_offset_});
    // If the area clears all floats, we can just return the layout opportunity
    // which matches the available space.
    if (offset.block_offset >= max_clear_offset) {
      BfcOffset end_offset(
          offset.line_offset + available_inline_size.ClampNegativeToZero(),
          LayoutUnit::Max());
      return LayoutOpportunity(BfcRect(offset, end_offset), nullptr);
    }

    return GetDerivedGeometry(offset.block_offset)
        .FindLayoutOpportunity(offset, available_inline_size,
                               minimum_inline_size);
  }

  LayoutOpportunityVector AllLayoutOpportunities(
      const BfcOffset& offset,
      const LayoutUnit available_inline_size) const {
    const LayoutUnit max_clear_offset =
        std::max({left_clear_offset_, right_clear_offset_,
                  initial_letter_left_clear_offset_,
                  initial_letter_right_clear_offset_});
    // If the area clears all floats, we can just return a single layout
    // opportunity which matches the available space.
    if (offset.block_offset >= max_clear_offset) {
      BfcOffset end_offset(
          offset.line_offset + available_inline_size.ClampNegativeToZero(),
          LayoutUnit::Max());
      return LayoutOpportunityVector(
          {LayoutOpportunity(BfcRect(offset, end_offset), nullptr)});
    }

    return GetDerivedGeometry(offset.block_offset)
        .AllLayoutOpportunities(offset, available_inline_size);
  }

  LayoutUnit ClearanceOffset(EClear clear_type) const {
    switch (clear_type) {
      case EClear::kNone:
        return LayoutUnit::Min();
      case EClear::kLeft:
        return left_clear_offset_;
      case EClear::kRight:
        return right_clear_offset_;
      case EClear::kBoth:
        return std::max(left_clear_offset_, right_clear_offset_);
      default:
        NOTREACHED();
    }
  }

  LayoutUnit ClearanceOffsetIncludingInitialLetter(EClear clear_type) const {
    return std::max(ClearanceOffset(clear_type),
                    InitialLetterClearanceOffset(EClear::kBoth));
  }

  LayoutUnit InitialLetterClearanceOffset(EClear clear_type) const {
    switch (clear_type) {
      case EClear::kNone:
        return LayoutUnit::Min();
      case EClear::kLeft:
        return initial_letter_left_clear_offset_;
      case EClear::kRight:
        return initial_letter_right_clear_offset_;
      case EClear::kBoth:
        return std::max(initial_letter_left_clear_offset_,
                        initial_letter_right_clear_offset_);
      default:
        NOTREACHED();
    }
  }

  LayoutUnit InitialLetterClearanceOffset(EFloat float_type) const {
    if (float_type == EFloat::kLeft)
      return initial_letter_left_clear_offset_;
    DCHECK_EQ(float_type, EFloat::kRight);
    return initial_letter_right_clear_offset_;
  }

  LayoutUnit NonHiddenClearanceOffsetIncludingInitialLetter() const {
    return non_hidden_clear_offset_;
  }

  void SetHasBreakBeforeFloat(EFloat type) {
    switch (type) {
      default:
        NOTREACHED();
      case EFloat::kLeft:
        has_break_before_left_float_ = true;
        break;
      case EFloat::kRight:
        has_break_before_right_float_ = true;
        break;
    }
  }

  void SetHasBreakInsideFloat(EFloat type) {
    switch (type) {
      default:
        NOTREACHED();
      case EFloat::kLeft:
        has_break_inside_left_float_ = true;
        break;
      case EFloat::kRight:
        has_break_inside_right_float_ = true;
        break;
    }
  }

  bool NeedsClearancePastFragmentainer(EClear type) const {
    bool needs_clearance = false;
    switch (type) {
      default:
        NOTREACHED();
      case EClear::kNone:
        return false;
      case EClear::kLeft:
      case EClear::kBoth:
        needs_clearance |=
            has_break_inside_left_float_ || has_break_before_left_float_;
        if (type == EClear::kLeft)
          break;
        [[fallthrough]];
      case EClear::kRight:
        needs_clearance |=
            has_break_inside_right_float_ || has_break_before_right_float_;
        break;
    }
    return needs_clearance;
  }

  bool NeedsBreakBeforeFloat(EClear type) {
    // Floats cannot start above any preceding floats, so if any float has been
    // pushed to the next fragmentainer, so will this one.
    if (has_break_before_left_float_ || has_break_before_right_float_)
      return true;
    return NeedsClearancePastFragmentainer(type);
  }

  bool HasFragmentainerBreak() const {
    return NeedsClearancePastFragmentainer(EClear::kBoth);
  }

  LayoutUnit LastFloatBlockStart() const { return last_float_block_start_; }

  bool IsEmpty() const { return !num_exclusions_; }

  // Pre-initializes the exclusions vector to something used in a previous
  // layout pass, however keeps the number of exclusions as zero.
  void PreInitialize(const ExclusionSpaceInternal& other) {
    DCHECK(exclusions_->empty());
    DCHECK_GT(other.exclusions_->size(), 0u);

    exclusions_ = other.exclusions_;
  }

  // See |ExclusionSpace::MoveAndUpdateDerivedGeometry|.
  void MoveAndUpdateDerivedGeometry(const ExclusionSpaceInternal& other) {
    if (!other.derived_geometry_)
      return;

    MoveDerivedGeometry(other);

    // Iterate through all the exclusions which were added by the layout, and
    // update the DerivedGeometry.
    for (wtf_size_t i = other.num_exclusions_; i < num_exclusions_; ++i) {
      const ExclusionArea& exclusion = *exclusions_->at(i);

      // If we come across an exclusion with shape data, we opt-out of this
      // optimization.
      if (!track_shape_exclusions_ && exclusion.shape_data) {
        track_shape_exclusions_ = true;
        derived_geometry_ = nullptr;
        return;
      }

      derived_geometry_->Add(exclusion);
    }
  }

  // See |ExclusionSpace::MoveDerivedGeometry|.
  void MoveDerivedGeometry(const ExclusionSpaceInternal& other) {
    if (!other.derived_geometry_)
      return;

    track_shape_exclusions_ = other.track_shape_exclusions_;
    derived_geometry_ = std::move(other.derived_geometry_);
    other.derived_geometry_ = nullptr;
  }

  // See |ExclusionSpace::MergeExclusionSpaces|.
  void MergeExclusionSpaces(const BfcDelta& offset_delta,
                            const ExclusionSpaceInternal& previous_output,
                            const ExclusionSpaceInternal* previous_input) {
    // We need to copy all the exclusions over which were added by the cached
    // layout result.
    for (wtf_size_t i = previous_input ? previous_input->num_exclusions_ : 0;
         i < previous_output.num_exclusions_; ++i) {
      Add(previous_output.exclusions_->at(i)->CopyWithOffset(offset_delta));
    }
  }

  bool operator==(const ExclusionSpaceInternal& other) const;
  bool operator!=(const ExclusionSpaceInternal& other) const {
    return !(*this == other);
  }

#if DCHECK_IS_ON()
  void CheckSameForSimplifiedLayout(const ExclusionSpaceInternal& other) const {
    DCHECK_EQ(num_exclusions_, other.num_exclusions_);
    for (wtf_size_t i = 0; i < num_exclusions_; ++i) {
      const auto& exclusion = *exclusions_->at(i);
      const auto& other_exclusion = *other.exclusions_->at(i);
      DCHECK(exclusion.rect == other_exclusion.rect);
      DCHECK_EQ(exclusion.type, other_exclusion.type);
      DCHECK_EQ((bool)exclusion.shape_data, (bool)other_exclusion.shape_data);
    }
  }
#endif

  // This struct represents the side of a float against the "edge" of a shelf.
  struct ShelfEdge {
    ShelfEdge(LayoutUnit block_start, LayoutUnit block_end)
        : block_start(block_start), block_end(block_end) {}

    LayoutUnit block_start;
    LayoutUnit block_end;
  };

  // The shelf is an internal data-structure representing the bottom of a
  // float. A shelf has a inline-size which is defined by the line_left and
  // line_right members. E.g.
  //
  //    0 1 2 3 4 5 6 7 8
  // 0  +---++--+    +---+
  //    |xxx||xx|    |xxx|
  // 10 |xxx|X-------Xxxx|
  //    +---+        +---+
  // 20
  //
  // In the above diagram the shelf is at the block-end edge of the smallest
  // float. It would have the internal values of:
  // {
  //   block_offset: 10,
  //   line_left: 20,
  //   line_right: 65,
  //   line_left_edges: [{0, 15}],
  //   line_right_edges: [{0, 15}],
  // }
  // The line_left_edges and line_right_edges are all the floats which are
  // "against" the shelf at the line_left and line_right offset respectively.
  //
  // An opportunity has a "solid" edge if there is at least one float adjacent
  // to the line-left or line-right edge. If an opportunity has no adjacent
  // floats it is invalid.
  //
  // These are used for:
  //  - When we create an opportunity, making sure it has "solid" edges.
  //  - The opportunity also holds onto a list of these edges to support
  //    css-shapes.
  struct Shelf final {
    DISALLOW_NEW();

   public:
    Shelf(LayoutUnit block_offset, bool track_shape_exclusions)
        : block_offset(block_offset),
          line_left(LayoutUnit::Min()),
          line_right(LayoutUnit::Max()),
          shape_exclusions(track_shape_exclusions
                               ? MakeGarbageCollected<ShapeExclusions>()
                               : nullptr),
          has_shape_exclusions(false) {}

    // The copy constructor explicitly copies the shape_exclusions member.
    Shelf(const Shelf& other)
        : block_offset(other.block_offset),
          line_left(other.line_left),
          line_right(other.line_right),
          line_left_edges(other.line_left_edges),
          line_right_edges(other.line_right_edges),
          shape_exclusions(other.shape_exclusions
                               ? MakeGarbageCollected<ShapeExclusions>(
                                     *other.shape_exclusions)
                               : nullptr),
          has_shape_exclusions(other.has_shape_exclusions) {}

    void Trace(Visitor* visitor) const { visitor->Trace(shape_exclusions); }

    LayoutUnit block_offset;
    LayoutUnit line_left;
    LayoutUnit line_right;

    // TODO(crbug.com/1195345): restore inline buffer removed in
    // https://crrev.com/c/2801713
    Vector<ShelfEdge> line_left_edges;
    Vector<ShelfEdge> line_right_edges;

    // shape_exclusions contains all the floats which sit below this shelf. The
    // has_shape_exclusions member will be true if shape_exclusions contains an
    // exclusion with shape-outside specified (and therefore should be copied
    // to any layout opportunity).
    Member<ShapeExclusions> shape_exclusions;
    bool has_shape_exclusions;
  };

  // The closed-off area is an internal data-structure representing an area
  // above a float. It contains a layout opportunity, and two vectors of
  // |ShelfEdge|. E.g.
  //
  //    0 1 2 3 4 5 6 7 8
  // 0  +---+.      .+---+
  //    |xxx|.      .|xxx|
  // 10 |xxx|.      .|xxx|
  //    +---+.      .+---+
  // 20      ........
  //      +---+
  // 30   |xxx|
  //      |xxx|
  // 40   +---+
  //
  // In the above example the closed-off area is represented with the dotted
  // line.
  //
  // It has the internal values of:
  // {
  //   opportunity: {
  //     start_offset: {20, LayoutUnit::Min()},
  //     end_offset: {65, 25},
  //   }
  //   line_left_edges: [{0, 15}],
  //   line_right_edges: [{0, 15}],
  // }
  //
  // Once a closed-off area has been created, it can never be changed due to
  // the property that floats always align their block-start edges.
  struct ClosedArea {
    DISALLOW_NEW();

   public:
    ClosedArea(LayoutOpportunity opportunity,
               const Vector<ShelfEdge>& line_left_edges,
               const Vector<ShelfEdge>& line_right_edges)
        : opportunity(opportunity),
          line_left_edges(line_left_edges),
          line_right_edges(line_right_edges) {}

    void Trace(Visitor* visitor) const { visitor->Trace(opportunity); }

    const LayoutOpportunity opportunity;

    // TODO(crbug.com/1195345): restore inline buffer removed in
    // https://crrev.com/c/2801713
    const Vector<ShelfEdge> line_left_edges;
    const Vector<ShelfEdge> line_right_edges;
  };

 private:
  // In order to reduce the amount of Vector copies, instances of a
  // ExclusionSpaceInternal can share the same exclusions_ Vector. See the
  // copy constructor.
  //
  // We implement a copy-on-write behaviour when adding an exclusion (if
  // exclusions_.size(), and num_exclusions_ differs).
  //
  // num_exclusions_ is how many exclusions *this* instance of an exclusion
  // space has, which may differ to the number of exclusions in the Vector.
  //
  // `exclusions_` contains `ExclusionArea` in ascent order of block start
  // offset.
  Persistent<GCedExclusionAreaPtrArray> exclusions_;
  wtf_size_t num_exclusions_ = 0;

  // These members are used for keeping track of the "lowest" offset for each
  // type of float. This is used for implementing float clearance.
  LayoutUnit left_clear_offset_ = LayoutUnit::Min();
  LayoutUnit right_clear_offset_ = LayoutUnit::Min();

  // This member is used for implementing the "top edge alignment rule" for
  // floats. Floats can be positioned at negative offsets, hence is initialized
  // the minimum value.
  LayoutUnit last_float_block_start_ = LayoutUnit::Min();

  // These member are used for keeping track of initial letter box offset.
  LayoutUnit initial_letter_left_clear_offset_ = LayoutUnit::Min();
  LayoutUnit initial_letter_right_clear_offset_ = LayoutUnit::Min();

  // The clear offset for both left and right, including both floats and initial
  // letters, for only content that isn't hidden for paint. Relevant for
  // line-clamp.
  LayoutUnit non_hidden_clear_offset_ = LayoutUnit::Min();

  // In order to reduce the amount of copies related to bookkeeping shape data,
  // we initially ignore exclusions with shape data. When we first see an
  // exclusion with shape data, we set this flag, and rebuild the
  // DerivedGeometry data-structure, to perform the additional bookkeeping.
  unsigned track_shape_exclusions_ : 1;

  // Set to true if we have found a left/right float that needs to start in the
  // next fragmentainer (e.g. because it has monolithic content or has break
  // avoidance requests inside).
  unsigned has_break_before_left_float_ : 1;
  unsigned has_break_before_right_float_ : 1;

  // Set to true if we have added a left/right float that will be resumed in the
  // next fragmentainer.
  unsigned has_break_inside_left_float_ : 1;
  unsigned has_break_inside_right_float_ : 1;

  // The derived geometry struct, is the data-structure which handles all of the
  // queries on the exclusion space. It can always be rebuilt from exclusions_
  // and num_exclusions_. This is mutable as it is passed down a chain of
  // exclusion spaces inside the copy constructor. E.g.
  //
  // ExclusionSpace space1;
  // space1.Add(exclusion1);
  // space1.FindLayoutOpportunity(); // Builds derived_geometry_.
  //
  // ExclusionSpace space2(space1); // Moves derived_geometry_ to space2.
  // space2.Add(exclusion2); // Modifies derived_geometry_.
  //
  // space1.FindLayoutOpportunity(); // Re-builds derived_geometry_.
  //
  // This is efficient (desirable) as the common usage pattern is only the last
  // exclusion space in the copy-chain is used for answering queries. Only when
  // we trigger a (rare) re-layout case will we need to rebuild the
  // derived_geometry_ data-structure.
  struct CORE_EXPORT DerivedGeometry final
      : public GarbageCollected<DerivedGeometry> {
   public:
    // |block_offset_limit| represents the highest block-offset for which the
    // geometry is valid. |FindLayoutOpportunity| and |AllLayoutOpportunities|
    // should not be called for a block-offset higher than this.
    // If |ExclusionSpaceInternal::GetDerivedGeometry| is called with a
    // higher limit the geometry is rebuilt.
    //
    // |track_shape_exclusions| is used to tell the geometry to track shape
    // exclusions. Tracking shape exclusions is expensive, and uncommon, so
    // when an exclusion with a shape is added we rebuilt the geometry to track
    // this.
    DerivedGeometry(LayoutUnit block_offset_limit, bool track_shape_exclusions);

    void Add(const ExclusionArea& exclusion);

    LayoutOpportunity FindLayoutOpportunity(
        const BfcOffset& offset,
        const LayoutUnit available_inline_size,
        const LayoutUnit minimum_inline_size) const;

    LayoutOpportunityVector AllLayoutOpportunities(
        const BfcOffset& offset,
        const LayoutUnit available_inline_size) const;

    template <typename LambdaFunc>
    void IterateAllLayoutOpportunities(const BfcOffset& offset,
                                       const LayoutUnit available_inline_size,
                                       const LambdaFunc&) const;

    void Trace(Visitor* visitor) const {
      visitor->Trace(shelves_);
      visitor->Trace(areas_);
    }

    // See |Shelf| for a broad description of what shelves are. We always
    // begin with one, which has the internal value of:
    // {
    //   block_offset: LayoutUnit::Min(),
    //   line_left: LayoutUnit::Min(),
    //   line_right: LayoutUnit::Max(),
    // }
    //
    HeapVector<Shelf, 4> shelves_;

    // See |ClosedArea| for a broad description of what closed-off areas are.
    //
    // Floats always align their block-start edges. We exploit this property by
    // keeping a list of closed-off areas. Once a closed-off area has been
    // created, it can never change.
    HeapVector<ClosedArea, 4> areas_;

    // This represents the highest block-offset for which the geometry is valid
    // for. If |ExclusionSpaceInternal::GetDerivedGeometry| is called with a
    // higher limit it is rebuilt.
    LayoutUnit block_offset_limit_;

    bool track_shape_exclusions_;
  };

  // Returns the derived_geometry_ member, potentially re-built from the
  // exclusions_, and num_exclusions_ members.
  const DerivedGeometry& GetDerivedGeometry(
      LayoutUnit block_offset_limit) const;

  // See DerivedGeometry struct description.
  mutable Persistent<DerivedGeometry> derived_geometry_;
};

// The exclusion space represents all of the exclusions within a block
// formatting context.
//
// The space is mutated simply by adding exclusions, and various information
// can be queried based on the exclusions.
class CORE_EXPORT ExclusionSpace {
  DISALLOW_NEW();

 public:
  ExclusionSpace() = default;
  ExclusionSpace(const ExclusionSpace& other)
      : exclusion_space_(other.exclusion_space_
                             ? std::make_unique<ExclusionSpaceInternal>(
                                   *other.exclusion_space_)
                             : nullptr) {}
  ExclusionSpace(ExclusionSpace&& other) noexcept = default;

  // This moves the cached `derived_geometry_`, see also `CopyFrom()`.
  ExclusionSpace& operator=(const ExclusionSpace& other) {
    exclusion_space_ =
        other.exclusion_space_
            ? std::make_unique<ExclusionSpaceInternal>(*other.exclusion_space_)
            : nullptr;
    return *this;
  }
  ExclusionSpace& operator=(ExclusionSpace&& other) = default;
  // Same as `operator=`, except that `operator=` moves the cached
  // `derived_geometry_` for when the copied instance is more likely to be used,
  // while `CopyFrom` doesn't.
  void CopyFrom(const ExclusionSpace&);

  void Add(const ExclusionArea* exclusion) {
    if (!exclusion_space_)
      exclusion_space_ = std::make_unique<ExclusionSpaceInternal>();
    exclusion_space_->Add(std::move(exclusion));
  }

  void SetHasBreakBeforeFloat(EFloat type) {
    DCHECK(exclusion_space_);
    exclusion_space_->SetHasBreakBeforeFloat(type);
  }

  void SetHasBreakInsideFloat(EFloat type) {
    DCHECK(exclusion_space_);
    exclusion_space_->SetHasBreakInsideFloat(type);
  }

  // Return true if an in-flow node (i.e. not another float, for instance) with
  // the given 'clear' property needs clearance past the current fragmentainer.
  bool NeedsClearancePastFragmentainer(EClear type) const {
    return exclusion_space_ &&
           exclusion_space_->NeedsClearancePastFragmentainer(type);
  }

  // Return true if a float with the given 'clear' property needs to be pushed
  // past the current fragmentainer, either because of clearance, or because we
  // already have floats that have been pushed.
  bool NeedsBreakBeforeFloat(EClear type) const {
    return exclusion_space_ && exclusion_space_->NeedsBreakBeforeFloat(type);
  }

  bool HasFragmentainerBreak() const {
    return exclusion_space_ && exclusion_space_->HasFragmentainerBreak();
  }

  // Returns a layout opportunity, within the BFC.
  // The area to search for layout opportunities is defined by the given offset,
  // and |available_inline_size|. The layout opportunity must be greater than
  // the given |minimum_inline_size|.
  LayoutOpportunity FindLayoutOpportunity(
      const BfcOffset& offset,
      const LayoutUnit available_inline_size,
      const LayoutUnit minimum_inline_size = LayoutUnit()) const {
    if (!exclusion_space_) {
      BfcOffset end_offset(
          offset.line_offset + available_inline_size.ClampNegativeToZero(),
          LayoutUnit::Max());
      return LayoutOpportunity(BfcRect(offset, end_offset), nullptr);
    }
    return exclusion_space_->FindLayoutOpportunity(
        offset, available_inline_size, minimum_inline_size);
  }

  // If possible prefer FindLayoutOpportunity over this function.
  LayoutOpportunityVector AllLayoutOpportunities(
      const BfcOffset& offset,
      const LayoutUnit available_inline_size) const {
    if (!exclusion_space_) {
      BfcOffset end_offset(
          offset.line_offset + available_inline_size.ClampNegativeToZero(),
          LayoutUnit::Max());
      return LayoutOpportunityVector(
          {LayoutOpportunity(BfcRect(offset, end_offset), nullptr)});
    }
    return exclusion_space_->AllLayoutOpportunities(offset,
                                                    available_inline_size);
  }

  LayoutUnit ClearanceOffset(EClear clear_type) const {
    if (!exclusion_space_)
      return LayoutUnit::Min();
    return exclusion_space_->ClearanceOffset(clear_type);
  }

  LayoutUnit ClearanceOffsetIncludingInitialLetter(EClear clear_type) const {
    if (!exclusion_space_)
      return LayoutUnit::Min();
    return exclusion_space_->ClearanceOffsetIncludingInitialLetter(clear_type);
  }

  LayoutUnit NonHiddenClearanceOffsetIncludingInitialLetter() const {
    if (!exclusion_space_) {
      return LayoutUnit::Min();
    }
    return exclusion_space_->NonHiddenClearanceOffsetIncludingInitialLetter();
  }

  LayoutUnit InitialLetterClearanceOffset(EClear clear_type) const {
    if (!exclusion_space_)
      return LayoutUnit::Min();
    return exclusion_space_->InitialLetterClearanceOffset(clear_type);
  }

  // Returns the initial letter clearance offset based on the provided
  // {@code float_type}.
  LayoutUnit InitialLetterClearanceOffset(EFloat float_type) const {
    if (!exclusion_space_)
      return LayoutUnit::Min();
    return exclusion_space_->InitialLetterClearanceOffset(float_type);
  }

  // Returns the block start offset of the last float added.
  LayoutUnit LastFloatBlockStart() const {
    if (!exclusion_space_)
      return LayoutUnit::Min();
    return exclusion_space_->LastFloatBlockStart();
  }

  bool IsEmpty() const {
    return !exclusion_space_ || exclusion_space_->IsEmpty();
  }

  // See |ExclusionSpaceInternal::PreInitialize|.
  void PreInitialize(const ExclusionSpace& other) const {
    // Don't pre-initialize if we've already got an exclusions vector.
    if (exclusion_space_)
      return;

    // Don't pre-initialize if the other exclusion space didn't have an
    // exclusions vector.
    if (!other.exclusion_space_)
      return;

    exclusion_space_ = std::make_unique<ExclusionSpaceInternal>();
    exclusion_space_->PreInitialize(*other.exclusion_space_);
  }

  // Shifts the |DerivedGeometry| data-structure to this exclusion space, and
  // adds any new exclusions.
  void MoveAndUpdateDerivedGeometry(const ExclusionSpace& other) const {
    if (!exclusion_space_ || !other.exclusion_space_)
      return;

    exclusion_space_->MoveAndUpdateDerivedGeometry(*other.exclusion_space_);
  }

  // Shifts the |DerivedGeometry| data-structure to this exclusion space.
  void MoveDerivedGeometry(const ExclusionSpace& other) const {
    DCHECK(*this == other);
    if (!exclusion_space_ || !other.exclusion_space_)
      return;

    exclusion_space_->MoveDerivedGeometry(*other.exclusion_space_);
  }

  // This produces a new exclusion space for a |LayoutResult| which is being
  // re-used for caching purposes.
  //
  // It takes:
  //  - |old_output| The exclusion space associated with the cached layout
  //    result (the output of layout).
  //  - |old_input| The exclusion space which produced the cached layout result
  //    (the input into layout).
  //  - |new_input| The exclusion space which is being used to produce a new
  //    layout result (the new input into layout).
  //  - |offset_delta| the amount that the layout result was moved in BFC
  //    coordinate space.
  //
  // |old_output| should contain the *at least* same exclusions as |old_input|
  // however may have added some more exclusions during its layout.
  //
  // This function takes those exclusions added by the cached layout-result
  // (the difference between |old_output| and |old_input|), and adds them to
  // |new_input|. It will additionally shift them by |offset_delta|.
  //
  // This produces the correct exclusion space "new_output" for the new reused
  // layout result.
  static ExclusionSpace MergeExclusionSpaces(const ExclusionSpace& old_output,
                                             const ExclusionSpace& old_input,
                                             const ExclusionSpace& new_input,
                                             const BfcDelta& offset_delta) {
    // We start building the new exclusion space from the new input, this
    // (should) have the derived geometry which will move to |new_output|.
    ExclusionSpace new_output = new_input;

    // If we didn't have any floats previously, we don't need to add any new
    // ones, just return the new output.
    if (!old_output.exclusion_space_)
      return new_output;

    // If the layout didn't add any new exclusions, we can just return the new
    // output.
    if (old_input == old_output)
      return new_output;

    if (!new_output.exclusion_space_) {
      new_output.exclusion_space_ = std::make_unique<ExclusionSpaceInternal>();
    }

    new_output.exclusion_space_->MergeExclusionSpaces(
        offset_delta, *old_output.exclusion_space_,
        old_input.exclusion_space_.get());

    return new_output;
  }

  bool operator==(const ExclusionSpace& other) const {
    if (exclusion_space_ == other.exclusion_space_)
      return true;
    if (exclusion_space_ && other.exclusion_space_)
      return *exclusion_space_ == *other.exclusion_space_;
    return false;
  }
  bool operator!=(const ExclusionSpace& other) const {
    return !(*this == other);
  }

#if DCHECK_IS_ON()
  void CheckSameForSimplifiedLayout(const ExclusionSpace& other) const {
    DCHECK_EQ((bool)exclusion_space_, (bool)other.exclusion_space_);
    if (exclusion_space_)
      exclusion_space_->CheckSameForSimplifiedLayout(*other.exclusion_space_);
  }
#endif

 private:
  mutable std::unique_ptr<ExclusionSpaceInternal> exclusion_space_;
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::ExclusionSpaceInternal::ShelfEdge)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::ExclusionSpaceInternal::Shelf)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::ExclusionSpaceInternal::ClosedArea)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_EXCLUSION_SPACE_H_
