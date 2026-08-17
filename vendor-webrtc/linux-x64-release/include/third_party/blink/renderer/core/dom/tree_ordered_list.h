/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 *           (C) 2006 Alexey Proskuryakov (ap@webkit.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2012 Apple Inc. All
 * rights reserved.
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (C) 2008, 2009, 2011, 2012 Google Inc. All rights reserved.
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TREE_ORDERED_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TREE_ORDERED_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

template <typename T>
class CORE_EXPORT TreeOrderedList final {
  static_assert(std::is_base_of_v<Node, T>,
                "TreeOrderedList only works with Nodes.");

  DISALLOW_NEW();

 public:
  TreeOrderedList() = default;
  TreeOrderedList(const TreeOrderedList&) = delete;
  TreeOrderedList& operator=(const TreeOrderedList&) = delete;

  void Add(T*);
  void Remove(const T*);
  bool IsEmpty() const { return nodes_.empty(); }
  void Clear() { nodes_.clear(); }
  wtf_size_t size() const { return nodes_.size(); }

  using iterator = HeapLinkedHashSet<Member<T>>::iterator;
  using const_iterator = HeapLinkedHashSet<Member<T>>::const_iterator;
  using const_reverse_iterator =
      HeapLinkedHashSet<Member<T>>::const_reverse_iterator;

  iterator begin() { return nodes_.begin(); }
  iterator end() { return nodes_.end(); }
  const_iterator begin() const { return nodes_.begin(); }
  const_iterator end() const { return nodes_.end(); }

  const_reverse_iterator rbegin() const { return nodes_.rbegin(); }
  const_reverse_iterator rend() const { return nodes_.rend(); }

  void Trace(Visitor*) const;

 private:
  HeapLinkedHashSet<Member<T>> nodes_;
};

template <typename T>
void TreeOrderedList<T>::Add(T* node) {
  if (nodes_.empty()) {
    nodes_.insert(node);
    return;
  }

  // Determine an appropriate insertion point.
  iterator begin = nodes_.begin();
  iterator end = nodes_.end();
  iterator it = end;
  iterator following = end;
  do {
    --it;
    T* n = *it;
    uint16_t position =
        n->compareDocumentPosition(node, Node::kTreatShadowTreesAsComposed);
    if (position & Node::kDocumentPositionFollowing) {
      nodes_.InsertBefore(following, node);
      return;
    }
    following = it;
  } while (it != begin);

  nodes_.InsertBefore(following, node);
}

template <typename T>
void TreeOrderedList<T>::Remove(const T* node) {
  nodes_.erase(const_cast<T*>(node));
}

template <typename T>
void TreeOrderedList<T>::Trace(Visitor* visitor) const {
  visitor->Trace(nodes_);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TREE_ORDERED_LIST_H_
