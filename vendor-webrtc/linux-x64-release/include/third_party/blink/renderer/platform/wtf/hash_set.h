/*
 * Copyright (C) 2005, 2006, 2007, 2008, 2011 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_HASH_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_HASH_SET_H_

#include <initializer_list>

#include "base/numerics/safe_conversions.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partition_allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_table.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {

struct IdentityExtractor;

// Note: empty or deleted values are not allowed, using them may lead to
// undefined behavior. For pointer keys this means that null pointers are not
// allowed; for integer keys 0 or -1 can't be used as a key. This restriction
// can be lifted if you supply custom key traits.
// See hash_traits.h for how to define hash traits.
template <typename ValueArg,
          typename TraitsArg = HashTraits<ValueArg>,
          typename Allocator = PartitionAllocator>
class HashSet {
  USE_ALLOCATOR(HashSet, Allocator);

 private:
  typedef TraitsArg ValueTraits;
  typedef typename ValueTraits::PeekInType ValuePeekInType;

 public:
  typedef typename ValueTraits::TraitType ValueType;
  using value_type = ValueType;
  using reference = value_type&;
  using const_reference = const value_type&;
  using pointer = value_type*;
  using const_pointer = const value_type*;

 private:
  typedef HashTable<ValueType,
                    ValueType,
                    IdentityExtractor,
                    ValueTraits,
                    ValueTraits,
                    Allocator>
      HashTableType;

 public:
  typedef HashTableConstIteratorAdapter<HashTableType, ValueTraits> iterator;
  typedef HashTableConstIteratorAdapter<HashTableType, ValueTraits>
      const_iterator;
  typedef typename HashTableType::AddResult AddResult;

  HashSet() = default;
  HashSet(const HashSet&) = default;
  HashSet& operator=(const HashSet&) = default;
  HashSet(HashSet&&) = default;
  HashSet& operator=(HashSet&&) = default;

  HashSet(std::initializer_list<ValueType> elements);
  HashSet& operator=(std::initializer_list<ValueType> elements);

  // Useful for constructing from, for example, STL and base sets.
  template <typename It>
    requires(std::forward_iterator<It>)
  HashSet(It begin, It end);

  void swap(HashSet& ref) { impl_.swap(ref.impl_); }

  unsigned size() const;
  unsigned Capacity() const;
  bool empty() const;

  void ReserveCapacityForSize(unsigned size) {
    impl_.ReserveCapacityForSize(size);
  }

  iterator begin() const;
  iterator end() const;

  // Returns an iterator to the found element, or end() if not found.
  iterator find(ValuePeekInType) const;
  bool Contains(ValuePeekInType) const;

  // An alternate version of find() that finds the object by hashing and
  // comparing with some other type, to avoid the cost of type
  // conversion. HashTranslator must have the following function members:
  //   static unsigned GetHash(const T&);
  //   static bool Equal(const ValueType&, const T&);
  template <typename HashTranslator, typename T>
  iterator Find(const T&) const;
  template <typename HashTranslator, typename T>
  bool Contains(const T&) const;

  // The return value is a pair of an iterator to the new value's location,
  // and a bool that is true if an new entry was added.
  template <typename IncomingValueType>
  AddResult insert(IncomingValueType&&);

  // An alternate version of insert() that finds the object by hashing and
  // comparing with some other type, to avoid the cost of type conversion if
  // the object is already in the table. HashTranslator must have the
  // following function members:
  //   static unsigned GetHash(const T&);
  //   static bool Equal(const ValueType&, const T&);
  //   static Store(ValueType&, T&&, unsigned hash_code);
  template <typename HashTranslator, typename T>
  AddResult AddWithTranslator(T&&);

  // Does nothing if the value is not found.
  // NOTE: You cannot continue using an iterator after erase()
  // (no modifications are allowed during iteration). Consider erase_if()
  // or RemoveAll().
  void erase(ValuePeekInType);
  void erase(iterator);

  // Erases all elements for which pred(element) returns true.
  //
  // The predicate should have a signature compatible with:
  //   bool pred(const ValueType&);
  template <typename Pred>
  void erase_if(Pred pred);

  void clear();
  template <typename Collection>
  void RemoveAll(const Collection& to_be_removed) {
    WTF::RemoveAll(*this, to_be_removed);
  }

  ValueType Take(iterator);
  ValueType Take(ValuePeekInType);
  ValueType TakeAny();

  std::unique_ptr<HashSet> Clone() const {
    return std::make_unique<HashSet>(*this);
  }

  void Trace(auto visitor) const
    requires Allocator::kIsGarbageCollected
  {
    impl_.Trace(visitor);
  }

 protected:
  ValueType** GetBufferSlot() { return impl_.GetBufferSlot(); }

 private:
  HashTableType impl_;

  struct TypeConstraints {
    constexpr TypeConstraints() {
      static_assert(!IsStackAllocatedTypeV<ValueArg>);
      static_assert(Allocator::kIsGarbageCollected ||
                        !IsPointerToGarbageCollectedType<ValueArg>,
                    "Cannot put raw pointers to garbage-collected classes into "
                    "an off-heap HashSet. Use HeapHashSet<Member<T>> instead.");
    }
  };
  NO_UNIQUE_ADDRESS TypeConstraints type_constraints_;
};

struct IdentityExtractor {
  STATIC_ONLY(IdentityExtractor);
  template <typename T>
  static const T& ExtractKey(const T& t) {
    return t;
  }
  template <typename T>
  static T& ExtractKey(T& t) {
    return t;
  }
  // Assumes out points to a buffer of size at least sizeof(T).
  template <typename T>
  static void ExtractKeyToMemory(const T& t, void* out) {
    AtomicReadMemcpy<sizeof(T), alignof(T)>(out, &t);
  }
  template <typename T>
  static void ClearValue(const T&) {}
};

template <typename Translator>
struct HashSetTranslatorAdapter {
  STATIC_ONLY(HashSetTranslatorAdapter);
  template <typename T>
  static unsigned GetHash(const T& key) {
    return Translator::GetHash(key);
  }
  template <typename T, typename U>
  static bool Equal(const T& a, const U& b) {
    return Translator::Equal(a, b);
  }
  template <typename T, typename U, typename V>
  static void Store(T& location, U&& key, const V&, unsigned hash_code) {
    Translator::Store(location, std::forward<U>(key), hash_code);
  }
};

template <typename Value, typename Traits, typename Allocator>
HashSet<Value, Traits, Allocator>::HashSet(
    std::initializer_list<ValueType> elements) {
  if (elements.size()) {
    impl_.ReserveCapacityForSize(
        base::checked_cast<wtf_size_t>(elements.size()));
  }
  for (const ValueType& element : elements)
    insert(element);
}

template <typename Value, typename Traits, typename Allocator>
auto HashSet<Value, Traits, Allocator>::operator=(
    std::initializer_list<ValueType> elements) -> HashSet& {
  *this = HashSet(std::move(elements));
  return *this;
}

template <typename Value, typename Traits, typename Allocator>
template <typename It>
  requires(std::forward_iterator<It>)
HashSet<Value, Traits, Allocator>::HashSet(It begin, It end) {
  if constexpr (std::random_access_iterator<It>) {
    ReserveCapacityForSize(base::checked_cast<wtf_size_t>(end - begin));
  }
  for (; begin != end; ++begin) {
    insert(*begin);
  }
}

template <typename T, typename U, typename V>
bool operator==(const HashSet<T, U, V>& a, const HashSet<T, U, V>& b) {
  if (a.size() != b.size())
    return false;

  const auto a_end = a.end();
  const auto b_end = b.end();
  for (auto it = a.begin(); it != a_end; ++it) {
    if (b.find(*it) == b_end)
      return false;
  }

  return true;
}

template <typename T, typename U, typename V>
inline bool operator!=(const HashSet<T, U, V>& a, const HashSet<T, U, V>& b) {
  return !(a == b);
}

template <typename T, typename U, typename V>
inline unsigned HashSet<T, U, V>::size() const {
  return impl_.size();
}

template <typename T, typename U, typename V>
inline unsigned HashSet<T, U, V>::Capacity() const {
  return impl_.Capacity();
}

template <typename T, typename U, typename V>
inline bool HashSet<T, U, V>::empty() const {
  return impl_.empty();
}

template <typename T, typename U, typename V>
inline typename HashSet<T, U, V>::iterator HashSet<T, U, V>::begin() const {
  return impl_.begin();
}

template <typename T, typename U, typename V>
inline typename HashSet<T, U, V>::iterator HashSet<T, U, V>::end() const {
  return impl_.end();
}

template <typename T, typename U, typename V>
inline typename HashSet<T, U, V>::iterator HashSet<T, U, V>::find(
    ValuePeekInType value) const {
  return impl_.find(value);
}

template <typename Value, typename Traits, typename Allocator>
inline bool HashSet<Value, Traits, Allocator>::Contains(
    ValuePeekInType value) const {
  return impl_.Contains(value);
}

template <typename Value, typename Traits, typename Allocator>
template <typename HashTranslator, typename T>
typename HashSet<Value, Traits, Allocator>::
    iterator inline HashSet<Value, Traits, Allocator>::Find(
        const T& value) const {
  return impl_.template Find<HashSetTranslatorAdapter<HashTranslator>>(value);
}

template <typename Value, typename Traits, typename Allocator>
template <typename HashTranslator, typename T>
inline bool HashSet<Value, Traits, Allocator>::Contains(const T& value) const {
  return impl_.template Contains<HashSetTranslatorAdapter<HashTranslator>>(
      value);
}

template <typename T, typename U, typename V>
template <typename IncomingValueType>
inline typename HashSet<T, U, V>::AddResult HashSet<T, U, V>::insert(
    IncomingValueType&& value) {
  return impl_.insert(std::forward<IncomingValueType>(value));
}

template <typename Value, typename Traits, typename Allocator>
template <typename HashTranslator, typename T>
inline typename HashSet<Value, Traits, Allocator>::AddResult
HashSet<Value, Traits, Allocator>::AddWithTranslator(T&& value) {
  // Forward only the first argument, because the second argument isn't actually
  // used in HashSetTranslatorAdapter.
  return impl_
      .template InsertPassingHashCode<HashSetTranslatorAdapter<HashTranslator>>(
          std::forward<T>(value), value);
}

template <typename T, typename U, typename V>
inline void HashSet<T, U, V>::erase(iterator it) {
  impl_.erase(it.impl_);
}

template <typename T, typename U, typename V>
inline void HashSet<T, U, V>::erase(ValuePeekInType value) {
  erase(find(value));
}

template <typename T, typename U, typename V>
template <typename Pred>
inline void HashSet<T, U, V>::erase_if(Pred pred) {
  impl_.erase_if(std::forward<Pred>(pred));
}

template <typename T, typename U, typename V>
inline void HashSet<T, U, V>::clear() {
  impl_.clear();
}

template <typename T, typename U, typename V>
inline auto HashSet<T, U, V>::Take(iterator it) -> ValueType {
  if (it == end())
    return ValueTraits::EmptyValue();

  ValueType result = std::move(const_cast<ValueType&>(*it));
  erase(it);

  return result;
}

template <typename T, typename U, typename V>
inline auto HashSet<T, U, V>::Take(ValuePeekInType value) -> ValueType {
  return Take(find(value));
}

template <typename T, typename U, typename V>
inline auto HashSet<T, U, V>::TakeAny() -> ValueType {
  return Take(begin());
}

}  // namespace WTF

using WTF::HashSet;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_HASH_SET_H_
