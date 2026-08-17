#pragma once
#include "../../../include/cxx.h"
#include "./friend_impl.h"
#include "./ptr_len.h"

namespace rust {
inline namespace cxxbridge1 {
namespace {
template <>
class impl<Error> final {
public:
  static Error error(repr::PtrLen repr) noexcept {
    Error error;
    error.msg = static_cast<char const *>(repr.ptr);
    error.len = repr.len;
    return error;
  }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust
