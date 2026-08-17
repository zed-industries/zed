// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_COLUMN_PSEUDO_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_COLUMN_PSEUDO_ELEMENT_H_

#include "third_party/blink/renderer/core/dom/pseudo_element.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"

namespace blink {

// A ::column pseudo element. When needed, each column in a multicol container
// will create one of these, during layout.
class ColumnPseudoElement : public PseudoElement {
 public:
  ColumnPseudoElement(Element* originating_element, wtf_size_t index);

  bool IsColumnPseudoElement() const final { return true; }
  wtf_size_t Index() const { return index_; }

  // The column rectangle, relatively to the multicol container.
  const PhysicalRect& ColumnRect() const { return column_rect_; }
  void SetColumnRect(PhysicalRect column_rect) { column_rect_ = column_rect; }

  // Return the first element that starts in the column, in DOM order.
  Element* FirstChildInDOMOrder() const;

  void AttachLayoutTree(AttachContext&) final;
  void DetachLayoutTree(bool performing_reattach) final;

 private:
  // Used for linear time tree traversals.
  wtf_size_t index_;

  PhysicalRect column_rect_;
};

template <>
struct DowncastTraits<ColumnPseudoElement> {
  static bool AllowFrom(const Node& node) {
    return node.IsColumnPseudoElement();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_COLUMN_PSEUDO_ELEMENT_H_
