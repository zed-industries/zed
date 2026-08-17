// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_FIXED_FLAT_MAP_H_
#define BASE_CONTAINERS_FIXED_FLAT_MAP_H_

#include <algorithm>
#include <array>
#include <functional>
#include <utility>

#include "base/containers/flat_map.h"
#include "base/containers/flat_tree.h"

namespace base {

namespace internal {
// Not constexpr to trigger a compile error.
void FixedFlatMapInputNotSortedOrNotUnique();
}  // namespace internal

// fixed_flat_map is a immutable container with a std::map-like interface that
// stores its contents in a sorted std::array.
//
// It is a special case of base::flat_map, and mostly useful as a look-up table.
//
// Please see //base/containers/README.md for an overview of which container
// to select.
//
// QUICK REFERENCE
//
// Most of the core functionality is inherited from flat_tree. Please see
// flat_tree.h for more details for most of these functions. As a quick
// reference, the functions available are:
//
// Constructors (inputs need to be sorted):
//   fixed_flat_map(const fixed_flat_map&);
//   fixed_flat_map(sorted_unique_t,
//                  const container_type& items,
//                  const Compare& compare = Compare());
//
// Size management functions:
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
//   mapped_type&         at(const K&);
//   const mapped_type&   at(const K&) const;

// Comparators (see std::map documentation).
//   key_compare   key_comp() const;
//   value_compare value_comp() const;
//
// Search functions:
//   template <typename K> size_t                   count(const K&) const;
//   template <typename K> iterator                 find(const K&);
//   template <typename K> const_iterator           find(const K&) const;
//   template <typename K> bool                     contains(const K&) const;
//   template <typename K>
//       pair<iterator, iterator>                   equal_range(const K&);
//   template <typename K>
//       pair<const_iterator, const_iterator>       equal_range(K&) const;
//   template <typename K> iterator                 lower_bound(const K&);
//   template <typename K> const_iterator           lower_bound(const K&) const;
//   template <typename K> iterator                 upper_bound(const K&);
//   template <typename K> const_iterator           upper_bound(const K&) const;
//
// Non-member operators:
//   bool operator==(const fixed_flat_map&, const fixed_flat_map&);
//   bool operator!=(const fixed_flat_map&, const fixed_flat_map&);
//   bool operator<(const fixed_flat_map&, const fixed_flat_map&);
//   bool operator>(const fixed_flat_map&, const fixed_flat_map&);
//   bool operator>=(const fixed_flat_map&, const fixed_flat_map&);
//   bool operator<=(const fixed_flat_map&, const fixed_flat_map&);
//
template <class Key, class Mapped, size_t N, class Compare = std::less<>>
using fixed_flat_map = base::
    flat_map<Key, Mapped, Compare, std::array<std::pair<const Key, Mapped>, N>>;

// Utility function to simplify constructing a fixed_flat_map from a fixed list
// of keys and values. Requires that the passed in `data` contains unique keys.
//
// Large inputs will run into compiler limits, e.g. "constexpr evaluation hit
// maximum step limit", unless `data` is already sorted.
//
// Example usage:
//   constexpr auto kMap = base::MakeFixedFlatMap<std::string_view, int>(
//       {{"foo", 1}, {"bar", 2}, {"baz", 3}});
template <class Key, class Mapped, class Compare = std::less<>, size_t N>
consteval fixed_flat_map<Key, Mapped, N, Compare> MakeFixedFlatMap(
    std::pair<Key, Mapped> (&&data)[N],
    const Compare& comp = Compare()) {
  using FixedFlatMap = fixed_flat_map<Key, Mapped, N, Compare>;
  typename FixedFlatMap::value_compare value_comp{comp};
  if (!internal::is_sorted_and_unique(data, value_comp)) {
    std::ranges::sort(data, value_comp);
    if (!internal::is_sorted_and_unique(data, value_comp)) {
      internal::FixedFlatMapInputNotSortedOrNotUnique();
    }
  }
  return FixedFlatMap(
      sorted_unique, internal::ToArray<typename FixedFlatMap::value_type>(data),
      comp);
}

}  // namespace base

#endif  // BASE_CONTAINERS_FIXED_FLAT_MAP_H_
