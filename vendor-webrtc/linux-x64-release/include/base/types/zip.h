// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_ZIP_H_
#define BASE_TYPES_ZIP_H_

#include <algorithm>
#include <iterator>
#include <tuple>
#include <utility>

#include "base/check.h"
#include "base/compiler_specific.h"

namespace base {

namespace internal {

template <typename... Ranges>
class Zipper {
 public:
  constexpr explicit Zipper(Ranges&... ranges LIFETIME_BOUND) noexcept
      : ranges_(ranges...) {}

  // A sentinel used by the iterator to constrain the comparison to make sure it
  // has the proper end of each range.
  struct ZipEnd {};

  class iterator {
   public:
    using value_type = std::tuple<
        std::remove_cv_t<decltype(*std::begin(std::declval<Ranges&>()))>...>;
    using reference = value_type;
    using element_type = value_type;
    using difference_type = std::ptrdiff_t;
    using pointer = void;
    // TODO(https://crbug.com/377940847): This could be improved going forward
    // to select a better iterator category, based on the common denominator of
    // the union of iterators, for instance output iterators, etc.
    using iterator_category = std::input_iterator_tag;
    using iterator_concept = std::input_iterator_tag;

    constexpr iterator& operator++() noexcept LIFETIME_BOUND {
      advance(std::index_sequence_for<Ranges...>{});
      return *this;
    }

    constexpr auto operator*() const noexcept LIFETIME_BOUND {
      return deref(std::index_sequence_for<Ranges...>{});
    }

    // Determines if the iterator has reached the end, so a for-range loop bails
    // out.
    constexpr bool operator!=(ZipEnd) const noexcept LIFETIME_BOUND {
      return has_more(std::index_sequence_for<Ranges...>{});
    }

   private:
    friend class Zipper;

    constexpr explicit iterator(
        std::tuple<decltype(std::begin(std::declval<Ranges&>()))...> begin
            LIFETIME_BOUND,
        std::tuple<decltype(std::end(std::declval<Ranges&>()))...> end
            LIFETIME_BOUND) noexcept
        : begin_(begin), end_(end) {}

    // Checks if any range has reached the end.
    template <std::size_t... Is>
    constexpr bool has_more(std::index_sequence<Is...>) const {
      return (... && (std::get<Is>(begin_) != std::get<Is>(end_)));
    }

    template <std::size_t... Is>
    constexpr void advance(std::index_sequence<Is...>) {
      CHECK(operator!=(ZipEnd()));
      // SAFETY: The increment is safe as it has been just CHECKed so it is
      // guaranteed to be inside [begin_, end_).
      UNSAFE_BUFFERS((++std::get<Is>(begin_), ...));
    }

    template <size_t... Is>
    constexpr value_type deref(std::index_sequence<Is...>) const
        LIFETIME_BOUND {
      return {*std::get<Is>(begin_)...};
    }

    std::tuple<decltype(std::begin(std::declval<Ranges&>()))...> begin_;
    std::tuple<decltype(std::end(std::declval<Ranges&>()))...> end_;
  };

  constexpr iterator begin() noexcept LIFETIME_BOUND {
    return begin_impl(std::index_sequence_for<Ranges...>{});
  }

  constexpr ZipEnd end() noexcept { return ZipEnd(); }

 private:
  template <size_t... Is>
  constexpr iterator begin_impl(std::index_sequence<Is...>) LIFETIME_BOUND {
    return iterator(std::make_tuple(std::begin(std::get<Is>(ranges_))...),
                    std::make_tuple(std::end(std::get<Is>(ranges_))...));
  }

  std::tuple<Ranges&...> ranges_;
};

}  // namespace internal

// Zipping utility that allows iterating over multiple ranges in lockstep.
//
// Example:
//
// std::vector<int> a = {1, 2, 3};
// std::vector<double> b = {4.5, 5.5, 6.5};
// std::vector<std::string> c = {"x", "y", "z"};
// for (auto [x, y, z] : zip(a, b, c)) {
//   LOG(INFO) << x << " " << y << " " << z;
// }
//
// Zipping will carry on until one of the ranges run out, at which the loop will
// bail.
template <typename... Ranges>
constexpr internal::Zipper<Ranges...> zip(Ranges&... ranges LIFETIME_BOUND) {
  return internal::Zipper<Ranges...>(ranges...);
}

}  // namespace base

#endif  // BASE_TYPES_ZIP_H_
