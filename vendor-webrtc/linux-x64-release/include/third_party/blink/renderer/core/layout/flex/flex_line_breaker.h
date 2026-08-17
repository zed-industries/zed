// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_LINE_BREAKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_LINE_BREAKER_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/core/layout/flex/flex_item.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Represents the items within a flex-line after line-breaking has occurred.
struct InitialFlexLine {
  DISALLOW_NEW();

 public:
  InitialFlexLine(base::span<FlexItem> line_items,
                  LayoutUnit sum_flex_base_size,
                  LayoutUnit sum_hypothetical_main_size)
      : line_items(std::move(line_items)),
        sum_flex_base_size(sum_flex_base_size),
        sum_hypothetical_main_size(sum_hypothetical_main_size) {}

  base::span<FlexItem> line_items;
  const LayoutUnit sum_flex_base_size;
  const LayoutUnit sum_hypothetical_main_size;
};

// The result of running the flex line-breaker.
struct FlexLineBreakerResult {
  STACK_ALLOCATED();

 public:
  ~FlexLineBreakerResult() { flex_lines.clear(); }

  HeapVector<InitialFlexLine, 1> flex_lines;
  LayoutUnit max_sum_hypothetical_main_size;
};

FlexLineBreakerResult BreakFlexItemsIntoLines(base::span<FlexItem> all_items,
                                              LayoutUnit line_break_size,
                                              LayoutUnit gap_between_items,
                                              bool is_multi_line);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_FLEX_LINE_BREAKER_H_
