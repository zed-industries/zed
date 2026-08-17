/*
 * Copyright (C) 2012 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_MULTI_COLUMN_FLOW_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_MULTI_COLUMN_FLOW_THREAD_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_flow_thread.h"

namespace blink {

class LayoutMultiColumnSet;
class LayoutMultiColumnSpannerPlaceholder;

// Flow thread implementation for CSS multicol. This will be inserted as an
// anonymous child block of the actual multicol container (i.e. the
// LayoutBlockFlow whose style computes to non-auto column-count and/or
// column-width). LayoutMultiColumnFlowThread is the heart of the multicol
// implementation, and there is only one instance per multicol container. Child
// content of the multicol container is parented into the flow thread at the
// time of layoutObject insertion.
//
// Apart from this flow thread child, the multicol container will also have
// LayoutMultiColumnSet children, which are used to position the columns
// visually. The flow thread is in charge of layout, and, after having
// calculated the column width, it lays out content as if everything were in one
// tall single column, except that there will typically be some amount of blank
// space (also known as pagination struts) at the offsets where the actual
// column boundaries are. This way, content that needs to be preceded by a break
// will appear at the top of the next column. Content needs to be preceded by a
// break when there's a forced break or when the content is unbreakable and
// cannot fully fit in the same column as the preceding piece of content.
// Although a LayoutMultiColumnFlowThread is laid out, it does not take up any
// space in its container. It's the LayoutMultiColumnSet objects that take up
// the necessary amount of space, and make sure that the columns are painted and
// hit-tested correctly.
//
// If there is any column content inside the multicol container, we create a
// LayoutMultiColumnSet. We only need to create multiple sets if there are
// spanners (column-span:all) in the multicol container. When a spanner is
// inserted, content preceding it gets its own set, and content succeeding it
// will get another set. The spanner itself will also get its own placeholder
// between the sets (LayoutMultiColumnSpannerPlaceholder), so that it gets
// positioned and sized correctly. The column-span:all element is inside the
// flow thread, but its containing block is the multicol container.
//
// Some invariants for the layout tree structure for multicol:
// - A multicol container is always a LayoutBlockFlow
// - Every multicol container has one and only one LayoutMultiColumnFlowThread
// - All multicol DOM children and pseudo-elements associated with the multicol
//   container are reparented into the flow thread.
// - The LayoutMultiColumnFlowThread is the first child of the multicol
//   container.
// - A multicol container may only have LayoutMultiColumnFlowThread,
//   LayoutMultiColumnSet and LayoutMultiColumnSpannerPlaceholder children.
// - A LayoutMultiColumnSet may not be adjacent to another LayoutMultiColumnSet;
//   there are no use-cases for it, and there are also implementation
//   limitations behind this requirement.
// - The flow thread is not in the containing block chain for children that are
//   not to be laid out in columns. This means column spanners and absolutely
//   positioned children whose containing block is outside column content
// - Each spanner (column-span:all) establishes a
//   LayoutMultiColumnSpannerPlaceholder
//
// The width of the flow thread is the same as the column width. The width of a
// column set is the same as the content box width of the multicol container; in
// other words exactly enough to hold the number of columns to be used, stacked
// horizontally, plus column gaps between them.
//
// Since it's the first child of the multicol container, the flow thread is laid
// out first, albeit in a slightly special way, since it's not to take up any
// space in its ancestors. Afterwards, the column sets are laid out. Column sets
// get their height from the columns that they hold. In single column-row
// constrained height non-balancing cases without spanners this will simply be
// the same as the content height of the multicol container itself. In most
// other cases we'll have to calculate optimal column heights ourselves, though.
// This process is referred to as column balancing, and then we infer the column
// set height from the height of the flow thread portion occupied by each set.
//
// More on column balancing: the columns' height is unknown in the first layout
// pass when balancing. This means that we cannot insert any implicit (soft /
// unforced) breaks (and pagination struts) when laying out the contents of the
// flow thread. We'll just lay out everything in tall single strip. After the
// initial flow thread layout pass we can determine a tentative / minimal /
// initial column height. This is calculated by simply dividing the flow
// thread's height by the number of specified columns. In the layout pass that
// follows, we can insert breaks (and pagination struts) at column boundaries,
// since we now have a column height.
// It may very easily turn out that the calculated height wasn't enough, though.
// We'll notice this at end of layout. If we end up with too many columns (i.e.
// columns overflowing the multicol container), it wasn't enough. In this case
// we need to increase the column heights. We'll increase them by the lowest
// amount of space that could possibly affect where the breaks occur. We'll
// relayout (to find new break points and the new lowest amount of space
// increase that could affect where they occur, in case we need another round)
// until we've reached an acceptable height (where everything fits perfectly in
// the number of columns that we have specified). The rule of thumb is that we
// shouldn't have to perform more of such iterations than the number of columns
// that we have.
//
// For each layout iteration done for column balancing, the flow thread will
// need a deep layout if column heights changed in the previous pass, since
// column height changes may affect break points and pagination struts anywhere
// in the tree, and currently no way exists to do this in a more optimized
// manner.
//
// There's also some documentation online:
// https://www.chromium.org/developers/design-documents/multi-column-layout
class CORE_EXPORT LayoutMultiColumnFlowThread final : public LayoutFlowThread {
 public:
  ~LayoutMultiColumnFlowThread() override;
  void Trace(Visitor*) const override;

  static LayoutMultiColumnFlowThread* CreateAnonymous(
      Document&,
      const ComputedStyle& parent_style);

  bool IsLayoutMultiColumnFlowThread() const final {
    NOT_DESTROYED();
    return true;
  }

  LayoutBlockFlow* MultiColumnBlockFlow() const {
    NOT_DESTROYED();
    return To<LayoutBlockFlow>(Parent());
  }

  LayoutMultiColumnSet* FirstMultiColumnSet() const;
  LayoutMultiColumnSet* LastMultiColumnSet() const;

  // Return the first column set or spanner placeholder.
  LayoutBox* FirstMultiColumnBox() const {
    NOT_DESTROYED();
    return NextSiblingBox();
  }
  // Return the last column set or spanner placeholder.
  LayoutBox* LastMultiColumnBox() const {
    NOT_DESTROYED();
    LayoutBox* last_sibling_box = MultiColumnBlockFlow()->LastChildBox();
    // The flow thread is the first child of the multicol container. If the flow
    // thread is also the last child, it means that there are no siblings; i.e.
    // we have no column boxes.
    return last_sibling_box != this ? last_sibling_box : nullptr;
  }

  // Find the first set inside which the specified layoutObject (which is a
  // flowthread descendant) would be rendered.
  LayoutMultiColumnSet* MapDescendantToColumnSet(LayoutObject*) const;

  // Return the spanner placeholder that belongs to the spanner in the
  // containing block chain, if any. This includes the layoutObject for the
  // element that actually establishes the spanner too.
  LayoutMultiColumnSpannerPlaceholder* ContainingColumnSpannerPlaceholder(
      const LayoutObject* descendant) const;

  // Populate the flow thread with what's currently its siblings. Called when a
  // regular block becomes a multicol container.
  void Populate();

  // Empty the flow thread by moving everything to the parent. Remove all
  // multicol specific layoutObjects. Then destroy the flow thread. Called when
  // a multicol container becomes a regular block.
  void EvacuateAndDestroy();

  unsigned ColumnCount() const {
    NOT_DESTROYED();
    return column_count_;
  }

  PhysicalOffset ColumnOffset(const PhysicalOffset&) const final;

  bool IsPageLogicalHeightKnown() const final;

  PhysicalOffset FlowThreadTranslationAtOffset(LayoutUnit,
                                               PageBoundaryRule) const;
  PhysicalOffset FlowThreadTranslationAtPoint(
      const PhysicalOffset& flow_thread_point) const;

  PhysicalOffset VisualPointToFlowThreadPoint(
      const PhysicalOffset& visual_point) const final;

  LayoutMultiColumnSet* ColumnSetAtBlockOffset(LayoutUnit,
                                               PageBoundaryRule) const final;

  void ColumnRuleStyleDidChange();

  // Remove the spanner placeholder and return true if the specified object is
  // no longer a valid spanner.
  bool RemoveSpannerPlaceholderIfNoLongerValid(
      LayoutBox* spanner_object_in_flow_thread);

  LayoutMultiColumnFlowThread* EnclosingFlowThread(
      AncestorSearchConstraint = kIsolateUnbreakableContainers) const;

  void SetColumnCountFromNG(unsigned column_count);
  void FinishLayoutFromNG(LayoutUnit flow_thread_offset);

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutMultiColumnFlowThread";
  }

  // Note: We call this constructor only in |CreateAnonymous()|, but mark this
  // "public" for |MakeGarbageCollected<T>|.
  explicit LayoutMultiColumnFlowThread();

  DeprecatedLayoutPoint LocationInternal() const override;
  PhysicalSize Size() const override;

 private:
  void CreateAndInsertMultiColumnSet(LayoutBox* insert_before = nullptr);
  void CreateAndInsertSpannerPlaceholder(
      LayoutBox* spanner_object_in_flow_thread,
      LayoutObject* inserted_before_in_flow_thread);
  void DestroySpannerPlaceholder(LayoutMultiColumnSpannerPlaceholder*);
  bool DescendantIsValidColumnSpanner(LayoutObject* descendant) const;
  bool CanContainSpannerInParentFragmentationContext(const LayoutObject&) const;

  void AddColumnSetToThread(LayoutMultiColumnSet*) override;
  void WillBeRemovedFromTree() override;
  void FlowThreadDescendantWasInserted(LayoutObject*) final;
  void FlowThreadDescendantWillBeRemoved(LayoutObject*) final;
  void FlowThreadDescendantStyleWillChange(
      LayoutBoxModelObject*,
      StyleDifference,
      const ComputedStyle& new_style) override;
  void FlowThreadDescendantStyleDidChange(
      LayoutBoxModelObject*,
      StyleDifference,
      const ComputedStyle& old_style) override;
  void ToggleSpannersInSubtree(LayoutBoxModelObject*);
  void UpdateGeometry();

  // The last set we worked on. It's not to be used as the "current set". The
  // concept of a "current set" is difficult, since layout may jump back and
  // forth in the tree, due to wrong top location estimates (due to e.g. margin
  // collapsing), and possibly for other reasons.
  Member<LayoutMultiColumnSet> last_set_worked_on_;

#if DCHECK_IS_ON()
  // Used to check consistency between calls to
  // flowThreadDescendantStyleWillChange() and
  // flowThreadDescendantStyleDidChange().
  static const LayoutBoxModelObject* style_changed_object_;
#endif

  // The used value of column-count
  unsigned column_count_;

  bool all_columns_have_known_height_ = false;

  bool is_being_evacuated_;

  // Specifies whether the the descendant whose style is about to change could
  // contain spanners or not. The flag is set in
  // flowThreadDescendantStyleWillChange(), and then checked in
  // flowThreadDescendantStyleDidChange().
  static bool could_contain_spanners_;

  static bool toggle_spanners_if_needed_;
};

template <>
struct DowncastTraits<LayoutMultiColumnFlowThread> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutFlowThread() &&
           To<LayoutFlowThread>(object).IsLayoutMultiColumnFlowThread();
  }
  static bool AllowFrom(const LayoutFlowThread& flow_thread) {
    return flow_thread.IsLayoutMultiColumnFlowThread();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_MULTI_COLUMN_FLOW_THREAD_H_
