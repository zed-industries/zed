// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_FIXED_FLAT_SET_H_
#define BASE_CONTAINERS_FIXED_FLAT_SET_H_

#include <algorithm>
#include <array>
#include <functional>
#include <type_traits>

#include "base/containers/flat_set.h"
#include "base/containers/flat_tree.h"

namespace base {

namespace internal {
// Not constexpr to trigger a compile error.
void FixedFlatSetInputNotSortedOrNotUnique();
}  // namespace internal

// fixed_flat_set is a immutable container with a std::set-like interface that
// stores its contents in a sorted std::array.
//
// It is a special case of base::flat_set, and mostly useful as a look-up table.
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
//   fixed_flat_set(const fixed_flat_set&);
//   fixed_flat_set(sorted_unique_t,
//                  const container_type& items,
//                  const Compare& compare = Compare());
//
// Size management functions:
//   size_t size() const;
//   size_t max_size() const;
//   bool   empty() const;
//
// Iterator functions:
//   const_iterator         begin() const;
//   const_iterator         cbegin() const;
//   const_iterator         end() const;
//   const_iterator         cend() const;
//   const reverse_iterator rbegin() const;
//   const_reverse_iterator crbegin() const;
//   const_reverse_iterator rend() const;
//   const_reverse_iterator crend() const;
//
// Comparators (see std::set documentation).
//   key_compare   key_comp() const;
//   value_compare value_comp() const;
//
// Search functions:
//   template <typename K> size_t                   count(const K&) const;
//   template <typename K> const_iterator           find(const K&) const;
//   template <typename K> bool                     contains(const K&) const;
//   template <typename K>
//       pair<const_iterator, const_iterator>       equal_range(K&) const;
//   template <typename K> const_iterator           lower_bound(const K&) const;
//   template <typename K> const_iterator           upper_bound(const K&) const;
//
// Non-member operators:
//   bool operator==(const fixed_flat_set&, const fixed_flat_set&);
//   bool operator!=(const fixed_flat_set&, const fixed_flat_set&);
//   bool operator<(const fixed_flat_set&, const fixed_flat_set&);
//   bool operator>(const fixed_flat_set&, const fixed_flat_set&);
//   bool operator>=(const fixed_flat_set&, const fixed_flat_set&);
//   bool operator<=(const fixed_flat_set&, const fixed_flat_set&);
//
template <class Key, size_t N, class Compare = std::less<>>
using fixed_flat_set = base::flat_set<Key, Compare, std::array<const Key, N>>;

// Utility function to simplify constructing a fixed_flat_set from a fixed list
// of keys and values. Requires that the passed in `data` contains unique keys
// and be sorted by key. See `MakeFixedFlatSet` for a variant that sorts the
// input automatically.
//
// Example usage:
//   constexpr auto kSet = base::MakeFixedFlatSet<std::string_view>(
//       base::sorted_unique, {"bar", "baz", "foo", "qux"});
template <class Key, size_t N, class Compare = std::less<>>
consteval fixed_flat_set<Key, N, Compare> MakeFixedFlatSet(
    sorted_unique_t,
    std::common_type_t<Key> (&&data)[N],
    const Compare& comp = Compare()) {
  if (!internal::is_sorted_and_unique(data, comp)) {
    internal::FixedFlatSetInputNotSortedOrNotUnique();
  }
  // Specify the value_type explicitly to ensure that the returned array has
  // immutable keys.
  return fixed_flat_set<Key, N, Compare>(
      sorted_unique, internal::ToArray<const Key>(data), comp);
}

// Utility function to simplify constructing a fixed_flat_set from a fixed list
// of keys. Requires that the passed in `data` contains unique keys.
//
// Large inputs will run into compiler limits, e.g. "constexpr evaluation hit
// maximum step limit". In that case, use `MakeFixedFlatSet(sorted_unique)`.
//
// Example usage:
//   constexpr auto kIntSet = base::MakeFixedFlatSet<int>({1, 2, 3, 4});
//
// Data needs not to be sorted:
//   constexpr auto kStringSet = base::MakeFixedFlatSet<std::string_view>(
//       {"foo", "bar", "baz", "qux"});
//
// Note: Wrapping `Key` in `std::common_type_t` below requires callers to
// explicitly specify `Key`, which is desired here.
template <class Key, class Compare = std::less<>, size_t N>
consteval fixed_flat_set<Key, N, Compare> MakeFixedFlatSet(
    std::common_type_t<Key> (&&data)[N],
    const Compare& comp = Compare()) {
  std::ranges::sort(data, comp);
  return MakeFixedFlatSet<Key>(sorted_unique, std::move(data), comp);
}

}  // namespace base

#endif  // BASE_CONTAINERS_FIXED_FLAT_SET_H_
