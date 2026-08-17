// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_TESTING_TOOLS_H_
#define BASE_ALLOCATOR_DISPATCHER_TESTING_TOOLS_H_

#include <array>
#include <tuple>
#include <utility>

namespace base::allocator::dispatcher::testing {

namespace internal {
template <size_t Size, typename Type, typename... AppendedTypes>
struct DefineTupleFromSingleType {
  using type = typename DefineTupleFromSingleType<Size - 1,
                                                  Type,
                                                  AppendedTypes...,
                                                  Type>::type;
};

template <typename Type, typename... AppendedTypes>
struct DefineTupleFromSingleType<0, Type, AppendedTypes...> {
  using type = std::tuple<AppendedTypes...>;
};

}  // namespace internal

template <size_t Size, typename Type>
struct DefineTupleFromSingleType {
  using type = typename internal::DefineTupleFromSingleType<Size, Type>::type;
};

template <typename Type, size_t Size, size_t... Indices>
typename internal::DefineTupleFromSingleType<Size, Type*>::type
CreateTupleOfPointers(std::array<Type, Size>& items,
                      std::index_sequence<Indices...>) {
  return std::make_tuple((&items[Indices])...);
}

template <typename Type, size_t Size>
typename internal::DefineTupleFromSingleType<Size, Type*>::type
CreateTupleOfPointers(std::array<Type, Size>& items) {
  return CreateTupleOfPointers(items, std::make_index_sequence<Size>{});
}

}  // namespace base::allocator::dispatcher::testing

#endif  // BASE_ALLOCATOR_DISPATCHER_TESTING_TOOLS_H_
