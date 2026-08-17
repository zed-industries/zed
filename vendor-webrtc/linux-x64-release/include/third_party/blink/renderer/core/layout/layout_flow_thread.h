/*
 * Copyright (C) 2011 Adobe Systems Incorporated. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer.
 * 2. Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials
 *    provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDER "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY,
 * OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
 * TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_FLOW_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_FLOW_THREAD_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/linked_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/pod_interval_tree.h"

namespace blink {

class LayoutMultiColumnSet;

typedef HeapLinkedHashSet<Member<LayoutMultiColumnSet>>
    LayoutMultiColumnSetList;

// LayoutFlowThread is used to collect all the layout objects that participate
// in a flow thread. It will also help in doing the layout. However, it will not
// layout directly to screen. Instead, LayoutMultiColumnSet objects will
// redirect their paint and nodeAtPoint methods to this object. Each
// LayoutMultiColumnSet will actually be a viewPort of the LayoutFlowThread.
class CORE_EXPORT LayoutFlowThread : public LayoutBlockFlow {
 public:
  explicit LayoutFlowThread();
  ~LayoutFlowThread() override = default;
  void Trace(Visitor*) const override;

  bool IsLayoutNGObject() const final;
  bool IsLayoutFlowThread() const final {
    NOT_DESTROYED();
    return true;
  }
  virtual bool IsLayoutMultiColumnFlowThread() const {
    NOT_DESTROYED();
    return false;
  }

  bool CreatesNewFormattingContext() const final {
    NOT_DESTROYED();
    // The spec requires multicol containers to establish new formatting
    // contexts. Blink uses an anonymous flow thread child of the multicol
    // container to actually perform layout inside. Therefore we need to
    // propagate the BFCness down to the flow thread, so that floats are fully
    // contained by the flow thread, and thereby the multicol container.
    return true;
  }

  // Search mode when looking for an enclosing fragmentation context.
  enum AncestorSearchConstraint {
    // No constraints. When we're not laying out (but rather e.g. painting or
    // hit-testing), we just want to find all enclosing fragmentation contexts,
    // e.g. to calculate the accumulated visual translation.
    kAnyAncestor,

    // Consider fragmentation contexts that are strictly unbreakable (seen from
    // the outside) to be isolated from the rest, so that such fragmentation
    // contexts don't participate in fragmentation of enclosing fragmentation
    // contexts, apart from taking up space and otherwise being completely
    // unbreakable. This is typically what we want to do during layout.
    kIsolateUnbreakableContainers,
  };

  static LayoutFlowThread* LocateFlowThreadContainingBlockOf(
      const LayoutObject&,
      AncestorSearchConstraint);

  PaintLayerType LayerTypeRequired() const final;

  virtual void FlowThreadDescendantWasInserted(LayoutObject*) {
    NOT_DESTROYED();
  }
  virtual void FlowThreadDescendantWillBeRemoved(LayoutObject*) {
    NOT_DESTROYED();
  }
  virtual void FlowThreadDescendantStyleWillChange(
      LayoutBoxModelObject*,
      StyleDifference,
      const ComputedStyle& new_style) {
    NOT_DESTROYED();
  }
  virtual void FlowThreadDescendantStyleDidChange(
      LayoutBoxModelObject*,
      StyleDifference,
      const ComputedStyle& old_style) {
    NOT_DESTROYED();
  }

  void QuadsInAncestorForDescendant(const LayoutBox& descendant,
                                    Vector<gfx::QuadF>&,
                                    const LayoutBoxModelObject* ancestor,
                                    MapCoordinatesFlags);

  void AddOutlineRects(OutlineRectCollector&,
                       OutlineInfo*,
                       const PhysicalOffset& additional_offset,
                       OutlineType) const override;

  void Paint(const PaintInfo& paint_info) const final;

  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation&,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase) final;

  virtual void AddColumnSetToThread(LayoutMultiColumnSet*) = 0;
  virtual void RemoveColumnSetFromThread(LayoutMultiColumnSet*);

  bool HasColumnSets() const {
    NOT_DESTROYED();
    return multi_column_set_list_.size();
  }

  void ValidateColumnSets();
  void InvalidateColumnSets() {
    NOT_DESTROYED();
    column_sets_invalidated_ = true;
  }
  bool HasValidColumnSetInfo() const {
    NOT_DESTROYED();
    return !column_sets_invalidated_ && !multi_column_set_list_.empty();
  }

  bool MapToVisualRectInAncestorSpaceInternal(
      const LayoutBoxModelObject* ancestor,
      TransformState&,
      VisualRectFlags = kDefaultVisualRectFlags) const override;

  virtual bool IsPageLogicalHeightKnown() const {
    NOT_DESTROYED();
    return true;
  }
  // Return the visual bounding box based on the supplied flow-thread bounding
  // box. Both rectangles are completely physical in terms of writing mode.
  PhysicalRect FragmentsBoundingBox(
      const PhysicalRect& layer_bounding_box) const;

  virtual PhysicalOffset VisualPointToFlowThreadPoint(
      const PhysicalOffset& visual_point) const = 0;

  virtual LayoutMultiColumnSet* ColumnSetAtBlockOffset(
      LayoutUnit,
      PageBoundaryRule) const = 0;

  const char* GetName() const override = 0;

  RecalcScrollableOverflowResult RecalcScrollableOverflow() final;

 protected:
  void GenerateColumnSetIntervalTree();

  LayoutMultiColumnSetList multi_column_set_list_;

  typedef WTF::PODInterval<LayoutUnit, LayoutMultiColumnSet*>
      MultiColumnSetInterval;
  typedef WTF::PODIntervalTree<LayoutUnit, LayoutMultiColumnSet*>
      MultiColumnSetIntervalTree;

  class MultiColumnSetSearchAdapter {
    STACK_ALLOCATED();

   public:
    MultiColumnSetSearchAdapter(LayoutUnit offset)
        : offset_(offset), result_(nullptr) {}

    const LayoutUnit& LowValue() const { return offset_; }
    const LayoutUnit& HighValue() const { return offset_; }
    void CollectIfNeeded(const MultiColumnSetInterval&);

    LayoutMultiColumnSet* Result() const { return result_; }

   private:
    LayoutUnit offset_;
    LayoutMultiColumnSet* result_;
  };

  MultiColumnSetIntervalTree multi_column_set_interval_tree_;

  bool column_sets_invalidated_ : 1;
};

template <>
struct DowncastTraits<LayoutFlowThread> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutFlowThread();
  }
};

}  // namespace blink

namespace WTF {
// These structures are used by PODIntervalTree for debugging.
#ifndef NDEBUG
template <>
struct ValueToString<blink::LayoutMultiColumnSet*> {
  static String ToString(const blink::LayoutMultiColumnSet* value) {
    return String::Format("%p", value);
  }
};
template <>
struct ValueToString<blink::LayoutUnit> {
  static String ToString(const blink::LayoutUnit value) {
    return String::Number(value.ToFloat());
  }
};
#endif
}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_FLOW_THREAD_H_
