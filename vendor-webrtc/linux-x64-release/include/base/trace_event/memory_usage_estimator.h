// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/40284755): Remove this and spanify to fix the errors.
#pragma allow_unsafe_buffers
#endif

#ifndef BASE_TRACE_EVENT_MEMORY_USAGE_ESTIMATOR_H_
#define BASE_TRACE_EVENT_MEMORY_USAGE_ESTIMATOR_H_

#include <stdint.h>

#include <array>
#include <concepts>
#include <deque>
#include <list>
#include <map>
#include <memory>
#include <queue>
#include <set>
#include <stack>
#include <string>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "base/base_export.h"
#include "base/containers/circular_deque.h"
#include "base/containers/flat_map.h"
#include "base/containers/flat_set.h"
#include "base/containers/heap_array.h"
#include "base/containers/linked_list.h"
#include "base/containers/lru_cache.h"
#include "base/containers/queue.h"
#include "base/containers/span.h"
#include "base/memory/raw_ptr.h"
#include "base/stl_util.h"
#include "base/types/always_false.h"

// Composable memory usage estimators.
//
// This file defines set of EstimateMemoryUsage(object) functions that return
// approximate dynamically allocated memory usage of their argument.
//
// The ultimate goal is to make memory usage estimation for a class simply a
// matter of aggregating EstimateMemoryUsage() results over all fields.
//
// That is achieved via composability: if EstimateMemoryUsage() is defined
// for T then EstimateMemoryUsage() is also defined for any combination of
// containers holding T (e.g. std::map<int, std::vector<T>>).
//
// There are two ways of defining EstimateMemoryUsage() for a type:
//
// 1. As a global function 'size_t EstimateMemoryUsage(T)' in
//    in base::trace_event namespace.
//
// 2. As 'size_t T::EstimateMemoryUsage() const' method. In this case
//    EstimateMemoryUsage(T) function in base::trace_event namespace is
//    provided automatically.
//
// Here is an example implementation:
//
// class MyClass {
//   ...
//   ...
//   size_t EstimateMemoryUsage() const {
//     return base::trace_event::EstimateMemoryUsage(set_) +
//            base::trace_event::EstimateMemoryUsage(name_) +
//            base::trace_event::EstimateMemoryUsage(foo_);
//   }
//   ...
//  private:
//   ...
//   std::set<int> set_;
//   std::string name_;
//   Foo foo_;
//   int id_;
//   bool success_;
// }
//
// The approach is simple: first call EstimateMemoryUsage() on all members,
// then recursively fix compilation errors that are caused by types not
// implementing EstimateMemoryUsage().
//
// Note that in the above example, the memory estimates for `id_` and `success_`
// are intentionally omitted. This is because these members do not allocate any
// _dynamic_ memory. If, for example, `MyClass` is declared as a heap-allocated
// `unique_ptr` member in some parent class, then `EstimateMemoryUsage` on the
// `unique_ptr` will automatically take into account `sizeof(MyClass)`.

