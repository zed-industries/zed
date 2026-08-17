/*
 * Copyright (C) 2005, 2006, 2007, 2008, 2011, 2012 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2011, Benjamin Poulain <ikipou@gmail.com>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_LINKED_HASH_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_LINKED_HASH_SET_H_

#include "third_party/blink/renderer/platform/wtf/allocator/partition_allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/sanitizers.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "third_party/blink/renderer/platform/wtf/vector_backed_linked_list.h"

namespace WTF {

// LinkedHashSet provides a Set interface like HashSet, but also has a
// predictable iteration order. It has O(1) insertion, removal, and test for
// containership. It maintains a linked list through its contents such that
// iterating it yields values in the order in which they were inserted.
// The linked list is implementing in a vector (with links being indexes instead
// of pointers), to simplify the move of backing during GC compaction.
//
// This container supports WeakMember<T>.
//
// LinkedHashSet iterators are not invalidated by mutation of the
// collection, unless they point to removed items. This means, for example, that
// you can safely modify the container while iterating over it generally, as
// long as you don't remove the current item. Moving items does not invalidate
// iterator, so that it may cause unexpected behavior (i.e. loop unexpectedly
// ends when moving the current item to last).
//
// Note: empty/deleted values as defined in HashTraits are not allowed.
template <typename ValueArg,
          typename TraitsArg = HashTraits<ValueArg>,
          typename Allocator = PartitionAllocator>
class LinkedHashSet {
  USE_ALLOCATOR(LinkedHashSet, Allocator);

 private:
  using Value = ValueArg;
  using Map =
      HashMap<Value, wtf_size_t, TraitsArg, HashTraits<wtf_size_t>, Allocator>;
  using ListType = VectorBackedLinkedList<Value, Allocator>;
  using BackingIterator = typename ListType::const_iterator;
  using BackingReverseIterator = typename ListType::const_reverse_iterator;
  using BackingConstIterator = typename ListType::const_iterator;

 public:
  using value_type = ValueArg;
  using reference = value_type&;
  using const_reference = const value_type&;
  using pointer = value_type*;
  using const_pointer = const value_type*;

  // TODO(keinakashima): add security check
  struct AddResult final {
    STACK_ALLOCATED();

   public:
    AddResult(const Value* stored_value, bool is_new_entry)
        : stored_value(stored_value), is_new_entry(is_new_entry) {}
    const Value* stored_value;
    bool is_new_entry;
  };

  template <typename T>
  class IteratorWrapper {
   public:
    using value_type = typename T::value_type;
    using size_type = typename T::size_type;
    using difference_type = typename T::difference_type;
    using pointer = typename T::pointer;
    using reference = typename T::reference;

    constexpr IteratorWrapper() = default;

    IteratorWrapper(const IteratorWrapper&) = default;
    IteratorWrapper& operator=(const IteratorWrapper&) = default;

    const Value& operator*() const { return *iterator_; }
    const Value* operator->() const { return &*iterator_; }

    IteratorWrapper& operator++() {
      ++iterator_;
      return *this;
    }

    IteratorWrapper& operator--() {
      --iterator_;
      return *this;
    }

    IteratorWrapper operator++(int) {
      auto copy = *this;
      operator++();
      return copy;
    }

    IteratorWrapper operator--(int) {
      auto copy = *this;
      operator--();
      return copy;
    }

    bool operator==(const IteratorWrapper& other) const {
      // No need to compare map_iterator_ here because it is not related to
      // iterator_'s value but only for strongifying WeakMembers for the
      // lifetime of this IteratorWrapper.
      return iterator_ == other.iterator_;
    }

    bool operator!=(const IteratorWrapper& other) const {
      return !(*this == other);
    }

   protected:
    IteratorWrapper(const T& it, const Map& map)
        : iterator_(it), map_iterator_(map.begin()) {}

    // LinkedHashSet::list_ iterator.
    T iterator_;

    // This is needed for WeakMember support in LinkedHashSet. Holding
    // value_to_index_'s iterator to map, for the lifetime of this iterator,
    // will strongify WeakMembers in both value_to_index_ as well as their
    // copies inside list_. This is necessary to prevent list_'s weak callback
    // to remove dead weak entries while an active iterator exists.
    typename Map::const_iterator map_iterator_;

    friend class LinkedHashSet<ValueArg, TraitsArg, Allocator>;
  };

  using iterator = IteratorWrapper<BackingIterator>;
  using const_iterator = IteratorWrapper<BackingIterator>;
  using reverse_iterator = IteratorWrapper<BackingReverseIterator>;
  using const_reverse_iterator = IteratorWrapper<BackingReverseIterator>;

  typedef typename TraitsArg::PeekInType ValuePeekInType;

  LinkedHashSet() = default;
  LinkedHashSet(const LinkedHashSet&) = default;
  LinkedHashSet(LinkedHashSet&&) = default;
  LinkedHashSet& operator=(const LinkedHashSet&) = default;
  LinkedHashSet& operator=(LinkedHashSet&&) = default;

  ~LinkedHashSet() = default;

  void Swap(LinkedHashSet&);

  wtf_size_t size() const {
    DCHECK(value_to_index_.size() == list_.size());
    return list_.size();
  }
  bool empty() const { return list_.empty(); }

  iterator begin() { return MakeIterator(list_.begin()); }
  const_iterator begin() const { return MakeIterator(list_.cbegin()); }
  const_iterator cbegin() const { return MakeIterator(list_.cbegin()); }
  iterator end() { return MakeIterator(list_.end()); }
  const_iterator end() const { return MakeIterator(list_.cend()); }
  const_iterator cend() const { return MakeIterator(list_.cend()); }

  reverse_iterator rbegin() { return MakeReverseIterator(list_.rbegin()); }
  const_reverse_iterator rbegin() const {
    return MakeReverseIterator(list_.crbegin());
  }
  const_reverse_iterator crbegin() const {
    return MakeReverseIterator(list_.crbegin());
  }
  reverse_iterator rend() { return MakeReverseIterator(list_.rend()); }
  const_reverse_iterator rend() const {
    return MakeReverseIterator(list_.crend());
  }
  const_reverse_iterator crend() const {
    return MakeReverseIterator(list_.crend());
  }

  const Value& front() const { return list_.front(); }
  const Value& back() const { return list_.back(); }

  iterator find(ValuePeekInType);
  const_iterator find(ValuePeekInType) const;
  bool Contains(ValuePeekInType) const;

  // An alternate version of find() that finds the object by hashing and
  // comparing with some other type, to avoid the cost of type conversion.
  // The HashTranslator interface is defined in HashSet.
  template <typename HashTranslator, typename T>
  iterator Find(const T&);
  template <typename HashTranslator, typename T>
  const_iterator Find(const T&) const;
  template <typename HashTranslator, typename T>
  bool Contains(const T&) const;

  template <typename IncomingValueType>
  AddResult insert(IncomingValueType&&);

  // If |value| already exists in the set, nothing happens.
  // If |before_value| doesn't exist in the set, appends |value|.
  template <typename IncomingValueType>
  AddResult InsertBefore(ValuePeekInType before_value,
                         IncomingValueType&& value);

  template <typename IncomingValueType>
  AddResult InsertBefore(const_iterator it, IncomingValueType&& value);

  template <typename IncomingValueType>
  AddResult AppendOrMoveToLast(IncomingValueType&&);

  template <typename IncomingValueType>
  AddResult PrependOrMoveToFirst(IncomingValueType&&);

  // Moves |target| right before |new_position| in a linked list. This operation
  // is executed by just updating indices of related nodes.
  void MoveTo(const_iterator target, const_iterator new_position);

  void erase(ValuePeekInType);
  void erase(const_iterator);
  void RemoveFirst();
  void pop_back();
  void clear();

  void Trace(auto visitor) const
    requires Allocator::kIsGarbageCollected
  {
    value_to_index_.Trace(visitor);
    list_.Trace(visitor);
  }

 private:
  enum class MoveType {
    kMoveIfValueExists,
    kDontMove,
  };

  class GCForbiddenScope {
    STACK_ALLOCATED();

   public:
    GCForbiddenScope() { Allocator::EnterGCForbiddenScope(); }
    ~GCForbiddenScope() { Allocator::LeaveGCForbiddenScope(); }
  };

  template <typename IncomingValueType>
  AddResult InsertOrMoveBefore(const_iterator, IncomingValueType&&, MoveType);

  iterator MakeIterator(const BackingIterator& it) const {
    return iterator(it, value_to_index_);
  }

  reverse_iterator MakeReverseIterator(const BackingReverseIterator& it) const {
    return reverse_iterator(it, value_to_index_);
  }

  Map value_to_index_;
  ListType list_;

  struct TypeConstraints {
    constexpr TypeConstraints() {
      static_assert(!IsStackAllocatedTypeV<Value>);
      static_assert(Allocator::kIsGarbageCollected ||
                        !IsPointerToGarbageCollectedType<Value>,
                    "Cannot put raw pointers to garbage-collected classes into "
                    "an off-heap LinkedHashSet. Use "
                    "HeapLinkedHashSet<Member<T>> instead.");
    }
  };
  NO_UNIQUE_ADDRESS TypeConstraints type_constraints_;
};

template <typename T, typename TraitsArg, typename Allocator>
inline void LinkedHashSet<T, TraitsArg, Allocator>::Swap(LinkedHashSet& other) {
  value_to_index_.swap(other.value_to_index_);
  list_.swap(other.list_);
}

template <typename T, typename TraitsArg, typename Allocator>
typename LinkedHashSet<T, TraitsArg, Allocator>::iterator
LinkedHashSet<T, TraitsArg, Allocator>::find(ValuePeekInType value) {
  typename Map::const_iterator it = value_to_index_.find(value);

  if (it == value_to_index_.end())
    return end();
  return MakeIterator(list_.MakeIterator(it->value));
}

template <typename T, typename TraitsArg, typename Allocator>
typename LinkedHashSet<T, TraitsArg, Allocator>::const_iterator
LinkedHashSet<T, TraitsArg, Allocator>::find(ValuePeekInType value) const {
  typename Map::const_iterator it = value_to_index_.find(value);

  if (it == value_to_index_.end())
    return end();
  return MakeIterator(list_.MakeConstIterator(it->value));
}

template <typename T, typename TraitsArg, typename Allocator>
bool LinkedHashSet<T, TraitsArg, Allocator>::Contains(
    ValuePeekInType value) const {
  return value_to_index_.Contains(value);
}

template <typename ValueType, typename TraitsArg, typename Allocator>
template <typename HashTranslator, typename T>
inline typename LinkedHashSet<ValueType, TraitsArg, Allocator>::iterator
LinkedHashSet<ValueType, TraitsArg, Allocator>::Find(const T& value) {
  typename Map::const_iterator it =
      value_to_index_.template Find<HashTranslator>(value);
  if (it == value_to_index_.end())
    return end();
  return MakeIterator(list_.MakeIterator(it->value));
}

template <typename ValueType, typename TraitsArg, typename Allocator>
template <typename HashTranslator, typename T>
inline typename LinkedHashSet<ValueType, TraitsArg, Allocator>::const_iterator
LinkedHashSet<ValueType, TraitsArg, Allocator>::Find(const T& value) const {
  typename Map::const_iterator it =
      value_to_index_.template Find<HashTranslator>(value);
  if (it == value_to_index_.end())
    return end();
  return MakeIterator(list_.MakeConstIterator(it->value));
}

template <typename ValueType, typename TraitsArg, typename Allocator>
template <typename HashTranslator, typename T>
bool LinkedHashSet<ValueType, TraitsArg, Allocator>::Contains(
    const T& value) const {
  return value_to_index_.template Contains<HashTranslator>(value);
}

template <typename T, typename TraitsArg, typename Allocator>
template <typename IncomingValueType>
typename LinkedHashSet<T, TraitsArg, Allocator>::AddResult
LinkedHashSet<T, TraitsArg, Allocator>::insert(IncomingValueType&& value) {
  return InsertOrMoveBefore(end(), std::forward<IncomingValueType>(value),
                            MoveType::kDontMove);
}

template <typename T, typename TraitsArg, typename Allocator>
template <typename IncomingValueType>
typename LinkedHashSet<T, TraitsArg, Allocator>::AddResult
LinkedHashSet<T, TraitsArg, Allocator>::InsertBefore(
    ValuePeekInType before_value,
    IncomingValueType&& value) {
  return InsertOrMoveBefore(find(before_value),
                            std::forward<IncomingValueType>(value),
                            MoveType::kDontMove);
}

template <typename T, typename TraitsArg, typename Allocator>
template <typename IncomingValueType>
typename LinkedHashSet<T, TraitsArg, Allocator>::AddResult
LinkedHashSet<T, TraitsArg, Allocator>::InsertBefore(
    const_iterator it,
    IncomingValueType&& value) {
  return InsertOrMoveBefore(it, std::forward<IncomingValueType>(value),
                            MoveType::kDontMove);
}

template <typename T, typename TraitsArg, typename Allocator>
template <typename IncomingValueType>
typename LinkedHashSet<T, TraitsArg, Allocator>::AddResult
LinkedHashSet<T, TraitsArg, Allocator>::AppendOrMoveToLast(
    IncomingValueType&& value) {
  return InsertOrMoveBefore(end(), std::forward<IncomingValueType>(value),
                            MoveType::kMoveIfValueExists);
}

template <typename T, typename TraitsArg, typename Allocator>
template <typename IncomingValueType>
typename LinkedHashSet<T, TraitsArg, Allocator>::AddResult
LinkedHashSet<T, TraitsArg, Allocator>::PrependOrMoveToFirst(
    IncomingValueType&& value) {
  return InsertOrMoveBefore(begin(), std::forward<IncomingValueType>(value),
                            MoveType::kMoveIfValueExists);
}

template <typename T, typename TraitsArg, typename Allocator>
void LinkedHashSet<T, TraitsArg, Allocator>::MoveTo(
    const_iterator target,
    const_iterator new_position) {
  list_.MoveTo(target.iterator_, new_position.iterator_);
}

template <typename T, typename TraitsArg, typename Allocator>
inline void LinkedHashSet<T, TraitsArg, Allocator>::erase(
    ValuePeekInType value) {
  erase(find(value));
}

template <typename T, typename TraitsArg, typename Allocator>
inline void LinkedHashSet<T, TraitsArg, Allocator>::erase(const_iterator it) {
  if (it == end())
    return;

  // Forbid GC while modifying LinkedHashSet to avoid conflict between
  // |value_to_index_| and |list_|.
  auto scope = GCForbiddenScope();

  value_to_index_.erase(*it);
  list_.erase(it.iterator_);
}

template <typename T, typename TraitsArg, typename Allocator>
inline void LinkedHashSet<T, TraitsArg, Allocator>::RemoveFirst() {
  DCHECK(!empty());
  erase(begin());
}

template <typename T, typename TraitsArg, typename Allocator>
inline void LinkedHashSet<T, TraitsArg, Allocator>::pop_back() {
  DCHECK(!empty());
  erase(--end());
}

template <typename T, typename TraitsArg, typename Allocator>
inline void LinkedHashSet<T, TraitsArg, Allocator>::clear() {
  // Forbid GC while modifying LinkedHashSet to avoid conflict between
  // |value_to_index_| and |list_|.
  auto scope = GCForbiddenScope();

  value_to_index_.clear();
  list_.clear();
}

template <typename T, typename TraitsArg, typename Allocator>
template <typename IncomingValueType>
typename LinkedHashSet<T, TraitsArg, Allocator>::AddResult
LinkedHashSet<T, TraitsArg, Allocator>::InsertOrMoveBefore(
    const_iterator position,
    IncomingValueType&& value,
    MoveType type) {
  // Forbid GC while modifying LinkedHashSet to avoid conflict between
  // |value_to_index_| and |list_|.
  auto scope = GCForbiddenScope();

  typename Map::AddResult result = value_to_index_.insert(value, kNotFound);

  if (result.is_new_entry) {
    BackingConstIterator stored_position_iterator = list_.insert(
        position.iterator_, std::forward<IncomingValueType>(value));
    result.stored_value->value = stored_position_iterator.GetIndex();
    return AddResult(&*stored_position_iterator, true);
  }

  BackingConstIterator stored_position_iterator =
      list_.MakeConstIterator(result.stored_value->value);
  if (type == MoveType::kDontMove)
    return AddResult(&*stored_position_iterator, false);

  BackingConstIterator moved_position_iterator =
      list_.MoveTo(stored_position_iterator, position.iterator_);
  return AddResult(&*moved_position_iterator, false);
}

}  // namespace WTF

using WTF::LinkedHashSet;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_LINKED_HASH_SET_H_
