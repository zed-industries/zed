#pragma once

namespace rust {
inline namespace cxxbridge1 {
namespace {
template <bool> struct deleter_if {
  template <typename T> void operator()(T *) {}
};
//
template <> struct deleter_if<true> {
  template <typename T> void operator()(T *ptr) { ptr->~T(); }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust
