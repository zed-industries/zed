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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_MULTI_COLUMN_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_MULTI_COLUMN_SET_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_multi_column_flow_thread.h"
#include "third_party/blink/renderer/core/layout/multi_column_fragmentainer_group.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// A set of columns in a multicol container. A column set is inserted as an
// anonymous child of the actual multicol container (i.e. the layoutObject whose
// style computes to non-auto column-count and/or column-width), next to the
// flow thread. There'll be one column set for each contiguous run of column
// content. The only thing that can interrupt a contiguous run of column content
// is a column spanner, which means that if there are no spanners, there'll
// only be one column set.
//
// Since a spanner interrupts an otherwise contiguous run of column content,
// inserting one may result in the creation of additional new column sets. A
// placeholder for the spanning layoutObject has to be placed in between the
// column sets that come before and after the spanner, if there's actually
// column content both before and after the spanner.
//
// A column set has no children on its own, but is merely used to slice a
// portion of the tall "single-column" flow thread into actual columns visually,
// to convert from flow thread coordinates to visual ones. It is in charge of
// both positioning columns correctly relatively to the parent multicol
// container, and to calculate the correct translation for each column's
// contents, and to paint any rules between them. LayoutMultiColumnSet objects
// are used for painting, hit testing, and any other type of operation that
// requires mapping from flow thread coordinates to visual coordinates.
//
// Columns are normally laid out in the inline progression direction, but if the
// multicol container is inside another fragmentation context (e.g. paged media,
// or an another multicol container), we may need to group the columns, so
// that we get one MultiColumnFragmentainerGroup for each outer fragmentainer
// (page / column) that the inner multicol container lives in. Each
// fragmentainer group has its own column height, but the column height is
// uniform within a group.
class CORE_EXPORT LayoutMultiColumnSet final : public LayoutBlockFlow {
 public:
  static LayoutMultiColumnSet* CreateAnonymous(
      LayoutFlowThread&,
      const ComputedStyle& parent_style);

  void Trace(Visitor*) const override;

  bool IsLayoutNGObject() const final;

  const MultiColumnFragmentainerGroup& FirstFragmentainerGroup() const {
    NOT_DESTROYED();
    UpdateGeometryIfNeeded();
    return fragmentainer_groups_.First();
  }
  const MultiColumnFragmentainerGroup& LastFragmentainerGroup() const {
    NOT_DESTROYED();
    UpdateGeometryIfNeeded();
    return fragmentainer_groups_.Last();
  }
  MultiColumnFragmentainerGroup& LastFragmentainerGroup() {
    NOT_DESTROYED();
    UpdateGeometryIfNeeded();
    return fragmentainer_groups_.Last();
  }
  unsigned FragmentainerGroupIndexAtFlowThreadOffset(LayoutUnit,
                                                     PageBoundaryRule) const;
  const MultiColumnFragmentainerGroup& FragmentainerGroupAtFlowThreadOffset(
      LayoutUnit flow_thread_offset,
      PageBoundaryRule rule) const {
    NOT_DESTROYED();
    UpdateGeometryIfNeeded();
    return fragmentainer_groups_[FragmentainerGroupIndexAtFlowThreadOffset(
        flow_thread_offset, rule)];
  }
  const MultiColumnFragmentainerGroup& FragmentainerGroupAtVisualPoint(
      const LogicalOffset&) const;
  const MultiColumnFragmentainerGroupList& FragmentainerGroups() const {
    NOT_DESTROYED();
    UpdateGeometryIfNeeded();
    return fragmentainer_groups_;
  }

  bool IsLayoutMultiColumnSet() const final {
    NOT_DESTROYED();
    return true;
  }
  bool CanHaveChildren() const final {
    NOT_DESTROYED();
    return false;
  }

  // Return the width and height of a single column or page in the set.
  LayoutUnit PageLogicalWidth() const {
    NOT_DESTROYED();
    return FlowThread()->LogicalWidth();
  }
  bool IsPageLogicalHeightKnown() const;

  LayoutFlowThread* FlowThread() const {
    NOT_DESTROYED();
    return flow_thread_.Get();
  }

  LayoutBlockFlow* MultiColumnBlockFlow() const {
    NOT_DESTROYED();
    return To<LayoutBlockFlow>(Parent());
  }
  LayoutMultiColumnFlowThread* MultiColumnFlowThread() const {
    NOT_DESTROYED();
    return To<LayoutMultiColumnFlowThread>(FlowThread());
  }

  LayoutMultiColumnSet* NextSiblingMultiColumnSet() const;
  LayoutMultiColumnSet* PreviousSiblingMultiColumnSet() const;

  MultiColumnFragmentainerGroup& AppendNewFragmentainerGroup();

  LayoutUnit LogicalTopInFlowThread() const;
  LayoutUnit LogicalBottomInFlowThread() const;

  // Return the amount of flow thread contents that the specified fragmentainer
  // group can hold without overflowing.
  LayoutUnit FragmentainerGroupCapacity(
      const MultiColumnFragmentainerGroup& group) const {
    NOT_DESTROYED();
    return group.ColumnLogicalHeight() * UsedColumnCount();
  }

  // The used CSS value of column-count, i.e. how many columns there are room
  // for without overflowing.
  unsigned UsedColumnCount() const {
    NOT_DESTROYED();
    return MultiColumnFlowThread()->ColumnCount();
  }

  // Find the column that contains the given block offset, and return the
  // translation needed to get from flow thread coordinates to visual
  // coordinates.
  PhysicalOffset FlowThreadTranslationAtOffset(LayoutUnit,
                                               PageBoundaryRule) const;

  LogicalOffset VisualPointToFlowThreadPoint(
      const PhysicalOffset& visual_point) const;

  // Reset previously calculated column height. Will mark for layout if needed.
  void ResetColumnHeight();

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  void AttachToFlowThread();
  void DetachFromFlowThread();

  PhysicalRect FragmentsBoundingBox(
      const PhysicalRect& bounding_box_in_flow_thread) const;

  LayoutUnit ColumnGap() const;

  // The "CSS actual" value of column-count. This includes overflowing columns,
  // if any.
  unsigned ActualColumnCount() const;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutMultiColumnSet";
  }
  DeprecatedLayoutPoint LocationInternal() const override;

  // Tell the column set that it shouldn't really exist. This happens when
  // there's a leftover column set after DOM / style changes, that NG doesn't
  // care about.
  void SetIsIgnoredByNG();

  LayoutMultiColumnSet(LayoutFlowThread*);

 private:
  void InsertedIntoTree() final;
  void WillBeRemovedFromTree() final;
  PhysicalSize Size() const override;

  // This function updates frame_location_, frame_size_, and build
  // fragmentainer_groups_.
  void UpdateGeometry();
  // Call UpdateGeometry() if !HasValidCachedGeometry().
  void UpdateGeometryIfNeeded() const;

  MultiColumnFragmentainerGroupList fragmentainer_groups_;
  Member<LayoutFlowThread> flow_thread_;
};

template <>
struct DowncastTraits<LayoutMultiColumnSet> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutMultiColumnSet();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_MULTI_COLUMN_SET_H_
