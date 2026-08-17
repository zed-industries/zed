// Copyright 2019 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_UTIL_MISC_ARRAYSIZE_H_
#define CRASHPAD_UTIL_MISC_ARRAYSIZE_H_

#include <sys/types.h>  // For size_t.

#include <type_traits>

//! \file

namespace crashpad {
namespace internal {

//! \brief A helper to implement ArraySize.
template <typename ArrayType>
constexpr size_t ArraySizeHelper() noexcept {
  return std::extent<typename std::remove_reference<ArrayType>::type>::value;
}

}  // namespace internal
}  // namespace crashpad

//! \brief A way of computing an array’s size.
//!
//! Use this only where `std::size()` won’t work, such as in constant
//! expressions (including `static_assert` expressions) that consider the
//! sizes of non-static data members.
#define ArraySize(array) crashpad::internal::ArraySizeHelper<decltype(array)>()

#endif  // CRASHPAD_UTIL_MISC_ARRAYSIZE_H_
