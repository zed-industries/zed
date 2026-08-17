#pragma once
#include "./maybe_uninit_detail.h"
#include <cstddef>

namespace rust {
inline namespace cxxbridge1 {
template <typename T>
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};
} // namespace cxxbridge1
} // namespace rust
