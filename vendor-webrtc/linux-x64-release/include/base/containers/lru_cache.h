// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains a template for a Least Recently Used cache that allows
// constant-time access to items, but easy identification of the
// least-recently-used items for removal. Variations exist to support use as a
// Map (`base::LRUCache`), HashMap (`base::HashingLRUCache`), Set
// (`base::LRUCacheSet`), or HashSet (`base::HashingLRUCacheSet`). These are
// implemented as aliases of `base::internal::LRUCacheBase`, defined at the
// bottom of this file.
//
// The key object (which is identical to the value, in the Set variations) will
// be stored twice, so it should support efficient copying.

#ifndef BASE_CONTAINERS_LRU_CACHE_H_
#define BASE_CONTAINERS_LRU_CACHE_H_

#include <stddef.h>

#include <algorithm>
#include <concepts>
#include <functional>
#include <list>
#include <map>
#include <type_traits>
#include <unordered_map>
#include <utility>

#include "base/check.h"

namespace base {
namespace trace_event::internal {

template <class LruCacheType>
size_t DoEstimateMemoryUsageForLruCache(const LruCacheType&);

}  // namespace trace_event::internal

namespace internal {

struct GetKeyFromKVPair {
  template <typename T1, typename T2>
  constexpr const T1& operator()(const std::pair<T1, T2>& pair) {
    return pair.first;
  }
};

// Base class for the LRU cache specializations defined below.
template <class ValueType, class GetKeyFromValue, class KeyIndexTemplate>
class LRUCacheBase {
 public:
  // The contents of the list. This must contain a copy of the key (that may be
  // extracted via `GetKeyFromValue()(value)` so we can efficiently delete
  // things given an element of the list.
  using value_type = ValueType;

 private:
  using ValueList = std::list<value_type>;
  using KeyIndex =
      typename KeyIndexTemplate::template Type<typename ValueList::iterator>;

 public:
  using size_type = typename ValueList::size_type;
  using key_type = typename KeyIndex::key_type;

  using iterator = typename ValueList::iterator;
  using const_iterator = typename ValueList::const_iterator;
  using reverse_iterator = typename ValueList::reverse_iterator;
  using const_reverse_iterator = typename ValueList::const_reverse_iterator;

  enum { NO_AUTO_EVICT = 0 };

  // The max_size is the size at which the cache will prune its members to when
  // a new item is inserted. If the caller wants to manage this itself (for
  // example, maybe it has special work to do when something is evicted), it
  // can pass NO_AUTO_EVICT to not restrict the cache size.
  explicit LRUCacheBase(size_type max_size) : max_size_(max_size) {}

  // In theory, LRUCacheBase could be copyable, but since copying `ValueList`
  // might be costly, it's currently move-only to ensure users don't
  // accidentally incur performance penalties. If you need this to become
  // copyable, talk to base/ OWNERS.
  LRUCacheBase(LRUCacheBase&&) noexcept = default;
  LRUCacheBase& operator=(LRUCacheBase&&) noexcept = default;

  ~LRUCacheBase() = default;

  size_type max_size() const { return max_size_; }

  // Inserts an item into the list. If an existing item has the same key, it is
  // removed prior to insertion. An iterator indicating the inserted item will
  // be returned (this will always be the front of the list).
  // In the map variations of this container, `value_type` is a `std::pair` and
  // it's preferred to use the `Put(k, v)` overload of this method.
  iterator Put(value_type&& value) {
    // Remove any existing item with that key.
    key_type key = GetKeyFromValue{}(value);
    typename KeyIndex::iterator index_iter = index_.find(key);
    if (index_iter != index_.end()) {
      // Erase the reference to it. The index reference will be replaced in the
      // code below.
      Erase(index_iter->second);
    } else if (max_size_ != NO_AUTO_EVICT) {
      // New item is being inserted which might make it larger than the maximum
      // size: kick the oldest thing out if necessary.
      ShrinkToSize(max_size_ - 1);
    }

    ordering_.push_front(std::move(value));
    index_.emplace(std::move(key), ordering_.begin());
    return ordering_.begin();
  }

