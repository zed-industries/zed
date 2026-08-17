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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_HASH_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_HASH_MAP_H_

#include <initializer_list>
#include <iterator>

#include "base/compiler_specific.h"
#include "base/numerics/safe_conversions.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partition_allocator.h"
#include "third_party/blink/renderer/platform/wtf/atomic_operations.h"
#include "third_party/blink/renderer/platform/wtf/construct_traits.h"
#include "third_party/blink/renderer/platform/wtf/hash_table.h"
#include "third_party/blink/renderer/platform/wtf/key_value_pair.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {

template <typename KeyTraits, typename MappedTraits>
struct HashMapValueTraits;

template <typename Value, typename Traits, typename Allocator>
class HashCountedSet;

struct KeyValuePairExtractor {
  STATIC_ONLY(KeyValuePairExtractor);
  template <typename T>
  static const typename T::KeyType& ExtractKey(const T& p) {
    return p.key;
  }
  template <typename T>
  static typename T::KeyType& ExtractKey(T& p) {
    return p.key;
  }
  // Assumes out points to a buffer of size at least sizeof(T::KeyType).
  template <typename T>
  static void ExtractKeyToMemory(const T& p, void* out) {
    AtomicReadMemcpy<sizeof(typename T::KeyType), alignof(typename T::KeyType)>(
        out, &p.key);
  }
  template <typename T>
  static void ClearValue(T& p) {
    using ValueType = typename T::ValueType;
    if (IsTraceable<ValueType>::value) {
      AtomicMemzero<sizeof(ValueType), alignof(ValueType)>(&p.value);
    } else {
      UNSAFE_TODO(memset(static_cast<void*>(&p.value), 0, sizeof(p.value)));
    }
  }
};

// Note: empty or deleted key values are not allowed, using them may lead to
// undefined behavior. For pointer keys this means that null pointers are not
// allowed; for integer keys 0 or -1 can't be used as a key. You can change
// the restriction with a custom key hash traits. See hash_traits.h for how to
// define hash traits.
// Commonly used key types define their key hash traits separately from the
// class itself, so e.g if you want a `WTF::HashMap<WTF::String, ...>` you must
// include `string_hash.h`.
template <typename KeyArg,
          typename MappedArg,
          typename KeyTraitsArg = HashTraits<KeyArg>,
          typename MappedTraitsArg = HashTraits<MappedArg>,
          typename Allocator = PartitionAllocator>
class HashMap {
  USE_ALLOCATOR(HashMap, Allocator);
  template <typename T, typename U, typename V>
  friend class HashCountedSet;

 private:
  typedef KeyTraitsArg KeyTraits;
  typedef MappedTraitsArg MappedTraits;
  typedef HashMapValueTraits<KeyTraits, MappedTraits> ValueTraits;

 public:
  typedef typename KeyTraits::TraitType KeyType;
  typedef const typename KeyTraits::PeekInType& KeyPeekInType;
  typedef typename MappedTraits::TraitType MappedType;
  typedef typename ValueTraits::TraitType ValueType;
  using value_type = ValueType;

 private:
  typedef typename MappedTraits::PeekOutType MappedPeekType;

  typedef HashTable<KeyType,
                    ValueType,
                    KeyValuePairExtractor,
                    ValueTraits,
                    KeyTraits,
                    Allocator>
      HashTableType;

  class HashMapKeysProxy;
  class HashMapValuesProxy;

 public:
  HashMap() = default;

#if DUMP_HASHTABLE_STATS_PER_TABLE
  void DumpStats() { impl_.DumpStats(); }
#endif
  HashMap(const HashMap&) = default;
  HashMap& operator=(const HashMap&) = default;
  HashMap(HashMap&&) = default;
  HashMap& operator=(HashMap&&) = default;

  // For example, HashMap<int, int>({{1, 11}, {2, 22}, {3, 33}}) will give you
  // a HashMap containing a mapping {1 -> 11, 2 -> 22, 3 -> 33}.
  HashMap(std::initializer_list<ValueType> elements);
  HashMap& operator=(std::initializer_list<ValueType> elements);

  // Useful for constructing from, for example, STL and base maps.
  template <typename It>
    requires(std::forward_iterator<It>)
  HashMap(It begin, It end);

