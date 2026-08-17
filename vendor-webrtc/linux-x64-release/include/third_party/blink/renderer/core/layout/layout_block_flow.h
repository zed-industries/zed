/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2007 David Smith (catfish.man@gmail.com)
 * Copyright (C) 2003-2013 Apple Inc. All rights reserved.
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BLOCK_FLOW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BLOCK_FLOW_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/inline_node_data.h"
#include "third_party/blink/renderer/core/layout/layout_block.h"

namespace blink {

class LayoutMultiColumnFlowThread;

struct InlineNodeData;

// LayoutBlockFlow is the class that implements a block container in CSS 2.1.
// http://www.w3.org/TR/CSS21/visuren.html#block-boxes
//
// LayoutBlockFlows are the only LayoutObject allowed to own floating objects
// (aka floats): http://www.w3.org/TR/CSS21/visuren.html#floats .
//
// LayoutBlockFlow enforces the following invariant:
//
// All in-flow children (ie excluding floating and out-of-flow positioned) are
// either all blocks or all inline boxes.
//
// This is suggested by CSS to correctly the layout mixed inlines and blocks
// lines (http://www.w3.org/TR/CSS21/visuren.html#anonymous-block-level). See
// LayoutBlock::addChild about how the invariant is enforced.
class CORE_EXPORT LayoutBlockFlow : public LayoutBlock {
 public:
  explicit LayoutBlockFlow(ContainerNode*);
  ~LayoutBlockFlow() override;
  void Trace(Visitor*) const override;

  static LayoutBlockFlow* CreateAnonymous(Document*, const ComputedStyle*);

  bool IsLayoutBlockFlow() const final {
    NOT_DESTROYED();
    return true;
  }

  bool CanContainFirstFormattedLine() const;

  void AddChild(LayoutObject* new_child,
                LayoutObject* before_child = nullptr) override;
  void RemoveChild(LayoutObject*) override;

  void MoveAllChildrenIncludingFloatsTo(LayoutBlock* to_block,
                                        bool full_remove_insert);

  void ChildBecameFloatingOrOutOfFlow(LayoutBox* child);
  void CollapseAnonymousBlockChild(LayoutBlockFlow* child);

  LayoutMultiColumnFlowThread* MultiColumnFlowThread() const {
    NOT_DESTROYED();
    return multi_column_flow_thread_.Get();
  }
  void ResetMultiColumnFlowThread() {
    NOT_DESTROYED();
    multi_column_flow_thread_ = nullptr;
  }

  // Return true if this block establishes a fragmentation context root (e.g. a
  // multicol container).
  //
  // Implementation detail: At some point in the future there should be no flow
  // threads. Callers that only want to know if this is a fragmentation context
  // root (and don't depend on flow threads) should call this method.
  bool IsFragmentationContextRoot() const override {
    NOT_DESTROYED();
    return MultiColumnFlowThread();
  }

  bool IsInitialLetterBox() const override;

  // Return true if this object is allowed to establish a multicol container.
  virtual bool AllowsColumns() const;

  bool CreatesNewFormattingContext() const override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutBlockFlow";
  }

  void SetShouldDoFullPaintInvalidationForFirstLine();

  PositionWithAffinity PositionForPoint(const PhysicalOffset&) const override;

  bool ShouldMoveCaretToHorizontalBoundaryWhenPastTopOrBottom() const;

  // Returns the associated `InlineNodeData`, or `nullptr` if `this` doesn't
  // have one (i.e., not an NG inline formatting context.)
  InlineNodeData* GetInlineNodeData() const {
    NOT_DESTROYED();
    return inline_node_data_.Get();
  }
  // Same as `GetInlineNodeData` and then `ClearInlineNodeData`.
  InlineNodeData* TakeInlineNodeData() {
    NOT_DESTROYED();
    return inline_node_data_.Release();
  }
  // Reset `InlineNodeData` to a new instance.
  void ResetInlineNodeData() {
    NOT_DESTROYED();
    inline_node_data_ = MakeGarbageCollected<InlineNodeData>();
  }
  // Clear `InlineNodeData` to `nullptr`.
  void ClearInlineNodeData() {
    NOT_DESTROYED();
    if (inline_node_data_) {
      // inline_node_data_ is not used from now on but exists until GC happens,
      // so it is better to eagerly clear HeapVector to improve memory
      // utilization.
      inline_node_data_->items.clear();
      inline_node_data_.Clear();
    }
  }
  virtual void WillCollectInlines() { NOT_DESTROYED(); }

 protected:
  void WillBeDestroyed() override;
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  void InvalidateDisplayItemClients(PaintInvalidationReason) const override;

  Node* NodeForHitTest() const final;
  bool HitTestChildren(HitTestResult&,
                       const HitTestLocation&,
                       const PhysicalOffset& accumulated_offset,
                       HitTestPhase) override;

  void AddOutlineRects(OutlineRectCollector&,
                       LayoutObject::OutlineInfo*,
                       const PhysicalOffset& additional_offset,
                       OutlineType) const override;

  void DirtyLinesFromChangedChild(LayoutObject* child) final;

 private:
  void CreateOrDestroyMultiColumnFlowThreadIfNeeded(
      const ComputedStyle* old_style);

  // Merge children of |sibling_that_may_be_deleted| into this object if
  // possible, and delete |sibling_that_may_be_deleted|. Returns true if we
  // were able to merge. In that case, |sibling_that_may_be_deleted| will be
  // dead. We'll only be able to merge if both blocks are anonymous.
  bool MergeSiblingContiguousAnonymousBlock(
      LayoutBlockFlow* sibling_that_may_be_deleted);

  // Reparent subsequent or preceding adjacent floating or out-of-flow siblings
  // into this object.
  void ReparentSubsequentFloatingOrOutOfFlowSiblings();
  void ReparentPrecedingFloatingOrOutOfFlowSiblings();

  void MakeChildrenInlineIfPossible();

  void MakeChildrenNonInline(LayoutObject* insertion_point = nullptr);
  void ChildBecameNonInline(LayoutObject* child) final;

 public:
  bool ShouldTruncateOverflowingText() const;

 private:
  Member<LayoutMultiColumnFlowThread> multi_column_flow_thread_;
  Member<InlineNodeData> inline_node_data_;

 protected:
  // LayoutRubyBase objects need to be able to split and merge, moving their
  // children around (calling MakeChildrenNonInline).
  friend class LayoutRubyBase;
};

template <>
struct DowncastTraits<LayoutBlockFlow> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutBlockFlow();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BLOCK_FLOW_H_
