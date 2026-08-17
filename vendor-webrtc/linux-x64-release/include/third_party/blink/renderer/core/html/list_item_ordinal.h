// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LIST_ITEM_ORDINAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LIST_ITEM_ORDINAL_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class HTMLOListElement;
class LayoutObject;
class Node;
class Element;

// Represents an "ordinal value" and its related algorithms:
// https://html.spec.whatwg.org/C/#ordinal-value
//
// The ordinal value is determined by the DOM tree order. However, since any
// elements with 'display: list-item' can be list items, the layout tree
// provides the storage for the instances of this class, and is responsible for
// firing events for insertions and removals.
class CORE_EXPORT ListItemOrdinal {
 public:
  ListItemOrdinal();

  // Get the corresponding instance for a node.
  static ListItemOrdinal* Get(const Node&);

  // Get the "ordinal value".
  int Value(const Node&) const;

  // Get/set/clear the explicit value; i.e., the 'value' attribute of an <li>
  // element.
  std::optional<int> ExplicitValue() const;
  void SetExplicitValue(int, const Element&);
  bool UseExplicitValue() const { return type_ == kExplicit; }
  void ClearExplicitValue(const Node&);
  void MarkDirty() { SetType(kNeedsUpdate); }

  static bool IsListItem(const Node&);
  static bool IsListItem(const LayoutObject*);
  static bool IsInReversedOrderedList(const Node&);

  // Compute the total item count of a list.
  static unsigned ItemCountForOrderedList(const HTMLOListElement*);

  // Invalidate all ordinal values of a list.
  static void InvalidateAllItemsForOrderedList(const HTMLOListElement*);

  // Invalidate items that are affected by an insertion or a removal.
  static void ItemInsertedOrRemoved(const LayoutObject*);
  // Invalidate items that are affected by counter style update.
  static void ItemCounterStyleUpdated(const LayoutObject&);

 private:
  enum ValueType { kNeedsUpdate, kUpdated, kExplicit };
  ValueType Type() const { return static_cast<ValueType>(type_); }
  void SetType(ValueType type) const { type_ = type; }

  static bool IsListOwner(const Node&);
  // https://drafts.csswg.org/css-contain-2/#containment-style
  static bool HasStyleContainment(const Node&);

  static Node* EnclosingList(const Node*);
  struct NodeAndOrdinal {
    STACK_ALLOCATED();

   public:
    Persistent<const Node> node;
    ListItemOrdinal* ordinal = nullptr;
    operator bool() const { return node; }
  };
  static NodeAndOrdinal NextListItem(const Node* list_node,
                                     const Node* item_node = nullptr);
  static NodeAndOrdinal PreviousListItem(const Node* list_node,
                                         const Node* item_node);
  static NodeAndOrdinal NextOrdinalItem(bool is_reversed,
                                        const Node* list_node,
                                        const Node* item_node = nullptr);

  int CalcValue(const Node&) const;

  void InvalidateSelf(const Node&, ValueType = kNeedsUpdate);
  static void InvalidateAfter(const Node* list_node, const Node* item_node);
  static void InvalidateOrdinalsAfter(bool is_reversed,
                                      const Node* list_node,
                                      const Node* item_node);
  enum UpdateType { kInsertedOrRemoved, kCounterStyle };
  static void ItemUpdated(const LayoutObject*, UpdateType type);

  mutable int value_ = 0;
  // `explicit_value_` represents the value of li elements. When the `type` is
  // set to `kExplicit`, the value of `value_` is the same as `explicit_value_`.
  mutable std::optional<int> explicit_value_;
  mutable unsigned type_ : 2;  // ValueType
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LIST_ITEM_ORDINAL_H_
