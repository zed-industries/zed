#pragma once
#include "../../../include/cxx.h"
#include <type_traits>
#include <vector>

namespace rust {
inline namespace cxxbridge1 {
namespace {
template <typename T, bool = ::std::is_move_constructible<T>::value>
struct if_move_constructible {
  static bool reserve(::std::vector<T> &, ::std::size_t) noexcept {
    return false;
  }
};
//
template <typename T>
struct if_move_constructible<T, true> {
  static bool reserve(::std::vector<T> &vec, ::std::size_t new_cap) {
    vec.reserve(new_cap);
    return true;
  }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust
