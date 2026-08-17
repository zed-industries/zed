#pragma once
#include "../../../include/cxx.h"
#include <cstdint>

namespace rust {
inline namespace cxxbridge1 {
namespace {
template <typename T>
struct IsRelocatableOrArray : IsRelocatable<T> {};
//
template <typename T, ::std::size_t N>
struct IsRelocatableOrArray<T[N]> : IsRelocatableOrArray<T> {};
} // namespace
} // namespace cxxbridge1
} // namespace rust
