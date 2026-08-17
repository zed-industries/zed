// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_FRAME_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_FRAME_SET_H_

#include "third_party/blink/renderer/core/layout/layout_block.h"

namespace blink {

class LayoutFrameSet final : public LayoutBlock {
 public:
  explicit LayoutFrameSet(Element*);

 private:
  const char* GetName() const override;
  bool IsFrameSet() const final {
    NOT_DESTROYED();
    return true;
  }
  bool IsChildAllowed(LayoutObject* child, const ComputedStyle&) const override;
  void AddChild(LayoutObject* new_child, LayoutObject* before_child) override;
  void RemoveChild(LayoutObject* child) override;
  CursorDirective GetCursor(const PhysicalOffset& point,
                            ui::Cursor& cursor) const override;
};

template <>
struct DowncastTraits<LayoutFrameSet> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsFrameSet();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_FRAME_SET_H_
