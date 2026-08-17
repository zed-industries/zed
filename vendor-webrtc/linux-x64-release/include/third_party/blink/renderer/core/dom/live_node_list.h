/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2006, 2007 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LIVE_NODE_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LIVE_NODE_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/live_node_list_base.h"
#include "third_party/blink/renderer/core/dom/node_list.h"
#include "third_party/blink/renderer/core/html/collection_items_cache.h"
#include "third_party/blink/renderer/core/html/collection_type.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Element;

class CORE_EXPORT LiveNodeList : public NodeList, public LiveNodeListBase {
 public:
  LiveNodeList(ContainerNode& owner_node,
               CollectionType collection_type,
               NodeListInvalidationType invalidation_type,
               NodeListSearchRoot search_root = NodeListSearchRoot::kOwnerNode);

  unsigned length() const final;
  Element* item(unsigned offset) const final;
  virtual bool ElementMatches(const Element&) const = 0;

  void InvalidateCache(Document* old_document = nullptr) const final;
  void InvalidateCacheForAttribute(const QualifiedName*) const;

  // Collection IndexCache API.
  virtual bool CanTraverseBackward() const { return true; }
  virtual Element* TraverseToFirst() const;
  virtual Element* TraverseToLast() const;
  virtual Element* TraverseForwardToOffset(unsigned offset,
                                           Element& current_node,
                                           unsigned& current_offset) const;
  virtual Element* TraverseBackwardToOffset(unsigned offset,
                                            Element& current_node,
                                            unsigned& current_offset) const;

  void Trace(Visitor*) const override;

 private:
  Node* VirtualOwnerNode() const final;

  mutable CollectionItemsCache<LiveNodeList, Element> collection_items_cache_;
};

template <>
struct DowncastTraits<LiveNodeList> {
  static bool AllowFrom(const LiveNodeListBase& list) {
    return IsLiveNodeListType(list.GetType());
  }
};

inline void LiveNodeList::InvalidateCacheForAttribute(
    const QualifiedName* attr_name) const {
  if (!attr_name ||
      ShouldInvalidateTypeOnAttributeChange(InvalidationType(), *attr_name))
    InvalidateCache();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LIVE_NODE_LIST_H_
