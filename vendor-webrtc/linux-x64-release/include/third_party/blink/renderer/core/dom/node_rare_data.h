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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_RARE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_RARE_DATA_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/thread_state_storage.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

enum class DynamicRestyleFlags;
enum class ElementFlags;
class FlatTreeNodeData;
class MutationObserverRegistration;
class NodeListsNodeData;
class NodeRareData;
class Part;
class ScrollTimeline;

using PartsList = GCedHeapDeque<Member<Part>>;
using TemporaryPartsList = HeapDeque<Member<Part>>;

class NodeMutationObserverData final
    : public GarbageCollected<NodeMutationObserverData> {
 public:
  NodeMutationObserverData() = default;
  NodeMutationObserverData(const NodeMutationObserverData&) = delete;
  NodeMutationObserverData& operator=(const NodeMutationObserverData&) = delete;

  const HeapVector<Member<MutationObserverRegistration>>& Registry() {
    return registry_;
  }

  const HeapHashSet<Member<MutationObserverRegistration>>& TransientRegistry() {
    return transient_registry_;
  }

  void AddTransientRegistration(MutationObserverRegistration* registration);
  void RemoveTransientRegistration(MutationObserverRegistration* registration);
  void AddRegistration(MutationObserverRegistration* registration);
  void RemoveRegistration(MutationObserverRegistration* registration);

  void Trace(Visitor* visitor) const;

 private:
  HeapVector<Member<MutationObserverRegistration>> registry_;
  HeapHashSet<Member<MutationObserverRegistration>> transient_registry_;
};

class NodeRareData : public GarbageCollected<NodeRareData> {
 public:
  enum {
    kConnectedFrameCountBits = 10,  // Must fit Page::maxNumberOfFrames.
    kNumberOfElementFlags = 8,
    kNumberOfDynamicRestyleFlags = 14
    // 0 bits remaining.
  };

  NodeRareData() = default;
  virtual ~NodeRareData() = default;
  NodeRareData(const NodeRareData&) = delete;
  NodeRareData& operator=(const NodeRareData&) = delete;

  void ClearNodeLists() { node_lists_.Clear(); }
  NodeListsNodeData* NodeLists() const { return node_lists_.Get(); }
  // EnsureNodeLists() and a following NodeListsNodeData functions must be
  // wrapped with a ThreadState::GCForbiddenScope in order to avoid an
  // initialized node_lists_ is cleared by NodeRareData::TraceAfterDispatch().
  NodeListsNodeData& EnsureNodeLists() {
    if (!node_lists_)
      return CreateNodeLists();
    return *node_lists_;
  }

  FlatTreeNodeData* GetFlatTreeNodeData() const {
    return flat_tree_node_data_.Get();
  }
  FlatTreeNodeData& EnsureFlatTreeNodeData();

  NodeMutationObserverData* MutationObserverData() {
    return mutation_observer_data_.Get();
  }
  NodeMutationObserverData& EnsureMutationObserverData() {
    if (!mutation_observer_data_) {
      mutation_observer_data_ =
          MakeGarbageCollected<NodeMutationObserverData>();
    }
    return *mutation_observer_data_;
  }

  uint16_t ConnectedSubframeCount() const { return connected_frame_count_; }
  void IncrementConnectedSubframeCount();
  void DecrementConnectedSubframeCount() {
    DCHECK(connected_frame_count_);
    --connected_frame_count_;
  }

  bool HasElementFlag(ElementFlags mask) const {
    return element_flags_ & static_cast<uint16_t>(mask);
  }
  void SetElementFlag(ElementFlags mask, bool value) {
    element_flags_ =
        (element_flags_ & ~static_cast<uint16_t>(mask)) |
        (-static_cast<uint16_t>(value) & static_cast<uint16_t>(mask));
  }
  void ClearElementFlag(ElementFlags mask) {
    element_flags_ &= ~static_cast<uint16_t>(mask);
  }

  bool HasRestyleFlag(DynamicRestyleFlags mask) const {
    return restyle_flags_ & static_cast<uint16_t>(mask);
  }
  void SetRestyleFlag(DynamicRestyleFlags mask) {
    restyle_flags_ |= static_cast<uint16_t>(mask);
  }
  bool HasRestyleFlags() const { return restyle_flags_; }
  void ClearRestyleFlags() { restyle_flags_ = 0u; }

  void RegisterScrollTimeline(ScrollTimeline*);
  void UnregisterScrollTimeline(ScrollTimeline*);
  void InvalidateAssociatedAnimationEffects();

  void AddDOMPart(Part& part);
  void RemoveDOMPart(Part& part);
  PartsList* GetDOMParts() const;

  DOMNodeId NodeId() const { return id_; }
  DOMNodeId& NodeId() { return id_; }

  virtual void Trace(Visitor*) const;

 protected:
  uint32_t restyle_flags_ : kNumberOfDynamicRestyleFlags = 0u;
  uint32_t connected_frame_count_ : kConnectedFrameCountBits = 0u;
  uint32_t element_flags_ : kNumberOfElementFlags = 0u;

 private:
  NodeListsNodeData& CreateNodeLists();

  Member<NodeListsNodeData> node_lists_;
  Member<NodeMutationObserverData> mutation_observer_data_;
  Member<FlatTreeNodeData> flat_tree_node_data_;
  // Keeps strong scroll timeline pointers linked to this node to ensure
  // the timelines are alive as long as the node is alive.
  Member<GCedHeapHashSet<Member<ScrollTimeline>>> scroll_timelines_;
  // An ordered set of DOM Parts for this Node, in order of construction. This
  // order is important, since `getParts()` returns a tree-ordered set of parts,
  // with parts on the same `Node` returned in `Part` construction order.
  Member<PartsList> dom_parts_;
  DOMNodeId id_ = kInvalidDOMNodeId;  // Used primarily for accessibility.
};

template <typename T>
struct ThreadingTrait<
    T,
    std::enable_if_t<std::is_base_of<blink::NodeRareData, T>::value>> {
  static constexpr ThreadAffinity kAffinity = kMainThreadOnly;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_RARE_DATA_H_
