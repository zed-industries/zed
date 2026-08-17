#pragma once
#include "../../../include/cxx.h"

namespace rust {
inline namespace cxxbridge1 {
template <typename T>
class Slice<T>::uninit {};
//
template <typename T>
inline Slice<T>::Slice(uninit) noexcept {}
} // namespace cxxbridge1
} // namespace rust
