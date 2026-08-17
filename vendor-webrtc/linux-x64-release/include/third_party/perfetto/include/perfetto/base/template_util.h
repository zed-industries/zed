/*
 * Copyright (C) 2021 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_BASE_TEMPLATE_UTIL_H_
#define INCLUDE_PERFETTO_BASE_TEMPLATE_UTIL_H_

#include <cstddef>
#include <type_traits>

namespace perfetto {
namespace base {

// Helper to express preferences in an overload set. If more than one overload
// is available for a given set of parameters the overload with the higher
// priority will be chosen.
template <size_t I>
struct priority_tag : priority_tag<I - 1> {};

template <>
struct priority_tag<0> {};

// enable_if_t is an implementation of std::enable_if_t from C++14.
//
// Specification:
// https://en.cppreference.com/w/cpp/types/enable_if
template <bool B, class T = void>
using enable_if_t = typename std::enable_if<B, T>::type;

// decay_t is an implementation of std::decay_t from C++14.
//
// Specification:
// https://en.cppreference.com/w/cpp/types/decay
template <class T>
using decay_t = typename std::decay<T>::type;

// remove_cvref is an implementation of std::remove_cvref from
// C++20.
//
// Specification:
// https://en.cppreference.com/w/cpp/types/remove_cvref

template <class T>
struct remove_cvref {
  using type = typename std::remove_cv<typename std::remove_cv<
      typename std::remove_reference<T>::type>::type>::type;
};
template <class T>
using remove_cvref_t = typename remove_cvref<T>::type;

// Check if a given type is a specialization of a given template:
// is_specialization<T, std::vector>::value.

template <typename Type, template <typename...> class Template>
struct is_specialization : std::false_type {};

template <template <typename...> class Ref, typename... Args>
struct is_specialization<Ref<Args...>, Ref> : std::true_type {};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_BASE_TEMPLATE_UTIL_H_
