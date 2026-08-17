/*
 * Copyright (C) 2008, 2010 Apple Inc. All rights reserved.
 * Copyright (C) 2008 David Smith <catfish.man@gmail.com>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_LISTS_NODE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_LISTS_NODE_DATA_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/dom/child_node_list.h"
#include "third_party/blink/renderer/core/dom/empty_node_list.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/core/dom/tag_collection.h"
#include "third_party/blink/renderer/core/html/collection_type.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class NodeListsNodeData final : public GarbageCollected<NodeListsNodeData> {
 public:
  ChildNodeList* GetChildNodeList(ContainerNode& node) {
    DCHECK(!child_node_list_ || node == child_node_list_->VirtualOwnerNode());
    return To<ChildNodeList>(child_node_list_.Get());
  }

  ChildNodeList* EnsureChildNodeList(ContainerNode& node) {
    if (child_node_list_)
      return To<ChildNodeList>(child_node_list_.Get());
    auto* list = MakeGarbageCollected<ChildNodeList>(node);
    child_node_list_ = list;
    return list;
  }

  EmptyNodeList* EnsureEmptyChildNodeList(Node& node) {
    if (child_node_list_)
      return To<EmptyNodeList>(child_node_list_.Get());
    auto* list = MakeGarbageCollected<EmptyNodeList>(node);
    child_node_list_ = list;
    return list;
  }

  using NamedNodeListKey = std::pair<CollectionType, AtomicString>;
  struct NodeListAtomicCacheMapEntryHashTraits
      : HashTraits<std::pair<CollectionType, AtomicString>> {
    static unsigned GetHash(const NamedNodeListKey& entry) {
      return WTF::GetHash(entry.second == CSSSelector::UniversalSelectorAtom()
                              ? g_star_atom
                              : entry.second) +
             entry.first;
    }
    static constexpr bool kSafeToCompareToEmptyOrDeleted =
        HashTraits<AtomicString>::kSafeToCompareToEmptyOrDeleted;
  };

  typedef HeapHashMap<NamedNodeListKey,
                      Member<LiveNodeListBase>,
                      NodeListAtomicCacheMapEntryHashTraits>
      NodeListAtomicNameCacheMap;
  typedef HeapHashMap<QualifiedName, Member<TagCollectionNS>>
      TagCollectionNSCache;

  template <typename T>
  T* AddCache(ContainerNode& node,
              CollectionType collection_type,
              const AtomicString& name) {
    NodeListAtomicNameCacheMap::AddResult result = atomic_name_caches_.insert(
        std::make_pair(collection_type, name), nullptr);
    if (!result.is_new_entry) {
      return static_cast<T*>(result.stored_value->value.Get());
    }

    auto* list = MakeGarbageCollected<T>(node, collection_type, name);
    result.stored_value->value = list;
    return list;
  }

  template <typename T>
  T* AddCache(ContainerNode& node, CollectionType collection_type) {
    NodeListAtomicNameCacheMap::AddResult result = atomic_name_caches_.insert(
        NamedNodeListKey(collection_type, CSSSelector::UniversalSelectorAtom()),
        nullptr);
    if (!result.is_new_entry) {
      return static_cast<T*>(result.stored_value->value.Get());
    }

    auto* list = MakeGarbageCollected<T>(node, collection_type);
    result.stored_value->value = list;
    return list;
  }

  template <typename T>
  T* Cached(CollectionType collection_type) {
    auto it = atomic_name_caches_.find(NamedNodeListKey(
        collection_type, CSSSelector::UniversalSelectorAtom()));
    return static_cast<T*>(it != atomic_name_caches_.end() ? &*it->value
                                                           : nullptr);
  }

  template <typename T>
  const T* Cached(CollectionType collection_type) const {
    auto it = atomic_name_caches_.find(NamedNodeListKey(
        collection_type, CSSSelector::UniversalSelectorAtom()));
    return static_cast<T*>(it != atomic_name_caches_.end() ? &*it->value
                                                           : nullptr);
  }

  TagCollectionNS* AddCache(ContainerNode& node,
                            const AtomicString& namespace_uri,
                            const AtomicString& local_name) {
    QualifiedName name(g_null_atom, local_name, namespace_uri);
    TagCollectionNSCache::AddResult result =
        tag_collection_ns_caches_.insert(name, nullptr);
    if (!result.is_new_entry)
      return result.stored_value->value.Get();

    auto* list = MakeGarbageCollected<TagCollectionNS>(
        node, kTagCollectionNSType, namespace_uri, local_name);
    result.stored_value->value = list;
    return list;
  }

  NodeListsNodeData() : child_node_list_(nullptr) {}
  NodeListsNodeData(const NodeListsNodeData&) = delete;
  NodeListsNodeData& operator=(const NodeListsNodeData&) = delete;

  void InvalidateCaches(const QualifiedName* attr_name = nullptr);

  bool IsEmpty() const {
    return !child_node_list_ && atomic_name_caches_.empty() &&
           tag_collection_ns_caches_.empty();
  }

  void AdoptTreeScope() { InvalidateCaches(); }

  void AdoptDocument(Document& old_document, Document& new_document) {
    DCHECK_NE(old_document, new_document);

    NodeListAtomicNameCacheMap::const_iterator atomic_name_cache_end =
        atomic_name_caches_.end();
    for (NodeListAtomicNameCacheMap::const_iterator it =
             atomic_name_caches_.begin();
         it != atomic_name_cache_end; ++it) {
      LiveNodeListBase* list = it->value.Get();
      list->DidMoveToDocument(old_document, new_document);
    }

    TagCollectionNSCache::const_iterator tag_end =
        tag_collection_ns_caches_.end();
    for (TagCollectionNSCache::const_iterator it =
             tag_collection_ns_caches_.begin();
         it != tag_end; ++it) {
      LiveNodeListBase* list = it->value.Get();
      DCHECK(!list->IsRootedAtTreeScope());
      list->DidMoveToDocument(old_document, new_document);
    }
  }

  void Trace(Visitor*) const;

 private:
  // Can be a ChildNodeList or an EmptyNodeList.
  Member<NodeList> child_node_list_;
  NodeListAtomicNameCacheMap atomic_name_caches_;
  TagCollectionNSCache tag_collection_ns_caches_;
};

template <typename Collection>
inline Collection* ContainerNode::EnsureCachedCollection(CollectionType type) {
  return EnsureNodeLists().AddCache<Collection>(*this, type);
}

template <typename Collection>
inline Collection* ContainerNode::EnsureCachedCollection(
    CollectionType type,
    const AtomicString& name) {
  return EnsureNodeLists().AddCache<Collection>(*this, type, name);
}

template <typename Collection>
inline Collection* ContainerNode::EnsureCachedCollection(
    CollectionType type,
    const AtomicString& namespace_uri,
    const AtomicString& local_name) {
  DCHECK_EQ(type, kTagCollectionNSType);
  return EnsureNodeLists().AddCache(*this, namespace_uri, local_name);
}

template <typename Collection>
inline Collection* ContainerNode::CachedCollection(CollectionType type) {
  NodeListsNodeData* node_lists = NodeLists();
  return node_lists ? node_lists->Cached<Collection>(type) : nullptr;
}

template <typename Collection>
inline const Collection* ContainerNode::CachedCollection(
    CollectionType type) const {
  const NodeListsNodeData* node_lists = NodeLists();
  return node_lists ? node_lists->Cached<Collection>(type) : nullptr;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_LISTS_NODE_DATA_H_
