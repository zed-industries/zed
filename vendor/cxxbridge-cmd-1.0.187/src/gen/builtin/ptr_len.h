#pragma once
#include <cstddef>

namespace rust {
inline namespace cxxbridge1 {
namespace repr {
struct PtrLen final {
  void *ptr;
  ::std::size_t len;
};
} // namespace repr
} // namespace cxxbridge1
} // namespace rust
