// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_CAPTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_CAPTION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"

namespace blink {

class CORE_EXPORT LayoutTableCaption final : public LayoutBlockFlow {
 public:
  explicit LayoutTableCaption(Element*);

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutTableCaption";
  }

  bool CreatesNewFormattingContext() const final {
    NOT_DESTROYED();
    return true;
  }

  bool IsTableCaption() const final {
    NOT_DESTROYED();
    return true;
  }
};

// wtf/casting.h helper.
template <>
struct DowncastTraits<LayoutTableCaption> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsTableCaption();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TABLE_LAYOUT_TABLE_CAPTION_H_
