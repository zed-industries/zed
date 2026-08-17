// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEMS_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEMS_DATA_H_

#include <memory>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/inline_item.h"
#include "third_party/blink/renderer/core/layout/inline/inline_item_text_index.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class InlineItemSegments;
class OffsetMapping;

// Represents a text content with a list of InlineItem. A node may have an
// additional InlineItemsData for ::first-line pseudo element.
struct CORE_EXPORT InlineItemsData : public GarbageCollected<InlineItemsData> {
 public:
  virtual ~InlineItemsData() = default;

  InlineItemTextIndex End() const {
    return {items.size(), text_content.length()};
  }

  // Text content for all inline items represented by a single InlineNode.
  // Encoded either as UTF-16 or latin-1 depending on the content.
  String text_content;
  InlineItems items;

  // Cache RunSegmenter segments when at least one item has multiple runs.
  // Set to nullptr when all items has only single run, which is common case for
  // most writing systems. However, in multi-script writing systems such as
  // Japanese, almost every item has multiple runs.
  std::unique_ptr<InlineItemSegments> segments;

  // The DOM to text content offset mapping of this inline node.
  Member<OffsetMapping> offset_mapping;

  bool IsValidOffset(unsigned index, unsigned offset) const {
    return index < items.size() && items[index]->IsValidOffset(offset);
  }
  bool IsValidOffset(const InlineItemTextIndex& index) const {
    return IsValidOffset(index.item_index, index.text_offset);
  }

  void AssertOffset(unsigned index, unsigned offset) const {
    items[index]->AssertOffset(offset);
  }
  void AssertOffset(const InlineItemTextIndex& index) const {
    AssertOffset(index.item_index, index.text_offset);
  }
  void AssertEndOffset(unsigned index, unsigned offset) const {
    items[index]->AssertEndOffset(offset);
  }

  // Get a list of `kOpenTag` items between `start_index` to
  // `start_index + size`.
  using OpenTagItems = HeapVector<Member<InlineItem>, 16>;
  void GetOpenTagItems(wtf_size_t start_index,
                       wtf_size_t size,
                       OpenTagItems* open_items) const;

#if DCHECK_IS_ON()
  void CheckConsistency() const;
#endif

  virtual void Trace(Visitor* visitor) const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEMS_DATA_H_
