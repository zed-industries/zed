#pragma once

namespace rust {
inline namespace cxxbridge1 {
namespace {
template <typename T>
void destroy(T *ptr) {
  ptr->~T();
}
} // namespace
} // namespace cxxbridge1
} // namespace rust