  typedef HashTableIteratorAdapter<HashTableType, ValueType> iterator;
  typedef HashTableConstIteratorAdapter<HashTableType, ValueType>
      const_iterator;
  typedef typename HashTableType::AddResult AddResult;

  void swap(HashMap& ref) { impl_.swap(ref.impl_); }

  wtf_size_t size() const;
  wtf_size_t Capacity() const;
  void ReserveCapacityForSize(unsigned size) {
    impl_.ReserveCapacityForSize(size);
  }

  bool empty() const;

  // iterators iterate over pairs of keys and values
  iterator begin();
  iterator end();
  const_iterator begin() const;
  const_iterator end() const;

  HashMapKeysProxy& Keys() { return static_cast<HashMapKeysProxy&>(*this); }
  const HashMapKeysProxy& Keys() const {
    return static_cast<const HashMapKeysProxy&>(*this);
  }

  HashMapValuesProxy& Values() {
    return static_cast<HashMapValuesProxy&>(*this);
  }
  const HashMapValuesProxy& Values() const {
    return static_cast<const HashMapValuesProxy&>(*this);
  }

  iterator find(KeyPeekInType);
  const_iterator find(KeyPeekInType) const;
  bool Contains(KeyPeekInType) const;
  // Returns a reference to the mapped value. Crashes if no mapped value exists.
  MappedPeekType at(KeyPeekInType) const;

  // Replaces value but not key if key is already present. Return value is a
  // pair of the iterator to the key location, and a boolean that's true if a
  // new value was actually added.
  template <typename IncomingKeyType, typename IncomingMappedType>
  AddResult Set(IncomingKeyType&&, IncomingMappedType&&);

  // Does nothing if key is already present. Return value is a pair of the
  // iterator to the key location, and a boolean that's true if a new value
  // was actually added.
  template <typename IncomingKeyType, typename IncomingMappedType>
  AddResult insert(IncomingKeyType&&, IncomingMappedType&&);

  // NOTE: You cannot continue using an iterator after erase()
  // (no modifications are allowed during iteration). Consider erase_if()
  // or RemoveAll().
  void erase(KeyPeekInType);
  void erase(iterator);

  // Erases all elements for which pred(element) returns true.
  //
  // The predicate should have a signature compatible with:
  //   bool pred(const WTF::KeyValuePair<KeyType, MappedType>&);
  template <typename Pred>
  void erase_if(Pred pred);

  void clear();
  template <typename Collection>
  void RemoveAll(const Collection& to_be_removed) {
    WTF::RemoveAll(*this, to_be_removed);
  }

  MappedType Take(KeyPeekInType);  // efficient combination of get with remove

  // An alternate version of find() that finds the object by hashing and
  // comparing with some other type, to avoid the cost of type conversion.
  // HashTranslator must have the following function members:
  //   static unsigned GetHash(const T&);
  //   static bool Equal(const ValueType&, const T&);
  template <typename HashTranslator, typename T>
  iterator Find(const T&);
  template <typename HashTranslator, typename T>
  const_iterator Find(const T&) const;
  template <typename HashTranslator, typename T>
  bool Contains(const T&) const;

  template <typename IncomingKeyType>
  static bool IsValidKey(const IncomingKeyType&);

  void Trace(auto visitor) const
    requires Allocator::kIsGarbageCollected
  {
    impl_.Trace(visitor);
  }

 protected:
  ValueType** GetBufferSlot() { return impl_.GetBufferSlot(); }

 private:
  template <typename IncomingKeyType, typename IncomingMappedType>
  AddResult InlineAdd(IncomingKeyType&&, IncomingMappedType&&);

  HashTableType impl_;

  struct TypeConstraints {
    constexpr TypeConstraints() {
      static_assert(!IsStackAllocatedTypeV<KeyArg>);
      static_assert(!IsStackAllocatedTypeV<MappedArg>);
      static_assert(Allocator::kIsGarbageCollected ||
                        !IsPointerToGarbageCollectedType<KeyArg>,
                    "Cannot put raw pointers to garbage-collected classes into "
                    "an off-heap HashMap.  Use HeapHashMap<> instead.");
      static_assert(Allocator::kIsGarbageCollected ||
                        !IsPointerToGarbageCollectedType<MappedArg>,
                    "Cannot put raw pointers to garbage-collected classes into "
                    "an off-heap HashMap.  Use HeapHashMap<> instead.");
    }
  };
  NO_UNIQUE_ADDRESS TypeConstraints type_constraints_;
};

