// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_CHILD_LAYOUT_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_CHILD_LAYOUT_CONTEXT_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/fragment_items_builder.h"
#include "third_party/blink/renderer/core/layout/inline/inline_box_state.h"
#include "third_party/blink/renderer/core/layout/inline/logical_line_item.h"
#include "third_party/blink/renderer/core/layout/inline/score_line_break_context.h"

namespace blink {

// A context object given to layout. The same instance should be given to
// children of a parent node, but layout algorithm should be prepared to be
// given a new instance when yield or fragmentation occur.
//
// Because this context is in initial state for when fragmentation occurs and
// some other cases, do not add things that are too expensive to rebuild.
//
// This class has no public constructors. Instantiate one of subclasses below
// depending on the line breaker type for the context.
class CORE_EXPORT InlineChildLayoutContext {
  STACK_ALLOCATED();

 public:
  ~InlineChildLayoutContext();

  FragmentItemsBuilder* ItemsBuilder() { return &items_builder_; }

  ScoreLineBreakContext* GetScoreLineBreakContext() const {
    return score_line_break_context_;
  }
  LineInfo& GetLineInfo(const InlineBreakToken* break_token,
                        bool& is_cached_out);

  // Acquire/release temporary |LogicalLineItems|, used for a short period of
  // time, but needed multiple times in a context.
  LogicalLineItems& AcquireTempLogicalLineItems();
  void ReleaseTempLogicalLineItems(LogicalLineItems&);

  // Returns the InlineLayoutStateStack in this context.
  bool HasBoxStates() const { return box_states_.has_value(); }
  InlineLayoutStateStack* BoxStates() { return &*box_states_; }
  InlineLayoutStateStack* ResetBoxStates() { return &box_states_.emplace(); }

  // Returns the box states in this context if it exists and it can be used to
  // create a line starting from |items[item_index}|, otherwise returns nullptr.
  //
  // To determine this, callers must call |SetItemIndex| to set the end of the
  // current line.
  InlineLayoutStateStack* BoxStatesIfValidForItemIndex(const InlineItems& items,
                                                       unsigned item_index);
  void SetItemIndex(const InlineItems& items, unsigned item_index) {
    items_ = &items;
    item_index_ = item_index;
  }

  const HeapVector<Member<const BreakToken>>& ParallelFlowBreakTokens() const {
    return parallel_flow_break_tokens_;
  }
  void ClearParallelFlowBreakTokens();
  void PropagateParallelFlowBreakToken(const BreakToken*);

  const std::optional<LayoutUnit>& BalancedAvailableWidth() const {
    return balanced_available_width_;
  }
  void SetBalancedAvailableWidth(std::optional<LayoutUnit> value) {
    balanced_available_width_ = value;
  }

 protected:
  InlineChildLayoutContext(const InlineNode& node,
                           BoxFragmentBuilder* container_builder,
                           LineInfo* line_info);
  InlineChildLayoutContext(const InlineNode& node,
                           BoxFragmentBuilder* container_builder,
                           ScoreLineBreakContext* score_line_break_context);

 private:
  BoxFragmentBuilder* container_builder_ = nullptr;
  FragmentItemsBuilder items_builder_;

  LineInfo* line_info_ = nullptr;
  ScoreLineBreakContext* score_line_break_context_ = nullptr;

  LogicalLineItems* temp_logical_line_items_ = nullptr;

  std::optional<InlineLayoutStateStack> box_states_;

  // The items and its index this context is set up for.
  const InlineItems* items_ = nullptr;
  unsigned item_index_ = 0;

  HeapVector<Member<const BreakToken>> parallel_flow_break_tokens_;

  // Used by `ParagraphLineBreaker`.
  std::optional<LayoutUnit> balanced_available_width_;
};

// A subclass of `InlineChildLayoutContext` for when the algorithm requires
// only one `LineInfo`.
class CORE_EXPORT SimpleInlineChildLayoutContext
    : public InlineChildLayoutContext {
 public:
  SimpleInlineChildLayoutContext(const InlineNode& node,
                                 BoxFragmentBuilder* container_builder)
      : InlineChildLayoutContext(node, container_builder, &line_info_storage_) {
  }

 private:
  LineInfo line_info_storage_;
};

// A subclass of `InlineChildLayoutContext` for when the algorithm requires
// `ScoreLineBreakContext`.
template <wtf_size_t max_lines>
class CORE_EXPORT OptimalInlineChildLayoutContext
    : public InlineChildLayoutContext {
 public:
  OptimalInlineChildLayoutContext(const InlineNode& node,
                                  BoxFragmentBuilder* container_builder)
      : InlineChildLayoutContext(node,
                                 container_builder,
                                 &score_line_break_context_instance_) {}

 private:
  ScoreLineBreakContextOf<max_lines> score_line_break_context_instance_;
};

inline LineInfo& InlineChildLayoutContext::GetLineInfo(
    const InlineBreakToken* break_token,
    bool& is_cached_out) {
  DCHECK(!is_cached_out);
  if (line_info_) {
    return *line_info_;
  }
  DCHECK(score_line_break_context_);
  return score_line_break_context_->GetLineInfoList().Get(break_token,
                                                          is_cached_out);
}

inline LogicalLineItems&
InlineChildLayoutContext::AcquireTempLogicalLineItems() {
  if (LogicalLineItems* line_items = temp_logical_line_items_) {
    temp_logical_line_items_ = nullptr;
    DCHECK_EQ(line_items->size(), 0u);
    return *line_items;
  }
  return *MakeGarbageCollected<LogicalLineItems>();
}

inline void InlineChildLayoutContext::ReleaseTempLogicalLineItems(
    LogicalLineItems& line_items) {
  DCHECK(&line_items);
  line_items.clear();
  temp_logical_line_items_ = &line_items;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_CHILD_LAYOUT_CONTEXT_H_