  // Inserts an item into the list. If an existing item has the same key, it is
  // removed prior to insertion. An iterator indicating the inserted item will
  // be returned (this will always be the front of the list).
  template <class K, class V>
    requires(std::same_as<GetKeyFromValue, GetKeyFromKVPair>)
  iterator Put(K&& key, V&& value) {
    return Put(value_type{std::forward<K>(key), std::forward<V>(value)});
  }

  // Retrieves the contents of the given key, or end() if not found. This method
  // has the side effect of moving the requested item to the front of the
  // recency list.
  iterator Get(const key_type& key) {
    typename KeyIndex::iterator index_iter = index_.find(key);
    if (index_iter == index_.end()) {
      return end();
    }
    typename ValueList::iterator iter = index_iter->second;

    // Move the touched item to the front of the recency ordering.
    ordering_.splice(ordering_.begin(), ordering_, iter);
    return ordering_.begin();
  }

  // Retrieves the item associated with a given key and returns it via
  // result without affecting the ordering (unlike Get()).
  iterator Peek(const key_type& key) {
    typename KeyIndex::const_iterator index_iter = index_.find(key);
    if (index_iter == index_.end()) {
      return end();
    }
    return index_iter->second;
  }

  const_iterator Peek(const key_type& key) const {
    typename KeyIndex::const_iterator index_iter = index_.find(key);
    if (index_iter == index_.end()) {
      return end();
    }
    return index_iter->second;
  }

  // Exchanges the contents of |this| by the contents of the |other|.
  void Swap(LRUCacheBase& other) {
    ordering_.swap(other.ordering_);
    index_.swap(other.index_);
    std::swap(max_size_, other.max_size_);
  }

  // Erases the item referenced by the given iterator. An iterator to the item
  // following it will be returned. The iterator must be valid.
  // Note that caller should avoid using std::remove_if() with this container as
  // the iterator from begin()/end() is not designed to have the key modified,
  // see comment on begin().
  iterator Erase(iterator pos) {
    index_.erase(GetKeyFromValue()(*pos));
    return ordering_.erase(pos);
  }

  // LRUCache entries are often processed in reverse order, so we add this
  // convenience function (not typically defined by STL containers).
  reverse_iterator Erase(reverse_iterator pos) {
    // We have to actually give it the incremented iterator to delete, since
    // the forward iterator that base() returns is actually one past the item
    // being iterated over.
    return reverse_iterator(Erase((++pos).base()));
  }

  // Shrinks the cache so it only holds |new_size| items. If |new_size| is
  // bigger or equal to the current number of items, this will do nothing.
  void ShrinkToSize(size_type new_size) {
    for (size_type i = size(); i > new_size; i--) {
      Erase(rbegin());
    }
  }

  // Deletes everything from the cache.
  void Clear() {
    index_.clear();
    ordering_.clear();
  }

  // Returns the number of elements in the cache.
  size_type size() const {
    // We don't use ordering_.size() for the return value because
    // (as a linked list) it can be O(n).
    DCHECK(index_.size() == ordering_.size());
    return index_.size();
  }

  // Allows iteration over the list. Forward iteration starts with the most
  // recent item and works backwards.
  //
  // Note that since these iterators are actually iterators over a list, you
  // can keep them as you insert or delete things (as long as you don't delete
  // the one you are pointing to) and they will still be valid.
  // Also, caller should avoid moving the order of items around, or any
  // operation that modifies the key in the value with these iterators, such as
  // using std::remove_if(). This is because the key in index_ is not updated
  // and the container will be corrupted.
  iterator begin() { return ordering_.begin(); }
  const_iterator begin() const { return ordering_.begin(); }
  iterator end() { return ordering_.end(); }
  const_iterator end() const { return ordering_.end(); }

  reverse_iterator rbegin() { return ordering_.rbegin(); }
  const_reverse_iterator rbegin() const { return ordering_.rbegin(); }
  reverse_iterator rend() { return ordering_.rend(); }
  const_reverse_iterator rend() const { return ordering_.rend(); }