template <typename KeyArg,
          typename MappedArg,
          typename KeyTraitsArg,
          typename MappedTraitsArg,
          typename Allocator>
class HashMap<KeyArg, MappedArg, KeyTraitsArg, MappedTraitsArg, Allocator>::
    HashMapKeysProxy : private HashMap<KeyArg,
                                       MappedArg,
                                       KeyTraitsArg,
                                       MappedTraitsArg,
                                       Allocator> {
  DISALLOW_NEW();

 public:
  using HashMapType =
      HashMap<KeyArg, MappedArg, KeyTraitsArg, MappedTraitsArg, Allocator>;
  using iterator = HashMapType::iterator::KeysIterator;
  using const_iterator = HashMapType::const_iterator::KeysIterator;
  using value_type = HashMapType::KeyType;

  iterator begin() { return HashMapType::begin().Keys(); }
  iterator end() { return HashMapType::end().Keys(); }

  const_iterator begin() const { return HashMapType::begin().Keys(); }
  const_iterator end() const { return HashMapType::end().Keys(); }

  wtf_size_t size() const { return HashMapType::size(); }

 private:
  friend class HashMap;

  HashMapKeysProxy() = delete;
  HashMapKeysProxy(const HashMapKeysProxy&) = delete;
  HashMapKeysProxy& operator=(const HashMapKeysProxy&) = delete;
  ~HashMapKeysProxy() = delete;
};

template <typename KeyArg,
          typename MappedArg,
          typename KeyTraitsArg,
          typename MappedTraitsArg,
          typename Allocator>
class HashMap<KeyArg, MappedArg, KeyTraitsArg, MappedTraitsArg, Allocator>::
    HashMapValuesProxy : private HashMap<KeyArg,
                                         MappedArg,
                                         KeyTraitsArg,
                                         MappedTraitsArg,
                                         Allocator> {
  DISALLOW_NEW();

 public:
  using HashMapType =
      HashMap<KeyArg, MappedArg, KeyTraitsArg, MappedTraitsArg, Allocator>;
  using iterator = HashMapType::iterator::ValuesIterator;
  using const_iterator = HashMapType::const_iterator::ValuesIterator;
  using value_type = HashMapType::MappedType;

  iterator begin() { return HashMapType::begin().Values(); }
  iterator end() { return HashMapType::end().Values(); }

  const_iterator begin() const { return HashMapType::begin().Values(); }
  const_iterator end() const { return HashMapType::end().Values(); }

  wtf_size_t size() const { return HashMapType::size(); }

 private:
  friend class HashMap;

  HashMapValuesProxy() = delete;
  HashMapValuesProxy(const HashMapValuesProxy&) = delete;
  HashMapValuesProxy& operator=(const HashMapValuesProxy&) = delete;
  ~HashMapValuesProxy() = delete;
};

template <typename KeyTraits, typename ValueTraits>
struct HashMapValueTraits : KeyValuePairHashTraits<KeyTraits, ValueTraits> {
  using P = typename KeyValuePairHashTraits<KeyTraits, ValueTraits>::TraitType;
  static bool IsEmptyValue(const P& value) {
    return IsHashTraitsEmptyValue<KeyTraits>(value.key);
  }
  // HashTable should never use the following functions/flags of this traits
  // type. They make sense in the KeyTraits only.
  static bool Equal(const P&, const P&) = delete;
  static void ConstructDeletedValue(P&) = delete;
  static bool IsDeletedValue(const P&) = delete;

 private:
  static const bool kSafeToCompareToEmptyOrDeleted;
};

template <typename KeyTraits, typename ValueTraits>
struct HashMapTranslator {
  STATIC_ONLY(HashMapTranslator);
  template <typename T>
  static unsigned GetHash(const T& key) {
    return KeyTraits::GetHash(key);
  }
  template <typename T, typename U>
  static bool Equal(const T& a, const U& b) {
    return KeyTraits::Equal(a, b);
  }
  template <typename T, typename U, typename V>
  static void Store(T& location, U&& key, V&& mapped) {
    location.key = std::forward<U>(key);
    location.value = std::forward<V>(mapped);
  }
};

