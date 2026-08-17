//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBCPP_TEST_STD_ITERATOR_UNQUALIFIED_LOOKUP_WRAPPER
#define LIBCPP_TEST_STD_ITERATOR_UNQUALIFIED_LOOKUP_WRAPPER

#include <iterator>
#include <utility>

namespace check_unqualified_lookup {
// Wrapper around an iterator for testing unqualified calls to `iter_move` and `iter_swap`.
template <typename I>
class unqualified_lookup_wrapper {
public:
  unqualified_lookup_wrapper() = default;

  constexpr explicit unqualified_lookup_wrapper(I i) noexcept : base_(std::move(i)) {}

  constexpr decltype(auto) operator*() const noexcept { return *base_; }

  constexpr unqualified_lookup_wrapper& operator++() noexcept {
    ++base_;
    return *this;
  }

  constexpr void operator++(int) noexcept { ++base_; }

  constexpr bool operator==(unqualified_lookup_wrapper const& other) const noexcept {
    return base_ == other.base_;
  }

  // Delegates `std::ranges::iter_move` for the underlying iterator. `noexcept(false)` will be used
  // to ensure that the unqualified-lookup overload is chosen.
  friend constexpr decltype(auto) iter_move(unqualified_lookup_wrapper& i) noexcept(false) {
    return std::ranges::iter_move(i.base_);
  }

private:
  I base_ = I{};
};

template <class It>
unqualified_lookup_wrapper(It) -> unqualified_lookup_wrapper<It>;

enum unscoped_enum { a, b, c };
constexpr unscoped_enum iter_move(unscoped_enum& e) noexcept(false) { return e; }

enum class scoped_enum { a, b, c };
constexpr scoped_enum* iter_move(scoped_enum&) noexcept { return nullptr; }

union some_union {
  int x;
  double y;
};
constexpr int iter_move(some_union& u) noexcept(false) { return u.x; }

} // namespace check_unqualified_lookup

class move_tracker {
public:
  move_tracker() = default;
  constexpr move_tracker(move_tracker&& other) noexcept : moves_{other.moves_ + 1} { other.moves_ = 0; }
  constexpr move_tracker& operator=(move_tracker&& other) noexcept {
    moves_ = other.moves_ + 1;
    other.moves_ = 0;
    return *this;
  }

  move_tracker(move_tracker const& other) = delete;
  move_tracker& operator=(move_tracker const& other) = delete;

  constexpr int moves() const noexcept { return moves_; }

private:
  int moves_ = 0;
};

#endif // LIBCPP_TEST_STD_ITERATOR_UNQUALIFIED_LOOKUP_WRAPPER
