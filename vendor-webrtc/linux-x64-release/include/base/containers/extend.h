// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_EXTEND_H_
#define BASE_CONTAINERS_EXTEND_H_

#include <algorithm>
#include <iterator>
#include <ranges>
#include <type_traits>
#include <vector>

namespace base {

// Append to |dst| all elements of |src| by std::move-ing them out of |src|.
// After this operation, |src| will be empty.
template <typename T>
void Extend(std::vector<T>& dst, std::vector<T>&& src) {
  dst.insert(dst.end(), std::make_move_iterator(src.begin()),
             std::make_move_iterator(src.end()));
  src.clear();
}

// Appends `range` to `dst`, copying them out of `range`.
template <typename T, typename Range, typename Proj = std::identity>
  requires std::ranges::range<Range> &&
           std::indirectly_unary_invocable<Proj, std::ranges::iterator_t<Range>>
void Extend(std::vector<T>& dst, Range&& range, Proj proj = {}) {
  if constexpr (std::ranges::sized_range<Range>) {
    dst.reserve(dst.size() + std::ranges::size(range));
  }
  std::ranges::transform(std::forward<Range>(range), std::back_inserter(dst),
                         std::move(proj));
}

}  // namespace base

#endif  // BASE_CONTAINERS_EXTEND_H_
