// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_MULTI_COLUMN_SPANNER_PLACEHOLDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_MULTI_COLUMN_SPANNER_PLACEHOLDER_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"

namespace blink {

// Placeholder layoutObject for column-span:all elements. The column-span:all
// layoutObject itself is a descendant of the flow thread, but due to its
// out-of-flow nature, we need something on the outside to take care of its
// positioning and sizing. LayoutMultiColumnSpannerPlaceholder objects are
// siblings of LayoutMultiColumnSet objects, i.e. direct children of the
// multicol container.
class LayoutMultiColumnSpannerPlaceholder final : public LayoutBox {
 public:
  bool IsLayoutMultiColumnSpannerPlaceholder() const final {
    NOT_DESTROYED();
    return true;
  }

  static LayoutMultiColumnSpannerPlaceholder* CreateAnonymous(
      const ComputedStyle& parent_style,
      LayoutBox&);

  void Trace(Visitor*) const override;

  LayoutBlockFlow* MultiColumnBlockFlow() const {
    NOT_DESTROYED();
    return To<LayoutBlockFlow>(Parent());
  }

  LayoutMultiColumnFlowThread* FlowThread() const {
    NOT_DESTROYED();
    return To<LayoutBlockFlow>(Parent())->MultiColumnFlowThread();
  }

  LayoutBox* LayoutObjectInFlowThread() const {
    NOT_DESTROYED();
    return layout_object_in_flow_thread_.Get();
  }

  bool AnonymousHasStylePropagationOverride() final {
    NOT_DESTROYED();
    return true;
  }

  void LayoutObjectInFlowThreadStyleDidChange(const ComputedStyle* old_style);
  void UpdateProperties(const ComputedStyle& parent_style);

  explicit LayoutMultiColumnSpannerPlaceholder(LayoutBox*);

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutMultiColumnSpannerPlaceholder";
  }

 protected:
  void InsertedIntoTree() override;
  void WillBeRemovedFromTree() override;

 private:
  DeprecatedLayoutPoint LocationInternal() const override;
  PhysicalSize Size() const override;

  // The actual column-span:all layoutObject inside the flow thread.
  Member<LayoutBox> layout_object_in_flow_thread_;
};

template <>
struct DowncastTraits<LayoutMultiColumnSpannerPlaceholder> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutMultiColumnSpannerPlaceholder();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_MULTI_COLUMN_SPANNER_PLACEHOLDER_H_
