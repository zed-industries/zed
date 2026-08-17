/*
 * Copyright (C) 2013 Apple Inc. All rights reserved.
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_COLLECTION_INDEX_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_COLLECTION_INDEX_CACHE_H_

#include "base/check_op.h"
#include "base/logging.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

template <typename Collection, typename NodeType>
class CollectionIndexCache {
  DISALLOW_NEW();

 public:
  CollectionIndexCache();

  bool IsEmpty(const Collection& collection) {
    if (IsCachedNodeCountValid())
      return !CachedNodeCount();
    if (CachedNode())
      return false;
    return !NodeAt(collection, 0);
  }
  bool HasExactlyOneNode(const Collection& collection) {
    if (IsCachedNodeCountValid())
      return CachedNodeCount() == 1;
    if (CachedNode())
      return !CachedNodeIndex() && !NodeAt(collection, 1);
    return NodeAt(collection, 0) && !NodeAt(collection, 1);
  }

  unsigned NodeCount(const Collection&);
  NodeType* NodeAt(const Collection&, unsigned index);

  void Invalidate();

  void NodeInserted();
  void NodeRemoved();

  virtual void Trace(Visitor* visitor) const { visitor->Trace(current_node_); }

 protected:
  ALWAYS_INLINE NodeType* CachedNode() const { return current_node_.Get(); }
  ALWAYS_INLINE unsigned CachedNodeIndex() const {
    DCHECK(CachedNode());
    return cached_node_index_;
  }
  ALWAYS_INLINE void SetCachedNode(NodeType* node, unsigned index) {
    DCHECK(node);
    current_node_ = node;
    cached_node_index_ = index;
  }

  ALWAYS_INLINE bool IsCachedNodeCountValid() const {
    return is_length_cache_valid_;
  }
  ALWAYS_INLINE unsigned CachedNodeCount() const { return cached_node_count_; }
  ALWAYS_INLINE void SetCachedNodeCount(unsigned length) {
    cached_node_count_ = length;
    is_length_cache_valid_ = true;
  }

 private:
  NodeType* NodeBeforeCachedNode(const Collection&, unsigned index);
  NodeType* NodeAfterCachedNode(const Collection&, unsigned index);

  Member<NodeType> current_node_;
  unsigned cached_node_count_;
  unsigned cached_node_index_ : 31;
  unsigned is_length_cache_valid_ : 1;
};

template <typename Collection, typename NodeType>
CollectionIndexCache<Collection, NodeType>::CollectionIndexCache()
    : current_node_(nullptr),
      cached_node_count_(0),
      cached_node_index_(0),
      is_length_cache_valid_(false) {}

template <typename Collection, typename NodeType>
void CollectionIndexCache<Collection, NodeType>::Invalidate() {
  current_node_ = nullptr;
  is_length_cache_valid_ = false;
}

template <typename Collection, typename NodeType>
void CollectionIndexCache<Collection, NodeType>::NodeInserted() {
  cached_node_count_++;
  current_node_ = nullptr;
}

template <typename Collection, typename NodeType>
void CollectionIndexCache<Collection, NodeType>::NodeRemoved() {
  cached_node_count_--;
  current_node_ = nullptr;
}

template <typename Collection, typename NodeType>
inline unsigned CollectionIndexCache<Collection, NodeType>::NodeCount(
    const Collection& collection) {
  if (IsCachedNodeCountValid())
    return CachedNodeCount();

  NodeAt(collection, UINT_MAX);
  DCHECK(IsCachedNodeCountValid());

  return CachedNodeCount();
}

template <typename Collection, typename NodeType>
inline NodeType* CollectionIndexCache<Collection, NodeType>::NodeAt(
    const Collection& collection,
    unsigned index) {
  if (IsCachedNodeCountValid() && index >= CachedNodeCount())
    return nullptr;

  if (CachedNode()) {
    if (index > CachedNodeIndex())
      return NodeAfterCachedNode(collection, index);
    if (index < CachedNodeIndex())
      return NodeBeforeCachedNode(collection, index);
    return CachedNode();
  }

  // No valid cache yet, let's find the first matching element.
  NodeType* first_node = collection.TraverseToFirst();
  if (!first_node) {
    // The collection is empty.
    SetCachedNodeCount(0);
    return nullptr;
  }
  SetCachedNode(first_node, 0);
  return index ? NodeAfterCachedNode(collection, index) : first_node;
}

template <typename Collection, typename NodeType>
inline NodeType*
CollectionIndexCache<Collection, NodeType>::NodeBeforeCachedNode(
    const Collection& collection,
    unsigned index) {
  DCHECK(CachedNode());  // Cache should be valid.
  unsigned current_index = CachedNodeIndex();
  DCHECK_GT(current_index, index);

  // Determine if we should traverse from the beginning of the collection
  // instead of the cached node.
  bool first_is_closer = index < current_index - index;
  if (first_is_closer || !collection.CanTraverseBackward()) {
    NodeType* first_node = collection.TraverseToFirst();
    DCHECK(first_node);
    SetCachedNode(first_node, 0);
    return index ? NodeAfterCachedNode(collection, index) : first_node;
  }

  // Backward traversal from the cached node to the requested index.
  DCHECK(collection.CanTraverseBackward());
  NodeType* current_node =
      collection.TraverseBackwardToOffset(index, *CachedNode(), current_index);
  DCHECK(current_node);
  SetCachedNode(current_node, current_index);
  return current_node;
}

template <typename Collection, typename NodeType>
inline NodeType*
CollectionIndexCache<Collection, NodeType>::NodeAfterCachedNode(
    const Collection& collection,
    unsigned index) {
  DCHECK(CachedNode());  // Cache should be valid.
  unsigned current_index = CachedNodeIndex();
  DCHECK_LT(current_index, index);

  // Determine if we should traverse from the end of the collection instead of
  // the cached node.
  bool last_is_closer = IsCachedNodeCountValid() &&
                        CachedNodeCount() - index < index - current_index;
  if (last_is_closer && collection.CanTraverseBackward()) {
    NodeType* last_item = collection.TraverseToLast();
    DCHECK(last_item);
    SetCachedNode(last_item, CachedNodeCount() - 1);
    if (index < CachedNodeCount() - 1)
      return NodeBeforeCachedNode(collection, index);
    return last_item;
  }

  // Forward traversal from the cached node to the requested index.
  NodeType* current_node =
      collection.TraverseForwardToOffset(index, *CachedNode(), current_index);
  if (!current_node) {
    // Did not find the node. On plus side, we now know the length.
    if (IsCachedNodeCountValid())
      DCHECK_EQ(current_index + 1, CachedNodeCount());
    SetCachedNodeCount(current_index + 1);
    return nullptr;
  }
  SetCachedNode(current_node, current_index);
  return current_node;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_COLLECTION_INDEX_CACHE_H_
