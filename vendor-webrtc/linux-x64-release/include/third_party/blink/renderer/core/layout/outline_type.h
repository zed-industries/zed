// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OUTLINE_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OUTLINE_TYPE_H_

namespace blink {

// Outline styles
enum class OutlineType {
  kDontIncludeBlockInkOverflow,       // Standard outline
  kIncludeBlockInkOverflow,           // Focus outline
  kIncludeBlockInkOverflowForAnchor,  // Focus outline for anchor
};

inline bool ShouldIncludeBlockInkOverflow(OutlineType type) {
  return type == OutlineType::kIncludeBlockInkOverflow ||
         type == OutlineType::kIncludeBlockInkOverflowForAnchor;
}

inline bool ShouldIncludeBlockInkOverflowForAnchorOnly(OutlineType type) {
  return type == OutlineType::kIncludeBlockInkOverflowForAnchor;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_OUTLINE_TYPE_H_