namespace base {
namespace trace_event {

// Declarations

// If T declares 'EstimateMemoryUsage() const' member function, then
// global function EstimateMemoryUsage(T) is available, and just calls
// the member function.
template <class T>
auto EstimateMemoryUsage(const T& object)
    -> decltype(object.EstimateMemoryUsage());

// String

template <class C, class T, class A>
size_t EstimateMemoryUsage(const std::basic_string<C, T, A>& string);

// Arrays

template <class T, size_t N>
size_t EstimateMemoryUsage(const std::array<T, N>& array);

template <class T, size_t N>
size_t EstimateMemoryUsage(T (&array)[N]);

template <class T>
size_t EstimateMemoryUsage(const base::HeapArray<T>& array);

template <class T>
size_t EstimateMemoryUsage(base::span<T> array);

// std::unique_ptr

template <class T, class D>
size_t EstimateMemoryUsage(const std::unique_ptr<T, D>& ptr);

// std::shared_ptr

template <class T>
size_t EstimateMemoryUsage(const std::shared_ptr<T>& ptr);

// Containers

template <class F, class S>
size_t EstimateMemoryUsage(const std::pair<F, S>& pair);

template <class T, class A>
size_t EstimateMemoryUsage(const std::vector<T, A>& vector);

template <class T, class A>
size_t EstimateMemoryUsage(const std::list<T, A>& list);

template <class T>
size_t EstimateMemoryUsage(const base::LinkedList<T>& list);

template <class T, class C, class A>
size_t EstimateMemoryUsage(const std::set<T, C, A>& set);

template <class T, class C, class A>
size_t EstimateMemoryUsage(const std::multiset<T, C, A>& set);

template <class K, class V, class C, class A>
size_t EstimateMemoryUsage(const std::map<K, V, C, A>& map);

template <class K, class V, class C, class A>
size_t EstimateMemoryUsage(const std::multimap<K, V, C, A>& map);

template <class T, class H, class KE, class A>
size_t EstimateMemoryUsage(const std::unordered_set<T, H, KE, A>& set);

template <class T, class H, class KE, class A>
size_t EstimateMemoryUsage(const std::unordered_multiset<T, H, KE, A>& set);

template <class K, class V, class H, class KE, class A>
size_t EstimateMemoryUsage(const std::unordered_map<K, V, H, KE, A>& map);

template <class K, class V, class H, class KE, class A>
size_t EstimateMemoryUsage(const std::unordered_multimap<K, V, H, KE, A>& map);

template <class T, class A>
size_t EstimateMemoryUsage(const std::deque<T, A>& deque);

template <class T, class C>
size_t EstimateMemoryUsage(const std::queue<T, C>& queue);

template <class T, class C>
size_t EstimateMemoryUsage(const std::priority_queue<T, C>& queue);

template <class T, class C>
size_t EstimateMemoryUsage(const std::stack<T, C>& stack);

template <class T>
size_t EstimateMemoryUsage(const base::circular_deque<T>& deque);

template <class T, class C>
size_t EstimateMemoryUsage(const base::flat_set<T, C>& set);

template <class K, class V, class C>
size_t EstimateMemoryUsage(const base::flat_map<K, V, C>& map);

template <class K, class V, class C>
size_t EstimateMemoryUsage(const base::LRUCache<K, V, C>& lru);

template <class K, class V, class C>
size_t EstimateMemoryUsage(const base::HashingLRUCache<K, V, C>& lru);

template <class V, class C>
size_t EstimateMemoryUsage(const base::LRUCacheSet<V, C>& lru);

template <class V, class C>
size_t EstimateMemoryUsage(const base::HashingLRUCacheSet<V, C>& lru);

// TODO(dskiba):
//   std::forward_list

// Definitions

namespace internal {

// HasEMU<T> is true iff EstimateMemoryUsage(const T&) is available.
template <typename T>
concept HasEMU = requires(const T& t) {
  { EstimateMemoryUsage(t) } -> std::same_as<size_t>;
};

template <typename I>
using IteratorValueType = typename std::iterator_traits<I>::value_type;

template <typename I, typename InstantiatedContainer>
concept IsIteratorOfInstantiatedContainer =
    (std::same_as<typename InstantiatedContainer::iterator, I> ||
     std::same_as<typename InstantiatedContainer::const_iterator, I> ||
     std::same_as<typename InstantiatedContainer::reverse_iterator, I> ||
     std::same_as<typename InstantiatedContainer::const_reverse_iterator, I>);

template <typename I, template <typename...> typename Container>
concept IsIteratorOfContainer =
    !std::is_pointer_v<I> &&
    IsIteratorOfInstantiatedContainer<I, Container<IteratorValueType<I>>>;

// std::array has an extra required template argument.
template <typename T>
using array_test_helper = std::array<T, 1>;

// TODO(dyaroshev): deal with maps iterators if there is a need.
// It requires to parse pairs into keys and values.
// TODO(dyaroshev): deal with unordered containers: they do not have reverse
// iterators.
template <typename T>
concept IsIteratorOfStandardContainer =
    IsIteratorOfContainer<T, array_test_helper> ||
    IsIteratorOfContainer<T, std::vector> ||
    IsIteratorOfContainer<T, std::deque> ||
    IsIteratorOfContainer<T, std::list> || IsIteratorOfContainer<T, std::set> ||
    IsIteratorOfContainer<T, std::multiset>;

template <typename T>
concept IsKnownNonAllocatingType =
    std::is_trivially_destructible_v<T> || base::IsRawPtr<T> ||
    IsIteratorOfStandardContainer<T>;

}  // namespace internal

// Estimates T's memory usage as follows:
// 1. Calls `EstimateMemoryUsage(T)` if it is available.
// 2. If `EstimateMemoryUsage(T)` is not available, but T has trivial dtor
//    (i.e. it's POD, integer, pointer, enum, etc.) then it returns 0. This is
//    useful for containers, which allocate memory regardless of T (also for
//    cases like std::map<int, MyClass>).
// 3. Otherwise, it triggers a `static_assert` with a helpful message.
//
// To be used by `EstimateMemoryUsage()` implementations for containers.
template <class T>
size_t EstimateItemMemoryUsage(const T& value) {
  if constexpr (internal::HasEMU<T>) {
    return EstimateMemoryUsage(value);
  } else if constexpr (!internal::IsKnownNonAllocatingType<T>) {
    static_assert(base::AlwaysFalse<T>,
                  "Neither global function 'size_t EstimateMemoryUsage(T)' "
                  "nor member function 'size_t T::EstimateMemoryUsage() const' "
                  "is defined for the type.");
  }
  return 0;
}

template <class I>
size_t EstimateIterableMemoryUsage(const I& iterable) {
  size_t memory_usage = 0;
  for (const auto& item : iterable) {
    memory_usage += EstimateItemMemoryUsage(item);
  }
  return memory_usage;
}

// Global EstimateMemoryUsage(T) that just calls T::EstimateMemoryUsage().
template <class T>
auto EstimateMemoryUsage(const T& object)
    -> decltype(object.EstimateMemoryUsage()) {
  static_assert(std::same_as<decltype(object.EstimateMemoryUsage()), size_t>,
                "'T::EstimateMemoryUsage() const' must return size_t.");
  return object.EstimateMemoryUsage();
}

// String

template <class C, class T, class A>
size_t EstimateMemoryUsage(const std::basic_string<C, T, A>& string) {
  using string_type = std::basic_string<C, T, A>;
  using value_type = typename string_type::value_type;
  // C++11 doesn't leave much room for implementors - std::string can
  // use short string optimization, but that's about it. We detect SSO
  // by checking that c_str() points inside |string|.
  const uint8_t* cstr = reinterpret_cast<const uint8_t*>(string.c_str());
  const uint8_t* inline_cstr = reinterpret_cast<const uint8_t*>(&string);
  if (cstr >= inline_cstr && cstr < inline_cstr + sizeof(string)) {
    // SSO string
    return 0;
  }
  return (string.capacity() + 1) * sizeof(value_type);
}

// Use explicit instantiations from the .cc file (reduces bloat).
extern template BASE_EXPORT size_t EstimateMemoryUsage(const std::string&);
extern template BASE_EXPORT size_t EstimateMemoryUsage(const std::u16string&);

// Arrays

template <class T, size_t N>
size_t EstimateMemoryUsage(const std::array<T, N>& array) {
  return EstimateIterableMemoryUsage(array);
}

template <class T, size_t N>
size_t EstimateMemoryUsage(T (&array)[N]) {
  return EstimateIterableMemoryUsage(array);
}

template <class T>
size_t EstimateMemoryUsage(const base::HeapArray<T>& array) {
  return sizeof(T) * array.size() + EstimateIterableMemoryUsage(array);
}

template <class T>
size_t EstimateMemoryUsage(base::span<T> array) {
  return sizeof(T) * array.size() + EstimateIterableMemoryUsage(array);
}

// std::unique_ptr

template <class T, class D>
size_t EstimateMemoryUsage(const std::unique_ptr<T, D>& ptr) {
  return ptr ? (sizeof(T) + EstimateItemMemoryUsage(*ptr)) : 0;
}

// std::shared_ptr

template <class T>
size_t EstimateMemoryUsage(const std::shared_ptr<T>& ptr) {
  auto use_count = ptr.use_count();
  if (use_count == 0) {
    return 0;
  }
  // Model shared_ptr after libc++,
  // see __shared_ptr_pointer from include/memory
  struct SharedPointer {
    raw_ptr<void> vtbl;
    long shared_owners;
    long shared_weak_owners;
    raw_ptr<T> value;
  };
  // If object of size S shared N > S times we prefer to (potentially)
  // overestimate than to return 0.
  return sizeof(SharedPointer) +
         (EstimateItemMemoryUsage(*ptr) + (use_count - 1)) / use_count;
}

// std::pair

template <class F, class S>
size_t EstimateMemoryUsage(const std::pair<F, S>& pair) {
  return EstimateItemMemoryUsage(pair.first) +
         EstimateItemMemoryUsage(pair.second);
}

// std::vector

template <class T, class A>
size_t EstimateMemoryUsage(const std::vector<T, A>& vector) {
  return sizeof(T) * vector.capacity() + EstimateIterableMemoryUsage(vector);
}

// std::list

template <class T, class A>
size_t EstimateMemoryUsage(const std::list<T, A>& list) {
  using value_type = typename std::list<T, A>::value_type;
  struct Node {
    raw_ptr<Node> prev;
    raw_ptr<Node> next;
    value_type value;
  };
  return sizeof(Node) * list.size() + EstimateIterableMemoryUsage(list);
}

template <class T>
size_t EstimateMemoryUsage(const base::LinkedList<T>& list) {
  size_t memory_usage = 0u;
  for (base::LinkNode<T>* node = list.head(); node != list.end();
       node = node->next()) {
    // Since we increment by calling node = node->next() we know that node
    // isn't nullptr.
    memory_usage += EstimateMemoryUsage(*node->value()) + sizeof(T);
  }
  return memory_usage;
}

// Tree containers

template <class V>
size_t EstimateTreeMemoryUsage(size_t size) {
  // Tree containers are modeled after libc++
  // (__tree_node from include/__tree)
  struct Node {
    raw_ptr<Node> left;
    raw_ptr<Node> right;
    raw_ptr<Node> parent;
    bool is_black;
    V value;
  };
  return sizeof(Node) * size;
}

template <class T, class C, class A>
size_t EstimateMemoryUsage(const std::set<T, C, A>& set) {
  using value_type = typename std::set<T, C, A>::value_type;
  return EstimateTreeMemoryUsage<value_type>(set.size()) +
         EstimateIterableMemoryUsage(set);
}

template <class T, class C, class A>
size_t EstimateMemoryUsage(const std::multiset<T, C, A>& set) {
  using value_type = typename std::multiset<T, C, A>::value_type;
  return EstimateTreeMemoryUsage<value_type>(set.size()) +
         EstimateIterableMemoryUsage(set);
}

template <class K, class V, class C, class A>
size_t EstimateMemoryUsage(const std::map<K, V, C, A>& map) {
  using value_type = typename std::map<K, V, C, A>::value_type;
  return EstimateTreeMemoryUsage<value_type>(map.size()) +
         EstimateIterableMemoryUsage(map);
}

template <class K, class V, class C, class A>
size_t EstimateMemoryUsage(const std::multimap<K, V, C, A>& map) {
  using value_type = typename std::multimap<K, V, C, A>::value_type;
  return EstimateTreeMemoryUsage<value_type>(map.size()) +
         EstimateIterableMemoryUsage(map);
}

// HashMap containers

namespace internal {

// While hashtable containers model doesn't depend on STL implementation, one
// detail still crept in: bucket_count. It's used in size estimation, but its
// value after inserting N items is not predictable.
// This function is specialized by unittests to return constant value, thus
// excluding bucket_count from testing.
template <class V>
size_t HashMapBucketCountForTesting(size_t bucket_count) {
  return bucket_count;
}

template <class LruCacheType>
size_t DoEstimateMemoryUsageForLruCache(const LruCacheType& lru_cache) {
  return EstimateMemoryUsage(lru_cache.ordering_) +
         EstimateMemoryUsage(lru_cache.index_);
}

}  // namespace internal

template <class V>
size_t EstimateHashMapMemoryUsage(size_t bucket_count, size_t size) {
  // Hashtable containers are modeled after libc++
  // (__hash_node from include/__hash_table)
  struct Node {
    raw_ptr<void> next;
    size_t hash;
    V value;
  };
  using Bucket = void*;
  bucket_count = internal::HashMapBucketCountForTesting<V>(bucket_count);
  return sizeof(Bucket) * bucket_count + sizeof(Node) * size;
}

template <class K, class H, class KE, class A>
size_t EstimateMemoryUsage(const std::unordered_set<K, H, KE, A>& set) {
  using value_type = typename std::unordered_set<K, H, KE, A>::value_type;
  return EstimateHashMapMemoryUsage<value_type>(set.bucket_count(),
                                                set.size()) +
         EstimateIterableMemoryUsage(set);
}

template <class K, class H, class KE, class A>
size_t EstimateMemoryUsage(const std::unordered_multiset<K, H, KE, A>& set) {
  using value_type = typename std::unordered_multiset<K, H, KE, A>::value_type;
  return EstimateHashMapMemoryUsage<value_type>(set.bucket_count(),
                                                set.size()) +
         EstimateIterableMemoryUsage(set);
}

template <class K, class V, class H, class KE, class A>
size_t EstimateMemoryUsage(const std::unordered_map<K, V, H, KE, A>& map) {
  using value_type = typename std::unordered_map<K, V, H, KE, A>::value_type;
  return EstimateHashMapMemoryUsage<value_type>(map.bucket_count(),
                                                map.size()) +
         EstimateIterableMemoryUsage(map);
}

template <class K, class V, class H, class KE, class A>
size_t EstimateMemoryUsage(const std::unordered_multimap<K, V, H, KE, A>& map) {
  using value_type =
      typename std::unordered_multimap<K, V, H, KE, A>::value_type;
  return EstimateHashMapMemoryUsage<value_type>(map.bucket_count(),
                                                map.size()) +
         EstimateIterableMemoryUsage(map);
}

// std::deque

template <class T, class A>
size_t EstimateMemoryUsage(const std::deque<T, A>& deque) {
// Since std::deque implementations are wildly different
// (see crbug.com/674287), we can't have one "good enough"
// way to estimate.

// kBlockSize      - minimum size of a block, in bytes
// kMinBlockLength - number of elements in a block
//                   if sizeof(T) > kBlockSize
#if defined(_LIBCPP_VERSION)
  size_t kBlockSize = 4096;
  size_t kMinBlockLength = 16;
#elif defined(__GLIBCXX__)
  size_t kBlockSize = 512;
  size_t kMinBlockLength = 1;
#elif defined(_MSC_VER)
  size_t kBlockSize = 16;
  size_t kMinBlockLength = 1;
#else
  size_t kBlockSize = 0;
  size_t kMinBlockLength = 1;
#endif

  size_t block_length =
      (sizeof(T) > kBlockSize) ? kMinBlockLength : kBlockSize / sizeof(T);

  size_t blocks = (deque.size() + block_length - 1) / block_length;

#if defined(__GLIBCXX__)
  // libstdc++: deque always has at least one block
  if (!blocks) {
    blocks = 1;
  }
#endif

#if defined(_LIBCPP_VERSION)
  // libc++: deque keeps at most two blocks when it shrinks,
  // so even if the size is zero, deque might be holding up
  // to 4096 * 2 bytes. One way to know whether deque has
  // ever allocated (and hence has 1 or 2 blocks) is to check
  // iterator's pointer. Non-zero value means that deque has
  // at least one block.
  if (!blocks && deque.begin().operator->()) {
    blocks = 1;
  }
#endif

  return (blocks * block_length * sizeof(T)) +
         EstimateIterableMemoryUsage(deque);
}

// Container adapters

template <class T, class C>
size_t EstimateMemoryUsage(const std::queue<T, C>& queue) {
  return EstimateMemoryUsage(GetUnderlyingContainer(queue));
}

template <class T, class C>
size_t EstimateMemoryUsage(const std::priority_queue<T, C>& queue) {
  return EstimateMemoryUsage(GetUnderlyingContainer(queue));
}

template <class T, class C>
size_t EstimateMemoryUsage(const std::stack<T, C>& stack) {
  return EstimateMemoryUsage(GetUnderlyingContainer(stack));
}

// base::circular_deque

template <class T>
size_t EstimateMemoryUsage(const base::circular_deque<T>& deque) {
  return sizeof(T) * deque.capacity() + EstimateIterableMemoryUsage(deque);
}

// Flat containers

template <class T, class C>
size_t EstimateMemoryUsage(const base::flat_set<T, C>& set) {
  using value_type = typename base::flat_set<T, C>::value_type;
  return sizeof(value_type) * set.capacity() + EstimateIterableMemoryUsage(set);
}

template <class K, class V, class C>
size_t EstimateMemoryUsage(const base::flat_map<K, V, C>& map) {
  using value_type = typename base::flat_map<K, V, C>::value_type;
  return sizeof(value_type) * map.capacity() + EstimateIterableMemoryUsage(map);
}

template <class K, class V, class C>
size_t EstimateMemoryUsage(const LRUCache<K, V, C>& lru_cache) {
  return internal::DoEstimateMemoryUsageForLruCache(lru_cache);
}

template <class K, class V, class C>
size_t EstimateMemoryUsage(const HashingLRUCache<K, V, C>& lru_cache) {
  return internal::DoEstimateMemoryUsageForLruCache(lru_cache);
}

template <class V, class C>
size_t EstimateMemoryUsage(const LRUCacheSet<V, C>& lru_cache) {
  return internal::DoEstimateMemoryUsageForLruCache(lru_cache);
}

template <class V, class C>
size_t EstimateMemoryUsage(const HashingLRUCacheSet<V, C>& lru_cache) {
  return internal::DoEstimateMemoryUsageForLruCache(lru_cache);
}

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_MEMORY_USAGE_ESTIMATOR_H_