  struct IndexRange {
    using iterator = KeyIndex::const_iterator;

    IndexRange(const iterator& begin, const iterator& end)
        : begin_(begin), end_(end) {}

    iterator begin() const { return begin_; }
    iterator end() const { return end_; }

   private:
    iterator begin_;
    iterator end_;
  };
  // Allows iterating the index, which can be useful when the index is ordered.
  IndexRange index() const { return IndexRange(index_.begin(), index_.end()); }

  bool empty() const { return ordering_.empty(); }

 private:
  template <class LruCacheType>
  friend size_t trace_event::internal::DoEstimateMemoryUsageForLruCache(
      const LruCacheType&);

  ValueList ordering_;
  // TODO(crbug.com/40069408): Remove annotation once crbug.com/1472363 is
  // fixed.
  __attribute__((annotate("blink_gc_plugin_ignore"))) KeyIndex index_;

  size_type max_size_;
};

template <class KeyType, class KeyCompare>
struct LRUCacheKeyIndex {
  template <class ValueType>
  using Type = std::map<KeyType, ValueType, KeyCompare>;
};

template <class KeyType, class KeyHash, class KeyEqual>
struct HashingLRUCacheKeyIndex {
  template <class ValueType>
  using Type = std::unordered_map<KeyType, ValueType, KeyHash, KeyEqual>;
};

}  // namespace internal

// Implements an LRU cache of `ValueType`, where each value can be uniquely
// referenced by `KeyType`. Entries can be iterated in order of
// least-recently-used to most-recently-used by iterating from `rbegin()` to
// `rend()`, where a "use" is defined as a call to `Put(k, v)` or `Get(k)`.
template <class KeyType, class ValueType, class KeyCompare = std::less<KeyType>>
using LRUCache =
    internal::LRUCacheBase<std::pair<KeyType, ValueType>,
                           internal::GetKeyFromKVPair,
                           internal::LRUCacheKeyIndex<KeyType, KeyCompare>>;

// Implements an LRU cache of `ValueType`, where each value can be uniquely
// referenced by `KeyType`, and `KeyType` may be hashed for O(1) insertion,
// removal, and lookup. Entries can be iterated in order of least-recently-used
// to most-recently-used by iterating from `rbegin()` to `rend()`, where a "use"
// is defined as a call to `Put(k, v)` or `Get(k)`.
template <class KeyType,
          class ValueType,
          class KeyHash = std::hash<KeyType>,
          class KeyEqual = std::equal_to<KeyType>>
using HashingLRUCache = internal::LRUCacheBase<
    std::pair<KeyType, ValueType>,
    internal::GetKeyFromKVPair,
    internal::HashingLRUCacheKeyIndex<KeyType, KeyHash, KeyEqual>>;

// Implements an LRU cache of `ValueType`, where each value is unique. Entries
// can be iterated in order of least-recently-used to most-recently-used by
// iterating from `rbegin()` to `rend()`, where a "use" is defined as a call to
// `Put(v)` or `Get(v)`.
template <class ValueType, class Compare = std::less<ValueType>>
using LRUCacheSet =
    internal::LRUCacheBase<ValueType,
                           std::identity,
                           internal::LRUCacheKeyIndex<ValueType, Compare>>;

// Implements an LRU cache of `ValueType`, where is value is unique, and may be
// hashed for O(1) insertion, removal, and lookup. Entries can be iterated in
// order of least-recently-used to most-recently-used by iterating from
// `rbegin()` to `rend()`, where a "use" is defined as a call to `Put(v)` or
// `Get(v)`.
template <class ValueType,
          class Hash = std::hash<ValueType>,
          class Equal = std::equal_to<ValueType>>
using HashingLRUCacheSet = internal::LRUCacheBase<
    ValueType,
    std::identity,
    internal::HashingLRUCacheKeyIndex<ValueType, Hash, Equal>>;

}  // namespace base

#endif  // BASE_CONTAINERS_LRU_CACHE_H_
