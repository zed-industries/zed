// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CONTAINERS_CONTAINS_H_
#define BASE_CONTAINERS_CONTAINS_H_

// Provides `Contains()`, a general purpose utility to check whether a container
// contains a value. This will probe whether a `contains` or `find` member
// function on `container` exists, and fall back to a generic linear search over
// `container`.

#include <algorithm>
#include <ranges>
#include <type_traits>
#include <utility>
// TODO(dcheng): Remove this after fixing any IWYU errors.
#include <ranges>

namespace base {

// A general purpose utility to check whether `container` contains `value`. This
// will probe whether a `contains` or `find` member function on `container`
// exists, and fall back to a generic linear search over `container`.
template <typename Container, typename Value>
constexpr bool Contains(const Container& container, const Value& value) {
  if constexpr (requires {
                  { container.contains(value) } -> std::same_as<bool>;
                }) {
    return container.contains(value);
  } else if constexpr (requires { container.find(value) != Container::npos; }) {
    return container.find(value) != Container::npos;
  } else if constexpr (requires { container.find(value) != container.end(); }) {
    return container.find(value) != container.end();
  } else {
    static_assert(
        !requires { typename Container::key_type; },
        "Error: About to perform linear search on an associative container. "
        "Either use a more generic comparator (e.g. std::less<>) or, if a "
        "linear search is desired, provide an explicit projection parameter.");
    return std::ranges::find(container, value) != std::ranges::end(container);
  }
}

// Overload that allows callers to provide an additional projection invocable.
// This projection will be applied to every element in `container` before
// comparing it with `value`. This will always perform a linear search.
template <typename Container, typename Value, typename Proj>
constexpr bool Contains(const Container& container,
                        const Value& value,
                        Proj proj) {
  return std::ranges::find(container, value, std::move(proj)) !=
         std::ranges::end(container);
}

}  // namespace base

#endif  // BASE_CONTAINERS_CONTAINS_H_