template <typename KeyArg,
          typename MappedArg,
          typename KeyTraitsArg,
          typename MappedTraitsArg,
          typename Allocator>
HashMap<KeyArg, MappedArg, KeyTraitsArg, MappedTraitsArg, Allocator>::HashMap(
    std::initializer_list<ValueType> elements) {
  if (elements.size()) {
    impl_.ReserveCapacityForSize(
        base::checked_cast<wtf_size_t>(elements.size()));
  }
  for (const ValueType& element : elements)
    insert(element.key, element.value);
}

template <typename T, typename U, typename V, typename W, typename X>
auto HashMap<T, U, V, W, X>::operator=(
    std::initializer_list<ValueType> elements) -> HashMap& {
  *this = HashMap(std::move(elements));
  return *this;
}

template <typename KeyArg,
          typename MappedArg,
          typename KeyTraitsArg,
          typename MappedTraitsArg,
          typename Allocator>
template <typename It>
  requires(std::forward_iterator<It>)
HashMap<KeyArg, MappedArg, KeyTraitsArg, MappedTraitsArg, Allocator>::HashMap(
    It begin,
    It end) {
  if constexpr (std::random_access_iterator<It>) {
    ReserveCapacityForSize(base::checked_cast<wtf_size_t>(end - begin));
  }
  for (; begin != end; ++begin) {
    insert(begin->first, begin->second);
  }
}

template <typename T, typename U, typename V, typename W, typename X>
inline wtf_size_t HashMap<T, U, V, W, X>::size() const {
  return impl_.size();
}

template <typename T, typename U, typename V, typename W, typename X>
inline wtf_size_t HashMap<T, U, V, W, X>::Capacity() const {
  return impl_.Capacity();
}

template <typename T, typename U, typename V, typename W, typename X>
inline bool HashMap<T, U, V, W, X>::empty() const {
  return impl_.empty();
}

template <typename T, typename U, typename V, typename W, typename X>
inline typename HashMap<T, U, V, W, X>::iterator
HashMap<T, U, V, W, X>::begin() {
  return impl_.begin();
}

template <typename T, typename U, typename V, typename W, typename X>
inline typename HashMap<T, U, V, W, X>::iterator HashMap<T, U, V, W, X>::end() {
  return impl_.end();
}

template <typename T, typename U, typename V, typename W, typename X>
inline typename HashMap<T, U, V, W, X>::const_iterator
HashMap<T, U, V, W, X>::begin() const {
  return impl_.begin();
}

template <typename T, typename U, typename V, typename W, typename X>
inline typename HashMap<T, U, V, W, X>::const_iterator
HashMap<T, U, V, W, X>::end() const {
  return impl_.end();
}

template <typename T, typename U, typename V, typename W, typename X>
inline typename HashMap<T, U, V, W, X>::iterator HashMap<T, U, V, W, X>::find(
    KeyPeekInType key) {
  return impl_.find(key);
}

template <typename T, typename U, typename V, typename W, typename X>
inline typename HashMap<T, U, V, W, X>::const_iterator
HashMap<T, U, V, W, X>::find(KeyPeekInType key) const {
  return impl_.find(key);
}

template <typename T, typename U, typename V, typename W, typename X>
inline bool HashMap<T, U, V, W, X>::Contains(KeyPeekInType key) const {
  return impl_.Contains(key);
}

template <typename T, typename U, typename V, typename W, typename X>
template <typename HashTranslator, typename TYPE>
inline typename HashMap<T, U, V, W, X>::iterator HashMap<T, U, V, W, X>::Find(
    const TYPE& value) {
  return impl_.template Find<HashTranslator>(value);
}

template <typename T, typename U, typename V, typename W, typename X>
template <typename HashTranslator, typename TYPE>
inline typename HashMap<T, U, V, W, X>::const_iterator
HashMap<T, U, V, W, X>::Find(const TYPE& value) const {
  return impl_.template Find<HashTranslator>(value);
}

template <typename T, typename U, typename V, typename W, typename X>
template <typename HashTranslator, typename TYPE>
inline bool HashMap<T, U, V, W, X>::Contains(const TYPE& value) const {
  return impl_.template Contains<HashTranslator>(value);
}

