#pragma once
#include <cstddef>

namespace rust {
inline namespace cxxbridge1 {
namespace repr {
#ifndef CXXBRIDGE_ALIGNMAX
#define CXXBRIDGE_ALIGNMAX
// This would be cleaner as the following, but GCC does not implement that
// correctly. <https://gcc.gnu.org/bugzilla/show_bug.cgi?id=64236>
//
//     template <::std::size_t... N>
//     class alignas(N...) alignmax {};
//
// Next, it could be this, but MSVC does not implement this correctly.
//
//     template <::std::size_t... N>
//     class alignmax { alignas(N...) union {} members; };
//
template <::std::size_t N>
class alignas(N) aligned {};
//
template <typename... T>
class alignmax_t { alignas(T...) union {} members; };
//
template <::std::size_t... N>
using alignmax = alignmax_t<aligned<N>...>;
#endif // CXXBRIDGE_ALIGNMAX
} // namespace repr
} // namespace cxxbridge1
} // namespace rust
