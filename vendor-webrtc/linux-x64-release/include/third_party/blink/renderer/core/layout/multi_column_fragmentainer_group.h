// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MULTI_COLUMN_FRAGMENTAINER_GROUP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MULTI_COLUMN_FRAGMENTAINER_GROUP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_multi_column_flow_thread.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

struct LogicalRect;

// A group of columns, that are laid out in the inline progression direction,
// all with the same column height.
//
// When a multicol container is inside another fragmentation context, and said
// multicol container lives in multiple outer fragmentainers (pages / columns),
// we need to put these inner columns into separate groups, with one group per
// outer fragmentainer. Such a group of columns is what comprises a "row of
// column boxes" in spec lingo.
//
// Column balancing, when enabled, takes place within a column fragmentainer
// group.
//
// Each fragmentainer group may have its own actual column count (if there are
// unused columns because of forced breaks, for example). If there are multiple
// fragmentainer groups, the actual column count must not exceed the used column
// count (the one calculated based on column-count and column-width from CSS),
// or they'd overflow the outer fragmentainer in the inline direction. If we
// need more columns than what a group has room for, we'll create another group
// and put them there (and make them appear in the next outer fragmentainer).
class CORE_EXPORT MultiColumnFragmentainerGroup {
  DISALLOW_NEW();

 public:
  explicit MultiColumnFragmentainerGroup(const LayoutMultiColumnSet&);

  // Position within the LayoutMultiColumnSet.
  LayoutUnit LogicalTop() const { return logical_top_; }
  void SetLogicalTop(LayoutUnit logical_top) { logical_top_ = logical_top; }

  // Return the amount of block space that this fragmentainer group takes up in
  // its containing LayoutMultiColumnSet.
  LayoutUnit GroupLogicalHeight() const {
    DCHECK(IsLogicalHeightKnown());
    return logical_height_;
  }

  // Return the block size of a column (or fragmentainer) in this fragmentainer
  // group. The spec says that this value must always be >= 1px, to ensure
  // progress.
  LayoutUnit ColumnLogicalHeight() const {
    DCHECK(IsLogicalHeightKnown());
    return std::max(LayoutUnit(1), logical_height_);
  }

  // Return whether we have some column height to work with. This doesn't have
  // to be the final height. It will only return false in the first layout pass,
  // and even then only if column height is auto and there's no way to even make
  // a guess (i.e. when there are no usable constraints).
  bool IsLogicalHeightKnown() const { return is_logical_height_known_; }

  LogicalOffset OffsetFromColumnSet() const;

  // The top of our flow thread portion
  LayoutUnit LogicalTopInFlowThread() const {
    return logical_top_in_flow_thread_;
  }
  void SetLogicalTopInFlowThread(LayoutUnit logical_top_in_flow_thread) {
    logical_top_in_flow_thread_ = logical_top_in_flow_thread;
  }

  // The bottom of our flow thread portion
  LayoutUnit LogicalBottomInFlowThread() const {
    return logical_bottom_in_flow_thread_;
  }
  void SetLogicalBottomInFlowThread(LayoutUnit logical_bottom_in_flow_thread) {
    logical_bottom_in_flow_thread_ = logical_bottom_in_flow_thread;
  }
  void ExtendLogicalBottomInFlowThread(LayoutUnit block_size) {
    logical_bottom_in_flow_thread_ += block_size;
  }

  // The height of the flow thread portion for the entire fragmentainer group.
  LayoutUnit LogicalHeightInFlowThread() const {
    // Due to negative margins, logical bottom may actually end up above logical
    // top, but we never want to return negative logical heights.
    return (logical_bottom_in_flow_thread_ - logical_top_in_flow_thread_)
        .ClampNegativeToZero();
  }
  // The height of the flow thread portion for the specified fragmentainer.
  // The last fragmentainer may not be using all available space.
  LayoutUnit LogicalHeightInFlowThreadAt(unsigned column_index) const;

  void ResetColumnHeight();

  PhysicalOffset FlowThreadTranslationAtOffset(
      LayoutUnit,
      LayoutBox::PageBoundaryRule) const;

  LogicalOffset VisualPointToFlowThreadPoint(
      const LogicalOffset& visual_point) const;

  PhysicalRect FragmentsBoundingBox(
      const PhysicalRect& bounding_box_in_flow_thread) const;

  PhysicalRect FlowThreadPortionRectAt(unsigned column_index) const;

  PhysicalRect FlowThreadPortionOverflowRectAt(unsigned column_index) const;

  // Get the first and the last column intersecting the specified block range.
  // Note that |logicalBottomInFlowThread| is an exclusive endpoint.
  void ColumnIntervalForBlockRangeInFlowThread(
      LayoutUnit logical_top_in_flow_thread,
      LayoutUnit logical_bottom_in_flow_thread,
      unsigned& first_column,
      unsigned& last_column) const;

  unsigned ColumnIndexAtOffset(LayoutUnit offset_in_flow_thread,
                               LayoutBox::PageBoundaryRule) const;

  // Like ColumnIndexAtOffset(), but with the return value clamped to actual
  // column count. While there are legitimate reasons for dealing with columns
  // out of bounds during layout, this should not happen when performing read
  // operations on the tree (like painting and hit-testing).
  unsigned ConstrainedColumnIndexAtOffset(LayoutUnit offset_in_flow_thread,
                                          LayoutBox::PageBoundaryRule) const;

  // The "CSS actual" value of column-count. This includes overflowing columns,
  // if any.
  // Returns 1 or greater, never 0.
  unsigned ActualColumnCount() const;

  void SetColumnBlockSizeFromNG(LayoutUnit);
  void ExtendColumnBlockSizeFromNG(LayoutUnit);

  void Trace(Visitor*) const;

 private:
  LogicalRect ColumnRectAt(unsigned column_index) const;
  LayoutUnit LogicalTopInFlowThreadAt(unsigned column_index) const {
    return logical_top_in_flow_thread_ + column_index * ColumnLogicalHeight();
  }
  LogicalRect LogicalFlowThreadPortionRectAt(unsigned column_index) const;
  // Return the column that the specified visual point belongs to. Only the
  // coordinate on the column progression axis is relevant. Every point belongs
  // to a column, even if said point is not inside any of the columns.
  unsigned ColumnIndexAtVisualPoint(const LogicalOffset& visual_point) const;

  unsigned UnclampedActualColumnCount() const;

  const Member<const LayoutMultiColumnSet> column_set_;

  LayoutUnit logical_top_;
  LayoutUnit logical_top_in_flow_thread_;
  LayoutUnit logical_bottom_in_flow_thread_;

  // Logical height of the group. This will also be the height of each column
  // in this group, with the difference that, while the logical height can be
  // 0, the height of a column must be >= 1px.
  LayoutUnit logical_height_;

  bool is_logical_height_known_ = false;
};

// List of all fragmentainer groups within a column set. There will always be at
// least one group. Deleting the one group is not allowed (or possible). There
// will be more than one group if the owning column set lives in multiple outer
// fragmentainers (e.g. multicol inside paged media).
class CORE_EXPORT MultiColumnFragmentainerGroupList {
  DISALLOW_NEW();

 public:
  explicit MultiColumnFragmentainerGroupList(LayoutMultiColumnSet&);
  ~MultiColumnFragmentainerGroupList();

  // Add an additional fragmentainer group to the end of the list, and return
  // it.
  MultiColumnFragmentainerGroup& AddExtraGroup();

  // Remove all fragmentainer groups but the first one.
  void DeleteExtraGroups();

  MultiColumnFragmentainerGroup& First() { return groups_.front(); }
  const MultiColumnFragmentainerGroup& First() const { return groups_.front(); }
  MultiColumnFragmentainerGroup& Last() { return groups_.back(); }
  const MultiColumnFragmentainerGroup& Last() const { return groups_.back(); }

  typedef HeapVector<MultiColumnFragmentainerGroup, 1>::iterator iterator;
  typedef HeapVector<MultiColumnFragmentainerGroup, 1>::const_iterator
      const_iterator;

  iterator begin() { return groups_.begin(); }
  const_iterator begin() const { return groups_.begin(); }
  iterator end() { return groups_.end(); }
  const_iterator end() const { return groups_.end(); }

  wtf_size_t size() const { return groups_.size(); }
  MultiColumnFragmentainerGroup& operator[](wtf_size_t i) {
    return groups_.at(i);
  }
  const MultiColumnFragmentainerGroup& operator[](wtf_size_t i) const {
    return groups_.at(i);
  }

  void Append(const MultiColumnFragmentainerGroup& group) {
    groups_.push_back(group);
  }
  void Shrink(wtf_size_t size) { groups_.Shrink(size); }

  void Trace(Visitor*) const;

 private:
  Member<LayoutMultiColumnSet> column_set_;

  HeapVector<MultiColumnFragmentainerGroup, 1> groups_;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::MultiColumnFragmentainerGroup)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MULTI_COLUMN_FRAGMENTAINER_GROUP_H_
