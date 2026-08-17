// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_RESOLVED_TEXT_LAYOUT_ATTRIBUTES_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_RESOLVED_TEXT_LAYOUT_ATTRIBUTES_ITERATOR_H_

#include <algorithm>
#include <iterator>
#include <utility>

#include "third_party/blink/renderer/core/layout/svg/svg_character_data.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// This class wraps a sparse list, |Vector<std::pair<unsigned,
// SvgCharacterData>>|, so that it looks to have SvgCharacterData for any index.
//
// For example, if |resolved| contains the following pairs:
//     resolved[0]: (0, SvgCharacterData)
//     resolved[1]: (10, SvgCharacterData)
//     resolved[2]: (42, SvgCharacterData)
//
// AdvanceTo(0) returns the SvgCharacterData at [0].
// AdvanceTo(1 - 9) returns the default SvgCharacterData, which has no data.
// AdvanceTo(10) returns the SvgCharacterData at [1].
// AdvanceTo(11 - 41) returns the default SvgCharacterData.
// AdvanceTo(42) returns the SvgCharacterData at [2].
// AdvanceTo(43 or greater) returns the default SvgCharacterData.
class ResolvedTextLayoutAttributesIterator final {
  USING_FAST_MALLOC(ResolvedTextLayoutAttributesIterator);

 public:
  explicit ResolvedTextLayoutAttributesIterator(
      const Vector<std::pair<unsigned, SvgCharacterData>>& resolved)
      : resolved_(resolved) {}
  ResolvedTextLayoutAttributesIterator(
      const ResolvedTextLayoutAttributesIterator&) = delete;
  ResolvedTextLayoutAttributesIterator& operator=(
      const ResolvedTextLayoutAttributesIterator&) = delete;

  const SvgCharacterData& AdvanceTo(unsigned addressable_index) {
    if (index_ >= resolved_.size()) {
      return default_data_;
    }
    if (addressable_index < resolved_[index_].first) {
      return default_data_;
    }
    if (addressable_index == resolved_[index_].first) {
      return resolved_[index_].second;
    }
    auto resolved_range = base::span(resolved_).subspan(index_);
    auto it = std::ranges::find_if(resolved_range,
                                   [addressable_index](const auto& pair) {
                                     return addressable_index <= pair.first;
                                   });
    index_ +=
        static_cast<wtf_size_t>(std::distance(resolved_range.begin(), it));
    return AdvanceTo(addressable_index);
  }

 private:
  const SvgCharacterData default_data_;
  const Vector<std::pair<unsigned, SvgCharacterData>>& resolved_;
  wtf_size_t index_ = 0u;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_RESOLVED_TEXT_LAYOUT_ATTRIBUTES_ITERATOR_H_
