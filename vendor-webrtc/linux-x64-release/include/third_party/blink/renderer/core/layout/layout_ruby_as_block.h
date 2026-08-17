// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_RUBY_AS_BLOCK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_RUBY_AS_BLOCK_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"

namespace blink {

// This is a general block container wrapping an anonymous LayoutRuby.
//
// https://drafts.csswg.org/css-ruby/#block-ruby
// > If an element has an inner display type of ruby and an outer display type
// > other than inline, then it generates two boxes: a principal box of the
// > required outer display type, and an inline-level ruby container.
class CORE_EXPORT LayoutRubyAsBlock : public LayoutBlockFlow {
 public:
  explicit LayoutRubyAsBlock(Element*);
  ~LayoutRubyAsBlock() override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutRubyAsBlock";
  }
  bool IsRuby() const final {
    NOT_DESTROYED();
    return true;
  }

  void AddChild(LayoutObject* child,
                LayoutObject* before_child = nullptr) override;
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
  void RemoveLeftoverAnonymousBlock(LayoutBlock*) override;
};

template <>
struct DowncastTraits<LayoutRubyAsBlock> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsRuby() && !object.IsLayoutInline();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_RUBY_AS_BLOCK_H_
