/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This implementation is borrowed from Chromium.

#ifndef RTC_BASE_CONTAINERS_FLAT_MAP_H_
#define RTC_BASE_CONTAINERS_FLAT_MAP_H_

#include <functional>
#include <tuple>
#include <utility>
#include <vector>

#include "rtc_base/checks.h"
#include "rtc_base/containers/flat_tree.h"  // IWYU pragma: export

namespace webrtc {

namespace flat_containers_internal {

// An implementation of the flat_tree GetKeyFromValue template parameter that
// extracts the key as the first element of a pair.
struct GetFirst {
  template <class Key, class Mapped>
  constexpr const Key& operator()(const std::pair<Key, Mapped>& p) const {
    return p.first;
  }
};

}  // namespace flat_containers_internal

// flat_map is a container with a std::map-like interface that stores its
// contents in a sorted container, by default a vector.
//
// Its implementation mostly tracks the corresponding standardization proposal
// https://wg21.link/P0429, except that the storage of keys and values is not
// split.
//
// PROS
//
//  - Good memory locality.
//  - Low overhead, especially for smaller maps.
//  - Performance is good for more workloads than you might expect (see
//    //base/containers/README.md in Chromium repository)
//  - Supports C++14 map interface.
//
// CONS
//
//  - Inserts and removals are O(n).
//
// IMPORTANT NOTES
//
//  - Iterators are invalidated across mutations. This means that the following
//    line of code has undefined behavior since adding a new element could
//    resize the container, invalidating all iterators:
//      container["new element"] = it.second;
//  - If possible, construct a flat_map in one operation by inserting into
//    a container and moving that container into the flat_map constructor.
//
// QUICK REFERENCE
//
// Most of the core functionality is inherited from flat_tree. Please see
// flat_tree.h for more details for most of these functions. As a quick
// reference, the functions available are:
//
// Constructors (inputs need not be sorted):
//   flat_map(const flat_map&);
//   flat_map(flat_map&&);
//   flat_map(InputIterator first, InputIterator last,
//            const Compare& compare = Compare());
//   flat_map(const container_type& items,
//            const Compare& compare = Compare());
//   flat_map(container_type&& items,
//            const Compare& compare = Compare()); // Re-use storage.
//   flat_map(std::initializer_list<value_type> ilist,
//            const Compare& comp = Compare());
//
// Constructors (inputs need to be sorted):
//   flat_map(sorted_unique_t,
//            InputIterator first, InputIterator last,
//            const Compare& compare = Compare());
//   flat_map(sorted_unique_t,
//            const container_type& items,
//            const Compare& compare = Compare());
//   flat_map(sorted_unique_t,
//            container_type&& items,
//            const Compare& compare = Compare());  // Re-use storage.
//   flat_map(sorted_unique_t,
//            std::initializer_list<value_type> ilist,
//            const Compare& comp = Compare());
//
// Assignment functions:
//   flat_map& operator=(const flat_map&);
//   flat_map& operator=(flat_map&&);
//   flat_map& operator=(initializer_list<value_type>);
//
// Memory management functions:
//   void   reserve(size_t);
//   size_t capacity() const;
//   void   shrink_to_fit();
//
// Size management functions:
//   void   clear();
//   size_t size() const;
//   size_t max_size() const;
//   bool   empty() const;
//
// Iterator functions:
//   iterator               begin();
//   const_iterator         begin() const;
//   const_iterator         cbegin() const;
//   iterator               end();
//   const_iterator         end() const;
//   const_iterator         cend() const;
//   reverse_iterator       rbegin();
//   const reverse_iterator rbegin() const;
//   const_reverse_iterator crbegin() const;
//   reverse_iterator       rend();
//   const_reverse_iterator rend() const;
//   const_reverse_iterator crend() const;
//
// Insert and accessor functions:
//   mapped_type&         operator[](const key_type&);
//   mapped_type&         operator[](key_type&&);
//   mapped_type&         at(const K&);
//   const mapped_type&   at(const K&) const;
//   pair<iterator, bool> insert(const value_type&);
//   pair<iterator, bool> insert(value_type&&);
//   iterator             insert(const_iterator hint, const value_type&);
//   iterator             insert(const_iterator hint, value_type&&);
//   void                 insert(InputIterator first, InputIterator last);
//   pair<iterator, bool> insert_or_assign(K&&, M&&);
//   iterator             insert_or_assign(const_iterator hint, K&&, M&&);
//   pair<iterator, bool> emplace(Args&&...);
//   iterator             emplace_hint(const_iterator, Args&&...);
//   pair<iterator, bool> try_emplace(K&&, Args&&...);
//   iterator             try_emplace(const_iterator hint, K&&, Args&&...);

// Underlying type functions:
//   container_type       extract() &&;
//   void                 replace(container_type&&);
//
// Erase functions:
//   iterator erase(iterator);
//   iterator erase(const_iterator);
//   iterator erase(const_iterator first, const_iterator& last);
//   template <class K> size_t erase(const K& key);
//
// Comparators (see std::map documentation).
//   key_compare   key_comp() const;
//   value_compare value_comp() const;
//
// Search functions:
//   template <typename K> size_t                   count(const K&) const;
//   template <typename K> iterator                 find(const K&);
//   template <typename K> const_iterator           find(const K&) const;
//   template <typename K> bool                     contains(const K&) const;
//   template <typename K> pair<iterator, iterator> equal_range(const K&);
//   template <typename K> iterator                 lower_bound(const K&);
//   template <typename K> const_iterator           lower_bound(const K&) const;
//   template <typename K> iterator                 upper_bound(const K&);
//   template <typename K> const_iterator           upper_bound(const K&) const;
//
// General functions:
//   void swap(flat_map&);
//
// Non-member operators:
//   bool operator==(const flat_map&, const flat_map);
//   bool operator!=(const flat_map&, const flat_map);
//   bool operator<(const flat_map&, const flat_map);
//   bool operator>(const flat_map&, const flat_map);
//   bool operator>=(const flat_map&, const flat_map);
//   bool operator<=(const flat_map&, const flat_map);
//
template <class Key,
          class Mapped,
          class Compare = std::less<>,
          class Container = std::vector<std::pair<Key, Mapped>>>
class flat_map : public ::webrtc::flat_containers_internal::flat_tree<
                     Key,
                     flat_containers_internal::GetFirst,
                     Compare,
                     Container> {
 private:
  using tree = typename ::webrtc::flat_containers_internal::
      flat_tree<Key, flat_containers_internal::GetFirst, Compare, Container>;

 public:
  using key_type = typename tree::key_type;
  using mapped_type = Mapped;
  using value_type = typename tree::value_type;
  using reference = typename Container::reference;
  using const_reference = typename Container::const_reference;
  using size_type = typename Container::size_type;
  using difference_type = typename Container::difference_type;
  using iterator = typename tree::iterator;
  using const_iterator = typename tree::const_iterator;
  using reverse_iterator = typename tree::reverse_iterator;
  using const_reverse_iterator = typename tree::const_reverse_iterator;
  using container_type = typename tree::container_type;

  // --------------------------------------------------------------------------
  // Lifetime and assignments.
  //
  // Note: we explicitly bring operator= in because otherwise
  //   flat_map<...> x;
  //   x = {...};
  // Would first create a flat_map and then move assign it. This most likely
  // would be optimized away but still affects our debug builds.

  using tree::tree;
  using tree::operator=;

  // Out-of-bound calls to at() will CHECK.
  template <class K>
  mapped_type& at(const K& key);
  template <class K>
  const mapped_type& at(const K& key) const;

  // --------------------------------------------------------------------------
  // Map-specific insert operations.
  //
  // Normal insert() functions are inherited from flat_tree.
  //
  // Assume that every operation invalidates iterators and references.
  // Insertion of one element can take O(size).

  mapped_type& operator[](const key_type& key);
  mapped_type& operator[](key_type&& key);

  template <class K, class M>
  std::pair<iterator, bool> insert_or_assign(K&& key, M&& obj);
  template <class K, class M>
  iterator insert_or_assign(const_iterator hint, K&& key, M&& obj);

  template <class K, class... Args>
  std::enable_if_t<std::is_constructible<key_type, K&&>::value,
                   std::pair<iterator, bool>>
  try_emplace(K&& key, Args&&... args);

  template <class K, class... Args>
  std::enable_if_t<std::is_constructible<key_type, K&&>::value, iterator>
  try_emplace(const_iterator hint, K&& key, Args&&... args);

  // --------------------------------------------------------------------------
  // General operations.
  //
  // Assume that swap invalidates iterators and references.

  void swap(flat_map& other) noexcept;

  friend void swap(flat_map& lhs, flat_map& rhs) noexcept { lhs.swap(rhs); }
};

// ----------------------------------------------------------------------------
// Lookups.

template <class Key, class Mapped, class Compare, class Container>
template <class K>
auto flat_map<Key, Mapped, Compare, Container>::at(const K& key)
    -> mapped_type& {
  iterator found = tree::find(key);
  RTC_CHECK(found != tree::end());
  return found->second;
}

template <class Key, class Mapped, class Compare, class Container>
template <class K>
auto flat_map<Key, Mapped, Compare, Container>::at(const K& key) const
    -> const mapped_type& {
  const_iterator found = tree::find(key);
  RTC_CHECK(found != tree::cend());
  return found->second;
}

// ----------------------------------------------------------------------------
// Insert operations.

template <class Key, class Mapped, class Compare, class Container>
auto flat_map<Key, Mapped, Compare, Container>::operator[](const key_type& key)
    -> mapped_type& {
  iterator found = tree::lower_bound(key);
  if (found == tree::end() || tree::key_comp()(key, found->first))
    found = tree::unsafe_emplace(found, key, mapped_type());
  return found->second;
}

template <class Key, class Mapped, class Compare, class Container>
auto flat_map<Key, Mapped, Compare, Container>::operator[](key_type&& key)
    -> mapped_type& {
  iterator found = tree::lower_bound(key);
  if (found == tree::end() || tree::key_comp()(key, found->first))
    found = tree::unsafe_emplace(found, std::move(key), mapped_type());
  return found->second;
}

template <class Key, class Mapped, class Compare, class Container>
template <class K, class M>
auto flat_map<Key, Mapped, Compare, Container>::insert_or_assign(K&& key,
                                                                 M&& obj)
    -> std::pair<iterator, bool> {
  auto result =
      tree::emplace_key_args(key, std::forward<K>(key), std::forward<M>(obj));
  if (!result.second)
    result.first->second = std::forward<M>(obj);
  return result;
}

template <class Key, class Mapped, class Compare, class Container>
template <class K, class M>
auto flat_map<Key, Mapped, Compare, Container>::insert_or_assign(
    const_iterator hint,
    K&& key,
    M&& obj) -> iterator {
  auto result = tree::emplace_hint_key_args(hint, key, std::forward<K>(key),
                                            std::forward<M>(obj));
  if (!result.second)
    result.first->second = std::forward<M>(obj);
  return result.first;
}

template <class Key, class Mapped, class Compare, class Container>
template <class K, class... Args>
auto flat_map<Key, Mapped, Compare, Container>::try_emplace(K&& key,
                                                            Args&&... args)
    -> std::enable_if_t<std::is_constructible<key_type, K&&>::value,
                        std::pair<iterator, bool>> {
  return tree::emplace_key_args(
      key, std::piecewise_construct,
      std::forward_as_tuple(std::forward<K>(key)),
      std::forward_as_tuple(std::forward<Args>(args)...));
}

template <class Key, class Mapped, class Compare, class Container>
template <class K, class... Args>
auto flat_map<Key, Mapped, Compare, Container>::try_emplace(const_iterator hint,
                                                            K&& key,
                                                            Args&&... args)
    -> std::enable_if_t<std::is_constructible<key_type, K&&>::value, iterator> {
  return tree::emplace_hint_key_args(
             hint, key, std::piecewise_construct,
             std::forward_as_tuple(std::forward<K>(key)),
             std::forward_as_tuple(std::forward<Args>(args)...))
      .first;
}

// ----------------------------------------------------------------------------
// General operations.

template <class Key, class Mapped, class Compare, class Container>
void flat_map<Key, Mapped, Compare, Container>::swap(flat_map& other) noexcept {
  tree::swap(other);
}

// Erases all elements that match predicate. It has O(size) complexity.
//
//  flat_map<int, Timestamp> last_times;
//  ...
//  EraseIf(last_times,
//          [&](const auto& element) { return now - element.second > kLimit; });

// NOLINTNEXTLINE(misc-unused-using-decls)
using ::webrtc::flat_containers_internal::EraseIf;

}  // namespace webrtc

#endif  // RTC_BASE_CONTAINERS_FLAT_MAP_H_
