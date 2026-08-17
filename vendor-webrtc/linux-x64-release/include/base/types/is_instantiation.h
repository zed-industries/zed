// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_IS_INSTANTIATION_H_
#define BASE_TYPES_IS_INSTANTIATION_H_

#include <type_traits>

namespace base {
namespace internal {

// True if and only if `T` is `C<Types...>` for some set of types, i.e. `T` is
// an instantiation of the template `C`.
//
// This is false by default. We specialize it to true below for pairs of
// arguments that satisfy the condition.
template <template <typename...> class C, typename T>
inline constexpr bool is_instantiation_v = false;

template <template <typename...> class C, typename... Ts>
inline constexpr bool is_instantiation_v<C, C<Ts...>> = true;

}  // namespace internal

// True if and only if the type `T` is an instantiation of the template `C` with
// some set of type arguments.
//
// Note that there is no allowance for reference or const/volatile qualifiers;
// if these are a concern you probably want to feed through `std::decay_t<T>`.
template <template <typename...> class C, typename T>
concept is_instantiation = internal::is_instantiation_v<C, T>;

}  // namespace base

#endif  // BASE_TYPES_IS_INSTANTIATION_H_
