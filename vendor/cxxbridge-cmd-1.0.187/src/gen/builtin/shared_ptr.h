#pragma once
#include "../../../include/cxx.h"
#include <memory>

namespace rust {
inline namespace cxxbridge1 {
namespace {
template <typename T, bool = ::rust::detail::is_complete<T>::value>
struct is_destructible : ::std::false_type {};
//
template <typename T>
struct is_destructible<T, true> : ::std::is_destructible<T> {};
//
template <typename T>
struct is_destructible<T[], false> : is_destructible<T> {};
//
template <typename T, bool = ::rust::is_destructible<T>::value>
struct shared_ptr_if_destructible {
  explicit shared_ptr_if_destructible(typename ::std::shared_ptr<T>::element_type *) {}
};
//
template <typename T>
struct shared_ptr_if_destructible<T, true> : ::std::shared_ptr<T> {
  using ::std::shared_ptr<T>::shared_ptr;
};
} // namespace
} // namespace cxxbridge1
} // namespace rust
