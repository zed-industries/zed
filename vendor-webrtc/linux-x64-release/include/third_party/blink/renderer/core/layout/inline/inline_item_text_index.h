// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEM_TEXT_INDEX_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEM_TEXT_INDEX_H_

#include <ostream>
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

// Represents an index of `InlineItem`, along with the text offset.
struct CORE_EXPORT InlineItemTextIndex {
  explicit operator bool() const { return text_offset || item_index; }
  bool IsZero() const { return !text_offset && !item_index; }

  bool operator==(const InlineItemTextIndex& other) const {
    return text_offset == other.text_offset && item_index == other.item_index;
  }
  bool operator!=(const InlineItemTextIndex& other) const {
    return !operator==(other);
  }
  bool operator>(const InlineItemTextIndex& other) const {
    return text_offset > other.text_offset || item_index > other.item_index;
  }
  bool operator<(const InlineItemTextIndex& other) const {
    return text_offset < other.text_offset || item_index < other.item_index;
  }
  bool operator>=(const InlineItemTextIndex& other) const {
    return !operator<(other);
  }
  bool operator<=(const InlineItemTextIndex& other) const {
    return !operator>(other);
  }

  // The index of `InlineItemsData::items`.
  wtf_size_t item_index = 0;
  // The offset of `InlineItemsData::text_content`.
  wtf_size_t text_offset = 0;
};

inline std::ostream& operator<<(std::ostream& ostream,
                                const InlineItemTextIndex& index) {
  return ostream << "{" << index.item_index << "," << index.text_offset << "}";
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_ITEM_TEXT_INDEX_H_
