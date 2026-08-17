// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LOGICAL_LINE_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LOGICAL_LINE_CONTAINER_H_

#include "third_party/blink/renderer/platform/fonts/font_height.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class LogicalLineItems;

// Represents a line with optional ruby annotations.
// This is used to pass InlineLayoutAlgorithm::CreateLine() deliverables to
// FragmentItemsBuilder.
class LogicalLineContainer : public GarbageCollected<LogicalLineContainer> {
 public:
  LogicalLineContainer();
  void Trace(Visitor* visitor) const;

  LogicalLineItems& BaseLine() const { return *base_line_; }
  struct AnnotationLine {
    DISALLOW_NEW();

   public:
    FontHeight metrics;
    Member<LogicalLineItems> line_items;

    AnnotationLine(FontHeight metrics, LogicalLineItems& line_items)
        : metrics(metrics), line_items(line_items) {}
    void Trace(Visitor* visitor) const { visitor->Trace(line_items); }
    LogicalLineItems* operator->() const { return line_items.Get(); }
  };
  const HeapVector<AnnotationLine>& AnnotationLineList() const {
    return annotation_line_list_;
  }
  wtf_size_t EstimatedFragmentItemCount() const;

  void AddAnnotation(FontHeight metrics, LogicalLineItems& line_items) {
    annotation_line_list_.push_back(AnnotationLine(metrics, line_items));
  }
  // Release all collection buffers.
  void Clear();
  // Set collection sizes to zero without releasing their buffers.
  void Shrink();
  void MoveInBlockDirection(LayoutUnit);

 private:
  Member<LogicalLineItems> base_line_;
  HeapVector<AnnotationLine> annotation_line_list_;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::LogicalLineContainer::AnnotationLine)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LOGICAL_LINE_CONTAINER_H_
