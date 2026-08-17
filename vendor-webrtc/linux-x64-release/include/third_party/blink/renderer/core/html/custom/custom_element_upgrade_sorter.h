// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_UPGRADE_SORTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_UPGRADE_SORTER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Element;
class Node;

// Does a shadow-including tree order sort of a subset of elements.
// https://dom.spec.whatwg.org/#concept-shadow-including-tree-order
class CORE_EXPORT CustomElementUpgradeSorter {
  STACK_ALLOCATED();

 public:
  CustomElementUpgradeSorter() = default;

  // Record an element of interest. The DOM tree must not be
  // modified between calls to `Add` and the call(s) to `Sorted`.
  void Add(Element*);

  // Adds shadow-including descendents of `parent` to result in
  // shadow-including tree order. This operation removes all shadow-including
  // descendants of `parent` from this sorter; After calling `Sorted`, this
  // sorted must not be called with `Add` or `Sorted` with any shadow-including
  // descendant of `parent`.
  void Sorted(HeapVector<Member<Element>>* result, Node* parent);

 private:
  using ChildSet = GCedHeapHashSet<Member<Node>>;
  using ParentChildMap = HeapHashMap<Member<Node>, Member<ChildSet>>;

  enum AddResult { kParentAlreadyExistsInMap, kParentAddedToMap };

  AddResult AddToParentChildMap(Node* parent, Node* child);
  void Visit(HeapVector<Member<Element>>* result,
             ChildSet&,
             const ChildSet::iterator&);

  HeapHashSet<Member<Element>> elements_;

  // This is the subset of the tree, from root node (usually
  // document) through elements and shadow roots, to candidates.
  ParentChildMap parent_child_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_UPGRADE_SORTER_H_
