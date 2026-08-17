// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_CUSTOM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_CUSTOM_H_

#include "third_party/blink/renderer/core/layout/layout_block_flow.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// The LayoutObject for elements which have "display: layout(foo);" specified.
// https://drafts.css-houdini.org/css-layout-api/
//
// This class inherits from LayoutBlockFlow so that when a web developer's
// intrinsicSizes/layout callback fails, it can fallback onto the "default"
// block-flow layout algorithm.
class LayoutCustom final : public LayoutBlockFlow {
 public:
  // NOTE: In the future there may be a third state "normal", this will mean
  // that not everything is blockified, (e.g. root inline boxes, so that
  // line-by-line layout can be performed).
  enum State { kUnloaded, kBlock };

  explicit LayoutCustom(Element*);

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutCustom";
  }
  bool CreatesNewFormattingContext() const override {
    NOT_DESTROYED();
    return true;
  }

  bool IsLoaded() {
    NOT_DESTROYED();
    return state_ != kUnloaded;
  }

  void AddChild(LayoutObject* new_child, LayoutObject* before_child) override;
  void RemoveChild(LayoutObject* child) override;

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  bool IsMonolithic() const final {
    NOT_DESTROYED();
    return true;
  }

 private:
  bool IsLayoutCustom() const final {
    NOT_DESTROYED();
    return true;
  }

  State state_;
};

template <>
struct DowncastTraits<LayoutCustom> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutCustom();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_LAYOUT_CUSTOM_H_
