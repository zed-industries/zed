// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEM_SPAN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEM_SPAN_H_

#include "third_party/blink/renderer/core/layout/inline/inline_items_data.h"

namespace blink {
// |InlineItemSpan| performs like base::span<InlineItem> but stores a
// pointer to |InlineItemsData|, not |InlineItems| in |InlineItemsData|, to keep
// it alive. |InlineItems| allocated in |InlineItemsData| is deleted in
// |InlineItemsData|'s destructor even if there is any pointer to the vector,
// and storing a pointer to it can cause use-after-free bugs.
struct InlineItemSpan final {
  DISALLOW_NEW();

 public:
  InlineItemSpan() = default;

  void SetItems(const InlineItemsData* data,
                wtf_size_t begin,
                wtf_size_t size) {
    SECURITY_DCHECK(begin < data->items.size());
    SECURITY_DCHECK(begin_ + size_ <= data->items.size());
    data_ = data;
    begin_ = begin;
    size_ = size;
  }
  void Clear() {
    data_ = nullptr;
    begin_ = 0;
    size_ = 0;
  }

  bool empty() const { return size_ == 0; }
  wtf_size_t size() const { return size_; }

  InlineItems::const_iterator begin() const {
    return base::span{data_->items}.subspan(begin_).begin();
  }
  InlineItems::const_iterator end() const {
    return base::span{data_->items}.first(begin_ + size_).end();
  }

  const InlineItem& front() const {
    CHECK(!empty());
    return **begin();
  }

  void Trace(Visitor* visitor) const { visitor->Trace(data_); }

 private:
  Member<const InlineItemsData> data_;
  wtf_size_t begin_ = 0;
  wtf_size_t size_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEM_SPAN_H_