template <typename T, typename U, typename V, typename W, typename X>
template <typename IncomingKeyType, typename IncomingMappedType>
typename HashMap<T, U, V, W, X>::AddResult HashMap<T, U, V, W, X>::InlineAdd(
    IncomingKeyType&& key,
    IncomingMappedType&& mapped) {
  return impl_.template insert<HashMapTranslator<KeyTraits, ValueTraits>>(
      std::forward<IncomingKeyType>(key),
      std::forward<IncomingMappedType>(mapped));
}

template <typename T, typename U, typename V, typename W, typename X>
template <typename IncomingKeyType, typename IncomingMappedType>
typename HashMap<T, U, V, W, X>::AddResult HashMap<T, U, V, W, X>::Set(
    IncomingKeyType&& key,
    IncomingMappedType&& mapped) {
  AddResult result = InlineAdd(std::forward<IncomingKeyType>(key),
                               std::forward<IncomingMappedType>(mapped));
  if (!result.is_new_entry) {
    // The InlineAdd call above found an existing hash table entry; we need
    // to set the mapped value.
    //
    // It's safe to call std::forward again, because |mapped| isn't moved if
    // there's an existing entry.
    result.stored_value->value = std::forward<IncomingMappedType>(mapped);
  }
  return result;
}

template <typename T, typename U, typename V, typename W, typename X>
template <typename IncomingKeyType, typename IncomingMappedType>
typename HashMap<T, U, V, W, X>::AddResult HashMap<T, U, V, W, X>::insert(
    IncomingKeyType&& key,
    IncomingMappedType&& mapped) {
  return InlineAdd(std::forward<IncomingKeyType>(key),
                   std::forward<IncomingMappedType>(mapped));
}

template <typename T, typename U, typename V, typename W, typename X>
typename HashMap<T, U, V, W, X>::MappedPeekType HashMap<T, U, V, W, X>::at(
    KeyPeekInType key) const {
  const ValueType* entry = impl_.Lookup(key);
  CHECK(entry) << "HashMap::at found no value for the given key. See "
                  "https://crbug.com/1058527.";
  return MappedTraits::Peek(entry->value);
}

template <typename T, typename U, typename V, typename W, typename X>
inline void HashMap<T, U, V, W, X>::erase(iterator it) {
  impl_.erase(it.impl_);
}

template <typename T, typename U, typename V, typename W, typename X>
inline void HashMap<T, U, V, W, X>::erase(KeyPeekInType key) {
  erase(find(key));
}

template <typename T, typename U, typename V, typename W, typename X>
template <typename Pred>
inline void HashMap<T, U, V, W, X>::erase_if(Pred pred) {
  impl_.erase_if(std::forward<Pred>(pred));
}

template <typename T, typename U, typename V, typename W, typename X>
inline void HashMap<T, U, V, W, X>::clear() {
  impl_.clear();
}

template <typename T, typename U, typename V, typename W, typename X>
auto HashMap<T, U, V, W, X>::Take(KeyPeekInType key) -> MappedType {
  iterator it = find(key);
  if (it == end())
    return MappedTraits::EmptyValue();
  MappedType result = std::move(it->value);
  erase(it);
  return result;
}

template <typename T, typename U, typename V, typename W, typename X>
template <typename IncomingKeyType>
inline bool HashMap<T, U, V, W, X>::IsValidKey(const IncomingKeyType& key) {
  return !IsHashTraitsEmptyOrDeletedValue<KeyTraits>(key);
}

template <typename T, typename U, typename V, typename W, typename X>
bool operator==(const HashMap<T, U, V, W, X>& a,
                const HashMap<T, U, V, W, X>& b) {
  if (a.size() != b.size())
    return false;

  typedef typename HashMap<T, U, V, W, X>::const_iterator const_iterator;

  const_iterator a_end = a.end();
  const_iterator b_end = b.end();
  for (const_iterator it = a.begin(); it != a_end; ++it) {
    const_iterator b_pos = b.find(it->key);
    if (b_pos == b_end || it->value != b_pos->value)
      return false;
  }

  return true;
}

template <typename T, typename U, typename V, typename W, typename X>
inline bool operator!=(const HashMap<T, U, V, W, X>& a,
                       const HashMap<T, U, V, W, X>& b) {
  return !(a == b);
}

}  // namespace WTF

using WTF::HashMap;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_HASH_MAP_H_
