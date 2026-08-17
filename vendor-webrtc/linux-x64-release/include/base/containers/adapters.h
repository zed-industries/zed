// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_ADAPTERS_H_
#define BASE_CONTAINERS_ADAPTERS_H_

#include <ranges>
#include <type_traits>
#include <utility>

#include "base/compiler_specific.h"
#include "base/containers/adapters_internal.h"

namespace base {

// Returns a range adapter that exposes its elements as rvalues. When used as an
// input range, this means the values in `range` will be moved from (and
// the values potentially in an unspecified but valid state).
template <typename Range>
  requires(std::ranges::input_range<Range> &&
           // The elements in the input range will be consumed, so restrict this
           // to non-borrowed ranges, as the elements from a borrowed range can
           // outlive this call.
           !std::ranges::borrowed_range<Range> &&
           // Disallow input ranges if the elements cannot be moved from (e.g.
           // if they are const-qualified).
           std::movable<
               std::remove_reference_t<std::ranges::range_reference_t<Range>>>)
auto RangeAsRvalues(Range&& range LIFETIME_BOUND) {
  return internal::RangeOfRvaluesAdapter<Range>(std::forward<Range>(range));
}

// Reversed returns a container adapter usable in a range-based "for" statement
// for iterating a reversible container in reverse order.
//
// Example:
//
//   std::vector<int> v = ...;
//   for (int i : base::Reversed(v)) {
//     // iterates through v from back to front
//   }
template <typename Range>
auto Reversed(Range&& range LIFETIME_BOUND) {
  return internal::ReversedAdapter<Range>(std::forward<Range>(range));
}

}  // namespace base

#endif  // BASE_CONTAINERS_ADAPTERS_H_
