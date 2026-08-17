// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_TEXT_AUTO_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_TEXT_AUTO_SPACE_H_

#include <unicode/umachine.h>
#include <ostream>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/inline/inline_item_segment.h"
#include "third_party/blink/renderer/core/layout/inline/inline_items_data.h"
#include "third_party/blink/renderer/platform/fonts/shaping/text_auto_space.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

struct InlineItemsData;

// A wrapper of TextAutoSpace for the inline layout.
class CORE_EXPORT InlineTextAutoSpace : public TextAutoSpace {
  STACK_ALLOCATED();

 public:
  explicit InlineTextAutoSpace(const InlineItemsData& data);

  // True if this may apply auto-spacing. If this is false, it's safe to skip
  // calling `Apply()`.
  bool MayApply() const { return !ranges_.empty(); }

  // Apply auto-spacing as per CSS Text:
  // https://drafts.csswg.org/css-text-4/#propdef-text-autospace
  //
  // The `data` must be the same instance as the one given to the constructor.
  //
  // If `offsets_out` is not null, the offsets of auto-space points are added to
  // it without applying auto-spacing. This is for tseting-purpose.
  void Apply(InlineItemsData& data, Vector<wtf_size_t>* offsets_out = nullptr);
  void ApplyIfNeeded(InlineItemsData& data,
                     Vector<wtf_size_t>* offsets_out = nullptr) {
    if (MayApply()) [[unlikely]] {
      Apply(data, offsets_out);
    }
  }

 private:
  void Initialize(const InlineItemsData& data);

  InlineItemSegments::RunSegmenterRanges ranges_;
};

inline InlineTextAutoSpace::InlineTextAutoSpace(const InlineItemsData& data) {
  if (!RuntimeEnabledFeatures::CSSTextAutoSpaceEnabled()) {
    return;
  }

  if (data.text_content.Is8Bit()) {
    return;  // 8-bits never be `kIdeograph`. See `TextAutoSpaceTest`.
  }

  Initialize(data);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INLINE_TEXT_AUTO_SPACE_H_
