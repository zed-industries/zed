// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_LAYOUT_INLINE_LIST_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_LAYOUT_INLINE_LIST_ITEM_H_

#include "third_party/blink/renderer/core/html/list_item_ordinal.h"
#include "third_party/blink/renderer/core/layout/layout_inline.h"

namespace blink {

// A LayoutObject subclass for 'display: inline list-item'.
class LayoutInlineListItem final : public LayoutInline {
 public:
  explicit LayoutInlineListItem(Element* element);

  ListItemOrdinal& Ordinal() {
    NOT_DESTROYED();
    return ordinal_;
  }
  int Value() const;
  void OrdinalValueChanged();

  LayoutObject* Marker() const;
  void UpdateMarkerTextIfNeeded();

  void UpdateCounterStyle();

 private:
  void WillBeDestroyed() override;
  const char* GetName() const override;
  bool IsInlineListItem() const final {
    NOT_DESTROYED();
    return true;
  }
  void InsertedIntoTree() override;
  void WillBeRemovedFromTree() override;
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
  void SubtreeDidChange() final;

  ListItemOrdinal ordinal_;
};

template <>
struct DowncastTraits<LayoutInlineListItem> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsInlineListItem();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_LAYOUT_INLINE_LIST_ITEM_H_
