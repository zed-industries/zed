#pragma once
#include <cstddef>
#include <new>

namespace rust {
inline namespace cxxbridge1 {
namespace detail {
template <typename T, typename = void *>
struct operator_new {
  void *operator()(::std::size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(::std::size_t sz) { return T::operator new(sz); }
};
} // namespace detail
} // namespace cxxbridge1
} // namespace rust
