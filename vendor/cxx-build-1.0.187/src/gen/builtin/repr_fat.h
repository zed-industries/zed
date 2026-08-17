#pragma once
#include <array>
#include <cstdint>

namespace rust {
inline namespace cxxbridge1 {
namespace repr {
using Fat = ::std::array<::std::uintptr_t, 2>;
} // namespace repr
} // namespace cxxbridge1
} // namespace rust
