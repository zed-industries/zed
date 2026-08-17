// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_PRIORITY_QUEUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_PRIORITY_QUEUE_H_

#include "base/check_op.h"
#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

// A basic priority queue based on a (binary min) heap.
//
// The queue tracks the position in the heap vector by storing (and
// maintaining) an index in the entry element itself using a
// PriorityQueueHandle() accessor.
//
// While it appears to be generic, it's currently only to make testing easier.
template <typename PriorityType, typename ElementType>
class PriorityQueue {
  DISALLOW_NEW();

 public:
  using EntryType = std::pair<PriorityType, Member<ElementType>>;
  using StorageType = HeapVector<EntryType>;
  using iterator = typename StorageType::iterator;
  using const_iterator = typename StorageType::const_iterator;

  PriorityQueue() = default;
  PriorityQueue(const PriorityQueue&) = delete;
  PriorityQueue& operator=(const PriorityQueue&) = delete;

  bool Contains(ElementType* element) const {
    return element->PriorityQueueHandle() != kNotFound;
  }
  void Insert(PriorityType priority, ElementType* element);
  // Updates the position of the specified object in the queue to the new
  // priority |priority|.
  void Update(PriorityType priority, ElementType* element);
  void Remove(ElementType* element);

  // Set the priority of all entries to |priority|.
  void ResetAllPriorities(PriorityType priority);

  wtf_size_t size() const { return heap_.size(); }
  bool IsEmpty() const { return heap_.empty(); }
  const PriorityType& Min() const { return heap_.front().first; }
  ElementType* MinElement() const { return heap_.front().second.Get(); }

  iterator begin() { return heap_.begin(); }
  iterator end() { return heap_.end(); }
  const_iterator begin() const { return heap_.begin(); }
  const_iterator end() const { return heap_.end(); }

  const EntryType& operator[](wtf_size_t index) const { return heap_[index]; }

  void Trace(Visitor* visitor) const { visitor->Trace(heap_); }

 private:
  FRIEND_TEST_ALL_PREFIXES(PriorityQueueTest, Updates);

  wtf_size_t PercolateUp(wtf_size_t index);
  void PercolateDown(wtf_size_t index);
  wtf_size_t SmallestChildIndex(wtf_size_t index) const;

  static bool IsRoot(wtf_size_t index) { return index == 0; }
  static wtf_size_t ParentIndex(wtf_size_t index) {
    DCHECK_NE(index, 0u);
    return (index - 1) / 2;
  }
  static wtf_size_t LeftChildIndex(wtf_size_t index) { return 2 * index + 1; }
  static bool CompareLess(const EntryType& a, const EntryType& b) {
    return a.first < b.first;
  }
  static void Swap(EntryType& a, EntryType& b) {
    std::swap(a.second->PriorityQueueHandle(), b.second->PriorityQueueHandle());
    std::swap(a, b);
  }

  StorageType heap_;
};

template <typename PriorityType, typename ElementType>
inline wtf_size_t PriorityQueue<PriorityType, ElementType>::PercolateUp(
    wtf_size_t index) {
  while (!IsRoot(index)) {
    wtf_size_t parent_index = ParentIndex(index);
    if (!CompareLess(heap_[index], heap_[parent_index]))
      break;
    Swap(heap_[index], heap_[parent_index]);
    index = parent_index;
  }
  return index;
}

template <typename PriorityType, typename ElementType>
inline wtf_size_t PriorityQueue<PriorityType, ElementType>::SmallestChildIndex(
    wtf_size_t index) const {
  wtf_size_t left_child_index = LeftChildIndex(index);
  if (left_child_index >= heap_.size())
    return left_child_index;
  wtf_size_t right_child_index = left_child_index + 1;
  if (right_child_index < heap_.size() &&
      CompareLess(heap_[right_child_index], heap_[left_child_index]))
    return right_child_index;
  return left_child_index;
}

template <typename PriorityType, typename ElementType>
inline void PriorityQueue<PriorityType, ElementType>::PercolateDown(
    wtf_size_t index) {
  while (true) {
    wtf_size_t smallest_child_index = SmallestChildIndex(index);
    if (smallest_child_index >= heap_.size())
      break;
    if (!CompareLess(heap_[smallest_child_index], heap_[index]))
      break;
    Swap(heap_[smallest_child_index], heap_[index]);
    index = smallest_child_index;
  }
}

template <typename PriorityType, typename ElementType>
inline void PriorityQueue<PriorityType, ElementType>::Insert(
    PriorityType priority,
    ElementType* element) {
  DCHECK(!Contains(element));
  heap_.push_back(EntryType{priority, element});
  wtf_size_t new_index = heap_.size() - 1;
  element->PriorityQueueHandle() = new_index;
  PercolateUp(new_index);
}

template <typename PriorityType, typename ElementType>
inline void PriorityQueue<PriorityType, ElementType>::Remove(
    ElementType* element) {
  DCHECK(Contains(element));
  wtf_size_t index = element->PriorityQueueHandle();
  Swap(heap_[index], heap_.back());
  heap_.pop_back();
  element->PriorityQueueHandle() = kNotFound;
  if (index == heap_.size() || PercolateUp(index) != index)
    return;
  PercolateDown(index);
}

template <typename PriorityType, typename ElementType>
inline void PriorityQueue<PriorityType, ElementType>::Update(
    PriorityType priority,
    ElementType* element) {
  DCHECK(Contains(element));
  wtf_size_t index = element->PriorityQueueHandle();
  heap_[index].first = priority;
  if (PercolateUp(index) != index)
    return;
  PercolateDown(index);
}

template <typename PriorityType, typename ElementType>
inline void PriorityQueue<PriorityType, ElementType>::ResetAllPriorities(
    PriorityType priority) {
  for (auto& entry : heap_)
    entry.first = priority;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_PRIORITY_QUEUE_H_
