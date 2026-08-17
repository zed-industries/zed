/*
 * Copyright (C) 2007 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_KEY_VALUE_PAIR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_KEY_VALUE_PAIR_H_

#include <utility>

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace WTF {

template <typename KeyTypeArg, typename ValueTypeArg>
struct KeyValuePair {
  using KeyType = KeyTypeArg;
  using ValueType = ValueTypeArg;

  template <typename IncomingKeyType, typename IncomingValueType>
  KeyValuePair(IncomingKeyType&& key, IncomingValueType&& value)
      : key(std::forward<IncomingKeyType>(key)),
        value(std::forward<IncomingValueType>(value)) {}

  template <typename OtherKeyType, typename OtherValueType>
  KeyValuePair(KeyValuePair<OtherKeyType, OtherValueType>&& other)
      : key(std::move(other.key)), value(std::move(other.value)) {}

  KeyTypeArg key;
  ValueTypeArg value;
};

template <typename K, typename V>
struct IsWeak<KeyValuePair<K, V>>
    : std::integral_constant<bool, IsWeak<K>::value || IsWeak<V>::value> {};

template <typename K, typename V>
struct IsTraceable<KeyValuePair<K, V>>
    : std::integral_constant<bool,
                             IsTraceable<K>::value || IsTraceable<V>::value> {};

template <typename KeyTraitsArg,
          typename ValueTraitsArg,
          typename P = KeyValuePair<typename KeyTraitsArg::TraitType,
                                    typename ValueTraitsArg::TraitType>>
struct KeyValuePairHashTraits
    : TwoFieldsHashTraits<P, &P::key, &P::value, KeyTraitsArg, ValueTraitsArg> {
  using TraitType = P;
  using KeyTraits = KeyTraitsArg;
  using ValueTraits = ValueTraitsArg;

  // Even non-traceable keys need to have their trait set. This is because
  // non-traceable keys still need to be processed concurrently for checking
  // empty/deleted state.
  static constexpr bool kCanTraceConcurrently =
      KeyTraits::kCanTraceConcurrently &&
      (ValueTraits::kCanTraceConcurrently ||
       !IsTraceable<typename ValueTraits::TraitType>::value);
  static constexpr bool kSupportsCompaction =
      KeyTraits::kSupportsCompaction && ValueTraits::kSupportsCompaction;
};

template <typename Key, typename Value>
struct HashTraits<KeyValuePair<Key, Value>>
    : public KeyValuePairHashTraits<HashTraits<Key>, HashTraits<Value>> {};

namespace internal {

template <typename T, bool NeedsStackCheck = IsTraceable<T>::value>
class IteratorAdapterBase {};

template <typename T>
struct IteratorAdapterBase<T, true> {
  STACK_ALLOCATED();
};

}  // namespace internal

template <typename HashTableType,
          typename KeyType,
          typename MappedType,
          typename Enable = void>
struct HashTableConstKeysIterator;
template <typename HashTableType,
          typename KeyType,
          typename MappedType,
          typename Enable = void>
struct HashTableConstValuesIterator;
template <typename HashTableType,
          typename KeyType,
          typename MappedType,
          typename Enable = void>
struct HashTableKeysIterator;
template <typename HashTableType,
          typename KeyType,
          typename MappedType,
          typename Enable = void>
struct HashTableValuesIterator;

template <typename HashTableType, typename KeyType, typename MappedType>
struct HashTableConstIteratorAdapter<HashTableType,
                                     KeyValuePair<KeyType, MappedType>>
    : internal::IteratorAdapterBase<KeyValuePair<KeyType, MappedType>> {
  typedef KeyValuePair<KeyType, MappedType> ValueType;
  typedef HashTableConstKeysIterator<HashTableType, KeyType, MappedType>
      KeysIterator;
  typedef HashTableConstValuesIterator<HashTableType, KeyType, MappedType>
      ValuesIterator;

  using iterator_category = std::bidirectional_iterator_tag;
  using value_type = ValueType;
  using difference_type = ptrdiff_t;
  using pointer = const ValueType*;
  using reference = const ValueType&;

  HashTableConstIteratorAdapter() = default;
  HashTableConstIteratorAdapter(
      const typename HashTableType::const_iterator& impl)
      : impl_(impl) {}

  const ValueType* Get() const { return (const ValueType*)impl_.Get(); }
  const ValueType& operator*() const { return *Get(); }
  const ValueType* operator->() const { return Get(); }

  HashTableConstIteratorAdapter& operator++() {
    ++impl_;
    return *this;
  }
  HashTableConstIteratorAdapter operator++(int) {
    HashTableConstIteratorAdapter copy(*this);
    ++*this;
    return copy;
  }

  HashTableConstIteratorAdapter& operator--() {
    --impl_;
    return *this;
  }
  HashTableConstIteratorAdapter operator--(int) {
    HashTableConstIteratorAdapter copy(*this);
    --*this;
    return copy;
  }

  KeysIterator Keys() { return KeysIterator(*this); }
  ValuesIterator Values() { return ValuesIterator(*this); }

  typename HashTableType::const_iterator impl_;
};

template <typename HashTableType, typename KeyType, typename MappedType>
struct HashTableIteratorAdapter<HashTableType,
                                KeyValuePair<KeyType, MappedType>>
    : internal::IteratorAdapterBase<KeyValuePair<KeyType, MappedType>> {
  typedef KeyValuePair<KeyType, MappedType> ValueType;
  typedef HashTableKeysIterator<HashTableType, KeyType, MappedType>
      KeysIterator;
  typedef HashTableValuesIterator<HashTableType, KeyType, MappedType>
      ValuesIterator;

  using iterator_category = std::bidirectional_iterator_tag;
  using value_type = ValueType;
  using difference_type = ptrdiff_t;
  using pointer = ValueType*;
  using reference = ValueType&;

  HashTableIteratorAdapter() = default;
  HashTableIteratorAdapter(const typename HashTableType::iterator& impl)
      : impl_(impl) {}

  ValueType* Get() const { return (ValueType*)impl_.Get(); }
  ValueType& operator*() const { return *Get(); }
  ValueType* operator->() const { return Get(); }

  HashTableIteratorAdapter& operator++() {
    ++impl_;
    return *this;
  }
  HashTableIteratorAdapter operator++(int) {
    HashTableIteratorAdapter copy(*this);
    ++*this;
    return copy;
  }

  HashTableIteratorAdapter& operator--() {
    --impl_;
    return *this;
  }
  HashTableIteratorAdapter operator--(int) {
    HashTableIteratorAdapter copy(*this);
    --*this;
    return copy;
  }

  operator HashTableConstIteratorAdapter<HashTableType, ValueType>() {
    typename HashTableType::const_iterator i = impl_;
    return i;
  }

  KeysIterator Keys() { return KeysIterator(*this); }
  ValuesIterator Values() { return ValuesIterator(*this); }

  typename HashTableType::iterator impl_;
};

template <typename HashTableType, typename KeyType, typename MappedType>
struct HashTableConstKeysIterator<HashTableType, KeyType, MappedType>
    : internal::IteratorAdapterBase<KeyValuePair<KeyType, MappedType>> {
 private:
  typedef HashTableConstIteratorAdapter<HashTableType,
                                        KeyValuePair<KeyType, MappedType>>
      ConstIterator;

 public:
  using iterator_category = typename ConstIterator::iterator_category;
  using value_type = KeyType;
  using difference_type = typename ConstIterator::difference_type;
  using pointer = const KeyType*;
  using reference = const KeyType&;

  constexpr HashTableConstKeysIterator() = default;

  HashTableConstKeysIterator(const ConstIterator& impl) : impl_(impl) {}

  const KeyType* Get() const { return &(impl_.Get()->key); }
  const KeyType& operator*() const { return *Get(); }
  const KeyType* operator->() const { return Get(); }

  HashTableConstKeysIterator& operator++() {
    ++impl_;
    return *this;
  }
  HashTableConstKeysIterator operator++(int) {
    HashTableConstKeysIterator copy(*this);
    ++*this;
    return copy;
  }

  HashTableConstKeysIterator& operator--() {
    --impl_;
    return *this;
  }
  HashTableConstKeysIterator operator--(int) {
    HashTableConstKeysIterator copy(*this);
    --*this;
    return copy;
  }

  ConstIterator impl_;
};

template <typename HashTableType, typename KeyType, typename MappedType>
struct HashTableConstValuesIterator<HashTableType, KeyType, MappedType>
    : internal::IteratorAdapterBase<KeyValuePair<KeyType, MappedType>> {
 private:
  typedef HashTableConstIteratorAdapter<HashTableType,
                                        KeyValuePair<KeyType, MappedType>>
      ConstIterator;

 public:
  using iterator_category = typename ConstIterator::iterator_category;
  using value_type = MappedType;
  using difference_type = typename ConstIterator::difference_type;
  using pointer = const MappedType*;
  using reference = const MappedType&;

  constexpr HashTableConstValuesIterator() = default;

  HashTableConstValuesIterator(const ConstIterator& impl) : impl_(impl) {}

  const MappedType* Get() const { return &(impl_.Get()->value); }
  const MappedType& operator*() const { return *Get(); }
  const MappedType* operator->() const { return Get(); }

  HashTableConstValuesIterator& operator++() {
    ++impl_;
    return *this;
  }
  HashTableConstValuesIterator operator++(int) {
    HashTableConstValuesIterator copy(*this);
    ++*this;
    return copy;
  }

  HashTableConstValuesIterator& operator--() {
    --impl_;
    return *this;
  }
  HashTableConstValuesIterator operator--(int) {
    HashTableConstValuesIterator copy(*this);
    --*this;
    return copy;
  }

  ConstIterator impl_;
};

template <typename HashTableType, typename KeyType, typename MappedType>
struct HashTableKeysIterator<HashTableType, KeyType, MappedType>
    : internal::IteratorAdapterBase<KeyValuePair<KeyType, MappedType>> {
 private:
  typedef HashTableIteratorAdapter<HashTableType,
                                   KeyValuePair<KeyType, MappedType>>
      Iterator;
  typedef HashTableConstIteratorAdapter<HashTableType,
                                        KeyValuePair<KeyType, MappedType>>
      ConstIterator;

 public:
  using iterator_category = typename Iterator::iterator_category;
  using value_type = KeyType;
  using difference_type = typename Iterator::difference_type;
  using pointer = KeyType*;
  using reference = KeyType&;

  constexpr HashTableKeysIterator() = default;

  HashTableKeysIterator(const Iterator& impl) : impl_(impl) {}

  KeyType* Get() const { return &(impl_.Get()->key); }
  KeyType& operator*() const { return *Get(); }
  KeyType* operator->() const { return Get(); }

  HashTableKeysIterator& operator++() {
    ++impl_;
    return *this;
  }
  HashTableKeysIterator operator++(int) {
    HashTableKeysIterator copy(*this);
    ++*this;
    return copy;
  }

  HashTableKeysIterator& operator--() {
    --impl_;
    return *this;
  }
  HashTableKeysIterator operator--(int) {
    HashTableKeysIterator copy(*this);
    --*this;
    return copy;
  }

  operator HashTableConstKeysIterator<HashTableType, KeyType, MappedType>() {
    ConstIterator i = impl_;
    return i;
  }

  Iterator impl_;
};

template <typename HashTableType, typename KeyType, typename MappedType>
struct HashTableValuesIterator<HashTableType, KeyType, MappedType>
    : internal::IteratorAdapterBase<KeyValuePair<KeyType, MappedType>> {
 private:
  typedef HashTableIteratorAdapter<HashTableType,
                                   KeyValuePair<KeyType, MappedType>>
      Iterator;
  typedef HashTableConstIteratorAdapter<HashTableType,
                                        KeyValuePair<KeyType, MappedType>>
      ConstIterator;

 public:
  using iterator_category = typename Iterator::iterator_category;
  using value_type = MappedType;
  using difference_type = typename Iterator::difference_type;
  using pointer = MappedType*;
  using reference = MappedType&;

  constexpr HashTableValuesIterator() = default;

  HashTableValuesIterator(const Iterator& impl) : impl_(impl) {}

  MappedType* Get() const { return &(impl_.Get()->value); }
  MappedType& operator*() const { return *Get(); }
  MappedType* operator->() const { return Get(); }

  HashTableValuesIterator& operator++() {
    ++impl_;
    return *this;
  }
  HashTableValuesIterator operator++(int) {
    HashTableValuesIterator copy(*this);
    ++*this;
    return copy;
  }

  HashTableValuesIterator& operator--() {
    --impl_;
    return *this;
  }
  HashTableValuesIterator operator--(int) {
    HashTableValuesIterator copy(*this);
    --*this;
    return copy;
  }

  operator HashTableConstValuesIterator<HashTableType, KeyType, MappedType>() {
    ConstIterator i = impl_;
    return i;
  }

  Iterator impl_;
};

template <typename T, typename U, typename V>
inline bool operator==(const HashTableConstKeysIterator<T, U, V>& a,
                       const HashTableConstKeysIterator<T, U, V>& b) {
  return a.impl_ == b.impl_;
}

template <typename T, typename U, typename V>
inline bool operator!=(const HashTableConstKeysIterator<T, U, V>& a,
                       const HashTableConstKeysIterator<T, U, V>& b) {
  return a.impl_ != b.impl_;
}

template <typename T, typename U, typename V>
inline bool operator==(const HashTableConstValuesIterator<T, U, V>& a,
                       const HashTableConstValuesIterator<T, U, V>& b) {
  return a.impl_ == b.impl_;
}

template <typename T, typename U, typename V>
inline bool operator!=(const HashTableConstValuesIterator<T, U, V>& a,
                       const HashTableConstValuesIterator<T, U, V>& b) {
  return a.impl_ != b.impl_;
}

template <typename T, typename U, typename V>
inline bool operator==(const HashTableKeysIterator<T, U, V>& a,
                       const HashTableKeysIterator<T, U, V>& b) {
  return a.impl_ == b.impl_;
}

template <typename T, typename U, typename V>
inline bool operator!=(const HashTableKeysIterator<T, U, V>& a,
                       const HashTableKeysIterator<T, U, V>& b) {
  return a.impl_ != b.impl_;
}

template <typename T, typename U, typename V>
inline bool operator==(const HashTableValuesIterator<T, U, V>& a,
                       const HashTableValuesIterator<T, U, V>& b) {
  return a.impl_ == b.impl_;
}

template <typename T, typename U, typename V>
inline bool operator!=(const HashTableValuesIterator<T, U, V>& a,
                       const HashTableValuesIterator<T, U, V>& b) {
  return a.impl_ != b.impl_;
}

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_KEY_VALUE_PAIR_H_
